use gmeta::{InOut, Metadata};
use gstd::{msg, Encode, MessageId}; 
use crate::services::service::{Action, AuctionInfo, Event, Error}; 

pub struct AuctionMetadata;

impl Metadata for AuctionMetadata {
    type Init = ();
    type Handle = InOut<Action, Result<Event, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = AuctionInfo;
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state(), 0).expect(
        "Failed to encode or reply with `<AuctionMetadata as Metadata>::State` from `state()`",
    );
}

fn common_state() -> <AuctionMetadata as Metadata>::State {
    static_mut_state().info()
}

fn static_mut_state() -> &'static mut Auction {
    unsafe { AUCTION.get_or_insert(Default::default()) }
}

fn reply(payload: impl Encode, value: u128) -> GstdResult<MessageId> {
    msg::reply(payload, value)
}
