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
	docker_platform="linux/$(map_target "$target")"
	echo "Building container for $docker_platform"
	cross build --release --target=$target
	cp "target/$target/release/$APPLICATION_DIR" "$build_context/app"
	docker buildx build \
		--platform "$docker_platform" \
		-t "th0masb/multi-platform-test:$VERSION-$target" \
		"$build_context"
done

