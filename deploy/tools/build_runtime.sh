# Builds and moves the lambda runtime for the game lambda into this project
# ready for deployment. Deposits the resulting zipped runtime into the
# deploy/runtime directory

LAMBDA_TARGET=x86_64-unknown-linux-musl
PWD=$(pwd)
SCRIPT_DIR=$(dirname "$0")
LAMBDA_DIR="$PWD/$SCRIPT_DIR/../../game-lambda"
DEPLOY_DIR="$PWD/$SCRIPT_DIR/.."

cargo build --target "$LAMBDA_TARGET" --manifest-path "$LAMBDA_DIR/Cargo.toml" --release
cp "$LAMBDA_DIR/target/$LAMBDA_TARGET/release/game-lambda" "$DEPLOY_DIR/runtime/bootstrap"
zip "$DEPLOY_DIR/runtime/lambda.zip" "$DEPLOY_DIR/runtime/bootstrap"
rm "$DEPLOY_DIR/runtime/bootstrap"

