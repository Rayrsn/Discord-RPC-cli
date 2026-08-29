use std::{process::exit, time::Duration, thread};
use clap::Parser;
use discord_rich_presence::{activity::{Activity, Assets, Button}, DiscordIpcClient, DiscordIpc};
use colored::*;
use user_idle::UserIdle;

mod cli;
mod error;
mod validate;
mod activity;
mod display;

fn main() {
    let args = cli::Cli::parse();
    let afk_rpc = args.afk_rpc;
    let afk_after = args.afk_after;
    let afk_update = args.afk_update;

    if !afk_rpc {
    if let Err(err) = validate::validate(&args) {
        eprintln!("{err}");
        exit(err.exit_code());
    }

    let mut client = DiscordIpcClient::new(args.clientid.as_deref().expect("Client id is validated before connecting")).expect("Failed to create client");



    match client.connect() {
        Ok(_) => {println!("Client connected to Discord successfully.");},
        Err(_) => {println!("Client failed to connect to Discord, Please try again or relaunch Discord."); exit(1)},
    };

    let activity = activity::build_activity(&args);
    match client.set_activity(activity) {
        Ok(_) => {println!("Client set activity successfully.");},
        Err(_) => {println!("Client failed to set activity, Please try again or relaunch Discord."); exit(1)},
        };

    display::print_summary(&args);
    
    if let Some(exit_after) = args.exit_after {
        if args.disable_color == false {
            println!("{}{}{}","Exiting in ".blue(),exit_after.to_string().red()," seconds".blue());}
        else {
            println!("Exiting in {} seconds",exit_after.to_string());}

        thread::sleep(Duration::from_secs(exit_after.try_into().unwrap_or_else(|_| {
            println!("{}{}","error: ".red().bold(),"Invalid exit_after value");
            exit(1);
        })));
        exit(0)
    } else {
        if args.disable_color == false {
            println!("{}","Running indefinitely, Press Ctrl+C to exit.".cyan());}
        else {
            println!("Running indefinitely, Press Ctrl+C to exit.");}
        loop {thread::sleep(Duration::from_secs(10));}} // didn't have any better idea to do this
    }
    if afk_rpc {
        let mut client = DiscordIpcClient::new("871404899915665438").expect("Failed to create client");
        match client.connect() {
            Ok(_) => {println!("Client connected to Discord successfully.");},
            Err(_) => {println!("Client failed to connect to Discord, Please try again or relaunch Discord."); exit(1)},
        };

        if args.disable_color == false {
            println!("{}{}", "AFK RPC".magenta(), " is enabled.");
            if afk_after == 1 {
                println!("{}{}{}", "Will AFK after ",afk_after.to_string().magenta(), " minute.");
            } else if afk_after == 0 {
                println!("{}{}", "Will AFK ","immediately".magenta());
            } else {
                println!("{}{}{}", "Will AFK after ",afk_after.to_string().magenta(), " minutes.");
            }
        } else {
            println!("AFK RPC is enabled.");
            if afk_after == 1 {
                println!("{}{}{}", "Will AFK after ",afk_after.to_string(), " minute.");
            } else if afk_after == 0 {
                println!("{}", "Will AFK immediately.");
            } else {
                println!("{}{}{}", "Will AFK after ",afk_after.to_string(), " minutes.");
            }
        }

        let activity = Activity::new();
        let activity_state = activity.state("Currently Inactive");
        let activity_large_image = activity_state.assets(Assets::new().large_image("idle").large_text("Inactive"));
        let activity_button = activity_large_image.buttons(vec![Button::new("Check out the Github!", "https://github.com/Rayrsn/Discord-RPC-cli")]);

        let mut connected = false;
        
        loop {
            thread::sleep(Duration::from_secs(afk_update.try_into().unwrap_or_else(|_| {
                println!("{}{}","error: ".red().bold(),"Invalid afk_update value");
                exit(1);
            })));
            let idle = UserIdle::get_time().expect("Failed to get idle time");

            if idle.as_minutes() >= afk_after.try_into().unwrap_or_else(|_| {
                println!("{}{}","error: ".red().bold(),"Invalid afk_after value");
                exit(1);
            }) && connected == false {
                client.connect().expect("Failed to connect to Discord");        
                
                match idle.as_seconds() {
                    0..=59 => client.set_activity(activity_button.clone().details(format!("I've been inactive for {} seconds", idle.as_seconds()).as_str())),
                    60..=119 => client.set_activity(activity_button.clone().details("I've been inactive for a minute")),
                    _ => client.set_activity(activity_button.clone().details(format!("I've been inactive for {} minutes", idle.as_minutes()).as_str())),
                    
                    }.expect("Failed to set activity");
                connected = true;
            } 

            if connected == true && idle.as_minutes() < afk_after.try_into().unwrap_or_else(|_| {
                println!("{}{}","error: ".red().bold(),"Invalid afk_after value");
                exit(1);
            }){
                client.clear_activity().expect("Failed to clear activity");
                connected = false;
                };
            }
    }
}