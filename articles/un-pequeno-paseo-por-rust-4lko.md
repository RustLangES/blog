---
title: Un pequeño paseo por Rust
description: Antes de comenzar, quiero aclarar que no es para nada sencillo escribir casi a diario. Cada vez que...
author: Maximiliano Burgos
github_user: maxwellnewage
date: 2023-05-27
tags:
  - rust
  - comunidad
  - devjournal
  - devto
social:
  github: https://github.com/maxwellnewage
  twitter: https://twitter.com/maxwellnewage
  linkedin: https://www.linkedin.com/in/maximilianoburgos/
---
Antes de comenzar, quiero aclarar que no es para nada sencillo escribir casi a diario. Cada vez que lo intento, termino inmerso en una nueva investigación que me lleva semanas, o incluso un mes entero. Este fue el caso de Rust.

# ¿Pero cómo llegué acá?

Es una pregunta que me hago probablemente desde mi nacimiento, pero en este caso fue un poco más extraño: si leyeron [mi artículo anterior](https://dev.to/maxwellnewage/creo-que-me-converti-en-un-desarrollador-en-react-y-typescript-2fkf), mi plan en muy resumidas cuentas, era aprender React y TypeScript. Mi felicidad estaba llegando, dado que terminaría mi fase de estudiante eterno para introducirme finalmente en el momento de empezar el bendito proyecto del clicker, hasta que llegó esto:

<center>
<youtube video="XYH6_GV5KQ0"></youtube>
</center>

Tauri es un duro competidor de ElectronJS, y da bastante miedo: he visto varias comparativas y supera con creces a éste último. El punto más fuerte de Tauri es que compila en código de máquina; no como Electron que hace una especie de paquete con NodeJS y arma un ejecutable híbrido, además de ensuciar bastante el proyecto. Pueden encontrar mis repositorios con el clicker implementado en [Electron](https://github.com/maxwellnewage/react-clicker) y [Tauri](https://github.com/maxwellnewage/tauri-clicker).

Encontré en Tauri una solución más limpia y robusta, pero existía un pequeño (gran) detalle: estaba desarrollada en Rust.

# Otro viaje de aprendizaje, el último, espero.

En el mundo del desarrollo, Rust es sinónimo de un "señor lenguaje de programación". La gente lo adora, especialmente en las vastas tierras de StackOverflow. Siempre me dió cierta curiosidad, pero en esas épocas mi objetivo era especializarme en Android y Kotlin.

Si bien se puede desarrollar una aplicación web con Tauri, y dejar que sus componentes hagan su trabajo como una caja negra; mi necesidad de saberlo todo no me dejaba en paz: tenía que conocer Rust, de la misma forma que todos los conductores deberíamos saber mecánica para entender cómo funciona un vehículo por dentro.

Como siempre, me dediqué a buscar artículos y videos de youtube. Dejo una lista de aquellos que me parecieron más interesantes:

- [Por qué tienes que aprender Rust](https://www.youtube.com/watch?v=-anm93UDkTk)

- [Rust esta mejorando el desarrollo web](https://www.youtube.com/watch?v=-anm93UDkTk)

- [Rust 101 Crash Course](https://www.youtube.com/watch?v=lzKeecy4OmQ): Este recurso en particular me encantó, nunca vi alguien que explicara tan bien los conceptos.

- [¿Salvará Rust el mundo? Parte 1](https://empresas.blogthinkbig.com/rust-errores-programacion-uso-memoria/): En el final del artículo esta el enlace a la segunda parte.

- [¿Qué es Rust?](https://www.youtube.com/shorts/QPhbR7oIOq8)

Luego de estudiar un par de días, me di cuenta que Rust era extremadamente difícil.

# Rust es difícil

Me encontré en una situación compleja: Rust se me antojaba esotérico a ratos, con una sintaxis muy propia del lenguaje que no había visto desde que dejé de programar en C o C++. Por ejemplo, esto es un "Hello World" con variables:

```rust
fn prints() {
    let mut name: &str = "Tomas";
    println!("Hello, {}!", name);
    name = "Max";
    println!("Hello, {}!", name);
}
```

En este caso, "let mut" define a _name_ como una variable mutable, y "&str" es algo parecido a un tipo String. Luego, "println!" tiene un signo de admiración porque se trata de una macro (la cual es distinta y parecida a una función, al mismo tiempo) y debemos utilizar llaves para representar valores por cómo funcionan los tipos.

Vengo de lenguajes de tipado dinámico (salvando quizá, Kotlin y Java), por lo cual encontrarme con lo que vamos a llamar "tipado muy fuerte", que al mínimo pasaje incorrecto deja de compilar, fue una pesadilla inicialmente.

La curva continuó subiendo a medida que aprendí sobre otros conceptos, como operadores:

```rust
fn ops() {
    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1 + 2,
        8 - 5,
        15 * 3
    );
    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9 / 2, 9.0 / 2.0);
    println!("9 / 2 = {}", 9f32 / 2f32);
}
```

Donde una división de enteros no podía generar un flotante, entonces necesitabas definirlo previamente (9 flotante de 32 bits).

Por otro lado, las tuplas eran similares a Python, pero muy extrañas en su definición:

```rust
fn tupla() {
    // Tuple of length 3
    let tuple_e: (char, i32, bool) = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );
}

```

El tipo "char" implica un valor de un solo caracter; mientras que "i32" implica un entero sin signo de 32 bits.

Estaba claro que Rust era un lenguaje de bajo nivel que tenía características que comparten otros lenguajes más contemporaneos. Esto me resultaba preocupante, dado que no manejaba este nivel de complejidad.

Por otro lado, **Rust no maneja el paradigma orientado a objetos**. Esto me dejó un poco atontado, porque venía trabajando con esta forma en todos los lenguajes que me he cruzado. Aquí estamos frente al uso de los paradigmas imperativo y funcional. Si queremos algo similar a los objetos, necesitamos utilizar cosas como struct e impl:

```rust

enum Color {
    Brown,
    Red,
}

impl Color {

    fn print(&self) {
        match self {
            Color::Brown =\u003E println!("brown"),
            Color::Red =\u003E println!("red")
        }
    }

}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -\u003E Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0
    };

    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);

    small_box.print();
}

```

En este ejemplo, _ShippingBox_ es una estructura de datos que posee su propia implementación, así como también pasa con _Color_. Es probable que los que estén más familiarizados con C, encuentren esto muy razonable; pero yo me crié (laboralmente) con lenguajes que aplicaban la POO, conjunto de sus patrones de diseño y buenas prácticas.

# Finalmente descubrí donde estaba el amor.

Si bien mi subtítulo es aplicable al título de una novela rosa, o un relato de Wattpad; la realidad es que finalmente encontré la clave de tanto amor a Rust: El concepto de Borrow, o "prestar".

A diferencia de C/C++, donde teníamos que manejar la memoria a mano, Rust nos ofrece un sistema inteligente de préstamo de recursos. Por ejemplo:

```rust

fn main() {
    let nums = vec![10, 20, 30, 40];

    for n in nums {
        match n {
            30 =\u003E println!("thirty"),
            _ =\u003E println!("{:?}", n),
        }
    }

    println!("Number of elements: {:?}", nums.len());
}

```

Aquí podemos ver cómo recorro un vector de números mediante un bucle for. En un lenguaje tradicional, pedir el número de elementos luego de recorrerlo no sería un problema; pero en nuestro caso va a fallar en la compilación.

Esto es porque la variable _nums_, luego de ser procesada en un bucle (o pasada por una función), se destruye. Por supuesto, en este caso deberíamos evitarlo, y lo hacemos mediante el símbolo "&":

```rust

for n in &nums {

```

Con este pequeño cambio, estamos pidiéndole a la función main (actual dueño de nuestro vector nums) que **preste** al bucle _for_ dicho vector. Ésta técnica podría compararse con un pasaje de datos por referencia.

Cuando nuestro bucle termina de utilizar su vector, lo devuelve a main() y se continúa su ejecución. Con este sistema de borrowing, podemos liberar memoria cuando realmente no necesitemos los recursos. Esto claramente es un digno competidor de los punteros en C; y supera con creces el Garbage Collector de lenguajes como Python, Java, C#, entre otros.

**Éste** es el verdadero potencial de Rust. Y no, no somos dignos de esta maravillosa tecnología.

# El trayecto final: La implementación.

Con el fin de aplicar todo lo que aprendí, me propuse armar un proyecto sencillo y que ya esta publicado en GitHub: [Rust Hero Game](https://github.com/maxwellnewage/rust-hero-game). Este proyecto utiliza Rust para acceder a la API de [Hero Game](https://github.com/maxwellnewage/udemy-django-hero-game) desarrollada en Django y DRF.

En primer lugar, definí los recursos que iba a utilizar: tenía una API, y el endpoint más sencillo era "/api/players/", un GET que obtenía los jugadores sin pedir autenticación:

```json
[
    {
        "id": 1,
        "name": "maxwell",
        "hp": 50,
        "money": 99,
        "score": 5,
        "owner": {
            "id": 1,
            "username": "admin",
            "is_author": true
        }
    }
]
```

Encendí el servidor y me puse a trabajar en un archivo llamado api.rs:

```rust
const BASE_URL: &str = "http://127.0.0.1:8001/api/";
async fn make_api_request(url: &str) -\u003E Result\u003Cserde_json::Value, Error\u003E {
    let resp = reqwest::get(url)
        .await?
        .json::\u003Cserde_json::Value\u003E()
        .await?;
    Ok(resp)
}
```

Definí una constante BASE_URL, la cual se explica por sí misma, y armé un método _make_api_request_, el cual toma un endpoint y devuelve un enum Result que se lleva serde_json::Value (si la cosa fue bien) y Error (si salió algo mal).

Utiliza async y await para detener los procesos en cada paso, pero seguir trabajando en los eventos asincrónicos como el llamado a la API y la conversión a json.

```rust
pub async fn get_all_players() -\u003E Result\u003CVec\u003CPlayer\u003E, ApiError\u003E {
    let url = &format!("{}{}", BASE_URL, "players/");
    match make_api_request(url).await {
        Ok(resp) =\u003E {
            match serde_json::from_value::\u003CVec\u003CPlayer\u003E\u003E(resp) {
                Ok(players) =\u003E Ok(players),
                Err(e) =\u003E Err(ApiError::from(e)),
            }
        }
        Err(e) =\u003E Err(ApiError::from(e)),
    }
}
```

Luego, mi función _get_all_players_ llama a _make_api_request_, pero procesa la respuesta para serializar el json en un struct Player:

```rust
pub struct Player {
    hp: i32,
    id: i32,
    money: i32,
    name: String,
    score: i32,
    owner: Owner
}
```

De esta manera, en main podemos trabajar con cada atributo como si fuera un "objeto":

```rust
async fn main() {
    match api::get_all_players().await {
        Ok(players) =\u003E {
            for player in players {
                println!("Jugador: {:?}", player);
            }
        }
        Err(e) =\u003E {
            eprintln!("Error al obtener los jugadores: {}", e);
        }
    }
}
```

Dentro de cada player del for (devuelve un array de players en json, vector en Rust) podríamos acceder a propiedades como el dinero mediante _player.money_.

Si corremos por consola el programa, obtendremos un resultado similar a este:

```
Finished dev [unoptimized + debuginfo] target(s) in 0.82s
Running `target\\debug\\hero-game.exe`
Jugador: Player { hp: 50, id: 1, money: 99, name: "maxwell", score: 5, owner: Owner { id: 1, is_author: true, username: "admin" } }
```

# Conclusiones

Creo que Rust es un excelente lenguaje de programación, creado bajo una muy buena idea acerca de cómo manejar los recursos en memoria. Al mismo tiempo, admito que no tengo las capacidades suficientes para dominarlo: puedo llevar un proyecto a cabo, pero he notado un grado de complejidad que me sobrepasa.

Actualmente, no logro comprender en detalle los errores que lanza, y casi todo el tiempo me encuentro preguntándole a ChatGPT qué estoy haciendo.

No obstante, esto también me sirve como una lección de humildad: todos los lenguajes que venía estudiando, los dominaba al mes. Hoy me cruzo con un gigante, y entiendo que parte de aprender se basa en la idea de admitir que no sabíamos todo; y que a veces hay que volver hacia atrás para tomar un envión más fuerte.

Quiero aclarar que no tengo quejas con Rust, sino más bien observaciones comparables a mis otros aprendizajes. Y por supuesto, entrará en el stack que implica hacer este juego.

En el próximo artículo, les contaré en detalle, a nivel mucho más técnico, sobre el clicker que me impulsó a aplicar Tauri y React.

