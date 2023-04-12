use chisel::prelude::{ChiselCommand, ChiselDispatcher, DispatchResult};
use regex::Regex;
use std::error::Error;
use yansi::Paint;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

use crate::helpers::dispatch::dispatch_print;

use super::context::create_context_string;

fn parse_response(input: &str) -> Vec<String> {
    let re = Regex::new(r"##Start##\s*([\s\S]+?)\s*##End##").unwrap();
    if let Some(captures) = re.captures(input) {
        let content = captures.get(1).unwrap().as_str();
        let commands: Vec<String> = content
            .split('`')
            .map(|s| {
                let mut owned = String::from(s.trim());
                owned
            })
            .filter(|s| s.len() > 0)
            .collect();

        return commands;
    }

    unimplemented!("Error: {}", Paint::red(input));
}

fn build_request(
    request: String,
    help_text: String,
    chisel_context: String,
) -> Result<CreateChatCompletionRequest, Box<dyn Error>> {
    // println!(
    //     "{}",
    //     Paint::green(create_context_string(
    //         help_text.clone(),
    //         chisel_context.clone()
    //     ))
    // );
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .temperature(0.0) // To get deterministic results
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(create_context_string(help_text, chisel_context))
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(request)
                .build()?,
        ])
        .build()?;

    Ok(request)
}

pub struct CompletionClient {
    client: Client,
    help_text: String,
}

type OpenAIResult<T> = Result<T, Box<dyn Error>>;

impl CompletionClient {
    pub async fn new(dispatcher: &mut ChiselDispatcher) -> Self {
        let client = Client::new();

        let help_result = dispatcher.dispatch_command(ChiselCommand::Help, &[]).await;

        let mut help_text: Option<String> = None;
        if let DispatchResult::CommandSuccess(help_result) = help_result {
            help_text = Some(Paint::green(help_result.unwrap()).to_string());
        }

        Self {
            client,
            help_text: help_text.unwrap(),
        }
    }

    pub async fn handle_chat_request(
        &self,
        dispatcher: &mut ChiselDispatcher,
        line: String,
    ) -> OpenAIResult<()> {
        let is_tracing = match &dispatcher.session.session_source {
            Some(result) => result.config.traces,
            None => false,
        };

        if is_tracing {
            println!("{}", Paint::blue("Getting command recipe from ChiselGPT"));
        }

        let mut chat_response = self.get_chat_response(dispatcher, line).await.unwrap();

        if is_tracing {
            println!("{}", Paint::green("Got Recipe: "));
            for (index, part) in chat_response.clone().iter().enumerate() {
                println!("{}: {}", index, part);
            }

            println!("{}", Paint::green("Running..."));
        }

        while chat_response.len() > 0 {
            let command = chat_response.get(0).unwrap();
            println!("{}", command);

            dispatch_print(dispatcher, &command).await;

            chat_response.remove(0);
        }

        Ok(())
    }

    async fn get_chat_response(
        &self,
        dispatcher: &mut ChiselDispatcher,
        request: String,
    ) -> OpenAIResult<Vec<String>> {
        let chisel_context = dispatcher
            .dispatch_command(ChiselCommand::Source, &[])
            .await;

        let mut chisel_state: Option<String> = None;
        if let DispatchResult::CommandSuccess(chisel_context) = chisel_context {
            if let Some(chisel_context) = chisel_context {
                chisel_state = Some(Paint::green(chisel_context).to_string());
            }
        }

        let chisel_state = chisel_state.unwrap();

        let response = self
            .client
            .chat()
            .create(build_request(
                request,
                self.help_text.clone(),
                chisel_state,
            )?)
            .await?;

        let choice = &response.choices.get(0).unwrap().message.content;

        Ok(parse_response(choice))
    }
}

// !chat Create a new contract with a method that takes in two uint256 values, multiplies the result and returns it. Then create an instance of the contract and call the method
// !chat deal me 100 eth and create a new contract that hashes the result of multiplying 2 input

// !TODO bugg
// !chat create a new contract that executes x*y/z
// !chat create an erc20 contract with a fallback that reverts, then call the fallback
