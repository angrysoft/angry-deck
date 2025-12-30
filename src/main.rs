mod deck;
use deck::Deck;
use std::env::{args, home_dir};
use std::path::Path;

fn main() {
    let user_dir = home_dir().expect("Could not find home directory");
    let mut config = Path::new(&user_dir)
        .join(".config")
        .join("angry-deck")
        .join("deck.toml");
    if args().len() > 1 {
        config = Path::new(&args().nth(1).unwrap()).to_path_buf();
        if !config.exists() {
            println!("Configuration file: {:?} not exist.", config);
            std::process::exit(1);
        }
    }
    let mut deck = Deck::new(config.to_str().unwrap());
    deck.find_device();
    deck.listen();
    deck.show_config();
}
