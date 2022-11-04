#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'airdrop/airdrop-distributor' ]; then
    forc fmt --path $PROJECT --check
    forc fmt --path $PROJECT/../simple-asset/ --check
elif [ $PROJECT = 'airdrop/simple-asset' ]; then
    forc fmt --path $PROJECT --check
elif [ $PROJECT = 'auctions/english-auction' ]; then
    forc fmt --path $PROJECT --check
    forc fmt --path $PROJECT/tests/artifacts/asset --check
    forc fnt --path $PROJECT/../../NFT/ --check
elif [ $PROJECT = 'dao-voting' ]; then
    forc fmt --path $PROJECT --check
    forc fmt --path $PROJECT/tests/artifacts/gov_token --check
elif [ $PROJECT = 'escrow' ]; then
    forc fmt --path $PROJECT --check
    forc fmt --path $PROJECT/tests/artifacts/asset --check
elif [ $PROJECT = 'fundraiser' ]; then
    forc fmt --path $PROJECT --check
    forc fmt --path $PROJECT/tests/artifacts/asset --check
elif [ $PROJECT = 'multisig-wallet' ]; then
    forc fmt --path $PROJECT --check
elif [ $PROJECT = 'NFT' ]; then
    forc fmt --path $PROJECT --check
elif [ $PROJECT = 'oracle/packages' ]; then
    forc fmt --path $PROJECT/contract --check
else
    echo "project name did not match"
    exit 1
fi
