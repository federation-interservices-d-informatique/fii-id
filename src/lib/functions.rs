
use rand::prelude::*;
pub fn gen_random() -> i64 {
    return rand::thread_rng().gen_range(1000000000, 9999999999);
}