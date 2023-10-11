---
title: String en Rust
description: El manejo de texto es algo muy importante en cualquier tipo de aplicacion, por lo que conocer los tipos de datos que proporiciona el lenguaje es muy importante, ademas de saber como poder manejar los datos de manera eficiente
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
Aunque parezca raro leer que Rust tiene mas de un tipo de string, déjame te cuento a que se refiere.

Primero tenemos que partir por la premisa de:

## Que es un String?
Bueno un `String` en la mayoría de los lenguajes de programación, es una secuencia de caracteres o símbolos que se utiliza para representar texto o información legible por humanos en un programa. Estos caracteres pueden ser letras, números, símbolos especiales, espacios en blanco, emojis, etc. Los Strings son fundamentales en la programación, ya que se utilizan para manejar y manipular texto, desde mensajes en una aplicación hasta contenido web y datos de usuario.

Los Strings suelen ser objetos o tipos de datos específicos en un lenguaje de programación y pueden tener diversas propiedades y características. Algunas de las características comunes de los Strings incluyen:

1. Inmutabilidad: En algunos lenguajes, los Strings son inmutables, lo que significa que una vez que se crea un String, no se puede modificar. En su lugar, cualquier operación que cambie el String crea uno nuevo.

2. Longitud: Los Strings tienen una longitud que representa la cantidad de caracteres que contienen. Puedes obtener la longitud de un String mediante una función o método proporcionado por el lenguaje.

3. Concatenación: Puedes combinar o unir Strings para crear uno nuevo más largo. Esto se llama concatenación y se utiliza para crear mensajes o texto compuesto.

4. Acceso a caracteres: Puedes acceder a caracteres individuales dentro de un String utilizando índices o posiciones. Algunos lenguajes comienzan a contar desde 0, lo que significa que el primer carácter tiene un índice 0, el segundo tiene un índice 1 y así sucesivamente.

5. Manipulación: Los Strings a menudo tienen métodos o funciones incorporados que permiten realizar diversas operaciones, como búsqueda y reemplazo de texto, conversión de mayúsculas a minúsculas y viceversa, y más.

Para permitir la representación multilenguaje a nivel mundial existe un estándar de codificación de caracteres llamado Unicode y su variante UTF-8 (Unicode Transformation Format - 8 bits) las cuales permiten representar símbolos y caracteres de todos los idiomas del mundo

Antes de ver los Strings en Rust, tenemos que ver los strings en otros lenguajes para poder apreciar el valor que propone Rust en sus implementaciones

## Cadenas de caracteres en C
En C, las cadenas de caracteres son arreglos de caracteres que terminan con el carácter nulo '\0'. Este enfoque es propenso a errores, ya que no se realiza un seguimiento explícito de la longitud de la cadena, lo que puede llevar a desbordamientos de búfer y problemas de seguridad.

1. No se Realiza un Seguimiento de la Longitud: A diferencia de los lenguajes de programación modernos, C no realiza un seguimiento automático de la longitud de las cadenas. Esto significa que debes usar funciones como `strlen()` para determinar la longitud de una cadena antes de manipularla. Si olvidas hacerlo o calculas incorrectamente la longitud, puedes introducir errores de acceso a memoria no válida.

2. No es Seguro contra Desbordamientos: La función `strcpy()` en C, que se utiliza para copiar una cadena en otra, no realiza comprobaciones de límites. Si la cadena de origen es más larga que la de destino, se producirá un desbordamiento de búfer, lo que puede ser explotado por un atacante.

Aquí un pequeño ejemplo:

```c
char destination[10];
char source[] = "Esta cadena es demasiado larga para el destino";
strcpy(destination, source);  // Desbordamiento de búfer
```

3. Dificultades con Caracteres Multibyte: C no maneja naturalmente caracteres multibyte, lo que puede ser problemático en aplicaciones internacionales que requieren soporte para varios idiomas. La manipulación de caracteres multibyte puede ser propensa a errores y no es trivial en C.

4. Falta de Abstracciones de Alto Nivel: C carece de abstracciones de alto nivel para trabajar con cadenas de caracteres, como las que ofrecen lenguajes más modernos. Esto hace que sea más fácil cometer errores y dificulta la escritura de código seguro y legible.

## Cadenas de Caracteres en Rust
Rust ofrece varios tipos de cadenas de caracteres que pueden adaptarse a diferentes necesidades, lo que hace que trabajar con texto sea seguro y eficiente. 

En Rust, las cadenas de caracteres son una colección de caracteres Unicode, lo que significa que pueden representar una amplia gama de idiomas y símbolos. A diferencia de otros lenguajes de programación, Rust ofrece varios tipos de cadenas de caracteres para abordar diferentes casos de uso:

- &str: Este tipo representa una "sección" de una cadena de caracteres. Es una referencia a un segmento de memoria que contiene texto. Las cadenas &str son inmutables y se utilizan principalmente para referenciar datos de texto existentes y suele ser el tipo de dato por defecto cuando declaramos una cadena de texto explicita en el programa.
    ```rs
    let name: &str = "Sergio Ribera"; // Cadena de texto de tipo &'static str
    ```

- String: Este tipo representa una cadena de caracteres de propiedad (owned). Se trata de una cadena de texto que es propiedad exclusiva del programa. Puedes modificar una cadena String, añadir o eliminar caracteres, lo que la hace útil para construir y manipular cadenas de texto. Aunque parezca raro es mejor pensar en este tipo de dato como un `Vec<str>` ya que es un arreglo de caracteres alojados en la memoria dinámica (Heap).
    ```rs
    let name: String = String::from("Sergio Ribera");
    // El programa se encargará automáticamente de liberar la memoria utilizada por `name` cuando ya no sea necesaria, generalmente cuando la variable sale de ámbito.
    ```

- str: Es el tipo de dato más general y se utiliza para representar una referencia a una cadena de caracteres sin asignación específica. Es el tipo subyacente de &str y se utiliza raramente directamente en el código.

> NOTA: Cuando se dice que una cadena de caracteres es de "propiedad" u "owned" en el contexto de Rust, significa que esa cadena está bajo el control exclusivo del programa y es responsable de su gestión de memoria. En otras palabras, la cadena de caracteres es propiedad del programa y se encargará de liberar automáticamente la memoria asignada a la cadena una vez que ya no sea necesaria. Esto es una parte fundamental del sistema de gestión de memoria de Rust y es una de las características clave que lo hacen seguro y eficiente.
> En Rust, las cadenas de caracteres de propiedad se representan con el tipo de dato String. Cuando creas una cadena String, estás asignando y administrando explícitamente la memoria necesaria para almacenar la cadena y su contenido. Esto permite que el programa realice operaciones de modificación en la cadena, como agregar o quitar caracteres, sin correr riesgo de desbordamientos de búfer o corrupción de memoria.

## Problemas con el String
Si, aunque parezca raro leer que Rust pueda tener problemas con un tipo de dato, en realidad esto se refiere mas al mal uso que pueda existir, por eso te comento algunos problemas comunes que suele haber al respecto:

1. Consumo de Memoria: El tipo `String` en Rust es dinámico y crece automáticamente para acomodar el texto. Si no se administra cuidadosamente, esto puede llevar al consumo excesivo de memoria. Al crear y manipular múltiples Strings grandes, podrías agotar la memoria disponible.
    ```rs
    fn main() {
        let mut big_string = String::new();
        for _ in 0..10000 {
            big_string.push_str("Texto grande ");  // Crecimiento automático
        }
    }
    ```

2. Copias Innecesarias: La copia de Strings puede ser costosa en términos de tiempo y memoria. Si copias una String cuando no es necesario, puedes incurrir en una sobrecarga de rendimiento. Por ejemplo, si clonas una cadena cuando podrías haber trabajado con una referencia prestada (&str), se realizará una copia innecesaria.
    ```rs
    fn main() {
        let original = "Texto original".to_string();
        let copied = original.clone();  // Copia innecesaria
    }
    ```

3. Fragmentación de la Memoria: La asignación y liberación frecuentes de Strings grandes pueden provocar fragmentación de la memoria. Esto puede afectar negativamente al rendimiento general del programa y al uso de la memoria.
    ```rs
    fn main() {
        let mut large_strings = Vec::new();
        for _ in 0..1000 {
            let new_string = "Texto grande".to_string();
            large_strings.push(new_string);  // Asignación y liberación de new_string
        }
    }
    ```

4. Textos innecesarios en el codigo: Si vienes de otros lenguajes muy probablemente sea una practica muy comun, pero en Rust tenemos otros tipos de datos que quizas puedan ser mejor en ciertas situaciones.
    ```rs
    fn main() {
        let type_account: &str = "PERSONAL"; // creamos y almacenamos un str estatico
        match type_account {
            "PERSONAL" => todo!(), // creamos y almacenamos un str estatico para la comparacion
            "SHARED" => todo!(), // creamos y almacenamos un str estatico para la comparacion
            "BUSINESS" => todo!(), // creamos y almacenamos un str estatico para la comparacion
            _ => todo!(),
        }

        // Para este caso concreto lo ideal seria usar un enum, ya que ocupa mucha menos memoria y tiene mejores implementaciones para estos casos
    }
    ```

## Dame soluciones no problemas
Paso a paso, primero necesito que entiendas los problemas que pueden existir en la manipulación de textos.
Ahora que ya viste los tipos de strings que maneja Rust y los problemas que pueden existir, veamos algunas estrategias para abordar estos problemas y optimizar el manejo de grandes cantidades de texto en Rust.

- Usa Referencias (&str) cuando sea posible: Cuando no necesitas modificar una cadena, utiliza referencias a cadenas de caracteres (&str) en lugar de clonar (String). Esto evita copias innecesarias y reduce el consumo de memoria.

- Utiliza Cow<'a, str>: Cow te permite trabajar con referencias prestadas o datos clonados según sea necesario, lo que puede ser útil al procesar texto dinámico.

- Usa la asignación cuidadosa de capacidad: Al crear Strings, puedes asignar una capacidad inicial para evitar asignaciones de memoria excesivas. Esto se hace utilizando el método .with_capacity().

- Recicla y reutiliza Strings: Si necesitas crear y desechar muchas Strings en un bucle, considera reutilizar Strings existentes para reducir la asignación de memoria.

- Optimiza las operaciones de cadena: Al realizar operaciones de cadena, como concatenación, utiliza métodos que minimicen las copias, como push_str() o push() en lugar de + o format!().
