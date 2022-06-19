#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'escrow' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'fundraiser' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc build --path $PROJECT
fi
