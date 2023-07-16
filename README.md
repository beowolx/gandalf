# Gandalf

Gandalf is an AI assistant chatbot leveraging the power of the [Wizard-Vicuna-7B-Uncensored model](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML). Gandalf is a good alternative for those who don't want to run private data through a model like GPT. The model runs entirely in your computer.

![gandalf-crab](https://github.com/LuisCardosoOliveira/gandalf/assets/61982523/ee123bcd-745a-4b7a-b8e6-c9efd1e9b485)

Gandalf is built using the Rust [Leptos](https://github.com/leptos-rs/leptos) framework, which is a full-stack, isomorphic Rust web framework leveraging fine-grained reactivity to build declarative user interfaces in WebAssembly.

## How does it look like?

Style is made using [Tailwind CSS](https://tailwindcss.com/).

![demo](https://github.com/LuisCardosoOliveira/gandalf/assets/61982523/561e584c-a6c4-4e63-8051-cd46c092bd6f)

## Installation

To get started with Gandalf, you need to install the nightly version of [Rust](https://www.rust-lang.org/learn/get-started), install the `wasm32-unknown-unknown` target, Trunk and `cargo-leptos` tools.

You are also going to need to download [Wizard-Vicuna-7B-Uncensored model](https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML).

After you have followed the instructions to install Rust, run the commands below in your terminal:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

You'll need to modify the `.env` file to set the model path of the Wizard model that you have downloaded. You can find an example in the `.env` file.

To run the chatbot, just run in your terminal `cargo leptos watch` and go to http://localhost:3000/

## How Does It Compare to ChatGPT?

![comp](https://github.com/LuisCardosoOliveira/gandalf/assets/61982523/a52d7957-82f2-4ec7-82f7-db722b821252)

Please note that this project is highly experimental and designed for proof of concept, not for actual usage.

It's important to be aware that the Wizard-Vicuna-7B-Uncensored model, as the name suggests, is "uncensored." As a consequence, it can potentially generate content that is unsafe, offensive, or otherwise objectionable. This doesn't necessarily mean it will do so regularly or intentionally, but the possibility exists due to the nature of the model's training and operation.

Please exercise caution and use your best judgement while interacting with Gandalf.
