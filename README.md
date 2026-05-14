# 0-shell 🐚

> A simple Unix shell implemented from scratch in Rust.

**0-shell** (internally named `minishel`) is a project that dives deep into how shells really work — from process creation and signal handling to I/O redirection and piping. Built entirely in Rust for memory safety and performance.

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build](#build)
  - [Run](#run)
- [Built-in Commands](#built-in-commands)
- [Usage Examples](#usage-examples)
- [Contributing](#contributing)
- [License](#license)

---

## About

This project involves designing and implementing a simple shell (command-line interpreter) from scratch. The goal is to gain a deep understanding of:

- How shells parse and execute commands
- How processes are created and managed (`fork` / `exec`)
- How **I/O redirection** (`>`, `<`, `>>`) works under the hood
- How **pipes** (`|`) chain commands together
- How **built-in commands** differ from external binaries

---

## Features

- Interactive REPL prompt with username and current directory display
- Command execution (external binaries from `$PATH`)
- Pipes — chain multiple commands: `cmd1 | cmd2 | cmd3`
- I/O redirection — `>`, `<`, `>>`
- Built-in commands (`cd`, `echo`, `exit`, `pwd`, `env`, etc.)
- Signal handling (`Ctrl+C`, `Ctrl+D`)
- Terminal control via `crossterm` for a clean interactive experience

---

## Tech Stack

| Tool        | Purpose                                       |
|-------------|-----------------------------------------------|
| **Rust**    | Core language — memory safe, no GC            |
| `crossterm` | Cross-platform terminal manipulation          |
| `users`     | Fetch current user info for the prompt        |
| `chrono`    | Date/time support                             |
| `tempfile`  | Temporary files for pipe/redirection handling |

---

## Project Structure

```
0-shell/
├── src/               # All Rust source files
│   └── main.rs        # Entry point
├── Cargo.toml         # Rust project manifest & dependencies
└── .gitignore
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, via `rustup`)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

```bash
git clone https://github.com/helbadaou/0-shell.git
cd 0-shell
cargo build --release
```

The compiled binary will be at:

```
target/release/minishel
```

### Run

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/release/minishel
```

You'll be greeted with an interactive prompt:

```
user@hostname:~$ 
```

---

## Built-in Commands

| Command           | Description                          |
|-------------------|--------------------------------------|
| `cd [dir]`        | Change the current directory         |
| `pwd`             | Print the current working directory  |
| `echo [args]`     | Print arguments to stdout            |
| `env`             | Display environment variables        |
| `export VAR=val`  | Set an environment variable          |
| `unset VAR`       | Unset an environment variable        |
| `exit [code]`     | Exit the shell with an optional code |

---

## Usage Examples

```bash
# Run a simple command
$ ls -la

# Pipe commands
$ ls -la | grep ".rs" | wc -l

# Redirect output to a file
$ echo "hello world" > output.txt

# Append to a file
$ echo "another line" >> output.txt

# Read input from a file
$ cat < output.txt

# Combine pipes and redirections
$ cat < input.txt | grep "pattern" > result.txt

# Change directory
$ cd /tmp && pwd
```

---

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Commit your changes: `git commit -m "feat: describe your change"`
4. Push and open a Pull Request

---

## License

This project is open source. See [LICENSE](LICENSE) for details.

---

<p align="center">Built with 🦀 Rust — because every great shell deserves a great language.</p>
