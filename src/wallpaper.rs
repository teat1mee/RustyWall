use rand::prelude::IndexedRandom;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_random_wallpaper(dir_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
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

pub fn set_gnome_wallpaper(uri: &str) -> Result<(), Box<dyn Error>> {
    let keys = ["picture-uri", "picture-uri-dark"];
    for key in keys {
        Command::new("gsettings")
            .args(["set", "org.gnome.desktop.background", key, uri])
            .status()?;
    }

    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-options",
            "zoom",
        ])
        .status()?;

    Ok(())
}
