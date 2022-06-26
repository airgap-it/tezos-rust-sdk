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
            (
                michelson::data::unit(),
                micheline::primitive_application("Unit").into(),
            ),
            (
                michelson::data::r#true(),
                micheline::primitive_application("True").into(),
            ),
            (
                michelson::data::r#false(),
                micheline::primitive_application("False").into(),
            ),
            (
                michelson::data::pair(vec![true.into(), false.into()]),
                micheline::primitive_application("Pair")
                    .with_args(vec![
                        micheline::primitive_application("True").into(),
                        micheline::primitive_application("False").into(),
                    ])
                    .into(),
            ),
            (
                michelson::data::left(().into()),
                micheline::primitive_application("Left")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::right(().into()),
                micheline::primitive_application("Right")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::some(().into()),
                micheline::primitive_application("Some")
                    .with_args(vec![micheline::primitive_application("Unit").into()])
                    .into(),
            ),
            (
                michelson::data::none(),
                micheline::primitive_application("None").into(),
            ),
            (
                vec![().into()].into(),
                vec![micheline::primitive_application("Unit").into()].into(),
            ),
            (
                michelson::data::sequence(vec![michelson::data::instructions::unit(), ().into()]),
                vec![
                    micheline::primitive_application("UNIT").into(),
                    micheline::primitive_application("Unit").into(),
                ]
                .into(),
            ),
            (
                michelson::types::unit(),
                micheline::primitive_application("unit").into(),
            ),
        ]
    }
}
