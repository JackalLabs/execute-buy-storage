# execute-buy-storage

## Build With CosmWasm/optimizer
```
docker run --rm -v "$(pwd)":/code --mount type=volume,source="devcontract_cache_burner",target=/target/ --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/optimizer:0.16.1 .
```
This should build the contract to /artifacts.

## Uploading Code:
```
canined tx wasm store [filename] --from [address] --fees=[fee]ujkl --gas=[gas]
```
This returns a contract code id.

## Downloading Code:
```
canined query wasm code [id] [filename]
```
This returns the uploaded .wasm file.

## Instantiate Contract:
```
canined tx wasm instantiate [id] '{"payment": ["ujkl"]}' --from [address]
```
This returns the contract address.

## Query Contract:
```
canined query wasm contract-state smart [contract address] [query]
```
This returns the query results. (or an error if queries are unimplemented)

## Buy Storage:
```
canined tx wasm execute [contract address] '{ "buy_storage": { "for_address": "[address]", "duration": 30, "bytes": 11073741824, "payment_denom": "ujkl" } }' --from [address] --fees=[fee]ujkl --gas=[gas]
```
This returns the Tx buying storage.

## Show Address:
```
canined keys show [name]
```
This returns the wallet address.