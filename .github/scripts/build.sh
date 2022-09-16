#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'airdrop/airdrop-distributor' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/../simple-asset/
elif [ $PROJECT = 'airdrop/simple-asset' ]; then
    forc build --path $PROJECT
elif [ $PROJECT = 'dao-voting' ]; then
    forc build --path $PROJECT
    forc build --path $PROJECT/tests/artifacts/gov_token
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
elif [ $PROJECT = 'oracle']; then
    forc build --path $PROJECT
elif [ $PROJECT = 'staking-rewards' ]; then
    forc build --path $PROJECT
fi
