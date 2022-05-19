# yeji-swap-contract
##build
``cargo build --target wasm32-unknown-unknown --release``
##deploy
``near deploy --wasmFile target/wasm32-unknown-unknown/release/yeji_swap_contract.wasm --accountId YOUR_ACCOUNT_HERE``
