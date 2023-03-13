#!/usr/bin/env bash

<<'EG'
./cargo_publish.sh
EG

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

cd ${script_path_root}oauth2-doorkeeper
cargo publish -v
sleep 2

find ${script_path_root}* -maxdepth 1 -prune -type d ! -name "oauth2-doorkeeper" -print0 | xargs -0 -I '{}' sh -c \
    "cd ${script_path_root}oauth2-doorkeeper{}; cargo publish -v; sleep 2"
