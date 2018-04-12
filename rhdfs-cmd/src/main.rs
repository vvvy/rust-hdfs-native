//! rhdfs: a Rust equivalent to `hdfs dfs`

#[macro_use] extern crate log;
extern crate env_logger;
extern crate chrono;


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

    let mut cp = SyncConnectionPoolST::new(&cfg)?;

    match cmd {
        Command::GetListing(args) =>
            get_listing(&mut cp, args),
        Command::Get(args) =>
            get(&mut cp, args),
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

fn get_listing(cp: &mut SyncConnectionPoolST, args: args::GetListing) -> Result<()> {
    struct PSink {
        src: Vec<String>
    }
    impl hdfs::ListingSink for PSink {
        fn files(&mut self, fs: &[HdfsFileStatusProto], src_pos: usize, _last_in_src: bool, _last: bool) {
            use self::chrono::prelude::*;

            for f in fs {
                let (
                    file_type, path, length, permission, owner, group, modification_time
                ) = pb_decons!(HdfsFileStatusProto, f,
                    file_type, path, length, permission, owner, group, modification_time
                );

                let mtime_dt: DateTime<Local> = Local.timestamp(modification_time as i64/ 1000 , 0);
                let mtime = mtime_dt.format("%Y-%m-%d %H:%M").to_string();

                let perm_s = perms_s(pb_decons!(FsPermissionProto, permission, perm));

                let path = String::from_utf8_lossy(path);
                let src: &str = if src_pos < self.src.len() { &self.src[src_pos] } else { "" };
                let sep = if src.is_empty() || src.ends_with('/') { "" } else { "/" };

                match file_type {
                    HdfsFileStatusProto_FileType::IS_DIR => println!(
                        "d{} {} {} {} {} {} {}{}{}",
                        perm_s, '-', owner, group, length, mtime, src, sep, path
                    ),
                    HdfsFileStatusProto_FileType::IS_SYMLINK => println!(
                        "u{} {} {} {} {} {} {}{}{} ->{}",
                        perm_s, '?', owner, group, length, mtime, src, sep, path,
                        String::from_utf8_lossy(pb_decons!(HdfsFileStatusProto, f, symlink))
                    ),
                    HdfsFileStatusProto_FileType::IS_FILE => println!(
                        "-{} {} {} {} {} {} {}{}{}",
                        perm_s, '?', owner, group, length, mtime, src, sep, path
                    )
                };
            }
        }
    }

    let src2 = args.src.clone();
    let _ = hdfs::get_listing(
        cp,
        config::GetListing { src: args.src, need_location: false },
        Box::new(PSink { src: src2 })
    )?;
    Ok(())
}

fn get(cp: &mut SyncConnectionPoolST, args: args::Get) -> Result<()> {
    let _ = hdfs::get(cp, config::Get { src: args.fs, tgt_dir: None } )?;
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

mod args {
    pub struct GetListing {
        pub src: Vec<String>,
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
                src: vec![],
                c_u: false, d: false, h: false, q: false, r_u: false,
                t: false, s_u: false, r: false, u: false
            }
        }
    }

    pub struct Get {
        pub fs: Vec<String>
    }

    impl Default for Get {
        fn default() -> Self {
            Get {
                fs: vec![]
            }
        }
    }
}

enum Command {
    Null,
    Help,
    Version,
    GetListing(args::GetListing),
    Get(args::Get)
}

fn parse_command_line() -> (Command, config::Common) {
    let mut cfg = config::Common::default();
    let mut cmd = Command::Null;

    parse_cmdln(|s, a|
        match &mut cmd {
            c @ &mut Command::Null => {
                match s {
                    None => match a.as_ref() {
                        "ls" => { *c = Command::GetListing(args::GetListing::default()); None },
                        "get" => { *c = Command::Get(args::Get::default()); None },
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
            &mut Command::GetListing(ref mut args) => {
                match a.as_ref() {
                    //[-C] [-d] [-h] [-q] [-R] [-t] [-S] [-r] [-u]
                    "-C" => args.c_u = true,
                    "-d" => args.d = true,
                    "-h" => args.h = true,
                    "-q" => args.q = true,
                    "-R" => args.r_u = true,
                    "-t" => args.t = true,
                    "-S" => args.s_u = true,
                    "-r" => args.r = true,
                    "-u" => args.u = true,
                    _ => args.src.push(a)
                };
                None
            },
            &mut Command::Get(ref mut args) => {
                args.fs.push(a);
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

/*
fn bool_opt(s: String) -> bool {
    match s.as_ref() {
        "true"|"+"|"yes" => true,
        "false"|"-"|"no" => false,
        v => panic!("invalid bool value '{}'", v)
    }
}
*/