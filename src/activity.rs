use std::time::{SystemTime, UNIX_EPOCH};

use discord_rich_presence::activity::{self, Activity, Assets, Button, Party, Secrets};

use crate::cli::Cli;
use crate::validate::{non_empty, parse_party_size};

pub(crate) fn build_activity(args: &Cli) -> Activity<'_> {
    let mut activity = Activity::new();
    if let Some(state) = &args.state {
        activity = activity.state(state);
    }
    if let Some(details) = &args.details {
        activity = activity.details(details);
    }
    let mut assets = Assets::new();
    if let Some(image) = &args.large_image {
        assets = assets.large_image(image);
    }
    if let Some(text) = &args.large_text {
        assets = assets.large_text(text);
    }
    if let Some(image) = &args.small_image {
        assets = assets.small_image(image);
    }
    if let Some(text) = &args.small_text {
        assets = assets.small_text(text);
    }
    if args.large_image.is_some()
        || args.large_text.is_some()
        || args.small_image.is_some()
        || args.small_text.is_some()
    {
        activity = activity.assets(assets);
    }
    let mut buttons = Vec::new();
    if let (Some(text), Some(url)) = (&args.button_text_1, &args.button_url_1) {
        if !url.is_empty() {
            buttons.push(Button::new(text, url));
        }
    }
    if let (Some(text), Some(url)) = (&args.button_text_2, &args.button_url_2) {
        if !url.is_empty() {
            buttons.push(Button::new(text, url));
        }
    }
    if !buttons.is_empty() {
        activity = activity.buttons(buttons);
    }
    if args.enable_time || args.start_time.is_some() || args.end_time.is_some() {
        let mut timestamps = activity::Timestamps::new();
        if args.enable_time {
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
        let (current_size, max_size) = parse_party_size(size);
        let mut party = Party::new().size([current_size, max_size]);
        if let Some(id) = &args.party_id {
            party = party.id(id);
        }
        activity = activity.party(party);
    }
    if let Some(match_id) = non_empty(&args.match_id) {
        let mut secrets = Secrets::new().r#match(match_id);
        if let Some(join_id) = non_empty(&args.join_id) {
            secrets = secrets.join(join_id);
        }
        if let Some(spectate_id) = non_empty(&args.spectate_id) {
            secrets = secrets.spectate(spectate_id);
        }
        activity = activity.secrets(secrets);
    }
    activity
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::test_cli;

    fn json_of(activity: &Activity) -> serde_json::Value {
        serde_json::to_value(activity).expect("activity serializes to json")
    }

    #[test]
    fn empty_args_produce_empty_payload() {
        let args = test_cli();
        let payload = json_of(&build_activity(&args));
        assert!(payload.as_object().unwrap().is_empty());
    }

    #[test]
    fn sets_state_and_details() {
        let mut args = test_cli();
        args.state = Some("In Lobby".to_string());
        args.details = Some("Competitive".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["state"], "In Lobby");
        assert_eq!(payload["details"], "Competitive");
    }

    #[test]
    fn builds_assets_when_provided() {
        let mut args = test_cli();
        args.large_image = Some("big".to_string());
        args.large_text = Some("Big Image".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["assets"]["large_image"], "big");
        assert_eq!(payload["assets"]["large_text"], "Big Image");
        assert!(payload["assets"].get("small_image").is_none());
    }

    #[test]
    fn drops_button_if_url_is_empty_or_absent() {
        let mut args = test_cli();
        args.button_text_1 = Some("GitHub".to_string());
        // url absent
        let payload = json_of(&build_activity(&args));
        assert!(payload.get("buttons").is_none());

        args.button_url_1 = Some(String::new()); // url empty
        let payload = json_of(&build_activity(&args));
        assert!(payload.get("buttons").is_none());
    }

    #[test]
    fn builds_both_buttons() {
        let mut args = test_cli();
        args.button_text_1 = Some("One".to_string());
        args.button_url_1 = Some("https://one.com".to_string());
        args.button_text_2 = Some("Two".to_string());
        args.button_url_2 = Some("https://two.com".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["buttons"][0]["label"], "One");
        assert_eq!(payload["buttons"][0]["url"], "https://one.com");
        assert_eq!(payload["buttons"][1]["label"], "Two");
        assert_eq!(payload["buttons"][1]["url"], "https://two.com");
    }

    #[test]
    fn enable_time_sets_recent_start_timestamp() {
        let mut args = test_cli();
        args.enable_time = true;
        let payload = json_of(&build_activity(&args));
        let start = payload["timestamps"]["start"].as_i64().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        assert!((start - now).abs() <= 5);
        assert!(payload["timestamps"].get("end").is_none());
    }

    #[test]
    fn explicit_start_and_end_are_both_retained() {
        let mut args = test_cli();
        args.start_time = Some(1000);
        args.end_time = Some(2000);
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["timestamps"]["start"], 1000);
        assert_eq!(payload["timestamps"]["end"], 2000);
    }

    #[test]
    fn end_time_alone_is_retained() {
        let mut args = test_cli();
        args.end_time = Some(2000);
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["timestamps"]["end"], 2000);
        assert!(payload["timestamps"].get("start").is_none());
    }

    #[test]
    fn party_size_and_id_build_party_object() {
        let mut args = test_cli();
        args.party_size = Some("[2,8]".to_string());
        args.party_id = Some("party123".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["party"]["size"][0], 2);
        assert_eq!(payload["party"]["size"][1], 8);
        assert_eq!(payload["party"]["id"], "party123");
    }

    #[test]
    fn party_size_without_id_has_no_id_field() {
        let mut args = test_cli();
        args.party_size = Some("[1,4]".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["party"]["size"][0], 1);
        assert_eq!(payload["party"]["size"][1], 4);
        assert!(payload["party"].get("id").is_none());
    }

    #[test]
    fn secrets_all_three_fields() {
        let mut args = test_cli();
        args.match_id = Some("m1".to_string());
        args.join_id = Some("j1".to_string());
        args.spectate_id = Some("s1".to_string());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["secrets"]["match"], "m1");
        assert_eq!(payload["secrets"]["join"], "j1");
        assert_eq!(payload["secrets"]["spectate"], "s1");
    }

    #[test]
    fn secrets_treats_empty_join_as_absent() {
        let mut args = test_cli();
        args.match_id = Some("m1".to_string());
        args.join_id = Some(String::new());
        let payload = json_of(&build_activity(&args));
        assert_eq!(payload["secrets"]["match"], "m1");
        assert!(payload["secrets"].get("join").is_none());
    }
}
