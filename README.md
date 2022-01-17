# avtag
Shows available tags for package repositories.

## Build
```
cargo build --release
```

Then copy created executable to appropriate path, e.g. `~/bin/avtag`, if
`~/bin` is in your PATH.
```
cp target/release/avtag ~/bin/avtag
```

## Run
First run `avtag` to create config.
It show path to config. You should edit config with your data.

Then `avtag` shows table with available tags.
