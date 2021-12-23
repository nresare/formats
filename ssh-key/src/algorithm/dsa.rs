//! Digital Signature Algorithm (DSA).

use crate::{base64, MPInt, Result};

/// Digital Signature Algorithm (DSA) public key.
///
/// Described in [FIPS 186-4](https://csrc.nist.gov/publications/detail/fips/186/4/final).
// TODO(tarcieri): use `dsa::PublicKey`? (doesn't exist yet)
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct DsaPublicKey {
    /// Prime modulus
    pub p: MPInt,

    /// Prime divisor of `p - 1`
    pub q: MPInt,

    /// Generator of a subgroup of order `q` in the multiplicative group
    /// `GF(p)`, such that `1 < g < p`.
    pub g: MPInt,

    /// The public key, where `y = gˣ mod p`
    pub y: MPInt,
}

impl DsaPublicKey {
    /// Decode DSA public key using the provided Base64 decoder.
    pub(crate) fn decode(decoder: &mut base64::Decoder<'_>) -> Result<Self> {
        let p = MPInt::decode(decoder)?;
        let q = MPInt::decode(decoder)?;
        let g = MPInt::decode(decoder)?;
        let y = MPInt::decode(decoder)?;
        Ok(Self { p, q, g, y })
    }
}