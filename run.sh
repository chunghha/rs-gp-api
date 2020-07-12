#!/usr/bin/env bash

if [ "$1" = "rebuild" ]; then
    docker-compose build
fi

docker-compose up -d rs-gp-api