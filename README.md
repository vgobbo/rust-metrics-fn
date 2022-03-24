# Function Metrics

This project is divided in a _build time_ module (`metrics-fn-codegen`), and a _runtime_ module (`metrics-fn`).

Simply include `metrics-fn` as a dependency and configure the features for the desired metric recorders to start using it.

For example, to enable collecting Prometheus metrics, use the following:
```toml
metrics-fn = { version = "0.1", features = [ "record-prometheus" ] }
```

For more info about the available recorders and usage, see _PENDING_.

## Code Style

Configure a pre-commit hook so you don't have to worry about it ever again:
```bash
$ ln -sr pre-commit .git/hooks/pre-commit
```

A nightly profile is also required. Ensure you have one installed by running:
```bash
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/user/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.59.0 (9d1b2106e 2022-02-23)
```