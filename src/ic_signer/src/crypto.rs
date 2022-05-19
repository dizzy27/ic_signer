use k256::ecdsa::{
    digest::BlockInput,
    signature::digest::{
        generic_array::{typenum::U32, GenericArray},
        FixedOutputDirty, Reset, Update,
    },
};
use sha3::Digest;
use std::marker::PhantomData;

const HASH_256_MSG_LEN: usize = 32;

// Hash256 includes sha3::{Keccak256, Sha3_256}
#[derive(Clone)]
pub struct Hash256<A> {
    msg: Vec<u8>,
    hash: [u8; 32],
    marker: PhantomData<*const A>,
}

impl<A> BlockInput for Hash256<A> {
    type BlockSize = U32;
}

impl<A: Default> Default for Hash256<A> {
    fn default() -> Self {
        Hash256 {
            msg: Default::default(),
            hash: Default::default(),
            marker: PhantomData,
        }
    }
}

impl<A> Reset for Hash256<A> {
    fn reset(&mut self) {
        self.msg = Default::default();
        self.hash = Default::default();
    }
}

impl<A> TryFrom<&[u8]> for Hash256<A> {
    type Error = &'static str;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        if input.len() != HASH_256_MSG_LEN {
            Err("The length of message hash error")
        } else {
            let mut arr = [0; 32];
            arr.copy_from_slice(input);
            Ok(Hash256::<A> {
                msg: Default::default(),
                hash: arr,
                marker: PhantomData,
            })
        }
    }
}

impl<A> Update for Hash256<A> {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        let vec_u8 = data.as_ref();
        self.msg = [self.msg.clone().as_ref(), vec_u8].concat().to_vec();
    }
}

impl<A: Digest> FixedOutputDirty for Hash256<A> {
    type OutputSize = U32;
    fn finalize_into_dirty(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.hash != <[u8; 32]>::default() {
            out.copy_from_slice(&self.hash);
        } else {
            out.copy_from_slice(&<A as Digest>::digest(&self.msg))
        }
    }
}