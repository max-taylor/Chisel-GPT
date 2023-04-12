use chisel::prelude::{ChiselDispatcher, DispatchResult};
use yansi::Paint;

pub async fn dispatch_command(dispatcher: &mut ChiselDispatcher, line: &str) {
    // Dispatch and match results
    match dispatcher.dispatch(&line).await {
  DispatchResult::Success(msg) | DispatchResult::CommandSuccess(msg) => if let Some(msg) = msg {
      println!("{}", Paint::green(msg));
  },
  DispatchResult::UnrecognizedCommand(e) => {
    println!("errror");
    eprintln!("{e}")
},
  DispatchResult::SolangParserFailed(e) => {
      eprintln!("{}", Paint::red("Compilation error"));
      eprintln!("{}", Paint::red(format!("{e:?}")));
  }
  DispatchResult::FileIoError(e) => eprintln!("{}", Paint::red(format!("⚒️ Chisel File IO Error - {e}"))),
  DispatchResult::CommandFailed(msg) | DispatchResult::Failure(Some(msg)) => eprintln!("{}", Paint::red(msg)),
  DispatchResult::Failure(None) => eprintln!("{}\nPlease Report this bug as a github issue if it persists: https://github.com/foundry-rs/foundry/issues/new/choose", Paint::red("⚒️ Unknown Chisel Error ⚒️")),
}
}
