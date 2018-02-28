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



//----------------
use std::fmt::{Formatter, Result};

/// Compact debug writer
/// If a slice is longer than `T` bytes, writes only `LH` initial bytes followed by
/// omitted/total bytes count and trailing `LT` bytes
pub struct CDebug<'a>(pub &'a [u8]);

impl<'a> CDebug<'a> {
    const T: usize = 20;
    const LH: usize = 8;
    const LT: usize = 8;
}

impl<'a> Debug for CDebug<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fn cw(a: &[u8], fmt: &mut Formatter) -> Result {
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

        fn xw(a: &[u8], fmt: &mut Formatter) -> Result {
            for byte in a {
                write!(fmt, "{:02x} ", byte)?;
            }
            Ok(())
        }

        let l = self.0.len();
        let (a, b) = if l <= CDebug::T {
            (self.0, None)
        } else {
            (&self.0[..CDebug::LH], Some((
                l - CDebug::LH - CDebug::LT,
                &self.0[l - CDebug::LT..]
             )))
        };

        cw(a, fmt)?;
        xw(a, fmt)?;
        if let Some((c, b)) = b {
            write!(fmt, " <{}/{} bytes> ", c, l)?;
            cw(b, fmt)?;
            xw(b, fmt)?;
        }
        Ok(())
    }
}

//----------------

/*
enum OwnedList<T> {
    Node(T, Box<OwnedList<T>>),
    Nil
}

impl<T> OwnedList<T> {
    fn empty() -> Self { OwnedList::Nil }
}
*/


#[cfg(test)]
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


    pub struct HexSlice<'a>(&'a [u8]);

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

}


