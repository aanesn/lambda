> [!WARNING]  
> it's still pretty early stages, so i probably wouldn't use it for anything serious

![demo](https://pub-b0a99bf888b34aaf8ebc5b21b7df2dc7.r2.dev/lambda.gif)

## requirements

you'll need to configure your aws credentials, the easiest way is to install [aws-cli](https://github.com/aws/aws-cli) and run `aws configure`

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
