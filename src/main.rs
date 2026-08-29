use std::process::exit;

use clap::Parser;

mod activity;
mod afk;
mod cli;
mod display;
mod error;
mod rpc;
mod validate;

fn run(args: cli::Cli) -> Result<(), error::RpcError> {
    if args.afk_rpc {
        afk::run(&args)
    } else {
        validate::validate(&args)?;
        let client_id = args
            .clientid
            .as_deref()
            .expect("Client id is validated before connecting");
        let mut client = rpc::connect(client_id)?;
        let activity = activity::build_activity(&args);
        rpc::set_activity(&mut client, activity)?;
        display::print_summary(&args);
        rpc::handle_exit(args.exit_after, args.disable_color)
    }
}

fn main() {
    let args = cli::Cli::parse();
    if let Err(err) = run(args) {
        eprintln!("{err}");
        exit(err.exit_code());
    }
}
