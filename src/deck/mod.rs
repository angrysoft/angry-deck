#![allow(dead_code)]
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Read;
// use std::collections::HashMap;
mod device;
use device::DeckDevice;

#[derive(Debug)]
pub struct Deck {
    pub deck: Option<DeckDevice>,
    current_page: String,
    config: Config,
    //pages: Vec<Page>,
    // handlers: HashMap<String, ActionHandler>,
    //config_path: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    default: String,
    pages: Vec<String>,
    brightness: u8,
}

impl Deck {
    pub fn new(config: &str) -> Self {
        let data = fs::read_to_string(config).unwrap_or_else(|err| {
            panic!("Failed to read config file {}: {}", config, err);
        });
        let config: Config = toml::from_str(&data).unwrap_or_else(|err| {
            panic!("Failed to parse config file {}: {}", config, err);
        });
        let default_page = config.default.clone();

        Deck {
            deck: None,
            current_page: default_page,
            config,
        }
    }

    pub fn show_config(&self) {
        println!("Deck configuration: {:#?}", self);
    }

    pub fn find_device(&mut self) {
        println!("Finding device...");
        let device = DeckDevice::new();

        self.deck = Some(device);
        // Implementation for finding the device would go here
    }

    pub fn load_pages(&mut self) {
        let device = match &self.deck {
            Some(dev) => dev,
            None => {
                println!("No device found.");
                return;
            }
        };
        device.clear();
        for page_name in self.config.pages.iter() {
            println!("Loaded page: {}", page_name);

        }
    }

    pub fn listen(&self) {
        if let Some(device) = &self.deck {
            let mut file = File::open(&device.path).expect("Failed to open device file");
            let mut buffer = [0u8; 64];
            let mut button_states = vec![0u8; device.keys];

            loop {
                match file.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            for i in 0..device.keys {
                                let state = buffer[device.key_state_offset + i];
                                if state != button_states[i] {
                                    button_states[i] = state;
                                    let msg = format!(
                                        "{}.{}.{}",
                                        self.current_page,
                                        i,
                                        if state != 0 { "pressed" } else { "released" }
                                    );
                                    println!("{}", msg);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from device: {}", e);
                        break;
                    }
                }
            }
        }
    }
}
