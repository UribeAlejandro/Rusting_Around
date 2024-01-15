# Rusting Around

## Installation

To install Rust on Unix like systems, run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Project Setup

To create a new project, run the following command in your terminal:

```bash
cargo new <project-name>
```

Or if you want to create a project in a existing directory, run the following command in your terminal:

```bash
cargo init .
```

Install pre-commit hooks (you need to have Python installed):

```bash
pre-commit install
```

Run pre-commit hooks:

```bash
pre-commit run --all
```

## Build

To build a project, run the following command in your terminal:

```bash
cargo build
```

## Run

To run a project, run the following command in your terminal:

```bash
cargo run
```
