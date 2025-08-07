import pygame
import sys
import os

def leer_contenido_archivo(archivo_resultado):
    """Lee el contenido completo del archivo de resultado"""
    if not os.path.exists(archivo_resultado):
        print(f"Error: Archivo {archivo_resultado} no existe")
        return None
    
    with open(archivo_resultado, 'r') as f:
        contenido = f.read().strip()
    
    return contenido

def leer_tableros(contenido):
    """Lee todos los tableros disponibles desde el contenido"""
    if not contenido:
        return None
    
    lineas = contenido.split('\n')
    tableros = {}
    
    tipos = ['TABLERO_INICIAL', 'SOLUCION_INICIAL', 'SOLUCION_FINAL', 'NO_SOLUCION']
    
    for tipo in tipos:
        inicio_idx = -1
        fin_idx = -1
        
        for i, linea in enumerate(lineas):
            if f"{tipo}_INICIO" in linea:
                inicio_idx = i
            elif f"{tipo}_FIN" in linea:
                fin_idx = i
                break
        
        if inicio_idx != -1 and fin_idx != -1:
            try:
                dimensiones = lineas[inicio_idx + 1].split(':')
                filas, columnas = int(dimensiones[0]), int(dimensiones[1])
                
                tablero = []
                for i in range(inicio_idx + 2, fin_idx):
                    if lineas[i].strip():
                        fila = [int(x) for x in lineas[i].strip().split()]
                        tablero.append(fila)
                
                tableros[tipo] = {
                    'tablero': tablero,
                    'filas': filas,
                    'columnas': columnas
                }
            except (ValueError, IndexError) as e:
                print(f"Error leyendo {tipo}: {e}")
                continue
    
    return tableros if tableros else None

def obtener_colores():
    """Retorna el diccionario de colores para los tableros"""
    return {
        0: (255, 255, 255),    # Blanco - vacío
        -1: (100, 100, 100),   # Gris - bloqueado
        1: (100, 100, 255),    # Azul
        2: (100, 255, 100),    # Verde
        3: (255, 200, 100),    # Naranja
        4: (255, 100, 150),    # Rosa
        5: (150, 255, 255),    # Cian
        6: (255, 255, 150),    # Amarillo
        7: (200, 100, 255),    # Violeta
        8: (255, 150, 100),    # Salmón
        9: (255, 100, 100),    # Rojo
        10: (100, 255, 200),   # Verde claro
    }

def agregar_texto_imagen(screen, texto, x, y, color=(255, 255, 255), tamaño=24):
    """Agrega texto a la imagen usando la fuente Arial."""
    font = pygame.font.SysFont("Arial", tamaño)
    text_surface = font.render(texto, True, color)
    screen.blit(text_surface, (x, y))
    return text_surface.get_height()

def agregar_texto_multilinea(screen, texto, x, y, max_width, color=(255, 255, 255), tamaño=24):
    """Agrega texto con salto de línea automático usando la fuente Arial."""
    font = pygame.font.SysFont("Arial", tamaño)
    palabras = texto.split(' ')
    lineas = []
    linea_actual = []
    
    for palabra in palabras:
        linea_test = ' '.join(linea_actual + [palabra])
        if font.size(linea_test)[0] <= max_width:
            linea_actual.append(palabra)
        else:
            if linea_actual:
                lineas.append(' '.join(linea_actual))
            linea_actual = [palabra]
    
    if linea_actual:
        lineas.append(' '.join(linea_actual))
    
    altura_total = 0
    interlineado = 5
    for i, linea in enumerate(lineas):
        text_surface = font.render(linea, True, color)
        screen.blit(text_surface, (x, y + i * (tamaño + interlineado)))
        altura_total += tamaño + interlineado
    
    return altura_total

def contar_casilleros_libres(tablero):
    """Cuenta los casilleros libres (valor 0) en el tablero"""
    return sum(fila.count(0) for fila in tablero)

def contar_casilleros_ocupados(tablero):
    """Cuenta los casilleros ocupados (valor > 0) en el tablero"""
    return sum(sum(1 for celda in fila if celda > 0) for fila in tablero)

def guardar_imagen_tablero(tablero, filas, columnas, nombre_archivo, titulo, mensaje_extra=None, es_solucion=False):
    """Genera y guarda una imagen individual del tablero, ajustando el ancho dinámicamente."""
    pygame.init()
    pygame.font.init()
    
    celda_size = 80
    margen_superior = 80
    padding_horizontal = 40

    # --- Determinar el ancho de la imagen ---
    board_width = columnas * celda_size
    
    font_titulo = pygame.font.SysFont("Arial", 36)
    title_width = font_titulo.size(titulo)[0]
    
    footer_text = ""
    font_footer = pygame.font.SysFont("Arial", 24)
    footer_width = 0
    if not mensaje_extra:
        if es_solucion:
            casilleros_ocupados = contar_casilleros_ocupados(tablero)
            casilleros_libres = contar_casilleros_libres(tablero)
            footer_text = f"Celdas ocupadas: {casilleros_ocupados}  |  Celdas libres: {casilleros_libres}"
        else:
            casilleros_libres = contar_casilleros_libres(tablero)
            footer_text = f"Celdas libres: {casilleros_libres}"
        footer_width = font_footer.size(footer_text)[0]

    width = max(board_width, title_width, footer_width) + padding_horizontal

    # --- Determinar la altura de la imagen ---
    margen_inferior = 80
    if mensaje_extra:
        font_msg = pygame.font.SysFont("Arial", 22)
        max_width_msg = width - 60
        palabras = mensaje_extra.split(' ')
        lineas_estimadas = 1
        linea_actual = ""
        for palabra in palabras:
            linea_test = f"{linea_actual} {palabra}".strip()
            if font_msg.size(linea_test)[0] > max_width_msg:
                lineas_estimadas += 1
                linea_actual = palabra
            else:
                linea_actual = linea_test
        altura_msg = lineas_estimadas * (font_msg.get_height() + 5)
        margen_inferior = altura_msg + 40

    height = filas * celda_size + margen_superior + margen_inferior
    
    screen = pygame.display.set_mode((width, height))
    pygame.display.set_caption(titulo)
    
    # --- Dibujar en la superficie ---
    colores = obtener_colores()
    screen.fill((0, 0, 0)) # Fondo negro
    
    title_pos_x = (width - title_width) / 2
    agregar_texto_imagen(screen, titulo, title_pos_x, 25, (255, 255, 255), 36)
    
    board_offset_x = (width - board_width) / 2
    offset_y = margen_superior
    for y in range(filas):
        for x in range(columnas):
            valor = tablero[y][x]
            rect = pygame.Rect(x * celda_size + board_offset_x, y * celda_size + offset_y, celda_size, celda_size)
            
            if valor == -1:
                pygame.draw.rect(screen, (20, 20, 20), rect)
                pygame.draw.line(screen, (80, 80, 80), rect.topleft, rect.bottomright, 3)
                pygame.draw.line(screen, (80, 80, 80), rect.bottomleft, rect.topright, 3)
            else:
                color = colores.get(valor, (200, 0, 0))
                pygame.draw.rect(screen, color, rect)
            
            pygame.draw.rect(screen, (50, 50, 50), rect, 2)
    
    inicio_footer_y = offset_y + filas * celda_size + 30
    
    if mensaje_extra:
        max_width_msg = width - 60
        agregar_texto_multilinea(screen, mensaje_extra, 30, inicio_footer_y, max_width_msg, (255, 200, 200), 22)
    else:
        footer_pos_x = (width - footer_width) / 2
        color_footer = (200, 255, 200) if es_solucion else (200, 200, 255)
        agregar_texto_imagen(screen, footer_text, footer_pos_x, inicio_footer_y, color_footer, 24)

    pygame.image.save(screen, nombre_archivo)
    pygame.quit()

def guardar_imagenes(tableros, caso_num):
    """Genera todas las imágenes para un caso"""
    os.makedirs("imagenes", exist_ok=True)
    os.makedirs(f"imagenes/caso_{caso_num}", exist_ok=True)
    
    nombres = {
        'TABLERO_INICIAL': f"imagenes/caso_{caso_num}/inicial.png",
        'SOLUCION_INICIAL': f"imagenes/caso_{caso_num}/solucion_inicial.png",
        'SOLUCION_FINAL': f"imagenes/caso_{caso_num}/solucion_final.png",
        'NO_SOLUCION': f"imagenes/caso_{caso_num}/no_solucion.png"
    }
    
    titulos = {
        'TABLERO_INICIAL': f"Caso {caso_num} - Tablero Inicial",
        'SOLUCION_INICIAL': f"Caso {caso_num} - Solución Inicial",
        'SOLUCION_FINAL': f"Caso {caso_num} - Solución Final",
        'NO_SOLUCION': f"Caso {caso_num} - Sin Solución"
    }
    
    for tipo, datos in tableros.items():
        if tipo in nombres:
            mensaje_extra = None
            es_solucion = tipo in ['SOLUCION_INICIAL', 'SOLUCION_FINAL']
            
            if tipo == 'NO_SOLUCION':
                casilleros_libres = contar_casilleros_libres(datos['tablero'])
                if casilleros_libres % 4 != 0:
                    mensaje_extra = f"No se puede resolver: El número de casilleros libres ({casilleros_libres}) no es múltiplo de 4."
                else:
                    mensaje_extra = "No se encontró una solución válida para este caso."
            
            guardar_imagen_tablero(
                datos['tablero'],
                datos['filas'],
                datos['columnas'],
                nombres[tipo],
                titulos[tipo],
                mensaje_extra,
                es_solucion
            )
            print(f"✅ Imagen guardada: {nombres[tipo]}")

def main():
    if len(sys.argv) != 3:
        print("Uso: python3 graphical_representation.py <archivo_resultado> <numero_caso>")
        sys.exit(1)
    
    archivo_resultado = sys.argv[1]
    caso_num = sys.argv[2]
    
    contenido = leer_contenido_archivo(archivo_resultado)
    if not contenido:
        print(f"Error: No se pudo leer el archivo {archivo_resultado}")
        return
    
    tableros = leer_tableros(contenido)
    if tableros is None:
        print(f"Caso {caso_num}: No se encontraron tableros válidos en el archivo")
        return
    
    guardar_imagenes(tableros, caso_num)
    
    if 'NO_SOLUCION' in tableros:
        print(f"📸 Imagen del caso {caso_num} generada (sin solución)")
    elif 'SOLUCION_FINAL' in tableros:
        print(f"📸 Todas las imágenes del caso {caso_num} generadas correctamente")
    else:
        print(f"📸 Imágenes parciales del caso {caso_num} generadas")

if __name__ == "__main__":
    main()