# Chain Extension contracts
This repository contains crates of chain extension that you can use in your contracts.

### Extended documentation
To know which chain extension is available in which networks and have more info about it please visit the official [Chain Extension docs](https://docs.astar.network/docs/build/wasm/contract_environment/chain-extension/chain_extensions/)

### Purpose
In `crates` folder you will find the chain extension struct that implements callable functions.
In `contracts` folder you will find full implementation of the chain extension struct and its integration tests in `tests` folder

### Versions
[ink! v5.0.0-rc](https://github.com/paritytech/ink/releases/tag/v5.0.0-rc)

### Chain Extensions

#### Pallet Assets
This crate exposes `AssetsExtension` struct that implement all functions of pallet-assets chain extension.    

**Usage**
1. add `assets_extension` in your `Cargo.toml` and to the `std` `features`
```toml
assets_extension = {  git = "https://github.com/AstarNetwork/chain-extension-contracts", default-features = false }

[features]
default = ["std"]
std = [
    "ink/std",
    "scale/std",
    "scale-info/std",
    "assets_extension/std",
]
```

2. Add use statement in your contract module and declare AssetsExtension type
```rust
use assets_extension::{AssetsError, AssetsExtension as _AssetsExtension};

type AssetsExtension = _AssetsExtension<DefaultEnvironment>;
```

3. Use struct functions directly in your contract
```rust
AssetsExtension::mint(asset_id, beneficiary, amount)?;
```

Note: As precompiles in Solidity, the contract is the `caller` (the Origin of the call in the runtime)

### License
Apache 2.0

## 🏗️ How to use - Contracts
##### 💫 Build
Use these [instructions](https://use.ink/getting-started/setup) to set up your ink!/Rust environment    
Run this command to compile contract:

```sh
yarn
yarn compile
```

2. Run the tests
Ensure a local node is running
```sh
yarn test
```