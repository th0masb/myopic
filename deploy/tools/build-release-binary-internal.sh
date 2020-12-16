cd ~
git clone "$REPO_ADDRESS" && cd "$(basename "$_" .git)"
git checkout "$REPO_BRANCH"
cargo build --manifest-path "$PROJECT_MANIFEST_DIR/Cargo.toml" --release
cp -f "$PROJECT_MANIFEST_DIR/target/release/$PROJECT_BINARY_NAME" ~
