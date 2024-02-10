---
title: Variables y declaraciones en Rust
description: La declaración de variables es algo fundamental en el aprendizaje de Rust, en este articulo abordaremos una guía rápida.
author: Fernando Pastorelli
github_user: Phosphorus-M
date: 2024-02-10
tags:
- rust
- data-type
- roadmap
- principiante
social:
  github: https://github.com/Phosphorus-M
  twitter: https://twitter.com/Phosphorus_M
  website: https://phosphorus-moscu.gitlab.io
---

En Rust a la hora de programar usaremos infinidad de veces algo llamado variables.

Una variable es un contenedor que almacena un valor o información en un programa de computadora. Estos valores pueden ser números, texto, lógicos como verdadero o falso, u otros tipos de información.

Veamos el siguiente ejemplo.
```rust
let mi_numero = 2;
``` 

En Rust para crear nuevas variables usamos la palabra clave `let`, con esa palabra crearemos a todas las variables existentes. 
Como se observa seguido de la palabra clave `let`se le dara un nombre a la variable, un identificador.
Ese identificador usaremos en el resto del código para referirnos a esa variable.
Seguido usaremos el `=`para asignar un valor inicial.
En este caso 2.

Cuando usemos la variable `mi_numero` obtendremos el valor 2.

Otra cosa que nos podría interesar es cambiar el valor de la variable a medida que lo usemos.
Eso debido a que quizás queremos re utilizar la misma variable.
Por ejemplo quiero usar la variable para guardar mi numero preferido.
Inicialmente era 2, pero ahora cambie de opinion.
Eso se reflejaría en el código de la siguiente manera

```rust
let mut mi_numero = 2;
// ... más código
mi_numero = 7;
```

Ahora se cambio, `mi_numero`pasa a ser 7.
Eso se logro porque nosotros usamos previamente, en la declaración de la variable como mut.
Es decir como mutable.
En la programación un problema común es que los desarrolladores crean variables y suele re utilizarlas bastante sin saber su uso en otro momento.
Por lo que inicialmente se espera que tenga un valor pero en entre un punto y otro alguien modifica ese valor, eso crea un error, se esperaba que ese valor permanezca con el estado inicial a lo largo de su ejecución.
En Rust, para solucionar ese problema se tomo la medida de que toda variable sea inmutable por defecto.
Es decir, a menos que no sea declarada para que sea modificada por futuros programadores esa variable no puede cambiar de valor.

Además de esto Rust infiere el tipo de dato.
En muchos lenguajes a la hora de crear una variable se debe definir el tipo de dato, es decir que tipo de valor va a contener la variable, numérico, de texto o booleano.
¿Por que? Porque perfectamente se podría tener algo como:
```rust
let mi_numero = "2";
```
Eso causaría confusión posiblemente en algunas personas, ¿Debo entender que  `mi_numero ` es realmente un numero o es un texto que tiene el numero?
Peor aún si esto lo cambiamos por 1 (representación numérica de verdadero), por lo que podría causar confusión.
Estos pueden ser casos muy pequeños pero entre más se complejiza el código y hay nuevos tipos de datos, se requiere más y más definir como debemos interpretar una variable.

Rust para resolver esto tiene la inferencia.
La inferencia significa que todos los valores en Rust tienen uno o muchos tipos por defecto.
Eso significa que para casos muy simples como:
```rust
let mi_numero = 2;
```
No habría necesidad de tipar.
Tipar es opcional. 
Pero en Rust hay varios tipos de datos numéricos, por defecto los enteros son de tipo `i32`.
Quiere decir Integer (Entero en español) de 32 bits, esto cobrara sentido más adelante..
Para definir un tipo usamos los `:` seguido del tipo de dato que seria.
```rust
let mi_numero:i32 = 2;
```
No es necesario hacer esto para este caso el `i32` no tendría mucho sentido tiparlo pero podríamos desear que sea de tipo `u8` en lugar de `i32`.
¿Por qué? Eso lo veremos cuando veamos esos tipos de datos.

Lo que podemos decir es que de momento para hacerlo de tipo `u8` podríamos usar:
 ```rust
let mi_numero:u8 = 2;
```
De esta forma si bien el valor seria el mismo el tipo de dato de `mi_numero` habría cambiado.
Otra cosa que podría resultar interesante es el uso de :

### const

En Rust la palabra clave `const` se utiliza para declarar constantes.
Al igual que las variables son inmutables pero a diferencia de ellas no se pueden cambiar a mutables.
Y a diferencia de las variables cuando se usa una constante siempre debemos definir el tipo de dato que tendrá.
Veamos el siguiente ejemplo

```rust
const MAYORIA_DE_EDAD: u8 = 18;

fn main() {
    println!("En este país la mayoría de edad es {}", MAYORIA_DE_EDAD);
}

```
Analicemos lo que esta sucediendo.
Estamos declarando una constante fuera del main.
Eso es algo que no hemos hablado.
Las variables suelen ser declaradas en el ámbito de una función.

No pueden haber variables sueltas.

En el caso de las constantes se puede declarar fuera de una función.

Otra característica que quizás notaremos es que las constantes se escriben en mayúsculas.

Y nos puede llamar la atención que la constante puede ser usada en cualquier lado del código. Sin hacer nada raro, tiene lo que se considera un scope (ámbito) global en nuestro código.

Quizás el uso más avanzado del que podremos llegar a ver sera el uso de `const` para hacer evaluaciones en tiempo de compilación y quizás sea rutinario ver `const`para valores que nunca, jamás cambiaran. Ni si quiera con un concepto más avanzado como la mutabilidad interna.

### static

Por ultimo quizás debamos de ver el uso de la palabra `static`.
El uso de `static` es justamente para crear valores que van a perdurar toda la vida de la ejecución.
Esto quizás se vea más en profundidad cuando veamos **Lifetimes** de momento retengan en mente el siguiente ejemplo.

```rust
// Declara una variable estática global
static GLOBAL_VARIABLE: u8 = 42;

fn main() {
    // Accede a la variable global
    println!("El valor de la variable global es: {}", GLOBAL_VARIABLE);
}
```

Si bien podríamos creer que es el mismo caso que una constante, en Rust el uso de `static` habilita la creación de variables globales, al igual que const deberemos de tipar la variable.
Acabo de decir variables globales por que a diferencia de las `const` estas pueden ser modificadas, inicialmente tendrán un valor inmutable, pero nosotros podríamos usar `mut`para volverlas mutable.
Sin embargo esto es desaconsejado.
A lo largo de los años, la experiencia de muchos desarrolladores ha demostrado que esto es una mala practica.

Rust sin embargo permitirá la modificación de la variable global pero bajo uso de `unsafe`. 
¿Qué es `unsafe`? Lo veremos más adelante.
Hay otra manera que Rust permite la modificación de variables globales.
Esta vez si es un uso recomendado por desarrolladores expertos que es usando Mutabilidad Interna. 

Otro concepto que veremos a lo largo del Roadmap, en el blog y demás.

Desde ya muchas gracias por leer.