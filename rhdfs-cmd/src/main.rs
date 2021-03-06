//! rhdfs: a Rust equivalent to `hdfs dfs`

#[macro_use] extern crate log;
extern crate env_logger;
extern crate chrono;
extern crate futures;
extern crate tokio;
#[macro_use] extern crate rhdfs;

mod util;

use futures::prelude::*;
use rhdfs::*;
use rhdfs::hdfs::*;



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
    use std::net::{ToSocketAddrs, SocketAddr};

    fn resolve_hostport(hp: &str) -> Result<SocketAddr> {
        Ok(hp
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| app_error!(other "Could not resolve host:port in `{}`", hp))?)
    }

    //replace "local" with "127.0.0.1:8020"
    let nn_hostport = match cfg.nn_hostport.as_ref() {
        "local" => "127.0.0.1:8020",
        r => r
    };

    //resolve nat data
    let mut nat = Vec::new();
    for (a, b) in cfg.nat {
        nat.push((resolve_hostport(&a)?, resolve_hostport(&b)?))
    }

    //resolve namenode host ip
    let nn = resolve_hostport(nn_hostport)?;

    let session_data = SessionData {
        effective_user: cfg.effective_user.clone(),
        forced_client_id: None
    };

    //TODO: move hardcoded values to config
    let mdx = Mdx::new(1, 1, session_data,  nn, nat);

    match cmd {
        Command::GetListing(args) =>
            get_listing(mdx, args),
        Command::Get(args) =>
            get(mdx, args),
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

fn get_listing(mdx: Mdx, args: args::GetListing) -> Result<()> {
    fn print_file(f: HdfsFileStatusProto, src: &str) {
        use chrono::prelude::*;
        let (
            file_type, path, length, permission, owner, group, modification_time, symlink
        ) = pb_decons!(HdfsFileStatusProto, f,
                    file_type, path, length, permission, owner, group, modification_time, symlink
                );

        let mtime_dt: DateTime<Local> = Local.timestamp(modification_time as i64/ 1000 , 0);
        let mtime = mtime_dt.format("%Y-%m-%d %H:%M").to_string();

        let perm_s = perms_s(pb_decons!(FsPermissionProto, permission, perm));

        let path = String::from_utf8_lossy(&path);
        let sep = if src.is_empty() || src.ends_with('/') { "" } else { "/" };

        match file_type {
            HdfsFileStatusProto_FileType::IS_DIR => println!(
                "d{} {} {} {} {} {} {}{}{}",
                perm_s, '-', owner, group, length, mtime, src, sep, path
            ),
            HdfsFileStatusProto_FileType::IS_SYMLINK => println!(
                "u{} {} {} {} {} {} {}{}{} ->{}",
                perm_s, '?', owner, group, length, mtime, src, sep, path,
                String::from_utf8_lossy(&symlink)
            ),
            HdfsFileStatusProto_FileType::IS_FILE => println!(
                "-{} {} {} {} {} {} {}{}{}",
                perm_s, '?', owner, group, length, mtime, src, sep, path
            )
        }
    }

    args.src.into_iter().try_fold(mdx, |mdx, src| -> Result<Mdx> {
        let (s, _) = hdfs::get_listing(mdx, src.clone(), false)
            .forward(util::ForEachSink::<_, _, Error>::new(|fs| Ok(for file in fs { print_file(file, &src) })))
            .wait()?;
        Ok(s.into_inner())
    })?;

    Ok(())
}




use std::path::{Path, PathBuf};


fn build_copy_list(mut fs: Vec<String>) -> Result<Vec<(String, PathBuf)>> {
    use std::ffi::OsStr;

    #[inline]
    fn extract_file_name(path: &Path) -> Result<&OsStr> {
        path.file_name().ok_or_else(|| app_error!(other "invalid source file `{}`", path.display()))
    }

    fn replicate_file_name(from: &str, to: &Path) -> Result<PathBuf> {
        Ok(to.join(extract_file_name(&Path::new(from))?))
    }

    match fs.len() {
        0 => Ok(vec![]),
        1 => {
            //extract filename from src
            let file = fs.pop().unwrap();
            let dpath = extract_file_name(&Path::new(&file))?.into();
            Ok(vec![(file, dpath)])
        }
        2 => {
            let dst = fs.pop().unwrap();
            let src = fs.pop().unwrap();
            let dpath = Path::new(&dst);
            if dpath.is_dir() {
                let d = replicate_file_name(&src, dpath)?;
                Ok(vec![(src, d)])
            } else if dpath.is_file() {
                Err(app_error!(other "File exists: `{}`", dpath.display()))
            } else {
                Ok(vec![(src, dpath.into())])
            }
        }
        _ => {
            let dst = fs.pop().unwrap();
            let dpath = Path::new(&dst);
            if !dpath.is_dir() {
                return Err(app_error!(other "The target directory `{}` does not exist, is not a directory, or is inaccessible", dpath.display()))
            }
            let mut rv = Vec::new();
            for s in fs {
                let dfile = replicate_file_name(&s, dpath)?;
                if dfile.exists() {
                    return Err(app_error!(other "File exists: `{}`", dfile.display()))
                }
                rv.push((s, dfile))
            }
            Ok(rv)
        }
    }

}


fn get(mdx: Mdx, args: args::Get) -> Result<()> {
    use tokio::fs::File;

    let copy_vec = build_copy_list(args.fs)?;

    let f =
        futures::stream::iter_ok(copy_vec).fold(mdx, |mdx, (src, dst)| {
        debug!("Copying {} -> {}", src, dst.display());
        let wf = File::create(dst);
        let r = hdfs::get(mdx, src);
        wf.and_then(|w|
            tokio::io::copy(r, w)
        ).map(|(count, r, _)| {
            debug!("{} bytes written", count);
            r.decons().0.into_inner()
        })
    });

    tokio::run(f.map(|_| ()).map_err(|e: IoError| eprintln!("Error: {}", e)));
    Ok(())

    //TODO add some parallelism here


/*

    for (src, dst) in copy_vec {
        debug!("Copying {} -> {}", src, dst.display());
        let wf = File::create(dst);
        let r = hdfs::get(mdx, src);
        let f = wf.and_then(|w| tokio::io::copy(r, w));
        let (count, r, _) = tokio::run(f)?;
        debug!("{} bytes written", count);
        mdx = r.decons().0.into_inner();
    }

    Ok(())*/
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
        /// path arguments as they are listed in the command line. The trailing arg is probably
        /// a target dir
        pub fs: Vec<String>,
        pub p: bool,
        pub ignore_crc: bool,
        pub crc: bool
    }

    impl Default for Get {
        fn default() -> Self {
            Get {
                fs: vec![],
                p: false,
                ignore_crc: false,
                crc: false
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
                            "-fs" => cfg.nn_hostport = a,
                            "-u" => cfg.effective_user = a,
                            "--add-nat" => cfg.nat.append(&mut split_arg(a)),
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
                match a.as_ref() {
                    "-p" => args.p = true,
                    "-ignoreCrc" => args.ignore_crc = true,
                    "-crc" => args.crc = true,
                    _ => args.fs.push(a)
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

/// 'n1=v1,n2=v2' -> (n1, v1), (n2, v2)
fn split_arg(v: String) -> Vec<(String, String)> {
    v.split(',').into_iter().map(|s| {
        let mut i = s.split('=').into_iter();
        let v0 = i.next().unwrap().to_owned();
        let v1 = i.next().unwrap().to_owned();
        (v0, v1)
    }).collect()
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