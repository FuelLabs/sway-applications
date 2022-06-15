#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'auctions/dutch-auction' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'escrow' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc build --path $PROJECT
fi
