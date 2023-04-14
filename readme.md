# ChiselGPT: AI-Boosted Solidity REPL

Chisel-GPT is an extension for [Chisel](https://github.com/foundry-rs/foundry/tree/master/chisel) allowing natural language requests. The natural language requests are converted into Chisel/Solidity commands and they are executed within the Chisel environment.

Still a work in progress, responses may not always be accurate.

## Examples

Commands can be requested by prefixing your request with `!chat`, for example:

```
!chat create a contract that uses assembly to bit shift numbers

!chat create a simple poc for a liquidity pool

!chat deal me 100 ETH
```

# Usage

In order to send requests to OpenAI for completion requests, it expects an environment variable for `OPENAI_API_KEY`

Clone the repository, then simply run:

```rust
cargo run
```

# TODO

Feel free to submit PR's or issues

- [ ] Include previous messages and responses in the openai request
- [ ] Modify !help, to include the custom command; !chat
- [ ] Replace the async_openai package with a package that allows you to set the seed, this will allow full deterministic responses
- [ ] Improve the prompt
