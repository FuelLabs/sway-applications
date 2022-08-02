#!/bin/sh

ENV_FILE=.env
if [ ! -f "$FILE" ]; then
    cp .env.example $ENV_FILE
fi
