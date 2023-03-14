#!/bin/bash

SCRIPTS_PATH=$( cd $(dirname $BASH_SOURCE[0]) && pwd )
CHAIN_CONFIG_PATH=$(dirname $SCRIPTS_PATH)/config/chainConfig.json
PROJECT_PATH=$(dirname $(dirname $SCRIPTS_PATH))/project

cd $PROJECT_PATH

fuel-core run --ip 127.0.0.1 --port 4000 --chain $CHAIN_CONFIG_PATH --db-type in-memory
