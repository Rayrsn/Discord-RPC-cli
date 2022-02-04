use discord_rich_presence::{activity, new_client, DiscordIpc};
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
    println!("Client ID: {}", client.get_client_id());
    println!("{}", args.large_image);
    client.connect()?;
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
    // if state and details and large_image and large_text and small_image and small_text and button_text_1 and button_url_1 are not empty, then set them
    if state != "" && details != "" && large_image != "" && large_text != "" && small_image != "" && small_text != ""{
        let activity = activity::Activity::new()
        .state(state)
        .details(details)
        .assets(
            activity::Assets::new()
                .large_image(large_image)
                .large_text(large_text)
                .small_image(small_image)
                .small_text(small_text),
        )
        .buttons(vec![activity::Button::new(
            button_text_1,
            button_url_1,
        )]);
    client.set_activity(activity)?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    client.close()?;
    }
    // if state and details and large_image and large_text and small_image and small_text but not button_text_1 and button_url_1 are not empty, then set them
    if state != "" && details != "" && large_image != "" && large_text != "" && small_image != "" && small_text != "" && button_text_1 == "" && button_url_1 == ""{
        let activity = activity::Activity::new()
        .state(state)
        .details(details)
        .assets(
            activity::Assets::new()
                .large_image(large_image)
                .large_text(large_text)
                .small_image(small_image)
                .small_text(small_text),
        );
    Ok(())
}
