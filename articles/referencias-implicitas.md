---
title: Referencias Implicitas
description: Una ventiladita al problema de las referencias Implicitas
author: Fernando Pastorelli
github_user: Phosphorus-M
date: 2026-06-09
tags:
- rust
- anecdota
- rusty solutions
- hartazgo
social:
  github: https://github.com/Phosphorus-M
  twitter: https://twitter.com/Phosphorus_M
  website: https://phosphorus-moscu.gitlab.io
---

# Referencia implícitas 

Estoy harto de ver bugs debido a una referencia implícita.

No es algo que sea frecuente necesariamente pero cuando lo sabes el peligro está ahí, no puedes ignorar ese caso.

Hoy a la mañana ví como un chico cometía un error por el uso del método `slice` de los arreglos de Javascript. En JS si bien no es ultra frecuente varios métodos y sintaxis pueden ocasionar un shallow copy, una copia de referencia apuntando al mismo sitio.

Mi experiencia con estos tipos de errores me hace explicar cuál está siendo el problema en la Kata que estaba resolviendo, una tontería pero al modificar las referencias el problema escalaba y no se sabía dónde estaba el origen del caos informático.

El shallow copy de JS hace una distinción, en dónde copia valores primitivos pero tipos complejos terminan siendo referencias.

Por lo que algo como:

```js
const original = [1, 2, 3];
const copia = original.slice();
```

Termina efectivamente creando una copia total de los datos, todas alocaciones nuevas.

Pero si hacemos algo como:

```js
const original = [{value: 1}];
const copia = original.slice();

copia[0].value = 42;

```

Entonces veremos qué el primer elemento del arreglo original termina siendo modificado también. En este caso es completamente obvio pero realmente si no sabes concretamente como funciona o te funcionó una vez y luego no lo volviste a usar puedes ignorar completamente este comportamiento condicional de su implementación.

En el caso de Go sucede algo similar, en Go no tenemos el comportamiento condicional pero es un misconception igual de grande. Podemos asumir que será un arreglo nuevo.

Algo como;

```go
original := []int{1, 2, 3, 4}
copia := original[1:3]
```

La forma en que lo genera Go es una referencia directa al array original pero limitado y con un offset. Definimos el rango y ya.

En un pseudo Rust seria algo como esto la implementación:

```rust
struct Slice<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
```

Además en el caso de Go no tenemos lo de una referencia mutable y n inmutables, todo es mutable por eso el puntero es `*mut T`, lo cual es una declaración unsafe en Rust.

Doy el caso de Go pero Python sufre del mismo problema.

```python
a = [1, 2, 3]
b = a
```

¿Se copio?

No :)

A y b ahora apuntan al mismo array pero son dos referencias vivas.

En el caso de Rust si intentamos hacer algo similar nos encontraremos con que el valor fue movido.

Si queremos una referencia debemos explicitarlo con el `&` por lo que no ocurrirían estos problemas.

Además de que la copia como dijimos antes tiene permisos de solo lectura en este caso, si necesitaríamos romper la regla del ownership deberíamos de llegar a usar el `unsafe` y terminar jugando con punteros, es mucho seguro y nos impide caer en problemas accidentales, además de que las reglas del ownership previenen problemas más complejos.

En muchos lenguajes no sabemos lo que pasa dentro de una función/método

Si tenemos algo como:

```java
hacer_algo(user)
```

¿Quizás diremos que lenguaje es? O ¿Qué es lo que hace la función?

La respuesta es que en prácticamente cualquier lenguaje este problema existe y es que no sabemos que pasa internamente, Rust no soluciona esto completamente pero nos da la herramienta de que si va a utilizar una referencia podemos pedir desde la firma justamente eso, que sea un pasaje por referencia, o por valor, o referencia mutable, y también verificar con el sistema de tipos que algo no sea concurrente entre otras muchas tareas.

```rust
fn hacer_algo(user: &User) {
    // ...
}
```

Por lo que al usarlo estaremos obligados sintácticamente a pasar por referencia, está visible el que tipo de operación es.

Todo esto en estos ejemplos sencillos parece como no relevante pero en casos donde se maneja herencia o se retornan variables que no sabes si son referencias o valores en si mismo, esto empieza a ganar una complejidad adicional imperceptible de ver al comienzo por un novato, esto es debido a los famosos unsoundness errors, los problemas que no detectamos debido a que suceden por debajo de la vista, requieren este conocimiento y tener en cuenta esto, que luego de miles de líneas en un proyecto suelen verse como meras curiosidades, jamás recordamos estás cosas porque nuestras horas programando sin problemas suelen opacar este tipo de casos bordes en los que uno nunca sufrirá algo así pero termina ocurriendo.

Es por eso que quería hacer está publicación, mostrar cómo Rust resuelve de base uno de los problemas más recurrentes en la programación.

Seguramente haga otros blog posts mostrando otras situaciones similares que me hacen apreciar más Rust.

Al final es por este tipo de casos en que decido divulgar acerca del lenguaje, porque es un lenguaje que soluciona este tipo de tonterías y creo que merece ser catalogado como software que suele tener una mejor calidad técnica que los demás, tipo de prevención que nos da el lenguaje es justo lo que necesitamos para hacer buen software, no desde un warning, desde un diseño de lenguaje que nos acompañará y se volverá más intuitivo en cada ocasión que lo usemos.