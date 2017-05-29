extern crate futures;
extern crate tokio_io as tio;
extern crate num;

use futures::Stream;
use futures::sync::mpsc::{Receiver, Sender};
use std::io::Cursor;
use tio::{AsyncRead, AsyncWrite};
use tio::io::{ReadHalf, WriteHalf};

use num::PrimInt;

/// Contains the permission level of a core. Only used for
/// hardware security and control, not priority handling or optimization.
pub struct Permission<Word> {
    /// Specifies the mask of the network this core has permission over.
    pub ownership: Word,
    /// Specifies the specific address of this core. Addresses are not necessarily unique.
    pub network: Word,
}

pub struct InBus<'a, Word> {
    pub inceptions: Vec<Receiver<Permission<Word>>>,
    pub kills: Vec<Receiver<Permission<Word>>>,
    pub message: Vec<Receiver<Word>>,
    pub streams: Vec<Receiver<ReadHalf<Cursor<&'a mut [u8]>>>>,
}

pub struct OutBus<'a, Word> {
    pub inceptions: Vec<Sender<Permission<Word>>>,
    pub kills: Vec<Sender<Permission<Word>>>,
    pub message: Vec<Sender<Word>>,
    pub streams: Vec<Sender<WriteHalf<Cursor<&'a mut [u8]>>>>,
}

pub trait Uarc {
    type Word: PrimInt;

    /// Operate the core to completion in a blocking manner.
    fn operate<'a>(&'a mut self, input: InBus<'a, Self::Word>, output: OutBus<'a, Self::Word>);

    /// Get the permission of this UARC core.
    fn permission(&self) -> Permission<Self::Word>;

    /// See if an external communication has permission to control this core.
    fn permit(&self, perm: Permission<Self::Word>) -> bool {
        let self_perm = self.permission();
        self_perm.ownership & self_perm.network == self_perm.ownership & perm.network
    }
}

