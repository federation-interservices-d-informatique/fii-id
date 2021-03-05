pub mod utils;
pub fn gen_id(server: &str, num: &str) -> String
{
    utils::idgen::generate_id(server, num)
}