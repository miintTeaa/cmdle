# cmdle
A word game for the command line.

Run `cmdle --help` to see a list of available commands.

## Installing
If you're on windows, get the zip from the latest release and unzip it. Add the cmdle folder to your path, and run `cmdle help` to see if you've installed it properly.

If you're on another platform, you'll probably have to firgue out how to build it yourself (it shouldn't be that hard though). (If anyone wants to modify Makefile.toml so it works with other platforms though, please go ahead and make a pull request)

## Setting it up
If you want to use `cargo run`, you'll have to copy `goals.json` and `guesses.json` into both `target/debug` and `target/release`.

You could do this manually, or use `cargo make setup` (you need cargo-make for this, see below on how to install it).

## Building
I'm assuming you already have cargo and rust installed. Also, as a reminder: **these instructions are meant for windows**, but they should be easily modifiable to suit other platforms.

- Install `cargo-make`: https://github.com/sagiegurari/cargo-make#installation

- Install `7zip`: https://www.7-zip.org/download.html

- Run `cargo make pack` in the project directory.

This will output a zip file at `target/packed/cmdle.zip`.

*Technically*, you can also do this without cargo-make, it just makes it more convenient. Simply make a `target/packed` directory, run `cargo build --release` and run `7z a -tzip target/packed/cmdle.zip goals.json guesses.json target/release/cmdle.exe`. Keep in mind - these instructions may change in future versions, but `cargo make pack` will always work.
