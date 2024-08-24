# Cargo workflows

A library for running workflows.

> [!WARNING]
> This is deprecated, use `cargo-make` instead.

## Usage

First, install the crate using `cargo install cargo-workflows`.

Second, initialize a new `workflows.toml` file using `cargo workflows init`.

Third, open the file and you should see:
```toml
# "default" is the default workflow
[default]

# Commands to run
[default.commands]
clippy = [ "cargo", "clippy" ]
build = [ "cargo", "build", "--release" ]
run = [ "cargo", "run", "--release" ]

# Environment variables
[default.env]
"Env 1" = "true"
"Env-2" = "42"
"Env_3" = "50"
```