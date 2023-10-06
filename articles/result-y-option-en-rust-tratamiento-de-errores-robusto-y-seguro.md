---
title: "Result y Option en Rust: Tratamiento de errores robusto y seguro"
description: Cómo Rust hace que el manejo de errores sea seguro y robusto con los tipos Result y Option, evitando Null Pointer Exceptions y garantizando contratos sólidos. 
author: Phosphorus
github_user: Phosphorus-M
date: 2023-10-06
tags:
  - rust
  - error handling
  - exceptions
  - safe
social:
  github: https://github.com/Phosphorus-M
  website: https://phosphorus-moscu.gitlab.io
---


El lenguaje de programación Rust se ha destacado en la comunidad de desarrollo por su enfoque innovador en el manejo de errores. 



En lugar de depender de excepciones como en algunos lenguajes, Rust utiliza dos tipos especiales, `Result` y `Option`, para manejar situaciones de error de manera segura y eficiente. 

El manejo de errores es una parte esencial de la programación, ya que los errores pueden ocurrir en cualquier momento, desde la entrada de datos incorrectos hasta problemas en tiempo de ejecución.

En Rust, un lenguaje de programación diseñado para ofrecer seguridad y rendimiento, el tratamiento de errores se aborda de manera elegante y segura mediante el uso de los tipos `Result` y `Option`. En este artículo, exploraremos las ventajas que ofrecen estos dos tipos en Rust sobre otros tratamientos de error, como `try-catch` en lenguajes como Java y cómo se comparan con enfoques similares en otros lenguajes como Go. Además, discutiremos cómo estos enfoques pueden generar problemas como los Null Pointers, que son una preocupación común en lenguajes como Java o C#.



## Result y Option en Rust: Introducción

Antes de profundizar en las ventajas de Result y Option, es importante entender qué son estos tipos en Rust.

* **Result**: Es un tipo genérico que se utiliza para representar operaciones que pueden tener éxito o fallar. Tiene dos variantes: Ok para el caso de éxito y Err para el caso de error. Esto obliga al programador a manejar explícitamente los errores y evita las excepciones no controladas.

    ```rust
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err("División por cero".to_string());
        }
        Ok(a / b)
    }
    ```

    En este ejemplo veremos cómo se puede utilizar el tipo Result para representar una operación que puede fallar. En este caso, la función divide recibe dos números de tipo f64 y devuelve un Result<f64, String>. Si la operación es exitosa, devuelve un Ok con el resultado de la división. Si la operación falla, devuelve un Err con un mensaje de error.

    ```rust
    fn main() {
        let a = 10.0;
        let b = 0.0;
        let result = divide(a, b);
        match result {
            Ok(value) => println!("El resultado es {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }
    ```


* **Option**: Es otro tipo genérico que se usa para representar la posibilidad de que un valor sea opcional o nulo. Tiene dos variantes: Some para representar un valor presente y None para representar la ausencia de valor. Esto elimina la necesidad de utilizar valores nulos y ayuda a evitar Null Pointer Exceptions.

    ```rust
    fn find_element<T: PartialEq>(list: Vec<T>, target: T) -> Option<usize> {
        for (index, item) in list.iter().enumerate() {
            if *item == target {
                return Some(index);
            }
        }
        None
    }
    ```

    En este ejemplo, la función find_element recibe una lista de valores genéricos y un valor objetivo. Devuelve un Option<usize> que representa la posición del valor objetivo en la lista. Si el valor objetivo no está presente en la lista, devuelve None.

    ```rust
    fn main() {
        let list = vec![1, 2, 3, 4, 5];
        let target = 3;
        let result = find_element(list, target);
        match result {
            Some(index) => println!("El elemento está en la posición {}", index),
            None => println!("El elemento no está en la lista"),
        }
    }
    ```

    
## El Problema con las Excepciones

En muchos lenguajes de programación, las excepciones se utilizan para manejar errores y situaciones excepcionales. Aunque las excepciones pueden ser útiles en algunos casos, también tienen desventajas significativas. Algunos de los problemas comunes con las excepciones incluyen:

* **Incertidumbre**: Las excepciones pueden ocurrir en cualquier momento y en cualquier lugar, lo que hace que sea difícil predecir cuándo ocurrirán. Esto puede dificultar la depuración de errores y la comprensión del flujo de control del programa.

* **Rendimiento**: Las excepciones pueden tener un impacto significativo en el rendimiento de un programa. Esto se debe a que el sistema de excepciones debe realizar un seguimiento de la pila de llamadas, además de los objetos que se están creando y destruyendo. Esto puede ser especialmente problemático en lenguajes como Java o C#, donde las excepciones se utilizan para el control de flujo.

* **Inseguridad**: Las excepciones pueden ocultar errores de programación y hacer que sea difícil determinar qué salió mal. Esto puede conducir a comportamientos inesperados.

* **Manejo Forzado**: En algunos lenguajes, el manejo de excepciones es obligatorio, lo que puede llevar a un código inflado y complicado.

* **Excepciones no controladas**: En algunos lenguajes, las excepciones no controladas pueden provocar que el programa se bloquee o se cierre. Esto puede ser especialmente problemático en aplicaciones críticas para la seguridad, como los sistemas operativos o los navegadores web. En Java y C# por las características del lenguaje en el que un objeto puede ser nulo, se puede producir una Null Pointer Exception, que es una de las excepciones más comunes en estos lenguajes, debido a que es una excepción implícita.

* **Excepciones genéricas**: En algunos lenguajes, las excepciones se utilizan para representar una amplia gama de errores, lo que puede dificultar la depuración de errores y la comprensión del flujo de control del programa. Si bien las buenas practicas de programación recomiendan crear excepciones personalizadas, en la práctica esto no siempre se hace, así que se trata muchos problemas con una sola excepción. Casos como estos pueden encontrarse en los `try-catch` de código legacy, se repite mucho las palabras `Exception`, `RuntimeException` o `Error`.

* **Problemas de diseño**: Como mencione anteriormente algunos lenguajes como Java y C# permiten que un objeto sea nulo, lo que puede provocar Null Pointer Exceptions. Esto si bien podría considerarse un problema de diseño del lenguaje no queda unicamente en eso, ya que se puede usar las excepciones como un mecanismo de eventos, no solo para manejar errores, esto hace que programadores que no conocen las buenas practicas terminen usando las excepciones para todo.

Rust aborda estos problemas de manera efectiva mediante el uso de Result y Option como alternativas al manejo de excepciones, lo que permite un manejo de errores más seguro, eficiente, sin anti patrones, con contratos sólidos.

## Ventajas de Result y Option en Rust

* **Tratamiento explícito de errores**

    Uno de los mayores beneficios de usar `Result` y `Option` en Rust es que obligan al programador a manejar explícitamente los errores. Esto significa que no se pueden ignorar los errores o dejar que pasen desapercibidos. En contraste, en lenguajes como Java o C#, es común que los errores se ignoren o se capturen de manera inadecuada con bloques `try-catch`, lo que puede llevar a problemas no detectados en tiempo de ejecución. Haciendo una comparativa con Go, es común utilizar valores nulos (`nil`) para representar la ausencia de un valor, lo que puede conducir a errores si no se manejan adecuadamente, al mismo tiempo se intenta conseguir más información de distintas maneras porque el error por default no es muy descriptivo.
    En Rust, podemos utilizar el tipo `Result<T>` para encapsular tanto el valor como el error, mismo caso usando `Option`. Errores específicos y descriptivos, que nos permiten saber que salió mal y por qué una firma segura, un contrato que es el mismo objeto `Result` y `Option`.

* **Eliminación de Null Pointers**

    En Java y C#, todas las variables pueden ser de valor `null`, lo que lleva al infame `NullPointerException`. En Rust, el uso de `Option` para representar valores opcionales elimina esta fuente común de errores. Si un valor es de tipo `Option`, el programador debe manejar explícitamente los casos de `Some` y `None`, lo que garantiza un manejo seguro de los valores nulos.

    Rust es considerado por este motivo un lenguaje null safety. 

* **Evita violaciones de contratos**

    En Rust, los contratos entre funciones y sus llamantes son más sólidos gracias a `Result`. Cuando una función devuelve un `Result`, se establece claramente que la operación puede fallar, lo que obliga al llamante a manejar este caso. En lenguajes como Java o C#, donde no existe esta convención, es más fácil que se violen los contratos y se produzcan errores inesperados.

    Rust es un lenguaje de tipado estático, por lo que el compilador puede verificar que se manejen todos los casos posibles, lo que garantiza que no se produzcan errores inesperados.
    La gran mayoría de los lenguajes de programación son de tipado dinámico, por lo que no se puede garantizar que se manejen todos los casos posibles, no suelen haber muchas comprobaciones de contrato, en el caso de Rust la inmensa mayoría de ellos son comprobados por el compilador, lo que garantiza tener un código limpio de errores para cuando se ejecute.

* **Rendimiento mejorado**

    A diferencia de las excepciones, Result y Option son estructuras de datos simples y predecibles en términos de rendimiento. No hay gastos adicionales asociados con la creación o manipulación de excepciones.

    En Rust, el uso de `Result` y `Option` en lugar de excepciones puede mejorar el rendimiento de un programa. Esto se debe a que no hay necesidad de realizar un seguimiento de la pila de llamadas o de crear y destruir objetos. Además, el uso de `Option` en lugar de valores nulos puede mejorar el rendimiento al eliminar la necesidad de comprobar si un valor es nulo.

    Nuestro primer vistazo si venimos del mundo funcional sera que `Result` y `Option` no son más que monadas, en efecto. 
    Pero si venimos del mundo imperativo de la vida tendremos que entrar en detalle, entenderemos que los `Result` y `Option` no son más que `Enums`.
    Enums de Rust, Enums poderosos, vitaminados, pero finalmente `Enums` y es por esa simpleza que hace que los errores en Rust sean más eficientes que en otros lenguajes.

    Internamente los posibles valores son o 0 o 1, dos valores, no hay más, y como son `Enums` de Rust, estos a su vez pueden tener valores asociados, lo que hace que sean más poderosos que los `Enums` de otros lenguajes, de esa forma asociamos las variantes y los valores que tienen dentro.

* **Código más Limpio y Consciente de Errores**

    Result y Option hacen que el código sea más claro al indicar de manera explícita los puntos en los que pueden ocurrir errores. Esto facilita la lectura y comprensión del código.

    Ejemplos de esto esta lleno, casos tan simples como la propagación de errores usando `?` al final de una linea que pueda dar `Err`:

    ```rust
    use std::result::Result;

    fn fetch_data(url: &str) -> Result<Vec<u8>, &str> {
        // ...
    }

    fn process_data(data: Vec<u8>) {
        // ...
    }

    fn main() -> Result<(), &str> {
        let url = "https://example.com";
        let data = fetch_data(url)?;
        process_data(data);
        Ok(())
    }
    ```

    Aquí, la función `fetch_data` devuelve un Result que contiene tanto los datos como el error. La función `process_data` también devuelve un Result, ya que puede fallar si los datos son inválidos. En este caso optamos por propagar el error usando `?`, lo que significa que la función `main` también devuelve un Result. Esto hace que sea más fácil para el programador saber dónde pueden ocurrir errores y cómo manejarlos.

    Pero si queremos una forma más funcional para trabajarlo podría ser:

    ```rust
    use std::result::Result;

    fn fetch_data(url: &str) -> Result<Vec<u8>, &str> {
        // ...
    }

    fn process_data(data: Vec<u8>) {
        // ...
    }

    fn main() -> Result<(), &str> {
        let url = "https://example.com";
        fetch_data(url)?.and_then(process_data);
        Ok(())
    }
    ```

* **Ausencia de Bloques `Try-Catch` Anidados**

    A diferencia de los lenguajes que utilizan bloques `try-catch` anidados para manejar excepciones, Rust evita esta complejidad y mantiene un flujo de control más lineal y legible.

    Tenemos que entender que las sentencias en Rust nos dan lugar a jugar con otras cosas, todo es una expresión y nosotros podemos organizar los errores creando en un Enum todos los posibles errores de nuestra aplicación, de esa forma podemos manejarlos de manera más sencilla, sin necesidad de usar `try-catch` anidados.

* **Patrones de concurrencia más seguros**

    Rust con un mecanismo tan simple ha logrado que el manejo de errores sea más seguro, pero también ha logrado que el manejo de concurrencia sea más seguro, ya que los `Result` y `Option` son `Send` y `Sync` por defecto, lo que hace que el manejo de concurrencia a su vez también sea seguro, ya que no se puede compartir un `Result` o `Option` entre hilos, los errores hay que tratarlos siempre, no se pueden propagar libremente sin tratarlos en algún punto.



* **Exhaustividad**

    Cuando se utiliza Result o Option en Rust, el compilador garantiza que todos los caminos de ejecución estén cubiertos en términos de manejo de errores, decimos que es exhaustivo por esta facultad que tiene el compilador de analizar todos los caminos posibles. Esto hace que sea más fácil detectar y corregir errores en etapa de compilación.
    Ahorra mucho tiempo de depuración, esta Exhausitividad es una de las mejores características de Rust, no aplica unicamente al manejo de errores, sino a todo el lenguaje, el compilador es muy inteligente y puede detectar errores que en otros lenguajes no se pueden detectar hasta que se ejecuta el programa.

    Con el `Pattern Matching` del lenguaje se pueden encontrar bugs donde ningún otro lenguaje los encuentra. 

    Esta exhausitividad es uno de los factores que determinan el porque Rust proporciona mejor manejo de errores que Go por ejemplo, la falta de información en el manejo de errores en Go hace que no se tenga esta característica en el lenguaje. La exhaustividad impide que el usuario al extender el lenguaje pueda romper la seguridad del lenguaje, el contrato de se respeta desde el compilador hasta el código que le llega al usuario, no hay excepciones.

## Conclusiones

Result y Option son características distintivas de Rust que ofrecen ventajas significativas en términos de manejo de errores, seguridad, extensibilidad y rendimiento. Al eliminar las excepciones y promover el manejo explícito de errores, Rust proporciona una forma más robusta y predecible de lidiar con situaciones excepcionales en la programación. Esta innovación hace que Rust sea una elección sólida para proyectos donde la fiabilidad y el control sobre el manejo de errores son fundamentales.

Tanto es así que recientemente se han sacado compiladores incluso más estrictos de Rust para sistemas tiempo real y de misión critica, el lenguaje da la posibilidad de extenderse en el manejo de errores porque provee toda la información que podemos requerir para desde lo existente tener sistemas más seguros si es que quisiéramos eso.

El manejo de errores en Rust es posible simplificarlo incluso, sin necesidad de remover la exhaustividad. Se puede crear una macro que nos permita manejar los errores de manera más sencilla o al mismo tiempo este mecanismo de errores de tipo Enum les podemos agregar manejos de auto tratamiento.

Por todo esto y más Rust es un lenguaje que vale la pena aprender, es un lenguaje que nos permite hacer cosas que otros lenguajes no pueden, es un lenguaje que nos permite tener un código más limpio, más seguro, más eficiente, entre muchas otras cosas más, todo esto es lo que me hace pensar que Rust es un lenguaje Robusto y Seguro.