mod utils;
fn main() {
    let servers = vec!("ADP", "MIM", "LPT", "CLI", "LXS");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3  {
        println!("Pas assez d'arguments!");
        std::process::exit(1); 
    }
    if ! servers.iter().any(|v| v.to_owned() == &args[1].to_uppercase()) {
        println!("Nom de serveur invalide!");
        std::process::exit(1);
    }
    let id = utils::idgen::generate_id(&args[1], &args[2]);
    println!("Votre ID FII:");
    println!("{}", id);
    println!("Cet ID ne doit pas être partagé.");
}
