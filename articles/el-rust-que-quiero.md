---
title: El Rust que quiero
description: Estos cambios lo cambian todo, son salvajes 🔥🔥
author: Fernando Pastorelli
github_user: Phosphorus-M
date: 2026-04-02
tags:
- rust
- opinion
- comptime
- reflection
- macros
social:
  github: https://github.com/Phosphorus-M
  twitter: https://twitter.com/Phosphorus_M
  website: https://phosphorus-moscu.gitlab.io
---

Hola, cómo todos ya saben me gusta Rust, es una tecnología que particularmente me parece estupenda, peroooo! Aún así tengo mis críticas y lo que considero mis puntos de mejora.

Si, no soy un iluminado pero queria compartir mis ideas de como me gustaría que fuese Rust, explícare brevemente como creo yo que podríamos tener un mejor Rust. ES UNA OPINION PERSONAL y tomenlo como tal, lo escribi mayoritariamente mientras viaja en el tren y no me explaye tanto como me gustaría en cada punto, pero bueno, es lo que hay, si tienen dudas me escriben o lo buscan en internet.

<center>
<youtube video="U5VIHraRclU"></youtube>
</center>

# Sistema de efectos

Voy a ser claro con esto, quiero un sistema de efectos, los lenguajes con sistemas de efectos declarativos me parecen bastante buenos.

Veamos un caso sencillo, que podría ser considerado un sistema efectos? El `try` `catch` `finally` con el sistema de excepciones de toda la vida.

```java
try {
    // código que puede lanzar una excepción
} catch (ExceptionType e) {
    // código para manejar la excepción
} finally {
    // código que se ejecuta siempre, haya o no haya una excepción
}
```

Por qué es un sistema de efectos? Porqué altera la normal lectura del código, modifica la lectura, no es de arriba hacia abajo y ya, es de adentro hacia afuera de los bloques en base a circunstancias, es como un `goto` gigante alternando el flujo cuando hay una excepción capturable, lo normal seria un `return` pero en flujo nos encontramos un error y no seguimos leyendo hacia abajo, no, en su lugar saltamos a un contexto distinto, el `catch` y allí hacemos cosas e independientemente del caso terminamos en un `finally` si tenemos.

```java
try {
    Boolean condition = true; // Código que determina si se lanza una excepción o no
    if (condition) { // seguimos leyendo de arriba hacia abajo, no hay nada raro
        throw new Exception("An error occurred!"); // Un error! Aquí se altera el flujo, no seguimos leyendo hacia abajo, saltamos al catch
    }
    // Código no ejecutable? para que sigues leyendo, ve al catch, no sigas leyendo, no es necesario, el flujo se alteró, no sigas leyendo, ve al catch!
    return "Hello, World!";
} catch (Exception e) {
    System.out.println(e.getMessage()); // Aquí manejamos la excepción, hacemos cosas, pero no seguimos leyendo hacia abajo, no es necesario, el flujo se alteró, ve al finally
} finally {
    System.out.println("This will always be executed."); // Aquí se ejecuta siempre, haya o no haya una excepción.
}
```

Qué pasa si tenemos `return` dentro de un `try` pero tenemos un `finally`? 

Normalmente uno esperaría que salga, que retome y en realidad no, se ejecuta el `finally` y luego el `return`.

```java
try {
    return "Hello, World!"; // Aquí se altera el flujo, no seguimos leyendo hacia abajo, saltamos al finally, no es necesario seguir leyendo, ve al finally
} finally {
    System.out.println("This will always be executed."); // Incluso si hay un return en el try, el flujo se alteró
}
```

Qué pasa si tenemos dos return uno en el try y otro en el finally? El return del finally se ejecuta y el return del try se ignora.

```java
try {
    return "Hello, World!";
} finally {
    return "Goodbye, World!"; // El return del try se ignora, tiene prioridad el return del finally, el flujo se alteró
}
```


Brillante, volvamos, se entiende como cambia el flujo? Otro caso polémico, el defer.

```go
func leer_archivito(path string) ([]byte, error) {
    file, err := os.Open(path)
    if err != nil { // ???
        return nil, err
    }

    // Garantiza cierre SIEMPRE
    defer file.Close() // 👈 Esta shit deberia de ser magica ✨

    data, err := os.ReadFile(path)
    if err != nil { // ??? 
        return nil, err
    }

    return data, nil
}
```

Maravilloso, maravilloso defer, todos te aman, pero yo te odio, por qué? Porque conocí Rust.

Rust tiene una hermosa cosa llamada RAII, un buen RAII, no como el de C++, C# o cualquier otro lenguaje del montón, buen y sólido RAII.
El defer no es más que un poor man's RAII, implementar RAII es muy complejo, no sabemos cómo hacerlo sencillo, es justo lo que Go, Swift y Zig pensaban, pues no mi ciela, agregamos líneas demás en nuestro código, casos de que deberíamos de hacer posteriormente los agregamos de ante mano porque es una buena práctica y nos podemos olvidar, por tanto la buena práctica nos dice capturar la excepcionalidad, no es una excepcionalidad? no sabemos si realmente será un caso borde... es un caso que quizás sea frecuente? Si es frecuente porque no está encapsulado? Por qué no tenemos un buen sistema de tipos? Efectivamente RAII es no implementable en la mayoría de lenguajes por esto, no podemos encapsular comportamiento muy específico.

```rust
fn leer_archivito(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = std::fs::File::open(path)?; // Aquí se abre el archivo, si hay un error se retorna inmediatamente
    let data = std::fs::read(path)?; // Aquí se lee el archivo, si hay un error se retorna inmediatamente
    Ok(data) // Aquí se retorna el resultado exitoso
} // Aquí se cierra el archivo automáticamente al salir del scope, no es necesario hacer nada más
```

Se entiende la diferencia?

Ahora bien si revisan ese codigo en Rust notaran una diferencia fundamental, el `?`, eso altera el flujo, entonces este operador, es parte de un sistema de efectos? No, es un operador de propagación de errores, se podría considerar similar al `throw` de Java? Si pero no funciona igual, el `throw` no para hasta alcanzar un bloque `catch` y ejecuta todos los bloques `finally` que encuentre en el camino por lo que los errores se propagan magicamente todo el tiempo y no los puedes parar, el `?` es solo sugar syntax para un handleo manual, no funciona con todo, solo con el `Result` de momento, se propaga un solo nivel, es como si hicieras un `return Err(_)` y ya. Hay una alternativa al `?` que se esta discutiendo aún que es el polemico operador `yeet` que deberia de ser renombrado por `throw` en el futuro, haciendo un flujo similar al de Java y otros muchos lenguajes con algunos cambios no necesariamente menores.

Ahora bien sistema de efectos? Si! Ya explicamos como funciona y en qué casos se puede ver pero esto es bueno?

No necesariamente, como lo planteo hasta ahora es mal, caca, feo.

Vamos a ver un caso de bien.

Tenemos algo que va a explotar? Java tiene como buena práctica declarar en la firma de los métodos el keyword throws para declarar nuestras posibles excepciones y porque romperá todo, OJO es buena práctica, se hace? No necesariamente, Java tiene muchos defectos y este si bien me parece algo interesante está mal implementado, nuevamente por el sistema de tipos, Java divide las excepciones entre checked y unchecked por lo que las declaraciones pueden ser ambas pero Java nos pide solo declarar lo checked, lo unchecked es responsabilidad del usuario de este método. El sabrá darle buena utilidad, si si.

Spoiler no. (Podría explayarme, no tengo ganas) EJEMPLO!

```java
public void miFuncion() throws IOException {
    // ...
    // Código que puede lanzar una IOException
}
```

En este ejemplo gracias a las maravillas de Java sabemos que esto puede arrojar una IOException, no es necesario revisar el código para saberlo, solo vemos la firma del metodo y lo sabemos, nice.

Pero que es eso???


```java
public void miFuncion() throws IOException {
    Boolean condition = false; // Código que determina si se lanza una excepción o no :0
    if (condition) { 
        throw new IOException("An error occurred!"); 
    }
    throw new RuntimeException("An error occurred!");
}
```

Al parecer si revisamos la implementación en detalle al final no hace nada de lo que promete! MALDITOS ME ENGAÑARON

Yo esperaba un `IOException` pero al parecer esta devolviendo una excepción que no esta en la firma?! VEN POR QUE ESTA MAL EL SISTEMA DE JAVA?!

Además los `catch` en Java son de tipado estatico haciendo que solo puedas capturar excepciones que tengan una relación de herencia, si escribes la excepción especifica (en este caso `IOException`) no podrás capturar la `RuntimeException` que es la que realmente se lanza, por lo que el sistema de excepciones de Java es completamente inútil, no sirve para nada, es una mentira, es un engaño, es basura, no lo uses, no lo implementes, no lo pienses, no lo consideres, no lo menciones, olvídalo, nunca más vuelvas a hablar de eso!!! 

Si, así de grave es el tema, tuve que decir todo eso. Si revisan el sistema que propone Rust para el operador `yeet` es mucho más refinado que esta basura 💅.

Pero volviendo al caso el `throws` está bien, está muy bien, casi tan bueno como el `Result` de Rust CASI, pero que paso? Los programadores del pasado decían muy verboso, no sirve, basura, y tiraron la idea y no se volvió a usar nunca, nunca, NUNCA más. Solo los programadores Java ortodoxos lo usan, no es la mayoría de los devs, la mayoría ignora el tema, unos tipazos y aparte todo en Java puede ser nulleable por defecto así que es un campo minado, todo puede dar `NullPointerException` y no lo sabes, no lo declaras, no lo documentas, no lo manejas, es un caos, es un desastre, es una locura, es un infierno, es por eso que existe el famoso [`The Billion Dollar Mistake`](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/).

Koka fue una locura cuando lo ví, y dije, está mierda es lo que me interesa.
Un lenguaje que maneja TODO como sistema de efectos, una idea totalmente olvidada retomada, que lo hace bueno? Es como Java?
No, para nada es como Java, y no me gusta tanto, pero goooood, el sistema de efectos es todo lo que está bien.
Quieres usar la salida estándar? Quieres usar el out o el error? DECLARALO, quieres abrir un archivo? DECLARALO, quieres acceso a red? DECLARALO, quieres hacer algo loco y único? Declara el efecto como tú quieras pero hazlo maldita sea!!!

cuál es el chiste de esto? Usamos efectos ya existentes o declaramos nuevos efectos como resultado de algo.

Si divido un número podemos tener como resultado un entero pero también un mensaje de error en consola, o un error, podría declarar que se hace uso de operaciones aritméticas y por tanto puedo tener un overflow o algo por el estilo, declaramos la complejidad que tenga lo que hagamos.

```koka
fun divide(x: int, y: int) : int <exn> {
  if (y == 0) {
    throw("division by zero")
  }
  else {
    x / y
  }
}

fun main() {
  val result = divide(10, 2) // Puede explotar porque no lo estoy tratando (pero se podría!!!)
  println(result)
}
```

Y también podríamos tratarlo como una Monada:

```koka
fun divide(x: int, y: int) : maybe<int> {
  if (y == 0) {
    Nothing
  }
  else {
    Just(x / y)
  }
}
```

Otro caso seria como mencione definir nuestros propios efectos:

```koka
effect yield: (E, V) -> V yield
  ctl yield( i : int ) : bool
```

Antes que nada si, se permiten brackets y no usar brackets, todo un tema pero volviendo al tema original definimos el efecto `yield` que deberia de ser algo así como un generador, vamos a darle un uso:

```koka
fun traverse( xs : list<int> ) : yield () 
  match xs 
    Cons(x,xx) -> if yield(x) then traverse(xx) else ()
    Nil        -> ()
```

Es decir de una forma simplificada, definimos la función `traverse` que recorre una lista de enteros y separa la lista en dos partes, el `head` (el primer elemento de la lista) y el tail (el resto de la lista), si no la puede separar en dos entonces es un `Nil` y terminamos, si la puede separar entonces hacemos un `yield` con el `head` de la lista, si el resultado del `yield` es verdadero entonces seguimos recorriendo la lista otra vez pero con `tail` es decir el resto, si es falso entonces terminamos.

Vamos a darle uso a esta función: 


```koka
fun print-elems() : console () 
  with ctl yield(i)
    println("yielded " ++ i.show)
    resume(i<=2)
  traverse([1,2,3,4])
```

Nos encontramos con cosas, los efectos siempre se leen al reves, es una cosa rara, pero bueno.

En la primera linea definimos la función `print-elems` que no recibe ningún parámetro, usara la consola y no retornara nada.

En la segunda linea encontramos el `with` con el `with` definimos un handler, decimos, cuando haya un efecto `yield` ejecuta este bloque de código, y luego extraemos del efecto el valor interno y lo usamos con la variable `i`, lo mostramos por pantalla y ejecutamos la función `resume` que es propia de Koka, la cual nos permite continuar con la ejecución del efecto en el lugar en que se quedo. 

Por lo que cuando ejecutemos `traverse` con la lista `[1,2,3,4]` se printeara por consola 1, 2, 3 y como 3 es mayor a 2 entonces el `resume` devolverá falso y se terminará la ejecución de `traverse` sin printear el 4. El efecto cortara la ejecución en ese lugar, no retomara la posición.

El output es:

```bash
yielded: 1
yielded: 2
yielded: 3
```

Resulta escamoso no? Complicado? Bueno por suerte deberia de simplificarse un poco más en Rust pero justamente esto desbloquea muuuuchas posibilidades en Rust.

Ventajas de esto? Si usamos una lib cerrada solo vemos nombre de métodos, implementaciones no sabemos, debemos usar un SDK, algo por FFI, etc. No nos vamos a poner a decompilar para ver qué hace internamente, porque falla en un caso súper raro o algo así. Está en la firma, ya sabemos que hace al llamar la función, necesito esto.

Beneficios adicionales? Sistema de permisos en el códigoooo todo de forma atómica, si quieres ejeuctar una función sabes explícitamente que va a necesitar acceso a internet, ahora nos preguntamos... PENSAMOS... Dudamos... Y decimos... que raro...
Por qué MI FUNCIÓN DIVIDIR ACCEDE A WALLET-RANDOM.COM????

AHÍ ESTÁ! CÓDIGO ESTATICO VERIFICADO! Hay permisos más complejos que otros, una llamada al file system siempre será más riesgoso que algo a acceso a red o a una salida estándar o  algo así.
Imaginen hardware declarativo, está función acceder a tu USB! Nice

Pero podemos desglosar, y si separamos FSRead y FSWrite o algo así???
Incluso forzariamos a las libs y bibliotecas a decirnos que pasa.

Otro caso? No compilar, no analyzer, nada, primero leer el código estático en crudo y decir analizar esto y hacer un check o un build puede pedir acceso a esto esto y esto, usamos una macro??? Este código puede ejecutar código arbitrario en build time. 
Todo declarado, maravilloso.

Se deberia de pulir bastante pero de base nos daría más información en todo lo que usamos, pienselo como usuarios.

### ¿Cuál es el punto de este punto?

**Acaso quiero un `try`/`catch`/`finally`/`throw`/`throws`???? No, no estoy diciendo eso, quiero un sistema de efectos, quizás no tan complejo como el de Koka pero si abres un archivo en algún punto que esa información se propague de forma en que sepa el usuario final de que va a pasar eso. Es más esto que estoy escribiendo va a estar en negrita para que se entienda**

### Quiero saber si algo tiene acceso a consola, a red, a file system, o algo y que cuando se analice con el rust analyzer o se compile o algo así me de una información de lo que hace el programa.

No quiero que se compile automaticamente, dame información, permisos por libs, y todo eso lo obtenemos en parte sabiendo con el sistema de efectos.

Actualmente tenemos dos sistemas de efectos en Rust `yield` (en nightly) y `async`.

Siguiente punto...

# Reflection en comptime

Si, ya tenemos cosas así pero no el 100% QUIERO, DESEO, EL 100% necesito que con una función simple poder poner el valor en comptime, lo modifico??? Obtengo un récord de hey, se agrego en comptime un Récord que es una derivada del tipo X

Es decir algo como
`Record<Gato,("vehículo_todo_terreno")>` y yo poder asignarle el vehículo de todo terreno en true y que estaticamente ese atributo ahora este disponible en mi nuevo valor...
SQLX lo logra por que no podría ser algo estándar???

Quiero decir obviamente requiere más esfuerzo que eso estamos hablando de agregar meta información acerca de las estructuras los traits que implementan y muchas otras cosas que actualmente son complicadas de conseguir. 
Siempre habrá trucos, como por ejemplo hacer que todas las `struct`s que quieras implementen un `trait` que devuelva la lista de atributos, que devuelva el nombre de la `struct`, o cosas así.
Es decir agregar un valor de tipo estático dentro de la implementación del `trait` para poder saber cosas del `struct`.

O simplemente hacer cosas similares a las que hace sqlx en donde macros se encargan de crear nuevos records con la información de los atributos de una tabla. 


```rust
let account = sqlx::query!("select * from accounts where id = ?", 1)
    .fetch_one(&mut conn)
    .await?;

println!("{:?}", account); // Record { id: 1, name: "Herp Derpinson" }
println!("{}: {}", account.id, account.name); // 1: Herp Derpinson
```

Por supuesto todo esto no es gratis requerirá un esfuerzo de optimización del rust analyzer para poder evaluar en tiempo de compilación más información de los tipos 

Y si se que SQLX lo logra mediante instropección le das una URI a la DB, va revisa las tablas, guarda en un JSON las cosas y luego en la macro revisa ese JSON y te crea el record en base al select. PERO! Para alguien que no toco nunca Rust es magico, es como wow que es esto, como es posible! Y me dices que no es un ORM??? IMPOSIBLE! DEBE TENER UN MINI ORM FUNCIONANDO AHÍ ADENTRO! DEBE HABER UNA AI ENTENDIENDO LO QUE ESTA PASANDO!! ES UN MODELO!!!!! Bueno ya paro con el sarcasmo (por ahora)

Además el sistema de reflection en comptime podría complementarse perfectamente con el sistema de efectos, no solo sabriamos que una `struct` implementa una función `save` sino que también sabríamos que esa función requiere acceso a la base de datos, o que una función `send_email` requiere acceso a la red, etc. Esto nos daría una información mucho más completa y útil sobre el código que estamos usando, y nos permitiría tomar decisiones informadas sobre cómo usarlo y qué esperar de él, o como mockear o mil cosas más.

---

# Input macro 

Considero que es necesario para mejorar la ergonomía del lenguaje agregar esta útil macro debido de que es un punto inicial y un punto en el frustración en los más nuevos en el lenguaje.

```rust
let name: String = input!("Please enter your name: ")?;
let age: u8 = input!("Please enter your age: ")?;

println!("Hello, {name}! You are {age} years old.");
```

Comprendo que algunas personas no concuerden, pero nuevamente lo veo como algo importante.
Quiero decir es algo relativamente estandarizado, la gente tiene formas simplificadas de obtener inputs de forma sencilla, nosotros no seguimos eso.

Uno aprende un lenguaje y generalmente comienza con un programita de terminal, no es una cosa rara.

Me resulta tonto profundizar acerca de esto luego de que se haya pedido tantas veces, para más información lean [mi RFC](https://github.com/rust-lang/rfcs/pull/3799).

Resumidamente quiero una macro que me permita leer la entrada del usuario por consola y como extra parsear los input a un tipo genérico que decidamos en el código por ejemplo si introduzco 42 quiero que sea un entero no un string, eso lo tipeo en mi código digo esto es un u8 y ya, se resuelve.
 
  
# Tipos escalares que escalen


Suena raro pero mi principal problema con los tipos escalares en Rust surgio cuando conocí Zig.
Es lógico. No debería de ser tan complicado.

Esto debería de ser un must, no un Nice to have.
Nosotros en Rust tenemos tipos escalares pero con algunos tamaños en específico.

En el caso de los enteros tenemos de 8, 16, 32, 64 y 128, Zig nos deja tener tipos escalares con toda  la escala, quieres un entero de 5 bytes? Puedes! Quieres un entero de 21 bytes puedes! Sabes el valor exacto para representar el rango maximo de tu variable puedes hacerlo a medida! Eso está cool, y quiero eso.

```rust
let x: i6 = 42; // Un entero de 6 bytes
let y: i21 = 12345678901234567890; // Un entero de 21 bytes
```

Creo que se entiende perfectamente, tiene ventajas muy positivas esto.

# Contratos 

No, no se trata de legales. Estoy buscando que Rust tenga contratos más estrictos. El random que estará recién aprendiendo Rust dirá más estricto? Mamhuevazo Rust es un dolor de cabeza constante. Si pero no. Rust es estricto. Pero lo que quiero es darle más fortaleza a eso. Cómo??? Muy específicamente usando la cláusula `where`. Yo sé perfectamente que esto no está saliendo como quiero, no quiero una macro para eso pero es mejor que nada.

Nosotros decimos que el contrato entre programadores son las firmas de los métodos, funciones, pero esto generalmente lo tenemos asociado a:
- Tipos de datos
- Nomenclatura 
- Posibles resultados, mensajes de error o resultados en caso de que salgan las cosas bien
- Documentación 

Pero digamos esto no está escrito en piedra, el contrato mientras cumplas estáticamente con el input y el output ya seria código compilable. Y yo puedo hacer una función dividir dónde retorne un error si se da como Input un 0 en el divisor pero honestamente meter un if es algo que quizás no queremos, nos lo podemos ahorrar? Si y no.

Depende cómo se lo piense, con los contratos (Dependiendo de si es estático o dinámico) nos podemos ahorrar ese if porque en comptime podemos establecer que el valor numérico de ese divisor puede ser cualquier cosa. Cualquier cosa menos cero y ya. Se verifica el valor de la variable divisor en tiempo de compilación asegurándote que el usuario no pueda ingresar un 0.

```rust
const fn dividir_sin_cero<T>(a: T, b: T) -> T 
where T: std::ops::Div<Output = T> + std::cmp::PartialEq + From<u8>
requires b != T::from(0)
{
    a / b
}
```

De esta forma seria como lo plantearía yo

En el caso de Rust se lo esta haciendo de otra forma:

```rust
use core::contracts::requires;

#[requires(b != T::from(0))]
const fn divide<T>(a: T, b: T) -> T 
where
    T: std::ops::Div<Output = T> + std::cmp::PartialEq + From<u8>,
{
    a / b
}
```

No me convence mucho el tema de más macros, prefiero keywords. Recomiendo la forma en que lo hace [`Verus`](https://verus-lang.github.io/verus/guide/requires_ensures.html), aquí hay algunos ejemplos:

```rust
const fn print_two_digit_number(i: i8)
    requires
        -99 <= i < 100,
{
    println!("The answer is {}", i);
}
```

Se asegura que la función solo tenga entre uno y dos digitos.

```rust
const fn just_positive_output(i: i8) -> (salida: i8)
    ensures
        salida > 0,
{
    i + 1
}
```

El `ensures` en este caso se asegura que el resultado de la función sea siempre positivo, no importa el valor de `i`, el resultado siempre será mayor a 0. Para eso le asignamos un alias al output de la función, en este caso `salida` y luego en el `ensures` hacemos referencia a ese alias para establecer la condición que queremos que se cumpla.

Se entiende? Con una firma normal de cualquier lenguaje verificamos que la variable respete el tipo de dato pero con contratos verificamos el valor! El contenido que le enviamos a la función a ejecutar.

Vale la pena agregar contratos en runtime? Supongo que no estaría demás pero con contratos en comptime ya me basta.


# Custom Allocators 

Rust no impide el uso de alocadores personalizados pero deberías de volver a re implementar algunas estructuras y cosas, la idea es darles una forma sencilla a los usuarios de que permitan alocar de forma custom. No soy especialmente un gran impulsor de esta idea porque claro, es un beneficio que en mi campo (programación de alto nivel) no suele considerarse esto pero me imagino que la programación de bajo nivel le podrá dar un buen uso a está feature, hay campos como la programación de videojuegos que quizás podrían optimizar la gestión de recursos y en el caso de programación web entiendo que mejora la liberación de Mutex.

Entiendo las ventajas. Pero bueno, personalmente no es una prioridad pero si estaría cool, la feature está a medio hacer, en nightly hay algo ya construido pero bueno, faltan cositas.

# Varidic Generics 

Qué es esto de los variadicos? Formas de representar cosas que varían. Cuando trabajamos con tuplas tenemos que definir cada elemento de la tupla, de forma en que no haya una forma de generalizar tuplas, decimos  
esto es un (i32, i32) pero si tenemos más i32?

Debemos hacer (i32, i32, i32)? Y si son 4 o 5 o 6 elementos?

Bueno se complica, lo mismo con la cantidad de parámetros de una función, como lo solucionan actualmente? macros, obvio, todo en rust lo solucionamos con macros.

Podría estar mejor.... Variadicos viene a solucionar eso, una forma de generalizar esto. 

```rust
fn make_tuple_sing<...T: Sing>(t: (...T)) {
    for member in ...t {
        member.sing();
    }
}

let kpop_band = (KPopStar::new(), KPopStar::new());
let rock_band = (RockStar::new(), RockStar::new(), RockStar::new(), RockStar::new());
let mixed_band = (KPopStar::new(), RockStar::new(), KPopStar::new());

make_tuple_sing(kpop_band);
make_tuple_sing(rock_band);
make_tuple_sing(mixed_band);
```

Un ejemplo basico sería así, una función `make_tuple_sing` que recibe una tupla de cualquier cantidad de elementos siempre y cuando todos los elementos implementen el trait `Sing`, luego dentro de la función iteramos sobre cada miembro de la tupla y llamamos al método `sing` de cada uno. De esta forma podemos tener tuplas de cualquier cantidad de elementos sin necesidad de definir cada una de ellas por separado.

Ignorando la cantidad de elementos podemos generalizar y construir cosas bastante increibles.

# Default values

Como es esto? Otra mejora de calidad de vida, en TS y muchos otros lenguajes es muy sencillo asignar valores por default a parámetros y estructuras, en el caso de Rust ya tenemos algo en curso.

El [RFC actual](https://github.com/rust-lang/rust/issues/132162) contempla valores por default en structs. 
Algo como:

```rust
#[derive(Default)]
struct Something {
    value: i32 = 42
}
```

Divino, está quedando genial ahora bien el problema con esto es que actualmente solo se está implementando para valores que pueden ser evaluados en comptime, si el tipo es dinámico estamos en un problema, no se puede hacer esto.

DEBERÍAMOS DE IMPLEMENTAR EL DEFAULT POR NUESTRA CUENTA! Horrible, pero bueno, vamos en camino, algo es algo, gran trabajo!

Este ejemplo esta tomado del RFC:

```rust
#[derive(Default)]
struct Pet {
    name: Option<String>, // impl Default for Pet will use Default::default() for name
    age: i128 = 42, // impl Default for Pet will use the literal 42 for age
}

// ...

let valid = Pet { name: None, .. };
assert_eq!(valid.age, 42);
let default = Pet::default();
assert_eq!(default.age, 42);

let invalid = Pet { .. };
```

Como digo si el `struct` tuviera un valor dinamico ya nos complicaria y en el caso del `Option<String>` podemos entender que `Option<T>` tiene implementación `const` de `Default` y que el valor por default es `None`.

# Optional Arguments

Esto es una tontería en otros lenguajes existe, lo quiero, se me hace ergonómico e intuitivo
Hay formas de hacerlo mejor o peor pero la idea es que no sea necesario pasar el parámetro, podríamos asignar las cosas opcionales como `?` o simplemente re utilizar el tipo `Option` y que si no le pasas un valor entonces que sea None por defecto y si se envia por parámetros no sea necesario wrappear el parámetro en un `Some` sino que sea mágico ✨✨🪄

No sé, me parece hasta lógico pero bueno, no se.

Algo como:

```rust

fn greet_with_age(name: &str, age: Option<u8>) {
    match age {
        Some(age) => println!("Hello, {name}! You are {age} years old."),
        None => println!("Hello, {name}!"),
    }
}

fn main() {
    greet_with_age("Mike", Some(25)); // Output: Hello, Mike! You are 25 years old.
    greet_with_age("Alice", 30); // Output: Hello, Alice! You are 30 years old.
    greet_with_age("Bob"); // Output: Hello, Bob!
    greet_with_age("Charlie", None); // Output: Hello, Charlie!
}
```

No código legacy, sigue funcionando lo ya existente solo algo de azucar sintactico nuevamente, no dejaria de ser explicito porque el compilador te avisara si cometes un error.

Además agregaría una opción de linter que sea obligatorio que los parámetros opcionales, literalmente debas tipar, que sea algo ultra explicito, por que? Porqué hay gente que lo prefiere, ser ultra estrictos hasta en esto y me parece bien, pero behaviour por default para no romper cosas sería opcional tipar el `None` de forma explicita o el `Some`.

# Default Arguments

Va de la mano con lo anterior pero esta vez con valores por defecto, que sea realmente opcional pasar algo y sino un valor por default, que nos soluciona esto? Fácil, un `unwrap_or`, más azucar sintactico 😋.

En lugar de hacer:
```rust
fn greet_with_age(name: Option<&str>, age: Option<u8>) {
    let name = name.unwrap_or("Carlos");
    match age {
        Some(age) => println!("Hello, {name}! You are {age} years old."),
        None => println!("Hello, {name}!"),
    }
}
```

Podríamos hacer:

```rust
fn greet_with_age(name: Option<&str> = "Carlos", age: Option<u8>) {
    match age {
        Some(age) => println!("Hello, {name}! You are {age} years old."),
        None => println!("Hello, {name}!"),
    }
}

fn main() {
    greet_with_age("Mike", Some(25)); // Output: Hello, Mike! You are 25 years old.
    greet_with_age("Alice", 30); // Output: Hello, Alice! You are 30 years old.
    greet_with_age("Bob"); // Output: Hello, Bob!
    greet_with_age("Charlie", None); // Output: Hello, Charlie!
    greet_with_age(); // Output: Hello, Carlos!
}
```

Seguimos sin romper código legacy, solo habilitamos nuevas posibilidades.

No es nada raro realmente lo que sugiero, una asignación magica de algo en un `Some(T)`.

# Más datos de tipo `const`

Basico, quiero poder usar más cosas en `const`, no solo tipos de datos sino también operaciones, funciones, etc. Esto es algo que se viene pidiendo hace mucho tiempo y que se ha ido implementando poco a poco pero me gustaría que se acelerara un poco más el proceso para poder tener un `const` más poderoso y versátil.

No se como se haría, no se como podría ser, se que va de la mano de algunas cosas que mencione antes y esto seria bloqueante para destrabar otras cosas pero lo quiero.

Zig tiene la increible forma de generar `struct`s anonimos en comptime, algo así como:

```zig
pub fn main() void {
    const data = .{
        .name = "Fernando",
        .age = 28,
    };

    serialize(data);
}
```

No esta mal, no quiero que todo en Rust sea anonimo, esta bueno que los `struct` tengan nombres y sean explícitos. Al mismo tiempo que esto cualquiera que lo lea me dira eso no es más que un diccionario, haces un `HashMap` y lo setteas.

```rust
use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();
    data.insert("name", "Fernando");
    data.insert("age", "28");

    serialize(data);
}
```

Pero puede que quieras otro tipo de dato, quizás quieres un `Json` y si, son practicamente lo mismo pero quizás la forma en que se serializan las cosas o se construye es distinta.

Si, lo mismo pero hay algunas diferencias, lo de Zig es en comptime y según el handler es más comodo, la forma en que hacemos los diccionarios resulta muy comodo, sugiero que de forma similar podamos hacer esto, pero en lugar de generar structs como tal generemos diccionarios, sean `HashMap`s o lo que sea pero que usando esta forma sea en comptime forzandonos a darle un tipo de dato. No que sea algo completamente dinamico sino que sea solo un `shorthand`

Versión con `HashMap`:

```rust
fn main() {
    const data: HashMap<&str, &str> = .{
        .name = "Fernando",
        .age = "28",
    };

    serialize(data);
}
```

Versión con Json:

```rust
fn main() {
    const data: Json = .{
        .name = "Fernando",
        .age = 28,
    };

    send_request(data);
}
```

Lo mismo pero más ergonomico, una copia de lo de Zig con la diferencia de no armar structs sino un `struct` ya existente, de esa forma nos aseguramos dos cosas, es suficientemente ergonomico para cuando tenemos casos así donde queremos serializar cosas, pero lo suficientemente incomodo para que no se vuelva costumbre y no se use para todo, es decir, no se vuelva el nuevo struct, no se vuelva el nuevo record, no se vuelva la nueva forma de definir cosas, es solo para casos muy específicos donde realmente lo necesitemos y no queramos armar un struct o algo así.

Sugiero que el shorthand no arroje el error de la misma forma que siempre con el tema de los trait bounds, el shorthand es solo valido si el valor de asignación cumple con el trait `FromAnonStructLiteral` o algo así, es decir, si el tipo de dato al que le asignamos el shorthand tiene una implementación de ese trait entonces el shorthand es valido, si no, entonces no es valido y arroja un error.

Toda esa conversión requeriria de un esfuerzo importante porque al final estamos dando información en comptime, los tokens no son lo mismo, estamos agregando nueva sintaxis porque estamos permitiendo que cualquier cosa seguida de un `.{ .[key] = value, ... }` sea un shorthand, entonces el parser tendría que reconocer eso y luego hacer la conversión a un tipo generico que implemente el trait `FromAnonStructLiteral` en comptime, es complejo, tengo ideas locas generalmente.

# Menciones honorificas

- Derivables más potentes y simples de hacer (eso está aceptado y se va a trabajar)
- Negative Implementations

    No esta nada mal seria muy cul tener esto, nos ayudaria de forma sencilla saber cuando se implementa o no algo, podriamos discriminar tipos de una manera más ergonomica.

- Macros con más información magica (También se esta trabajando en esto)
- Named Arguments: Si, lo tenia en mente, no me interesan tanto, estaría cul tal vez pero no es algo que precisamente me motive a escribir esto
- Integrar funciones de librerias en el lenguaje:
  Esto es algo en que se esta trabajando, no es algo muy avanzado, solo en el caso de la lib `rand` de momento, se la esta integrando y esta en nightly, pero quiero algo mejor, `Serialize` y `Deserialize` de Serde, practicamente todos lo usamos, me gustaria que sea más estandar y que rompa con el monopolio que lleva teniendo Serde desde hace rato, Serde ocasiona otros problemas en que quizás en algún momento hablo pero es importante eso.

