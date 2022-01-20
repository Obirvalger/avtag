# avtag
Shows available tags for git repositories.

## Build
```
cargo build --release
```

Then copy created executable to appropriate path, e.g. `~/bin/avtag`, if
`~/bin` is in your PATH.
```
cp target/release/avtag ~/bin/avtag
```

Completions e.g. zsh could be installed via
```
avtag --completion=zsh >| ~/.zsh/completion/_avtag
```
Available completions are: bash, elvish, fish, powershell and zsh.

## Run
First run `avtag` to create config.
It show path to config. You should edit config with your data.

To add repos with default values run one of the following commands:
* Some repos from command line:
```
echo repo1 repo2 | xargs -n 1 | sed -E 's/(.*)/[[repos]]\n    path = "\1"/ >> ~/.config/avtag/config.toml
```
* All repos in some dir:
```
ls ~/repos-dir | sed -E 's/(.*)/[[repos]]\n    path = "\1"/' >> ~/.config/avtag/config.toml
```

Then `avtag` shows table with available tags.
Example output:
```
┌─────────────┬───────────────┬─────────────────────────┐
│ Name        ┆ Built version ┆ Tags                    │
╞═════════════╪═══════════════╪═════════════════════════╡
│ consul      ┆ 1.10.4        ┆ v1.11.1 v1.11.0 v1.11.2 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ cri-o       ┆ 1.22.1        ┆ v1.23.0                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ cri-tools   ┆ 1.22.0        ┆ v1.23.0                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ firecracker ┆ 0.23.1        ┆ v0.25.1 v0.25.2         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ grpcurl     ┆ 1.7.0         ┆ v1.8.0 v1.8.1 v1.8.4    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ jsonnet     ┆ 0.17.0        ┆ v0.18.0                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ k8s         ┆ 1.22.5        ┆ v1.22.6                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ libguestfs  ┆ 1.46.2        ┆ v1.47.2 v1.47.1         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ restic      ┆ 0.12.0        ┆ v0.12.1                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ semaphore   ┆ 2.5.1         ┆ v2.8.25 v2.8.0 v2.8.19  │
└─────────────┴───────────────┴─────────────────────────┘
```
