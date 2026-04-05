use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

mod network;
mod wallpaper;

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
            network::download_image(value, name)?
        }
        "-dir" => {
            let value = args.get(2).ok_or("Ошибка: Вы не ввели путь")?;

            println!("Режим: Случайные обои из папки...");
            wallpaper::get_random_wallpaper(Path::new(value))?
        }
        _ => {
            return Err(format!("Неизвестный флаг '{}'", flag).into());
        }
    };

    let uri = format!("file://{}", wallpaper_path.display());
    println!("Устанавливаю обои: {}", uri);
    wallpaper::set_gnome_wallpaper(&uri)?;

    println!("Ready!");
    Ok(())
}
