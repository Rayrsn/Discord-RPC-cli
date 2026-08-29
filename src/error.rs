use std::fmt;

use colored::Colorize;

#[derive(Debug)]
pub(crate) enum RpcError {
    MissingClientId,
    ButtonOrder {
        button_text_1: Option<String>,
        button_url_1: Option<String>,
        button_text_2: Option<String>,
        button_url_2: Option<String>,
    },
    TimeConflict,
    PartyIdWithoutSize,
    MatchIdMissing,
    InvalidPartySize,
    ConnectFailed,
    SetActivityFailed,
    InvalidNumericValue(&'static str),
    IdleDetectionFailed(String),
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
            RpcError::TimeConflict => write!(
                f,
                "{prefix}{}",
                "Start time and End time cannot be set while enable_time is true."
                    .red()
                    .bold()
            ),
            RpcError::ButtonOrder {
                button_text_1,
                button_url_1,
                button_text_2,
                button_url_2,
            } => write!(
                f,
                "{prefix}Replace button 2 with button 1.\nbutton_text_1: {} button_url_1: {} button_text_2: {} button_url_2: {}",
                display_value(button_text_1),
                display_value(button_url_1),
                display_value(button_text_2),
                display_value(button_url_2)
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
            RpcError::ConnectFailed => {
                write!(
                    f,
                    "Client failed to connect to Discord, Please try again or relaunch Discord."
                )
            }
            RpcError::SetActivityFailed => {
                write!(
                    f,
                    "Client failed to set activity, Please try again or relaunch Discord."
                )
            }
            RpcError::InvalidNumericValue(name) => write!(f, "{prefix}Invalid {name} value"),
            RpcError::IdleDetectionFailed(err) => {
                write!(f, "{prefix}Failed to get user idle time: {err}")
            }
        }
    }
}

/// Renders absent fields as `__None` in error output, matching the historical
/// sentinel-based messages.
fn display_value(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("__None")
}
