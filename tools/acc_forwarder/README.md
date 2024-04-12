# Account Forwarder

Account Forwarder is a tool designed to replay account changes, primarily originating from geyser. It forwards serialized account information to a specified Redis endpoint.

## Usage

### Send a single account

To forward information about a single account, run the following command:

`cargo run -- --redis-url <REDIS_URL> --rpc-url <RPC_URL> account --account <ACCOUNT>`

### Send mint, Asset controller, Data registry, Identity registry and Policy registry Account

To forward mint and the corresponding rwa accounts for a specific mint, use the following commands.

Locally:

```
cargo run -- --redis-url redis://localhost --rpc-url http://localhost:8899 mint --mint DyuYvHwMtYGt4xJoToce4CEuBk62uGhiYDXeogwSfD3D
```

Dev/Prod:
`cargo run -- --redis-url $REDIS_URL --rpc-url $RPC_URL mint --mint t8nGUrFQozLtgiqnc5Pu8yiodbrJCaFyE3CGeubAvky`

### Process accounts from a file

To forward account information for multiple accounts listed in a file, execute the following command:

`cargo run -- --redis-url <REDIS_URL> --rpc-url <RPC_URL> mint-scenario --scenario-file <FILENAME>`

Replace <REDIS_URL>, <RPC_URL>, <ACCOUNT>, and <FILENAME> with the appropriate values for your use case.

