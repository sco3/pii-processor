#!/usr/bin/env -S bash

set -xueo pipefail


for i in $(seq 1 10000)  ;do 
    ./publish.sh
    #sleep 2
done 