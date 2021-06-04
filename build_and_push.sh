#!/bin/bash

set -euo pipefail

NAME=compute-waster
TAG=latest
REPOSITORY=

if [ -f .env ]; then
    source .env
fi

if [ -z "$REPOSITORY" ]; then
    echo "Need to set the image repository"
    exit 1
fi

echo -e "\n===== building image as $NAME:$TAG =====\n"
docker build --tag $NAME:$TAG .
echo -e "\n===== tagging $NAME:$TAG image as $REPOSITORY/$NAME:$TAG =====\n"
docker image tag $NAME:$TAG $REPOSITORY/$NAME:$TAG
echo -e "\n===== pushing image as $REPOSITORY/$NAME:$TAG =====\n"
docker image push $REPOSITORY/$NAME:$TAG
