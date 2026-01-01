use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
mod key_img;
use image::{DynamicImage, RgbImage, imageops};
use key_img::KeyImg;

#[derive(Debug)]
pub struct DeckDevice {
    pub sys_fs: PathBuf,
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
    pub path: PathBuf,
    pub key_state_offset: usize,
    pub keys: usize,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub type_name: String,
    /*
    id: String,
    columns: u8,
    rows: u8,
    keys: u8,
    dpi: u16,
    padding : u8,
    */
}

const ELGATO_VID: &str = "0fd9";
const USB_PID_STREAMDECK_MINI: &str = "0063";
const USB_PID_STREAMDECK_MINI_MK2: &str = "0090";
const USB_PID_STREAMDECK_MINI_MK2_MODULE: &str = "00b8";
const USB_PID_STREAMDECK_MK2: &str = "0080";
const USB_PID_STREAMDECK_MK2_MODULE: &str = "00b9";
const USB_PID_STREAMDECK_MK2_SCISSOR: &str = "00a5";
const USB_PID_STREAMDECK_MK2_V2: &str = "00B9";
const USB_PID_STREAMDECK_NEO: &str = "009a";
const USB_PID_STREAMDECK_ORIGINAL: &str = "0060";
const USB_PID_STREAMDECK_ORIGINAL_V2: &str = "006d";
const USB_PID_STREAMDECK_PEDAL: &str = "0086";
const USB_PID_STREAMDECK_PLUS: &str = "0084";
const USB_PID_STREAMDECK_XL: &str = "006c";
const USB_PID_STREAMDECK_XL_V2: &str = "008f";
const USB_PID_STREAMDECK_STUDIO: &str = "00aa";
const USB_PID_STREAMDECK_XL_V2_MODULE: &str = "00ba";

impl DeckDevice {
    pub fn new() -> Self {
        DeckDevice::find_device().expect("No Stream Deck device found")
    }

    fn find_device() -> Option<Self> {
        let dir = Path::new("/sys/bus/usb/devices/");
        if dir.exists() {
        } else {
            panic!("USB devices directory does not exist: {:?}", dir);
        }
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let vid_path = Path::new("/sys/bus/usb/devices/")
                .join(entry.file_name())
                .join("idVendor");
            if vid_path.exists() {
                if let Some(vid) = DeckDevice::read_line_from_file(&vid_path) {
                    if vid == ELGATO_VID {
                        println!("Elgato device found at {:?}", entry.path());
                        let id_path = Path::new("/sys/bus/usb/devices/")
                            .join(entry.file_name())
                            .join("idProduct");
                        if id_path.exists() {
                            if let Some(pid) = DeckDevice::read_line_from_file(&id_path) {
                                return DeckDevice::match_device(&pid, &entry.path());
                            }
                        }
                    }
                }
            } else {
                continue;
            }

            // let path = entry.path();
        }

        None
    }

    fn match_device(_pid: &str, sys_path: &Path) -> Option<Self> {
        // Match known Elgato Stream Deck VID/PID combinations
        let full_path = sys_path.canonicalize().unwrap();
        let dev_path = match DeckDevice::find_dev_path(&full_path) {
            Some(p) => {
                println!("Device path found: {:?}", p);
                Path::new("/dev").join(p.file_name().unwrap())
            }
            None => {
                panic!("Could not find device path for Stream Deck Neo.");
            }
        };

        match _pid {
            USB_PID_STREAMDECK_MINI => None,
            USB_PID_STREAMDECK_MINI_MK2 => None,
            USB_PID_STREAMDECK_MINI_MK2_MODULE => None,
            USB_PID_STREAMDECK_MK2 => None,
            USB_PID_STREAMDECK_MK2_MODULE => None,
            USB_PID_STREAMDECK_MK2_SCISSOR => None,
            USB_PID_STREAMDECK_MK2_V2 => None,
            USB_PID_STREAMDECK_NEO => {
                println!("Stream Deck Neo device detected.");

                Some(DeckDevice {
                    sys_fs: full_path.clone(),
                    manufacturer: DeckDevice::read_line_from_file(&sys_path.join("manufacturer"))
                        .unwrap_or_default(),
                    product: DeckDevice::read_line_from_file(&sys_path.join("product"))
                        .unwrap_or_default(),
                    serial: DeckDevice::read_line_from_file(&sys_path.join("serial"))
                        .unwrap_or_default(),
                    path: dev_path,
                    key_state_offset: 4,
                    keys: 10,
                    pixel_width: 96,
                    pixel_height: 96,
                    type_name: "jpg".to_string(),
                })
            }
            USB_PID_STREAMDECK_ORIGINAL => None,
            USB_PID_STREAMDECK_ORIGINAL_V2 => None,
            USB_PID_STREAMDECK_PEDAL => None,
            USB_PID_STREAMDECK_PLUS => None,
            USB_PID_STREAMDECK_XL => None,
            USB_PID_STREAMDECK_XL_V2 => None,
            USB_PID_STREAMDECK_STUDIO => None,
            USB_PID_STREAMDECK_XL_V2_MODULE => None,
            _ => {
                println!("Unknown Stream Deck device with PID: {}", _pid);
                None
            }
        }
    }

    fn find_dev_path(path: &Path) -> Option<PathBuf> {
        let entries = fs::read_dir(path).ok()?;

        for entry in entries.flatten() {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if file_name_str.starts_with("hidraw")
                && entry_path.to_str().unwrap().contains("/hidraw/")
            {
                return entry_path.canonicalize().ok();
            }

            if entry_path.is_dir() {
                if let Some(found) = DeckDevice::find_dev_path(&entry_path) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn write_to_device(&self, _data: &[u8]) {
        let mut file = File::create(&self.path).expect("Failed to open device file for writing");
        file.write_all(_data)
            .expect("Failed to write data to device");
    }

    fn read_line_from_file(path: &Path) -> Option<String> {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => Some(content.trim().to_string()),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn clear(&self) {
        let black_image = KeyImg::new_black_image(self.pixel_width, self.pixel_height);
        for key_index in 0..self.keys {
            self.set_image(key_index as u8, black_image.clone());
        }
    }

    pub fn set_image(&self, key_index: u8, img: RgbImage) {
        // 1 resize if necessary
        let img = self.resize_image(&img);
        // 2 convert
        // 3 create packet
        // 4 send to device
        // self.write_to_device(&packet);
        println!("Setting image for key {}", key_index);
    }

    // fn 

    fn resize_image(&self, img: &RgbImage) -> RgbImage {
        if img.width() > self.pixel_width || img.height() > self.pixel_height {
            DynamicImage::ImageRgb8(img.clone())
                .resize_to_fill(
                    self.pixel_width,
                    self.pixel_height,
                    imageops::FilterType::Lanczos3,
                )
                .to_rgb8()
        } else {
            img.clone()
        }
    }
}
