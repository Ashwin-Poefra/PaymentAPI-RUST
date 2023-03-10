# Rust Axum Api using Lambda with Api Gateway Proxy Integration
The following is a Rust Axum Api using Lambda with Api Gateway Proxy integration. This application uses DynamoDB for storage.


### Prerequisites
1. `Rust` (https://www.rust-lang.org/learn/get-started).
2. `Cargo Lambda` (https://www.cargo-lambda.info/guide/getting-started.html).

### Running Locally
1. Run `cargo lambda watch -a 127.0.0.1 -p 8080` or `make run`.

### Deploying Project
1. Run `cargo lambda build --release --arm64  --output-format zip`.
2. Deploy AWS Lambda function with Api Gateway `/` and `{proxy+}` resources with `ANY` method. With `bootstrap` as handler and `arm64` architecture.
3. Upload zip file located in `/target/lambda/<PROJECT_NAME>/bootstrap.zip` after build.

