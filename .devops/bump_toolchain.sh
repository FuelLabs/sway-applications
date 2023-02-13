#!/bin/bash

APPS=("AMM" "DAO" "NFT" "OTC-swap-predicate" "airdrop" "auctions" "escrow" "fractional-NFT" "fundraiser" "games/TicTacToe" "multisig-wallet" "name-registry" "oracle" "timelock")
errors=()
success=()

# This can probably be cleaner
REPO_ROOT=$(dirname $(dirname $(realpath $0)))
TOOLCHAIN="fuel-toolchain.toml"
TMP_TOOLCHAIN="fuel-toolchain2.toml"

cd $REPO_ROOT

for app in "${APPS[@]}"
do
    echo Building $app
    mv $app/$TOOLCHAIN $app/$TMP_TOOLCHAIN
    cp .devops/$TOOLCHAIN $app/$TOOLCHAIN
    cd $app
    forc build
    
    # Check if there was an error and report the app at the end
    status=$?
    if [ $status -ne 0 ]; then
        errors+=("${app}")
        mv $TMP_TOOLCHAIN $TOOLCHAIN
    else
	success+=("${app}")
	rm $TMP_TOOLCHAIN
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


if [ ${#success[@]} -ne 0 ]; then
    echo "Bumped"
    for app in "${success[@]}"
    do
        echo "  " $app
    done
fi
