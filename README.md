# Rust: Juego de Solitario

Este proyecto es la implementación de un juego de solitario en el lenguaje de programación Rust. Incluye el desarrollo de la lógica del juego, interacción a través de comandos y la opción de una interfaz gráfica.

## Características del Juego

- **Baraja**: La baraja consiste en 52 cartas con los siguientes valores:
  - Valores: 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K, A.
  - Colores:
    - Rojo: Corazones (C) y Diamantes (D).
    - Negro: Tréboles (T) y Espadas (E).


- **Comandos de Interacción**:
  - `<ESC>`: Salir del juego.
  - `n/N`: Comienza un nuevo juego aleatorio.
  - `<RET>`: Saca una nueva carta.
  - `1-7`: Selecciona columnas para mover cartas.
  - `u/U`: Deshacer el último movimiento.

- **Randomización**:
  - Generación de barajas aleatorias por defecto.
  - Uso de semillas para reproducir juegos idénticos si se especifica un número en la línea de comandos.

- **Registro del Juego**:
  - Guarda el historial de la interacción con el usuario en un archivo de log.
  - El log incluye el estado completo de las cartas (todas descubiertas).


## Instrucciones para Ejecutar el Proyecto

1. **Clonar el Repositorio**:
   ```bash
   git clone <URL_del_repositorio>
   cd <directorio_del_proyecto>
   ```

2. Compilar el Proyecto:
  ```bash
  cargo build
  ```
3. Ejecutar el Juego:
  ```bash
  cargo run
  ```



