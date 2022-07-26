use tezos_michelson::michelson::metadata::TypeFieldMetadata;

pub trait AnyAnnotationValue {
    fn any_annotation_value(&self) -> Option<&str>;
}

impl AnyAnnotationValue for TypeFieldMetadata {
    fn any_annotation_value(&self) -> Option<&str> {
        self.field_name().as_ref().map_or(
            self.type_name()
                .as_ref()
                .map(|annotation| annotation.value_without_prefix()),
            |annotation| Some(annotation.value_without_prefix()),
        )
    }
}
