use std::{process::exit, time::{SystemTime,UNIX_EPOCH, Duration}, thread};
use discord_rich_presence::{activity::{self, Activity,Party,Secrets}, new_client, DiscordIpc};
use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Cli {
    #[clap(short = 'c', long = "clientid",help = "Your application's client id (REQUIRED)" ,required = true,display_order = 1)]
    clientid: String,

    #[clap(short = 's', long = "state",help = "Your desired state string (optional)" ,required = false,default_value="__None",display_order=2)]
    state: String,

    #[clap(short = 'd', long = "details",help = "Your desired details string (optional)",required = false,default_value="__None",display_order=3)]
    details: String,

    #[clap(short = 'N', long = "large_image",help = "The name of your large image (optional)",required = false,default_value="__None",display_order=4)]
    large_image: String,

    #[clap(short = 'I', long = "large_image_text",help = "The text shown on your large image (optional)",required = false,default_value="__None",display_order=5)]
    large_text: String,

    #[clap(short = 'n', long = "small_image",help = "The name of your small image (optional)",required = false,default_value="__None",display_order=6)]
    small_image: String,

    #[clap(short = 'i', long = "small_image_text",help = "The text shown on your small image (optional)",required = false,default_value="__None",display_order=7)]
    small_text: String,

    #[clap(short = 'U', long = "button_url_1",help = "The url of your first button (optional)",required = false,default_value="__None",display_order=8)]
    button_url_1: String,

    #[clap(short = 'B', long = "button_text_1",help = "The text shown on your first button (optional)",required = false,default_value="__None",display_order=9)]
    button_text_1: String,

    #[clap(short = 'u', long = "button_url_2",help = "The url of your second button (optional)",required = false,default_value="__None",display_order=10)]
    button_url_2: String,

    #[clap(short = 'b', long = "button_text_2",help = "The text shown on your second button (optional)",required = false,default_value="__None",display_order=11)]
    button_text_2: String,

    #[clap(short = 'S', long = "start_time",help = "Set the start time (Unix time) (optional)",default_value="-1",display_order=12)]
    start_time: i64,

    #[clap(short = 'E', long = "end_time",help = "Set the end time (Unix time) (optional)",default_value="-1",display_order=13)]
    end_time: i64,

    #[clap(short = 'P', long = "party_size",help = "Creates a party with a current size of _ and a max size of _ (Example: [1,10]) (optional)",default_value="__None",display_order=14)]
    party_size: String,

    #[clap(short = 'p', long = "party_id",help = "Sets the ID of the party (Has to be used with party_size) (optional)",default_value="__None",display_order=15)]
    party_id: String,

    #[clap(short = 'm', long = "match_id",help = "Sets the ID of the match (Can't be used with buttons) (optional)",default_value="__None",display_order=16)]
    match_id: String,

    #[clap(short = 'j', long = "join_id",help = "Sets the join ID of the match (Has to be used with match_id) (optional)",default_value="__None",display_order=17)]
    join_id: String,

    #[clap(short = 'y', long = "spectate_id",help = "Sets the spectate ID of the match (Has to be used with match_id) (optional)",default_value="__None",display_order=18)]
    spectate_id: String,

    #[clap(short = 't', long = "enable_time",help = "Whether to enable time or not (will count from current time) (optional)",display_order=19)]
    enable_time: bool, 

    #[clap(short = 'e', long = "exit_after",help = "Exit after a given time (optional)",default_value="-1",display_order=20)]
    exit_after: i64,

    #[clap(short = 'C', long = "disable_color",help = "Whether to disable colors or not (optional)",display_order=21)]
    disable_color: bool,
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
fn check_start_time()->i64{
    let args = Cli::parse();
    if args.start_time != -1{
        return args.start_time;
    } else {
        return -1;
    }
}
fn check_end_time()->i64{
    let args = Cli::parse();
    if args.end_time != -1{
        return args.end_time;
    } else {
        return -1;
    }
}
fn check_time()->bool{
    let args = Cli::parse();
    if args.enable_time == true{
        return args.enable_time;
    } else {
        return false;
    }
}
fn check_current_party_size()->String{
    let args = Cli::parse();
    if args.party_size != "__None"{
        args.party_size.trim_matches('[').trim_matches(']').split(",").nth(0).unwrap().to_string()
    } else {
        return "".to_string();
    }
}
fn check_max_party_size()->String{
    let args = Cli::parse();
    if args.party_size != "__None"{
        args.party_size.trim_matches('[').trim_matches(']').split(",").nth(1).unwrap().to_string()
    } else {
        return "".to_string();
    }
}
fn check_party_id()->String{
    let args = Cli::parse();
    if args.party_id != "__None"{
        return args.party_id;
    } else {
        return "".to_string();
    }
}
fn check_match_id()->String{
    let args = Cli::parse();
    if args.match_id != "__None"{
        return args.match_id;
    } else {
        return "".to_string();
    }
}
fn check_join_id()->String{
    let args = Cli::parse();
    if args.join_id != "__None"{
        return args.join_id;
    } else {
        return "".to_string();
    }
}
fn check_spectate_id()->String{
    let args = Cli::parse();
    if args.spectate_id != "__None"{
        return args.spectate_id;
    } else {
        return "".to_string();
    }
}
//// functoion moonde
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
    let current_party_size = &check_current_party_size();
    let max_party_size = &check_max_party_size();
    let party_id = &check_party_id();
    let match_id = &check_match_id();
    let join_id = &check_join_id();
    let spectate_id = &check_spectate_id();
    let enable_time = check_time();
    let activity = activity::Activity::new();

    if (button_text_1 == "" && button_text_2 !="") || (button_url_1 == "" && button_url_2 !=""){
        println!("Replace button 2 with button 1.");
        exit(1)
    }
    if (enable_time == true && check_start_time() != -1) || (enable_time == true && check_end_time() != -1) {
        println!("Start time and End time cannot be set while enable_time is true.");
        exit(1)
    }

    if party_id != "" && args.party_size == "__None"{
        println!("party_id has to be run with a party_size.");
        exit(1)
    }
    if (match_id == "" && join_id != "") || (match_id == "" && spectate_id != "") {
        println!("match_id is not specified.");
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
    let time_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let activity_time:Activity = if enable_time == true {
        activity_button_2.timestamps(activity::Timestamps::new().start(time_unix)).clone()
    } else {
        activity_button_2
    };
    let activity_start_time:Activity = if check_start_time() != -1 {
        activity_time.timestamps(activity::Timestamps::new().start(check_start_time())).clone()
    } else {
        activity_time
    };
    let activity_end_time:Activity = if check_end_time() != -1 {
        activity_start_time.timestamps(activity::Timestamps::new().end(check_end_time())).clone()
    } else {
        activity_start_time
    };
    let activity_party_size:Activity = if args.party_size != "__None" {
        activity_end_time.party(Party::new().size([current_party_size.parse::<i32>().unwrap(), max_party_size.parse::<i32>().unwrap()]))
    } else {
        activity_end_time
    };
    let activity_party_id:Activity = if party_id != "" {
        activity_party_size.party(Party::new().size([current_party_size.parse::<i32>().unwrap(), max_party_size.parse::<i32>().unwrap()]).id(party_id))
    } else {
        activity_party_size
    };
    let activity_match_id:Activity = if match_id != "" && join_id == "" && spectate_id == "" {
        activity_party_id.secrets(Secrets::new().r#match(match_id))
    } else {
        activity_party_id
    };
    let activity_join_id:Activity = if match_id != "" && join_id != "" && spectate_id == "" {
        activity_match_id.secrets(Secrets::new().r#match(match_id).join(join_id))
    } else {
        activity_match_id
    };
    let activity_spectate_id:Activity = if match_id != "" && join_id == "" && spectate_id != "" {
        activity_join_id.secrets(Secrets::new().r#match(match_id).spectate(spectate_id))
    } else {
        activity_join_id
    };
    let activity_mjs_id:Activity = if match_id != "" && join_id != "" && spectate_id != "" {
        activity_spectate_id.secrets(Secrets::new().r#match(match_id).spectate(spectate_id).join(join_id))
    } else {
        activity_spectate_id
    };

    let _activity_init:Activity = activity_mjs_id;
    client.set_activity(_activity_init)?;
    if args.disable_color == false {
        if state != ""{println!("{}{}","State : ".magenta(),state.blue());}
        if details != ""{println!("{}{}","Detail : ".magenta(),details.blue());}
        if large_image != ""{println!("{}{}","Large Image name : ".magenta(),large_image.blue());}
        if large_text != ""{println!("{}{}","Large Image text : ".magenta(),large_text.blue());}
        if small_image != ""{println!("{}{}","Small Image name : ".magenta(),small_image.blue());}
        if small_text != ""{println!("{}{}","Small Image text : ".magenta(),small_text.blue());}
        if button_text_1 != "" {println!("{}{}","Button 1 Text: ".magenta(),button_text_1.blue());}
        if button_url_1 != "" {println!("{}{}","Button 1 URL: ".magenta(),button_url_1.blue());}
        if button_text_2 != "" {println!("{}{}","Button 2 Text: ".magenta(),button_text_2.blue());}
        if button_url_2 != "" {println!("{}{}","Button 2 URL: ".magenta(),button_url_2.blue());}
        if check_start_time() != -1 {println!("{}{}","Start time is set to: ".magenta(),check_start_time().to_string().blue());}
        if check_end_time() != -1 {println!("{}{}","End time is set to: ".magenta(),check_end_time().to_string().blue());}
        if args.party_size != "__None" {println!("{}{}","Party Size: ".magenta(),args.party_size.blue());}
        if party_id != "" {println!("{}{}","Party ID: ".magenta(),party_id.to_string().blue());}
        if match_id != "" {println!("{}{}","Match ID: ".magenta(),match_id.blue());}
        if join_id != "" {println!("{}{}","Match Join ID: ".magenta(),join_id.blue());}
        if spectate_id != "" {println!("{}{}","Match Spectate ID: ".magenta(),spectate_id.blue());}
        if enable_time == true {println!("{}","Time is Enabled".blue());}
    } else {
        if state != ""{println!("State : {}",state);}
        if details != ""{println!("Detail : {}",details);}
        if large_image != ""{println!("Large Image name : {}",large_image);}
        if large_text != ""{println!("Large Image text : {}",large_text);}
        if small_image != ""{println!("Small Image name : {}",small_image);}
        if small_text != ""{println!("Small Image text : {}",small_text);}
        if button_text_1 != "" {println!("Button 1 Text: {}",button_text_1);}
        if button_url_1 != "" {println!("Button 1 URL: {}",button_url_1);}
        if button_text_2 != "" {println!("Button 2 Text: {}",button_text_2);}
        if button_url_2 != "" {println!("Button 2 URL: {}",button_url_2);}
        if check_start_time() != -1 {println!("{}{}","Start time is set to: ",check_start_time().to_string());}
        if check_end_time() != -1 {println!("{}{}","End time is set to: ",check_end_time().to_string());}
        if args.party_size != "__None" {println!("{}{}","Party Size: ",args.party_size);}
        if party_id != "" {println!("{}{}","Party ID: ",party_id.to_string());}
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
        loop {}} // didn't have any better idea to do this
    
}
