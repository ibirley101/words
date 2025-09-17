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

You can then continue playing and use the `help` command to show you the best move.

```txt
> show
   00 01 02 03 04 05 06 07 08 09 10 11 12 13 14
00 tw -- -- dl -- -- -- tw -- -- -- dl -- -- tw
01 -- dw -- -- -- tl -- -- -- tl -- -- -- dw --
02 -- -- dw -- -- -- dl -- C  -- -- -- dw -- --
03 dl -- -- dw -- -- -- G  I  -- -- dw -- -- dl
04 -- -- -- -- dw -- -- E  N  -- dw -- -- -- --
05 -- tl -- -- -- tl -- N  E  D  -- -- -- tl --
06 -- -- dl -- -- -- dl T  dl E  -- -- dl -- --
07 tw -- -- dl -- -- -- O  -- I  -- dl -- -- tw
08 -- -- dl -- -- -- dl O  dl C  -- -- dl -- --
09 -- tl -- -- -- tl -- S  -- E  -- -- -- tl --
10 -- -- -- -- dw -- -- -- -- -- dw -- -- -- --
11 dl -- -- dw -- -- -- dl -- -- -- dw -- -- dl
12 -- -- dw -- -- -- dl -- dl -- -- -- dw -- --
13 -- dw -- -- -- tl -- -- -- tl -- -- -- dw --
14 tw -- -- dl -- -- -- tw -- -- -- dl -- -- tw

Rack:
There are 81 tiles in the bag.
Score: 16
> help c e e h i n w 
Highest scorer is WHENCE at (7, 6) down for 35 points.
```
