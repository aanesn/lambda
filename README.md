## requirements

this tool assumes your aws credentials are correctly setup. the easiest way to do that is to install [aws-cli](https://github.com/aws/aws-cli) and run:

```sh
aws configure
```

## installation

```sh
npm install -g @aanesn/lambda
```

## commands

- `new`: scaffold a new lambda
- `build`: compile and package lambda
- `deploy`: publish lambda to aws

## credits

- `build` and `deploy` commands are inspired by [cargo lambda](https://github.com/cargo-lambda/cargo-lambda)
- `new` command is inspired by [create-o7-app](https://github.com/ottomated/create-o7-app), [create-tauri-app](https://github.com/tauri-apps/create-tauri-app) and [cargo new](https://github.com/rust-lang/cargo/blob/master/src/cargo/ops/cargo_new.rs)
- [lambda web adapter](https://github.com/awslabs/aws-lambda-web-adapter) which makes it possible to run normal http apis on lambda
