pub mod caboose;
pub mod checkpoint;
pub mod savepoint;

fn path<S: AsRef<str>>(chain_id: S) -> String {
    format!("{}/levels", super::path(chain_id))
}
