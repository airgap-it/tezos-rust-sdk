mod macros;
mod sequence;

use macros::{make_instruction, make_instructions};

make_instructions!(
    (Never, NEVER, never, 121),
    (Swap, SWAP, swap, 76),
    (GetAndUpdate, GET_AND_UPDATE, get_and_update, 140),
    (Apply, APPLY, apply, 115),
    (FailWith, FAILWITH, failwith, 39),
    (
        Rename,
        RENAME,
        rename,
        88,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Car,
        CAR,
        car,
        22,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Cast,
        CAST,
        cast,
        87,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Cdr,
        CDR,
        cdr,
        23,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Cons,
        CONS,
        cons,
        27,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Exec,
        EXEC,
        exec,
        38,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Mem,
        MEM,
        mem,
        57,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Size,
        SIZE,
        size,
        69,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Some,
        SOME,
        some,
        70,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata
    ),
    (
        Unit,
        UNIT,
        unit,
        79,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata
    ),
    (Dig, DIG, dig, 112, (n: crate::michelson::data::Nat)),
    (
        Drop,
        DROP,
        drop,
        32,
        optional: (n: crate::michelson::data::Nat)
    ),
    (Dug, DUG, dug, 113, (n: crate::michelson::data::Nat)),
    (
        Iter,
        ITER,
        iter,
        82,
        (expression: crate::michelson::data::instructions::Sequence)
    ),
    (
        LoopLeft,
        LOOP_LEFT,
        loop_left,
        83,
        (body: crate::michelson::data::instructions::Sequence)
    ),
    (
        Loop,
        LOOP,
        r#loop,
        52,
        (body: crate::michelson::data::instructions::Sequence)
    ),
    (
        Concat,
        CONCAT,
        concat,
        26,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (Slice, SLICE, slice, 111),
    (Pack, PACK, pack, 12),
    (
        Unpack,
        UNPACK,
        unpack,
        13,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Add,
        ADD,
        add,
        18,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Dip,
        DIP,
        dip,
        31,
        (instruction: crate::michelson::data::instructions::Sequence),
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        Dup,
        DUP,
        dup,
        33,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        EmptyBigMap,
        EMPTY_BIG_MAP,
        empty_big_map,
        114,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (key_type: crate::michelson::types::Type),
        (value_type: crate::michelson::types::Type)
    ),
    (
        EmptyMap,
        EMPTY_MAP,
        empty_map,
        35,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (key_type: crate::michelson::types::Type),
        (value_type: crate::michelson::types::Type)
    ),
    (
        EmptySet,
        EMPTY_SET,
        empty_set,
        36,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Get,
        GET,
        get,
        41,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        IfCons,
        IF_CONS,
        if_cons,
        45,
        (if_branch: crate::michelson::data::instructions::Sequence),
        (else_branch: crate::michelson::data::instructions::Sequence)
    ),
    (
        IfLeft,
        IF_LEFT,
        if_left,
        46,
        (if_branch: crate::michelson::data::instructions::Sequence),
        (else_branch: crate::michelson::data::instructions::Sequence)
    ),
    (
        IfNone,
        IF_NONE,
        if_none,
        47,
        (if_branch: crate::michelson::data::instructions::Sequence),
        (else_branch: crate::michelson::data::instructions::Sequence)
    ),
    (
        If,
        IF,
        r#if,
        44,
        (if_branch: crate::michelson::data::instructions::Sequence),
        (else_branch: crate::michelson::data::instructions::Sequence)
    ),
    (
        Lambda,
        LAMBDA,
        lambda,
        49,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        (parameter_type: crate::michelson::types::Type),
        (return_type: crate::michelson::types::Type),
        (body: crate::michelson::data::instructions::Sequence)
    ),
    (
        Left,
        LEFT,
        left,
        51,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Right,
        RIGHT,
        right,
        68,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Map,
        MAP,
        map,
        56,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        (expression: crate::michelson::data::instructions::Sequence)
    ),
    (
        Nil,
        NIL,
        nil,
        61,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        None,
        NONE,
        none,
        62,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (
        Pair,
        PAIR,
        pair,
        66,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        Unpair,
        UNPAIR,
        unpair,
        122,
        metadata_type: crate::michelson::metadata::TypeVariableMetadata,
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        Push,
        PUSH,
        push,
        67,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        (r#type: crate::michelson::types::Type),
        boxed: (value: crate::michelson::Data)
    ),
    (
        Update,
        UPDATE,
        update,
        80,
        metadata_type: crate::michelson::metadata::VariableMetadata,
        optional: (n: crate::michelson::data::Nat)
    ),
    (
        Sub,
        SUB,
        sub,
        75,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        SubMutez,
        SUB_MUTEZ,
        sub_mutez,
        147,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Mul,
        MUL,
        mul,
        58,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Ediv,
        EDIV,
        ediv,
        34,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Abs,
        ABS,
        abs,
        17,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        IsNat,
        ISNAT,
        isnat,
        86,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Int,
        INT,
        int,
        48,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Neg,
        NEG,
        neg,
        59,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Lsl,
        LSL,
        lsl,
        53,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Lsr,
        LSR,
        lsr,
        54,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Or,
        OR,
        or,
        65,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        And,
        AND,
        and,
        20,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Xor,
        XOR,
        xor,
        81,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Not,
        NOT,
        not,
        63,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Compare,
        COMPARE,
        compare,
        25,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Eq,
        EQ,
        eq,
        37,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Neq,
        NEQ,
        neq,
        60,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Lt,
        LT,
        lt,
        55,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Gt,
        GT,
        gt,
        42,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Le,
        LE,
        le,
        50,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Ge,
        GE,
        ge,
        40,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Self_,
        SELF,
        self_,
        73,
        metadata_type: crate::michelson::metadata::FieldMetadata
    ),
    (
        SelfAddress,
        SELF_ADDRESS,
        self_address,
        119,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Contract,
        CONTRACT,
        contract,
        85,
        metadata_type: crate::michelson::metadata::FieldMetadata,
        (r#type: crate::michelson::types::Type)
    ),
    (TransferTokens, TRANSFER_TOKENS, transfer_tokens, 77),
    (
        SetDelegate,
        SET_DELEGATE,
        set_delegate,
        78,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        CreateContract,
        CREATE_CONTRACT,
        create_contract,
        29,
        metadata_type: crate::michelson::metadata::TwoVariableMetadata,
        (parameter_type: crate::michelson::types::Type),
        (storage_type: crate::michelson::types::Type),
        (code: crate::michelson::data::instructions::Sequence)
    ),
    (
        ImplicitAccount,
        IMPLICIT_ACCOUNT,
        implicit_account,
        30,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (VotingPower, VOTING_POWER, voting_power, 123),
    (
        Now,
        NOW,
        now,
        64,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Level,
        LEVEL,
        level,
        118,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Amount,
        AMOUNT,
        amount,
        19,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Balance,
        BALANCE,
        balance,
        21,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        CheckSignature,
        CHECK_SIGNATURE,
        check_signature,
        24,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Blake2B,
        BLAKE2B,
        blake2b,
        14,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Keccak,
        KECCAK,
        keccak,
        125,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Sha3,
        SHA3,
        sha3,
        126,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Sha256,
        SHA256,
        sha256,
        15,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Sha512,
        SHA512,
        sha512,
        16,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        HashKey,
        HASH_KEY,
        hash_key,
        43,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Source,
        SOURCE,
        source,
        71,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Sender,
        SENDER,
        sender,
        72,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        Address,
        ADDRESS,
        address,
        84,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        ChainId,
        CHAIN_ID,
        chain_id,
        117,
        metadata_type: crate::michelson::metadata::VariableMetadata
    ),
    (
        TotalVotingPower,
        TOTAL_VOTING_POWER,
        total_voting_power,
        124
    ),
    (PairingCheck, PAIRING_CHECK, pairing_check, 127),
    (
        SaplingEmptyState,
        SAPLING_EMPTY_STATE,
        sapling_empty_state,
        133,
        (memo_size: crate::michelson::data::Nat)
    ),
    (
        SaplingVerifyUpdate,
        SAPLING_VERIFY_UPDATE,
        sapling_verify_update,
        134
    ),
    (Ticket, TICKET, ticket, 136),
    (ReadTicket, READ_TICKET, read_ticket, 137),
    (SplitTicket, SPLIT_TICKET, split_ticket, 138),
    (JoinTickets, JOIN_TICKETS, join_ticket, 139),
    (OpenChest, OPEN_CHEST, open_chest, 143),
);

impl From<Primitive> for crate::michelson::Primitive {
    fn from(value: Primitive) -> Self {
        Self::Instruction(value)
    }
}
