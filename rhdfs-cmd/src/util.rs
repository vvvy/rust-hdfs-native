use std::marker::PhantomData;
use futures::prelude::*;


pub struct ForEachSink<F, T, E> {
    f: F,
    t_type: PhantomData<T>,
    e_type: PhantomData<E>,
}

impl<F, T, E> ForEachSink<F, T, E> {
    pub fn new(f: F) -> ForEachSink<F, T, E> { ForEachSink { f, t_type: PhantomData, e_type: PhantomData } }
}

impl<F, T, E> Sink for ForEachSink<F, T, E> where F: FnMut(T) -> Result<(), E> {
    type SinkItem = T;
    type SinkError = E;

    fn start_send(&mut self, item: T) -> Result<AsyncSink<T>, E> {
        (self.f)(item).map(|_| AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, E> {
        Ok(Async::Ready(()))
    }

    fn close(&mut self) -> Result<Async<()>, E> {
        Ok(Async::Ready(()))
    }
}
