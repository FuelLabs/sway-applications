#!/bin/bash

forc build
./node_modules/.bin/typechain --target=fuels --out-dir=src/todo-contract-types out/debug/todo-contract-abi.json
