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

# Update README.md
COMMAND="cargo run -r -- --help"

# Run the command and capture the output
OUTPUT=$($COMMAND)

# Path to the README.md file
README_PATH="./README.md"

# Temporary file for storing new README content
TEMP_README="./README.md.tmp"

# Process the README file
{
    PRINT=true
    while IFS= read -r line || [[ -n "$line" ]]; do
        if [[ $line == "<!-- Usage -->" ]]; then
            echo "$line"
            echo '```bash'
            echo "$OUTPUT"
            PRINT=false
        elif [[ $line == "<!-- Usage End -->" ]]; then
            echo '```'
            PRINT=true
        fi
        $PRINT && echo "$line"
    done < "$README_PATH"
} > "$TEMP_README"

# Move the temporary file to the original README path
mv "$TEMP_README" "$README_PATH"