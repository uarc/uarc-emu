extern crate futures;
extern crate tokio_io as tio;
extern crate num;

mod echo;
pub use echo::EchoCore;

use futures::{Stream, Sink, Future};
use futures::sync::mpsc::{Receiver, Sender};
use std::io::Cursor;
use tio::io::{ReadHalf, WriteHalf};

use num::PrimInt;
use num::cast::NumCast;

/// Contains the permission level of a core. Only used for
/// hardware security and control, not priority handling or optimization.
#[derive(Default, Debug, Clone, Copy)]
pub struct Permission<Word> {
    /// Specifies the mask of the network this core has permission over.
    pub ownership: Word,
    /// Specifies the specific address of this core. Addresses are not necessarily unique.
    pub network: Word,
}

impl<Word> Permission<Word> where Word: PrimInt {
    pub fn has_privilege(&self, other: Self) -> bool {
        self.ownership & self.network == self.ownership & other.network
    }
}

#[derive(Debug, Clone)]
pub enum Data<S> {
    Incept,
    Kill,
    Message,
    Stream(S),
}

/// `Word` is the bus word for this implementation. `Stream` is the stream used to stream data words.
#[derive(Debug, Clone)]
pub struct Transport<Word, Stream> {
    /// The ID of the bus.
    pub bus: Word,
    /// The permission of the sender.
    pub permission: Permission<Word>,
    /// The actual data sent.
    pub data: Data<Stream>,
}

/// An implementer of this trait is UARC compatible and can connect to other cores over the simulated bus.
pub trait Uarc {
    type Word: PrimInt;
    type Sink: Sink<SinkItem=Self::Word>;

    /// Obtains a new sink which can send data to this core.
    fn sink(&self) -> Self::Sink;
}

