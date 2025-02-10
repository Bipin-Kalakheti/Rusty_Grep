# Rusty_Grep 🦀

A lightning-fast command-line search utility built in Rust, bringing the power of grep to your fingertips with modern ergonomics and blazing performance.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🔍 Fast pattern searching in text files
- 🎯 Case-sensitive and case-insensitive search modes
- 📁 Support for searching across multiple files
- ⚡ Optimized for performance using Rust's zero-cost abstractions
- 🛡️ Robust error handling with clear, helpful messages

## 🚀 Quick Start

### Prerequisites

- Rust (2021 edition)
- Cargo package manager

### Installation

```bash
# Clone the repository
git clone https://github.com/Bipin-Kalakheti/Rusty_Grep.git

# Navigate to project directory
cd Rusty_Grep

# Build the project
cargo build --release
```

## 💻 Usage

Basic pattern search:
```bash
cargo run <pattern> <file_path>
```

Example:
```bash
cargo run "search_term" poem.txt
```

## 🧪 Testing

Run the comprehensive test suite:
```bash
cargo test
```

## 🏗️ Project Structure

```
src/
├── main.rs    # Application entry point
├── lib.rs     # Core search functionality
└── tests/     # Test modules
```

## 🤝 Contributing

Contributions make the open source community amazing! Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

## 👤 Author

**Bipin Kalakheti**

* GitHub: [@Bipin-Kalakheti](https://github.com/Bipin-Kalakheti)

## ⭐ Show your support

Give a ⭐️ if this project helped you!

## 📚 Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [Grep Manual](https://www.gnu.org/software/grep/manual/grep.html)

---
<div align="center">
Made with ❤️ and Rust 🦀
</div>