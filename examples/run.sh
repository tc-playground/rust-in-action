#!/bin/bash

function compile_run_clean() {
    local target=$1
    if [[ -z "${target}" ]]; then
        echo "No terget was speicifed."
        exit 1
    fi
    echo $target
    local binary=${target%.rs}
    rustc "${target}"
    ./${binary}
    rm ${binary}
}

compile_run_clean $@
