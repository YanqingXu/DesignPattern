extern crate image;
extern crate rodio;

use image::DynamicImage;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// 子系统1：图像加载
struct ImageLoader;

impl ImageLoader {
    fn load_image(path: &str) -> Result<DynamicImage, image::ImageError> {
        let image = image::open(path)?;
        Ok(image)
    }
}

// 子系统2：声音加载
struct SoundLoader;

impl SoundLoader {
    fn load_sound(path: &str) -> Result<Decoder<BufReader<File>>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        Ok(decoder)
    }
}

// 资源管理外观
struct ResourceManager;

impl ResourceManager {
    fn load_image(path: &str) -> Result<DynamicImage, image::ImageError> {
        ImageLoader::load_image(path)
    }

    fn load_sound(path: &str) -> Result<Decoder<BufReader<File>>, Box<dyn Error>> {
        SoundLoader::load_sound(path)
    }
}

fn main() {
    // 初始化资源管理器
    let image_path = "liuyifei.png";
    let sound_path = "qingtian.mp3";

    // 加载图像资源
    match ResourceManager::load_image(image_path) {
        Ok(image) => println!("Image loaded successfully: {:?}", image),
        Err(err) => eprintln!("Failed to load image: {:?}", err),
    }

    // 加载声音资源
    match ResourceManager::load_sound(sound_path) {
        Ok(sound) => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(sound);
            sink.play();
            std::thread::sleep(std::time::Duration::from_secs(5)); // 播放声音5秒钟
            println!("Sound loaded and played successfully.");
        }
        Err(err) => eprintln!("Failed to load sound: {:?}", err),
    }
}
