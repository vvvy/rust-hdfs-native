#![allow(dead_code)]

use std::fmt;
use std::fmt::Debug;


/// Vector of `n` default values
#[inline]
pub fn vector_of_size<T: Default + Clone>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    v.resize(n, T::default());
    v
}

/*
/// ??? Discard contents of `v` and make it a vector of `n` default values
#[inline]
pub fn recycle_to_vector_of_size<T: Default + Clone>(mut v: Vec<T>, n: usize) -> Vec<T> {
    v.resize(n, T::default());
    v.shrink_to_fit();
    v
}
/// `resize` followed by `shrink_to_fit`
#[inline]
pub fn to_vector_of_size<T: Default + Clone>(v: &mut Vec<T>, n: usize) -> () {
    v.resize(n, T::default());
    v.shrink_to_fit();
}
*/


pub fn vec_cons<T>(mut v: Vec<T>, t: T) -> Vec<T> {
    v.push(t);
    v
}

pub fn vec_cons_opt<T>(mut v: Vec<T>, t: Option<T>) -> Vec<T> {
    t.map(|t| v.push(t));
    v
}
pub trait LazyMonoid<T> {
    fn lazy_plus<F>(self, F) -> T where F: FnOnce() -> T;
}

impl<T, E> LazyMonoid<Result<Option<T>, E>> for Result<Option<T>, E> {
    fn lazy_plus<F>(self, t: F) -> Result<Option<T>, E> where F: FnOnce() -> Result<Option<T>, E> {
        match self {
            r @ Ok(Some(..)) | r @ Err(..) => r,
            Ok(None) => t()
        }
    }
}



/// Opt-State-Delta-and-Value
#[derive(Debug)]
pub enum SnV<S, V> {
    SV(S, V),
    V(V)
}

/// Handles `SnV`
#[inline]
pub fn switch_state<S, V>(s: &mut S, sr: SnV<S, V>) -> V {
    match sr {
        SnV::SV(ns, r) => { *s = ns; r }
        SnV::V(r) => r
    }
}

/// Generates (via `f`) and handles `SnV`
#[inline]
pub fn switch_state_f<S, V, F>(s: &mut S, f: F) -> V where F: FnOnce(&mut S) -> SnV<S, V> {
    let snv = f(s);
    switch_state(s, snv)
}


/// Generates (via `f`) and handles `SnV`, plus trace
#[inline]
pub fn logging_switch_state_f<S: Debug, V: Debug, F>(tgt: &'static str, s: &mut S, f: F) -> V where F: FnOnce(&mut S) -> SnV<S, V> {
    let snv = f(s);
    trace!(target: tgt, "switch-state: {:?} => {:?}", s, snv);
    switch_state(s, snv)
}

//FSM handling primitives

///Either switch to a (S)tate or return a (V)alue
#[derive(Debug)]
pub enum SV<S, V> {
    S(S),
    V(V)
}

#[inline]
pub fn fsm_turn<S, V, F>(s: &mut S, mut f: F) -> V where F: FnMut(&mut S) -> SV<S, V> {
    loop {
        let sv = f(s);
        match sv {
            SV::S(ns) => *s = ns,
            SV::V(v) => break v
        }
    }
}

#[inline]
pub fn logging_fsm_turn<S: Debug, V: Debug, F>(tgt: &'static str, s: &mut S, mut f: F) -> V where F: FnMut(&mut S) -> SV<S, V> {
    loop {
        let sv = f(s);
        trace!(target: tgt, "fsm_turn: {:?} => {:?}", s, sv);
        match sv {
            SV::S(ns) => *s = ns,
            SV::V(v) => break v
        }
    }
}





/// Compact binary writer
/// If a slice is longer than `T` bytes, writes only `LH` initial bytes followed by
/// omitted/total bytes count and trailing `LT` bytes
/// Also writes string representation along with byte dump
pub struct CBinary<'a>(pub &'a [u8]);

impl<'a> CBinary<'a> {
    const T: usize = 36;
    const LH: usize = 16;
    const LT: usize = 16;
}

impl<'a> fmt::Display for CBinary<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fn cw(a: &[u8], fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "b\"")?;
            for &c in a {
                // https://doc.rust-lang.org/reference.html#byte-escapes
                if c == b'\n' {
                    write!(fmt, "\\n")?;
                } else if c == b'\r' {
                    write!(fmt, "\\r")?;
                } else if c == b'\t' {
                    write!(fmt, "\\t")?;
                } else if c == b'\\' || c == b'"' {
                    write!(fmt, "\\{}", c as char)?;
                } else if c == b'\0' {
                    write!(fmt, "\\0")?;
                    // ASCII printable
                } else if c >= 0x20 && c < 0x7f {
                    write!(fmt, "{}", c as char)?;
                } else {
                    write!(fmt, "\\x{:02x}", c)?;
                }
            }
            write!(fmt, "\"")?;
            Ok(())
        }

        fn xw(a: &[u8], fmt: &mut fmt::Formatter) -> fmt::Result {
            for byte in a {
                write!(fmt, "{:02x} ", byte)?;
            }
            Ok(())
        }

        let l = self.0.len();
        let (a, b) = if l <= CBinary::T {
            (self.0, None)
        } else {
            (&self.0[..CBinary::LH], Some((
                l - CBinary::LH - CBinary::LT,
                &self.0[l - CBinary::LT..]
             )))
        };

        write!(fmt, "[")?;
        cw(a, fmt)?;
        write!(fmt, " ")?;
        xw(a, fmt)?;
        if let Some((c, b)) = b {
            write!(fmt, "<{}/{} bytes> ", c, l)?;
            cw(b, fmt)?;
            write!(fmt, " ")?;
            xw(b, fmt)?;
        }
        write!(fmt, "]")?;
        Ok(())
    }
}

//----------------

#[cfg(test)]
#[macro_use]
pub mod test {
    use std::fmt::{Display, Formatter, Result};

    pub trait ToBytes {
        fn to_bytes(&self) -> Vec<u8>;
    }

    impl ToBytes for str {
        fn to_bytes(&self) -> Vec<u8> {
            enum S {
                N,
                B(u8)
            };

            let mut rv = Vec::new();

            self.chars().fold(S::N, |s, ch| match (s, ch) {
                (S::N, c) if c.is_digit(16) => S::B(c.to_digit(16).unwrap() as u8),
                (S::B(b), c) if c.is_digit(16) => {
                    rv.push((b << 4) | c.to_digit(16).unwrap() as u8);
                    S::N
                },
                (S::N, ':') => S::N,
                (S::N, c) if c == ' ' || c == '\x0a' || c == '\t' => S::N,
                _ => panic!("Invalid hex string")
            });
            rv
        }
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!("00:01:02".to_bytes(), vec![0x00u8, 0x01, 0x02]);
        assert_eq!("
    00:
    01:
    02:
    03".to_bytes(), vec![0x00u8, 0x01, 0x02, 0x03]);

        assert_eq!("
    00 01 02 03
    07 06 05 04".to_bytes(),
                   vec![0x00u8, 0x01, 0x02, 0x03, 0x07, 0x06, 0x05, 0x04]
        );
    }


    pub struct HexSlice<'a>(pub &'a [u8]);

    impl<'a> HexSlice<'a> {
        pub fn new<T>(data: &'a T) -> HexSlice<'a>
            where T: ?Sized + AsRef<[u8]> + 'a
        {
            HexSlice(data.as_ref())
        }
    }

    impl<'a> Display for HexSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> Result {
            for byte in self.0 {
                write!(f, "{:02x} ", byte)?;
            }
            Ok(())
        }
    }

    #[macro_export]
    macro_rules! assert_enum_variant {
        ($e: expr, $p:pat) => { match $e {
            $p => (),
            other => panic!("assertion failed: expected {}, got {:?}", stringify!($p), other)
        }};
    }


    /// Protocols Test Kit
    #[macro_use]
    pub mod ptk {
        use util::*;
        use util::test::*;
        use std::net::{TcpListener, TcpStream};
        use std::thread;
        use std::io::{Read, Write};

        /// Test Script Instruction
        pub enum TsInstr {
            Expect(&'static str),
            Send(&'static str)
        }

        macro_rules! test_script_cmd {
            { expect $v:expr } => { TsInstr::Expect($v) };
            { send $v:expr } => { TsInstr::Send($v) };
        }

        #[macro_export]
        macro_rules! test_script {
            { $($c:ident $v:expr),* } => { vec![$(test_script_cmd!($c $v)),*] };
        }

        fn expect(conn: &mut TcpStream, e: Vec<u8>) {
            let mut v = vector_of_size(e.len());
            let r = conn.read_exact(&mut v);
            assert!(r.is_ok());
            assert_eq!(v, e);
        }

        fn send(conn: &mut TcpStream, e: Vec<u8>) {
            let r = conn.write(&e);
            assert_eq!(r.ok(), Some(e.len()));
        }

        /// gets addr as `"host:port"` and test script
        pub fn spawn_test_server<'a>(bind_addr: &'a str, script: Vec<TsInstr>) -> thread::JoinHandle<()> {
            let listener = TcpListener::bind(bind_addr).unwrap();

            let t = thread::spawn(move || {
                let mut conn = listener.incoming().next().unwrap().unwrap();

                for instr in script {
                    match instr {
                        TsInstr::Expect(s) => expect(&mut conn, s.to_bytes()),
                        TsInstr::Send(s) => send(&mut conn, s.to_bytes())
                    }
                }
            });

            t
        }
    }


}


