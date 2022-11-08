#!/usr/bin/env bash

PROJECT=$1
COMMAND=$2
FLAG=$3

if [ $PROJECT = 'airdrop/airdrop-distributor' ]; then
    forc $COMMAND --path $PROJECT $FLAG
    forc $COMMAND --path $PROJECT/../simple-asset/ $FLAG
elif [ $PROJECT = 'airdrop/simple-asset' ]; then
    forc $COMMAND --path $PROJECT $FLAG
elif [ $PROJECT = 'auctions/english-auction' ]; then
    forc $COMMAND --path $PROJECT $FLAG
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAG
    forc $COMMAND --path $PROJECT/../../NFT/ $FLAG
elif [ $PROJECT = 'dao-voting' ]; then
    forc $COMMAND --path $PROJECT $FLAG
    forc $COMMAND --path $PROJECT/tests/artifacts/gov_token $FLAG
elif [ $PROJECT = 'escrow' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAG
elif [ $PROJECT = 'fundraiser' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
    forc $COMMAND --path $PROJECT/tests/artifacts/asset $FLAG
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc $COMMAND --path $PROJECT $FLAGS
elif [ $PROJECT = 'name-registry' ]; then
    forc $COMMAND --path $PROJECT $FLAG
elif [ $PROJECT = 'NFT' ]; then
    forc $COMMAND --path $PROJECT $FLAG
elif [ $PROJECT = 'oracle' ]; then
    forc $COMMAND --path $PROJECT/packages/contract $FLAG
else
    echo "project name did not match"
    exit 1
fi
