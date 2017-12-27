extern crate futures;
extern crate num;

use num::PrimInt;
use futures::{Sink, Future};
use futures::sync::mpsc::*;

#[derive(Debug)]
pub enum Data<W> {
    Msg(W),
    Stream(Receiver<W>),
}

/// An implementer of this trait is UARC compatible and can connect to other cores over the simulated bus.
pub trait Uarc {
    type Word: PrimInt;

    /// Obtains a new bus which can send data to this core.
    /// This requires the bus ID to be provided for the receiving core.
    /// The implementor should panic if two buses are asked for with the same ID.
    fn make_bus(&self, id: Self::Word) -> Bus<Self::Word>;
}

/// Wraps a Sender to provide the basic UARC semantics.
pub struct Bus<W> {
    sender: Option<Sender<Data<W>>>,
}

impl<W> Bus<W> {
    pub fn new(sender: Sender<Data<W>>) -> Bus<W> {
        Bus { sender: Some(sender) }
    }

    pub fn msg(&mut self, data: Data<W>) {
        self.sender = Some(self.sender.take().expect("error: uarc-emu::Bus::sender in invalid state")
            .send(data).wait().expect("error: uarc-emu::Bus::msg() failed to send word"));
    }

    pub fn send(&mut self, word: W) {
        self.msg(Data::Msg(word));
    }

    pub fn stream(&mut self, receiver: Receiver<W>) {
        self.msg(Data::Stream(receiver));
    }
}

