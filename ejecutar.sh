#!/bin/bash

if [ $# -ne 3 ]; then
    echo "Uso: $0 <file_path> <# Caso> <iteraciones>"
    exit 1
fi

file_path="$1"
caso="$2"
iteraciones="$3"

echo "=== Procesando Caso ${caso} ==="

cargo build --release > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error en la compilación"
    exit 1
fi

# Ejecutar algoritmo
resultado=$(./target/release/proyecto-busqueda-local "$file_path" false "$caso" "$iteraciones" 1)
if [ $? -ne 0 ]; then
    echo "Error al ejecutar el algoritmo"
    exit 1
fi

# Guardar resultado
mkdir -p resultados
archivo="resultados/caso_${caso}.txt"
echo "$resultado" > "$archivo"

# Generar imagen
source .venv/bin/activate
python3 graphical_representation.py "$archivo" "$caso" > /dev/null 2>&1
echo "Ejecucion terminada."