# RPG Scheduler

Simple scheduler app for RPG.

## Setup

```bash
cargo install diesel_cli --no-default-features --features sqlite
./setup.sh
cargo run
```

## Test

```python
import requests
import json

# create a game
post_resp = requests.post("http://localhost:8080/scheduler/game/game_name")
print(post_resp.text)

# get the id
json_output = json.loads(post_resp.text)
game_id = json_output['id']

get_resp = requests.get(f"http://localhost:8080/scheduler/game/{game_id}")
print(get_resp.text)

delete_resp = requests.delete(f"http://localhost:8080/scheduler/game/{game_id}")
```

## Links

- [atix db](https://actix.rs/docs/databases/)
- [atix db](https://actix.rs/docs/databases/)
- [diesel doc](https://diesel.rs/guides/getting-started)
