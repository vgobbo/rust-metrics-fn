# Function Metrics

## Code Style

Configure a pre-commit hook so you don't have to worry about it ever again:
```bash
$ ln -sr pre-commit .git/hooks/pre-commit
```

A nightly profile is also required. Ensure you have one installed by running:
```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/vgobbo/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu
gentoo

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.57.0 (f1edd0429 2021-11-29)
```