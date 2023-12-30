---
title: "Esta semana en Rust #12"
number_of_week: 12
description: El crate de esta semana es rouille, un pequeño marco web síncrono.
date: 2023-12-27
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Oficial
* [Anunciando 'async fn' y return-position 'impl Trait' en los rasgos](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html)
* [Actualización de la Iniciativa de Refactorización del Sistema de Rasgos de Rustc: Un llamado a la prueba](https://blog.rust-lang.org/inside-rust/2023/12/22/trait-system-refactor-initiative.html)

### Fundación
* [Mejora de la seguridad de la cadena de suministro para Rust a través de la firma de artefactos](https://foundation.rust-lang.org/news/2023-12-21-improving-supply-chain-security/)

### Actualizaciones de proyectos/herramientas
* [2023: Slint en revisión](https://slint.dev/blog/2023-in-review)
* [Presentación de Loco: The Rails of Rust](https://www.shuttle.rs/blog/2023/12/20/loco-rust-rails)

### Observaciones/Pensamientos
* [Mi camino para convertirme en rustáceo](https://thedataquarry.com/posts/path-to-becoming-a-rustacean/)
* [La seguridad de la memoria es una pista falsa](https://steveklabnik.com/writing/memory-safety-is-a-red-herring)
* [Los errores más comunes del compilador de Rust que se encuentran en RustRover: Parte 2](https://blog.jetbrains.com/rust/2023/12/20/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-2/)
* [Mi referencia fue eliminada, ¿por qué el compilador se queja de múltiples préstamos?](https://ntietz.com/blog/my-reference-was-dropped-why-is-the-compiler-complaining-about-multiple-borrows/)
* [¿Puede CppRef ser ergonómico?](https://medium.com/@adetaylor/can-cppref-t-be-ergonomic-c9cb1365bda1)
* [video] [Rust 1.74.1 & Rust News](https://youtu.be/_UItLy_nLf8)

### Tutoriales de Rust
* [El lado oscuro de la inserción y la monomorfización](https://nickb.dev/blog/the-dark-side-of-inlining-and-monomorphization/)
* [El corazón de un servidor de idiomas](https://rust-analyzer.github.io/blog/2023/12/26/the-heart-of-a-language-server.html)
* [Rust: subprocesos múltiples](https://priver.dev/blog/rust/multi-threading/)
* [serie] [Meilisearch amplía el poder de búsqueda con la RNA de disco filtrado de Arroy](https://blog.kerollmops.com/meilisearch-expands-search-power-with-arroy-s-filtered-disk-ann)
* [Portales encriptados entre Macs – construidos en Rust y Swift (algo similar a ngrok y tailscale)](https://github.com/build-trust/ockam/blob/develop/examples/app/portals/README.md)
* [video] [Manejo de errores en Rust](https://www.youtube.com/watch?v=IdnjG5N2l4M)

### Miscelánea
* [Una encuesta anónima sobre estática mutable](https://www.surveyhero.com/c/static-mut)
* [Desarrollo web en Rust](https://rust.code-maven.com/web)
* [Rocket: ¡Hola Mundo! basado en la web con pruebas](https://rust.code-maven.com/rocket-hello-world)

## Crate de la semana

El crate de esta semana es [rouille](https://crates.io/crates/rouille), un pequeño marco web síncrono.

¡Gracias a [Peter Puetz](https://users.rust-lang.org/t/crate-of-the-week/2704/1275) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [greptimedb - Añadir más tablas en 'information_schema' para una mejor compatibilidad](https://github.com/GreptimeTeam/greptimedb/issues/2931)
* [Ockam - Comprueba que el argumento del comando bootstrap_server es un host:port](https://github.com/build-trust/ockam/issues/7254) válido
* [Ockam - Refactorizar para usar interfaces tipificadas](https://github.com/build-trust/ockam/issues/6702)
* [Ockam - Comprobar que las estructuras cumplen con el esquema cddl](https://github.com/build-trust/ockam/issues/6684)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* _No nuevas convocatorias de ponentes este week_

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío, ya sea a través de un PR a TWiR o en los [foros de Rust-lang]. [enlace por determinar]

## Actualizaciones del Proyecto Rust

Se [fusionaron 268 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-20..2023-12-27

* [añadir soporte para bucles 'for await'](https://github.com/rust-lang/rust/pull/118847)
* [añadir el objetivo de Illumos Aarch64 para Rust](https://github.com/rust-lang/rust/pull/112936)
* [añadir soporte para hexagon-unknown-none-elf como objetivo](https://github.com/rust-lang/rust/pull/117601)
* [-Znext-solver: adaptar las reglas de desbordamiento para evitar roturas](https://github.com/rust-lang/rust/pull/119071)
* ['rustc_codegen_ssa': No sueltes 'IncorrectCguReuseType', haz que 'rustc_expected_cgu_reuse' attr funcione](https://github.com/rust-lang/rust/pull/118973)
* ['subtype_predicate': eliminar la sonda innecesaria](https://github.com/rust-lang/rust/pull/119107)
* [añadir comprobación de posibles literales 'CStr' en anteriores a 2021](https://github.com/rust-lang/rust/pull/118691)
* [añadir método para obtener argumentos de instanciación de instancia](https://github.com/rust-lang/rust/pull/119141)
* [agregar la puerta de función faltante para el desinfectante CFI cfgs](https://github.com/rust-lang/rust/pull/119235)
* [evitar la opción redundante para 'cross_crate_inlinable'](https://github.com/rust-lang/rust/pull/119225)
* [Cobertura: Comprueba si hay 'fn asíncrono' explícitamente, sin necesidad de una heurística](https://github.com/rust-lang/rust/pull/119155)
* [no permitir discrepancias de ABI dentro de los tipos 'repr(C)'](https://github.com/rust-lang/rust/pull/119037)
* [no buscar HIR en 'inferred_outlives_of'](https://github.com/rust-lang/rust/pull/119261)
* [emitir mejores sugerencias para '&T == T' y 'T == &T'](https://github.com/rust-lang/rust/pull/118431)
* [emite error si tiene regiones enlazadas](https://github.com/rust-lang/rust/pull/119215)
* [codificar 'CoroutineKind' directamente](https://github.com/rust-lang/rust/pull/119173)
* [Exhaustividad: mejora la complejidad en algunos partidos amplios](https://github.com/rust-lang/rust/pull/118796)
* [exhaustividad: mantener el original 'thir::P at'](https://github.com/rust-lang/rust/pull/119233)
* [exhaustividad: revelar opacos vacíos en profundidad](https://github.com/rust-lang/rust/pull/119218)
* [exhaustividad: revelar correctamente los tipos opacos](https://github.com/rust-lang/rust/pull/116821)
* [reserva 'por defecto' a 'Ninguno' durante la reducción de ast para la carpeta de por vida](https://github.com/rust-lang/rust/pull/119042)
* [arreglar ICE cuando se usa ptr sin procesar en un patrón](https://github.com/rust-lang/rust/pull/119274)
* [se corrige el bloqueo debido a que 'CrateItem::kind()' no maneja constructores](https://github.com/rust-lang/rust/pull/119135)
* [dar temporales si deja que los guardias corrijan los alcances](https://github.com/rust-lang/rust/pull/119122)
* [hacer que 'soft_unstable' aparezca en futuros informes de roturas](https://github.com/rust-lang/rust/pull/116274)
* [hacer que los cierres lleven su propio ClosureKind](https://github.com/rust-lang/rust/pull/119258)
* [marque 'ty::Const::Error' cuando cumpla con un ty no compatible con los parámetros genéricos const](https://github.com/rust-lang/rust/pull/117176)
* [pasa 'DeadItem' y lint como grupo consistente en código muerto](https://github.com/rust-lang/rust/pull/119297)
* [eliminar la duplicación de la API 'DiagCtxt'](https://github.com/rust-lang/rust/pull/119146)
* [eliminar la caché de decodificación de metadatos 'DefPathHash'](https://github.com/rust-lang/rust/pull/119265)
* [Resuelve: Alimenta con entusiasmo las visibilidades de los cierres](https://github.com/rust-lang/rust/pull/119136)
* [Resolver: Alimentar las visibilidades de los elementos de impl de rasgos no resueltos](https://github.com/rust-lang/rust/pull/119134)
* [Resolver: Dejar de alimentar las visibilidades de los tallos de la lista de importación](https://github.com/rust-lang/rust/pull/119168)
* [reelaboración '-Zverbose'](https://github.com/rust-lang/rust/pull/119129)
* [modificación simple de la información de diagnóstico de «non_lifetime_binders» para adaptarla a las carpetas de tipos](https://github.com/rust-lang/rust/pull/119154)
* [omitir la codificación duplicada de ID de caja estable en los metadatos](https://github.com/rust-lang/rust/pull/119238)
* [separar el tipo de desazúcar de corrutina de la fuente](https://github.com/rust-lang/rust/pull/119198)
* [sincronización de subárbol para 'rustc_codegen_cranelift'](https://github.com/rust-lang/rust/pull/119278)
* [sugerir '=' a '==' en más casos, incluso ante la falta de coincidencia de referencias](https://github.com/rust-lang/rust/pull/119328)
* [agregar la función ABI y el diseño de tipo a StableMIR](https://github.com/rust-lang/rust/pull/119094)
* [separar las pelusas MIR de la validación](https://github.com/rust-lang/rust/pull/119077)
* [Miri: implementar y probar 'simd_masked_load' y 'simd_masked_store'](https://github.com/rust-lang/miri/pull/3237)
* [mejorar la eficiencia de codificación para 'RawDefId'](https://github.com/rust-lang/rust/pull/119226)
* [use 'Vec' para las restricciones de región en lugar de 'BTreeMap'](https://github.com/rust-lang/rust/pull/118824)
* [estabilizar 'file_create_new'](https://github.com/rust-lang/rust/pull/119153)
* [Estabilizar la función 'ip_in_core'](https://github.com/rust-lang/rust/pull/119276)
* [añadir más nichos a rawvec](https://github.com/rust-lang/rust/pull/106790)
* [add 'IntoAsyncIterator'](https://github.com/rust-lang/rust/pull/119222)
* [añadir 'pista::assert_unchecked'](https://github.com/rust-lang/rust/pull/119133)
* [cargo: extender la sintaxis de la directiva de compilación con 'cargo:':](https://github.com/rust-lang/cargo/pull/12201)
* [Carga: mantén el bloqueo exclusivo de mutar al vender](https://github.com/rust-lang/cargo/pull/12509)
* [cargo: refactorizar: centralizar los checkouts de Git y las rutas de base de datos](https://github.com/rust-lang/cargo/pull/13187)
* [cargo: refactor: tipos de error personalizados para 'cargo-util-schemas'](https://github.com/rust-lang/cargo/pull/13186)
* [cargo: rework '--check-cfg' comentario de generación](https://github.com/rust-lang/cargo/pull/13195)
* [rustdoc: Agregar información 'is_object_safe' para rasgos en la salida JSON](https://github.com/rust-lang/rust/pull/119246)
* [Rustdoc: Se corrigió la visualización del bloque de advertencia si es el primer elemento del bloque de documentación superior](https://github.com/rust-lang/rust/pull/119283)
* [clippy: 'question_mark': también se activa en las sentencias 'return'](https://github.com/rust-lang/rust-clippy/pull/11994)
* [clippy: comprueba si está fuera de límite cuando se accede a una matriz de longitud conocida con un índice constante](https://github.com/rust-lang/rust-clippy/pull/11998)
* [clippy: no considerar 'async { (impl IntoFuture).await }' como redundante](https://github.com/rust-lang/rust-clippy/pull/11967)
* [clippy: extender 'UNNECESSARY_TO_OWNED' para manejar 'split'](https://github.com/rust-lang/rust-clippy/pull/11871)
* [clippy: mover 'uninhabited_references' a 'vivero'](https://github.com/rust-lang/rust-clippy/pull/11997)
* [clippy: nuevas pelusas 'iter_filter_is_some' y 'iter_filter_is_ok'](https://github.com/rust-lang/rust-clippy/pull/12004)
* [clippy: evitar que la sugerencia de 'bool_comparison' consuma paréntesis](https://github.com/rust-lang/rust-clippy/pull/11991)
* [rust-analyzer: macros exportadas completas en '#[macro_use($0)]'](https://github.com/rust-lang/rust-analyzer/pull/16137)
* [rust-analyzer: implemente un modo de servidor proc-macro respaldado por span de rust-analyzer](https://github.com/rust-lang/rust-analyzer/pull/16088)
* [Rust-Analyzer: Elimina automáticamente los llaves innecesarios después de eliminar las importaciones no utilizadas](https://github.com/rust-lang/rust-analyzer/pull/16066)
* [rust-analyzer: configurar y marcar correctamente los intervalos proc-macro](https://github.com/rust-lang/rust-analyzer/pull/16175)
* [rust-analyzer: arreglar el análisis de finalizaciones que no almacenan en caché todos los nodos en Semantics](https://github.com/rust-lang/rust-analyzer/pull/16184)
* [Rust-analyzer: Arreglar el marcado de intervalo para macros FN incorporadas](https://github.com/rust-lang/rust-analyzer/pull/16178)
* [Analizador de Rust: elimine completamente los tramos ficticios](https://github.com/rust-lang/rust-analyzer/pull/16167)
* [Rust-analyzer: eliminar la coma incorrecta después de eliminar los corchetes innecesarios](https://github.com/rust-lang/rust-analyzer/pull/16185)

### Clasificación del rendimiento del compilador de Rust

Algunas de las recientes oscilaciones de ruido han vuelto a aparecer esta semana, pero afortunadamente
con menos frecuencia que antes. Hubo algunas regresiones reales, pero la mayoría de ellas
se localizaron en una sola prueba de esfuerzo. Por otro lado, hubo algunos muy
Buenas victorias en todos los ámbitos, especialmente para las comprobaciones y las construcciones incrementales, principalmente
gracias a [#118824](https://github.com/rust-lang/rust/pull/118824) y
[#119265](https://github.com/rust-lang/rust/pull/119265).

Triaje realizado por **@kobzol**.
Rango de revisión: [bf9229a2e366b4c311f059014a4aa08af16de5d8.. 1ab783112ab4e4807304dbd249b39771246013ef](https://perf.rust-lang.org/?start=bf9229a2e366b4c311f059014a4aa08af16de5d8&end=1ab783112ab4e4807304dbd249b39771246013ef&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.6% | [0.6%, 0.6%] | 1 |
| Regresiones ❌ <br /> (secundaria) | 2.5% | [0.3%, 4.2%] | 10 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-3,3%, -0,1%] | 180 |
| Mejoras ✅ <br /> (secundaria) | -1,2% | [-5,5%, -0,2%] | 109 |
| Todos ❌✅ (primario) | -0,8% | [-3,3%, 0,6%] | 181 |

5 regresiones, 7 mejoras, 2 mixtas; 1 de ellos en rollups
58 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/73d96e7ca26ef9ddfc1c32c7701e1f1159512c49/triage/2023-12-26.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Usar la clasificación de versiones para todas las clasificaciones](https://github.com/rust-lang/rust/pull/115046)
* [disposición: fusionar] [Problema de seguimiento para patrones de rango exclusivos](https://github.com/rust-lang/rust/issues/37854)
* [Disposición: Fusionar] [RustDoc: Limpiar el botón de ocultación de la barra lateral de origen](https://github.com/rust-lang/rust/pull/119066)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Permitir inferencia de tipos para const o static](https://github.com/rust-lang/rfcs/pull/3546)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-12-27 - 2024-01-24 🦀

### Virtual

* 28/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 03/01/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 09/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fvdtgtygccbmb/)
* 11/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 16/01/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/297128172/)

### Europa

* 27/12/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs y peluches**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)
* 28/12/2023 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 3: Edición Navideña**](https://www.meetup.com/rust-vienna/events/297826979/)
* 11/01/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 11/01/2024 | Wrocław, PL | [Rust de Breslavia](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 13/01/2024 | Helsinki, FI | [Grupo Rust-lang de Finlandia](https://www.meetup.com/finland-rust-meetup/)
    * [**Encuentro de enero**](https://www.meetup.com/finland-rust-meetup/events/297811750/)

### América del Norte

* 27/12/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 06/01/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Beacon Hill**](https://www.meetup.com/bostonrust/events/297633937/)
* 08/01/2024 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 09/01/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Noche de Codificación/Hackeo/Aprendizaje Oxidado**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 09/01/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 14/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 16/01/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 17/01/2024 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/298003233/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/182f6dv/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Rust puede ser bastante más prolijo que C; Hay muchas invariantes que tienen que ser expresadas en el código. Pero eso se contrarresta con la necesidad de mucho menos código de manejo de errores; Resulta ser un lavado, ya que el tamaño de las dos implementaciones es aproximadamente el mismo.

– [Alice Ryhl en la Conferencia de Plomeros de Linux citada por Jonathan Corbet, LWN](https://lwn.net/Articles/953116)

¡Gracias a [Ivan Fraixedes](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1498) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18sjuxk/this_week_in_rust_527/)</small>
