# Instrucciones de uso
Este repositorio contiene un conjunto de scripts y herramientas para la gestión de datos y la automatización de tareas. A continuación, se detallan las instrucciones para su uso.
## Requisitos 
- Python 3.x
- pip
- Rust
- Cargo (Se instala junto con Rust)

## Ejecución de scripts
Es recomendado usar entornos virtuales
```bash
python3 -m venv .venv && source .venv/bin/activate && pip install -r requirements.txt 
````
Preparacion de script
```bash
chmod +x ejecutar_casos.sh
./ejecutar_casos.sh
```
<small>Actualmente hay una condicion carrera por el paralelismo debido a que en cada ejecucion intenta realizar la construccion del ejecutable del programa</small>

Se crean dos carpetas, una con los resultados de los casos en formato TXT `resultados/` y otra con las graficas renderizadas `imagenes/`