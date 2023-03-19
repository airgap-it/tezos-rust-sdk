use super::annotations::{Annotation, Kind};
use crate::{micheline::primitive_application::PrimitiveApplication, Error, Result};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldMetadata {
    field_name: Option<Annotation>,
}

impl FieldMetadata {
    pub fn field_name(&self) -> &Option<Annotation> {
        &self.field_name
    }

    pub fn annotations(&self) -> Vec<&Annotation> {
        vec![self.field_name()]
            .into_iter()
            .flat_map(|annot| annot)
            .collect::<Vec<&Annotation>>()
    }

    pub fn new(field_name: Option<Annotation>) -> Result<Self> {
        if let Some(field_name) = &field_name {
            if !Self::is_valid_field_name(&field_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        Ok(Self { field_name })
    }

    pub fn with_field_name(mut self, name: String) -> Self {
        self.field_name = Some(Annotation::new_with_kind(Kind::Field, name));

        self
    }

    fn is_valid_field_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Field
    }
}

impl Default for FieldMetadata {
    fn default() -> Self {
        Self { field_name: None }
    }
}

impl TryFrom<&PrimitiveApplication> for FieldMetadata {
    type Error = Error;

    fn try_from(value: &PrimitiveApplication) -> Result<Self> {
        Self::new(value.first_annot(Kind::Field)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFieldMetadata {
    type_name: Option<Annotation>,
    field_name: Option<Annotation>,
}

impl TypeFieldMetadata {
    pub fn type_name(&self) -> &Option<Annotation> {
        &self.type_name
    }

    pub fn field_name(&self) -> &Option<Annotation> {
        &self.field_name
    }

    pub fn annotations(&self) -> Vec<&Annotation> {
        vec![self.type_name(), self.field_name()]
            .into_iter()
            .flat_map(|annot| annot)
            .collect::<Vec<&Annotation>>()
    }

    pub fn new(type_name: Option<Annotation>, field_name: Option<Annotation>) -> Result<Self> {
        if let Some(type_name) = &type_name {
            if !Self::is_valid_type_name(&type_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        if let Some(field_name) = &field_name {
            if !Self::is_valid_field_name(&field_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        Ok(Self {
            type_name,
            field_name,
        })
    }

    pub fn with_type_name(mut self, name: String) -> Self {
        self.type_name = Some(Annotation::new_with_kind(Kind::Type, name));

        self
    }

    pub fn with_field_name(mut self, name: String) -> Self {
        self.field_name = Some(Annotation::new_with_kind(Kind::Field, name));

        self
    }

    fn is_valid_type_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Type
    }

    fn is_valid_field_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Field
    }
}

impl Default for TypeFieldMetadata {
    fn default() -> Self {
        Self {
            type_name: None,
            field_name: None,
        }
    }
}

impl TryFrom<&PrimitiveApplication> for TypeFieldMetadata {
    type Error = Error;

    fn try_from(value: &PrimitiveApplication) -> Result<Self> {
        Self::new(
            value.first_annot(Kind::Type)?,
            value.first_annot(Kind::Field)?,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableMetadata {
    variable_name: Option<Annotation>,
}

impl VariableMetadata {
    pub fn variable_name(&self) -> &Option<Annotation> {
        &self.variable_name
    }

    pub fn annotations(&self) -> Vec<&Annotation> {
        vec![self.variable_name()]
            .into_iter()
            .flat_map(|annot| annot)
            .collect::<Vec<&Annotation>>()
    }

    pub fn new(variable_name: Option<Annotation>) -> Result<Self> {
        if let Some(variable_name) = &variable_name {
            if variable_name.kind() != Kind::Variable {
                return Err(Error::InvalidAnnotation);
            }
        }
        Ok(Self { variable_name })
    }
}

impl TryFrom<&PrimitiveApplication> for VariableMetadata {
    type Error = Error;

    fn try_from(value: &PrimitiveApplication) -> Result<Self> {
        Self::new(value.first_annot(Kind::Variable)?)
    }
}

impl Default for VariableMetadata {
    fn default() -> Self {
        Self {
            variable_name: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVariableMetadata {
    type_name: Option<Annotation>,
    variable_name: Option<Annotation>,
}

impl TypeVariableMetadata {
    pub fn type_name(&self) -> &Option<Annotation> {
        &self.type_name
    }

    pub fn variable_name(&self) -> &Option<Annotation> {
        &self.variable_name
    }

    pub fn annotations(&self) -> Vec<&Annotation> {
        vec![self.type_name(), self.variable_name()]
            .into_iter()
            .flat_map(|annot| annot)
            .collect::<Vec<&Annotation>>()
    }

    pub fn new(type_name: Option<Annotation>, variable_name: Option<Annotation>) -> Result<Self> {
        if let Some(type_name) = &type_name {
            if !Self::is_valid_type_name(&type_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        if let Some(variable_name) = &variable_name {
            if !Self::is_valid_variable_name(&variable_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        Ok(Self {
            type_name,
            variable_name,
        })
    }

    fn is_valid_type_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Type
    }

    fn is_valid_variable_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Variable
    }
}

impl Default for TypeVariableMetadata {
    fn default() -> Self {
        Self {
            type_name: None,
            variable_name: None,
        }
    }
}

impl TryFrom<&PrimitiveApplication> for TypeVariableMetadata {
    type Error = Error;

    fn try_from(value: &PrimitiveApplication) -> Result<Self> {
        Self::new(
            value.first_annot(Kind::Type)?,
            value.first_annot(Kind::Variable)?,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TwoVariableMetadata {
    first_variable_name: Option<Annotation>,
    second_variable_name: Option<Annotation>,
}

impl TwoVariableMetadata {
    pub fn first_variable_name(&self) -> &Option<Annotation> {
        &self.first_variable_name
    }

    pub fn second_variable_name(&self) -> &Option<Annotation> {
        &self.second_variable_name
    }

    pub fn annotations(&self) -> Vec<&Annotation> {
        vec![self.first_variable_name(), self.second_variable_name()]
            .into_iter()
            .flat_map(|annot| annot)
            .collect::<Vec<&Annotation>>()
    }

    pub fn new(
        first_variable_name: Option<Annotation>,
        second_variable_name: Option<Annotation>,
    ) -> Result<Self> {
        if let Some(first_variable_name) = &first_variable_name {
            if !Self::is_valid_variable_name(&first_variable_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        if let Some(second_variable_name) = &second_variable_name {
            if !Self::is_valid_variable_name(&second_variable_name) {
                return Err(Error::InvalidAnnotation);
            }
        }
        Ok(Self {
            first_variable_name,
            second_variable_name,
        })
    }

    fn is_valid_variable_name(annotation: &Annotation) -> bool {
        annotation.kind() == Kind::Variable
    }
}

impl Default for TwoVariableMetadata {
    fn default() -> Self {
        Self {
            first_variable_name: None,
            second_variable_name: None,
        }
    }
}

impl TryFrom<&PrimitiveApplication> for TwoVariableMetadata {
    type Error = Error;

    fn try_from(value: &PrimitiveApplication) -> Result<Self> {
        Self::new(
            value.first_annot(Kind::Variable)?,
            value.second_annot(Kind::Variable)?,
        )
    }
}

impl PrimitiveApplication {
    fn first_annot(&self, kind: Kind) -> Result<Option<Annotation>> {
        self.annots()
            .as_ref()
            .and_then(|annots| {
                annots.iter().find_map(|annot| {
                    if annot.starts_with(kind.prefix()) {
                        return Some(annot.clone().try_into());
                    }
                    None
                })
            })
            .map_or(Ok(None), |v| v.map(Some))
    }

    fn second_annot(&self, kind: Kind) -> Result<Option<Annotation>> {
        self.annots()
            .as_ref()
            .and_then(|annots| {
                let filtered = annots
                    .iter()
                    .filter(|annot| annot.starts_with(kind.prefix()))
                    .collect::<Vec<_>>();
                if filtered.len() >= 2 {
                    let value = (*filtered.iter().nth(1).unwrap()).clone();
                    return Some(value.try_into());
                }
                None
            })
            .map_or(Ok(None), |v| v.map(Some))
    }
}
