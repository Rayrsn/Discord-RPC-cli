use std::fmt;

use colored::Colorize;

#[derive(Debug)]
pub(crate) enum RpcError {
    MissingClientId,
    ButtonOrder {
        button_text_1: String,
        button_url_1: String,
        button_text_2: String,
        button_url_2: String,
    },
    TimeConflict,
    PartyIdWithoutSize,
    MatchIdMissing,
    InvalidPartySize,
}

impl RpcError {
    pub(crate) fn exit_code(&self) -> i32 {
        1
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "error: ".red().bold();
        match self {
            RpcError::MissingClientId => write!(
                f,
                "{prefix}The following required argument was not provided: clientid\n\nUSAGE:\n    discord-rpc-cli [OPTIONS]\n\nFor more information try --help"
            ),
            RpcError::ButtonOrder {
                button_text_1,
                button_url_1,
                button_text_2,
                button_url_2,
            } => write!(
                f,
                "{prefix}Replace button 2 with button 1.\nbutton_text_1: {button_text_1} button_url_1: {button_url_1} button_text_2: {button_text_2} button_url_2: {button_url_2}"
            ),
            RpcError::TimeConflict => write!(
                f,
                "{prefix}{}",
                "Start time and End time cannot be set while enable_time is true.".red().bold()
            ),
            RpcError::PartyIdWithoutSize => write!(
                f,
                "{prefix}{}",
                "party_id has to be run with a party_size.".red().bold()
            ),
            RpcError::MatchIdMissing => write!(f, "{prefix}match_id is not specified."),
            RpcError::InvalidPartySize => write!(
                f,
                "{prefix}Invalid party size values. Current size and max size must be positive, and current size cannot exceed max size."
            ),
        }
    }
}
