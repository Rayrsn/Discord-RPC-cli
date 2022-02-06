use std::{process::exit};
use discord_rich_presence::{activity::{self, Activity}, new_client, DiscordIpc};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short = 'c', long = "clientid",help = "Your application's client id (REQUIRED)" ,required = true,display_order = 1)]
    clientid: String,

    #[clap(short = 's', long = "state",help = "Your desired state string (optional)" ,required = false,default_value="__None",display_order=2)]
    state: String,

    #[clap(short = 'd', long = "details",help = "Your desired details string (optional)",required = false,default_value="__None",display_order=3)]
    details: String,

    #[clap(short = 'I', long = "large_image",help = "The name of your large image (optional)",required = false,display_order=4)]
    large_image: String,

    #[clap(short = 'N', long = "large_image_name",help = "The text shown on your large image (optional)",required = false,default_value="__None",display_order=5)]
    large_text: String,

    #[clap(short = 'i', long = "small_image",help = "The name of your small image (optional)",required = false,default_value="__None",display_order=6)]
    small_image: String,
    
    #[clap(short = 'n', long = "small_image_name",help = "The text shown on your small image (optional)",required = false,default_value="__None",display_order=7)]
    small_text: String,

    #[clap(long = "button_url_1",help = "The url of your first button (optional)",required = false,default_value="__None",display_order=8)]
    button_url_1: String,

    #[clap(long = "button_text_1",help = "The text shown on your first button (optional)",required = false,default_value="__None",display_order=9)]
    button_text_1: String,

    #[clap(long = "button_url_2",help = "The url of your second button (optional)",required = false,default_value="__None",display_order=10)]
    button_url_2: String,

    #[clap(long = "button_text_2",help = "The text shown on your second button (optional)",required = false,default_value="__None",display_order=11)]
    button_text_2: String,

    #[clap(short = 't', long = "enable_time",help = "Whether to enable time or not (will count from current time, expects boolean) (optional)",display_order=12)]
    enable_time: bool,    
}
fn check_state()->String{
    let args = Cli::parse();
    if args.state != "__None"{
        return args.state;
    } else {
        return "".to_string();
    }
}
fn check_details()->String{
    let args = Cli::parse();
    if args.details != "__None"{
        return args.details;
    } else {
        return "".to_string();
    }
}
fn check_button_text_1()->String{
    let args = Cli::parse();
    if args.button_text_1 != "__None"{
        return args.button_text_1;
    } else {
        return "".to_string();
    }
}
fn check_button_url_1()->String{
    let args = Cli::parse();
    if args.button_url_1 != "__None"{
        return args.button_url_1;
    } else {
        return "".to_string();
    }
}
fn check_button_text_2()->String{
    let args = Cli::parse();
    if args.button_text_2 != "__None"{
        return args.button_text_2;
    } else {
        return "".to_string();
    }
}
fn check_button_url_2()->String{
    let args = Cli::parse();
    if args.button_url_2 != "__None"{
        return args.button_url_2;
    } else {
        return "".to_string();
    }
}
fn check_large_image()->String{
    let args = Cli::parse();
    if args.large_image != "__None"{
        return args.large_image;
    } else {
        return "".to_string();
    }
}
fn check_large_text()->String{
    let args = Cli::parse();
    if args.large_text != "__None"{
        return args.large_text;
    } else {
        return "".to_string();
    }
}
fn check_small_image()->String{
    let args = Cli::parse();
    if args.small_image != "__None"{
        return args.small_image;
    } else {
        return "".to_string();
    }
}
fn check_small_text()->String{
    let args = Cli::parse();
    if args.small_text != "__None"{
        return args.small_text;
    } else {
        return "".to_string();
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let mut client = new_client(&args.clientid)?;
    let state = &check_state();
    let details = &check_details();
    let large_image = &check_large_image();
    let large_text = &check_large_text();
    let small_image = &check_small_image();
    let small_text = &check_small_text();
    let button_text_1 = &check_button_text_1();
    let button_url_1 = &check_button_url_1();
    let button_text_2 = &check_button_text_2();
    let button_url_2 = &check_button_url_2();
    let activity = activity::Activity::new();
    if (button_text_1 == "" && button_text_2 !="") || (button_url_1 == "" && button_url_2 !=""){
        println!("Replace button 2 with button 1.");
        exit(1)
    }
    match client.connect() {
        Ok(_) => {println!("Client connected to Discord successfully.");},
        Err(_) => {println!("Client failed to connect to Discord, Please try again or relaunch Discord."); exit(1)},
    };
    let activity_state:Activity = if state != "" {
        activity.state(state).clone()
    } else {
        activity
    };
    let activity_details:Activity = if details != "" {
        activity_state.details(details).clone()
    } else {
        activity_state
    };
    let activity_large_image:Activity = if large_image != "" {
        activity_details.assets(activity::Assets::new().large_image(large_image)).clone()
    } else {
        activity_details
    };
    let activity_large_text:Activity = if large_text != "" {
        activity_large_image.assets(activity::Assets::new().large_image(large_image).large_text(large_text)).clone()
    } else {
        activity_large_image
    };
    let activity_small_image:Activity = if small_image != "" {
        activity_large_text.assets(activity::Assets::new().large_image(large_image).large_text(large_text).small_image(small_image)).clone()
    } else {
        activity_large_text
    };
    let activity_small_text:Activity = if small_text != "" {
        activity_small_image.assets(activity::Assets::new().large_image(large_image).large_text(large_text).small_image(small_image).small_text(small_text)).clone()
    } else {
        activity_small_image
    };
    let activity_button_1:Activity = if button_text_1 != "" && button_url_1 !="" {
        activity_small_text.buttons(vec![activity::Button::new(button_text_1,button_url_1)]).clone()
    } else {
        activity_small_text
    };
    let activity_button_2:Activity = if button_text_2 != "" && button_url_2 !="" {
        activity_button_1.buttons(vec![activity::Button::new(button_text_1,button_url_1),activity::Button::new(button_text_2,button_url_2)]).clone()
    } else {
        activity_button_1
    };
    
    client.set_activity(activity_button_2)?;
    println!("State is: {}",state);
    println!("Detail is: {}",details);
    std::thread::sleep(std::time::Duration::from_secs(5));
    client.close()?;
    Ok(())
}