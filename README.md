# Chain Extension contracts
This repository contains crates of chain extension that you can use in your contracts.

### Extended documentation
To know which chain extension is available in which networks and have more info about it please visit the official [Chain Extension docs](https://docs.astar.network/docs/build/wasm/contract_environment/chain-extension/chain_extensions/)

### Purpose
In `crates` folder you will find the chain extension struct that implements callable functions.
In `examples` folder you will find full implementation of the chain extension struct and its integration tests in `tests` folder

### Versions
[ink! v4.2.0](https://github.com/paritytech/ink/releases/tag/v4.2.0)   
[openbrush 3.1.0 ](https://github.com/Brushfam/openbrush-contracts/releases/tag/3.1.0)

### Chain Extensions

#### Pallet Assets
This crate exposes `AssetsExtension` struct that implement all functions of pallet-assets chain extension.    

**Usage**
1. add `assets_extension` in your `Cargo.toml` and to the `std` `features`
```toml
assets_extension = {  git = "https://github.com/swanky-dapps/chain-extension-contracts", default-features = false }

[features]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink/std",
    "scale/std",
    "scale-info/std",
    "assets_extension/std",
]
```

2. Add use statement in your contract module
```rust
use assets_extension::*;

```

3. Use struct functions directly in your contract
```rust
AssetsExtension::create(Origin::Address, asset_id, contract, min_balance)
```

Note: For now only call as contract `Origin::Address` is supported

#### Dapps Staking
This crate exposes `DappsStaking` struct that implement all functions of dapps-staking chain extension.     

**Usage**
1. add `dapps_staking_extension` in your `Cargo.toml` and to the `std` `features`
```toml
dapps_staking_extension = { git = "https://github.com/swanky-dapps/chain-extension-contracts", default-features = false }

[features]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink/std",
    "scale/std",
    "scale-info/std",
    "dapps_staking_extension/std"
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

#### Pallet Scheduler
This crate exposes `SchedulerExtension` struct that implement functions of pallet-scheduler chain extension.

**Usage**
1. add `scheduler_extension` in your `Cargo.toml` and to the `std` `features`
```toml
scheduler_extension = {  git = "https://github.com/swanky-dapps/chain-extension-contracts", default-features = false }

[features]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink/std",
    "scale/std",
    "scale-info/std",
    "scheduler_extension/std",
]
```

2. Add use statement in your contract module
```rust
    use scheduler_extension::*;
```

3. Use struct functions directly in your contract
```rust
SchedulerExtension::schedule(Origin::Address, when, maybe_periodic, 0, call_input);
```

Note: For now only call as contract `Origin::Address` is supported

### License
Apache 2.0

## 🏗️ How to use - Contracts
##### 💫 Build
Use these [instructions](https://use.ink/getting-started/setup) to set up your ink!/Rust environment    
Run this command in the contract folder:

```sh
cargo contract build
```

##### 💫 Deploy
First start your local node.  
Deploy using contracts UI. Instructions on [Astar docs](https://docs.astar.network/docs/wasm/sc-dev/polkadotjs-ui)

##### 💫 Run Typechain integration test
First start your local node. 
This repo needs a node version of at least `polkadot v0.9.37`
Recommended [swanky-node 1.4.0](https://github.com/AstarNetwork/swanky-node/releases/tag/v1.4.0)

```sh
yarn
yarn compile
yarn test
```

##### 💫 Run ink! e2e test
1. Add the path of the node (astar-local or swanky). example:
```sh
export CONTRACTS_NODE="/home/astar/Documents/astar/Astar/target/release/astar-collator"
```

2. Run the tests
```sh
   cargo test --features e2e-tests
```