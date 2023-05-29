use chisel::{
    prelude::{format_source, ChiselCommand, ChiselDispatcher, DispatchResult},
    solidity_helper::SolidityHelper,
};
use foundry_config::FormatterConfig;

use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionStreamResponse, Role,
    },
    Client,
};
use std::error::Error;
use std::{iter::Iterator, vec};
use yansi::Paint;

use crate::helpers::dispatch::log_dispatch_result;
use futures::StreamExt;

use super::{command_response::CommandResponse, context::create_context_string};

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

        let total_commands = self.get_chat_response(dispatcher, line).await?;

        if total_commands == 0 {
            // eprintln!(
            //     "No Commands found for response: {}",
            //     Paint::red(raw_response)
            // );
            eprintln!("No Commands found for response");
        } else {
            println!(
                "{}",
                Paint::green(format!(
                    "Finished command recipe of ({} ingredients)",
                    total_commands
                ))
            );
        }

        // for (index, raw_command) in commands.into_iter().enumerate() {
        //     let formatted_command = match format_source(&raw_command, self.formatter_config.clone())
        //     {
        //         Ok(formatted_source) => SolidityHelper::highlight(&formatted_source).into_owned(),
        //         Err(_) => SolidityHelper::highlight(&raw_command).into_owned(),
        //     };

        //     println!("\n{}", Paint::magenta(format!("Ingredient {}:", index + 1)));
        //     println!("{}", Paint::green(&formatted_command));

        //     let dispatch_result = dispatcher.dispatch(&raw_command).await;
        //     log_dispatch_result(&dispatch_result);
        // }

        Ok(())
    }

    fn parse_chat_completion_response(response: CreateChatCompletionStreamResponse) -> String {
        let mut response_string = String::new();

        for choice in response.choices {
            if let Some(content) = choice.delta.content {
                response_string.push_str(&content);
            }
        }

        response_string
    }

    async fn get_chat_response(
        &self,
        dispatcher: &mut ChiselDispatcher,
        request: String,
    ) -> OpenAIResult<usize> {
        let chisel_context = dispatcher
            .dispatch_command(ChiselCommand::Source, &[])
            .await;

        let chisel_state = match chisel_context {
            DispatchResult::CommandSuccess(chisel_context) => chisel_context,
            _ => {
                return Err("Failed to get Chisel Context".into());
            }
        }
        .unwrap();

        let mut stream = self
            .client
            .chat()
            .create_stream(self.get_request(request, chisel_state)?)
            .await?;

        let mut command_response = CommandResponse::new();

        println!("{}", Paint::green(format!("Cooking command recipe")));

        // TODO: Escape kills stream

        while let Some(response) = stream.next().await {
            match response {
                Ok(ccr) => {
                    // Create string of all choices
                    let next_line = Self::parse_chat_completion_response(ccr);

                    if command_response.within_command_response {
                        let highlighted_command =
                            SolidityHelper::highlight(&next_line).into_owned();

                        print!("{}", highlighted_command);
                    }

                    if command_response.handle_next_stream_value(&next_line) {
                        println!(
                            "\n{}",
                            Paint::magenta(format!(
                                "Trying to create ingredient {}:",
                                command_response.commands.len()
                            ))
                        );

                        command_response
                            .try_send_pending_commands(dispatcher)
                            .await?;
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }

        Ok(command_response.commands.len())

        // Ok((vec![], "".to_string()))

        // let commands = split_commands(&raw_response);

        // Ok((commands, raw_response))
    }

    fn get_request(
        &self,
        request: String,
        chisel_context: String,
    ) -> Result<CreateChatCompletionRequest, OpenAIError> {
        CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .temperature(0.0) // To get close-to deterministic results
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(create_context_string(
                        self.help_text.clone(),
                        chisel_context,
                    ))
                    .build()?,
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(request)
                    .build()?,
            ])
            .build()
    }
}
