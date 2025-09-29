#!/bin/sh

set -e -x

bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css -m

dx bundle --release --platform web "$@"
