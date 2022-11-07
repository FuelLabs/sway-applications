#!/usr/bin/env bash

PROJECT=$1
COMMAND=$2
FLAGS=${@:3}

if [ $PROJECT = 'airdrop/airdrop-distributor' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/../simple-asset/ $FLAGS
elif [ $PROJECT = 'airdrop/simple-asset' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
elif [ $PROJECT = 'auctions/english-auction' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAGS
    forc $COMMAND --path $PROJECT/../../NFT/ $FLAGS
elif [ $PROJECT = 'dao-voting' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/gov_token $FLAGS
elif [ $PROJECT = 'escrow' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAGS
elif [ $PROJECT = 'fundraiser' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAGS
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
elif [ $PROJECT = 'name-registry' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
elif [ $PROJECT = 'NFT' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
elif [ $PROJECT = 'oracle' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
else
    echo "project name did not match"
    exit 1
fi
