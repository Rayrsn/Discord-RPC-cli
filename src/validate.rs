use crate::cli::Cli;
use crate::error::RpcError;

pub(crate) fn validate(args: &Cli) -> Result<(), RpcError> {
    if args.clientid.is_none() {
        return Err(RpcError::MissingClientId);
    }
    if (args.button_text_1.is_none() && args.button_text_2.is_some())
        || (args.button_url_1.is_none() && args.button_url_2.is_some())
    {
        return Err(RpcError::ButtonOrder {
            button_text_1: args.button_text_1.clone(),
            button_url_1: args.button_url_1.clone(),
            button_text_2: args.button_text_2.clone(),
            button_url_2: args.button_url_2.clone(),
        });
    }
    if args.enable_time && (args.start_time.is_some() || args.end_time.is_some()) {
        return Err(RpcError::TimeConflict);
    }
    if args.party_id.is_some() && args.party_size.is_none() {
        return Err(RpcError::PartyIdWithoutSize);
    }
    if args.match_id.is_none()
        && (non_empty(&args.join_id).is_some() || non_empty(&args.spectate_id).is_some())
    {
        return Err(RpcError::MatchIdMissing);
    }
    if let Some(party_size) = &args.party_size {
        let (current_size, max_size) = parse_party_size(party_size);
        if current_size < 1 || max_size < 1 || current_size > max_size {
            return Err(RpcError::InvalidPartySize);
        }
    }
    Ok(())
}

/// Absent (`None`) and empty (`Some("")`) strings are treated the same way,
/// replacing the old mix of `"__None"` sentinel and `""` comparisons.
pub(crate) fn non_empty(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}

/// Parses `"[current,max]"` (brackets optional, whitespace not trimmed) with a
/// silent fallback to 1 for unparseable parts, matching the historical
/// `check_current_party_size`/`check_max_party_size` behavior.
pub(crate) fn parse_party_size(party_size: &str) -> (i32, i32) {
    let mut parts = party_size.trim_matches('[').trim_matches(']').split(',');
    let current = parts.next().unwrap_or("1").parse::<i32>().unwrap_or(1);
    let max = parts.next().unwrap_or("1").parse::<i32>().unwrap_or(1);
    (current, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_args() -> Cli {
        Cli {
            clientid: None,
            details: None,
            state: None,
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
            button_url_1: None,
            button_text_1: None,
            button_url_2: None,
            button_text_2: None,
            start_time: None,
            end_time: None,
            party_size: None,
            party_id: None,
            match_id: None,
            join_id: None,
            spectate_id: None,
            enable_time: false,
            afk_rpc: false,
            afk_after: 5,
            afk_update: 20,
            exit_after: None,
            disable_color: false,
        }
    }

    #[test]
    fn accepts_minimal_valid_args() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
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
        args.clientid = Some("1234567890".to_string());
        args.button_url_2 = Some("https://example.com".to_string());
        args.button_text_2 = Some("Btn2".to_string());
        assert!(matches!(validate(&args), Err(RpcError::ButtonOrder { .. })));
    }

    #[test]
    fn accepts_button_1_alone() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.button_text_1 = Some("Btn1".to_string());
        args.button_url_1 = Some("https://example.com".to_string());
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn rejects_time_conflict() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.enable_time = true;
        args.start_time = Some(1756000000);
        assert!(matches!(validate(&args), Err(RpcError::TimeConflict)));
    }

    #[test]
    fn rejects_party_id_without_party_size() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.party_id = Some("abc".to_string());
        assert!(matches!(validate(&args), Err(RpcError::PartyIdWithoutSize)));
    }

    #[test]
    fn rejects_join_id_without_match_id() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.join_id = Some("join".to_string());
        assert!(matches!(validate(&args), Err(RpcError::MatchIdMissing)));
    }

    #[test]
    fn treats_empty_join_id_as_absent() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.join_id = Some(String::new());
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn rejects_invalid_party_size_range() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.party_size = Some("[5,2]".to_string());
        assert!(matches!(validate(&args), Err(RpcError::InvalidPartySize)));
    }

    #[test]
    fn rejects_non_positive_party_size() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.party_size = Some("[0,10]".to_string());
        assert!(matches!(validate(&args), Err(RpcError::InvalidPartySize)));
    }

    #[test]
    fn accepts_valid_party_size() {
        let mut args = test_args();
        args.clientid = Some("1234567890".to_string());
        args.party_size = Some("[1,10]".to_string());
        assert!(validate(&args).is_ok());
    }

    #[test]
    fn parse_party_size_handles_bracketed_pair() {
        assert_eq!(parse_party_size("[3,7]"), (3, 7));
        assert_eq!(parse_party_size("3,7"), (3, 7));
        assert_eq!(parse_party_size("[[1,10]]"), (1, 10));
    }

    #[test]
    fn parse_party_size_falls_back_to_one() {
        assert_eq!(parse_party_size("abc"), (1, 1));
        assert_eq!(parse_party_size("[5]"), (5, 1));
        assert_eq!(parse_party_size(""), (1, 1));
        assert_eq!(parse_party_size("[1, 10]"), (1, 1)); // whitespace untrimmed, like before
    }

    #[test]
    fn non_empty_treats_empty_as_absent() {
        assert_eq!(non_empty(&None), None);
        assert_eq!(non_empty(&Some(String::new())), None);
        assert_eq!(non_empty(&Some("x".to_string())), Some("x"));
    }
}
