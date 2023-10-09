---
title: Todos los tipos de string en Rust
description: Aunque parezca raro leer que Rust tiene mas de un tipo de string, dejame te cuento a que se refiere.
author: SergioRibera
github_user: SergioRibera
date: 2023-09-21
tags:
- rust
- data-type
social:
  github: https://github.com/SergioRibera
  twitter: https://twitter.com/sergioribera_rs
  website: https://bento.me/sergioribera
---
Aunque parezca raro leer que Rust tiene mas de un tipo de string, dejame te cuento a que se refiere.

Primero tenemos que partir por la premisa de:

## Que es un String?
Bueno un `String` en la mayoría de los lenguajes de programación, es una secuencia de caracteres o símbolos que se utiliza para representar texto o información legible por humanos en un programa. Estos caracteres pueden ser letras, números, símbolos especiales, espacios en blanco, emojis, etc. Los Strings son fundamentales en la programación, ya que se utilizan para manejar y manipular texto, desde mensajes en una aplicación hasta contenido web y datos de usuario.

Los Strings suelen ser objetos o tipos de datos específicos en un lenguaje de programación y pueden tener diversas propiedades y características. Algunas de las características comunes de los Strings incluyen:

1. Inmutabilidad: En algunos lenguajes, los Strings son inmutables, lo que significa que una vez que se crea un String, no se puede modificar. En su lugar, cualquier operación que cambie el String crea uno nuevo.

2. Longitud: Los Strings tienen una longitud que representa la cantidad de caracteres que contienen. Puedes obtener la longitud de un String mediante una función o método proporcionado por el lenguaje.

3. Concatenación: Puedes combinar o unir Strings para crear uno nuevo más largo. Esto se llama concatenación y se utiliza para crear mensajes o texto compuesto.

4. Acceso a caracteres: Puedes acceder a caracteres individuales dentro de un String utilizando índices o posiciones. Algunos lenguajes comienzan a contar desde 0, lo que significa que el primer carácter tiene un índice 0, el segundo tiene un índice 1 y así sucesivamente.

5. Manipulación: Los Strings a menudo tienen métodos o funciones incorporados que permiten realizar diversas operaciones, como búsqueda y reemplazo de texto, conversión de mayúsculas a minúsculas y viceversa, y más.

Para permitir la representación multilenguaje a nivel mundial existe un estándar de codificación de caracteres llamado Unicode y su variante UTF-8 (Unicode Transformation Format - 8 bits) las cuales permiten representar simbolos y caracteres de todos los idiomas del mundo

Antes de ver los Strings en Rust, tenemos que ver los strings en otros lenguajes para poder apreciar el valor que propone Rust en sus implementaciones

## Cadenas de caracteres en C
En C, las cadenas de caracteres son arreglos de caracteres que terminan con el carácter nulo '\0'. Este enfoque es propenso a errores, ya que no se realiza un seguimiento explícito de la longitud de la cadena, lo que puede llevar a desbordamientos de búfer y problemas de seguridad.

1. No se Realiza un Seguimiento de la Longitud: A diferencia de los lenguajes de programación modernos, C no realiza un seguimiento automático de la longitud de las cadenas. Esto significa que debes usar funciones como strlen() para determinar la longitud de una cadena antes de manipularla. Si olvidas hacerlo o calculas incorrectamente la longitud, puedes introducir errores de acceso a memoria no válida.

2. No es Seguro contra Desbordamientos: La función strcpy() en C, que se utiliza para copiar una cadena en otra, no realiza comprobaciones de límites. Si la cadena de origen es más larga que la de destino, se producirá un desbordamiento de búfer, lo que puede ser explotado por un atacante.

Aquí un pequeño ejemplo:

```c
char destination[10];
char source[] = "Esta cadena es demasiado larga para el destino";
strcpy(destination, source);  // Desbordamiento de búfer
```

3. Dificultades con Caracteres Multibyte: C no maneja naturalmente caracteres multibyte, lo que puede ser problemático en aplicaciones internacionales que requieren soporte para varios idiomas. La manipulación de caracteres multibyte puede ser propensa a errores y no es trivial en C.

4. Falta de Abstracciones de Alto Nivel: C carece de abstracciones de alto nivel para trabajar con cadenas de caracteres, como las que ofrecen lenguajes más modernos. Esto hace que sea más fácil cometer errores y dificulta la escritura de código seguro y legible.
