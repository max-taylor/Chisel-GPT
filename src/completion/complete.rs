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

use crate::helpers::dispatch::dispatch_command;

use super::context::create_context_string;

fn parse_response(input: &str) -> Vec<String> {
    let re = Regex::new(r"##Start##\s*([\s\S]+?)\s*##End##").unwrap();
    if let Some(captures) = re.captures(input) {
        let content = captures.get(1).unwrap().as_str();
        println!("{}", Paint::green(content));
        let commands: Vec<String> = content
            .split("`")
            .map(|s| String::from(s.trim()))
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
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .temperature(0.0) // To get close-to deterministic results
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

        let chat_response = self.get_chat_response(dispatcher, line).await.unwrap();

        if is_tracing {
            println!("{}", Paint::green("Got Recipe: "));
            for (index, part) in chat_response.clone().iter().enumerate() {
                println!("{}: {}", index, part);
            }

            println!("{}", Paint::green("Running..."));
        }

        for command in chat_response {
            println!("{}", command);

            dispatch_command(dispatcher, &command).await;
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
