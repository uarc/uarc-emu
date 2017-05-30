extern crate futures;
extern crate tokio_io as tio;
extern crate num;

use futures::{Stream, Async};
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

#[derive(Default, Debug)]
pub struct InBus<'a, Word> {
    pub inceptions: Vec<Receiver<Permission<Word>>>,
    pub kills: Vec<Receiver<Permission<Word>>>,
    pub message: Vec<Receiver<Word>>,
    pub streams: Vec<Receiver<ReadHalf<Cursor<&'a mut [u8]>>>>,
}

#[derive(Default, Debug)]
pub struct OutBus<'a, Word> {
    pub inceptions: Vec<Sender<Permission<Word>>>,
    pub kills: Vec<Sender<Permission<Word>>>,
    pub message: Vec<Sender<Word>>,
    pub streams: Vec<Sender<WriteHalf<Cursor<&'a mut [u8]>>>>,
}

/// An implementer of this trait is UARC compatible and can connect to other cores over the simulated bus.
pub trait Uarc {
    type Word: PrimInt;

    /// Operate the core to completion in a blocking manner.
    fn operate<'a>(&'a mut self, input: InBus<'a, Self::Word>, output: OutBus<'a, Self::Word>);

    // TODO: Add method to do a single cycle.

    /// Get the permission of this UARC core.
    fn permission(&self) -> Permission<Self::Word>;

    /// See if an external communication has permission to control this core.
    fn permit(&self, perm: Permission<Self::Word>) -> bool {
        let self_perm = self.permission();
        self_perm.ownership & self_perm.network == self_perm.ownership & perm.network
    }

    /// Look for an oncoming inception and give an `Option` of its bus ID.
    fn poll_inception<'a>(&'a self, input: &mut InBus<'a, Self::Word>) -> Option<(Self::Word, Permission<Self::Word>)> {
        for (ix, channel) in input.inceptions.iter_mut().enumerate() {
            if let Ok(Async::Ready(Some(perm))) = channel.poll() {
                if self.permit(perm) {
                    return Some((NumCast::from(ix).expect("Error: Unable to convert bus ID into word!"), perm));
                }
            }
        }
        None
    }
}

