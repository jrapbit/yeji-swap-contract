cargo build --target wasm32-unknown-unknown --release
# near dev-deploy -f --initFunction new --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/yeji_swap_contract.wasm
near deploy -f --initFunction new --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/yeji_swap_contract.wasm --accountId jrapbit2.testnet