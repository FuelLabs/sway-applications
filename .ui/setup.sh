#!/bin/bash

MAGENTA=`tput setaf 5`
NO_COLOR=`tput sgr0`
RED=`tput setaf 1`
YELLOW=`tput setaf 3`

declare -a LINES

while IFS= read -e -p "Enter contract project path: ${MAGENTA}sway-applications/${NO_COLOR}" line
do 
    [ $line ] || break
    LINES+=("$line")
done

[ ${#LINES[@]} != 1 ] || { echo "You must provide at least one Sway contract path"; exit; }

REPO_ROOT=$(dirname $(dirname $(realpath $0)))
declare -a CONTRACT_NAMES
declare -a CONTRACT_PATHS
declare -a ABI_PATHS

for path_index in "${!LINES[@]}"
do
    [ $path_index != 0 ] || continue

    contract_path="$REPO_ROOT/${LINES[$path_index]}"

    if [[ $contract_path == *"/" ]]; then
        contract_path=${contract_path%?}
    fi

    [ -d "$contract_path/" ] || { echo "Directory does not exist $contract_path"; exit; }
    CONTRACT_PATHS+=($contract_path)    

    contract_name=${contract_path##*\/}
    CONTRACT_NAMES+=($contract_name)

    abi_path="${contract_path}/out/debug/${contract_name}-abi.json"
    [ -f $abi_path ] || { echo "File does not exist $abi_path"; exit; }
    ABI_PATHS+=($abi_path)
done

PROJECT_ROOT=$(dirname ${CONTRACT_PATHS[0]})

if [[ $PROJECT_ROOT != *"/project" ]]; then
    PROJECT_ROOT=$(dirname $PROJECT_ROOT)
fi

UI_ROOT=$PROJECT_ROOT/ui/

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

for abi_index in "${!ABI_PATHS[@]}"
do
    abi_path=${ABI_PATHS[$abi_index]}
    echo -e "\n${YELLOW}Generating types for ${PURPLE}$abi_path${NO_COLOR}\n" 

    contract_types_path="$PROJECT_ROOT/ui/src/contracts"

    if [[ ${#ABI_PATHS[@]} > 1 ]]; then
        contract_types_path="${contract_types_path}/${CONTRACT_NAMES[$abi_index]}"
    fi

    npx fuels typegen -i $abi_path -o $contract_types_path
done
