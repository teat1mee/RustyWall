🦀 RustyWall

A lightweight, blazingly fast CLI utility written in Rust to manage your GNOME desktop wallpapers. Whether you have a local collection or found a cool image online, RustyWall handles it for you.

🚀 Features

    🖼️ Local Randomizer: Automatically picks a random image from any specified directory.

    🌐 Web Downloader: Provide a direct URL, and the tool will download and apply the wallpaper instantly.

    🛡️ Robust Error Handling: Built-in checks for network timeouts, invalid paths, and server-side errors.

    🐧 Linux Native: Optimized for GNOME environments using gsettings.

🛠 Installation

    Clone the repository:
    code Bash

    git clone git@github.com:teat1mee/RustyWall.git
    cd RustyWall

    Build and run:
    code Bash

    cargo build --release

📖 Usage

    From local folder: ./target/release/RustyWall /home/user/Pictures/Wallpapers

    From URL: ./target/release/RustyWall https://example.com/image.jpg


🦀 RustyWall

Легкая и быстрая консольная утилита на Rust для управления обоями рабочего стола в среде GNOME. Больше не нужно вручную скачивать и устанавливать картинки — RustyWall сделает это за вас.

🚀 Возможности

    🖼️ Случайный выбор: Автоматически выбирает случайное изображение из локальной папки.

    🌐 Загрузка из сети: Просто вставьте прямую ссылку на картинку, и утилита сама скачает и установит её.

    🛡️ Надежность: Обработка сетевых таймаутов, ошибок доступа и проверка путей на лету.

    🐧 Нативно для Linux: Работает напрямую с gsettings в GNOME.

📖 Как пользоваться

    Из папки: ./RustyWall /путь/к/папке/с/обоями

    По ссылке: ./RustyWall https://ссылка.com/картинка.jpg
