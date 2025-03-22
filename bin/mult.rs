use my_cli::{Cmd, MyCLI};


fn main() {
    let cli = MyCLI::create_from_args()
        .add_cmd(
            "test",
            Cmd::new()
                .arg("INPUT", 0)
                .flag("output", "FILE", false)
        );
    match cli.get_matches() {
        Some(("test", flags, args)) => {
            dbg!(args.get(0));
            dbg!(flags.get("output"));
        }
        _ => cli.usage(),
    }
}
