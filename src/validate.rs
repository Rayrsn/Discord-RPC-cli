use crate::cli::Cli;
use crate::error::RpcError;

pub(crate) fn validate(args: &Cli) -> Result<(), RpcError> {
    if args.clientid == "__None" {
        return Err(RpcError::MissingClientId);
    }
    if (args.button_text_1 == "__None" && args.button_text_2 != "__None")
        || (args.button_url_1 == "__None" && args.button_url_2 != "__None")
    {
        return Err(RpcError::ButtonOrder {
            button_text_1: args.button_text_1.clone(),
            button_url_1: args.button_url_1.clone(),
            button_text_2: args.button_text_2.clone(),
            button_url_2: args.button_url_2.clone(),
        });
    }
    if args.enable_time && (args.start_time != -1 || args.end_time != -1) {
        return Err(RpcError::TimeConflict);
    }
    if args.party_id != "__None" && args.party_size == "__None" {
        return Err(RpcError::PartyIdWithoutSize);
    }
    if args.match_id == "__None" && (args.join_id != "__None" || args.spectate_id != "__None") {
        return Err(RpcError::MatchIdMissing);
    }
    if args.party_size != "__None" {
        let current_size = check_current_party_size(&args.party_size)
            .parse::<i32>()
            .unwrap_or(1);
        let max_size = check_max_party_size(&args.party_size)
            .parse::<i32>()
            .unwrap_or(1);
        if current_size < 1 || max_size < 1 || current_size > max_size {
            return Err(RpcError::InvalidPartySize);
        }
    }
    Ok(())
}

pub(crate) fn check_current_party_size(party_size: &str) -> String {
    if party_size != "__None" {
        party_size
            .trim_matches('[')
            .trim_matches(']')
            .split(",")
            .next()
            .unwrap_or("1")
            .to_string()
    } else {
        "1".to_string()
    }
}

pub(crate) fn check_max_party_size(party_size: &str) -> String {
    if party_size != "__None" {
        party_size
            .trim_matches('[')
            .trim_matches(']')
            .split(",")
            .nth(1)
            .unwrap_or("1")
            .to_string()
    } else {
        "1".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_args() -> Cli {
        Cli {
            clientid: "__None".to_string(),
            details: "__None".to_string(),
            state: "__None".to_string(),
            large_image: "__None".to_string(),
            large_text: "__None".to_string(),
            small_image: "__None".to_string(),
            small_text: "__None".to_string(),
            button_url_1: "__None".to_string(),
            button_text_1: "__None".to_string(),
            button_url_2: "__None".to_string(),
            button_text_2: "__None".to_string(),
            start_time: -1,
            end_time: -1,
            party_size: "__None".to_string(),
            party_id: "__None".to_string(),
            match_id: "__None".to_string(),
            join_id: "__None".to_string(),
            spectate_id: "__None".to_string(),
            enable_time: false,
            afk_rpc: false,
            afk_after: 5,
            afk_update: 20,
            exit_after: -1,
            disable_color: false,
        }
    }

    #[test]
    fn accepts_minimal_valid_args() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn rejects_missing_client_id() {
        let args = test_args();
        assert!(matches!(validate(&args), Err(RpcError::MissingClientId)));
    }

    #[test]
    fn rejects_button_2_without_button_1() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.button_url_2 = "https://example.com".to_string();
        args.button_text_2 = "Btn2".to_string();
        assert!(matches!(validate(&args), Err(RpcError::ButtonOrder { .. })));
    }

    #[test]
    fn rejects_time_conflict() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.enable_time = true;
        args.start_time = 1756000000;
        assert!(matches!(validate(&args), Err(RpcError::TimeConflict)));
    }

    #[test]
    fn rejects_party_id_without_party_size() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.party_id = "abc".to_string();
        assert!(matches!(validate(&args), Err(RpcError::PartyIdWithoutSize)));
    }

    #[test]
    fn rejects_join_id_without_match_id() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.join_id = "join".to_string();
        assert!(matches!(validate(&args), Err(RpcError::MatchIdMissing)));
    }

    #[test]
    fn rejects_invalid_party_size_range() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.party_size = "[5,2]".to_string();
        assert!(matches!(validate(&args), Err(RpcError::InvalidPartySize)));
    }

    #[test]
    fn rejects_non_positive_party_size() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.party_size = "[0,10]".to_string();
        assert!(matches!(validate(&args), Err(RpcError::InvalidPartySize)));
    }

    #[test]
    fn accepts_valid_party_size() {
        let mut args = test_args();
        args.clientid = "1234567890".to_string();
        args.party_size = "[1,10]".to_string();
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn party_size_helpers_split_bracketed_pair() {
        assert_eq!(check_current_party_size("[3,7]"), "3");
        assert_eq!(check_max_party_size("[3,7]"), "7");
    }

    #[test]
    fn party_size_helpers_fall_back_to_one() {
        assert_eq!(check_current_party_size("abc"), "abc");
        assert_eq!(check_max_party_size("abc"), "1");
        assert_eq!(check_current_party_size("__None"), "1");
        assert_eq!(check_max_party_size("__None"), "1");
    }
}
