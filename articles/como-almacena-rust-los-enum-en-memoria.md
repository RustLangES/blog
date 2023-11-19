---
title: ¿Cómo almacena Rust los enum en memoria?
description: En este artículo exploraremos como Rust calcula el tamaño que un enum debe de ocupar en la memoria.
author: Tomás Senovilla Polo
github_user: tsenovilla
date: 2023-11-18
tags:
  - rust
  - memory layout
  - enum
social:
  github: https://github.com/tsenovilla
  linkedin: https://www.linkedin.com/in/tomás-senovilla-polo-3989a9187/
---

## Prólogo

Antes de empezar, me gustaría recalcar que escribí este artículo en una primera instancia para mi blog. Esa versión la puedes encontrar [siguiendo este enlace](https://dev-otion.com/es/entrada/como-almacena-rust-los-enums-en-memoria). El contenido es prácticamente el mismo, pero allí lo puedes leer también en inglés si lo prefieres :)

## Introducción

Los enums de Rust tienen un tamaño conocido en tiempo de compilación, pero puede que te estés preguntando cómo determina Rust cuánto espacio ocupará un enum en memoria. Vamos a explorarlo a través de algunos ejemplos que ilustran lo que hace el compilador en cada caso. Al ser un tipo compuesto definido por el usuario, los enums siguen una representación que determina su diseño en memoria. Si no especificamos ninguna, el compilador utilizará el diseño determinado por la representación de Rust, que ofrece las siguientes garantías necesarias para asegurar la solidez del código:

- Los campos están alineados correctamente.
- Los campos no se solapan.
- El alineamiento del tipo es al menos el máximo alineamiento de sus campos.

Estaremos asumiendo la representación por defecto a lo largo de este post, así que para entender como el compilador organiza el diseño de memoria para los enums, tenemos que tener estas reglas siempre en mente. 

Si quieres profundizar en las diferentes representaciones disponibles en Rust, te recomendamos leer [esta página](https://doc.rust-lang.org/reference/type-layout.html#representations).

Hay otro concepto clave que necesitamos conocer: el discriminante de un enum. Una instancia de un enum puede contener solo una variante del enum a la vez, por lo tanto es necesario un método para distinguir entre estas variantes en memoria de forma eficiente. Este es el rol de los discriminantes, que son números asignados a cada variante por el compilador que las identifica de forma única.

En teoría, los discriminantes están representados por un valor de tamaño isize en la representación por defecto, en la práctica el compilador hace un poco de magia para optimizar el tamaño de estos números y con ello el tamaño del enum. La mayoría de las veces, el discriminante asignado a una variante es el discriminante de la variante anterior más uno, siendo 0 el discriminante de la primera variante declarada.

La definición completa de discriminante se puede encontrar [aquí](https://doc.rust-lang.org/reference/items/enumerations.html#discriminants).

Vamos a utilizar la siguiente función para observar como se organizan los bytes de los enum que usaremos en los ejemplos, para entender que es lo que está haciendo el compilador:


```rust 
use std::mem;
use std::slice

fn print_bytes<T>(input: &T){
    let size = mem::size_of::<T>();
    let bytes = unsafe {
        slice::from_raw_parts(
            input as *const T as *const u8,
            size
        )
    };
    println!("{:?}",bytes);
}
```

La función tomará una referencia a nuestros enums y mostrará por pantalla sus bytes. La función es segura porque una referencia siempre es válida, por lo que el puntero usado en from_raw_parts también es válido. Además, como estamos convirtiendo este puntero a un puntero sobre un conjunto de bytes, el tamaño del input será la longitud de la dirección en memoria, por lo que la referencia devuelta por from_raw_parts es correcta.

## Vamos al lío!

El primer ejemplo es el caso más simple posible, un enum sin campos:

```rust
enum FirstEx{
    A,
    B
}

print_bytes(&FirstEx::A); // [0]
print_bytes(&FirstEx::B); // [1]
```

Como las variantes del enum no tienen contenido, lo único que necesitamos para distinguirlas es el discriminante de cada variante. Hemos dicho que el discriminante se representa como un isize, pero este enum tiene tamaño de un solo byte. Si ejecutamos nuestra función get_bytes, vemos que los bytes asociados a la variante A son [0] y los bytes asociados a la variante B son [1]. En este caso, es suficiente con un byte para representar el discriminante, por lo que el compilador almacena este enum en un byte.

Podríamos deducir que estas variantes sin campo tienen un tamaño de sólo un byte, pero esto no es cierto en general. Dependiendo de que variantes contenga el enum, el tamaño de las variantes sin campo puede cambiar, a veces el tamaño de las variantes sin campo es igual al tamaño más pequeño entre los discriminantes de las otras variantes, a veces tienen 1 byte e incluso pueden tener 0.

El tamaño de las variantes sin campo no es algo de lo que preocuparse para determinar el tamaño de un enum excepto en los enums sin campos: como son las únicas variantes, su tamaño debe ser uno, de lo contrario el enum tendría tamaño 0 y no habría espacio para almacenar los discriminantes. Los enum con menos de dos variantes son excepciones ya que no necesitan discriminantes, por lo tanto un enum con una variante sin campo tendrá tamaño 0, y un enum con una variante con valores tendrá el tamaño necesario para almacenar esos valores.

En los enums que contienen variantes con contenido, el tamaño de las variantes sin campos no es importante para determinar el tamaño del enum porque su alineación suele ser 1, y su tamaño nunca será el mayor del enum. Sin embargo, merece la pena entender cómo se determina el tamaño de las variantes sin campos para entender cómo funcionan internamente los enums.

Veamos que ocurre si modificamos nuestro ejemplo ligeramente:

```rust
enum SecondEx{
    A,
    B(u16)
}

print_bytes(&SecondEx::A); // [0,0,0,0]

print_bytes(&SecondEx::B(13)); // [1,0,13,0]
```

Si vemos los outputs, podríamos pensar que algo va mal ya que un u16 sólo ocupa dos bytes y nuestro enum tiene un tamaño de 4. Recordemos que el discriminante sigue aquí, por lo que la variante B necesitará dos bytes para almacenar su valor y algo de espacio extra para almacenar el discriminante. Como el contenido debe estar alineado este discriminante toma otros dos bytes, de ahí nuestra variante B de 4 bytes, que es también el tamaño del enum.

Es un buen momento para mencionar que estos ejemplos han sido ejecutados en una máquina con arquitectura de 64 bytes y little-endian, por lo que si los vas a ejecutar en una arquitectura diferente, los bytes apareceran representados consecuentemente a esa arquitectura.

Podemos ver que el discriminante asignado a A es 0 y el discriminante asignado a B es 1. Aquí el tamaño de A es en realidad 2 bytes: debido a que la variante A no tiene campos, no tiene ningún contenido significativo y por tanto debe ser comparada con el discriminante de otra variante para poder distinguirla. Como el discriminante de B tiene un tamaño de 2 bytes, A tiene un tamaño de 2 bytes.

Veamos otra caracterísitca del almacenado en memoria de los enums:

```rust
enum ThirdEx{
    A(u16),
    B([u8;6])
}

print_bytes(&ThirdEx::A(3)); // [0,0,0,0,3,0,0,0]
print_bytes(&ThirdEx::B([1,2,3,4,5,6])); // [1,1,2,3,4,5,6,0]
```

En este ejemplo, podemos ver que el tamaño del enum es 8 mientras que el tamaño de sus variantes son 4 y 7 respectivamente. Como el tamaño de cualquier tipo de dato debe ser múltiplo de su alineamiento, el tamaño del enum es 8 para poder contener los 7 bytes de las variantes B y cumplir con este requisito. Llama la atención que aquí los discriminantes tienen tamaños diferentes para cada variante, debido a que cada una tiene su propio alineamiento y contenido. 

¿Significa esto que después del discriminante 255 es imposible crear una variante que contenga un [u8;6]? ¡Para nada!. El compilador es capaz de cambiar el alineamiento de las variantes de 1 a 2 con el fin de aceptar estas nuevas variantes y sus discriminantes sin romper ninguna regla. Vale la pena señalar que es la variante la que cambia su alineamiento, pero no el array [u8;6] interno, de lo contrario no sería posible acceder a los elementos u8 de manera eficiente.

De hecho, podemos gestionar el alineamiento de un enum de forma manual usando los modificadores packed y align, y el compilador tiene permitido también incrementar el tamaño y el alineamiento de las variantes para conseguir un diseño del enum que cumpla con lo que necesitemos.

Para obtener más información acerca de estos modificadores, puedes echar un ojo [aquí](https://doc.rust-lang.org/reference/type-layout.html#the-alignment-modifiers).

Aquí hay que hacer una distinción: en el segundo ejemplo la variante A cambia su tamaño pero no su alineamiento. En ese ejemplo, el alineamiento no necesita cambiar porque con redimensionar el tamaño es suficiente, ya que había bytes libres que la variante no estaba utilizando. Como la variante A no tenía campos, la única forma de distinguirla de B era comparándola con su discriminante, por lo que se aumentó el tamaño de A.

Sin embargo en el tercer ejemplo, para distinguir A y B, no es necesario que los discriminantes tengan el mismo tamaño ya que hay contenido significativo después de ellos. Por lo tanto, como el alineamiento de [u8;6] es 1, el discriminante de B se coloca justo antes del principio del array, y debido a ello, si necesitamos discriminantes más grandes que un byte, tenemos que aumentar el alineamiento de B (y otras variantes alineadas a 1) a 2 para poder contenerlos.

Ahora podemos también entender cómo se comportan las variantes sin campos dependiendo de las otras variantes del enum: como su único contenido significativo es el discriminante, su tamaño y alineamiento serán los menores posibles para poder comparar ese discriminante con los de otras variantes. Por suerte, no tenemos que preocuparnos de eso porque el compilador lo gestiona todo por nosotros :)

Veamos un poco más de la magia que el compilador es capaz de hacer:


```rust
enum FourthEx{
    A(u64),
    B(String)
} 

print_bytes(&FourthEx::A(4))); // [0] * 8 + [4]+[0]*7 + [0]*8
print_bytes(&FourthEx::B(String::from("Dev-otion")))); // [something]*8 + [9]+[0]*7 + [9]+[0]*7
```

Suponiendo que estamos usando una máquina de 64 bits y por tanto que el usize ocupa 8 bytes, este enum debería necesitar 32 bytes: el String necesita 24 bytes + 8 bytes para su discriminante, ya que su alineamiento es 8. Sin embargo, ¡sólo ocupa 24 bytes! Veamos, el tipo String es una colección de tres elementos: un puntero al heap, un usize que contiene su longitud y un usize que contiene su capacidad. El punto clave es el primero, como un puntero al heap no puede ser nulo en Rust, el compilador "otorga" un puntero nulo a la primera variante en sus primeros 8 bytes. De esta forma puede distinguir entre variantes simplemente mirando si el puntero es válido (B) o no (A). Nótese que esto no destruye ninguna regla de seguridad ya que verdaderamente no estamos usando un puntero nulo, solo usamos los 8 primeros bytes igualados a 0 para identificar a la variante A. Podemos almacenar felizmente el valor del u64 interno en el segundo trozo de 8 bytes que contiene la variante A. Observemos también que en este caso, ¡ni siquiera tenemos discriminantes!

Esto cambiaría claramente si hubíeramos añadido una nueva variante, un campo que apuntase al heap o un valor que viviese en el stack pero con un tamaño superior a 16 bytes en la variante A. En esos casos, los discriminantes serían necesarios de nuevo y el tamaño del enum crecería hasta los 32 bytes por lo menos.

El campo contenido en la variante B no es el verdadero String, solo metadatos relacionados con él, por lo que podemos hacerlo todavía mejor:

```rust
enum FifthEx{
  A(u64),
  B(Box<String>)
}

print_bytes(&FifthEx::A(4)); // [0]*8 + [4]+[0]*7
print_bytes(&FifthEx::B(Box::new(String::from("Dev-otion")))); // [1]+[0]*7 + [something]*8
```

Mandando estos metadatos al heap mediante un Box reducimos considerablemente el tamaño del enum, ya que pasamos de tener un puntero y dos usize a tener solamente un puntero. Es importante notar que en este caso el tamaño del enum es 16 bytes ya que necesitamos discriminantes: si el compilador usase el mismo truco que en el ejemplo anterior, no habría espacio para almacenar el u64 de la variante A. Sin embargo, si A no contuviera ningún campo, el truco volvería a ser posible y el enum ocuparía solo 8 bytes. Podemos ver que merece la pena usar smart pointers como Box dentro de algunas variantes en algunos casos para reducir el tamaño del enum.

Veamos otro caso:

```rust
enum SixthEx{
    A,
    B([u8;4],bool),
    C
}

print_bytes(&SixthEx::A)); // [2,0,0,0,0]
print_bytes(&SixthEx::B([8,2,3,4],true))); // [1,8,2,3,4]
print_bytes(&SixthEx::C)); // [4,0,0,0,0]
```

En este caso, la variante B contiene 5 bytes y el enum tiene también un tamaño de 5 bytes. No hay necesidad de uno extra ya que el byte bool interior en la variante B sólo puede tomar dos valores: 0, 1. Por ello, podemos usar ese byte como "discriminante" para distinguirlo de las otras variantes.

Observa que la variante A tiene discriminante 2 y la variante C tiene discriminante 4. Esto se debe a que el discriminante de las variantes no puede ser 0 ó 1 para distinguirse del booleano almacenado en B, por lo que el primer valor disponible para A es 2. La razón de que C tenga discriminante 4 es que el discriminante 3 en realidad pertenece a B, a pesar de que no necesitamos usarlo, tenemos que asignar un discriminante a esta variante al igual que hacemos con las otras.

¡Observa también que aquí las variantes sin campo tienen tamaño 0! No hay razón para que tengan un tamaño mayor, la variante B da su tamaño al enum y no estamos comparando estas variantes con B usando su discriminante, ya que las comparamos con 0 o 1, no con 3 que es el discriminante real de B, así que no hay necesidad de comparar con discriminantes de variantes con campos, y por tanto estas variantes sin campos no necesitan tener un tamaño.

Hay una modificación importante que hacer aquí, cambiar el array contenido en B para almacenar u16 esta vez:


```rust
enum SeventhEx{
    A,
    B([u16;4],bool),
    C
}

print_bytes(&SeventhEx::A)); // [0]*8
print_bytes(&SeventhEx::B([8,2,3,4],false))); // [1, 1, 8, 0, 2, 0, 3, 0, 4, 0]
print_bytes(&SeventhEx::C)); // [2] + [0]*7
```

A diferencia del ejemplo anterior, A y C tendrán un tamaño de 1 byte. El bool contenido en B es ahora el segundo byte en el array de bytes, siguiendo al discriminante. Como el bool sólo ocupa 1 byte, el compilador lo empaqueta dentro del alineamiento del discriminante, por lo que basta con 2 bytes para almacenar el discriminante y el bool. Pero los discriminantes están ahí y la comparación ya no es con el bool, y por lo tanto las variantes sin campos necesitan tener un tamaño de 1 byte. El tamaño del enum ha pasado de ser 5 bytes a ser 10, debido a que B necesita este espacio.

## Conclusión

¡Hay muchas posibilidades para organizar los enum! Hemos presentado muchos casos de cómo Rust almacena enums en memoria, pero no son los únicos. Ten en cuenta que el compilador siempre hará sus trucos si es posible obtener un enum más pequeño sin arriesgar la seguridad de la memoria. Después de leer este post, deberías estar familiarizado con algunos de estos trucos, así que si encuentras algo inesperado a priori, ¡siempre puedes explorar sus bytes y averiguar qué está pasando! También puedes considerar dividir tus enums para obtener tipos más pequeños si la lógica de tu programa lo permite.