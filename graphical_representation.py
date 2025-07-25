import pygame
import time

# Configuración inicial
pygame.init()
size = width, height = 600, 600
screen = pygame.display.set_mode(size)
clock = pygame.time.Clock()

# Tablero con IDs de piezas L
tablero_final = [
    [1, -1, 0, 0, 0, 0],
    [1, -1, 2, 2, 2, 0],
    [1, 1, 2, 0, 0, 0],
    [-1, 0, 0, 0, 3, 0],
    [0, 0, 0, 0, 3, 0],
    [0, 0, 0, 0, 3, 3],
]

filas, columnas = len(tablero_final), len(tablero_final[0])
celda = width // columnas

# Colores para piezas L
colores = {
    0: (255, 255, 255),
    1: (100, 100, 255),
    2: (100, 255, 100),
    3: (255, 200, 100),
    4: (255, 100, 150),
    5: (150, 255, 255),
    6: (255, 255, 150),
}

# Obtener IDs únicos usados en el tablero (en orden creciente)
ids_en_uso = sorted(set(c for fila in tablero_final for c in fila if c != 0))

# Tablero temporal para animación
tablero_actual = [[0 for _ in range(columnas)] for _ in range(filas)]

def dibujar_tablero():
    screen.fill((0, 0, 0))
    for y in range(filas):
        for x in range(columnas):
            valor = tablero_actual[y][x]
            if valor == -1:
                pygame.draw.line(screen, (150, 150, 150), (x * celda, y * celda), ((x+1) * celda, (y+1) * celda), 2)
                pygame.draw.line(screen, (150, 150, 150), ((x+1) * celda, y * celda), (x * celda, (y+1) * celda), 2)
                continue
            color = colores.get(valor, (200, 0, 0))
            rect = pygame.Rect(x * celda, y * celda, celda, celda)
            pygame.draw.rect(screen, color, rect)
            pygame.draw.rect(screen, (50, 50, 50), rect, 1)
    pygame.display.flip()

# Animar paso a paso el llenado
running = True
paso = 0
tiempo_ultimo = time.time()
delay = 0.5  # segundos entre pasos

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    ahora = time.time()
    if paso < len(ids_en_uso) and ahora - tiempo_ultimo > delay:
        id_actual = ids_en_uso[paso]
        for y in range(filas):
            for x in range(columnas):
                if tablero_final[y][x] == id_actual:
                    tablero_actual[y][x] = id_actual
        paso += 1
        tiempo_ultimo = ahora

    dibujar_tablero()
    clock.tick(60)

pygame.quit()

