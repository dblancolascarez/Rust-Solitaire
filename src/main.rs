// dependencias
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::io::{self, Write};

use log::{info, warn, LevelFilter};
use simplelog::*;
use crossterm::event::{self, Event, KeyCode};


struct Mazo {
    cartas: Vec<Carta>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Carta {
    rango: Rango, color: Color,
}

// color de cartas
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Corazones,Diamantes,Treboles,Picas,
}

// // rango de cartas
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rango {
    Dos,Tres,Cuatro,Cinco,Seis,Siete,Ocho,Nueve,Diez,Jota,Reina,Rey,As,
}

// constructor carta
impl Carta {
    fn new(rango: Rango, color: Color) -> Carta {
        Carta { rango, color }
    }
}

// constructor mazo
impl Mazo {
    fn new() -> Mazo {
        let mut cartas = Vec::with_capacity(52);
        for &color in &[Color::Corazones, Color::Diamantes, Color::Treboles, Color::Picas] {
            for &rango in &[Rango::Dos, Rango::Tres, Rango::Cuatro, Rango::Cinco, Rango::Seis, Rango::Siete, Rango::Ocho, Rango::Nueve, Rango::Diez, Rango::Jota, Rango::Reina, Rango::Rey, Rango::As] {
                cartas.push(Carta::new(rango, color));
            }
        }
        Mazo { cartas }
    }

    fn barajar(&mut self) {
        let mut rng = thread_rng();
        self.cartas.shuffle(&mut rng);
    }
}

// apariencia cartas
impl fmt::Display for Carta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_str = match self.color {
            Color::Corazones => "♡",
            Color::Diamantes => "♢",
            Color::Treboles => "♧",
            Color::Picas => "♤",
        };
        let rango_str = match self.rango {
            Rango::Dos => "2",
            Rango::Tres => "3",
            Rango::Cuatro => "4",
            Rango::Cinco => "5",
            Rango::Seis => "6",
            Rango::Siete => "7",
            Rango::Ocho => "8",
            Rango::Nueve => "9",
            Rango::Diez => "T",
            Rango::Jota => "J",
            Rango::Reina => "Q",
            Rango::Rey => "K",
            Rango::As => "A",
        };
        write!(f, "{}{}", rango_str, color_str)
    }
}


/*  
Imprime el estado actual de la cuadrícula de cartas.

Esta función itera sobre la matriz de cartas e imprime la representación de cada carta. 
Si una carta está presente, imprime la carta; de lo contrario, imprime "Nada".
También imprime los números de columna para referencia.
 
# Parámetros

* `matriz`: Una referencia a un vector 2D de `Option<Carta>`. Cada `Option<Carta>` representa una carta en la cuadrícula. 
  Si una carta está presente, es `Some(Carta)`,   de lo contrario, es `None`.

# Retornos
Esta función no retorna ningún valor. Solo imprime la cuadrícula de cartas.
*/ 
fn matriz_imprimir(matriz: &[Vec<Option<Carta>>]) {
    println!("Cartas:");
    for (_i, row) in matriz.iter().enumerate() {
        for carta in row {
            match carta {
                Some(c) => print!("{} ", c), None => print!("Nada "),
            }
        }
        println!(""); 
    }
    println!("1| 2| 3| 4| 5| 6| 7| 8| -> columnas"); 
}

/*
Verifica si la cuadrícula y las cartas sobrantes están vacías.

Esta función recorre la cuadrícula y las cartas sobrantes para determinar si todas las posiciones son `None`.

# Parámetros

* `grid`: Una referencia a un vector 2D de `Option<Carta>`. Cada `Option<Carta>` representa una carta en la cuadrícula.
  Si hay una carta presente, es `Some(Carta)`; de lo contrario, es `None`.

* `cartas_sobrantes`: Una referencia a un slice de `Option<Carta>`. Cada `Option<Carta>` representa una carta extra.
  Si hay una carta presente, es `Some(Carta)`; de lo contrario, es `None`.

# Retorna

* `bool`: Retorna `true` si todas las posiciones en la cuadrícula y las cartas sobrantes son `None`, indicando que están vacías.
   Retorna `false` en caso contrario.
*/
fn matriz_nula(matriz: &[Vec<Option<Carta>>], cartas_faltantes: &[Option<Carta>]) -> bool {
    matriz.iter().all(|row| row.iter().all(|&card| card.is_none())) &&
    cartas_faltantes.iter().all(|&carta| carta.is_none())
}

/*
Busca la carta más alta en una columna específica de la cuadrícula.

Esta función recorre las filas de la columna especificada en orden inverso, buscando la primera ranura de carta no vacía. 
Si se encuentra una carta, devuelve la carta junto con su posición de fila.

# Parámetros

* `grid`: Una referencia a un vector 2D de `Option<Carta>`, que representa la cuadrícula de cartas.
  Cada `Option<Carta>` puede ser `Some(Carta)` si hay una carta presente, o `None` si la ranura está vacía.
* `col`: El índice de la columna que se va a buscar dentro de la cuadrícula.

# Retorna

* `Option<(Carta, usize)>`: Retorna `Some((Carta, usize))` si se encuentra una carta, donde `Carta` es la carta encontrada y `usize` es el índice de fila de la carta. 
  Retorna `None` si no se encuentra ninguna carta en la columna.
*/
fn buscar_carta_alta(matriz: &[Vec<Option<Carta>>], col: usize) -> Option<(Carta, usize)> {
    for row in (0..6).rev() {
        if let Some(carta) = matriz[row][col] {
            return Some((carta, row));
        }
    }
    None
}

/*
Registra el estado actual de la cuadrícula de cartas.

Esta función recorre la cuadrícula de cartas y registra la representación de cada carta.
Si hay una carta presente, registra la carta; de lo contrario, registra "None".
También registra los números de fila para referencia.

# Parámetros

* `grid`: Una referencia a un vector 2D de `Option<Carta>`. Cada `Option<Carta>`
  representa una carta en la cuadrícula. Si hay una carta presente, es `Some(Carta)`; de lo contrario, es `None`.

# Retorna

Esta función no retorna ningún valor. Registra la cuadrícula de cartas.
*/
fn cartas_log(matriz: &[Vec<Option<Carta>>]) {
    info!("\nCartas:");
    for (i, row) in matriz.iter().enumerate() {
        let row_string: Vec<String> = row.iter().map(|cartas| match cartas {
            Some(c) => format!("{}", c), None => "Nada".to_string(),
        }).collect();
        info!("{} | {}", row_string.join(" "), i + 1);
    }
    info!("1| 2| 3| 4| 5| 6| 7| 8| -> columnas");  
}

/*
Registra las cartas sobrantes en el juego.

Esta función recorre el slice de cartas sobrantes proporcionado y registra la representación de cada carta. 
Si hay una carta presente, registra el rango y el palo de la carta. 
Si no hay carta, registra "None" para esa posición.

# Parámetros

* `cartas_sobrantes`: Una referencia a un slice de `Option<Carta>`. Cada `Option<Carta>` representa una carta extra en el juego. 
  Si hay una carta presente, es `Some(Carta)`; de lo contrario, es `None`.

# Retorna

Esta función no retorna ningún valor. Registra las cartas sobrantes.
*/
fn log_cartas_extra(cartas_faltantes: &[Option<Carta>]) {
    info!("\nCartas faltantes:");
    for (i, carta) in cartas_faltantes.iter().enumerate() {
        match carta {
            Some(c) => info!("{}: {}", i + 1, c), None => info!("{}: Nada", i + 1),
        }
    }
}

/* 
Imprime las cartas extra disponibles en el juego.

Esta función itera sobre la lista proporcionada de cartas extra e imprime la representaciónde cada carta. 
Si una carta está presente, imprime el rango y el color de la carta. 
Si una carta está ausente, imprime "Nada" para esa posición.

# Parámetros

* `cartas_sobrantes`: Una referencia a un slice de `Option<Carta>`. 
  Cada `Option<Carta>` representa una carta extra en el juego. 
  Si una carta está presente, es `Some(Carta)`, de lo contrario, es `Nada`.

# Retornos

Esta función no retorna ningún valor. Solo imprime las cartas extra.
*/
fn cartas_extra_imprimir(cartas_faltantes: &[Option<Carta>]) {
    println!("\nCartas extra:");
    for (i, carta) in cartas_faltantes.iter().enumerate() {
        match carta {
            Some(c) => println!("{}: {}", i + 1, c), None => println!("{}: Nada", i + 1),
        }
    }
}

/*
Lee y valida la entrada del usuario como un entero positivo.

Esta función solicita continuamente al usuario que ingrese un valor hasta que se proporcione un entero positivo válido.
Si el usuario ingresa un valor no numérico o un número negativo, se muestra un mensaje de error y se repite la solicitud.

# Parámetros

* `prompt`: Una cadena que se muestra al usuario como solicitud de entrada.

# Retorna

* Un entero positivo ingresado por el usuario.
*/
fn input(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    loop {
        if event::poll(std::time::Duration::from_secs(10)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => {
                        println!("\nTecla ESC presionada. Saliendo...");
                        return None;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        println!("\nTecla 'n' presionada. Iniciando un nuevo juego...");
                        return Some(usize::MAX); // Indicador especial para reiniciar el juego.
                    }
                    KeyCode::Char('u') | KeyCode::Char('U') => {
                        println!("\nTecla 'u' presionada. volviendo al movimiento anterior...");
                        return Some(usize::MAX); // Indicador especial para reiniciar el juego.
                    }
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let num = c.to_digit(10).map(|n| n as usize);
                        if let Some(n) = num {
                            println!("{}", n);
                        }
                        return c.to_digit(10).map(|n| n as usize);
                    }
                    _ => {
                        println!("Entrada invalida :P");
                    }
                }
            }
        } else {
            println!("Esperando entrada...");
        }
    }
}

/*
Ejecuta el bucle principal del juego de solitario tipo Nestor.

Esta función controla la lógica del juego, permitiendo al usuario seleccionarcartas de la cuadrícula o de las cartas sobrantes para emparejarlas. 
El juego continúa hasta que todas las cartas han sido emparejadas y removidas.

# Parámetros

* `grid`: Un vector 2D mutable de `Option<Carta>`, que representa la cuadrícula de cartas en el juego. 
  Cada `Option<Carta>` puede ser `Some(Carta)` si hay una carta presente, o `None` si la ranura está vacía.
* `cartas_sobrantes`: Un vector mutable de `Option<Carta>`, que representa las cartas extra disponibles para emparejar. 
  Cada `Option<Carta>` puede ser `Some(Carta)` si hay una carta presente, o `None` si la ranura está vacía.

# Retornos

Esta función no retorna ningún valor. El juego termina cuando todas las cartas han sido emparejadas y removidas, mostrando un mensaje de victoria.
*/
fn juego(mut matriz: Vec<Vec<Option<Carta>>>, mut cartas_faltantes: Vec<Option<Carta>>) {
    loop {
        
        matriz_imprimir(&matriz);
        cartas_log(&matriz);
        cartas_extra_imprimir(&cartas_faltantes);
        log_cartas_extra(&cartas_faltantes);
        
        print!("\nColumna 9 en caso de usar las cartas extra \n");
        println!("Comandos: <ESC> Salir, n/N Nuevo juego, u/U Deshacer.");

        // input carta 1
        let carta_columna1 = match input("\nCarta columna 1: ") {
            Some(num) => num - 1,
            None => {
                println!("\nJuego terminado ;)");
                break;
            }
        };
        info!("Has seleccionado la columna {} para la carta 1 :O", carta_columna1 + 1);

        // input carta 2
        let carta_columna2 = match input("\nCarta columna 2: ") {
            Some(num) => num - 1,
            None => {
                println!("\nJuego terminado ;)");
                break;
            }
        };
        info!("Has seleccionado la columna {} para la carta 2 :O", carta_columna2 + 1);

        if carta_columna1 < 9 && carta_columna2 < 9 && carta_columna1 != carta_columna2{
            let (carta1, posicion_carta1) = if carta_columna1 < 8 {
                match buscar_carta_alta(&matriz, carta_columna1) {
                    Some((carta, posicion)) => (Some(carta), Some(posicion)),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            let (carta2, posicion_carta2) = if carta_columna2 < 8 {
                match buscar_carta_alta(&matriz, carta_columna2) {
                    Some((carta, posicion)) => (Some(carta), Some(posicion)),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            if carta1.is_none() && carta_columna1 < 8 || carta2.is_none() && carta_columna2 < 8 {
                println!("\nno se posee ninguna carta en una o ambas posiciones :P intentalo de nuevo :D");
                continue;
            }

            // caso carta 1
            let carta1 = if carta_columna1 < 8 { carta1.unwrap() } else { cartas_faltantes[carta_columna1 - 8].unwrap() };
            if carta_columna1 == 8 {
                cartas_faltantes[carta_columna1 - 8] = Some(carta1);
            }

            // caso carta 2
            let carta2 = if carta_columna2 < 8 { carta2.unwrap() } else { cartas_faltantes[carta_columna2 - 8].unwrap() };
            if carta_columna2 == 8 {
                cartas_faltantes[carta_columna2 - 8] = Some(carta2);
            }

            // mismo numero de carta
            if carta1.rango == carta2.rango {
                if carta_columna1 < 8 {
                    matriz[posicion_carta1.unwrap()][carta_columna1] = None;
                } else {
                    cartas_faltantes[carta_columna1 - 8] = None;
                }
                if carta_columna2 < 8 {
                    matriz[posicion_carta2.unwrap()][carta_columna2] = None;
                } else {
                    cartas_faltantes[carta_columna2 - 8] = None;
                }
            
            
            } else if carta_columna1 == 8 {
                let mut iguales = false;
                for (_i, carta_falt) in cartas_faltantes.iter_mut().enumerate() {
                    if carta_falt.is_some() && carta_falt.unwrap().rango == carta2.rango {

                        cartas_faltantes[_i] = None;
                        matriz[posicion_carta2.unwrap()][carta_columna2] = None;
                        iguales = true;
                        break;
                    }
                }
                if !iguales {
                    println!("\nLas cartas parecen no coincidir con ninguna carta extra :/ Intentalo de nuevo :)");
                    warn!("Las cartas parecen no coincidir con ninguna carta extra :/ Intentalo de nuevo :)");
                }
            } else if carta_columna2 == 8 {
                let mut iguales = false;
                for (_i, carta_falt) in cartas_faltantes.iter_mut().enumerate() {
                    if carta_falt.is_some() && carta_falt.unwrap().rango == carta1.rango {

                        cartas_faltantes[_i] = None;
                        matriz[posicion_carta1.unwrap()][carta_columna1] = None;
                        iguales = true;
                        break;
                    }
                }
                if !iguales {
                    println!("\nLas cartas parecen no coincidir con ninguna carta extra :/ Intentalo de nuevo :)");
                    warn!("Las cartas parecen no coincidir con ninguna carta extra :/ Intentalo de nuevo :)");
                }
            } else {
                println!("\nParece que las cartas no coinciden :/ Intentalo de nuevo :)");
                warn!("Parece que las cartas no coinciden :/ Intentalo de nuevo :)");
            }
        } else {
            println!("\nPosicion no valida :P Intentalo de nuevo :)");
            warn!("Posicion no valida :P Intentalo de nuevo :)");
        }
        if matriz_nula(&matriz, &cartas_faltantes) {
            println!("\nHas ganado el juego :D");
            info!("Has ganado el juego :D");
            break;
        }
    }
}

/*
Inicializa el entorno del juego, baraja el mazo y distribuye las cartas.

# Parámetros

* `level_filter`: El filtro de nivel de registro para el logger.
* `config`: La configuración para el logger.
* `file`: El archivo donde se escribirá la salida del registro.

# Retornos

* Una tupla que contiene la cuadrícula inicializada y las cartas sobrantes.
*/
fn main() {
    WriteLogger::init(LevelFilter::Info, 
    Config::default(), 
    std::fs::File::create("juego.log").unwrap()).unwrap();

    print!("Juego Solitario tipo Nestor \n");
    let mut mazo = Mazo::new();
    mazo.barajar();

    let mut matriz: Vec<Vec<Option<Carta>>> = vec![vec![None; 8]; 6];
    let mut cartas_sobrantes: Vec<Option<Carta>> = vec![None; 4];

    for (i, carta) in mazo.cartas.iter().enumerate() {
        if i < 48 {
            let row = i / 8;
            let col = i % 8;
            matriz[row][col] = Some(*carta);
        } else {
            cartas_sobrantes[i - 48] = Some(*carta);
        }
    }

    juego(matriz, cartas_sobrantes);
}





