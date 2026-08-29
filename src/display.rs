use crate::cli::Cli;

use colored::Colorize;

pub(crate) fn print_summary(args: &Cli) {
    let mut entries: Vec<(&'static str, String)> = Vec::new();
    str_entry(&mut entries, "State : ", &args.state);
    str_entry(&mut entries, "Detail : ", &args.details);
    str_entry(&mut entries, "Large Image name : ", &args.large_image);
    str_entry(&mut entries, "Large Image text : ", &args.large_text);
    str_entry(&mut entries, "Small Image name : ", &args.small_image);
    str_entry(&mut entries, "Small Image text : ", &args.small_text);
    str_entry(&mut entries, "Button 1 Text: ", &args.button_text_1);
    str_entry(&mut entries, "Button 1 URL: ", &args.button_url_1);
    str_entry(&mut entries, "Button 2 Text: ", &args.button_text_2);
    str_entry(&mut entries, "Button 2 URL: ", &args.button_url_2);
    int_entry(&mut entries, "Start time is set to: ", args.start_time);
    int_entry(&mut entries, "End time is set to: ", args.end_time);
    str_entry(&mut entries, "Party Size: ", &args.party_size);
    str_entry(&mut entries, "Party ID: ", &args.party_id);
    str_entry(&mut entries, "Match ID: ", &args.match_id);
    str_entry(&mut entries, "Match Join ID: ", &args.join_id);
    str_entry(&mut entries, "Match Spectate ID: ", &args.spectate_id);

    for (label, value) in &entries {
        if args.disable_color {
            println!("{label}{value}");
        } else {
            println!("{}{}", label.magenta(), value.blue());
        }
    }
    if args.enable_time {
        if args.disable_color {
            println!("Time is Enabled");
        } else {
            println!("{}", "Time is Enabled".blue());
        }
    }
}

fn str_entry(entries: &mut Vec<(&'static str, String)>, label: &'static str, value: &Option<String>) {
    if let Some(value) = value {
        entries.push((label, value.clone()));
    }
}

fn int_entry(entries: &mut Vec<(&'static str, String)>, label: &'static str, value: Option<i64>) {
    if let Some(value) = value {
        entries.push((label, value.to_string()));
    }
}
