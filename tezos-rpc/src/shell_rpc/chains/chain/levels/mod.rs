pub mod checkpoint;

fn path(chain_alias: String) -> String {
    format!("{}{}", super::path(chain_alias),"/levels")
}
