#![allow(dead_code)]
use serde::Deserialize;
use std::fs;
mod device;
use device::DeckDevice;

#[derive(Debug, Deserialize)]
pub struct Deck {
    #[serde(skip_deserializing)]
    config: String,
    #[serde(skip_deserializing)]
    pub deck: Option<DeckDevice>,
    default: String,
    pages: Vec<String>,
    brightness: u8,
}

impl Deck {
    pub fn new(config: &str) -> Self {
        let data = fs::read_to_string(config).unwrap_or_else(|err| {
            panic!("Failed to read config file {}: {}", config, err);
        });
        let mut deck: Deck = toml::from_str(&data).unwrap_or_else(|err| {
            panic!("Failed to parse config file {}: {}", config, err);
        });
        deck.config = config.to_string();
        deck
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

    pub fn listen(&self) {
        if let Some(device) = &self.deck {
            device.listen_events();
        } else {
            println!("No device found to listen for events.");
        }
    }
}
