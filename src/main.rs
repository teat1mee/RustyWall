use rand::prelude::IndexedRandom;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Используйте: rustywall -h для справки");
        return Ok(());
    }

    let flag = &args[1];

    let wallpaper_path: PathBuf = match flag.as_str() {
        "-h" | "--help" => {
            println!("Использование:");
            println!("  rustywall -url <ссылка>");
            println!("  rustywall -dir <путь>");
            println!("\nСовет по качеству:");
            println!("  Используйте прямые ссылки с фотостоков (Unsplash, Pexels),");
            println!("  чтобы избежать размытых изображений.");

            return Ok(());
        }
        "-url" => {
            let value = args.get(2).ok_or("Ошибка: Вы не ввели URL")?;
            println!("Режим: Загрузка по ссылке...");
            let name = value
                .split('/')
                .last()
                .unwrap_or("wall.jpg")
                .split('?')
                .next()
                .unwrap_or("wall.jpg");
            download_image(value, name)?
        }
        "-dir" => {
            let value = args.get(2).ok_or("Ошибка: Вы не ввели путь")?;

            println!("Режим: Случайные обои из папки...");
            get_random_wallpaper(Path::new(value))?
        }
        _ => {
            return Err(format!("Неизвестный флаг '{}'", flag).into());
        }
    };

    let uri = format!("file://{}", wallpaper_path.display());
    println!("Устанавливаю обои: {}", uri);
    set_gnome_wallpaper(&uri)?;

    println!("Ready!");
    Ok(())
}

fn download_image(url: &str, name_wallpaper: &str) -> Result<PathBuf, Box<dyn Error>> {
    let save_path = Path::new("/home/tea/Изображения/Wallpaper").join(name_wallpaper);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    print!("Запрос отправлен, жду ответа...");
    io::stdout().flush()?; // Добавил flush, чтобы увидеть текст сразу

    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Ошибка сервера: {}", response.status()).into());
    }

    let mut file = fs::File::create(&save_path)?;
    std::io::copy(&mut response, &mut file)?;

    let metadata = fs::metadata(&save_path)?;
    let size_kb = metadata.len() / 1024;
    println!("Загрузка завершена. Размер файла: {} KB", size_kb);

    if size_kb < 200 {
        println!(
            "Предупреждение: Файл очень маленький. Скорее всего, это превью,качество будет плохим!"
        )
    }

    Ok(save_path)
}

fn get_random_wallpaper(dir_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let wallpapers: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.is_file())
        .collect();

    if wallpapers.is_empty() {
        return Err("Обои не найдены в указанной папке".into());
    }

    let mut rng = rand::rng();
    let selected = wallpapers
        .choose(&mut rng)
        .ok_or("Не удалось выбрать файл")?;

    Ok(selected.clone())
}

fn set_gnome_wallpaper(uri: &str) -> Result<(), Box<dyn Error>> {
    let keys = ["picture-uri", "picture-uri-dark"];
    for key in keys {
        Command::new("gsettings")
            .args(["set", "org.gnome.desktop.background", key, uri])
            .status()?;
    }
    Ok(())
}
