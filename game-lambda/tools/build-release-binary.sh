# Arg 1 => Output directory for the compiled binary
# Arg 2 => REPO_ADDRESS
# Arg 3 => REPO_BRANCH
# Arg 4 => PROJECT_MANIFEST_DIR
# Arg 5 => PROJECT_BINARY_NAME
# Arg 6 => Docker image name
function build_al2_runtime_lambda {
  # Make sure the output directory exists
  mkdir -p "$1"

  # Build the release binary within the al2 container forwarding
  # the parameters to the internal implementation script via
  # environment variables
  docker run \
    --env REPO_ADDRESS="$2" \
    --env REPO_BRANCH="$3" \
    --env PROJECT_MANIFEST_DIR="$4" \
    --env PROJECT_BINARY_NAME="$5" \
    "$6" /bin/bash -c "/root/build-release-binary-internal.sh"

  # Lift the compiled binary out of the container
  docker cp "$(docker ps -alq):/root/$5" "$1"
}