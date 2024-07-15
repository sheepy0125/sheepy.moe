#!/bin/sh

# constants for deploy

SSH_KEY="-i ~/.ssh/sheepy.moe" # as in `ssh ${SSH_KEY}`
TARGET="sheepy@cinderblock" # as in `ssh ${SSH_TARGET}`
TARGET_DIR="/server/sheepy.moe/" # cwd is ~
UPLOAD="target/release/server dist blog static .env"

SSH="$(which ssh) ${SSH_KEY}"
SCP="$(which scp) -r ${SSH_KEY}"

fail() {
    echo "failed at ${1}">&2
    exit 1
}
