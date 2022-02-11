# Near Challenge 2: Hello World

## Deploy the smart contract
```
> ./build.sh
> near create-account hello.{youraccount}.testnet --masterAccount {youraccount}.testnet //If it is not created
> near deploy hello.cy6erninja.testnet --wasmFile target/wasm32-unknown-unknown/release/chal_2_hello_world.wasm
> near view hello.cy6erninja.testnet say_hello
```

## Result
https://explorer.testnet.near.org/accounts/hello.cy6erninja.testnet