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

First clone the repository

In order to send requests to OpenAI for natural language to Solidity/Chisel commands, it requires an api key.

To set this on a MAC or Linux do:

```bash
export OPENAI_API_KEY=...
```

This project uses foundry to enable support for commonly used solidity libraries. If you don't have foundry installed [go here](https://github.com/foundry-rs/foundry/blob/master/README.md) for help.

Then run the following to install the solidity dependencies:

```bash
forge install
```

Now start the tool with:

```bash
cargo run
```

# Disclaimer

Not that ChatGPT was last trained on data up to September 2021. As a result, some responses may be outdated or not accurately reflect the latest information, best practices, or updates in the space. This tool serves to help understand new concepts and quickly trial ideas using ChatGPT!

# TODO

Feel free to submit PR's or issues

- [ ] Include previous messages and responses in the openai request
- [ ] Modify !help, to include the custom command; !chat
- [ ] If '##START##' is found but '##END##' isn't, query for the remaining code
- [ ] Remove .unwrap() calls, update error handling to be more coherent
- [x] Update parsing logic to not require a command delineator (to reduce the chance of ChatGPT sending bad responses)
- [x] Improve the prompt
