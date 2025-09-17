Helper for Scrabble-Likes
=========================

# Command Line Syntax

```txt
words [PLAYER1-TYPE] [PLAYER2-TYPE] [PLAYER3-TYPE] [PLAYER4-TYPE]
```

Where the types are `human`, `human-no-rack`, `cpu`, and `none`.
All players default to `none`.

## Bot vs. Bot Example

```txt
cargo build --release
./target/release/words.exe cpu cpu
```

## Human vs. Bot

```txt
cargo build --release
./target/release/words.exe human cpu
```

## Help Mode

Suppose you're playing on a real board and want to find out the best word to
play in some situation. The `human-no-rack` player type removes the virtual rack, allowing
you to play any tile you want.

```txt
cargo build --release
./target/release/words.exe human-no-rack
```
