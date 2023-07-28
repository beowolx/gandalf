
# Gandalf

Gandalf is an AI assistant chatbot leveraging the power of the [TheBloke/Llama-2-7B-Chat-GGML](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGML). Gandalf is a good alternative for those who don't want to run private data through a model like GPT. The model runs entirely in your computer.

Gandalf is built using the Rust [Leptos](https://github.com/leptos-rs/leptos) framework, which is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces in WebAssembly.

Style is made using [Tailwind CSS](https://tailwindcss.com/).

![An image of Gandalf from Lord of the Ring riding a crab](https://github.com/LuisCardosoOliveira/gandalf/assets/61982523/4ab0f3c4-6923-4087-a9d0-87b357570872)


## LLaMa2-Chat

This project is a chatbot that uses the LLM LLaMa2-chat, a state-of-the-art large language model fine-tuned for dialogue use cases. LLaMa2-chat is based on [LLaMa2](https://ai.meta.com/llama/), an open-source foundation model that is trained on 40% more data and has twice the context length of its predecessor, LLaMa1. LLaMa2-chat is optimized for naturalness, helpfulness, and safety using human feedback and reinforcement learning1. It can handle various types of conversations, such as chit-chat, instruction-following, and question-answering. This chatbot aims to provide a friendly and engaging experience for users who want to chat with an intelligent and creative agent.

## How does it look like?

![Image 20-07-2023 at 09 01](https://github.com/LuisCardosoOliveira/gandalf/assets/61982523/85a76ab4-c7cf-4b70-acc5-1456221a6f7e)

## Installation

To get started with Gandalf, you need to install [Rust](https://www.rust-lang.org/learn/get-started).

You will also need to install the `wasm32-unknown-unknown` target, `trunk` and `cargo-leptos` tools.

You are also going to need to download [Llama2-Chat 7B](https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGMLL).

After you have followed the instructions to install Rust, run the commands below in your terminal:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

You'll need to modify the `.env` file to set the model path of the LLaMa2 model that you have downloaded. You can find an example in the `.env` file.

To run the chatbot, just run in your terminal `cargo leptos watch` and go to http://localhost:3000/
