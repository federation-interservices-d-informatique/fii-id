mod lib;
use lib::functions::gen_random;
fn main() {
    let servers = vec!("MIM", "TH", "LPT", "TAB");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2  {
        println!("Pas assez d'arguments!"); 
        std::process::exit(1); 
    }
    if ! servers.iter().any(|v| v.to_owned() == &args[1].to_uppercase()) {
        println!("Nom de serveur invalide!");
        std::process::exit(1);
    }
    let rnd = gen_random();

    println!("Votre ID FII:");
    println!("FII - {} - {} - FII", args[1].to_uppercase(), rnd);
}
