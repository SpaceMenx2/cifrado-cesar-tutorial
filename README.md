# Cifrado César en Rust 🦀

Implementación del algoritmo de cifrado César en el lenguaje de programación Rust, con soporte para el abecedario español (incluyendo la Ñ).

## 📺 Videotutorial

Este código corresponde al videotutorial disponible en YouTube:
[Cifrado César | Lenguaje de programación Rust](https://www.youtube.com/watch?v=zbOy8vXusqY)

## 📋 Descripción

El cifrado César es una técnica de cifrado por sustitución donde cada letra del texto original se desplaza un número fijo de posiciones en el alfabeto. Este programa permite:

- **Encriptar** texto usando un desplazamiento personalizado
- **Desencriptar** texto previamente cifrado
- Soporte completo para caracteres españoles (incluye **Ñ**)
- Validación de entrada robusta
- Interfaz interactiva por consola

## 🚀 Características

- ✅ Abecedario español de 27 letras (A-Z + Ñ)
- ✅ Manejo de mayúsculas/minúsculas
- ✅ Preserva caracteres no alfabéticos (espacios, números, símbolos)
- ✅ Validación de entrada para evitar errores
- ✅ Bucle principal para múltiples operaciones

## 🛠️ Requisitos

- Rust instalado (versión estable recomendada)
- Sistema operativo: Windows, Linux o macOS

## 💻 Compilación y Ejecución

```bash
# Clonar el repositorio
git clone https://github.com/SpaceMenx2/cifrado-cesar-tutorial/
cd cifrado-cesar-rust

# Compilar
cargo build --release

# Ejecutar
cargo run
