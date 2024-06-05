# WASM File Converter

Welcome to the **WASM File Converter** repository! This project is designed to convert different file formats quickly and securely using WebAssembly (WASM). The app is built with TypeScript, Vue, Nuxt, and Rust, utilizing the `image-rs` library for image processing.

## Table of Contents

- [WASM File Converter](#wasm-file-converter)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- **Fast and Secure**: Client-side conversions ensure speed and privacy.
- **Supports Multiple Formats**: Convert between various image formats (png, jpg, jpeg, gif, webp, etc.).
- **User-Friendly Interface**: Simple drag-and-drop functionality with a clean, modern design.
- **Free and Open Source**: No cost and open for contributions.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

- **Node.js** (v16 or higher)
- **pnpm**
- **Rust** and **Wasm-pack**

### Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/tomvoet/wasm-convert.git
    cd wasm-convert
    ```

2. **Install dependencies**:
    ```sh
    pnpm i
    ```

3. **Build the Rust project**:
    ```sh
    pnpm wasm:bundle
    ```

4. **Start the development server**:
    ```sh
    pnpm dev
    ```

5. Open your browser and navigate to `http://localhost:3000`.

## Usage

1. **Upload your file** by dragging and dropping it into the designated area or by clicking to upload.
2. **Select the output format** from the dropdown menu.
3. **Click "Start Conversion"** to convert the file.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
