#!/bin/bash
set -euo pipefail

file_path="src/casos_tetrominos.txt"
iteraciones=50
max_jobs=5

run_case(){ 
    i=$1
    ./ejecutar.sh "$file_path" "$i" "$iteraciones"
}

for i in {1..20}; do
    run_case "$i" &
    while [ "$(jobs -p | wc -l)" -ge "$max_jobs" ]; do
        sleep 0.2
    done
done

wait
echo "Todos los casos han sido ejecutados."