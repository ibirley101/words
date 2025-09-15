Helper for Scrabble-Likes
=========================

## Bot vs. Bot

```txt
cargo build --release
./target/release/words.exe --num-cpus 2
```

## Human vs. Bot

```txt
cargo build --release
./target/release/words.exe --num-humans 1 --num-cpus 1
```

## Help Mode

Suppose you're playing on a real board and want to find out the best word to
play in some situation. The `--rackless` removes the virtual rack, allowing
you to play any tile you want.

```txt
cargo build --release
./target/release/words.exe --num-humans 1 --rackless
```
