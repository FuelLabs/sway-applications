#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'dao-voting' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/gov_token
elif [ $PROJECT = 'escrow' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'fundraiser' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'games/TicTacToe' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc build --path $PROJECT
fi
