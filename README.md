# HackMD Desktop

A desktop application for [HackMD.io](https://hackmd.io) built with Tauri, providing a native desktop experience for the HackMD collaborative Markdown editor.

## Description

This application wraps the HackMD.io web service in a native desktop window, allowing you to use HackMD without having to keep it open in a browser tab. It provides a seamless desktop experience while preserving all the functionality of the web application.

## Complete Setup Guide

### Prerequisites

Before you can build and run this project, you need to install the following prerequisites:

1. **Node.js and npm** (v16 or higher)
   - Download and install from [Node.js official website](https://nodejs.org/)
   - Verify installation: `node -v` and `npm -v`

2. **Rust and Cargo**
   - Install Rust using rustup:
     - macOS/Linux: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
     - Windows: Download and run [rustup-init.exe](https://win.rustup.rs/)
   - Verify installation: `rustc --version` and `cargo --version`

3. **System Dependencies for Tauri**
   - **macOS**:
     - Install Xcode Command Line Tools: `xcode-select --install`
     - Optionally install [Homebrew](https://brew.sh/)
   
   - **Windows**:
     - Install [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
     - Make sure to select the "Desktop Development with C++" workload
   
   - **Linux** (Ubuntu/Debian):
     ```bash
     sudo apt update
     sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libappindicator3-dev librsvg2-dev patchelf
     ```

### Project Setup

After cloning the repository and installing the prerequisites, follow these steps to set up the project:

1. **Navigate to project directory**
   ```bash
   cd hackmd-desktop
   ```

2. **Install Node dependencies**
   ```bash
   npm install
   ```

3. **Verify Tauri can build successfully**
   ```bash
   npm run tauri info
   ```
   This should display information about your Tauri environment if everything is set up correctly.

### Running the Application

#### Development Mode

To run the application in development mode:

```bash
npm run start
```

This will launch the application with hot-reloading for any changes made to the Rust code.

#### Build for Production

To create a production build that can be distributed:

```bash
npm run build-app
```

The built application will be available in the `src-tauri/target/release/bundle` directory.

## Troubleshooting

### Common Issues

1. **Missing Rust components**
   ```bash
   rustup component add rustfmt clippy
   ```

2. **Compilation errors related to dependencies**
   - Make sure you have all system dependencies installed for your platform
   - Try running `cargo clean` in the `src-tauri` directory, then rebuild

3. **WebView loading issues**
   - In development mode, check the console for errors
   - Make sure your internet connection is working properly

## Project Structure

- `src-tauri/`: Contains the Rust backend code that powers the desktop window
  - `src/`: Rust source code
  - `tauri.conf.json`: Configuration for the Tauri application
- `index.html`: Minimal HTML file used for the WebView container
- `package.json`: Node.js package configuration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is for personal use and is not affiliated with the official HackMD team. Please respect HackMD's terms of service when using this application.
