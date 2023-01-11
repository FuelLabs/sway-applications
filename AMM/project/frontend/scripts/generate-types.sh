#!/bin/bash

FRONTEND_ROOT=$(dirname $(dirname $(realpath $0)))

cd $FRONTEND_ROOT
npx fuels typegen -i ../contracts/AMM-contract/out/debug/*-abi.json -o ./src/types/contracts
npx fuels typegen -i ../contracts/exchange-contract/out/debug/*-abi.json -o ./src/types/contracts
