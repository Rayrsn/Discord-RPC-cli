use std::{process::exit, time::{SystemTime,UNIX_EPOCH, Duration}, thread};
use clap::Parser;
use discord_rich_presence::{activity::{self, Party,Secrets}, DiscordIpcClient, DiscordIpc};
use colored::*;
use user_idle::UserIdle;

mod cli;
mod error;
mod validate;

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

    let state = args.state;
    let details = args.details;
    let large_image = args.large_image;
    let large_text = args.large_text;
    let small_image = args.small_image;
    let small_text = args.small_text;
    let button_text_1 = args.button_text_1;
    let button_url_1 = args.button_url_1;
    let button_text_2 = args.button_text_2;
    let button_url_2 = args.button_url_2;
    let party_id = args.party_id;
    let match_id = args.match_id;
    let join_id = args.join_id;
    let spectate_id = args.spectate_id;
    let enable_time = args.enable_time;
    let mut activity = activity::Activity::new();


    match client.connect() {
        Ok(_) => {println!("Client connected to Discord successfully.");},
        Err(_) => {println!("Client failed to connect to Discord, Please try again or relaunch Discord."); exit(1)},
    };

        if let Some(state) = &state {
            activity = activity.state(state);
        }
        if let Some(details) = &details {
            activity = activity.details(details);
        }
        // Build assets incrementally
        let mut assets = activity::Assets::new();
        if let Some(image) = &large_image {
            assets = assets.large_image(image);
        }
        if let Some(text) = &large_text {
            assets = assets.large_text(text);
        }
        if let Some(image) = &small_image {
            assets = assets.small_image(image);
        }
        if let Some(text) = &small_text {
            assets = assets.small_text(text);
        }
        if large_image.is_some() || large_text.is_some() || small_image.is_some() || small_text.is_some() {
            activity = activity.assets(assets);
        }
        let mut buttons = Vec::new();
        if let (Some(text), Some(url)) = (&button_text_1, &button_url_1) {
            if !url.is_empty() {
                buttons.push(activity::Button::new(text, url));
            }
        }
        if let (Some(text), Some(url)) = (&button_text_2, &button_url_2) {
            if !url.is_empty() {
                buttons.push(activity::Button::new(text, url));
            }
        }
        if !buttons.is_empty() {
            activity = activity.buttons(buttons);
        }
        if enable_time || args.start_time.is_some() || args.end_time.is_some() {
            let mut timestamps = activity::Timestamps::new();
            if enable_time {
                let time_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
                timestamps = timestamps.start(time_unix);
            }
            if let Some(start) = args.start_time {
                timestamps = timestamps.start(start);
            }
            if let Some(end) = args.end_time {
                timestamps = timestamps.end(end);
            }
            activity = activity.timestamps(timestamps);
        }
        if let Some(size) = &args.party_size {
            let (current_size, max_size) = validate::parse_party_size(size);
            let mut party = Party::new().size([current_size, max_size]);
            if let Some(id) = &party_id {
                party = party.id(id);
            }
            activity = activity.party(party);
        }
        if let Some(match_id) = validate::non_empty(&match_id) {
            let mut secrets = Secrets::new().r#match(match_id);
            if let Some(join_id) = validate::non_empty(&join_id) {
                secrets = secrets.join(join_id);
            }
            if let Some(spectate_id) = validate::non_empty(&spectate_id) {
                secrets = secrets.spectate(spectate_id);
            }
            activity = activity.secrets(secrets);
        }
    match client.set_activity(activity) {
        Ok(_) => {println!("Client set activity successfully.");},
        Err(_) => {println!("Client failed to set activity, Please try again or relaunch Discord."); exit(1)},
        };
    
    
    if args.disable_color == false {
        if let Some(state) = &state {println!("{}{}","State : ".magenta(),state.blue());}
        if let Some(details) = &details {println!("{}{}","Detail : ".magenta(),details.blue());}
        if let Some(large_image) = &large_image {println!("{}{}","Large Image name : ".magenta(),large_image.blue());}
        if let Some(large_text) = &large_text {println!("{}{}","Large Image text : ".magenta(),large_text.blue());}
        if let Some(small_image) = &small_image {println!("{}{}","Small Image name : ".magenta(),small_image.blue());}
        if let Some(small_text) = &small_text {println!("{}{}","Small Image text : ".magenta(),small_text.blue());}
        if let Some(button_text_1) = &button_text_1 {println!("{}{}","Button 1 Text: ".magenta(),button_text_1.blue());}
        if let Some(button_url_1) = &button_url_1 {println!("{}{}","Button 1 URL: ".magenta(),button_url_1.blue());}
        if let Some(button_text_2) = &button_text_2 {println!("{}{}","Button 2 Text: ".magenta(),button_text_2.blue());}
        if let Some(button_url_2) = &button_url_2 {println!("{}{}","Button 2 URL: ".magenta(),button_url_2.blue());}
        if let Some(start_time) = args.start_time {println!("{}{}","Start time is set to: ".magenta(),start_time.to_string().blue());}
        if let Some(end_time) = args.end_time {println!("{}{}","End time is set to: ".magenta(),end_time.to_string().blue());}
        if let Some(party_size) = &args.party_size {println!("{}{}","Party Size: ".magenta(),party_size.blue());}
        if let Some(party_id) = &party_id {println!("{}{}","Party ID: ".magenta(),party_id.blue());}
        if let Some(match_id) = &match_id {println!("{}{}","Match ID: ".magenta(),match_id.blue());}
        if let Some(join_id) = &join_id {println!("{}{}","Match Join ID: ".magenta(),join_id.blue());}
        if let Some(spectate_id) = &spectate_id {println!("{}{}","Match Spectate ID: ".magenta(),spectate_id.blue());}
        if enable_time == true {println!("{}","Time is Enabled".blue());}
    } else {
        if let Some(state) = &state {println!("State : {}",state);}
        if let Some(details) = &details {println!("Detail : {}",details);}
        if let Some(large_image) = &large_image {println!("Large Image name : {}",large_image);}
        if let Some(large_text) = &large_text {println!("Large Image text : {}",large_text);}
        if let Some(small_image) = &small_image {println!("Small Image name : {}",small_image);}
        if let Some(small_text) = &small_text {println!("Small Image text : {}",small_text);}
        if let Some(button_text_1) = &button_text_1 {println!("Button 1 Text: {}",button_text_1);}
        if let Some(button_url_1) = &button_url_1 {println!("Button 1 URL: {}",button_url_1);}
        if let Some(button_text_2) = &button_text_2 {println!("Button 2 Text: {}",button_text_2);}
        if let Some(button_url_2) = &button_url_2 {println!("Button 2 URL: {}",button_url_2);}
        if let Some(start_time) = args.start_time {println!("{}{}","Start time is set to: ",start_time.to_string());}
        if let Some(end_time) = args.end_time {println!("{}{}","End time is set to: ",end_time.to_string());}
        if let Some(party_size) = &args.party_size {println!("{}{}","Party Size: ",party_size);}
        if let Some(party_id) = &party_id {println!("{}{}","Party ID: ",party_id);}
        if enable_time == true {println!("Time is Enabled");}
        }
    
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

        let activity = activity::Activity::new();
        let activity_state = activity.state("Currently Inactive");
        let activity_large_image = activity_state.assets(activity::Assets::new().large_image("idle").large_text("Inactive"));
        let activity_button = activity_large_image.buttons(vec![activity::Button::new("Check out the Github!", "https://github.com/Rayrsn/Discord-RPC-cli")]);

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