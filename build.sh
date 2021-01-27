#!/bin/bash
# this is not how docker is supposed to be used?  :D (also, this wasy copy-paste)
npm install &&
npm run build:prod &&
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest cargo build --release
