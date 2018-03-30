//! rhdfs: a Rust equivalent to `hdfs dfs`

#[macro_use] extern crate log;
extern crate env_logger;

#[macro_use] extern crate rhdfs;

use rhdfs::*;


fn main() {
    env_logger::init();
    trace!("Tracing started");

    let (cmd, cfg) = parse_command_line();

    let r = main_loop(cmd, cfg);

    match r {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(2)
        }
    }
}

fn main_loop(cmd: Command, cfg: config::Common) -> Result<()> {

    let mut cp = ConnectionPoolST::new(&cfg)?;

    match cmd {
        Command::GetListing(gl, xargs) =>
            get_listing(&mut cp, gl, &cfg),
        Command::Version =>
            version(),
        Command::Help|Command::Null =>
            usage()
    }
}


fn perms_s(perm: u32) -> String {
    let xwr = ['x', 'w', 'r'];
     (0..9).map(|n| 8 - n).map(|n| {
        if (perm & (1 << n)) != 0 { xwr[n % 3] } else { '-' }
    }).collect()
}

#[test]
fn test_perms_s() {
    assert_eq!(perms_s(0o0777), "rwxrwxrwx");
    assert_eq!(perms_s(0o0333), "-wx-wx-wx");
    assert_eq!(perms_s(0o0337), "-wx-wxrwx");

}

fn get_listing(cp: &mut ConnectionPoolST, args: config::GetListing, cfg: &config::Common) -> Result<()> {
    struct PSink {
        src: Vec<String>
    }
    impl hdfs::ListingSink for PSink {
        fn files(&mut self, fs: &[HdfsFileStatusProto], src_pos: usize, last_in_src: bool, last: bool) {
            for f in fs {
                let (
                    file_type, path, length, permission, owner, group, modification_time
                ) = pb_decons!(HdfsFileStatusProto, f,
                    file_type, path, length, permission, owner, group, modification_time
                );
                let perm_s = perms_s(pb_decons!(FsPermissionProto, permission, perm));

                let path = String::from_utf8_lossy(path);
                let src: &str = if src_pos < self.src.len() { &self.src[src_pos] } else { "" };
                let sep = if src.is_empty() || src.ends_with('/') { "" } else { "/" };

                match file_type {
                    HdfsFileStatusProto_FileType::IS_DIR => println!(
                        "d{} {} {} {} {} {} {}{}{}",
                        perm_s, '-', owner, group, length, modification_time, src, sep, path
                    ),
                    HdfsFileStatusProto_FileType::IS_SYMLINK => println!(
                        "u{} {} {} {} {} {} {}{}{} ->{}",
                        perm_s, '?', owner, group, length, modification_time, src, sep, path,
                        String::from_utf8_lossy(pb_decons!(HdfsFileStatusProto, f, symlink))
                    ),
                    HdfsFileStatusProto_FileType::IS_FILE => println!(
                        "-{} {} {} {} {} {} {}{}{}",
                        perm_s, '?', owner, group, length, modification_time, src, sep, path
                    )
                };
            }
        }
    }

    let p = hdfs::GetListingState::new(Box::new(PSink { src: args.src.clone() }), args);

    let _ = cp.run_nn(p)?;
    Ok(())
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

mod xargs {
    pub struct GetListing {
        //[-C] [-d] [-h] [-q] [-R] [-t] [-S] [-r] [-u]
        pub c_u: bool,
        pub d: bool,
        pub h: bool,
        pub q: bool,
        pub r_u: bool,
        pub t: bool,
        pub s_u: bool,
        pub r: bool,
        pub u: bool
    }

    impl Default for GetListing {
        fn default() -> Self {
            GetListing {
                c_u: false, d: false, h: false, q: false, r_u: false,
                t: false, s_u: false, r: false, u: false
            }
        }
    }
}

enum Command {
    Null,
    Help,
    Version,
    GetListing(config::GetListing, xargs::GetListing)
}

fn parse_command_line() -> (Command, config::Common) {
    let mut cfg = config::Common::default();
    let mut cmd = Command::Null;

    parse_cmdln(|s, a|
        match &mut cmd {
            c @ &mut Command::Null => {
                match s {
                    None => match a.as_ref() {
                        "ls" => { *c = Command::GetListing(config::GetListing::default(), xargs::GetListing::default()); None },
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
            &mut Command::GetListing(ref mut gl, ref mut _xgl) => {
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
