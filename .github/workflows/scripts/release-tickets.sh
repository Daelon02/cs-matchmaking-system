#!/bin/sh

# Don't forget to add required permission to the Git Index
# git update-index --chmod=+x .github/workflows/scripts/release-tickets.sh

# Parse the command line arguments
dry_run=false

while [[ "$#" -gt 0 ]]; do
    case $1 in
        --release) release_type="release"; shift ;;
        --candidate) release_type="candidate"; shift ;;
        --dry-run) dry_run=true; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
done

# Get the previous release tag
if [ "$release_type" == "release" ]; then
    previous=$(git tag --sort=-creatordate | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | sed -n '2p')
    current=$(git tag --sort=-creatordate | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | sed -n '1p')
elif [ "$release_type" == "candidate" ]; then
    previous=$(git tag --sort=-creatordate | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+)?$' | sed -n '2p')
    current=$(git tag --sort=-creatordate | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+-rc\.[0-9]+$' | sed -n '1p')
else
    echo "Please specify either --release or --candidate"
    exit 1
fi

if [ "$dry_run" = true ]; then
    echo "Dry run mode"
fi

echo "Changes between $previous and $current"