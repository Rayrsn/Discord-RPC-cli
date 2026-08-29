use std::process::exit;
use std::thread;
use std::time::Duration;

use colored::Colorize;
use discord_rich_presence::activity::Activity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

use crate::error::RpcError;

pub(crate) fn connect(client_id: &str) -> Result<DiscordIpcClient, RpcError> {
    let mut client = DiscordIpcClient::new(client_id).map_err(|_| RpcError::ConnectFailed)?;
    match client.connect() {
        Ok(_) => {
            println!("Client connected to Discord successfully.");
            Ok(client)
        }
        Err(_) => Err(RpcError::ConnectFailed),
    }
}

pub(crate) fn set_activity(
    client: &mut DiscordIpcClient,
    activity: Activity,
) -> Result<(), RpcError> {
    match client.set_activity(activity) {
        Ok(_) => {
            println!("Client set activity successfully.");
            Ok(())
        }
        Err(_) => Err(RpcError::SetActivityFailed),
    }
}

pub(crate) fn clear_activity(client: &mut DiscordIpcClient) -> Result<(), RpcError> {
    client
        .clear_activity()
        .map_err(|_| RpcError::SetActivityFailed)
}

pub(crate) fn handle_exit(exit_after: Option<i64>, disable_color: bool) -> Result<(), RpcError> {
    if let Some(exit_after) = exit_after {
        if !disable_color {
            println!(
                "{}{}{}",
                "Exiting in ".blue(),
                exit_after.to_string().red(),
                " seconds".blue()
            );
        } else {
            println!("Exiting in {} seconds", exit_after);
        }

        let secs = exit_after
            .try_into()
            .map_err(|_| RpcError::InvalidNumericValue("exit_after"))?;
        thread::sleep(Duration::from_secs(secs));
        exit(0);
    } else {
        if !disable_color {
            println!(
                "{}",
                "Running indefinitely, Press Ctrl+C to exit.".cyan()
            );
        } else {
            println!("Running indefinitely, Press Ctrl+C to exit.");
        }
        loop {
            thread::sleep(Duration::from_secs(10));
        }
    }
}
