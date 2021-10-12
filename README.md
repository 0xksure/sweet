# Sweet

## solana cli

Make sure you use the v1.8.0 of the solana cli.

To download run the following cmd in your terminal

```
    > sh -c "$(curl -sSfL https://release.solana.com/v1.8.0/install)"
```

## build as on-chain program

```
    > cargo build-bpf
```

this will create the solana program executable in the target folder. If you wish to deploy the program hit

```
    > solana program deploy /Users/kristofferhovlandberg/shiftx/personal/sweet/target/deploy/sweet.so
```

make sure to set the --url flag to the right network

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
