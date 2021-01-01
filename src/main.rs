mod lib;
use lib::functions::{
    checknum,
    gen_random
};
fn main() {
    let servers = vec!("ADP", "MIM", "LPT", "CLI");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3  {
        println!("Pas assez d'arguments!");
        lib::functions::usage();
        std::process::exit(1); 
    }
    if ! servers.iter().any(|v| v.to_owned() == &args[1].to_uppercase()) {
        println!("Nom de serveur invalide!");
        lib::functions::usage();
        std::process::exit(1);
    }
    let rnd = gen_random();
    let num = checknum(format!("{}", args[2]));
    println!("Votre ID FII:");
    println!("FII-{}-{}-{}-FII", args[1].to_uppercase(), num, rnd);
}