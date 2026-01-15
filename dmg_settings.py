import os.path

# Configuración de rutas
base_dir = os.path.abspath('.')

# 1. CONTENIDO REAL
files = {
    os.path.join(base_dir, 'dist/gui_main.app'): 'gui_main.app',
    os.path.join(base_dir, 'dist/gui_main/_internal'): '_internal'
}

# 2. ENLACES SIMBÓLICOS (Esto es lo que te faltaba)
symlinks = {
    'Aplicaciones': '/Applications'
}

# 3. POSICIONES (Asegúrate de que los nombres coincidan con las llaves de arriba)
icon_locations = {
    'gui_main.app': (140, 120),
    'Aplicaciones': (460, 120),
    '_internal': (300, 400) # Lejos para que no moleste
}

# 4. APARIENCIA
window_rect = ((200, 200), (600, 350))
icon_size = 100
volume_name = 'Instalador Transparente'