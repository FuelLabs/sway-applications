#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'airdrop/airdrop-distributor' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/../simple-asset/
elif [ $PROJECT = 'airdrop/simple-asset' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'auctions/english-auction' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
    forc build --path $PROJECT/../../NFT/
elif [ $PROJECT = 'dao-voting/packages/dao' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/gov_token
elif [ $PROJECT = 'dao-voting/packages/governor_abi' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'dao-voting/packages/governor_contract' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'escrow' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'fundraiser' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/asset
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'NFT' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'oracle' ]; then
    forc build --path $PROJECT
else
    echo "project name did not match"
    exit 1
fi
