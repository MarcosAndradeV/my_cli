# MyCLI

A simple rust cli library inspired by clap builder.

# Usage

```console
$ cargo add my_cli
```

or

```console
$ git clone git@github.com:MarcosAndradeV/my_cli.git # Just copy an paste into your project. :)
```

# Example

```rust
use my_cli::*;

fn main() {
    let cl = MyCLI::create_from_args()
        .add_cmd("say", Cmd::new().arg("msg", 1)
            .help("prints the message.")
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
                println!("Expected msg");
            }
        }
        Some(("sayTo", flags, args)) => {
            if let Some(msg) = args.get("msg") {
                if let Some(Some(name)) = flags.get("-t") {
                    println!("You say: \"{msg}\"\nTo: {name}");
                } else {
                    println!("You say: \"{msg}\"");
                }
            }
        }
        _ =>  cl.usage(),
    }
}


```
