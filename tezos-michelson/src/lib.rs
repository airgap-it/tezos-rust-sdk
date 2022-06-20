mod common;
pub mod error;
mod internal;
pub mod micheline;
pub mod michelson;

pub use error::{Error, Result};

#[cfg(test)]
mod test {
    use crate::{
        micheline::{self, Micheline},
        michelson::{self, Michelson},
        Result,
    };

    #[test]
    fn test_michelson_to_micheline() -> Result<()> {
        for (michelson, expected) in michelson_micheline_values() {
            let micheline: Micheline = michelson.into();
            assert_eq!(expected, micheline);
        }
        Ok(())
    }

    #[test]
    fn test_micheline_to_michelson() -> Result<()> {
        for (expected, micheline) in michelson_micheline_values() {
            let michelson: Michelson = micheline.try_into()?;
            assert_eq!(expected, michelson);
        }
        Ok(())
    }

    fn michelson_micheline_values() -> Vec<(Michelson, Micheline)> {
        vec![
            (michelson::data::int(1), micheline::int(1)),
            (
                michelson::data::try_string("string").unwrap(),
                micheline::try_string("string").unwrap(),
            ),
            (
                michelson::data::try_bytes("0x00").unwrap(),
                micheline::try_bytes("0x00").unwrap(),
            ),
            (michelson::data::unit(), micheline::prim("Unit")),
            (michelson::data::r#true(), micheline::prim("True")),
            (michelson::data::r#false(), micheline::prim("False")),
            (
                michelson::data::pair(true.into(), false.into()),
                micheline::prim_with_args(
                    "Pair",
                    vec![micheline::prim("True"), micheline::prim("False")],
                ),
            ),
            (
                michelson::data::left(().into()),
                micheline::prim_with_args("Left", vec![micheline::prim("Unit")]),
            ),
            (
                michelson::data::right(().into()),
                micheline::prim_with_args("Right", vec![micheline::prim("Unit")]),
            ),
            (
                michelson::data::some(().into()),
                micheline::prim_with_args("Some", vec![micheline::prim("Unit")]),
            ),
            (michelson::data::none(), micheline::prim("None")),
            (vec![().into()].into(), vec![micheline::prim("Unit")].into()),
            (
                vec![
                    michelson::data::instructions::unit().try_into().unwrap(),
                    ().into(),
                ]
                .into(),
                vec![micheline::prim("UNIT"), micheline::prim("Unit")].into(),
            ),
            (
                michelson::types::comparables::unit(),
                micheline::prim("unit"),
            ),
        ]
    }
}
