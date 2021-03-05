use crate::{checknum,gen_random};
pub fn generate_id(server: &str, num: &str) -> String {
    let rnd = gen_random();
    let num = checknum(format!("{}", num));
    format!("FII-{}-{}-{}-FII", server.to_uppercase(), num, rnd)
}