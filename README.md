# GPT-Ask

GPT-Ask is a CLI tool written in Rust that queries that chat gpt AI in order to either ask question/refactor/document your code without going to the browser.

## Commands

* `gpt-ask ask --question=question`: Ask a question to chat gpt.
* `gpt-ask document --file=code_file --out=documented_code_file`: Rewrite code provided with comments and write the result in out.
* `gpt-ask refactor --file=code_file --out=documented_code_file`: Refactor code provided and write the result in out.

## Build from source and Installation

```sh
git clone https://github.com/Giulio2002/gpt-ask
cargo install --path .
```



