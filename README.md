# Chain-extension contracts
This repository contains crates of chain-extension that you can use in your contracts.

### Crates
#### Dapps Staking
This crate exposes `DappsStaking` struct that implement all functions of dapps-staking chain extension. \
**Usage**

1. add `dapps_staking_extension` in your `Cargo.toml` and to the `std` `features`
```rust
dapps_staking_extension = { git = "https://github.com/AstarNetwork/chain-extension-contracts", default-features = false }
...

[features]
default = ["std"]
std = [
"ink_metadata/std",
"ink_env/std",
"ink_storage/std",
"ink_primitives/std",
"scale/std",
"scale-info/std",
"dapps_staking_extension/std" <--- Here
]
```

2. Add use statement in your contract module
```rust
pub mod staking_example {
    use dapps_staking_extension::*;
...
```

3. Use struct functions directly in your contract
```rust
DappsStaking::read_unbonding_period()
```
#### Balances - WIP

#### XCM - WIP

### Examples & Tests
These folders contain an example of how to use chain-extion structs in your contracts. The tests folders is an end-to-end tests for the chain extension. 

**Runs the tests**
1. Run a local node \
Use [swanky-node](https://github.com/AstarNetwork/swanky-node) or [Astar-local](https://github.com/AstarNetwork/Astar) that have the specified chain-extension enabled. Please follow the build & run instructions in their respective repository.
2. The end-to-end test uses redspot as testing environment. Node version should be 14
```bash
yarn install
npx redspot test
```
