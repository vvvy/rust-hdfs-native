
use std;

/// '--sw=arg' => '--sw' 'arg'
/// '-abc' => -a -b -c
pub fn convert_arg(v: String) -> Vec<String> {
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
pub fn error_exit(msg: &str, detail: &str) -> ! {
    eprint!("Error: {}", msg);
    if detail.is_empty() {
        eprintln!()
    } else {
        eprintln!(" ({})", detail);
    }
    std::process::exit(1)
}

/// Expect2 function
pub trait Expect2<T> {
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
pub fn parse_cmdln<F>(f: F) where F: FnMut(Option<String>, String) -> Option<String> {
    match std::env::args().skip(2).flat_map(convert_arg).fold(None, f) {
        None => (),
        Some(ref e) => error_exit(&format!("Invalid cmd line at `{}`<EOL>", e), "unknown option")
    }
}

pub fn bool_opt(s: String) -> bool {
    match s.as_ref() {
        "true"|"+"|"yes" => true,
        "false"|"-"|"no" => false,
        v => panic!("invalid bool value '{}'", v)
    }
}
