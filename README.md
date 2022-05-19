
# YejiSwap smart contract

Smart contract for decentralized exchange on NEAR protocol.  
Final project for bachelor of computer engineering.
## Tech Stack

**Web:** [React](https://www.github.com/jrapbit)

**Smart Contract:** Rust


## Authors

- [@jrapbit](https://www.github.com/jrapbit)
- [@narawitPtm](https://www.github.com/narawitPtm)

## How to deploy

First build your rust project to .wasm

```bash
  cargo build --target wasm32-unknown-unknown --release
```
Then replace `YOUR_ACCOUNT_HERE` with your wallet address.

```bash
  near deploy --wasmFile target/wasm32-unknown-unknown/release/yeji_swap_contract.wasm --accountId YOUR_ACCOUNT_HERE
```
