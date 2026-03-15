# Gemini Desktop for Linux

An unofficial Linux desktop wrapper for Google Gemini, built with Rust and Tauri. This is a port of the macOS Gemini Desktop wrapper, designed specifically with Arch Linux and minimal resource usage in mind.

## Features
- **Main Window**: Access `https://gemini.google.com` natively via WebKit2GTK.
- **Chat Bar**: A frameless, always-on-top quick-access panel.
- **Global Shortcut**: Toggle the chat bar from anywhere using `Ctrl + Space`.
- **Zoom Control**: Use `Ctrl + =`, `Ctrl + -`, and `Ctrl + 0` to adjust text size.

## Prerequisites (Arch Linux)

Install the required build dependencies:
```bash
sudo pacman -S base-devel curl wget pnpm rustup webkit2gtk-4.1
rustup default stable
```

## Build and Run

1. Clone or enter the project directory:
   ```bash
   cd gemini-desktop-linux
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build the release version:
   ```bash
   npm run tauri build
   ```
   The executable will be located in `src-tauri/target/release/gemini-desktop-linux`.

   *Note: If you encounter a `failed to run linuxdeploy` error during the AppImage bundling process on Linux, you can bypass it by running:*
   ```bash
   APPIMAGE_EXTRACT_AND_RUN=1 NO_STRIP=true npm run tauri build
   ```

## Disclaimer
This project is **not affiliated with, endorsed by, or sponsored by Google**. "Gemini" is a trademark of **Google LLC**. This app does not modify, scrape, or redistribute Gemini content — it simply loads the official website.
