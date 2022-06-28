pub mod caboose;
pub mod checkpoint;
pub mod savepoint;

fn path(chain_id: String) -> String {
    format!("{}{}", super::path(chain_id), "/levels")
}
