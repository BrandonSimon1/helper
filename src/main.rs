use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::path::Path;
use std::{env, io::Read};

use clap::{arg, command, Parser};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Conversation {
    id: i64,
    messages: Vec<ChatCompletionMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct History {
    current_conversation_id: i64,
    conversations: HashMap<i64, Conversation>,
}

lazy_static! {
    static ref HELPER_HISTORY_FILE: String =
        env::var("HELPER_HISTORY_FILE").unwrap_or("history.json".to_string());
    static ref HELPER_SYSTEM_MESSAGE: String =
        env::var("HELPER_SYSTEM_MESSAGE").unwrap_or("".to_string());
}

// env var with static lifetime
#[derive(Parser, Debug)]
#[command(author="Brandon Simon <brandon.n.simon@gmail.com", version="v0.1.0", about="llm cli", long_about = None)]
struct Args {
    #[arg(short = 's', long = "system", default_value = HELPER_SYSTEM_MESSAGE.as_str())]
    system: Option<String>,
    #[arg(short = 'c', long = "conversation", conflicts_with = "new")]
    conversation: Option<i64>,
    #[arg(short = 'n', long = "new")]
    new: bool,
    #[arg(short = 'H', long = "history", default_value = HELPER_HISTORY_FILE.as_str())]
    history_file_path: Option<String>,
    #[arg(short = 'i', long = "stdin", default_value = "false")]
    stdin: bool,
    #[arg(short = 'm', long = "message")]
    message: Option<String>,
}

#[tokio::main]
async fn main() {
    // get env vars first here

    set_key(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));

    let args = Args::parse();

    // get history file as json and create if not exists
    let history_file = args.history_file_path.unwrap();
    let history_file_path = Path::new(history_file.as_str());
    let mut history: Option<History> = if history_file_path.exists() {
        let history_file_contents = fs::read_to_string(history_file_path).unwrap();
        serde_json::from_str(&history_file_contents).unwrap()
    } else {
        None
    };

    if args.new || history.is_none() {
        //args.system is either the system message or the path to a text file containing it. First see if a file exists then see if not use it as context
        let system_message_text: String;
        let system_message: ChatCompletionMessage;
        if args.system.is_some() {
            let s = args.system.clone().unwrap();
            let path = Path::new(&s);
            match path.try_exists() {
                Ok(exists) if exists => {
                    system_message_text = fs::read_to_string(path).unwrap();
                }
                Ok(_) | Err(_) => {
                    system_message_text = args.system.unwrap();
                }
            }
            system_message = ChatCompletionMessage {
                content: Some(system_message_text),
                role: ChatCompletionMessageRole::System,
                name: None,
                function_call: None,
            };
            if history.is_none() {
                let mut conversation = Conversation {
                    id: 0,
                    messages: Vec::new(),
                };
                conversation.messages.push(system_message);
                let mut h = History {
                    current_conversation_id: conversation.id,
                    conversations: HashMap::new(),
                };
                h.current_conversation_id = conversation.id;
                h.conversations.insert(conversation.id, conversation);
                history = Some(h);
            } else {
                let mut h = history.unwrap();
                let mut conversation = Conversation {
                    id: h.conversations.len() as i64,
                    messages: Vec::new(),
                };
                conversation.messages.push(system_message);
                h.current_conversation_id = conversation.id;
                h.conversations.insert(conversation.id, conversation);
                history = Some(h);
            }
        }
    }

    // set new current conversation or not and get current conversation
    let mut conversation = if args.conversation.is_some() {
        let conversation_id = args.conversation.unwrap();
        history.as_mut().unwrap().current_conversation_id = conversation_id;
        history
            .as_mut()
            .unwrap()
            .conversations
            .get_mut(&conversation_id)
            .unwrap()
            .clone()
    } else {
        let current_conversation_id = history.as_ref().unwrap().current_conversation_id;
        history
            .as_mut()
            .unwrap()
            .conversations
            .get_mut(&current_conversation_id)
            .unwrap()
            .clone()
    };

    // if stdin is true, get message from stdin
    let message = if args.stdin {
        let mut message = String::new();
        io::stdin()
            .read_to_string(&mut message)
            .expect("Failed to read stdin");
        message.push_str("\n");
        message.push_str(args.message.unwrap().as_str());
        message
    } else {
        args.message.unwrap()
    };

    // add message to conversation
    let message = ChatCompletionMessage {
        content: Some(message),
        role: ChatCompletionMessageRole::User,
        name: None,
        function_call: None,
    };

    conversation.messages.push(message.clone());

    // get completion from openai
    let chat_completion = ChatCompletion::builder("gpt-4-0314", conversation.messages.clone())
        .create()
        .await
        .unwrap();
    let returned_message = chat_completion.choices.first().unwrap().message.clone();
    // add to history
    conversation.messages.push(returned_message.clone());
    // print returned message
    println!("{}", returned_message.content.unwrap());

    // write conversation to history
    history
        .as_mut()
        .unwrap()
        .conversations
        .insert(conversation.id, conversation);

    let history_file_contents = serde_json::to_string(&history.unwrap()).unwrap();
    fs::write(history_file_path, history_file_contents).unwrap();
}
