#!/bin/bash

APPS=("AMM" "DAO" "NFT" "OTC-swap-predicate" "airdrop" "auctions" "escrow" "fractional-NFT" "fundraiser" "games/TicTacToe" "multisig-wallet" "name-registry" "oracle" "timelock")
errors=()

# This can probably be cleaner
REPO_ROOT=$(dirname $(dirname $(realpath $0)))

cd $REPO_ROOT

for app in "${APPS[@]}"
do
    echo Building $app
    cd $app
    forc build
    
    # Check if there was an error and report the app at the end
    status=$?
    if [ $status -ne 0 ]; then
        errors+=("${app}")
    fi

    cd $REPO_ROOT
    echo
done

if [ ${#errors[@]} -ne 0 ]; then
    echo "Errors found in"
    for app in "${errors[@]}"
    do
        echo "  " $app
    done
fi
