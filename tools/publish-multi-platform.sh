#!/bin/bash

set -e -u -o pipefail

function map_target {
  case $1 in
    "aarch64-unknown-linux-gnu")
      echo "arm64"
      ;;
    "x86_64-unknown-linux-gnu")
      echo "amd64"
      ;;
    *)
      echo "Unknown argument: $1"
      return 1
      ;;
  esac
}

MANIFEST_PATH=${MANIFEST_PATH:-"$APP_NAME/Cargo.toml"}
VERSION=${VERSION:-"$(git rev-parse HEAD)"}
DRYRUN=${DRYRUN:-"0"}

build_context="/tmp/$(date +%s)"
mkdir -p "$build_context"
cp "$DOCKERFILE" "$build_context/Dockerfile"
manifest_amendments=""
image_name="ghcr.io/th0masb/myopic/$APP_NAME"

for target in $TARGETS; do
  mapped_target=$(map_target "$target")
  echo "Building container for $mapped_target"
  cross build --release --target="$target" --manifest-path="$MANIFEST_PATH"
  cp "target/$target/release/$APP_NAME" "$build_context/app"
  image_tag="$image_name:$VERSION-$mapped_target"
  push=""
  if [ "$DRYRUN" != "1" ]; then
    push="--push"
  fi
  docker buildx build \
    $push \
    --platform "linux/$mapped_target" \
    -t "$image_tag" \
    "$build_context"
  manifest_amendments="$manifest_amendments --amend $image_tag"
done

if [ "$DRYRUN" != "1" ]; then
  for version in "$VERSION" "latest"; do
    # shellcheck disable=SC2086
    docker manifest create "$image_name:$version" $manifest_amendments
    docker manifest push "$image_name:$version"
  done
fi
