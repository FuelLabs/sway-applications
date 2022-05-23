#!/usr/bin/env bash

PROJECT=$1

if [ $PROJECT = 'escrow' ]; then
	cd $PROJECT
    forc build
    forc build --path tests/artifacts/asset
fi
