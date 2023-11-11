---
title: "Esta semana en Rust #5"
number_of_week: 5
description: Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.
date: 2023-11-08
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

### Boletines informativos

* [Este mes en Rust OSDev: octubre de 2023](https://rust-osdev.com/this-month/2023-10/)

### Actualizaciones de proyectos/herramientas

* [rust-libp2p v0.53 ha sido liberado](https://github.com/libp2p/rust-libp2p/releases/tag/libp2p-v0.53.0)
* [Lanzamiento de Zellij 0.39.0](https://zellij.dev/news/session-resurrection-ui-components/)
* [Calificado oficialmente - Ferroceno](https://ferrous-systems.com/blog/officially-qualified-ferrocene/)
* [Rocket ́s 4th v0.5 Release Candidate](https://rocket.rs/v0.5-rc/news/2023-11-01-version-0.5-rc.4/)

### Observaciones/Pensamientos

* [Escribir enlaces de Rust para mi aplicación de Python](https://dhruv-ahuja.github.io/posts/writing-rust-bindings/)
* [Un plan de cuatro años para Rust asíncrono](https://without.boats/blog/a-four-year-plan/)
* [Cursed Rust: Printing Things The Wrong Way](https://endler.dev/2023/cursed-rust/)
* [Rust para desarrolladores de JavaScript: una descripción general de las pruebas](https://www.shuttle.rs/blog/2023/11/08/testing-in-rust)
* [¿Por qué Golang en lugar de Rust para desarrollar la aplicación de escritorio Krater](https://blog.moonguard.dev/why-golang-instead-of-rust-to-develop-the-krater-desktop-app)
* [Google reescribe la carpeta de Android en Rust con resultados prometedores](https://www.phoronix.com/news/Google-Linux-Binder-In-Rust)
* [volcar la estructura de Rust o la representación de memoria de enumeración como bytes](https://bennett.dev/rust/dump-struct-bytes/)
* [Cómo los proyectos de código abierto están usando Kani para escribir mejor software en Rust](https://aws.amazon.com/blogs/opensource/how-open-source-projects-are-using-kani-to-write-better-software-in-rust/)
* [Creación de backends asíncronos de alto rendimiento con Burn-Compute](https://burn.dev/blog/creating-high-performance-asynchronous-backends-with-burn-compute)
* [Adiós Python, Hola Rust: Construyendo una aplicación RAG CLI con Orca](https://huggingface.co/blog/santiagomed/building-a-rag-cli-application-application)
* [Genéricos variádicos, de nuevo](https://poignardazur.github.io/2023/11/08/time-for-variadic-generics/)
* [Uso de Rust, Chrome y NixOS para tomar capturas de pantalla sin cabeza para compartir en redes sociales](https://lgug2z.com/articles/using-rust-chrome-and-nixos-to-take-headless-screenshots-for-social-sharing/)
* [Primera auditoría de seguridad de Sudo-RS](https://ferrous-systems.com/blog/sudo-rs-audit/)
* [Destruir árboles de forma segura y barata](https://ismailmaj.github.io/destructing-trees-safely-and-cheaply)
* [Edge IoT con Rust en ESP: NTP](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-ntp)

### Tutoriales de Rust

* [Uso de sockets Linux modernos](https://devork.be/blog/2023/11/modern-linux-sockets/)

### Miscelánea
* [Migración del backend PGP de SecureDrop de GnuPG a Sequoia](https://securedrop.org/news/migrating-securedrops-pgp-backend-from-gnupg-to-sequoia/)
* [video] [10 veces más rápido - haciéndose cargo del backend del compilador](https://www.youtube.com/watch?v=FCVfofYsWHU)
* [video] [Extensión vectorial RISC-V en Rust](https://www.youtube.com/watch?v=jb5-P_r3jmw)
* [Incrustación de un archivo CSV simple en la aplicación Rust](https://rust.code-maven.com/embedding-simple-csv-file)
 
## Crate de la semana

El crate de esta semana es [floem](https://github.com/lapce/floem), una biblioteca nativa de la interfaz de usuario de Rust con reactividad de grano fino.

A pesar de no recibir sugerencias, llogiq está razonablemente satisfecho con su elección.

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Ockam - Hacer que 'ockam identity delete' (sin args) sea interactivo pidiendo al usuario que elija de una lista de nombres de identidad para eliminar (tuify)](https://github.com/build-trust/ockam/issues/6463)
* [Ockam - Hacer que 'ockam tcp-outlet delete' (sin args) sea interactivo pidiendo al usuario que elija de una lista de alias tcp-outlet para eliminar (tuify)](https://github.com/build-trust/ockam/issues/6465)
* [Ockam - Hacer que 'ockam project show' (sin argumentos) sea interactivo pidiendo al usuario que elija de una lista de proyectos para mostrar (tuify)](https://github.com/build-trust/ockam/issues/6473)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

366 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-30..2023-11-06

* ['dropck_outlives' compruebe si el testigo del generador 'needs_drop'](https://github.com/rust-lang/rust/pull/117134)
* [tenga en cuenta 'ref' y 'mut' en el lugar incorrecto para el cambio de nombre de la identificación del patrón](https://github.com/rust-lang/rust/pull/117289)
* [añadir un visitante MIR estable](https://github.com/rust-lang/rust/pull/117417)
* [añadir todos los RPITITs al aumentar param-env con límites GAT en 'check_type_bounds'](https://github.com/rust-lang/rust/pull/117131)
* [Agregar elementos de diagnóstico para algunas de las macros integradas de Core](https://github.com/rust-lang/rust/pull/117596)
* [añadir soporte para fechas de archivo anteriores a la época de Unix en las plataformas de Apple](https://github.com/rust-lang/rust/pull/117451)
* [añadir el 'Span' de la palabra clave 'move' al HIR](https://github.com/rust-lang/rust/pull/117585)
* [también se considera que el TAIT no es computable si el cuerpo MIR está contaminado](https://github.com/rust-lang/rust/pull/117416)
* [evite el recorte de la ruta de pelusa ICE en el informe de errores](https://github.com/rust-lang/rust/pull/117373)
* [evite la comparación innecesaria en 'partition_equal'](https://github.com/rust-lang/rust/pull/117179)
* [Compruebe las carpetas con variables enlazadas para los límites globales que no se cumplen](https://github.com/rust-lang/rust/pull/117637)
* [consts: eliminar el código muerto alrededor de los valores constantes 'i1'](https://github.com/rust-lang/rust/pull/117554)
* [cobertura: reemplazar imposible 'cobertura::Error' con aserciones](https://github.com/rust-lang/rust/pull/117421)
* [derivar 'TyEncodable'/'TyDecodable' en 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/117578)
* [Detectar binop mal analizado causado por la falta de semi](https://github.com/rust-lang/rust/pull/117292)
* [Detectar errores de seguridad de objetos cuando falta el tipo de asociación](https://github.com/rust-lang/rust/pull/116405)
* [no hacer ICE en caso de fallo de evaluación constante en GVN](https://github.com/rust-lang/rust/pull/117438)
* [no afirmar en 'op_to_const'](https://github.com/rust-lang/rust/pull/117441)
* [No comprobar los límites de alias en Liveness cuando los alias tienen variables enlazadas de escape](https://github.com/rust-lang/rust/pull/117466)
* [no emita errores de buena ruta retrasada en pánico](https://github.com/rust-lang/rust/pull/117397)
* [no pases '-stdlib=libc++' al compilar archivos C en macOS](https://github.com/rust-lang/rust/pull/116017)
* [habilitar la inserción entre cajas cuando la inserción MIR está habilitada](https://github.com/rust-lang/rust/pull/117363)
* [Habilitar el front-end paralelo de Rustc en compilaciones nocturnas](https://github.com/rust-lang/rust/pull/117435)
* [reserva para 'construct_generic_bound_failure'](https://github.com/rust-lang/rust/pull/117570)
* [Se corrige la inicialización excesiva y las lecturas más allá de EOF en la especialización 'io::copy(_, Vec)<u8>'](https://github.com/rust-lang/rust/pull/117576)
* [Corregir sugerencia incorrecta de restricción vinculada a rasgos](https://github.com/rust-lang/rust/pull/117505)
* [Se corrigió el orden de las implementaciones en la sección "Implementaciones en tipos foráneos"](https://github.com/rust-lang/rust/pull/117521)
* [garantizar la representación de Ninguno en la OSFL](https://github.com/rust-lang/rust/pull/115333)
* [Garantizar que 'char' tiene el mismo tamaño y alineación que 'U32'](https://github.com/rust-lang/rust/pull/116894)
* [optimizador de sugerencias sobre la capacidad reservada por prueba](https://github.com/rust-lang/rust/pull/117503)
* [en línea y eliminar 'create_session'](https://github.com/rust-lang/rust/pull/117475)
* [asegúrese de que los predicados con variables enlazadas no mencionadas todavía se consideren globales en el solucionador anterior](https://github.com/rust-lang/rust/pull/117589)
* [hacer que la función de aleatorización del aditivo 'rustc_abi' sea aditiva](https://github.com/rust-lang/rust/pull/117603)
* [Hacer coincidir usize/isize exhaustivamente con rangos semiabiertos](https://github.com/rust-lang/rust/pull/116692)
* [Rellene previamente el almacenamiento opaco antes de usarlo](https://github.com/rust-lang/rust/pull/117439)
* [bonitos rasgos 'Fn' en 'rustc_on_unimplemented'](https://github.com/rust-lang/rust/pull/116439)
* [recuperarse de la lista de parámetros faltantes en las definiciones de funciones](https://github.com/rust-lang/rust/pull/117298)
* [Refactorizar: mover funciones de sugerencia de demanda a sugerencias](https://github.com/rust-lang/rust/pull/117401)
* [eliminar el soporte obsoleto para vincular el desenredado en Android](https://github.com/rust-lang/rust/pull/117504)
* [eliminar el soporte para el alias '-Z symbol-mangling-version'](https://github.com/rust-lang/rust/pull/117509)
* [eliminar el soporte para complementos de compilador](https://github.com/rust-lang/rust/pull/116412)
* [reemplazar el cambio a inalcanzable por las declaraciones de asunción](https://github.com/rust-lang/rust/pull/113970)
* [establezca 'max_atomic_width' para riscv32*-esp-espidf en 32](https://github.com/rust-lang/rust/pull/117307)
* [convertir 'const_caller_location' de una consulta a un gancho](https://github.com/rust-lang/rust/pull/117388)
* [use 'FxIndexSet' en el integrador de símbolos](https://github.com/rust-lang/rust/pull/117508)
* [use el derivado para 'Clone'/'PartialOrd'/'Ord'/'Hash' en 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/117407)
* [Usar caché global al calcular árboles de prueba](https://github.com/rust-lang/rust/pull/117394)
* [use el intervalo correcto al emitir el resultado 'env!'](https://github.com/rust-lang/rust/pull/117592)
* [advertir a los usuarios que establecen un nivel de pelusa 'non_exhaustive_omitted_patterns' en un brazo de coincidencia](https://github.com/rust-lang/rust/pull/117094)
* [Al encontrar delimitadores no cerrados durante el lexing, compruebe si hay marcadores de diferencia](https://github.com/rust-lang/rust/pull/116712)
* [habilitar src/math para todos los objetivos UEFI](https://github.com/rust-lang/compiler-builtins/pull/553)
* [Macro intrínseca: Arreglar la generación de aeabi no débil](https://github.com/rust-lang/compiler-builtins/pull/552)
* [Esto habilita el módulo matemático para los objetivos RISCV32](https://github.com/rust-lang/compiler-builtins/pull/554)
* [estabilizar 'const_maybe_uninit_zeroed' y 'const_mem_zeroed'](https://github.com/rust-lang/rust/pull/116218)
* [estabilizar 'file_set_times'](https://github.com/rust-lang/rust/pull/117422)
* [arreglar 'switch_stdout_to' en Windows7](https://github.com/rust-lang/rust/pull/117386)
* [añadir 'track_caller' a 'transmute_copy'](https://github.com/rust-lang/rust/pull/117510)
* [delegar '<Box as Error>::p rovide'<E> a '<E as Error>::p rovide'](https://github.com/rust-lang/rust/pull/117434)
* [soporta variantes de 'enum' en 'offset_of!'](https://github.com/rust-lang/rust/pull/114208)
* [enumeraciones de la puerta de características en 'offset_of'](https://github.com/rust-lang/rust/pull/117537)
* [override 'Waker::clone_from' para evitar clonar 'Waker' innecesariamente](https://github.com/rust-lang/rust/pull/96979)
* [CodeGen\_gcc: Corregir error de compilación de vectores](https://github.com/rust-lang/rustc_codegen_gcc/pull/368)
* [cargo: 'feat(trim-paths)': establece env 'CARGO_TRIM_PATHS' para scripts de compilación](https://github.com/rust-lang/cargo/pull/12900)
* [cargo toml: Saca el esquema](https://github.com/rust-lang/cargo/pull/12911)
* [Cargo: Arreglar un mensaje de pánico inútil](https://github.com/rust-lang/cargo/pull/12923)
* [cargo: implement '-Ztrim-paths'](https://github.com/rust-lang/cargo/pull/12625) (RFC [#3127](https://rust-lang.github.io/rfcs/3127-trim-paths.html))
* [cargo: fusionar 'rutas de recorte' de diferentes perfiles](https://github.com/rust-lang/cargo/pull/12908)
* [rustdoc: aceptar Rust menos inválido](https://github.com/rust-lang/rust/pull/117450)
* [rustfmt: corrige la coma añadida al comentario en la cláusula where](https://github.com/rust-lang/rustfmt/pull/5954)
* [clippy: 'unused_enumerate_index': no hacer hielo en tuplas vacías](https://github.com/rust-lang/rust-clippy/pull/11756)
* [clippy: añadir pelusa 'unused_enumerate_index'](https://github.com/rust-lang/rust-clippy/pull/10404)
* [clippy: arreglar el cálculo de semi span 'dbg_macro'](https://github.com/rust-lang/rust-clippy/pull/11743)
* [clippy: arreglar 'enum_variant_names' dependiendo de la pelusa dependiendo del orden](https://github.com/rust-lang/rust-clippy/pull/11498)
* [clippy: corrige el falso negativo 'get_first' para VecDeque](https://github.com/rust-lang/rust-clippy/pull/11744)
* [clippy: nueva pelusa: 'unnecessary_fallible_conversions'](https://github.com/rust-lang/rust-clippy/pull/11669)
* [Rust-analyzer: añadir asistencia 'generate_mut_trait_impl'](https://github.com/rust-lang/rust-analyzer/pull/15832)
* [rust-analyzer: importar rasgo con alias](https://github.com/rust-lang/rust-analyzer/pull/15788)
* [rust-analyzer: omitir la comprobación del recuento de árboles de tokens para la llamada a la macro include!](https://github.com/rust-lang/rust-analyzer/pull/15819)
* [rust-analyzer: arreglar la ruta de los documentos para derivar macros](https://github.com/rust-lang/rust-analyzer/pull/15834)
* [rust-analyzer: metadatos de vSCode. category:forformates](https://github.com/rust-lang/rust-analyzer/pull/15827)

### Clasificación del rendimiento del compilador de Rust

Una semana difícil para el triaje, debido a lo que parece ser una interrupción a nivel del sistema
aparato de medición, produciendo ruido transitorio (y potencialmente enmascarando
problemas). El principal cambio en el rendimiento sin ruido fue la enorme regresión a los mapas de bits introducida
por PR 117131, y que ya tiene una corrección en vuelo (PR #117542). Lo otro
Vale la pena señalar que el front-end paralelo de rustc se ha habilitado en las compilaciones nocturnas,
lo que ha introducido cierta sobrecarga que era esperada por wg-parallel-rustc.

Triaje realizado por **@pnkfelix**.
Rango de revisión: [650991d6.. 7b97a5ca](https://perf.rust-lang.org/?start=650991d62c3a2c80ba27009d06839adbb038bf5e&end=7b97a5ca8422d1495a8918106d3249aa405812d4&absolute=false&stat=instructions%3Au)

10 regresiones, 4 mejoras, 3 mixtas; 3 de ellos en rollups
68 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/9804362a1bf583dddd7070095e674b0bd6eee887/triage/2023-11-07.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* [Agregar RFC "Actualización de política crates.io"](https://github.com/rust-lang/rfcs/pull/3463)
* [Fusionar RFC 3498: "Reglas de captura de por vida 2024"](https://github.com/rust-lang/rfcs/pull/3498)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Añadir T: ? Con el tamaño de 'RwLockReadGuard' y 'RwLockWriteGuard's Debug impls.](https://github.com/rust-lang/rust/pull/117138)
* [disposición: fusionar] [Problema de seguimiento para 'file_create_new'](https://github.com/rust-lang/rust/issues/105135)
* [disposición: fusionar] [feat: implementar 'DoubleEndedSearcher' para 'CharArray[Ref]Searcher'](https://github.com/rust-lang/rust/pull/111922)
* [disposición: fusionar] [TAIT definiendo las opciones de alcance](https://github.com/rust-lang/rust/issues/107645)
* [disposición: fusionar] [Añadir exportaciones 'std::hash::{DefaultHasher, RandomState}' (necesita FCP)](https://github.com/rust-lang/rust/pull/115694)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Decidir sobre desplazamientos de tamaño cero y accesos a memoria](https://github.com/rust-lang/unsafe-code-guidelines/issues/472)

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Tipos de sí mismos arbitrarios v2.](https://github.com/rust-lang/rfcs/pull/3519)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-11-08 - 2023-12-06 🦀

### Virtual

* 08/11/2023 | Virtual(Boulder, CO, EE. UU.) | [Depósito de Estado Sólido - El Espacio de Creación de Boulder](https://www.meetup.com/solidstatedepot/)
    * [**Marcador de posición: Boulder Rust Meetup**](https://www.meetup.com/solidstatedepot/events/296661062/)
* 09/11/2023 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 34ª edición**](https://www.meetup.com/rust-linz/events/297133538/)
* 09/11/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 10/11/2023 | Virtual (Bangalore, IN) | [Aprende todo sobre programación](https://www.meetup.com/just-code/)
    * [**Introducción a rust-lang**](https://www.meetup.com/just-code/events/297172855/)
* 12/11/2023 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Rust in Israel - Rust Digger**](https://www.meetup.com/code-mavens/events/297064458/)
* 14/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 14/11/2023 | Virtual (Kiev, UA) | [Yalantis Educación](https://www.meetup.com/yeducation/)
    * [**Довгий шлях до першого комерційного досвіду або до чого тут Rust?**](https://www.meetup.com/yeducation/events/297219539/)
* 15/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Construyendo Nuestras Propias Cerraduras (Atómicas y Cerraduras Capítulo 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 15/11/2023 | Virtual (Richmond, VA, EE. UU.) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 15/11/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 16/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 16/11/2023 | Virtual (Vilnius, LT) | [Grupo de Meetup de Vilnius Rust and Go](https://www.meetup.com/vilnius-rust-go-meetup-group/)
    * [**Disfruta de nuestro primer evento de Rust**](https://www.meetup.com/vilnius-rust-go-meetup-group/events/297133832/)
* 21/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 21/11/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/296807537/)
* 23/11/2023 | Virtual (Edmonton, AB, CA) | [Grupo de usuarios de Edmonton R - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Reunión del grupo de usuarios de Edmonton R: R y Rust, como una pareja hecha en el cielo**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 28/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 29/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [** ¡Capítulo final del Club de Lectura de Atomics & Locks! (Capítulo 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 30/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 30/11/2023 | Virtual (Dublín, IE) | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Automatización de la experiencia con comprobaciones de carga de carga**](https://www.meetup.com/rust-dublin/events/296346693/)
* 01/12/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 02/12/2023 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 05/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Espejo**](https://berline.rs/)
* 05/12/2023 | Virtual (Búfalo, NY, EE. UU.) | [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)

### Europa

* 09/11/2023 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/296567395)
* 09/11/2023 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547/)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547)
* 09/11/2023 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 21/11/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Procesamiento de GPU en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 23/11/2023 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### América del Norte

* 08/11/2023 | Boulder, CO, EE. UU. | [Reunión de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**¡Hagamos un bot de Discord!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 14/11/2023 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀 **](https://www.meetup.com/rust-nyc/events/296895126/)
* 14/11/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Noche de Codificación/Hackeo/Aprendizaje Oxidado**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 15/11/2023 | Richmond, VA, EE. UU. + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 16/11/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 16/11/2023 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**¡A Python le encanta Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 16/11/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 21/11/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 22/11/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 28/11/2023 | Pasadena, CA, EE. UU. | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Grupo mensual de Rust**](https://www.meetup.com/thursday-go/events/297062186/)

### Oceanía

* 21/11/2023 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Reunión de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 28/11/2023 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/296391733/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/13yx1dn/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Para que Binder siga satisfaciendo las necesidades de Android, necesitamos mejores formas de gestionar (¡y reducir!) la complejidad sin aumentar el riesgo.
>
> El mayor cambio es, obviamente, la elección del lenguaje de programación. Decidimos utilizar Rust porque aborda directamente una serie de desafíos dentro de Binder a los que nos hemos enfrentado durante los últimos años.

– [Alice Ryhl en la lista de correo del kernel de Linux](https://lore.kernel.org/rust-for-linux/20231101-rust-binder-v1-0-08ba9197f637@google.com/)

¡Gracias a [Vincent de Phily](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1475) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/17r3usm/this_week_in_rust_520/)</small>

