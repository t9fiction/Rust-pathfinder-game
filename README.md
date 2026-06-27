# Pathfinder Game

A text-based dungeon adventure game in Rust. Navigate rooms using directional commands.

## Usage

```bash
cargo run
```

## Commands

| Command | Action |
|---------|--------|
| `look` | Describe current room and exits |
| `go north/south/east/west` | Move to adjacent room |
| `quit` | Exit game |

## Rooms

Entrance → Hall → Armory → Treasure Room

## Dependencies

- `rustyline` — readline input with history
