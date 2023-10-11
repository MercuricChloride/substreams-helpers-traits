use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::Block, Event};

pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

/// A trait that allows us to extract a type from an ethereum block.
/// In most cases, this will be an event.
///
/// This won't be used directly most of the time, instead it will be used
/// the macro helpers I am writing.
pub trait FromBlock: Sized + Event {
    /// Attempts to extract this type from an ethereum block.
    /// You can optionally pass in an address to filter the logs by.
    /// Just note that this string expects a 0x prefixed address.
    fn from_block(
        block: substreams_ethereum::pb::eth::v2::Block,
        address: Option<&str>,
    ) -> Vec<Self>
    where
        Self: Sized + substreams_ethereum::Event,
    {
        block
            .logs()
            .filter(|log| {
                if let Some(address) = address {
                    log.address() == Hex::decode(address).expect("Failed to decode address")
                } else {
                    true
                }
            })
            .filter_map(|log| Self::match_and_decode(log))
            .collect::<Vec<Self>>()
    }
}

/// A trait that allows us to convert an event into another type. This again
/// will be used by the macro helpers I am writing.
pub trait FromEvent<T: substreams_ethereum::Event>: Sized {
    /// Attempts to create this type from an event.
    /// Returning `None` if the event does not contain this type.
    fn from_event(event: T) -> Option<Self>
    where
        Self: Sized;
}

// This Trait describes mapping from one protobuf to another
pub trait Map<E: FromBlock + substreams_ethereum::Event, P: From<E>>: Sized + From<Vec<P>> {
    fn map(
        block: substreams_ethereum::pb::eth::v2::Block,
        address: Option<&str>,
    ) -> Result<Self, substreams::errors::Error>
    where
        Self: Sized,
    {
        let events = E::from_block(block, address);
        let proto = events
            .into_iter()
            .map(|event| P::from(event))
            .collect::<Vec<_>>();

        let proto = proto.into();
        Ok(proto)
    }
}

/// A trait that allows us to store this type in a store module.
/// This is abstract, because what kind of store we are interacting with depends
/// on the type of the store.
pub trait Store<T> {
    /// Stores this type in the store.
    fn store(&self, store: &T);
}

/// A trait that allows us to store this type in a store module, but only
/// for a certain duration, before it is cleared.
pub trait TimeStore<T> {
    /// Stores this type in the store for a certain duration.
    fn store(&self, store: &T);
    /// Clears the store.
    fn clear(&self);
}

#[derive(derive_more::Into)]
pub struct StringW(String);

impl From<Vec<u8>> for StringW {
    fn from(value: Vec<u8>) -> Self {
        StringW(format_hex(&value))
    }
}

impl From<substreams::scalar::BigInt> for StringW {
    fn from(value: substreams::scalar::BigInt) -> Self {
        StringW(value.to_string())
    }
}
