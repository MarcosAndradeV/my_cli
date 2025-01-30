use my_cli::*;

fn main() {
    let cl = MyCLI::create_from_args()
        .add_cmd("say", Cmd::new().arg("msg", 1)
            .help("Prints the message.")
        )
        .add_cmd("sayTo",
            Cmd::new()
            .arg("msg", 1)
            .flag("-t", "NAME")
            .help("Prints the message to <NAME>.")
        );
    match cl.get_matches() {
        Some(("say", _, args)) => {
            if let Some(msg) = args.get("msg") {
                println!("You say: \"{msg}\"");
            } else {
                println!("ERROR: Expected msg");
            }
        }
        Some(("sayTo", flags, args)) => {
            if let Some(msg) = args.get("msg") {
                if let Some(name) = flags.get("-t") {
                    println!("You say: \"{msg}\"\nTo: {name}");
                } else {
                    println!("You say: \"{msg}\"");
                }
            }
        }
        _ =>  cl.usage(),
    }
}
