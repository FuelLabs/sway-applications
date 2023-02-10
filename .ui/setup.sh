#!/bin/bash

APP_PATH=$(pwd)
SCRIPT_PATH=$(realpath $0)
SCAFFOLD_ROOT=$(dirname $SCRIPT_PATH)
COMMON_PATH=$SCAFFOLD_ROOT/common
UI=ui
UI_PATH=$APP_PATH/$UI
declare -a ABIS

# colors used for some messages written to standard output
NO_COLOR=`tput sgr0` # used to reset to default color
ERROR_COLOR=`tput setaf 1` # red
ADDITION_COLOR=`tput setaf 2` # green
INFORMATIVE_COLOR=`tput setaf 3` # yellow
PATH_COLOR=`tput setaf 4` # blue

# assumes that all contract projects are under the project directory, either directly or in a subdirectory
# TODO: can be simplified once https://github.com/FuelLabs/sway-applications/pull/409 is merged
function find_abis() {
    cd $APP_PATH
    PROJECT_PATH=$APP_PATH/project
    
    [ -d $APP_PATH/project ] || { echo "${ERROR_COLOR}Directory not found ${PATH_COLOR}$PROJECT_PATH${NO_COLOR}"; exit; }

    CONTRACTS=("$PROJECT_PATH/**/*-contract" "$PROJECT_PATH/*-contract")
    
    for contract in ${CONTRACTS[@]}; do
        contract_name=$(basename ${contract})
        abi=$contract/out/debug/$contract_name-abi.json
        [ -f $abi ] || continue
        ABIS+=($abi)
        echo -e "${INFORMATIVE_COLOR}Found ABI ${PATH_COLOR}$abi${NO_COLOR}"
    done

    # check that at least one ABI has been found
    [ ${#ABIS[@]} != 0 ] || { echo "${ERROR_COLOR}No ABI found in directory ${PATH_COLOR}$APP_PATH/project${NO_COLOR}"; exit; }
}

function create_ui_project() {
    echo -e "\n${ADDITION_COLOR}Creating react-ts template in ${PATH_COLOR}$UI_PATH${NO_COLOR}"
    pnpm create vite@latest $UI --template react-ts
    cd $UI
}

function install_dependencies() {
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}react-ts template dependencies${NO_COLOR}"
    pnpm install
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels${NO_COLOR}"
    pnpm install fuels --save
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels @fuel-wallet/sdk${NO_COLOR}"
    pnpm install fuels @fuel-wallet/sdk --save
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels @fuel-wallet/types${NO_COLOR}"
    pnpm install fuels @fuel-wallet/types --save
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels @fuel-ui/react${NO_COLOR}"
    pnpm install fuels @fuel-ui/react --save
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels @fuel-ui/css${NO_COLOR}"
    pnpm install fuels @fuel-ui/css --save
    echo -e "${ADDITION_COLOR}Installing ${PATH_COLOR}fuels @tanstact/react-query${NO_COLOR}"
    pnpm install @tanstack/react-query --save
}

function generate_types() {
    echo -e "\n${ADDITION_COLOR}Generating types for ${PURPLE}${ABIS[@]}${NO_COLOR}\n" 
    npx fuels typegen -i ${ABIS[@]} -o src/contracts
}

function apply_template() {
    rm -rf public
    rm -rf src/assets
    rm src/App.css
    rm src/index.css
    > tsconfig.json && cat $COMMON_PATH/tsconfig.json > tsconfig.json
    > index.html && cat $COMMON_PATH/index.html > index.html
    > src/App.tsx && cat $COMMON_PATH/src/App.tsx | tail -n +2 > src/App.tsx 
    > src/main.tsx && cat $COMMON_PATH/src/main.tsx | tail -n +2 > src/main.tsx 
    mkdir src/hooks
    cp $COMMON_PATH/hooks/index.ts src/hooks/index.ts
    cat $COMMON_PATH/hooks/useContract.ts | tail -n +2 > src/hooks/useContract.ts
    cat $COMMON_PATH/hooks/useFuel.ts | tail -n +2 > src/hooks/useFuel.ts
    cat $COMMON_PATH/hooks/useWallet.ts | tail -n +2 > src/hooks/useWallet.ts
    mkdir src/utils
    cp $COMMON_PATH/utils/index.tsx src/utils/index.tsx
    cat $COMMON_PATH/utils/queryClient.tsx | tail -n +2 > src/utils/queryClient.tsx
}

find_abis
create_ui_project
install_dependencies
generate_types
apply_template
pnpm run dev
