# Helper CLI Tool

Helper CLI is a command line tool built in Rust that utilizes OpenAI chat models. It facilitates ongoing or one-off conversations with the AI and stores conversation histories and the current conversation in a json file. All input goes to the same conversation until you create a new one, and you may go back to a pervious conversation by specifying the conversation id once, after which it is the current conveersation. 

You can set a default system message with an environment variable or provide it as an argument when you create a new conversation.

Helper is great for use in scripts and integrates well with traditional shell tools!

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
    - [Environment Variables](#environment-variables)
    - [Input Arguments](#input-arguments)
    - [Examples](#examples)
- [Contribution](#contribution)

## Installation

Before using the Helper CLI tool, ensure you have Rust installed on your machine. Follow the instructions found in the [official Rust documentation](https://www.rust-lang.org/tools/install) to install Rust. Download the source code and, in the project directory, run the following to build and install all dependencies:

```bash
cargo build --release
```

To install the CLI tool, run:

```bash
cargo install --path .
```

## Usage

To start using the Helper CLI tool, you will need to set the `OPENAI_API_KEY` environment variable with your OpenAI API key.

### Environment Variables

- `OPENAI_API_KEY` (required): The API key required to authenticate with OpenAI.
- `HELPER_HISTORY_FILE` (optional): The file path to the history JSON file which stores conversation history.
- `HELPER_SYSTEM_MESSAGE` (optional): The system message to provide context for the conversation.

### Input Arguments

- `-s, --system`: The system message to provide context for the conversation or the text file containing the system message. By default, the value is set to the `HELPER_SYSTEM_MESSAGE` environment variable.
- `-c, --conversation`: The ID of an existing conversation to continue.
- `-n, --new`: Create a new conversation.
- `-H, --history`: The path to the history JSON file. By default, the value is set to the `HELPER_HISTORY_FILE` environment variable.
- `-i, --stdin`: Read user input from stdin.
- `-m, --message`: The user message to be sent to the AI model.

  If both stdin and message arguments are provided, the message argument will be appended to the stdin content separated by a newline. This is useful when discussing a file's content with the AI and providing additional context or questions about the file.

### Examples

1. Create a new conversation with a system message:

```bash
helper -n -s "You are an AI assistant helping the user with programming tasks."
```

2. Continue an existing conversation:

```bash
helper -c 1 -m "What's the difference between a HashMap and a BTreeMap?"
```

3. Get a response from the AI model:

```bash
helper -m "How can I iterate over a HashMap in Rust?"
```

4. Provide file content via stdin and ask a question about it:

```bash
cat example_code.rs | helper -i -m "How can I optimize this code for better performance?"
```

## Contribution

Contributions are welcome! If you'd like to help improve or add new features to this tool, fork the repository, make your changes, and submit a pull request back to the main repository. Please ensure your code adheres to the existing style and follows Rust best practices.