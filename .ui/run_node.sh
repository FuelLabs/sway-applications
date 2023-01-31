#!/bin/bash

NO_COLOR=`tput sgr0`
YELLOW=`tput setaf 3`

SCRIPT_ROOT=$(dirname $(realpath $0))

cd $SCRIPT_ROOT

echo -e "${YELLOW}Starting local Fuel node${NO_COLOR}"

fuel-core run --ip 127.0.0.1 --port 4000 --chain ./chainConfig.json --db-type in-memory
