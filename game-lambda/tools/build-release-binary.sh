# Arg 1 => REPO_ADDRESS
# Arg 2 => REPO_BRANCH
# Arg 3 => PROJECT_MANIFEST_DIR
# Arg 4 => PROJECT_BINARY_NAME
# Arg 5 => Docker image name
function build_al2_runtime_lambda {
  # Ensure the target directory exists
  TARGET_DIR="$(pwd)/$(dirname "$0")/../target/amazonlinux2"
  mkdir -p "$TARGET_DIR"

  # Build the release binary within the al2 container forwarding
  # the parameters to the internal implementation script via
  # environment variables
  docker run \
    --env REPO_ADDRESS="$1" \
    --env REPO_BRANCH="$2" \
    --env PROJECT_MANIFEST_DIR="$3" \
    --env PROJECT_BINARY_NAME="$4" \
    "$5" /bin/bash -c "/root/build-release-binary-internal.sh"

  # Lift the compiled binary out of the container
  docker cp "$(docker ps -alq):/root/$4" "$TARGET_DIR"
}