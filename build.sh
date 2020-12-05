#!/bin/bash
npm install &&
npm run build:prod &&
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.48.0 cargo build --release
