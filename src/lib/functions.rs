use rand::prelude::*;
pub fn gen_random() -> i64 {
    return rand::thread_rng().gen_range(1000000000, 9999999999);
}
pub fn checknum(num: String) -> String {
    match num.len() {
        1 => {
            return format!("0000{}", num);
        }
        2 => {
            return format!("000{}", num);
        }
        3 => {
            return format!("00{}", num);
        }
        4 => {
            return format!("0{}", num);
        }
        5 => {
            return format!("{}", num);
        }
        _ => {
            return "00000".to_owned();
        }
    }
}