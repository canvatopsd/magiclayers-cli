mod arguments;
mod client;
mod conversion;
mod delivery;
mod payments;
mod skill;
mod task_access;
mod task_id;
mod tasks;

use anyhow::Result;
use arguments::{Arguments, Command};
use clap::Parser;
use client::ProductClient;
use serde_json::{json, Value};

const CONTRACT: &str = include_str!("../contracts/guest-agent-commands.json");

fn execute(args: Arguments) -> Result<Value> {
    match &args.command {
        Command::Discover => return Ok(serde_json::from_str(CONTRACT)?),
        Command::Skill { command } => return skill::run(command),
        _ => {}
    }
    let client = ProductClient::new(&args.base_url)?;
    let service = client.origin();
    match args.command {
        Command::Discover => Ok(serde_json::from_str(CONTRACT)?),
        Command::Skill { command } => skill::run(&command),
        Command::Doctor => Ok(json!({
            "version":env!("CARGO_PKG_VERSION"),
            "service":service,
            "configuration":client.post("/api/v2/task/config/read",json!({}),None)?,
            "authentication":"per-task access; no account session is used"
        })),
        Command::Convert(input) => conversion::submit(input, &client, &service),
        Command::Status { task_id } => tasks::status(&client, &service, task_id),
        Command::Download {
            task_id,
            kind,
            output,
        } => tasks::download(&client, &service, task_id, &kind, &output),
        Command::Checkout { task_id, provider } => {
            payments::checkout(&client, &service, task_id, provider)
        }
        Command::Payment { task_id } => payments::read(&client, &service, task_id),
        Command::VerifyPayment {
            task_id,
            checkout_id,
            provider,
        } => payments::verify(&client, &service, task_id, checkout_id, provider),
    }
}
fn main() {
    let args = match Arguments::try_parse() {
        Ok(args) => args,
        Err(error)
            if matches!(
                error.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) =>
        {
            error.exit()
        }
        Err(error) => {
            println!(
                "{}",
                json!({"ok":false,"error":{"code":"invalid_arguments","message":error.to_string()}})
            );
            std::process::exit(2);
        }
    };
    match execute(args) {
        Ok(data) => println!("{}", json!({"ok":true,"data":data})),
        Err(error) => {
            let failure = match error.downcast_ref::<client::ApiFailure>() {
                Some(api) => json!({"code":api.code,"message":api.message,"status":api.status}),
                None => json!({"code":"command_failed","message":format!("{error:#}")}),
            };
            println!("{}", json!({"ok":false,"error":failure}));
            std::process::exit(1);
        }
    }
}
