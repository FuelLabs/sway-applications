#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'escrow' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
fi
