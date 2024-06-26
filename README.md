# typirst

`typirst` is a simple rust TUI application to test your typing skills. It is
inspired by monkeytype.com.

## Installation

Download the executable from the [Releases][latest-release] page. Move the
executable to a directory in your PATH (e.g. `~/.local/bin/`) and make it
executable.

```sh
mv typirst ~/.local/bin/
chmod +x ~/.local/bin/typirst
```

## Options

By pressing Esc, you can change the options of the current test. The options
are:

- Number of words
- Difficulty (lowercase, uppercase, numbers, symbols)
- Highlighting (current character, current word, next word, next 2 words)

[latest-release]: https://github.com/vtsiolkas/typirst/releases/latest
