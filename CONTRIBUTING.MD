# Contributing to DNS-Server-Rust

Thank you for considering contributing to the **DNS-Server-Rust** project! 🎉 This document provides guidelines to help you successfully contribute, whether you’re fixing bugs, improving documentation, or adding features.

---

## Table of Contents
1. [How Can I Contribute?](#how-can-i-contribute)
   - [Reporting Bugs](#reporting-bugs)
   - [Suggesting Features](#suggesting-features)
   - [Improving Documentation](#improving-documentation)
   - [Contributing Code](#contributing-code)
2. [Project Structure](#project-structure)
3. [Code Guidelines](#code-guidelines)
4. [Pull Request Process](#pull-request-process)
5. [Testing](#testing)
6. [Getting Started](#getting-started)
7. [Code of Conduct](#code-of-conduct)

---

## How Can I Contribute?

### Reporting Bugs

If you encounter a bug or unexpected behavior:
1. Check if it has already been reported in [Issues](https://github.com/SkySingh04/DNS-Server-Rust/issues).
2. If not, open a new issue with the following details:
   - A clear and descriptive title.
   - Steps to reproduce the problem.
   - The version of Rust and operating system.
   - Any error messages or relevant logs.
   - Screenshots (if applicable).

---

### Suggesting Features

We’re always open to suggestions! If you have an idea:
1. Check the [Issues](https://github.com/SkySingh04/DNS-Server-Rust/issues) to see if the feature has already been requested.
2. Open a new issue using the "Feature Request" template:
   - Describe the feature clearly.
   - Explain the use case or problem it solves.
   - Include any references or links that might help.

---

### Improving Documentation

Documentation is key for a successful project. If you notice any gaps:
- Update existing README or inline comments.
- Add examples or clarify confusing areas.
- Submit a Pull Request for documentation changes.

---

### Contributing Code

Follow these steps to contribute code:
1. **Fork the Repository**  
   Go to the project repository and click the **Fork** button.

2. **Clone the Repository**  
   Clone your forked repository:
   ```bash
   git clone https://github.com/SkySingh04/DNS-Server-Rust/DNS-Server-Rust.git
   cd DNS-Server-Rust
   ```

3. **Create a Branch**  
   Use a descriptive branch name:
   ```bash
   git checkout -b feature/my-new-feature
   ```

4. **Implement Your Changes**  
   Follow the [Code Guidelines](#code-guidelines).

5. **Test Your Code**  
   Ensure your changes do not break existing functionality:
   ```bash
   cargo test
   ```

6. **Commit Your Changes**  
   Write meaningful commit messages:
   ```bash
   git add .
   git commit -m "Add support for MX record type"
   ```

7. **Push and Submit a PR**  
   Push your changes:
   ```bash
   git push origin feature/my-new-feature
   ```
   Then create a Pull Request on GitHub.

---

## Project Structure

Here’s an overview of the key files and directories:

```
DNS-Server-Rust/
│
├── src/
│   ├── main.rs               # Entry point of the application
│   ├── protocol/
│   │   ├── dnsheader.rs      # DNS Header implementation
│   │   ├── dnsquestion.rs    # DNS Question section
│   │   ├── dnsrecord.rs      # DNS Record section
│   │   ├── dnspacket.rs      # DNS Packet handling
│   │   ├── byte_packet_buffer.rs # Reads/writes DNS packets
│   │   ├── querytype.rs      # Query type implementation
│   │   ├── resultcode.rs     # DNS response codes
│   │
├── response_packet.txt       # Sample DNS response packet
├── Cargo.toml                # Rust dependencies
├── README.md                 # Project overview and notes
└── CONTRIBUTING.md           # Contribution guidelines
```

---

## Code Guidelines

Follow these coding standards for consistency:
1. **Formatting**: Use `rustfmt` for consistent formatting.
   ```bash
   rustfmt src/*.rs
   ```

2. **Linting**: Ensure clean code using `clippy`.
   ```bash
   cargo clippy
   ```

3. **Error Handling**: Use `Result` and proper error handling instead of panics.

4. **Testing**: Write unit tests for all new features and edge cases.

5. **Naming**: Use snake_case for variables and functions, and PascalCase for types.

6. **Documentation**: Add comments and Rust docstrings (`///`) for public modules and functions.

---

## Pull Request Process

1. Ensure your PR targets the **main** branch.
2. Include a concise description of the changes.
3. Reference any related issues (e.g., `Fixes #123`).
4. Ensure the code compiles without errors or warnings.
5. Write or update tests, if applicable.
6. Add documentation if needed.

Once submitted, maintainers will review your PR. Be open to feedback and iterate as necessary.

---

## Testing

Run tests locally using Cargo:
```bash
cargo test
```

For any new feature, write tests in the corresponding file under `src/protocol/`.

---

## Getting Started

To set up the project locally:
1. Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/).
2. Clone the repository:
   ```bash
   git clone https://github.com/SkySingh04/DNS-Server-Rust/DNS-Server-Rust.git
   cd DNS-Server-Rust
   ```
3. Build and run the project:
   ```bash
   cargo run
   ```

4. Test the DNS server using `dig`:
   ```bash
   dig @127.0.0.1 -p 2053 google.com
   ```

---

## Code of Conduct

This project adheres to the [Contributor Covenant](https://www.contributor-covenant.org/). Please be respectful to others, whether you’re contributing code, opening an issue, or participating in discussions.

---

Thank you for contributing to **DNS-Server-Rust**! Let’s build something awesome together. 🚀
