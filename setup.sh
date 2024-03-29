#!/usr/bin/env bash

# make sure to install diesel_cli
# cargo install diesel_cli

/home/drfailer/.local/share/cargo/bin/diesel setup
/home/drfailer/.local/share/cargo/bin/diesel migration generate --diff-schema create_games
/home/drfailer/.local/share/cargo/bin/diesel migration run
