#!/usr/bin/env zsh
codesign -s - "$1"
exec "$@"