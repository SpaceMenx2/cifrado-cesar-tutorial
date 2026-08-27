use std::io::{self, Write};

const ABECEDARIO: [char; 27] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'Ñ', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn leer_entrada(texto: &str) -> String {
    print!("{texto}");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin().read_line(&mut entrada);
    entrada
}

fn cifrar_caracteres(caracter: char, k: usize, encriptar: bool) -> char {
    if let Some(posicion) = ABECEDARIO
        .iter()
        .position(|&caract| caract == caracter.to_ascii_uppercase())
    {
        let nueva_posicion = if encriptar {
            (posicion + k) % ABECEDARIO.len()
        } else {
            ((posicion + ABECEDARIO.len()) - k) % ABECEDARIO.len()
        };

        ABECEDARIO[nueva_posicion]
    } else {
        caracter
    }
}

fn main() {
    println!("Bienvenido al programa de Cifrado César!");

    loop {
        let encriptar: bool = loop {
            match leer_entrada("¿Qué desea hacer?\n1. Encriptar.\n2. Desencriptar.\nTu respuesta: ")
                .to_lowercase()
                .trim()
            {
                "1" | "encriptar" | "cifrar" => break true,
                "2" | "desencriptar" | "descifrar" => break false,
                _ => println!("Esa opción no es válida. Inténtalo nuevamente."),
            }
        };

        let k: usize = loop {
            match leer_entrada("Escriba el valor de desplazamiento: ").trim().parse::<usize>() {
                Ok(val) => break val % ABECEDARIO.len(),
                Err(_) => println!("Escriba por favor un valor de desplazamiento válido."),
            }
        };

        let oracion = loop {
            match leer_entrada(&format!("Escriba la oración objetivo (con k={}): ", k)) {
                valor if !valor.trim().is_empty() => break valor,
                _ => println!("Por favor, escriba una oración objetiva válida."),
            }
        };

        let resultado: String = oracion
            .chars()
            .map(|c| cifrar_caracteres(c, k, encriptar))
            .collect();

        println!("Texto original: {}", oracion.trim());
        println!("Texto final: {}", resultado.trim());

        match leer_entrada("¿Quieres seguir utilizando el programa? Si no, escriba 'n' o 'no': ")
            .to_lowercase()
            .trim()
        {
            "n" | "no" => break println!("Muchas gracias por utilizar el programa"),
            _ => (),
        }
    }
}
