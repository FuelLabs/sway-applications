#!/bin/bash

# Check if arguments are provided
if [ $# -eq 0 ]; then
	echo "No arguments provided"
	exit 1
fi

args=("${@:2}")

# Set cache directory from environment variable or default to .turbo
CACHE_DIR=${TURBO_CACHE_DIR:-.turbo}

# Run pnpm turbo with the same arguments and cache directory
pnpm turbo run "$1" --cache-dir="$CACHE_DIR" "${args[@]}"