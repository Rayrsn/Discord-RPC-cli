use std::thread;
use std::time::Duration;

use colored::Colorize;
use discord_rich_presence::DiscordIpc;
use discord_rich_presence::activity::{Activity, Assets, Button};

use crate::cli::Cli;
use crate::error::RpcError;
use crate::idle::{IdleMonitor, IdleState};
use crate::rpc;

pub(crate) const AFK_CLIENT_ID: &str = "871404899915665438";

pub(crate) fn format_inactive_message(seconds: u64, minutes: u64) -> String {
    match seconds {
        0..=59 => format!("I've been inactive for {seconds} seconds"),
        60..=119 => "I've been inactive for a minute".to_string(),
        _ => format!("I've been inactive for {minutes} minutes"),
    }
}

pub(crate) fn print_banner(threshold_seconds: u64, disable_color: bool) {
    if !disable_color {
        println!("{} is enabled.", "AFK RPC".magenta());
        if threshold_seconds == 0 {
            println!("Will AFK {}", "immediately".magenta());
        } else if threshold_seconds < 60 {
            println!(
                "Will AFK after {} seconds.",
                threshold_seconds.to_string().magenta()
            );
        } else if threshold_seconds == 60 {
            println!("Will AFK after {} minute.", "1".magenta());
        } else {
            let minutes = threshold_seconds / 60;
            println!("Will AFK after {} minutes.", minutes.to_string().magenta());
        }
    } else {
        println!("AFK RPC is enabled.");
        if threshold_seconds == 0 {
            println!("Will AFK immediately.");
        } else if threshold_seconds < 60 {
            println!("Will AFK after {threshold_seconds} seconds.");
        } else if threshold_seconds == 60 {
            println!("Will AFK after 1 minute.");
        } else {
            let minutes = threshold_seconds / 60;
            println!("Will AFK after {minutes} minutes.");
        }
    }
}

pub(crate) fn get_threshold(args: &Cli) -> Result<Duration, RpcError> {
    if let Some(seconds) = args.afk_seconds {
        Ok(Duration::from_secs(seconds))
    } else {
        let mins: u64 = args
            .afk_after
            .try_into()
            .map_err(|_| RpcError::InvalidNumericValue("afk_after"))?;
        Ok(Duration::from_secs(mins * 60))
    }
}

pub(crate) fn run(args: &Cli) -> Result<(), RpcError> {
    let mut client = rpc::connect(AFK_CLIENT_ID)?;
    let threshold = get_threshold(args)?;
    print_banner(threshold.as_secs(), args.disable_color);

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

    let mut monitor = IdleMonitor::new(threshold);
    let mut is_afk_active = false;

    loop {
        thread::sleep(Duration::from_secs(update_secs));

        match monitor.check_idle(threshold) {
            IdleState::Idle(elapsed) => {
                let seconds = elapsed.as_secs();
                let minutes = seconds / 60;
                let details = format_inactive_message(seconds, minutes);
                let _ = client.set_activity(activity_button.clone().details(&details));
                is_afk_active = true;
            }
            IdleState::Active => {
                if is_afk_active {
                    let _ = rpc::clear_activity(&mut client);
                    is_afk_active = false;
                }
            }
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

    #[test]
    fn threshold_prefers_explicit_seconds() {
        let mut args = crate::cli::test_cli();
        args.afk_seconds = Some(10);
        args.afk_after = 5;
        assert_eq!(get_threshold(&args).unwrap(), Duration::from_secs(10));
    }

    #[test]
    fn threshold_falls_back_to_minutes() {
        let mut args = crate::cli::test_cli();
        args.afk_after = 5;
        assert_eq!(get_threshold(&args).unwrap(), Duration::from_secs(300));

        args.afk_after = 0;
        assert_eq!(get_threshold(&args).unwrap(), Duration::from_secs(0));
    }
}
