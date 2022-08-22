use std::{process::exit, time::{SystemTime,UNIX_EPOCH, Duration}, thread};
use clap::StructOpt;
use discord_rich_presence::{activity::{self, Activity,Party,Secrets}, DiscordIpcClient, DiscordIpc};
use colored::*;
mod cli;

fn check_current_party_size()->String{
    let args = cli::Cli::parse();
    if args.party_size != "__None"{
        args.party_size.trim_matches('[').trim_matches(']').split(",").nth(0).unwrap().to_string()
    } else {
        return "".to_string();
    }
}
fn check_max_party_size()->String{
    let args = cli::Cli::parse();
    if args.party_size != "__None"{
        args.party_size.trim_matches('[').trim_matches(']').split(",").nth(1).unwrap().to_string()
    } else {
        return "".to_string();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();
    let mut client = DiscordIpcClient::new(&args.clientid)?;
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
    let current_party_size = &check_current_party_size();
    let max_party_size = &check_max_party_size();
    let party_id = args.party_id;
    let match_id = args.match_id;
    let join_id = args.join_id;
    let spectate_id = args.spectate_id;
    let enable_time = args.enable_time;
    let activity = activity::Activity::new();

    if (button_text_1 == "" && button_text_2 !="") || (button_url_1 == "" && button_url_2 !=""){
        println!("Replace button 2 with button 1.");
        exit(1)
    }
    if (enable_time == true && args.start_time != -1) || (enable_time == true && args.end_time != -1) {
        println!("Start time and End time cannot be set while enable_time is true.");
        exit(1)
    }

    if party_id != "__None" && args.party_size == "__None"{
        println!("party_id has to be run with a party_size.");
        exit(1)
    }
    if (match_id == "" && join_id != "__None") || (match_id == "" && spectate_id != "__None") {
        println!("match_id is not specified.");
        exit(1)
    }


    match client.connect() {
        Ok(_) => {println!("Client connected to Discord successfully.");},
        Err(_) => {println!("Client failed to connect to Discord, Please try again or relaunch Discord."); exit(1)},
    };

    let activity_state:Activity = if state != "__None" {
        activity.state(&state).clone()
    } else {
        activity
    };
    let activity_details:Activity = if details != "__None" {
        activity_state.details(&details).clone()
    } else {
        activity_state
    };
    let activity_large_image:Activity = if large_image != "__None" {
        activity_details.assets(activity::Assets::new().large_image(&large_image)).clone()
    } else {
        activity_details
    };
    let activity_large_text:Activity = if large_text != "__None" {
        activity_large_image.assets(activity::Assets::new().large_image(&large_image).large_text(&large_text)).clone()
    } else {
        activity_large_image
    };
    let activity_small_image:Activity = if small_image != "__None" {
        activity_large_text.assets(activity::Assets::new().large_image(&large_image).large_text(&large_text).small_image(&small_image)).clone()
    } else {
        activity_large_text
    };
    let activity_small_text:Activity = if small_text != "__None" {
        activity_small_image.assets(activity::Assets::new().large_image(&large_image).large_text(&large_text).small_image(&small_image).small_text(&small_text)).clone()
    } else {
        activity_small_image
    };
    let activity_button_1:Activity = if button_text_1 != "__None" && button_url_1 !="" {
        activity_small_text.buttons(vec![activity::Button::new(&button_text_1,&button_url_1)]).clone()
    } else {
        activity_small_text
    };
    let activity_button_2:Activity = if button_text_2 != "__None" && button_url_2 !="" {
        activity_button_1.buttons(vec![activity::Button::new(&button_text_1,&button_url_1),activity::Button::new(&button_text_2,&button_url_2)]).clone()
    } else {
        activity_button_1
    };
    let time_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let activity_time:Activity = if enable_time == true {
        activity_button_2.timestamps(activity::Timestamps::new().start(time_unix)).clone()
    } else {
        activity_button_2
    };
    let activity_start_time:Activity = if args.start_time != -1 {
        activity_time.timestamps(activity::Timestamps::new().start(args.start_time)).clone()
    } else {
        activity_time
    };
    let activity_end_time:Activity = if args.end_time != -1 {
        activity_start_time.timestamps(activity::Timestamps::new().end(args.end_time)).clone()
    } else {
        activity_start_time
    };
    let activity_party_size:Activity = if args.party_size != "__None" {
        activity_end_time.party(Party::new().size([current_party_size.parse::<i32>().unwrap(), max_party_size.parse::<i32>().unwrap()]))
    } else {
        activity_end_time
    };
    let activity_party_id:Activity = if party_id != "__None" {
        activity_party_size.party(Party::new().size([current_party_size.parse::<i32>().unwrap(), max_party_size.parse::<i32>().unwrap()]).id(&party_id))
    } else {
        activity_party_size
    };
    let activity_match_id:Activity = if match_id != "__None" && join_id == "" && spectate_id == "" {
        activity_party_id.secrets(Secrets::new().r#match(&match_id))
    } else {
        activity_party_id
    };
    let activity_join_id:Activity = if match_id != "__None" && join_id != "__None" && spectate_id == "" {
        activity_match_id.secrets(Secrets::new().r#match(&match_id).join(&join_id))
    } else {
        activity_match_id
    };
    let activity_spectate_id:Activity = if match_id != "__None" && join_id == "" && spectate_id != "__None" {
        activity_join_id.secrets(Secrets::new().r#match(&match_id).spectate(&spectate_id))
    } else {
        activity_join_id
    };
    let activity_mjs_id:Activity = if match_id != "__None" && join_id != "__None" && spectate_id != "__None" {
        activity_spectate_id.secrets(Secrets::new().r#match(&match_id).spectate(&spectate_id).join(&join_id))
    } else {
        activity_spectate_id
    };

    let _activity_init:Activity = activity_mjs_id;
    match client.set_activity(_activity_init) {
        Ok(_) => {println!("Client set activity successfully.");},
        Err(_) => {println!("Client failed to set activity, Please try again or relaunch Discord."); exit(1)},
    };
    if args.disable_color == false {
        if state != "__None"{println!("{}{}","State : ".magenta(),state.blue());}
        if details != "__None"{println!("{}{}","Detail : ".magenta(),details.blue());}
        if large_image != "__None"{println!("{}{}","Large Image name : ".magenta(),large_image.blue());}
        if large_text != "__None"{println!("{}{}","Large Image text : ".magenta(),large_text.blue());}
        if small_image != "__None"{println!("{}{}","Small Image name : ".magenta(),small_image.blue());}
        if small_text != "__None"{println!("{}{}","Small Image text : ".magenta(),small_text.blue());}
        if button_text_1 != "__None" {println!("{}{}","Button 1 Text: ".magenta(),button_text_1.blue());}
        if button_url_1 != "__None" {println!("{}{}","Button 1 URL: ".magenta(),button_url_1.blue());}
        if button_text_2 != "__None" {println!("{}{}","Button 2 Text: ".magenta(),button_text_2.blue());}
        if button_url_2 != "__None" {println!("{}{}","Button 2 URL: ".magenta(),button_url_2.blue());}
        if args.start_time != -1 {println!("{}{}","Start time is set to: ".magenta(),args.start_time.to_string().blue());}
        if args.end_time != -1 {println!("{}{}","End time is set to: ".magenta(),args.end_time.to_string().blue());}
        if args.party_size != "__None" {println!("{}{}","Party Size: ".magenta(),args.party_size.blue());}
        if party_id != "__None" {println!("{}{}","Party ID: ".magenta(),party_id.to_string().blue());}
        if match_id != "__None" {println!("{}{}","Match ID: ".magenta(),match_id.blue());}
        if join_id != "__None" {println!("{}{}","Match Join ID: ".magenta(),join_id.blue());}
        if spectate_id != "__None" {println!("{}{}","Match Spectate ID: ".magenta(),spectate_id.blue());}
        if enable_time == true {println!("{}","Time is Enabled".blue());}
    } else {
        if state != "__None"{println!("State : {}",state);}
        if details != "__None"{println!("Detail : {}",details);}
        if large_image != "__None"{println!("Large Image name : {}",large_image);}
        if large_text != "__None"{println!("Large Image text : {}",large_text);}
        if small_image != "__None"{println!("Small Image name : {}",small_image);}
        if small_text != "__None"{println!("Small Image text : {}",small_text);}
        if button_text_1 != "__None" {println!("Button 1 Text: {}",button_text_1);}
        if button_url_1 != "__None" {println!("Button 1 URL: {}",button_url_1);}
        if button_text_2 != "__None" {println!("Button 2 Text: {}",button_text_2);}
        if button_url_2 != "__None" {println!("Button 2 URL: {}",button_url_2);}
        if args.start_time != -1 {println!("{}{}","Start time is set to: ",args.start_time.to_string());}
        if args.end_time != -1 {println!("{}{}","End time is set to: ",args.end_time.to_string());}
        if args.party_size != "__None" {println!("{}{}","Party Size: ",args.party_size);}
        if party_id != "__None" {println!("{}{}","Party ID: ",party_id.to_string());}
        if enable_time == true {println!("Time is Enabled");}
    }
    if args.exit_after != -1 {
        if args.disable_color == false {
            println!("{}{}{}","Exiting in ".blue(),args.exit_after.to_string().red()," seconds".blue());}
        else {
            println!("Exiting in {} seconds",args.exit_after.to_string());}

        thread::sleep(Duration::from_secs(args.exit_after.try_into().unwrap()));
        exit(0)
    } else {
        if args.disable_color == false {
            println!("{}","Running indefinitely, Press Ctrl+C to exit.".cyan());}
        else {
            println!("Running indefinitely, Press Ctrl+C to exit.");}
        loop {thread::sleep(Duration::from_secs(10));}} // didn't have any better idea to do this
}