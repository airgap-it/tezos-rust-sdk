use tezos_core::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
        utils,
    },
    types::encoded::Encoded,
};

use crate::{
    block_header::BlockHeader,
    operations::{
        ActivateAccount, Ballot, Delegation, DoubleBakingEvidence, DoubleEndorsementEvidence,
        DoublePreendorsementEvidence, Endorsement, FailingNoop, InlinedEndorsement,
        OperationContent, Origination, Preendorsement, Proposals, RegisterGlobalConstant, Reveal,
        SeedNonceRevelation, SetDepositsLimit, TraitOperationContent, Transaction,
    },
    Error, Result,
};

pub struct OperationContentBytesCoder;

impl Encoder<OperationContent, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &OperationContent) -> Result<Vec<u8>> {
        match value {
            OperationContent::SeedNonceRevelation(value) => Self::encode(value),
            OperationContent::DoubleEndorsementEvidence(value) => Self::encode(value),
            OperationContent::DoubleBakingEvidence(value) => Self::encode(value),
            OperationContent::ActivateAccount(value) => Self::encode(value),
            OperationContent::Proposals(value) => Self::encode(value),
            OperationContent::Ballot(value) => Self::encode(value),
            OperationContent::DoublePreendorsementEvidence(value) => Self::encode(value),
            OperationContent::FailingNoop(value) => Self::encode(value),
            OperationContent::Preendorsement(value) => Self::encode(value),
            OperationContent::Endorsement(value) => Self::encode(value),
            OperationContent::Reveal(value) => Self::encode(value),
            OperationContent::Transaction(value) => Self::encode(value),
            OperationContent::Origination(value) => Self::encode(value),
            OperationContent::Delegation(value) => Self::encode(value),
            OperationContent::RegisterGlobalConstant(value) => Self::encode(value),
            OperationContent::SetDepositsLimit(value) => Self::encode(value),
        }
    }
}

impl Encoder<SeedNonceRevelation, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &SeedNonceRevelation) -> Result<Vec<u8>> {
        let level_bytes = utils::encode_i32(value.level());
        let nonce_bytes = value.nonce().to_bytes();

        let tag = SeedNonceRevelation::tag();

        Ok([tag, &level_bytes, &nonce_bytes].concat())
    }
}

impl Encoder<DoubleEndorsementEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoubleEndorsementEvidence) -> Result<Vec<u8>> {
        let op1_bytes = utils::encode_bytes(&Self::encode(value.op1())?);
        let op2_bytes = utils::encode_bytes(&Self::encode(value.op2())?);

        let tag = DoubleEndorsementEvidence::tag();

        Ok([tag, &op1_bytes, &op2_bytes].concat())
    }
}

impl Encoder<InlinedEndorsement, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &InlinedEndorsement) -> Result<Vec<u8>> {
        let branch_bytes = value.branch().to_bytes()?;
        let operations_bytes = Self::encode(value.operations())?;
        let signature_bytes = value.signature().to_bytes()?;

        Ok([branch_bytes, operations_bytes, signature_bytes].concat())
    }
}

impl Encoder<DoubleBakingEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoubleBakingEvidence) -> Result<Vec<u8>> {
        let bh1 = utils::encode_bytes(&Self::encode(value.bh1())?);
        let bh2 = utils::encode_bytes(&Self::encode(value.bh2())?);

        let tag = DoubleBakingEvidence::tag();

        Ok([tag, &bh1, &bh2].concat())
    }
}

impl Encoder<BlockHeader, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &BlockHeader) -> Result<Vec<u8>> {
        let level_bytes = utils::encode_i32(value.level);
        let proto_bytes = &[value.proto];
        let predecessor_bytes = value.predecessor.to_bytes()?;
        let timestamp_bytes = utils::encode_i64(value.timestamp.timestamp_millis());

        todo!()
    }
}

impl Encoder<ActivateAccount, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &ActivateAccount) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Proposals, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Proposals) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Ballot, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Ballot) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<DoublePreendorsementEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoublePreendorsementEvidence) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<FailingNoop, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &FailingNoop) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Preendorsement, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Preendorsement) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Endorsement, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Endorsement) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Reveal, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Reveal) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Transaction, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Transaction) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Origination, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Origination) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<Delegation, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Delegation) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<RegisterGlobalConstant, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &RegisterGlobalConstant) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Encoder<SetDepositsLimit, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &SetDepositsLimit) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Decoder<OperationContent, [u8], Error> for OperationContentBytesCoder {
    fn decode(value: &[u8]) -> Result<OperationContent> {
        Self::decode_consuming(&mut ConsumableBytes::new(value))
    }
}

impl ConsumingDecoder<OperationContent, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<OperationContent> {
        todo!()
    }
}
