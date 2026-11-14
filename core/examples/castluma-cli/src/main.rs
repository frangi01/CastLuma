mod scenario;
use std::process::ExitCode;

use castluma_core::CastSession;

fn main() -> ExitCode {
    let mut session = CastSession::new();

    let mut args = std::env::args_os().skip(1);
    let argument = args.next();

    if args.next().is_some() {
        eprintln!("Usage: castluma-cli [normal|timeout]");
        return ExitCode::FAILURE;
    }

    let events = match argument.as_deref() {
        None => scenario::normal_events(),
        Some(value) if value == "normal" => scenario::normal_events(),
        Some(value) if value == "timeout" => scenario::timeout_events(),
        Some(_) => {
            eprintln!("Unknown scenario. Usage: castluma-cli [normal|timeout]");
            return ExitCode::FAILURE;
        }
    };

    println!("Simulation only: no receiver is contacted.");
    println!("Initial state: {:?}", session.state());

    for event in events {
        let previous_state = session.state();

        match session.apply(event) {
            Ok(()) => {
                println!("{previous_state:?} --{event:?}--> {:?}", session.state());
            }
            Err(error) => {
                eprintln!(
                    "Simulation failed: event {:?} was rejected in state {:?}",
                    error.event, error.state
                );
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
