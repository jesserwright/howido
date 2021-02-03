#!/bin/bash
docker run --name howido-db -e POSTGRES_PASSWORD=pass -p 5432:5432 -v $HOME/docker/volumes/postgres:/var/lib/postgresql/data -d postgres:alpine
