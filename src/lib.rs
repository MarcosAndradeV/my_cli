use std::{collections::HashMap, env};

/// A simple CLI.
/// `inputs` are anything that does not starts with `-` and `flags` are anything that does starts with `-`
#[derive(Debug, Default)]
pub struct MyCLI {
    #[allow(unused)]
    program: String,
    subcommand: Option<String>,
    flags: Vec<String>,
    inputs: Vec<(u64, String)>,

    cmds: HashMap<String, Cmd>,
    usage: Vec<(String, String, String)>,
}

impl MyCLI {
    /// Create [`MyCLIParser`] with `env::args()`.
    ///
    /// # Panics
    ///
    /// Panics if `env::args().next()` return `None`.
    pub fn create_from_args() -> Self {
        let mut args = env::args();
        let program = args.next().expect("Expect program");
        let subcommand = args.next();
        let mut flags = vec![];
        let mut inputs = vec![];
        let mut i = 1;
        for arg in args {
            if arg.starts_with("-") {
                flags.push(arg);
            } else {
                inputs.push((i, arg));
                i += 1;
            }
        }
        Self {
            program,
            subcommand,
            flags,
            inputs,
            ..Default::default()
        }
    }

    /// Create [`MyCLIParser`] with provided arguments.
    ///
    pub fn create_with_args(program: String, args: impl Iterator<Item = String>) -> Self {
        let mut args = args.into_iter();
        let subcommand = args.next();
        let mut flags = vec![];
        let mut inputs = vec![];
        for (i, arg) in args.enumerate() {
            if arg.starts_with("-") {
                flags.push(arg);
            } else {
                inputs.push((i as u64, arg));
            }
        }
        Self {
            program,
            subcommand,
            flags,
            inputs,
            ..Default::default()
        }
    }

    pub fn get_matches(
        &self,
    ) -> Option<(
        &str,
        MatchedFlags,
        HashMap<String, String>,
    )> {
        let mut matched_flags: MatchedFlags = Default::default();
        let mut matched_args: HashMap<String, String> = Default::default();
        let subcommand = match self.subcommand {
            Some(ref sc) => sc,
            None => return None,
        };
        let cmd = match self.cmds.get(subcommand).cloned() {
            Some(c) => c,
            None => return None,
        };
        let mut args = self.inputs.iter();
        for flag in &self.flags {
            match cmd.flags.get(flag) {
                Some(None) => {
                    matched_flags.insert(flag.clone(), None);
                }
                Some(Some(_)) => loop {
                    match args.next() {
                        Some((n, v)) if cmd.args.get(n).is_some() => {
                            matched_args.insert(cmd.args.get(n).cloned().unwrap(), v.clone());
                            continue;
                        }
                        Some((_, v)) => {
                            matched_flags.insert(flag.clone(), Some(v.clone()));
                            break;
                        }
                        None => return None,
                    }
                },
                None => return None,
            }
        }

        for (n, v) in args {
            let k = match cmd.args.get(n).cloned() {
                Some(ok) => ok,
                None => return None,
            };
            matched_args.insert(k, v.clone());
        }

        Some((
            self.subcommand.as_ref().unwrap().as_str(),
            matched_flags,
            matched_args,
        ))
    }

    pub fn add_cmd(mut self, name: &'static str, cmd: Cmd) -> Self {
        let get_help = cmd.get_help();
        self.usage.push((name.to_string(), get_help.0, get_help.1));
        self.cmds.insert(name.to_string(), cmd);
        self
    }

    pub fn usage(&self) {
        println!("Usage: {} <COMMAND> [ARGS] [[-|--]FLAG]", self.program);
        println!("COMMANDS:");

        let max_w = self.usage.iter().fold(0, |w, (name, arg, _)| {
            let size = name.len() + arg.len();
            if w < size {
                size
            } else {
                w
            }
        });

        for (name, args, descripition) in self.usage.iter() {
            let part1 = format!("    {name} {args}");
            let w = if max_w > part1.len() {
                max_w + part1.len()
            } else {
                2 + descripition.len()
            };
            let part2 = format!("{:>w$}", descripition);
            println!("{part1}   {part2}")
        }
    }
}

#[derive(Default)]
pub struct MatchedFlags(HashMap<String, Option<String>>);

impl MatchedFlags {
    pub fn get(&self, k: &str) -> Option<&String> {
        if let Some(v) = self.0.get(k) {
            v.as_ref()
        } else {
            None
        }
    }

    pub fn is_present(&self, k: &str) -> bool {
        self.0.get(k).is_some()
    }

    pub fn insert(&mut self, k: String, v: Option<String>) -> Option<Option<String>> {
        self.0.insert(k, v)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cmd {
    args: HashMap<u64, String>,
    flags: HashMap<String, Option<String>>,
    help: Option<String>,
}

impl Cmd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }

    pub fn arg(mut self, name: &str, pos: u64) -> Self {
        self.args.insert(pos, name.to_string());
        self
    }

    pub fn flag(mut self, name: &str, val: &str) -> Self {
        self.flags.insert(name.to_string(), Some(val.to_string()));
        self
    }

    pub fn flag_bool(mut self, name: &str) -> Self {
        self.flags.insert(name.to_string(), None);
        self
    }

    fn get_help(&self) -> (String, String) {
        let mut args = String::new();
        let mut iter = self.args.iter().collect::<Vec<(&u64, &String)>>();
        iter.sort_by(|a, b| a.0.cmp(b.0));
        for (_, arg) in iter {
            args.push_str(&format!("<{arg}>"));
        }

        let mut iter = self.flags.iter().collect::<Vec<_>>();
        iter.sort();
        for (fname, arg) in iter {
            if let Some(arg) = arg {
                args.push_str(&format!(" [{fname} "));
                args.push_str(&format!("<{arg}>]"));
            } else {
                args.push_str(&format!(" [{fname}] "));
            }
        }
        (args, self.help.clone().unwrap_or_default())
    }
}
