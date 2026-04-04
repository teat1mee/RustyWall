// Путь к папке
// Взять все файлы
// Выбрать рандомный файл
// Установить его как фон
// Добавить интернет и скачку с интернета.

use rand::prelude::IndexedRandom;
use std::error::Error;
use std::fs::{self};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Получаем ввод от пользователя.
    print!("Введите путь к папке или URL картинки: ");
    io::stdout().flush()?; // Напечатывание текста до того как мы начнем ввод.

    let mut input_url = String::new();
    io::stdin().read_line(&mut input_url)?;

    let input_url = input_url.trim();

    //Скачивание или поиск локально
    let wallpaper_path: PathBuf = if input_url.starts_with("http") {
        print!("Введите название картинки: ");
        io::stdout().flush()?;

        let mut input_name_wallpaper = String::new();
        io::stdin().read_line(&mut input_name_wallpaper)?;

        let input_name_wallpaper = input_name_wallpaper.trim();

        println!("Обнаружена ссылка. Скачиваю...");
        download_image(input_url, input_name_wallpaper)?
    } else {
        print!("Обнаружен путь. Выбираю случайный файл...");
        get_random_wallpaper(Path::new(input_url))?
    };

    // Формируем URI и устанавливаем обои
    let uri = format!("file://{}", wallpaper_path.display());
    println!("Устанавливаю обои {}", uri);
    set_gnome_wallpaper(&uri)?;

    print!("Ready!");
    Ok(())
}

fn download_image(url: &str, name_wallpaper: &str) -> Result<PathBuf, Box<dyn Error>> {
    // путь для сохранения обой
    let save_path = PathBuf::from(format!("/home/tea/Изображения/Wallpaper/{}", name_wallpaper));

    // Создаем клиента с ожидание в 30сек
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .timeout(Duration::from_secs(30)) //Ждать 30 сек
        .connect_timeout(Duration::from_secs(10)) //ждать подключения в 30 сек
        .build()?;
    print!("Запрос отправлен жду ответа...");

    // запрос
    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Ошибка сервара: {}", response.status()).into());
    }

    // создание файла
    let mut file = fs::File::create(&save_path)?;

    // копируем данные из интернета в файл
    std::io::copy(&mut response, &mut file)?;

    Ok(save_path)
}

fn get_random_wallpaper(dir_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    // Выбор пути\ Установка пути
    // let path = Path::new("/home/tea/Изображения/Wallpaper");

    // wallpapers - собирает файлы в динамический массив.
    let wallpapers: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok()) // Проверяет удалось ли прочитать файл затем открывается ли файл или он заблокирован/пустой.
        .map(|entry| entry.path()) //берет файл прошедшие проверку и выделяет их путь.
        .filter(|p| p.is_file()) // Проверка на файл/папку.
        .collect(); // Ленивые вычисления делаются до collect. Collect  собирает все, в динамический массив Vec<>

    if wallpapers.is_empty() {
        return Err("No wallpapers found in dir".into());
    }

    //Выбор случайного файла через choose
    let mut rng = rand::rng();
    let selected = wallpapers
        .choose(&mut rng)
        .ok_or("Failed to choose a random file")?;

    // let uri = format!("file://{}", selected.display());
    // set_gnome_wallpaper(&uri)?;

    Ok(selected.clone())
}

fn set_gnome_wallpaper(uri: &str) -> Result<(), Box<dyn Error>> {
    let keys = ["picture-uri", "picture-uri-dark"];

    for key in keys {
        let status = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.background", key, uri])
            .status()?;

        if !status.success() {
            eprint!("Failed to set settings key {}", key);
        }
    }

    Ok(())
}
