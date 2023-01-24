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

declare -a contract_names
declare -a contract_ids

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
        PACKAGES_ROOT=$PROJECT_ROOT/packages
        UI_ROOT=$PACKAGES_ROOT/app

        # env_test=$PROJECT_ROOT/.env.test
        # [ ! -f $env_test ] || { echo "Clearing the contents of $env_test"; > $env_test; }
        
        json=$PROJECT_ROOT/vars.json # replace with env.test

        pattern="\"owner\":[[:space:]]\"([a-zA-Z0-9]*)\""
        [[ $(grep owner $SCRIPT_PATH/chainConfig.json) =~ $pattern ]] && wallet="${BASH_REMATCH[1]}"

        # printf "%b" "WALLET_SECRET=\"${wallet}\"\n" >> $env_test

        printf "%b" "{\n\t\"walletSecret\": \"${wallet}\"" >> $json
    fi

    contract_name=${contract_path##*\/}
    CONTRACT_NAMES+=($contract_name)

    uppercase_underscore_contract_name=$(tr '[:lower:]' '[:upper:]' <<< ${contract_name//-/_})

    abi_path="${contract_path}/out/debug/${contract_name}-abi.json"
    [ -f $abi_path ] || { echo "File does not exist $abi_path"; exit; }
    ABI_PATHS+=($abi_path)

    cd $contract_path
    pattern="Contract id:[[:space:]](.*)[[:space:]]contract.*"
    [[ $(forc deploy --url localhost:4000 --unsigned) =~ $pattern ]] && contract_id="${BASH_REMATCH[1]}"

    # printf "%b" "${uppercase_underscore_contract_name}_ID=\"${contract_id}\"\n" >> $env_test

    json_contract_name="${contract_name//-/}"
    json_contract_name="${json_contract_name//contract/Contract}"
    contract_names+=($json_contract_name)
    contract_ids+=($contract_id)
done

printf "%b" ",\n\t\"contractNames\": [" >> $json
for contract_index in "${!contract_names[@]}"
do
    if [[ $contract_index != 0 ]]; then
        printf "%b" ", " >> $json
    fi
    printf "%b" "\"${contract_names[$contract_index]}\"" >> $json
done
printf "%b" "],\n" >> $json

printf "%b" "\t\"contractIds\": [" >> $json
for contract_index in "${!contract_ids[@]}"
do
    if [[ $contract_index != 0 ]]; then
        printf "%b" ", " >> $json
    fi
    printf "%b" "\"${contract_ids[$contract_index]}\"" >> $json
done
printf "%b" "]\n}" >> $json

if [[ -d $PACKAGES_ROOT ]]; then
    read -e -p "The directory ${MAGENTA}packages${NO_COLOR} already exists. Remove to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    read -e -p "${RED}THIS IS A DESTRUCTIVE OPERATION.${NO_COLOR} Would you still like to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    echo -e "${YELLOW}Removing ${MAGENTA}$PACKAGES_ROOT ${YELLOW}and its contents${NO_COLOR}"
    rm -rf $PACKAGES_ROOT
fi

mkdir $PACKAGES_ROOT
cd $PACKAGES_ROOT

npx create-react-app app --template typescript

npm install fuels --save

echo -e "\n${YELLOW}Generating types for ${PURPLE}${ABI_PATHS[@]}${NO_COLOR}\n" 
CONTRACT_TYPES_PATH="${CONTRACT_TYPES_ROOT}/$UI_ROOT/src/contracts"

npx fuels typegen -i ${ABI_PATHS[@]} -o $CONTRACT_TYPES_PATH

cd $UI_ROOT

rm -v public/favicon.ico
rm -v public/logo192.png
rm -v public/logo512.png
rm -v public/manifest.json
rm -v public/robots.txt
rm -v src/App.test.tsx
rm -v src/logo.svg
rm -v src/react-app-env.d.ts
rm -v src/reportWebVitals.ts
rm -v src/setupTests.ts
> public/index.html
> src/App.css
> src/App.tsx
> src/index.tsx
cat $SCRIPT_PATH/basic/index.html > public/index.html
cat $SCRIPT_PATH/basic/App.css > src/App.css
cat $SCRIPT_PATH/basic/App.tsx | tail -n +2 > src/App.tsx
cat $SCRIPT_PATH/basic/index.tsx | tail -n +2 > src/index.tsx
cat $SCRIPT_PATH/basic/Main.tsx | tail -n +2 > src/Main.tsx
mv $json src/vars.json
printf "%b" "/node_modules\n" >> $PACKAGES_ROOT/.gitignore

cp $SCRIPT_PATH/basic/Components.tsx src/Components.tsx

npm start
