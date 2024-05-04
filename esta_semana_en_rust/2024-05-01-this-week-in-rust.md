---
title: "Esta semana en Rust #20"
number_of_week: 20
description: El crate de esta semana es efs, una implementación del sistema de archivos ext2 sin estándar con planes para agregar otros sistemas de archivos en el futuro.
date: 2024-05-01
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
* [Desenfoque de movimiento, visualizaciones y hermosos renders](https://thisweekinbevy.com/issue/2024-04-29-motion-blur-visualizations-and-beautiful-renders)

### Actualizaciones de proyectos/herramientas
* [Publicado r3bl_trerminal_async v0.5.1](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v051-2024-04-28)
* [minbpe-rs v0.1.0: Puerto de 'minbpe' de Andrej Karpathy a Rust](https://github.com/gnp/minbpe-rs)
* [Retención de mensajes y reproducción con Selium](https://selium.com/news/introducing-selium-log)

### Observaciones/Pensamientos
* [Dejar el desarrollo de juegos de Rust después de 3 años](https://loglog.games/blog/leaving-rust-gamedev/)
* [Las tareas son una abstracción incorrecta](https://blog.yoshuawuyts.com/tasks-are-the-wrong-abstraction/)
* [¿Ir o Oxidarse? Solo escucha a los bots](https://cybernetist.com/2024/04/25/go-or-rust-just-listen-to-the-bots/)
* [Descifrando el Críptico (con Z3 y Rust)](https://hugopeters.me/posts/21/)
* [Entonces, quieres escribir una caja insegura](https://blog.dureuill.net/articles/nolife-0-4/)
* [Diseño de un diseño de memoria eficiente en Rust con uniones y unsafes, o una guía demasiado larga para evitar el envío dinámico](https://alonely0.github.io/blog/unions/)
* [Microservicios basados en eventos usando Kafka y Rust](https://www.shuttle.rs/blog/2024/04/25/event-driven-services-using-kafka-rust)
* [Escribir aserciones asíncronas ergonómicas en Rust](https://www.niklaslong.ch/deadline/)
* [Hacer un script de análisis HTML cien veces más rápido con Rayon](https://medium.com/@sam.van.overmeire/making-an-html-parsing-script-a-hundred-times-faster-with-rayon-5ed952c9011c)
* [Estabilidad de binarios de Rust](https://mirekdlugosz.com/blog/2024/rust-binaries-stability/)
* [audio] [Ratatui con Orhun Parmaksiz :: Estación Rustácea](https://rustacean-station.org/episode/orhun-parmaksiz/)
* [La guía del programador mediocre de Rust](https://www.hezmatt.org/~mpalmer/blog/2024/05/01/the-mediocre-programmers-guide-to-rust.html)

### Tutoriales de Rust
* [Mejorar la experiencia de desarrollo con Serverless Rust en RustRover](https://blog.jetbrains.com/rust/2024/04/26/boosting-dev-experience-with-serverless-rust-in-rustrover/)
* [developerlife.com - Polimorfismo de Rust, dyn, impl, uso de rasgos existentes, objetos de rasgos para pruebas y extensibilidad](https://developerlife.com/2024/04/28/rust-polymorphism-dyn-impl-trait-objects-for-testing-and-extensibiity/)
* [Optimización del rendimiento con flamegraph y Divan](https://hegdenu.net/posts/performance-optimization-flamegraph-divan/)

### Investigación
* [Rust Digger: Hay 4.907 páginas de inicio interesantes de Crate](https://rust-digger.code-maven.com/news/interesting-homepages)

### Miscelánea
* [Escribir un tiempo de ejecución de Wasm en Rust](https://skanehira.github.io/writing-a-wasm-runtime-in-rust/)
* [Patrocinador de GitHub: desarrollador de Rust, Andrew Gallant (BurntSushi)](https://dev.to/szabgab/github-sponsor-rust-developer-andrew-gallant-burntsushi-31lh)
* [Dando a Rust una oportunidad para códecs en el kernel](https://lwn.net/SubscriberLink/970565/ac5ffc2e9ad20f1e/)
* [Zed Decodificado: Cuerda y SumTree](https://zed.dev/blog/zed-decoded-rope-sumtree)
* [Un Iterador de Fibonacci casi infinito](https://rust.code-maven.com/fibonacci-iterator)
* [video] [De C a Rust: Trayendo abstracciones de Rust a Linux embebido](https://www.youtube.com/watch?v=hmQr4fq6sH0)

## Crate de la semana

El crate de esta semana es [efs](https://codeberg.org/RatCornu/efs), una implementación del sistema de archivos ext2 sin estándar con planes para agregar otros sistemas de archivos en el futuro.

Otra semana completamente desprovista de sugerencias, pero Llogiq mantiene la esperanza de no tener que buscar la caja de la próxima semana por sí mismo.

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *No se emitieron convocatorias para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* *Esta semana no se han presentado convocatorias ni presentaciones.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Cierra el 03/06/2024 | Viena, Austria y en línea | Fecha del evento: 2024-10-10
* [Computación científica en Rust 2024](https://scientificcomputing.rs/)| Cierra 14/06/2024 | En línea | Fecha del evento: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

Se presentaron 409 solicitudes de incorporación de cambios [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-23..2024-04-30

* [abortar un proceso cuando se viola la propiedad de FD](https://github.com/rust-lang/rust/pull/124210)
* [Se ha añadido soporte para que las pruebas unitarias run-make-support se ejecuten con bootstrap](https://github.com/rust-lang/rust/pull/124321)
* [ast: generalizar el tipo de visita de artículos](https://github.com/rust-lang/rust/pull/124382)
* [Cobertura: Evite los valores codificados de forma rígida al visitar operaciones lógicas](https://github.com/rust-lang/rust/pull/124508)
* [cobertura: reemplace las opciones booleanas con una enumeración 'CoverageLevel'](https://github.com/rust-lang/rust/pull/124507)
* [debuginfo: estabilizar '-Z debug-macros', '-Z collapse-macro-debuginfo' y '#[collapse_debuginfo]'](https://github.com/rust-lang/rust/pull/120845)
* [delegación: admite el cambio de nombre, y funciones asíncronas, const, extern "ABI" y C-variádicas](https://github.com/rust-lang/rust/pull/122500)
* [denigrar palabra clave en 'edition_2024_compat' lints](https://github.com/rust-lang/rust/pull/123680)
* [deref patterns: lower deref patterns to MIR](https://github.com/rust-lang/rust/pull/122598)
* [detectar un error de préstamo que involucre sub-rebanadas y sugerir 'split_at_mut'](https://github.com/rust-lang/rust/pull/124313)
* [no permitir atributos ambiguos en expresiones](https://github.com/rust-lang/rust/pull/124099)
* [no ICE en consts no válidas al caminar bloques mono-alcanzables](https://github.com/rust-lang/rust/pull/124425)
* [no ice cuando 'codegen_select_candidate' devuelva ambigüedad en el nuevo solucionador](https://github.com/rust-lang/rust/pull/124374)
* [no es fatal al llamar a 'expect_one_of' al recuperar arg en 'parse_seq'](https://github.com/rust-lang/rust/pull/124169)
* [forzar el cierre de los argumentos + el tipo de retorno son WF](https://github.com/rust-lang/rust/pull/123531)
* [arreglar ICE en tipos de parámetros const no válidos](https://github.com/rust-lang/rust/pull/124394)
* [arreglar ICE cuando la cola ADT tiene un error de tipo](https://github.com/rust-lang/rust/pull/124057)
* [corregir un error de memoria débil en TLS en Windows](https://github.com/rust-lang/rust/pull/124281)
* [Mejorar el diagnóstico para la solicitud desconocida '--print'](https://github.com/rust-lang/rust/pull/124333)
* [mejorar el manejo de errores expr→field](https://github.com/rust-lang/rust/pull/124200)
* [marcar uniones no propagables por const en 'KnownPanicsLint' sin llamar a layout](https://github.com/rust-lang/rust/pull/124504)
* [paréntesis de letra bonita alrededor del binario en la coincidencia de sufijo](https://github.com/rust-lang/rust/pull/124269)
* [proporcionar más contexto y sugerencias en los errores de Borrowck que involucran cierres](https://github.com/rust-lang/rust/pull/124136)
* [Registrar la certeza de la llamada 'evaluate_added_goals_and_make_canonical_response' en el candidato](https://github.com/rust-lang/rust/pull/124444)
* [eliminar mayúsculas y minúsculas especiales para 'SimplifiedType' para el siguiente solucionador](https://github.com/rust-lang/rust/pull/124379)
* [cambiar el nombre de 'inhibit_union_abi_opt()' a 'inhibits_union_abi_opt()'](https://github.com/rust-lang/rust/pull/124463)
* [renombrado 'DerivedObligation' a 'WellFormedDeriveObligation'](https://github.com/rust-lang/rust/pull/124381)
* [Requerir que se marquen explícitamente los cierres como corrutinas](https://github.com/rust-lang/rust/pull/123792)
* [restringir la promoción de las llamadas 'const fn'](https://github.com/rust-lang/rust/pull/121557)
* [establecer los atributos escribibles y 'dead_on_unwind' para los argumentos sret](https://github.com/rust-lang/rust/pull/121298)
* [Fortalecer la política de seguimiento de problemas con consecuencias](https://github.com/rust-lang/rust/pull/124334)
* [sugerir ref mut para la asignación de coincidencia de patrones](https://github.com/rust-lang/rust/pull/119650)
* [sugerir el uso de argumentos de tipo directamente en lugar de la restricción de igualdad](https://github.com/rust-lang/rust/pull/122591)
* [use el cumplimiento en la sonda del método, no en la evaluación](https://github.com/rust-lang/rust/pull/122317)
* [Usar sondas de forma más agresiva en el nuevo solucionador](https://github.com/rust-lang/rust/pull/124415)
* [no se permite que los elementos de idioma débil sean '#[track_caller]'](https://github.com/rust-lang/rust/pull/124067)
* [Miri: detectar vtables incorrectos en punteros anchos](https://github.com/rust-lang/rust/pull/124220)
* [miri: 'unix_sigpipe': no inserte DEFAULT, solo úselo desde rustc](https://github.com/rust-lang/miri/pull/3510)
* [miri: añadir '-Zmiri-env-set' para establecer variables de entorno sin modificar el entorno host](https://github.com/rust-lang/miri/pull/3493)
* [miri env: dividir el manejo de variables de entorno de Windows y Unix](https://github.com/rust-lang/miri/pull/3517)
* [Miri: Descriptores de archivo: make write take &mut self](https://github.com/rust-lang/miri/pull/3524)
* [miri: implementar intrínsecos LLVM x86 AVX2](https://github.com/rust-lang/miri/pull/3492)
* [Miri: Hacer que Miri-Script sea una raíz del espacio de trabajo](https://github.com/rust-lang/miri/pull/3512)
* [miri: usa la variable TZ del programa interpretado en 'localtime_r'](https://github.com/rust-lang/miri/pull/3523)
* [miri: windows: soporte básico para GetUserProfileDirectoryW](https://github.com/rust-lang/miri/pull/3502)
* [estabilizar 'inline_const'](https://github.com/rust-lang/rust/pull/104087)
* [estabilizar 'Utf8Chunks'](https://github.com/rust-lang/rust/pull/123909)
* [estabilizar 'non_null_convenience'](https://github.com/rust-lang/rust/pull/124498)
* [estabilizar 'std::p ath::absolute'](https://github.com/rust-lang/rust/pull/124335)
* [estabilizar 'io_error_downcast'](https://github.com/rust-lang/rust/pull/124076)
* [deLLVMize some intrínsecos (use 'u32' en lugar de 'Self' en algunos intrínsecos enteros)](https://github.com/rust-lang/rust/pull/124003)
* [dejar de usar los tipos 'struct' de LLVM para alloca](https://github.com/rust-lang/rust/pull/122053)
* ['thread_local': ser insoportablemente explícito en el código dtor](https://github.com/rust-lang/rust/pull/124387)
* [arreglar 'offset_of!' devolviendo un temporal](https://github.com/rust-lang/rust/pull/124484)
* [relax 'A: Clone' con destino a 'rc::Weak::into_raw_and_alloc'](https://github.com/rust-lang/rust/pull/124432)
* ['PathBuf': reemplaza la transmutación por funciones de acceso](https://github.com/rust-lang/rust/pull/124410)
* [codegen\_gcc: algunas correcciones para aarch64](https://github.com/rust-lang/rustc_codegen_gcc/pull/504)
* [codegen\_gcc: algunas correcciones y soluciones más para Aarch64](https://github.com/rust-lang/rustc_codegen_gcc/pull/508)
* [cargo: alias: Los alias sin subcomandos no deben entrar en pánico](https://github.com/rust-lang/cargo/pull/13819)
* [cargo: pelusas: no siempre heredan pelusas del espacio de trabajo](https://github.com/rust-lang/cargo/pull/13812)
* [instalación de carga: No respetar MSRV para instalaciones no locales](https://github.com/rust-lang/cargo/pull/13790)
* [cargo toml: Sé más contundente con la redundancia de guión bajo/guión](https://github.com/rust-lang/cargo/pull/13798)
* [cargo toml: No advertir dos veces cuando se usa un guión bajo en el espacio de trabajo dep](https://github.com/rust-lang/cargo/pull/13800)
* [cargo toml: Eliminar el soporte de campo de subrayado en 2024](https://github.com/rust-lang/cargo/pull/13804)
* [cargo toml: Advertir, en lugar de fallar la publicación, si se excluye un objetivo](https://github.com/rust-lang/cargo/pull/13713)
* [Cargo Toml: Eliminar el soporte para heredar insignias](https://github.com/rust-lang/cargo/pull/13788)
* [cargo: tenga en cuenta dónde se colocó la pelusa](https://github.com/rust-lang/cargo/pull/13801)
* [Carga: Sistema de limpieza de revestimientos](https://github.com/rust-lang/cargo/pull/13797)
* [cargo: arreglar la entrada de destino en .gitignore](https://github.com/rust-lang/cargo/pull/13817)
* [cargo: Corregir la supresión de advertencias para config.toml vs enlaces simbólicos de compatibilidad con configuración](https://github.com/rust-lang/cargo/pull/13793)
* [bindgen: añadir carga dinámica de variable](https://github.com/rust-lang/rust-bindgen/pull/2812)
* [bindgen: eliminar qué dependencia](https://github.com/rust-lang/rust-bindgen/pull/2809)
* [bindgen: simplificar la conversión de objetivos de Rust a Clang](https://github.com/rust-lang/rust-bindgen/pull/2808)
* [clippy: 'single_match'('_else') puede ser aplicable a la máquina](https://github.com/rust-lang/rust-clippy/pull/12726)
* [clippy: 'non_canonical_partial_ord_impl': Se corrigen las advertencias que entran en conflicto con 'needless_return'](https://github.com/rust-lang/rust-clippy/pull/12702)
* [clippy: 'type_complexity': Corregir errores duplicados](https://github.com/rust-lang/rust-clippy/pull/12736)
* [clippy: comprobar si el cierre como método arg tiene acceso de lectura en 'collection_is_never_read'](https://github.com/rust-lang/rust-clippy/pull/12694)
* [clippy: permite de forma configurable 'useless_vec' en las pruebas](https://github.com/rust-lang/rust-clippy/pull/12725)
* [clippy: arreglar el linaje 'large_stack_arrays' en la macro 'vec'](https://github.com/rust-lang/rust-clippy/pull/12624)
* [clippy: corregir falso positivo en 'cast_possible_truncation'](https://github.com/rust-lang/rust-clippy/pull/12722)
* [clippy: suprimir 'readonly_write_lock' para los enlaces con el prefijo de guión bajo](https://github.com/rust-lang/rust-clippy/pull/12734)
* [Rust-analyzer: código de error diferente de error "No such field" basado en el tipo de variante](https://github.com/rust-lang/rust-analyzer/pull/17131)
* [Rust-analyzer: No reintentar las solicitudes de posición y los datos de resolución de versiones](https://github.com/rust-lang/rust-analyzer/pull/17157)
* [rust-analyzer: corrige atributos en parámetros genéricos que colisionan en el árbol de elementos](https://github.com/rust-lang/rust-analyzer/pull/17151)
* [rust-analyzer: arreglar el desazúcar de los comentarios de doc para proc-macros](https://github.com/rust-lang/rust-analyzer/pull/17153)
* [rust-analyzer: se corrigen los ámbitos de expresión que no se calculan para las consts en línea](https://github.com/rust-lang/rust-analyzer/pull/17135)
* [rust-analyzer: arreglar las raíces de origen que no siempre se crean cuando es necesario](https://github.com/rust-lang/rust-analyzer/pull/17145)
* [Rust-analyzer: hacer que 'Cargo Run' esté siempre disponible para binarios](https://github.com/rust-lang/rust-analyzer/pull/16972)
* [rust-analyzer: manual: eliminar sugerencia de rust-project.json ejemplo](https://github.com/rust-lang/rust-analyzer/pull/17144)
* [Rust-analyzer: Soporta límites flotantes para ADTs](https://github.com/rust-lang/rust-analyzer/pull/17021)
* [rustfmt: corregir sangría incorrecta en el atributo interno](https://github.com/rust-lang/rustfmt/pull/6148)

### Clasificación del rendimiento del compilador de Rust

Varios cambios sin ruido esta semana, con mejoras y regresiones
como resultado. El rendimiento general del compilador es más o menos neutro en todo el
semana.

Triaje realizado por **@simulacrum**.
Rango de revisión: [a77f76e2.. C65b2DC9](https://perf.rust-lang.org/?start=a77f76e26302e9a084fb321817675b1dfc1dcd63&end=c65b2dc935c27c0c8c3997c6e8d8894718a2cb1a&absolute=false&stat=instructions%3Au)

2 regresiones, 2 mejoras, 3 mixtas; 1 de ellos en rollups
51 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-04-29.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [Programa de objetivos del proyecto experimental para 2024 H2](https://github.com/rust-lang/rfcs/pull/3614)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [Captura precisa](https://github.com/rust-lang/rfcs/pull/3617)
* [disposición: fusionar] [Bloqueos externos inseguros](https://github.com/rust-lang/rfcs/pull/3484)
* [disposición: fusionar] [MaybeDangling](https://github.com/rust-lang/rfcs/pull/3336)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Añadir 'Opción::take_if'](https://github.com/rust-lang/rust/issues/98934)
* [disposición: fusionar] [elaborar obligaciones en coherencia](https://github.com/rust-lang/rust/pull/124532)
* [disposición: fusionar] [Permitir la coerción de funciones cuya firma difiere en tipos opacos en su ámbito de definición en un tipo de puntero de función compartida](https://github.com/rust-lang/rust/pull/124297)
* [disposición: fusionar] [Vamos a '#[esperar]' algunas pelusas: 'Estabilizar lint_reasons' (RFC 2383)](https://github.com/rust-lang/rust/pull/120924)
* [disposición: fusionar] [Problema de seguimiento de las funciones de recorte ASCII en segmentos de bytes](https://github.com/rust-lang/rust/issues/94035)
* [disposición: fusionar] [Agregar 'IntoIterator' para 'Box<[T]>' + pelusas específicas de la edición 2024](https://github.com/rust-lang/rust/pull/124097)
* [disposición: fusionar] [Añadir 'Box<[T; N]>: IntoIterator' sin ningún método dispatch hacks](https://github.com/rust-lang/rust/pull/124108)
* [disposición: fusionar] [rustdoc-search: buscar referencias](https://github.com/rust-lang/rust/pull/124148)
* [disposición: cerrar] [Un rasgo adicional limitado hace que el cuerpo de la función no pueda realizar la comprobación de tipos](https://github.com/rust-lang/rust/issues/82219)
* [disposición: fusionar] [Hacer que las conversiones de punteros a objetos de rasgo sean más estrictas](https://github.com/rust-lang/rust/pull/120248)
* [disposición: fusionar] [Problema de seguimiento para split_at_checked](https://github.com/rust-lang/rust/issues/119128)

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Captura precisa](https://github.com/rust-lang/rfcs/pull/3617)

## Próximos eventos

Eventos oxidados entre 2024-05-01 - 2024-05-29 🦀

### Virtual

* 01/05/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 5 - Estructura del proyecto**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 01/05/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/299047895/)
* 02/05/2024 | Virtual (Aarhus, DK) | [Organizadores de Rust Aarhus](https://www.meetup.com/rust-aarhus-organizers/)
    * [**Rust Aarhus Organizadores: Estado**](https://www.meetup.com/rust-aarhus-organizers/events/300416935/)
* 02/05/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 02/05/2024 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Mujeres en Rust: ¡Almuerza y aprende! (Virtual)**](https://www.meetup.com/women-in-rust/events/300208946/)
* 07/05/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 09/05/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477697/)
* 09/05/2024 | Virtual (Israel) | [Rust en Israel](https://rust.org.il/)
    * [**Rust en Microsoft, Tel Aviv - ¿Ya estamos integrados?**](https://www.meetup.com/code-mavens/events/300144781/)
* 09/05/2024 | Virtual (Núremberg/Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 14/05/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341699/)
* 14/05/2024 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/300437775/)
* 14/05/2024 | Virtual + presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - híbrido (Reprogramado)**](https://www.meetup.com/rust-munich/events/298507657/)
* 15/05/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 16/05/2024 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rustful de mediados de mes: análisis forense a través de Artemisa**](https://www.meetup.com/rustdc/events/299346490/)
* 23/05/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477699/)
* 28/05/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/300533392/)

### África

* 04/05/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 11/05/2024 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de mayo de 2024**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europa

* 01/05/2024 | Köln/Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/)
    * [**Este mes en Rust, mayo**](https://www.meetup.com/rustcologne/events/300610856/)
* 01/05/2024 | Utrecht, Países Bajos | [Comunidad NL-RSE](https://nl-rse.org/events/2024-05-01-meetup)
    * [**Reunión de NL-RSE RUST**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 06/05/2024 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europa 2024**](https://europe2024.gosim.org/)
* 07/05/2024 y 08/05/2024 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 07/05/2024 | Oxford, Reino Unido | [Grupo de Encuentro de Oxfrod Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Más Rust - Genéricos, restricciones, seguridad.**](https://www.meetup.com/oxford-rust-meetup-group/events/300567559/)
* 08/05/2024 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/300573716/)
* 09/05/2024 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 14/05/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn mayo de 2024**](https://www.meetup.com/rust-london-user-group/events/300715979/)
* 14/05/2024 | Virtual + presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - híbrido (Reprogramado)**](https://www.meetup.com/rust-munich/events/298507657/)
* 14/05/2024 | Praga, República Checa | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Praga (mayo de 2024)**](https://www.meetup.com/rust-prague/events/300566374/)
* 14/05/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Leyendo Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/299694474/)
* 16/05/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 16/05/2024 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/rust-zurich/)
    * [**Reserve la fecha - Encuentro de mayo**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Desarrollo de software preparado para el futuro por FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Sesión de Dojo de Codificación**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 23/05/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 24/05/2024 | Burdeos, FR | [Rust Burdeos](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discusiones**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-28 - 2024-05-30 | Berlín, DE | [Oxidar](https://oxidizeconf.com/)
    * [**Oxidar Conf 2024**](https://oxidizeconf.com/)

### América del Norte

* 04/05/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, 4 de mayo**](https://www.meetup.com/bostonrust/events/300116701/)
* 08/05/2024 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Rust Social - Ann Arbor**](https://www.meetup.com/detroitrust/events/300763859/)
* 09/05/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/)
    * [**Reunión mensual: ¡Tema por determinar!**](https://www.meetup.com/spokane-rust/events/300020003/)
* 12/05/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Coolidge Corner Brookline, 12 de mayo**](https://www.meetup.com/bostonrust/events/300116747/)
* 14/05/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/300744140/)
* 16/05/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 20/05/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, 20 de mayo**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Función doble de Rust Talk**](https://www.meetup.com/deep-dish-rust/events/300665520/)

### Oceanía

* 02/05/2024 | Ciudad de Brisbane, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**Encuentro de mayo**](https://www.meetup.com/rust-brisbane/events/300647409/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1bpg8b8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> "¡Nunca lo haré!"
> "No, nunca está en la edición de 2024".
> "¡Pero nunca puede ser este año, nunca es nunca!"
> "¡Bueno, estamos tratando de hacer que suceda ahora!"
> "¿Pero nunca es no ahora?" "Quiero decir, técnicamente, ahora nunca es la unidad".
> "Pero, ¿cómo tienes una unidad entera si nunca sucede?"

– [Jubileo sobre Zulip](https://rust-lang.zulipchat.com/#narrow/stream/268952-edition/topic/should.20have.20been.202025.20edition/near/435845944)

¡Gracias a [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1565) por la sugerencia! 

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ci5khm/this_week_in_rust_545/)</small>
