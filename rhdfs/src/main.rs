//! rhdfs: a Rust equivalent to `hdfs dfs`

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate rhdfs;

use rhdfs::*;


fn main() {
    env_logger::init();
    trace!("Tracing started");

    let (cmd, cfg) = parse_command_line();

    let r = match cmd {
        Command::GetListing(gl) => hdfs::read_dir_listing(gl, &cfg),
        Command::Version => version(),
        Command::Help|Command::Null => usage()
    };

    match r {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2)
        }
    }
}

fn version() -> ! {
    println!(
        "{} ({}) version {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    std::process::exit(0);
}

fn usage() -> ! {
    println!("USAGE:\
rhdfs ls -[CdhqRtSru] [<path> ...]
rhdfs help
rhdfs version
    ");
    std::process::exit(1);
}

enum Command {
    Null,
    Help,
    Version,
    GetListing(config::GetListing)
}

fn parse_command_line() -> (Command, config::Common) {
    let mut cfg = config::Common::default();
    let mut cmd = Command::Null;

    parse_cmdln(|s, a|
        match &mut cmd {
            c @ &mut Command::Null => {
                match s {
                    None => match a.as_ref() {
                        "ls" => { *c = Command::GetListing(config::GetListing::default()); None },
                        "help" => { *c = Command::Help; None },
                        "version" => { *c = Command::Version; None },
                        _ => Some(a)
                    },
                    Some(ref a0) => {
                        match a0.as_ref() {
                            "-nn" => cfg.nn_hostport = a,
                            "-u" => cfg.effective_user = a,
                            _ => error_exit(&format!("Invalid cmd line at `{} {}`", a0, a), "unknown option")
                        };
                        None
                    }
                }
            },
            &mut Command::GetListing(ref mut gl) => {
                match a.as_ref() {
                    //[-C] [-d] [-h] [-q] [-R] [-t] [-S] [-r] [-u]
                    "-C" => gl.c_u = true,
                    "-d" => gl.d = true,
                    "-h" => gl.h = true,
                    "-q" => gl.q = true,
                    "-R" => gl.r_u = true,
                    "-t" => gl.t = true,
                    "-S" => gl.s_u = true,
                    "-r" => gl.r = true,
                    "-u" => gl.u = true,
                    _ => gl.src.push(a)
                };
                None
            },
            &mut Command::Help | &mut Command::Version => None
        }
    );

    (cmd, cfg)
}



/// '--sw=arg' => '--sw' 'arg'
/// '-abc' => -a -b -c
fn convert_arg(v: String) -> Vec<String> {
    use std::iter::FromIterator;
    if v.starts_with("--") {
        v.splitn(2, "=").map(|r| r.to_string()).collect()
    } else if v.starts_with("-") && v != "-" {
        v.chars().skip(1).map(|c| String::from_iter(vec!['-', c])).collect()
    } else {
        vec![v]
    }
}

/// Prints two-part message to stderr and exits
fn error_exit(msg: &str, detail: &str) -> ! {
    eprint!("Error: {}", msg);
    if detail.is_empty() {
        eprintln!()
    } else {
        eprintln!(" ({})", detail);
    }
    std::process::exit(1)
}

/// Expect2 function
trait Expect2<T> {
    /// Same as Result::expect but the error message is brief and not intimidating
    fn expect2(self, msg: &str) -> T;
}

impl<T, E: std::error::Error> Expect2<T> for std::result::Result<T, E> {
    fn expect2(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => error_exit(msg, e.description())
        }
    }
}

/// Parses command line for 0- and 1-argument options.
/// `f` consumes the current state and a command line word, and produces the new state.
/// State transitions: an option taking no argument: `(None, "--opt") -> None`;
/// An option taking an arg: `(None, "--opt") -> Some("--opt")`, then `(Some("--opt"), "arg") -> None`.
fn parse_cmdln<F>(f: F) where F: FnMut(Option<String>, String) -> Option<String> {
    match std::env::args().skip(1).flat_map(convert_arg).fold(None, f) {
        None => (),
        Some(ref e) => error_exit(&format!("Invalid cmd line at `{}`<EOL>", e), "unknown option")
    }
}


fn bool_opt(s: String) -> bool {
    match s.as_ref() {
        "true"|"+"|"yes" => true,
        "false"|"-"|"no" => false,
        v => panic!("invalid bool value '{}'", v)
    }
}
