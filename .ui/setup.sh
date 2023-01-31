#!/bin/bash

SCRIPT_PATH=$(dirname $(realpath $0)) # path: sway-applications/.ui
REPO_ROOT=$(dirname $SCRIPT_PATH) # path: sway-applications

# colors used for some messages written to standard output
MAGENTA=`tput setaf 5` # used for paths
NO_COLOR=`tput sgr0` # used to reset to default color
RED=`tput setaf 1` # used for destructive operations
YELLOW=`tput setaf 3` # used for purely informative messages

# declare arrays
declare -a contract_project_paths # array of contract project paths relative to REPO_ROOT, e.g. AMM/project/contracts/exchange-contract
declare -a abi_paths # array of real paths of generated ABIs, e.g. sway-applications/AMM/project/contracts/exchange-contract/out/exchange-contract-abi.json

# read standard input to get contract projects
# TODO: change to expect abi.json paths instead
while IFS= read -e -p "Enter contract project path: ${MAGENTA}sway-applications/${NO_COLOR}" input_path
do 
    [ $input_path ] || break
    contract_project_paths+=("$input_path")
done

# check that at least one path has been inputted
[ ${#contract_project_paths[@]} != 0 ] || { echo "You must provide at least one Sway contract project path"; exit; }

# iterate all provided contract project paths
for path_index in "${!contract_project_paths[@]}"
do
    # real path of contract project
    contract_path=$REPO_ROOT/${contract_project_paths[$path_index]}

    # remove trailing slash if it exists, to avoid duplicate slashes and to be able to extract contract name
    if [[ $contract_path == *"/" ]]; then
        contract_path=${contract_path%?}
    fi

    # check that directory exists
    [ -d "$contract_path" ] || { echo "Directory does not exist $contract_path"; exit; }

    # contract name is the part of contract path that is before the last "/", e.g. exchange-contract
    contract_name=${contract_path##*\/}

    # real path of generated abi file, e.g. sway-applications/AMM/project/contracts/exchange-contract/out/debug/exchange-contract-abi.json 
    abi_path="${contract_path}/out/debug/${contract_name}-abi.json"
    [ -f $abi_path ] || { echo "File does not exist $abi_path"; exit; }
    abi_paths+=($abi_path)

    cd $contract_path
    pattern="Contract id:[[:space:]](.*)[[:space:]]contract.*"
    # deploy contract and extract the contract id from standard output
    [[ $(forc deploy --url localhost:4000 --unsigned) =~ $pattern ]] && contract_id="${BASH_REMATCH[1]}"

    # operations for the first iteration
    # assumes that the application is structured as app/project/contracts/<contract> or app/project/<contract>
    if [[ $path_index == 0 ]]; then
        # real path of project root, e.g. sway-applications/AMM/project or sway-applications/auctions/english-auction/project
        project_path=$(dirname ${contract_path})
        if [[ $project_path != *"/project" ]]; then
            project_path=$(dirname $project_path)
        fi

        # real path of packages directory that will be created, e.g. sway-applications/AMM/project/packages
        packages_path=$project_path/packages
        # real path of the frontend application that will be created, e.g. sway-applications/AMM/project/packages/app
        ui_path=$packages_path/app

        # file that will contain deployed contract ids
        # TODO: replace with env.test with wallet private keys
        json=$project_path/vars.json

        # write the private key of the first wallet in .ui/chainConfig.json to vars.json
        # TODO: remove once environment variables and the wallet SDK are integrated
        printf "%b" "{\n\t\"walletSecret\": \"0xa449b1ffee0e2205fa924c6740cc48b3b473aa28587df6dab12abc245d1f5298\"," >> $json
        printf "%b" "\n\t\"contractIds\": [" >> $json
    fi

    # write the deployed contract ids to vars.json
    printf "%b" "\"${contract_id}\"" >> $json
    if [[ $path_index != $((${#contract_project_paths[@]} - 1)) ]]; then
        printf "%b" ", " >> $json
    else
        printf "%b" "]\n}" >> $json
    fi
done

# check if the packages directory already exists and remove it after confirming twice, in order to avoid collisions
if [[ -d $packages_path ]]; then
    read -e -p "The directory ${MAGENTA}$packages_path${NO_COLOR} already exists. Remove to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    read -e -p "${RED}THIS IS A DESTRUCTIVE OPERATION.${NO_COLOR} Would you still like to continue? (y/N) " response
    ([ $response ] && [ $response == "y" ]) || exit
    echo -e "${YELLOW}Removing ${MAGENTA}$packages_path${YELLOW} and its contents${NO_COLOR}"
    rm -rf $packages_path
fi

mkdir $packages_path
cd $packages_path

# create react app with the typescript template
npx create-react-app app --template typescript

# install fuels
npm install fuels --save

# generate types for contracts
echo -e "\n${YELLOW}Generating types for ${PURPLE}${abi_paths[@]}${NO_COLOR}\n" 
npx fuels typegen -i ${abi_paths[@]} -o $ui_path/src/contracts

cd $ui_path

# remove unnecessary files
rm public/favicon.ico
rm public/logo192.png
rm public/logo512.png
rm public/manifest.json
rm public/robots.txt
rm src/App.test.tsx
rm src/logo.svg
rm src/react-app-env.d.ts
rm src/reportWebVitals.ts
rm src/setupTests.ts

# replace contents of some files in the src directory
> public/index.html
> src/App.css
> src/App.tsx
> src/index.tsx
cat $SCRIPT_PATH/src/index.html > public/index.html
cat $SCRIPT_PATH/src/App.css > src/App.css
cat $SCRIPT_PATH/src/App.tsx | tail -n +2 > src/App.tsx
cat $SCRIPT_PATH/src/index.tsx | tail -n +2 > src/index.tsx

# add missing files to the src directory
cat $SCRIPT_PATH/src/Components.tsx | tail -n +2 > src/Components.tsx
cat $SCRIPT_PATH/src/Main.tsx | tail -n +2 > src/Main.tsx
mv $json src/vars.json

# you can run `npm start` in $ui_path, e.g. AMM/project/packages/app
# or uncomment the next line before running this script. it will clear the previous standard outputs by this script
# npm start
