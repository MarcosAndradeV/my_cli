# MyCLI

A simple rust cli library inspired by clap builder.

# Usage

Justa copy an paste into your project. :)

```console
$ git clone git@github.com:MarcosAndradeV/my_cli.git
```

# Example

```rust
use my_cli::{Cmd, MyCLI};

fn main() {
    let cli = MyCLI::create_from_args()
        .add_cmd(
            "compile",
            Cmd::new().arg("FILE", 1).flag("-o", "FILE").flag_bool("-r")
        )
        .add_cmd(
            "say",
            Cmd::new().arg("MSG", 1)
        )
    ;
    match cli.get_matches() {
        Some(("say", _, args)) => {
            if let Some(msg) = args.get("MSG") {
                println!("{msg}")
            }
        }
        Some(("compile", flags, args)) => {
            if let Some(file) = args.get("FILE") {
                println!("{file}")
            }
            if let Some(Some(file)) = flags.get("-o") {
                println!("{file}")
            }
            if let Some(None) = flags.get("-r") {
                println!("-r")
            }
        }
        _ => (),
    }
}

```
