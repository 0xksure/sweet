# Sweet

## build as on-chain program

```
    > cargo build-bpf
```

## testing

```
    > cargo
```

## Solana resources

### Functional solana program tests examples

- https://github.com/solana-labs/solana-program-library/blob/c01665832abb5effd76c121fd2695a0d10fb6b04/memo/program/tests/functional.rs#L19

## FAQ

> Error type : _failed to parse manifest at_
> Solution: update solana cli to match the version in the cargo.toml.
> so if the package is version v1.8.0 make sure solana --version returns 1.9.0
