#!/bin/bash

set -e

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

start_dir="$(pwd)"
build_context="/tmp/$(date +%s)"
mkdir -p "$build_context"
cp "$DOCKERFILE" "$build_context"
cd $APPLICATION_DIR
manifest_amendments=""
image_base_tag="ghcr.io/th0masb/myopic/test:$VERSION"
for target in $TARGETS; do
	mapped_target=$(map_target "$target")
	#docker_platform="linux/$(map_target "$target")"
	echo "Building container for $mapped_target"
	cross build --release --target=$target
	cp "target/$target/release/$APPLICATION_DIR" "$build_context/app"
	image_tag="$image_base_tag-$mapped_target"
	docker buildx build \
		--push \
		--platform "linux/$mapped_target" \
		-t "$image_tag" \
		"$build_context"
	manifest_amendments="$manifest_amendments --amend $image_tag"
done
docker manifest create "$image_base_tag" $manifest_amendments
docker manifest push "$image_base_tag"

