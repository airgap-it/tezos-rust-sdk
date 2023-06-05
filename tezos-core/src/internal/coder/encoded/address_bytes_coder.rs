use core::ops::Add;

use crate::{
    internal::{
        coder::{
            encoded::implicit_address_bytes_coder::ImplicitAddressBytesCoder, ConsumingDecoder,
            Decoder, Encoder,
        },
        consumable_list::ConsumableList,
        types::BytesTag,
    },
    types::encoded::Address,
    Error, Result,
};
use alloc::vec::Vec;

use super::contract_address_bytes_coder::ContractAddressBytesCoder;

pub struct AddressBytesCoder;

impl Encoder<Address, Vec<u8>, Error> for AddressBytesCoder {
    fn encode(value: &Address) -> Result<Vec<u8>> {
        match value {
            Address::Implicit(address) => {
                Ok(AddressTag::Implicit + ImplicitAddressBytesCoder::encode(address)?)
            }
            Address::Originated(address) => {
                Ok(AddressTag::Originated + ContractAddressBytesCoder::encode(address)?)
            }
        }
    }
}

impl Decoder<Address, [u8], Error> for AddressBytesCoder {
    fn decode(value: &[u8]) -> Result<Address> {
        let tag = AddressTag::recognize(&value).ok_or(Error::InvalidBytes)?;
        let bytes = value[tag.value().len()..].to_vec();

        match tag {
            AddressTag::Implicit => Ok(Address::Implicit(ImplicitAddressBytesCoder::decode(
                &bytes,
            )?)),
            AddressTag::Originated => Ok(Address::Originated(ContractAddressBytesCoder::decode(
                &bytes,
            )?)),
        }
    }
}

impl ConsumingDecoder<Address, u8, Error> for AddressBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Address> {
        let tag_byte = value.consume_first()?;
        let tag = AddressTag::recognize(&[tag_byte]).ok_or(Error::InvalidBytes)?;

        match tag {
            AddressTag::Implicit => Ok(Address::Implicit(
                ImplicitAddressBytesCoder::decode_consuming(value)?,
            )),
            AddressTag::Originated => Ok(Address::Originated(
                ContractAddressBytesCoder::decode_consuming(value)?,
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AddressTag {
    Implicit,
    Originated,
}

impl AddressTag {
    fn recognize(bytes: &[u8]) -> Option<Self> {
        Self::values()
            .iter()
            .find(|item| bytes.starts_with(item.value()))
            .map(|tag| *tag)
    }

    fn values() -> &'static [AddressTag] {
        &[AddressTag::Implicit, AddressTag::Originated]
    }
}

impl BytesTag for AddressTag {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Implicit => &[0],
            Self::Originated => &[1],
        }
    }

    fn prefixed_to(&self, bytes: &[u8]) -> Vec<u8> {
        [self.value(), bytes].concat()
    }
}

impl Add<Vec<u8>> for AddressTag {
    type Output = Vec<u8>;

    fn add(self, rhs: Vec<u8>) -> Self::Output {
        self.prefixed_to(&rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::encoded::Encoded;

    #[test]
    fn test_encode_1() -> Result<()> {
        let address: Address = "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e".try_into()?;
        let bytes = AddressBytesCoder::encode(&address)?;
        assert_eq!(
            bytes,
            [
                0, 0, 215, 166, 13, 78, 144, 232, 163, 62, 200, 53, 21, 145, 145, 177, 76, 228, 69,
                47, 18, 248
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_1() -> Result<()> {
        let bytes = [
            0, 0, 215, 166, 13, 78, 144, 232, 163, 62, 200, 53, 21, 145, 145, 177, 76, 228, 69, 47,
            18, 248,
        ]
        .to_vec();
        let address = AddressBytesCoder::decode(&bytes)?;
        assert_eq!(address.value(), "tz1fJGtrdmckD3VkiDxqUEci5h4gGcvocw6e");
        Ok(())
    }

    #[test]
    fn test_encode_2() -> Result<()> {
        let address: Address = "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into()?;
        let bytes = AddressBytesCoder::encode(&address)?;
        assert_eq!(
            bytes,
            [
                0, 1, 26, 134, 135, 137, 33, 97, 18, 25, 75, 59, 176, 3, 232, 56, 76, 143, 114, 23,
                178, 136
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_2() -> Result<()> {
        let bytes = [
            0, 1, 26, 134, 135, 137, 33, 97, 18, 25, 75, 59, 176, 3, 232, 56, 76, 143, 114, 23,
            178, 136,
        ]
        .to_vec();
        let address = AddressBytesCoder::decode(&bytes)?;
        assert_eq!(address.value(), "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy");
        Ok(())
    }

    #[test]
    fn test_encode_3() -> Result<()> {
        let address: Address = "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B".try_into()?;
        let bytes = AddressBytesCoder::encode(&address)?;
        assert_eq!(
            bytes,
            [
                0, 2, 26, 120, 244, 51, 42, 111, 225, 91, 151, 153, 4, 198, 194, 229, 249, 82, 30,
                31, 252, 74
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_3() -> Result<()> {
        let bytes = [
            0, 2, 26, 120, 244, 51, 42, 111, 225, 91, 151, 153, 4, 198, 194, 229, 249, 82, 30, 31,
            252, 74,
        ]
        .to_vec();
        let address = AddressBytesCoder::decode(&bytes)?;
        assert_eq!(address.value(), "tz3Nk25g51knuzFZZz2DeA5PveaQYmCtV68B");
        Ok(())
    }

    #[test]
    fn test_encode_4() -> Result<()> {
        let address: Address = "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7".try_into()?;
        let bytes = AddressBytesCoder::encode(&address)?;
        assert_eq!(
            bytes,
            [
                1, 96, 119, 205, 152, 253, 138, 202, 148, 133, 27, 131, 164, 196, 66, 3, 183, 5,
                210, 0, 75, 0
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_4() -> Result<()> {
        let bytes = [
            1, 96, 119, 205, 152, 253, 138, 202, 148, 133, 27, 131, 164, 196, 66, 3, 183, 5, 210,
            0, 75, 0,
        ]
        .to_vec();
        let address = AddressBytesCoder::decode(&bytes)?;
        assert_eq!(address.value(), "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7");
        Ok(())
    }

    #[test]
    fn test_encode_5() -> Result<()> {
        let address: Address = "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7%mint".try_into()?;
        let bytes = AddressBytesCoder::encode(&address)?;
        assert_eq!(
            bytes,
            [
                1, 96, 119, 205, 152, 253, 138, 202, 148, 133, 27, 131, 164, 196, 66, 3, 183, 5,
                210, 0, 75, 0, 109, 105, 110, 116
            ]
        );
        Ok(())
    }

    #[test]
    fn test_decode_5() -> Result<()> {
        let bytes = [
            1, 96, 119, 205, 152, 253, 138, 202, 148, 133, 27, 131, 164, 196, 66, 3, 183, 5, 210,
            0, 75, 0, 109, 105, 110, 116,
        ]
        .to_vec();
        let address = AddressBytesCoder::decode(&bytes)?;
        assert_eq!(address.value(), "KT1HNqxFJxnmUcX8wF915wxxaAAU4ixDwWQ7%mint");
        Ok(())
    }
}
