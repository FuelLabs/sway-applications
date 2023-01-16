#!/bin/bash

MAGENTA=`tput setaf 5`
NO_COLOR=`tput sgr0`
RED=`tput setaf 1`
YELLOW=`tput setaf 3`

declare -a LINES
declare -a CONTRACT_NAMES
declare -a ABI_PATHS

SCRIPT_PATH=$(dirname $(realpath $0))
REPO_ROOT=$(dirname $SCRIPT_PATH)

while IFS= read -e -p "Enter contract project path: ${MAGENTA}sway-applications/${NO_COLOR}" line
do 
    [ $line ] || break
    LINES+=("$line")
done

[ ${#LINES[@]} != 1 ] || { echo "You must provide at least one Sway contract path"; exit; }

for path_index in "${!LINES[@]}"
do
    [ $path_index != 0 ] || continue

    contract_path=$REPO_ROOT/${LINES[$path_index]}

    if [[ $contract_path == *"/" ]]; then
        contract_path=${contract_path%?}
    fi

    [ -d "$contract_path/" ] || { echo "Directory does not exist $contract_path"; exit; }

    if [[ $path_index == 1 ]]; then
        PROJECT_ROOT=$(dirname ${contract_path})
        if [[ $PROJECT_ROOT != *"/project" ]]; then
            PROJECT_ROOT=$(dirname $PROJECT_ROOT)
        fi
        UI_ROOT=$PROJECT_ROOT/ui
        env_test=$PROJECT_ROOT/.env.test
        [ ! -f $env_test ] || { echo "Clearing the contents of $env_test"; > $env_test; }
        
        pattern="\"owner\":[[:space:]]\"([a-zA-Z0-9]*)\""
        [[ $(grep owner $SCRIPT_PATH/chainConfig.json) =~ $pattern ]] && wallet="${BASH_REMATCH[1]}"
        printf "%b" "WALLET_SECRET=${wallet}\n" >> $env_test
    fi

    contract_name=${contract_path##*\/}
    uppercase_contract_name=$(tr '[:lower:]' '[:upper:]' <<< ${contract_name//-/_})_ID

    abi_path="${contract_path}/out/debug/${contract_name}-abi.json"
    [ -f $abi_path ] || { echo "File does not exist $abi_path"; exit; }
    ABI_PATHS+=($abi_path)

    cd $contract_path
    pattern="Contract id:[[:space:]](.*)[[:space:]]contract.*"
    [[ $(forc deploy --url localhost:4000 --unsigned) =~ $pattern ]] && contract_id="${BASH_REMATCH[1]}"

    printf "%b" "${uppercase_contract_name}=${contract_id}\n" >> $env_test
done

if [[ -d $UI_ROOT ]]; then
    read -e -p "The directory ${MAGENTA}ui${NO_COLOR} already exists. Remove to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    read -e -p "${RED}THIS IS A DESTRUCTIVE OPERATION.${NO_COLOR} Would you still like to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    echo -e "${YELLOW}Removing ${MAGENTA}$UI_ROOT ${YELLOW}and its contents${NO_COLOR}"
    rm -rf $UI_ROOT
fi

cd $PROJECT_ROOT

npx create-react-app ui --template typescript

npm install fuels --save

CONTRACT_TYPES_ROOT="$UI_ROOT/src/contracts"

for abi_index in "${!ABI_PATHS[@]}"
do
    abi_path=${ABI_PATHS[$abi_index]}
    echo -e "\n${YELLOW}Generating types for ${PURPLE}$abi_path${NO_COLOR}\n" 

    if [[ ${#ABI_PATHS[@]} > 1 ]]; then
        CONTRACT_TYPES_PATH="${CONTRACT_TYPES_ROOT}/${CONTRACT_NAMES[$abi_index]}"
    fi

    npx fuels typegen -i $abi_path -o $CONTRACT_TYPES_PATH
done
