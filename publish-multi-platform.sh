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
for target in $TARGETS; do
	mapped_target=$(map_target "$target")
	#docker_platform="linux/$(map_target "$target")"
	echo "Building container for $mapped_target"
	cross build --release --target=$target
	cp "target/$target/release/$APPLICATION_DIR" "$build_context/app"
	docker buildx build \
		--push \
		--platform "linux/$mapped_target" \
		-t "ghcr.io/th0masb/myopic/test:$VERSION-$mapped_target" \
		"$build_context"
done

