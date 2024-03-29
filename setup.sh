#!/usr/bin/env bash

# make sure to install diesel_cli:
# cargo install diesel_cli --no-default-features --features sqlite


$HOME/.local/share/cargo/bin/diesel setup
$HOME/.local/share/cargo/bin/diesel migration generate --diff-schema create_games
$HOME/.local/share/cargo/bin/diesel migration run
