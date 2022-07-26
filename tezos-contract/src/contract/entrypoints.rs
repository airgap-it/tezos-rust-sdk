use tezos_michelson::michelson::types::{Or, Parameter, Type};
use tezos_operation::operations::Entrypoint;

use crate::Result;

#[derive(Debug, Clone)]
pub struct MappedEntrypoints {
    parameters_type: Type,
    entrypoint_paths: Vec<(Entrypoint, Vec<EntrypointPathComponent>)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntrypointPathComponent {
    Left,
    Right,
}

impl MappedEntrypoints {
    pub fn new(parameter: Parameter) -> Result<Self> {
        let parameters_type = *parameter.r#type;
        let entrypoint_paths = if let Type::Or(value) = &parameters_type {
            entrypoint_paths(value, None)
        } else {
            vec![]
        };
        return Ok(MappedEntrypoints {
            parameters_type,
            entrypoint_paths,
        });
    }

    pub fn get(&self, entrypoint: &Entrypoint) -> Option<&Type> {
        if &Entrypoint::default() == entrypoint {
            return Some(&self.parameters_type);
        }

        if let Some((_, path)) = self
            .entrypoint_paths
            .iter()
            .find(|(entry, _)| entry == entrypoint)
        {
            return get_at_path(&self.parameters_type, path);
        }

        None
    }

    pub fn get_entrypoint_at_path(&self, path: &[EntrypointPathComponent]) -> Option<Entrypoint> {
        let value = get_at_path(&self.parameters_type, path)?;
        if let Some(annotation) = value.metadata().field_name() {
            let entrypoint = Entrypoint::from_str(annotation.value());

            return Some(entrypoint);
        }

        None
    }
}

fn entrypoint_paths(
    value: &Or,
    current_path: Option<Vec<EntrypointPathComponent>>,
) -> Vec<(Entrypoint, Vec<EntrypointPathComponent>)> {
    let mut result: Vec<(Entrypoint, Vec<EntrypointPathComponent>)> = Vec::new();

    fn handle(
        value: &Type,
        path_component: EntrypointPathComponent,
        current_path: Option<&Vec<EntrypointPathComponent>>,
        entries: &mut Vec<(Entrypoint, Vec<EntrypointPathComponent>)>,
    ) {
        if let Type::Or(lhs) = value {
            let mut path = current_path.map(|value| value.clone()).unwrap_or(vec![]);
            path.push(path_component);
            let entrypoints = entrypoint_paths(lhs, Some(path));
            entries.extend(entrypoints.into_iter());
        } else if let Some(name) = value.metadata().field_name() {
            let mut path = current_path.map(|value| value.clone()).unwrap_or(vec![]);
            path.push(path_component);
            entries.push((Entrypoint::from_str(name.value_without_prefix()), path));
        }
    }

    handle(
        &value.lhs,
        EntrypointPathComponent::Left,
        current_path.as_ref(),
        &mut result,
    );
    handle(
        &value.rhs,
        EntrypointPathComponent::Right,
        current_path.as_ref(),
        &mut result,
    );

    result
}

fn get_at_path<'a>(value: &'a Type, path: &[EntrypointPathComponent]) -> Option<&'a Type> {
    if let Some(path_component) = path.first() {
        if let Type::Or(value) = value {
            let next_value = match path_component {
                EntrypointPathComponent::Left => &*value.lhs,
                EntrypointPathComponent::Right => &*value.rhs,
            };
            if path.len() > 1 {
                return get_at_path(next_value, &path[1..]);
            } else {
                return Some(next_value);
            }
        }
    }
    Some(value)
}
