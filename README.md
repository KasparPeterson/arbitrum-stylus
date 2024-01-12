Make sure that you have: 
* Rust installed on your system with stylus
* nitro-testnode is running with WASM runtime


```
cargo stylus check \
  --endpoint http://localhost:8547

cargo stylus deploy \
  --endpoint http://localhost:8547 \
  --private-key-path=private-key.txt \
  --estimate-gas-only
  
cargo stylus export-abi

# Usage
node call_contract.js
```