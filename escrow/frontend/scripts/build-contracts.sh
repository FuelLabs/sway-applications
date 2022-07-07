#!/bin/sh

ROOT_DIR=$(realpath ../)
ESCROW_CONTRACT=$ROOT_DIR/contracts/escrow

echo $ESCROW_CONTRACT

echo "Build Fuel Escrow contract"
forc build -p $ESCROW_CONTRACT
echo "Build Types for contracts"
pnpm exec typechain --target fuels --out-dir=./src/systems/Core/types/contracts '../contracts/**/out/debug/**.json'
echo "Prettify code"
pnpm exec prettier --write src/systems/Core/types
