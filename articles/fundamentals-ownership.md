---
title: Fundamentals | Ownership
author: Michael Cardoza
date: 2023-09-18
tags:
  - rust
  - comunidad
social:
  github: https://github.com/RustLangES
  website: https://rustlanges.github.io
---

El __Ownership__ es una característica fundamental del lenguaje que se utiliza para gestionar la asignación y liberación de memoria de manera segura y prevenir errores.

<!-- more -->


## ¿Qué es el ownership en Rust?

Es fundamental para los programadores rastrear la memoria, ya que no hacerlo puede resultar en una "fuga de memoria" - (Leak). Rust utiliza un modelo de "propiedad" conocido como Ownership para gestionar la memoria.

El __Ownership__ (propiedad) en Rust nos permite tener un control extremadamente preciso sobre la gestión de la memoria, garantizando la seguridad y la prevención de errores de acceso a la memoria en tiempo de ejecución.


## ¿Comó funciona el ownership en Rust?

En Rust, la memoria se puede "mover" (move) o "tomar prestada" (borrowed).

### Ejemplo: Move

```rust
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;

    display_light(dull);
    display_light(dull); // will not work
}
```

En este ejemplo, el valor asignado a la variable `dull` se "moverá" al ámbito de la primera función `display_light` que se ejecuta. Como resultado, ya no está disponible en el ámbito de la función main. Cuando se intenta ejecutar la segunda función `display_light`, el valor de la variable `dull` ya no está disponible.

### Ejemplo: Borrow

```rust
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;

    display_light(&dull);
    display_light(&dull);
}
```

En este segundo ejemplo, agregamos el símbolo `&`.

El símbolo `&` indica una referencia (puntero) a una instancia de Light. En otras palabras, estamos "prestando" el valor de `dull` a la ejecución de cada función `display_light`, en lugar de moverlo. Esto permite que el valor de dull siga estando disponible después de llamar a la función `display_light`.
