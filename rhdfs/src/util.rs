
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

use std::fmt::Debug;

/// Generates (via `f`) and handles `SnV`, plus trace
#[inline]
pub fn logging_switch_state_f<S: Debug, V: Debug, F>(s: &mut S, f: F) -> V where F: FnOnce(&mut S) -> SnV<S, V> {
    let snv = f(s);
    trace!(target: "switch_state_f", "switch-state: {:?} => {:?}", s, snv);
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
pub fn logging_fsm_turn<S: Debug, V: Debug, F>(s: &mut S, mut f: F) -> V where F: FnMut(&mut S) -> SV<S, V> {
    loop {
        let sv = f(s);
        trace!(target: "fsm_turn", "fsm_turn: {:?} => {:?}", s, sv);
        match sv {
            SV::S(ns) => *s = ns,
            SV::V(v) => break v
        }
    }
}
