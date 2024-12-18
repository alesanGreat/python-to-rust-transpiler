# juego.py
import sdl2
import sdl2.ext

# Constantes
ANCHO = 800
ALTO = 600
BLANCO = (255, 255, 255)
NEGRO = (0, 0, 0)
ROJO = (255, 0, 0)

x = 100
y = 200
radio = 50

# Inicialización de SDL2
sdl2.ext.init()
window = sdl2.ext.Window("Ejemplo SDL2", size=(ANCHO, ALTO))
window.show()
renderer = sdl2.ext.Renderer(window)

# Función para dibujar un círculo (no nativo en SDL2)
def draw_circle(renderer, color, center, radius):
    cx, cy = center
    for dx in range(-radius, radius + 1):
        for dy in range(-radius, radius + 1):
            if dx * dx + dy * dy <= radius * radius:
                renderer.draw_point((cx + dx, cy + dy), color)

# Bucle principal
running = True
while running:
    for event in sdl2.ext.get_events():
        if event.type == sdl2.SDL_QUIT:
            running = False
            break

    # Rellenar pantalla de negro
    renderer.clear(NEGRO)

    # Dibujar círculo en el centro de la pantalla
    centro_x = ANCHO // 2
    centro_y = ALTO // 2
    draw_circle(renderer, BLANCO, (centro_x, centro_y), radio)

    # Dibujar rectángulo rojo
    rect = sdl2.SDL_Rect(x, y, 50, 50)
    renderer.fill(rect, ROJO)

    # Actualizar pantalla
    renderer.present()

    # Actualizar posición del rectángulo
    x += 1
    if x > 700:
        x = 100

# Finalizar SDL2
sdl2.ext.quit()