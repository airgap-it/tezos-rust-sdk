use chrono::NaiveDateTime;
use num_traits::{FromPrimitive, ToPrimitive};
use tezos_core::{
    internal::{
        coder::{ConsumingDecoder, Decoder, Encoder},
        consumable_list::{ConsumableBytes, ConsumableList},
        types::BytesTag,
        utils,
    },
    types::{
        encoded::{
            Address, BlockHash, BlockPayloadHash, ContextHash, Ed25519PublicKeyHash, Encoded,
            ImplicitAddress, NonceHash, OperationListListHash, ProtocolHash, PublicKey, Signature,
        },
        hex_string::HexString,
        mutez::Mutez,
        number::Nat,
    },
};
use tezos_michelson::micheline::Micheline;

use crate::{
    block_header::{BlockHeader, LiquidityBakingToggleVote},
    operations::{
        ActivateAccount, Ballot, BallotType, Delegation, DoubleBakingEvidence,
        DoubleEndorsementEvidence, DoublePreendorsementEvidence, Endorsement, Entrypoint,
        FailingNoop, InlinedEndorsement, InlinedPreendrosement, OperationContent,
        OperationContentTag, Origination, Parameters, Preendorsement, Proposals,
        RegisterGlobalConstant, Reveal, Script, SeedNonceRevelation, SetDepositsLimit,
        TraitOperationConsensusContent, TraitOperationContent, TraitOperationManagerContent,
        Transaction,
    },
    Error, Result,
};

pub struct OperationContentBytesCoder;

impl OperationContentBytesCoder {
    fn encode_manager_operation_content<Operation: TraitOperationManagerContent>(
        value: &Operation,
    ) -> Result<Vec<u8>> {
        let source_bytes = value.source().to_bytes()?;
        let fee_bytes = value.fee().to_bytes()?;
        let counter_bytes = value.counter().to_bytes()?;
        let gas_limit_bytes = value.gas_limit().to_bytes()?;
        let storage_limit_bytes = value.storage_limit().to_bytes()?;

        Ok([
            source_bytes,
            fee_bytes,
            counter_bytes,
            gas_limit_bytes,
            storage_limit_bytes,
        ]
        .concat())
    }

    fn require_consume_operation_content_tag<CL: ConsumableList<u8>>(
        tag: OperationContentTag,
        value: &mut CL,
    ) -> Result<()> {
        let tag_byte = value.consume_first()?;
        if tag_byte != tag as u8 {
            return Err(Error::InvalidOperationContentTag);
        }
        Ok(())
    }

    fn decode_consensus_operation<T: TraitOperationConsensusContent, F, CL: ConsumableList<u8>>(
        value: &mut CL,
        create: F,
    ) -> Result<T>
    where
        F: FnOnce(u16, i32, i32, BlockPayloadHash, &mut CL) -> T,
    {
        let slot = utils::decode_consuming_u16(value)?;
        let level = utils::decode_consuming_i32(value)?;
        let round = utils::decode_consuming_i32(value)?;
        let block_payload_hash = BlockPayloadHash::from_consumable_bytes(value)?;

        Ok(create(slot, level, round, block_payload_hash, value))
    }

    fn decode_manager_operation<T: TraitOperationManagerContent, F, CL: ConsumableList<u8>>(
        value: &mut CL,
        create: F,
    ) -> Result<T>
    where
        F: FnOnce(ImplicitAddress, Mutez, Nat, Nat, Nat, &mut CL) -> Result<T>,
    {
        let source = ImplicitAddress::from_consumable_bytes(value)?;
        let fee = Mutez::from_consumable_bytes(value)?;
        let counter = Nat::from_consumable_bytes(value)?;
        let gas_limit = Nat::from_consumable_bytes(value)?;
        let storage_limit = Nat::from_consumable_bytes(value)?;

        create(source, fee, counter, gas_limit, storage_limit, value)
    }
}

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
        let level_bytes = utils::encode_i32(value.level);
        let nonce_bytes = value.nonce.to_bytes();

        let tag = SeedNonceRevelation::tag().to_bytes();

        Ok([tag.as_slice(), &level_bytes, &nonce_bytes].concat())
    }
}

impl Encoder<DoubleEndorsementEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoubleEndorsementEvidence) -> Result<Vec<u8>> {
        let op1_bytes = utils::encode_bytes(&Self::encode(&value.op1)?);
        let op2_bytes = utils::encode_bytes(&Self::encode(&value.op2)?);

        let tag = DoubleEndorsementEvidence::tag().to_bytes();

        Ok([tag.as_slice(), &op1_bytes, &op2_bytes].concat())
    }
}

impl Encoder<InlinedEndorsement, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &InlinedEndorsement) -> Result<Vec<u8>> {
        let branch_bytes = (&value).branch.to_bytes()?;
        let operations_bytes = Self::encode(&value.operations)?;
        let signature_bytes = value.signature.to_bytes()?;

        Ok([branch_bytes, operations_bytes, signature_bytes].concat())
    }
}

impl Encoder<DoubleBakingEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoubleBakingEvidence) -> Result<Vec<u8>> {
        let bh1 = utils::encode_bytes(&Self::encode(&value.bh1)?);
        let bh2 = utils::encode_bytes(&Self::encode(&value.bh2)?);

        let tag = DoubleBakingEvidence::tag().to_bytes();

        Ok([tag.as_slice(), &bh1, &bh2].concat())
    }
}

impl Encoder<BlockHeader, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &BlockHeader) -> Result<Vec<u8>> {
        let level_bytes = utils::encode_i32(value.level);
        let proto_bytes = [value.proto];
        let predecessor_bytes = value.predecessor.to_bytes()?;
        let timestamp_bytes = utils::encode_i64(value.timestamp.timestamp_millis());
        let validation_pass_bytes = [value.validation_pass];
        let operation_hash_bytes = value.operations_hash.to_bytes()?;
        let fitness_bytes = utils::encode_bytes(&value.fitness.iter().fold(
            Vec::<u8>::new(),
            |mut acc, item| {
                acc.append(&mut utils::encode_bytes(&item.to_bytes()));
                acc
            },
        ));
        let context_bytes = value.context.to_bytes()?;
        let payload_hash_bytes = value.payload_hash.to_bytes()?;
        let payload_round_bytes = utils::encode_i32(value.payload_round);
        let proof_of_work_nonce_bytes = value.proof_of_work_nonce.to_bytes();
        let seed_nonce_hash_bytes = if let Some(value) = &value.seed_nonce_hash {
            value.to_bytes()?
        } else {
            vec![]
        };
        let seed_nonce_hash_presence = utils::encode_bool(!seed_nonce_hash_bytes.is_empty());
        let liquidity_baking_escape_vote_bytes =
            [value.liquidity_baking_toggle_vote.to_u8().unwrap()];
        let signature_bytes = value.signature.to_bytes()?;

        Ok([
            level_bytes.as_slice(),
            proto_bytes.as_slice(),
            predecessor_bytes.as_slice(),
            timestamp_bytes.as_slice(),
            validation_pass_bytes.as_slice(),
            operation_hash_bytes.as_slice(),
            fitness_bytes.as_slice(),
            context_bytes.as_slice(),
            payload_hash_bytes.as_slice(),
            payload_round_bytes.as_slice(),
            proof_of_work_nonce_bytes.as_slice(),
            seed_nonce_hash_presence.as_slice(),
            seed_nonce_hash_bytes.as_slice(),
            liquidity_baking_escape_vote_bytes.as_slice(),
            signature_bytes.as_slice(),
        ]
        .concat())
    }
}

impl Encoder<ActivateAccount, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &ActivateAccount) -> Result<Vec<u8>> {
        let pkh_bytes = value.pkh.to_bytes()?;
        let secret_bytes = value.secret.to_bytes();

        let tag = ActivateAccount::tag().to_bytes();

        Ok([tag.as_slice(), &pkh_bytes, &secret_bytes].concat())
    }
}

impl Encoder<Proposals, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Proposals) -> Result<Vec<u8>> {
        let source_bytes = (&value).source.to_bytes()?;
        let period_bytes = utils::encode_i32((&value).period);
        let proposals_bytes = utils::encode_list(&value.proposals)?;

        let tag = Proposals::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &source_bytes,
            &period_bytes,
            &proposals_bytes,
        ]
        .concat())
    }
}

impl Encoder<Ballot, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Ballot) -> Result<Vec<u8>> {
        let source_bytes = value.source.to_bytes()?;
        let period_bytes = utils::encode_i32(value.period);
        let proposal_bytes = value.proposal.to_bytes()?;
        let ballot_bytes = value.ballot.value();

        let tag = Ballot::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &source_bytes,
            &period_bytes,
            &proposal_bytes,
            ballot_bytes,
        ]
        .concat())
    }
}

impl Encoder<DoublePreendorsementEvidence, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &DoublePreendorsementEvidence) -> Result<Vec<u8>> {
        let op1_bytes = utils::encode_bytes(&Self::encode(&value.op1)?);
        let op2_bytes = utils::encode_bytes(&Self::encode(&value.op2)?);

        let tag = DoublePreendorsementEvidence::tag().to_bytes();

        Ok([tag.as_slice(), &op1_bytes, &op2_bytes].concat())
    }
}

impl Encoder<InlinedPreendrosement, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &InlinedPreendrosement) -> Result<Vec<u8>> {
        let branch_bytes = (&value).branch.to_bytes()?;
        let operations_bytes = Self::encode(&value.operations)?;
        let signature_byte = value.signature.to_bytes()?;

        Ok([branch_bytes, operations_bytes, signature_byte].concat())
    }
}

impl Encoder<FailingNoop, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &FailingNoop) -> Result<Vec<u8>> {
        let bytes = utils::encode_bytes(&value.arbitrary.to_bytes());

        let tag = FailingNoop::tag().to_bytes();

        Ok([tag.as_slice(), &bytes].concat())
    }
}

impl<ConsensusOperation: TraitOperationContent + TraitOperationConsensusContent>
    Encoder<ConsensusOperation, Vec<u8>, Error> for OperationContentBytesCoder
{
    fn encode(value: &ConsensusOperation) -> std::result::Result<Vec<u8>, Error> {
        let slot_bytes = utils::encode_u16(value.slot());
        let level_bytes = utils::encode_i32(value.level());
        let round_bytes = utils::encode_i32(value.round());
        let block_payload_hash_bytes = value.block_payload_hash().to_bytes()?;

        let tag = ConsensusOperation::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &slot_bytes,
            &level_bytes,
            &round_bytes,
            &block_payload_hash_bytes,
        ]
        .concat())
    }
}

impl Encoder<Reveal, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Reveal) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let public_key_bytes = value.public_key.to_bytes()?;

        let tag = Reveal::tag().to_bytes();

        Ok([tag.as_slice(), &content_bytes, &public_key_bytes].concat())
    }
}

impl Encoder<Transaction, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Transaction) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let amount_bytes = value.amount.to_bytes()?;
        let destination_bytes = value.destination.to_bytes()?;
        let parameters_bytes = if let Some(parameters) = &value.parameters {
            Self::encode(parameters)?
        } else {
            vec![]
        };
        let parameters_presence = utils::encode_bool(!parameters_bytes.is_empty());

        let tag = Transaction::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &content_bytes,
            &amount_bytes,
            &destination_bytes,
            &parameters_presence,
            &parameters_bytes,
        ]
        .concat())
    }
}

impl Encoder<Parameters, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Parameters) -> Result<Vec<u8>> {
        let entrypoint_bytes = Self::encode(&value.entrypoint)?;
        let value_bytes = utils::encode_bytes(&value.value.to_bytes()?);

        Ok([entrypoint_bytes, value_bytes].concat())
    }
}

impl Encoder<Entrypoint, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Entrypoint) -> std::result::Result<Vec<u8>, Error> {
        let tag = value.tag();
        if let Entrypoint::Named(value) = value {
            let bytes = value.as_bytes();
            if bytes.len() > u8::MAX as usize {
                return Err(Error::InvalidBytes);
            }
            let bytes_length = bytes.len() as u8;
            return Ok([&[tag], &[bytes_length], bytes].concat());
        }
        Ok(vec![tag])
    }
}

impl Encoder<Origination, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Origination) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let balance_bytes = (&value.balance).to_bytes()?;
        let delegate_bytes = if let Some(delegate) = &value.delegate {
            delegate.to_bytes()?
        } else {
            vec![]
        };
        let delegate_presence = utils::encode_bool(!delegate_bytes.is_empty());
        let script_bytes = Self::encode(&value.script)?;

        let tag = Origination::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &content_bytes,
            &balance_bytes,
            &delegate_presence,
            &delegate_bytes,
            &script_bytes,
        ]
        .concat())
    }
}

impl Encoder<Script, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Script) -> Result<Vec<u8>> {
        let code_bytes = utils::encode_bytes(&value.code.to_bytes()?);
        let storage_bytes = utils::encode_bytes(&value.storage.to_bytes()?);

        Ok([code_bytes, storage_bytes].concat())
    }
}

impl Encoder<Delegation, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &Delegation) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let delegate_bytes = if let Some(delegate) = &value.delegate {
            delegate.to_bytes()?
        } else {
            vec![]
        };
        let delegate_presence = utils::encode_bool(!delegate_bytes.is_empty());

        let tag = Delegation::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &content_bytes,
            &delegate_presence,
            &delegate_bytes,
        ]
        .concat())
    }
}

impl Encoder<RegisterGlobalConstant, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &RegisterGlobalConstant) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let value_bytes = utils::encode_bytes(&value.value.to_bytes()?);

        let tag = RegisterGlobalConstant::tag().to_bytes();

        Ok([tag.as_slice(), &content_bytes, &value_bytes].concat())
    }
}

impl Encoder<SetDepositsLimit, Vec<u8>, Error> for OperationContentBytesCoder {
    fn encode(value: &SetDepositsLimit) -> Result<Vec<u8>> {
        let content_bytes = Self::encode_manager_operation_content(value)?;
        let limit_bytes = if let Some(limit) = value.limit {
            limit.to_bytes()?
        } else {
            vec![]
        };
        let limit_presence = utils::encode_bool(!limit_bytes.is_empty());

        let tag = SetDepositsLimit::tag().to_bytes();

        Ok([
            tag.as_slice(),
            &content_bytes,
            &limit_presence,
            &limit_bytes,
        ]
        .concat())
    }
}

impl Decoder<OperationContent, [u8], Error> for OperationContentBytesCoder {
    fn decode(value: &[u8]) -> Result<OperationContent> {
        Self::decode_consuming(&mut ConsumableBytes::new(value))
    }
}

impl ConsumingDecoder<OperationContent, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<OperationContent> {
        let tag_byte = *value.inner_value().first().ok_or(Error::InvalidBytes)?;
        let tag =
            OperationContentTag::from_u8(tag_byte).ok_or(Error::InvalidOperationContentTag)?;
        match tag {
            OperationContentTag::SeedNonceRevelation => Ok(OperationContent::SeedNonceRevelation(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::DoubleEndorsementEvidence => Ok(
                OperationContent::DoubleEndorsementEvidence(Self::decode_consuming(value)?),
            ),
            OperationContentTag::DoubleBakingEvidence => Ok(
                OperationContent::DoubleBakingEvidence(Self::decode_consuming(value)?),
            ),
            OperationContentTag::ActivateAccount => Ok(OperationContent::ActivateAccount(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Proposals => {
                Ok(OperationContent::Proposals(Self::decode_consuming(value)?))
            }
            OperationContentTag::Ballot => {
                Ok(OperationContent::Ballot(Self::decode_consuming(value)?))
            }
            OperationContentTag::DoublePreendorsementEvidence => Ok(
                OperationContent::DoublePreendorsementEvidence(Self::decode_consuming(value)?),
            ),
            OperationContentTag::FailingNoop => Ok(OperationContent::FailingNoop(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Preendorsement => Ok(OperationContent::Preendorsement(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Endorsement => Ok(OperationContent::Endorsement(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Reveal => {
                Ok(OperationContent::Reveal(Self::decode_consuming(value)?))
            }
            OperationContentTag::Transaction => Ok(OperationContent::Transaction(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Origination => Ok(OperationContent::Origination(
                Self::decode_consuming(value)?,
            )),
            OperationContentTag::Delegation => {
                Ok(OperationContent::Delegation(Self::decode_consuming(value)?))
            }
            OperationContentTag::RegisterGlobalConstant => Ok(
                OperationContent::RegisterGlobalConstant(Self::decode_consuming(value)?),
            ),
            OperationContentTag::SetDepositsLimit => Ok(OperationContent::SetDepositsLimit(
                Self::decode_consuming(value)?,
            )),
        }
    }
}

impl ConsumingDecoder<SeedNonceRevelation, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<SeedNonceRevelation> {
        Self::require_consume_operation_content_tag(
            OperationContentTag::SeedNonceRevelation,
            value,
        )?;

        let level = utils::decode_consuming_i32(value)?;
        let nonce_bytes = value.consume_until(32)?;
        let nonce = HexString::from_bytes(nonce_bytes);

        Ok(SeedNonceRevelation::new(level, nonce))
    }
}

impl ConsumingDecoder<DoubleEndorsementEvidence, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(
        value: &mut CL,
    ) -> Result<DoubleEndorsementEvidence> {
        Self::require_consume_operation_content_tag(
            OperationContentTag::DoubleEndorsementEvidence,
            value,
        )?;
        let bytes = utils::decode_bytes(value)?;
        let mut op1_bytes = ConsumableBytes::new(&bytes);
        let op1: InlinedEndorsement = Self::decode_consuming(&mut op1_bytes)?;

        let bytes = utils::decode_bytes(value)?;
        let mut op2_bytes = ConsumableBytes::new(&bytes);
        let op2: InlinedEndorsement = Self::decode_consuming(&mut op2_bytes)?;

        Ok(DoubleEndorsementEvidence::new(op1, op2))
    }
}

impl ConsumingDecoder<InlinedEndorsement, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<InlinedEndorsement> {
        let branch = BlockHash::from_consumable_bytes(value)?;
        let operations: Endorsement = Self::decode_consuming(value)?;
        let signature = Signature::from_consumable_bytes(value)?;

        Ok(InlinedEndorsement::new(branch, operations, signature))
    }
}

impl ConsumingDecoder<DoubleBakingEvidence, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<DoubleBakingEvidence> {
        Self::require_consume_operation_content_tag(
            OperationContentTag::DoubleBakingEvidence,
            value,
        )?;
        let bytes = utils::decode_bytes(value)?;
        let mut bh1_bytes = ConsumableBytes::new(&bytes);
        let bh1: BlockHeader = Self::decode_consuming(&mut bh1_bytes)?;

        let bytes = utils::decode_bytes(value)?;
        let mut bh2_bytes = ConsumableBytes::new(&bytes);
        let bh2: BlockHeader = Self::decode_consuming(&mut bh2_bytes)?;

        Ok(DoubleBakingEvidence::new(bh1, bh2))
    }
}

impl ConsumingDecoder<BlockHeader, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<BlockHeader> {
        let level = utils::decode_consuming_i32(value)?;
        let proto = value.consume_first()?;
        let predecessor = BlockHash::from_consumable_bytes(value)?;
        let timestamp_millis = utils::decode_consuming_i64(value)?;

        let ts_secs = timestamp_millis / 1000;
        let ts_ns = (timestamp_millis % 1000) * 1_000_000;
        let timestamp = NaiveDateTime::from_timestamp_opt(ts_secs, ts_ns as u32)
            .expect("out-of-range number of seconds and/or invalid nanosecond");

        let validation_pass = value.consume_first()?;
        let operations_hash = OperationListListHash::from_consumable_bytes(value)?;

        let fitness_bytes = utils::decode_bytes(value)?;
        let mut fitness_bytes = ConsumableBytes::new(&fitness_bytes);
        let mut fitness = Vec::<HexString>::new();
        while !fitness_bytes.is_empty() {
            let bytes = utils::decode_bytes(&mut fitness_bytes)?;
            fitness.push(HexString::from_bytes(bytes))
        }

        let context = ContextHash::from_consumable_bytes(value)?;
        let payload_hash = BlockPayloadHash::from_consumable_bytes(value)?;
        let payload_round = utils::decode_consuming_i32(value)?;
        let proof_of_work_nonce = HexString::from_bytes(value.consume_until(8)?);

        let seed_nonce_hash_presence = utils::decode_consuming_bool(value)?;
        let seed_nonce_hash = if seed_nonce_hash_presence {
            Some(NonceHash::from_consumable_bytes(value)?)
        } else {
            None
        };

        let liquidity_baking_toggle_vote =
            LiquidityBakingToggleVote::from_u8(value.consume_first()?)
                .ok_or(Error::InvalidBytes)?;
        let signature = Signature::from_consumable_bytes(value)?;

        Ok(BlockHeader {
            level,
            proto,
            predecessor,
            timestamp,
            validation_pass,
            operations_hash,
            fitness,
            context,
            payload_hash,
            payload_round,
            proof_of_work_nonce,
            seed_nonce_hash,
            liquidity_baking_toggle_vote,
            signature,
        })
    }
}

impl ConsumingDecoder<ActivateAccount, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<ActivateAccount> {
        Self::require_consume_operation_content_tag(OperationContentTag::ActivateAccount, value)?;
        let pkh = Ed25519PublicKeyHash::from_consumable_bytes(value)?;
        let secret = HexString::from_bytes(value.consume_until(20)?);

        Ok(ActivateAccount::new(pkh, secret))
    }
}

impl ConsumingDecoder<Proposals, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Proposals> {
        Self::require_consume_operation_content_tag(OperationContentTag::Proposals, value)?;
        let source = ImplicitAddress::from_consumable_bytes(value)?;
        let period = utils::decode_consuming_i32(value)?;
        let proposals: Vec<ProtocolHash> = utils::decode_consuming_list(value)?;

        Ok(Proposals::new(source, period, proposals))
    }
}

impl ConsumingDecoder<Ballot, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Ballot> {
        Self::require_consume_operation_content_tag(OperationContentTag::Ballot, value)?;
        let source = ImplicitAddress::from_consumable_bytes(value)?;
        let period = utils::decode_consuming_i32(value)?;
        let proposal = ProtocolHash::from_consumable_bytes(value)?;
        let ballot = BallotType::from_value(&[value.consume_first()?])?;

        Ok(Ballot::new(source, period, proposal, ballot))
    }
}

impl ConsumingDecoder<DoublePreendorsementEvidence, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(
        value: &mut CL,
    ) -> Result<DoublePreendorsementEvidence> {
        Self::require_consume_operation_content_tag(
            OperationContentTag::DoublePreendorsementEvidence,
            value,
        )?;
        let bytes = utils::decode_bytes(value)?;
        let mut op1_bytes = ConsumableBytes::new(&bytes);
        let op1: InlinedPreendrosement = Self::decode_consuming(&mut op1_bytes)?;

        let bytes = utils::decode_bytes(value)?;
        let mut op2_bytes = ConsumableBytes::new(&bytes);
        let op2: InlinedPreendrosement = Self::decode_consuming(&mut op2_bytes)?;

        Ok(DoublePreendorsementEvidence::new(op1, op2))
    }
}

impl ConsumingDecoder<InlinedPreendrosement, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<InlinedPreendrosement> {
        let branch = BlockHash::from_consumable_bytes(value)?;
        let operations: Preendorsement = Self::decode_consuming(value)?;
        let signature = Signature::from_consumable_bytes(value)?;

        Ok(InlinedPreendrosement::new(branch, operations, signature))
    }
}

impl ConsumingDecoder<FailingNoop, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<FailingNoop> {
        Self::require_consume_operation_content_tag(OperationContentTag::FailingNoop, value)?;
        let bytes = utils::decode_bytes(value)?;
        let arbitrary = HexString::from_bytes(bytes);

        Ok(FailingNoop::new(arbitrary))
    }
}

impl ConsumingDecoder<Preendorsement, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Preendorsement> {
        Self::require_consume_operation_content_tag(OperationContentTag::Preendorsement, value)?;
        Self::decode_consensus_operation(value, |slot, level, round, block_payload_hash, _| {
            Preendorsement::new(slot, level, round, block_payload_hash)
        })
    }
}

impl ConsumingDecoder<Endorsement, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Endorsement> {
        Self::require_consume_operation_content_tag(OperationContentTag::Endorsement, value)?;
        Self::decode_consensus_operation(value, |slot, level, round, block_payload_hash, _| {
            Endorsement::new(slot, level, round, block_payload_hash)
        })
    }
}

impl ConsumingDecoder<Reveal, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Reveal> {
        Self::require_consume_operation_content_tag(OperationContentTag::Reveal, value)?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let public_key = PublicKey::from_consumable_bytes(value)?;

                Ok(Reveal::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    public_key,
                ))
            },
        )
    }
}

impl ConsumingDecoder<Transaction, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Transaction> {
        Self::require_consume_operation_content_tag(OperationContentTag::Transaction, value)?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let amount = Mutez::from_consumable_bytes(value)?;
                let destination = Address::from_consumable_bytes(value)?;

                let parameters_presence = utils::decode_consuming_bool(value)?;
                let parameters: Option<Parameters> = if parameters_presence {
                    Some(Self::decode_consuming(value)?)
                } else {
                    None
                };

                Ok(Transaction::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    amount,
                    destination,
                    parameters,
                ))
            },
        )
    }
}

impl ConsumingDecoder<Parameters, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Parameters> {
        let entrypoint: Entrypoint = Self::decode_consuming(value)?;
        let micheline_bytes = utils::decode_bytes(value)?;
        let value = Micheline::from_bytes(&micheline_bytes)?;

        Ok(Parameters::new(entrypoint, value))
    }
}

impl ConsumingDecoder<Entrypoint, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Entrypoint> {
        let tag_byte = value.consume_first()?;
        if let Some(entrypoint) = Entrypoint::from_tag(tag_byte) {
            return Ok(entrypoint);
        }
        if tag_byte == Entrypoint::named_tag() {
            let name_length = value.consume_first()?;
            let bytes = value.consume_until(name_length.into())?;
            let name = String::from_utf8(bytes.to_vec())?;
            return Ok(Entrypoint::Named(name));
        }
        Err(Error::InvalidBytes)
    }
}

impl ConsumingDecoder<Origination, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Origination> {
        Self::require_consume_operation_content_tag(OperationContentTag::Origination, value)?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let balance = Mutez::from_consumable_bytes(value)?;
                let delegate_presence = utils::decode_consuming_bool(value)?;
                let delegate = if delegate_presence {
                    Some(ImplicitAddress::from_consumable_bytes(value)?)
                } else {
                    None
                };

                let script: Script = Self::decode_consuming(value)?;

                Ok(Origination::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    balance,
                    delegate,
                    script,
                ))
            },
        )
    }
}

impl ConsumingDecoder<Script, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Script> {
        let code = Micheline::from_bytes(&utils::decode_bytes(value)?)?;
        let storage = Micheline::from_bytes(&utils::decode_bytes(value)?)?;

        Ok(Script::new(code.try_into()?, storage))
    }
}

impl ConsumingDecoder<Delegation, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<Delegation> {
        Self::require_consume_operation_content_tag(OperationContentTag::Delegation, value)?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let delegate_presence = utils::decode_consuming_bool(value)?;
                let delegate = if delegate_presence {
                    Some(ImplicitAddress::from_consumable_bytes(value)?)
                } else {
                    None
                };

                Ok(Delegation::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    delegate,
                ))
            },
        )
    }
}

impl ConsumingDecoder<RegisterGlobalConstant, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<RegisterGlobalConstant> {
        Self::require_consume_operation_content_tag(
            OperationContentTag::RegisterGlobalConstant,
            value,
        )?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let value = Micheline::from_bytes(&utils::decode_bytes(value)?)?;

                Ok(RegisterGlobalConstant::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    value,
                ))
            },
        )
    }
}

impl ConsumingDecoder<SetDepositsLimit, u8, Error> for OperationContentBytesCoder {
    fn decode_consuming<CL: ConsumableList<u8>>(value: &mut CL) -> Result<SetDepositsLimit> {
        Self::require_consume_operation_content_tag(OperationContentTag::SetDepositsLimit, value)?;
        Self::decode_manager_operation(
            value,
            |source, fee, counter, gas_limit, storage_limit, value| {
                let limit_presence = utils::decode_consuming_bool(value)?;
                let limit = if limit_presence {
                    Some(Mutez::from_consumable_bytes(value)?)
                } else {
                    None
                };

                Ok(SetDepositsLimit::new(
                    source,
                    fee,
                    counter,
                    gas_limit,
                    storage_limit,
                    limit,
                ))
            },
        )
    }
}
