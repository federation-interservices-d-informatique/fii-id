use rand::prelude::*;
pub fn gen_random() -> i64 {
    return rand::thread_rng().gen_range(1000000000..9999999999);
}
pub fn checknum(num: String) -> String {
    let mut n: String;
    n = num.clone();
    while n.len() < 5 {
        n = format!("0{}", n.clone());
    }
    return n.to_string();
}
pub fn usage() {
    let argsvec: Vec<String> = std::env::args().collect();
    println!("Usage: {} server number", argsvec[0]);
}