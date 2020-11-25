# Lifts an existing binary which has been compiled against amazonlinux2
# and massages it into an acceptable format for a lambda function
DEPLOY_DIR="$(pwd)/$(dirname "$0")/.."
LAMBDA_DIR="$DEPLOY_DIR/../game-lambda"

cp "$LAMBDA_DIR/target/amazonlinux2/game-lambda" "$DEPLOY_DIR/runtime/bootstrap"
# We need to junk everything except the filename of the binary
zip -j "$DEPLOY_DIR/runtime/lambda.zip" "$DEPLOY_DIR/runtime/bootstrap"
rm "$DEPLOY_DIR/runtime/bootstrap"

