all: build

build:
    cargo build

run:
    cargo run -- -v --user

call:
    busctl --user call --json=pretty -- org.betterkit /org/betterkit/betterkit1 org.betterkit.betterkit1 Run as 2 "ls" "-alh" | jq -r ".data[0]"

get:
    busctl --user call --json=pretty -- org.betterkit /org/betterkit/betterkit1 org.betterkit.betterkit1 Get t 6 | jq -r ".data[0]"
