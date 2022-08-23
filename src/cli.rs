use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(short = 'c', long = "clientid",help = "Your application's client id (REQUIRED if AFK is off)" ,required = false, default_value="__None",display_order = 1)]
    pub clientid: String,

    #[clap(short = 'd', long = "details",help = "Your desired details string (optional)",required = false,default_value="__None",display_order=2)]
    pub details: String,

    #[clap(short = 's', long = "state",help = "Your desired state string (optional)" ,required = false,default_value="__None",display_order=3)]
    pub state: String,

    #[clap(short = 'N', long = "large_image",help = "The name of your large image (optional)",required = false,default_value="__None",display_order=4)]
    pub large_image: String,

    #[clap(short = 'I', long = "large_image_text",help = "The text shown on your large image (optional)",required = false,default_value="__None",display_order=5)]
    pub large_text: String,

    #[clap(short = 'n', long = "small_image",help = "The name of your small image (optional)",required = false,default_value="__None",display_order=6)]
    pub small_image: String,

    #[clap(short = 'i', long = "small_image_text",help = "The text shown on your small image (optional)",required = false,default_value="__None",display_order=7)]
    pub small_text: String,

    #[clap(short = 'U', long = "button_url_1",help = "The url of your first button (optional)",required = false,default_value="__None",display_order=8)]
    pub button_url_1: String,

    #[clap(short = 'B', long = "button_text_1",help = "The text shown on your first button (optional)",required = false,default_value="__None",display_order=9)]
    pub button_text_1: String,

    #[clap(short = 'u', long = "button_url_2",help = "The url of your second button (optional)",required = false,default_value="__None",display_order=10)]
    pub button_url_2: String,

    #[clap(short = 'b', long = "button_text_2",help = "The text shown on your second button (optional)",required = false,default_value="__None",display_order=11)]
    pub button_text_2: String,

    #[clap(short = 'S', long = "start_time",help = "Set the start time (Unix time) (optional)",default_value="-1",display_order=12)]
    pub start_time: i64,

    #[clap(short = 'E', long = "end_time",help = "Set the end time (Unix time) (optional)",default_value="-1",display_order=13)]
    pub end_time: i64,

    #[clap(short = 'P', long = "party_size",help = "Creates a party with a current size of _ and a max size of _ (Example: [1,10]) (optional)",default_value="__None",display_order=14)]
    pub party_size: String,

    #[clap(short = 'p', long = "party_id",help = "Sets the ID of the party (Has to be used with party_size) (optional)",default_value="__None",display_order=15)]
    pub party_id: String,

    #[clap(short = 'm', long = "match_id",help = "Sets the ID of the match (Can't be used with buttons) (optional)",default_value="__None",display_order=16)]
    pub match_id: String,

    #[clap(short = 'j', long = "join_id",help = "Sets the join ID of the match (Has to be used with match_id) (optional)",default_value="__None",display_order=17)]
    pub join_id: String,

    #[clap(short = 'y', long = "spectate_id",help = "Sets the spectate ID of the match (Has to be used with match_id) (optional)",default_value="__None",display_order=18)]
    pub spectate_id: String,

    #[clap(short = 't', long = "enable_time",help = "Whether to enable time or not (will count from current time) (optional)",display_order=19)]
    pub enable_time: bool, 

    #[clap(short = 'a', long = "afk",help = "Whether to enable AFK RPC or not) (optional)",display_order=20)]
    pub afk_rpc: bool, 

    #[clap(short = 'f', long = "afk_after",help = "How many seconds should pass after the AFK RPC is started [In Seconds] (optional)",default_value="5",display_order=21)]
    pub afk_after: i64,
    
    #[clap(short = 'k', long = "afk_update",help = "How often to check wether the user is idle or not [In Seconds](optional)",default_value="20",display_order=22)]
    pub afk_update: i64,

    #[clap(short = 'e', long = "exit_after",help = "Exit after a given time (optional)",default_value="-1",display_order=23)]
    pub exit_after: i64,

    #[clap(short = 'C', long = "disable_color",help = "Whether to disable colors or not (optional)",display_order=24)]
    pub disable_color: bool,
}