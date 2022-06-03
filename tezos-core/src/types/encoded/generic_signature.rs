use crate::{
    internal::coder::{encoded::encoded_bytes_coder::EncodedBytesCoder, Encoder},
    types::encoded::{Ed25519Signature, Encoded, MetaEncoded, P256Signature, Secp256K1Signature},
    Error, Result,
};

#[derive(Debug)]
pub struct GenericSignature {
    base58: String,
}

impl GenericSignature {
    pub fn is_valid_base58(value: &str) -> bool {
        META.is_valid_base58(value)
    }

    pub fn is_valid_bytes(value: &[u8]) -> bool {
        META.is_valid_bytes(value)
    }

    pub fn is_valid_prefixed_bytes(value: &[u8]) -> bool {
        META.is_valid_prefixed_bytes(value)
    }
}

impl Encoded for GenericSignature {
    fn base58(&self) -> &str {
        &self.base58
    }
    fn meta(&self) -> &MetaEncoded {
        &META
    }
    fn new(base58: String) -> Result<Self> {
        if META.is_valid_base58(&base58) {
            return Ok(GenericSignature { base58 });
        }
        return Err(Error::InvalidBase58EncodedData);
    }
}

pub const META: MetaEncoded = MetaEncoded {
    base58_prefix: "sig",
    base58_length: 96,
    bytes_prefix: &[4, 130, 43],
    bytes_length: 64,
};

impl TryFrom<&Vec<u8>> for GenericSignature {
    type Error = Error;

    fn try_from(value: &Vec<u8>) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(value, &META)
    }
}

impl TryFrom<[u8; META.bytes_length]> for GenericSignature {
    type Error = Error;

    fn try_from(value: [u8; META.bytes_length]) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.decode_with_meta(&value, &META)
    }
}

impl TryFrom<String> for GenericSignature {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        GenericSignature::new(value)
    }
}

impl TryFrom<&str> for GenericSignature {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        GenericSignature::new(value.to_string())
    }
}

impl TryFrom<&Ed25519Signature> for GenericSignature {
    type Error = Error;

    fn try_from(value: &Ed25519Signature) -> Result<Self> {
        let bytes = value.to_bytes()?;
        (&bytes).try_into()
    }
}

impl TryFrom<&Secp256K1Signature> for GenericSignature {
    type Error = Error;

    fn try_from(value: &Secp256K1Signature) -> Result<Self> {
        let bytes = value.to_bytes()?;
        (&bytes).try_into()
    }
}

impl TryFrom<&P256Signature> for GenericSignature {
    type Error = Error;

    fn try_from(value: &P256Signature) -> Result<Self> {
        let bytes = value.to_bytes()?;
        (&bytes).try_into()
    }
}

impl TryFrom<&GenericSignature> for Vec<u8> {
    type Error = Error;

    fn try_from(value: &GenericSignature) -> Result<Self> {
        let coder = EncodedBytesCoder::new();
        coder.encode(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_from_string() -> Result<()> {
        let value: GenericSignature = "sigS9AYGk12AjCHEPowcEhrURh1Stk2TyqqDEKqGEPLBTeAsgq8ZRgPdDc4TvxMEy8PLBqf4BtDptD4jY5o9yjJzSqpeqPpS".try_into()?;
        assert_eq!(
            value.base58(),
            "sigS9AYGk12AjCHEPowcEhrURh1Stk2TyqqDEKqGEPLBTeAsgq8ZRgPdDc4TvxMEy8PLBqf4BtDptD4jY5o9yjJzSqpeqPpS"
        );
        assert_eq!(
            value.to_bytes()?,
            [
                31, 190, 78, 29, 238, 27, 255, 115, 99, 209, 110, 112, 35, 100, 193, 115, 218, 145,
                132, 232, 30, 75, 195, 172, 85, 230, 127, 133, 215, 71, 44, 244, 12, 23, 159, 110,
                63, 239, 226, 87, 158, 196, 71, 85, 230, 88, 73, 110, 127, 78, 130, 222, 143, 88,
                55, 85, 254, 62, 242, 157, 247, 151, 88, 6
            ]
        );

        Ok(())
    }

    #[test]
    fn test_convert_from_bytes() -> Result<()> {
        let value: GenericSignature = [
            31, 190, 78, 29, 238, 27, 255, 115, 99, 209, 110, 112, 35, 100, 193, 115, 218, 145,
            132, 232, 30, 75, 195, 172, 85, 230, 127, 133, 215, 71, 44, 244, 12, 23, 159, 110, 63,
            239, 226, 87, 158, 196, 71, 85, 230, 88, 73, 110, 127, 78, 130, 222, 143, 88, 55, 85,
            254, 62, 242, 157, 247, 151, 88, 6,
        ]
        .try_into()?;
        assert_eq!(
            value.base58(),
            "sigS9AYGk12AjCHEPowcEhrURh1Stk2TyqqDEKqGEPLBTeAsgq8ZRgPdDc4TvxMEy8PLBqf4BtDptD4jY5o9yjJzSqpeqPpS"
        );

        Ok(())
    }
}
