
切换工作环境 13：09 2h
配置代理服务器

使用solana_client_wasm 读取数据
测试使用Vec保存数据
新建项目 重命名项目名称
测试本地节点的访问速度


# oxylana
### Full Stack Solana Development in Rust
This repository provides a template for a Solana application with a full Rust stack. The repository includes

:white_check_mark: Rust Smart Contract (Anchor)

:white_check_mark: Rust Frontend (Dioxus)

:white_check_mark: Rust Unit Tests

A dummy keypair is provided. Presently, the frontend only supports Phantom, but similar adapters can be built for other wallets. This repository takes inspiration from [this repository](https://github.com/russellwmy/yew-dapp-examples).

There are many benefits to having Rust across the stack. The main benefit is having faster feedback/errors due to Rust's type system between frontend and backend code.

### Getting started

To run the unit tests, use `anchor test`.

To display the frontend, first spin up an `anchor localnet` and then run `trunk serve`.

If you do not have `trunk`, install via `cargo install trunk`.

This requires the wasm32 target, obtainable via `rustup target install wasm32-unknown-unknown` if you do not yet have it.

The use of localnet is hardcoded into the template; be sure to switch your wallet to localnet to avoid issues regarding recent blockhashes.

### Learn More

Visit the Dioxus, Trunk, and Anchor docs/repositories to learn more.

oxyzEsUj9CV6HsqPCUZqVwrFJJvpd9iCBrPdzTBWLBb