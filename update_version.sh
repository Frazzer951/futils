#!/bin/bash

# Check if a version was provided as an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

version=$1

# Set the cargo version
cargo-set-version set-version $version

# Update the CHANGELOG.md with git cliff
git cliff -o CHANGELOG.md --tag $version
