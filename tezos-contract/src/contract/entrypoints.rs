use std::collections::HashMap;

use tezos_michelson::michelson::types::{Or, Parameter, Type};
use tezos_operation::operations::Entrypoint;

use crate::Result;

#[derive(Debug, Clone)]
pub struct MappedEntrypoints {
    parameters_type: Type,
    entrypoint_paths: HashMap<Entrypoint, Vec<EntrypointPath>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntrypointPath {
    Left,
    Right,
}

impl MappedEntrypoints {
    pub fn new(parameter: Parameter) -> Result<Self> {
        let parameters_type = *parameter.r#type;
        let entrypoint_paths = if let Type::Or(value) = &parameters_type {
            entrypoint_paths(value, None)
        } else {
            HashMap::new()
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

        if let Some(path) = self.entrypoint_paths.get(entrypoint) {
            return get_at_path(&self.parameters_type, path);
        }

        None
    }

    pub fn get_entrypoint_at_path(&self, path: &[EntrypointPath]) -> Option<Entrypoint> {
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
    current_path: Option<Vec<EntrypointPath>>,
) -> HashMap<Entrypoint, Vec<EntrypointPath>> {
    let mut result: HashMap<Entrypoint, Vec<EntrypointPath>> = HashMap::new();

    fn handle(
        value: &Type,
        path_component: EntrypointPath,
        current_path: Option<&Vec<EntrypointPath>>,
        map: &mut HashMap<Entrypoint, Vec<EntrypointPath>>,
    ) {
        if let Type::Or(lhs) = value {
            let mut path = current_path.map(|value| value.clone()).unwrap_or(vec![]);
            path.push(path_component);
            let entrypoints = entrypoint_paths(lhs, Some(path));
            map.extend(entrypoints.into_iter());
        } else if let Some(name) = value.metadata().field_name() {
            let mut path = current_path.map(|value| value.clone()).unwrap_or(vec![]);
            path.push(path_component);
            map.insert(Entrypoint::from_str(name.value()), path);
        }
    }

    handle(
        &value.lhs,
        EntrypointPath::Left,
        current_path.as_ref(),
        &mut result,
    );
    handle(
        &value.rhs,
        EntrypointPath::Right,
        current_path.as_ref(),
        &mut result,
    );

    result
}

fn get_at_path<'a>(value: &'a Type, path: &[EntrypointPath]) -> Option<&'a Type> {
    if let Some(path_component) = path.first() {
        if let Type::Or(value) = value {
            let next_value = match path_component {
                EntrypointPath::Left => &*value.lhs,
                EntrypointPath::Right => &*value.rhs,
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
