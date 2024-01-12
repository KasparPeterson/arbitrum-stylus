```
cargo stylus check \
  --endpoint http://localhost:8547

cargo stylus deploy \
  --endpoint http://localhost:8547 \
  --private-key-path=private-key.txt \
  --estimate-gas-only
  
cargo stylus export-abi
```