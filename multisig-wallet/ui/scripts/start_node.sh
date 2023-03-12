#!/bin/bash

cd ../../project
fuel-core run --ip 127.0.0.1 --port 4000 --chain ../ui/config/chainConfig.json --db-type in-memory
