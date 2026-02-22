#!/bin/sh

set -e -x

bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css -m

dx build --web --release --debug-symbols=false "$@"
