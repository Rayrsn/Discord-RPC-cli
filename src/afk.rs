use std::thread;
use std::time::Duration;

use colored::Colorize;
use discord_rich_presence::DiscordIpc;
use discord_rich_presence::activity::{Activity, Assets, Button};
use user_idle::UserIdle;

use crate::cli::Cli;
use crate::error::RpcError;
use crate::rpc;

pub(crate) const AFK_CLIENT_ID: &str = "871404899915665438";

pub(crate) fn format_inactive_message(seconds: u64, minutes: u64) -> String {
    match seconds {
        0..=59 => format!("I've been inactive for {seconds} seconds"),
        60..=119 => "I've been inactive for a minute".to_string(),
        _ => format!("I've been inactive for {minutes} minutes"),
    }
}

pub(crate) fn print_banner(afk_after: i64, disable_color: bool) {
    if !disable_color {
        println!("{} is enabled.", "AFK RPC".magenta());
        if afk_after == 1 {
            println!("Will AFK after {} minute.", afk_after.to_string().magenta());
        } else if afk_after == 0 {
            println!("Will AFK {}", "immediately".magenta());
        } else {
            println!(
                "Will AFK after {} minutes.",
                afk_after.to_string().magenta()
            );
        }
    } else {
        println!("AFK RPC is enabled.");
        if afk_after == 1 {
            println!("Will AFK after {afk_after} minute.");
        } else if afk_after == 0 {
            println!("Will AFK immediately.");
        } else {
            println!("Will AFK after {afk_after} minutes.");
        }
    }
}

pub(crate) fn run(args: &Cli) -> Result<(), RpcError> {
    let mut client = rpc::connect(AFK_CLIENT_ID)?;
    print_banner(args.afk_after, args.disable_color);

    let activity = Activity::new();
    let activity_state = activity.state("Currently Inactive");
    let activity_large_image =
        activity_state.assets(Assets::new().large_image("idle").large_text("Inactive"));
    let activity_button = activity_large_image.buttons(vec![Button::new(
        "Check out the Github!",
        "https://github.com/Rayrsn/Discord-RPC-cli",
    )]);

    let update_secs: u64 = args
        .afk_update
        .try_into()
        .map_err(|_| RpcError::InvalidNumericValue("afk_update"))?;
    let afk_after_mins: u64 = args
        .afk_after
        .try_into()
        .map_err(|_| RpcError::InvalidNumericValue("afk_after"))?;

    let mut connected = false;

    loop {
        thread::sleep(Duration::from_secs(update_secs));
        let idle = match UserIdle::get_time() {
            Ok(idle) => idle,
            Err(err) => return Err(RpcError::IdleDetectionFailed(err.to_string())),
        };

        let idle_minutes = idle.as_minutes();
        let idle_seconds = idle.as_seconds();

        if idle_minutes >= afk_after_mins && !connected {
            client.connect().map_err(|_| RpcError::ConnectFailed)?;

            let details = format_inactive_message(idle_seconds, idle_minutes);
            client
                .set_activity(activity_button.clone().details(&details))
                .map_err(|_| RpcError::SetActivityFailed)?;
            connected = true;
        }

        if connected && idle_minutes < afk_after_mins {
            rpc::clear_activity(&mut client)?;
            connected = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_seconds_under_a_minute() {
        assert_eq!(
            format_inactive_message(45, 0),
            "I've been inactive for 45 seconds"
        );
        assert_eq!(
            format_inactive_message(0, 0),
            "I've been inactive for 0 seconds"
        );
    }

    #[test]
    fn formats_single_minute() {
        assert_eq!(
            format_inactive_message(60, 1),
            "I've been inactive for a minute"
        );
        assert_eq!(
            format_inactive_message(119, 1),
            "I've been inactive for a minute"
        );
    }

    #[test]
    fn formats_multiple_minutes() {
        assert_eq!(
            format_inactive_message(120, 2),
            "I've been inactive for 2 minutes"
        );
        assert_eq!(
            format_inactive_message(360, 6),
            "I've been inactive for 6 minutes"
        );
    }
}
