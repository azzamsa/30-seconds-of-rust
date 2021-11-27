#!/usr/bin/env bash

# inspired from git-cliff
# takes the tag as an argument (e.g. v0.1.0)
if [ -n "$1" ]; then
	# update the version
	sed "s/^version = .* $msg$/version = \"${1#v}\" $msg/" -i Cargo.toml
	# update the changelog
	git-cliff --tag "$1" > CHANGELOG.md
	git add -A && git commit -m "$1"
	git show
        git tag -s -a "$1" -m "$1" -m "For details, see the CHANGELOG.md"
	git tag -v "$1"
else
	echo "warn: Please provide a tag"
fi
