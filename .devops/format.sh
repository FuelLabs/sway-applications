#!/bin/bash

declare -a APPS
mapfile -t APPS < apps.txt

REPO_ROOT=$(dirname $(dirname $(realpath $0)))

cd $REPO_ROOT

for app in "${APPS[@]}"
do
    echo Formatting $app
    cd $app/project
    forc fmt
    cargo fmt
    cd $REPO_ROOT
    echo
done
