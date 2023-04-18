use chisel::{
    prelude::{format_source, ChiselCommand, ChiselDispatcher, DispatchResult},
    solidity_helper::SolidityHelper,
};
use foundry_config::FormatterConfig;
use std::error::Error;
use yansi::Paint;

use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

use crate::helpers::{dispatch::log_dispatch_result, split_commands::split_commands};

use super::context::create_context_string;

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
    formatter_config: FormatterConfig,
}

type OpenAIResult<T> = Result<T, Box<dyn Error>>;

impl CompletionClient {
    pub async fn new(dispatcher: &mut ChiselDispatcher, formatter_config: FormatterConfig) -> Self {
        let client = Client::new();

        let help_result = dispatcher.dispatch_command(ChiselCommand::Help, &[]).await;

        let mut help_text: Option<String> = None;
        if let DispatchResult::CommandSuccess(help_result) = help_result {
            help_text = Some(Paint::green(help_result.unwrap()).to_string());
        }

        Self {
            client,
            help_text: help_text.unwrap(),
            formatter_config,
        }
    }

    pub async fn handle_chat_request(
        &self,
        dispatcher: &mut ChiselDispatcher,
        line: String,
    ) -> OpenAIResult<()> {
        println!(
            "{}",
            Paint::blue("\nFetching required command recipe from ChiselGPT\n")
        );

        let (commands, raw_response) = self.get_chat_response(dispatcher, line).await.unwrap();

        if commands.len() == 0 {
            eprintln!(
                "No Commands found for response: {}",
                Paint::red(raw_response)
            );

            return Ok(());
        }

        println!(
            "{}",
            Paint::green(format!(
                "Cooking command recipe ({} ingredients)",
                commands.len()
            ))
        );

        for (index, raw_command) in commands.into_iter().enumerate() {
            let formatted_command = match format_source(&raw_command, self.formatter_config.clone())
            {
                Ok(formatted_source) => SolidityHelper::highlight(&formatted_source).into_owned(),
                Err(_) => SolidityHelper::highlight(&raw_command).into_owned(),
            };

            println!("\n{}", Paint::magenta(format!("Ingredient {}:", index + 1)));
            println!("{}", Paint::green(&formatted_command));

            let dispatch_result = dispatcher.dispatch(&raw_command).await;
            log_dispatch_result(&dispatch_result);
        }

        Ok(())
    }

    async fn get_chat_response(
        &self,
        dispatcher: &mut ChiselDispatcher,
        request: String,
    ) -> OpenAIResult<(Vec<String>, String)> {
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

        let raw_response = response.choices.get(0).unwrap().message.content.clone();

        let commands = split_commands(&raw_response);

        Ok((commands, raw_response))
    }
}
