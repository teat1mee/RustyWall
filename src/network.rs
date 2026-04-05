use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn download_image(url: &str, name_wallpaper: &str) -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = std::env::var("HOME")?;
    let save_dir = Path::new(&home_dir).join("Изображения/Wallpaper");

    if save_dir.exists() {
        println!("Папка не найдена. Создаю: {}", save_dir.display());
        fs::create_dir_all(&save_dir)?;
    }

    let save_path = save_dir.join(name_wallpaper);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    print!("Запрос отправлен, жду ответа...");
    io::stdout().flush()?; // Добавил flush, чтобы увидеть текст сразу

    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Ошибка сервера: {}", response.status()).into());
    }

    let content_length = response.content_length();

    let mut file = fs::File::create(&save_path)?;

    let bytes_written = std::io::copy(&mut response, &mut file)?;

    if let Some(expected) = content_length {
        if bytes_written < expected {
            fs::remove_file(&save_path)?;
            return Err(format!(
                "Ошибка: Файл скачан не полностью ({} из {} байт)",
                bytes_written, expected
            )
            .into());
        }
    }

    println!(
        "\nЗагрузка завершена успешно. Записано: {} байт",
        bytes_written
    );
    Ok(save_path)
}
