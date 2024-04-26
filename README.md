This repo is a minimal reproducible case for an issue I have with `datatest-stable` tests being reported as 'leaky' when run under nextest.


### Requirements

```sh
cargo install nextest --locked
```

### Steps

1. Run `cargo nextest run` inside the repo
2. Observe some number of the 1000 tests being reported as leaky, e.g.
    ```
    Summary [  17.160s] 1000 tests run: 1000 passed (399 leaky), 0 skipped
    ```

