use substreams::Hex;
use substreams_ethereum::{pb::eth::v2::Block, Event};

pub fn format_hex(hex: &[u8]) -> String {
    format!("0x{}", Hex(hex).to_string())
}

// example
// impl FromB for SubmitBid
// which goes from a block -> Vec<Events>
// Which is Into<SubmitBid>
pub trait FromBlock: Sized + FromBlockAndAddress {
    fn from_block(
        block: substreams_ethereum::pb::eth::v2::Block,
        address: Option<String>,
    ) -> Vec<Self>
    where
        Self: Sized + FromBlockAndAddress,
    {
        Self::from_block_and_address(block, address)
    }
}

pub trait FromBlockAndAddress: Sized {
    fn from_block_and_address(
        block: substreams_ethereum::pb::eth::v2::Block,
        address: Option<String>,
    ) -> Vec<Self>;
}

// This Trait describes mapping from one protobuf to another
pub trait Map<I: FromBlock + Into<P>, P>: Sized + From<Vec<P>> {
    fn map(
        block: substreams_ethereum::pb::eth::v2::Block,
        address: Option<String>,
    ) -> Result<Self, substreams::errors::Error>
    where
        Self: Sized,
    {
        let events = I::from_block_and_address(block, address);
        let protos = events
            .into_iter()
            .map(|event| event.into())
            .collect::<Vec<_>>();

        Ok(protos.into())
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
