---
title: "Esta semana en Rust #73"
number_of_week: 73
description: El crate de esta semana es web-route, una biblioteca para definir y administrar ergonómicamente las rutas del servidor web en Rust.
date: 2025-08-27
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software confiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíenos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!--

Estimados colaboradores de la comunidad:
Lea README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe tener la siguiente forma:

* [Título de la página enlazada](https://example.com/my_article)

Si agrega un enlace a un contenido que no es de texto, prefije el prefijo '[video]' o '[audio]':

* [video] [Título del video vinculado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio vinculado](https://example.com/my_podcast)

Si no sabe qué categoría usar, no dude en enviar un PR de todos modos
y solo pida a los editores que seleccionen la categoría.

-->

### Actualizaciones de proyectos/herramientas
* [StrongBox: cifrado de datos simple y seguro para Rust](https://www.hezmatt.org/~mpalmer/blog/2025/08/27/strong-box-simple-safe-data-encryption-for-rust.html)
* [Mensajería del Danubio - Versión 0.4.0 - Aspectos destacados](https://dev-state.com/posts/danube_update_040/)

### Observaciones/Pensamientos
* [Emulando aarch64 en software usando compilación JIT y Rust](https://pitsidianak.is/blog/posts/2025-08-25_emulating_aarch64_in_software_using_JIT_compilation.html)
* [el núcleo del Rust](https://jyn.dev/the-core-of-rust/)
* [Dificultar el cambio fácil](https://blog.appliedcomputing.io/p/make-the-easy-change-hard)
* [Intentando obtener correctamente los backtraces de error en las bibliotecas de Rust](https://www.iroh.computer/blog/error-handling-in-iroh)
* [El inesperado aumento de productividad de Rust](https://lubeno.dev/blog/rusts-productivity-curve)
* [audio] [Netstack.FM — Episodio 2: Hyper con Sean McArthur](https://netstack.fm/#episode-2)

### Tutoriales de Rust
* [Rust ints a Rust enumeraciones con menos instrucciones](https://sailor.li/ints-to-enums)
* [Interactividad de GPUI: creación de una aplicación de contador](https://blog.0xshadow.dev/posts/learning-gpui/gpui-interactivity/)
* [Rust para ingenieros de JavaScript - Building Connect-4](https://www.afloat.boats/posts/rust-for-javascript-engineers-pt-1)
* [Construyendo un sistema de complementos para Rust: bibliotecas nativas vs lenguaje de scripting vs WASM vs motor de reglas](https://kerkour.com/rust-plugins)
* [Hacer una pequeña pelusa Clippy](https://erk.dev/2025/08/21/clippy-lint)
* [Trampas de plagas](https://andreabergia.com/blog/2025/08/pest-gotchas/)
* [Deficiencias de las macros y cómo superarlas](https://cryptical.xyz/rust/shortcomings-of-macros)
* [video] [Controlador FAT32 integrado mínimo - ¡en Rust!](https://www.youtube.com/watch?v=VcWXn8B9RoE)

### Miscelánea
* [GreptimeDB Rust Client - Una guía completa para inserciones de flujo masivo de alto rendimiento](https://greptime.com/blogs/2025-07-30-greptimedb-rust-guide-bulk-stream-insert)

## Crate de la semana

El crate de esta semana es [web-route](https://crates.io/crates/web-route), una biblioteca para definir y administrar ergonómicamente las rutas del servidor web en Rust.

¡Gracias a [sidrubs](https://users.rust-lang.org/t/crate-of-the-week/2704/1463) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háznoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y / o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

* [greptimedb - Añadir formato de fecha y marca de tiempo a COPIAR A (CSV, JSON)](https://github.com/GreptimeTeam/greptimedb/issues/6287)
* [greptimedb - Agregar opciones de compresión a COPIAR A](https://github.com/GreptimeTeam/greptimedb/issues/6286)
* [greptimedb - Actualmente KILL no puede terminar consultas como INSERT INTO SELECT](https://github.com/GreptimeTeam/greptimedb/issues/6334)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 553 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-19..2025-08-26

#### Compilador
* [degradar 'x86_64-apple-darwin' al Nivel 2 con herramientas de host](https://github.com/rust-lang/rust/pull/145252)
* ['rustc_expand': asegúrese de la pila en 'InvocationCollector::visit_expr'](https://github.com/rust-lang/rust/pull/145410)
* [Cuenta de límites imposibles que hacen lanzamientos de dina a dina aparentemente insatisfactorios](https://github.com/rust-lang/rust/pull/145620)
* [agregar lint contra entero para transmutar el puntero](https://github.com/rust-lang/rust/pull/144531)
* [corregir ICE al validar la transmutación de ZST a 'enumeración' habitado](https://github.com/rust-lang/rust/pull/145791)
* [refactorizar el análisis de atributos para mejorar la ergonomía y algunos diagnósticos](https://github.com/rust-lang/rust/pull/145507)
* [simplificar cachés de intervalo](https://github.com/rust-lang/rust/pull/145505)
* [optimizar ligeramente la lectura de los archivos fuente](https://github.com/rust-lang/rust/pull/145848)
* [miri: tenga en cuenta el tiempo dedicado al rastreo, use RDTSC para un tiempo más rápido](https://github.com/rust-lang/miri/pull/4524)
* [miri: apoyar definiciones débiles](https://github.com/rust-lang/miri/pull/4414)

#### Biblioteca
* ['UnsafePinned::raw_get': sincronizar firma con get](https://github.com/rust-lang/rust/pull/145593)
* ['bufreader::Buffer::backshift': no mueve los bytes de uninit](https://github.com/rust-lang/rust/pull/145538)
* [experimento: rasgo de repetición prestado](https://github.com/rust-lang/rust/pull/145726)
* [corregir el orden de los parámetros para las variantes '_by()' de 'min' / 'max' / 'minmax' en 'std::cmp'](https://github.com/rust-lang/rust/pull/139357)
* [FMT de la base no decimal desenredada](https://github.com/rust-lang/rust/pull/143730)
* [implementación: '#[feature(nonpoison_rwlock)]'](https://github.com/rust-lang/rust/pull/144648)
* [estabilizar 'const_array_each_ref'](https://github.com/rust-lang/rust/pull/143383)
* [función de estabilización 'const_pathbuf_osstring_new'](https://github.com/rust-lang/rust/pull/145464)
* [hashbrown: funciones 'get_inner' para restaurar el rendimiento de la búsqueda](https://github.com/rust-lang/hashbrown/pull/639)

#### Carga
* [carga: sistema de pelusa](https://github.com/rust-lang/cargo/pull/15865)
* [cargo: sugerir sugerencias de espacio de trabajo para dependencias booleanas](https://github.com/rust-lang/cargo/pull/15507)

#### Rustdoc
* [agregar soporte para la expansión de macros en las páginas de código fuente de Rustdoc](https://github.com/rust-lang/rust/pull/137229)
* [hacer que los atributos se representen de manera consistente](https://github.com/rust-lang/rust/pull/145782)
* [renderizar atributos en las secciones Campo y Variantes](https://github.com/rust-lang/rust/pull/145812)

#### Clippy
* [clippy: 'bool_comparison': corregir sugerencia incorrecta con '>'/'<' y macros](https://github.com/rust-lang/rust-clippy/pull/15513)
* [clippy: 'bool_comparison': ya no se suelta pelusa en '!x != y'](https://github.com/rust-lang/rust-clippy/pull/15498)
* [clippy: 'cast_slice_from_raw_parts': comprobar si hay conversión implícita a puntero de corte sin procesar](https://github.com/rust-lang/rust-clippy/pull/15437)
* [clippy: 'ptr_as_ptr': corregir sugerencia incorrecta con 'pointer::cast' y macros](https://github.com/rust-lang/rust-clippy/pull/15514)
* [clippy: 'too_many_lines': solo resalta la firma de la función](https://github.com/rust-lang/rust-clippy/pull/15461)
* [clippy: 'unnecessary_mut_passed': agregar sugerencia estructurada](https://github.com/rust-lang/rust-clippy/pull/15438)
* [clippy: 'unused_unit': no pelusa en los tipos de retorno de cierre](https://github.com/rust-lang/rust-clippy/pull/15549)
* [clippy: mejor verifique 'assign_op_pattern' en el contexto 'const'](https://github.com/rust-lang/rust-clippy/pull/15532)
* [clippy: verifique f16 y f128 en 'float_equality_without_abs'](https://github.com/rust-lang/rust-clippy/pull/15054)
* [clippy: detecta un bucle infinito en 'async fn' que no devuelve '!'](https://github.com/rust-lang/rust-clippy/pull/15545)
* [clippy: no reemplace 'match' por 'if' si algún brazo contiene un enlace](https://github.com/rust-lang/rust-clippy/pull/15352)
* [clippy: arreglar 'unnecessary_safety_comment' no pelusa para la primera línea](https://github.com/rust-lang/rust-clippy/pull/15354)
* [clippy: arreglar 'async_yields_async' macros mal destruídas](https://github.com/rust-lang/rust-clippy/pull/15553)
* [clippy: arreglar 'derivable_impls' sugiere erróneamente en 'derive_const'](https://github.com/rust-lang/rust-clippy/pull/15535)
* [clippy: arreglar 'manual_is_ascii_check': también agregar tipo explícito al linting 'coincide!'](https://github.com/rust-lang/rust-clippy/pull/15492)
* [clippy: fix 'or_then_unwrap': la sugerencia conserva las llamadas macro](https://github.com/rust-lang/rust-clippy/pull/15483)
* [clippy: corregir el falso positivo 'semicolon_inside_block' cuando el atributo sobre expr no está habilitado](https://github.com/rust-lang/rust-clippy/pull/15476)
* [clippy: corregir el falso positivo 'unnested_or_patterns' en estructuras con solo parcelas de campo abreviadas](https://github.com/rust-lang/rust-clippy/pull/15343)

#### Analizador de Rust
* [analizador de Rust: 'replace_arith_op' no aplicable en seleccionados](https://github.com/rust-lang/rust-analyzer/pull/20512)
* [rust-analyzer: agregue la sugerencia de finalización 'ReturnExpr'](https://github.com/rust-lang/rust-analyzer/pull/20507)
* [rust-analyzer: agregar soporte de finalización de let en let-chain](https://github.com/rust-lang/rust-analyzer/pull/20513)
* [Rust-analyzer: Se ha añadido una opción para ocultar los représtamos en las sugerencias de incrustación de ajuste](https://github.com/rust-lang/rust-analyzer/pull/20520)
* [rust-analyzer: corrige la finalización de 'else' en 'let _ = if x {} $0'](https://github.com/rust-lang/rust-analyzer/pull/20518)
* [Rust-analyzer: Corregir el pánico en 'syntax_highlighting'](https://github.com/rust-lang/rust-analyzer/pull/20506)
* [rust-analyzer: corregir la referencia de rust-analyzer-contributors](https://github.com/rust-lang/rust-analyzer/pull/20529)
* [Analizador de Rust: corregir la sangría en 'move_guard_to_arm_body'](https://github.com/rust-lang/rust-analyzer/pull/20509)
* [Analizador de Rust: corregir genéricos opacos](https://github.com/rust-lang/rust-analyzer/pull/20523)
* [Rust-analyzer: mejorar el manejo del punto y coma en 'toggle_macro_delimiter'](https://github.com/rust-lang/rust-analyzer/pull/20534)
* [Analizador de Rust: Recursión infinita mientras se reducen los límites de tipo assoc de los superrasgos](https://github.com/rust-lang/rust-analyzer/pull/20504)
* [Rust-analyzer: hacer que el orden de clasificación de importación siga el estilo de edición 2024](https://github.com/rust-lang/rust-analyzer/pull/20423)
* [rust-analyzer: se hace pasar por carga nocturna al invocar flycheck con '-Zscript'](https://github.com/rust-lang/rust-analyzer/pull/20528)
* [Analizador de Rust: normalizar todos los tipos al finalizar la inferencia](https://github.com/rust-lang/rust-analyzer/pull/20537)
* [rust-analyzer: eliminar llamadas innecesarias a 'salsa::attach()'](https://github.com/rust-lang/rust-analyzer/pull/20502)

### Triaje de rendimiento del compilador de Rust

Muchas regresiones esta semana, principalmente en los puntos de referencia de rustdoc de las características recién agregadas. El resto de la suite vio en su mayoría pequeñas regresiones en pequeños puntos de referencia y también algunas mejoras, en particular de la optimización del análisis del árbol de tokens en el código de macros, la optimización del intervalo y el trabajo en curso en el nuevo solucionador, que aún no está completamente habilitado.

Triaje realizado por **@panstromek**.
Rango de revisión: [239e8b1b.. ee361e8f](https://perf.rust-lang.org/?start=239e8b1b47b34120287ec36b33228c1e177f0c38&end=ee361e8fca1c30e13e7a31cc82b64c045339d3a8&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 3,7% | [0,3%, 34,5%] | 42 |
| Regresiones ❌ <br /> (secundaria) | 2,3% | [0,0%, 53,3%] | 79 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-0,7%, -0,3%] | 9 |
| Mejoras ✅ <br /> (secundario) | -0,9% | [-2,8%, -0,0%] | 30 |
| Todos ❌✅ (primarios) | 3.0% | [-0,7%, 34,5%] | 51 |

5 regresiones, 1 mejora, 7 mixtas; 6 de ellos en rollups
38 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/76b6beef3a67f4c97f61745ea510b4c4a924046f/triage/2025/2025-08-25.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* [RFC: Promover aarch64-pc-windows-msvc al nivel 1](https://github.com/rust-lang/rfcs/pull/3817)

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para nombres de archivo terminados en NUL con '#[track_caller]'](https://github.com/rust-lang/rust/issues/141727)
* [guía de estilo: Ausencia de espacios en blanco finales del documento](https://github.com/rust-lang/rust/pull/145735)
* [Estabilizar parcialmente los conceptos básicos de 'bigint_helper_methods'](https://github.com/rust-lang/rust/pull/144494)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Superrasgo Impl automático](https://github.com/rust-lang/rfcs/pull/3851)
* [nuevo] [RFC: Atributo autogenerado](https://github.com/rust-lang/rfcs/pull/3850)

## Próximos eventos

Rusty Eventos entre 2025-08-27 - 2025-09-24 🦀

### Virtual
* 2025-08-28 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-28 | Virtual (Los Ángeles, CA, EE. UU.) | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Taller de contratos inteligentes impulsados por IA**](https://www.meetup.com/rust-los-angeles/events/310603465)
* 2025-08-31 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Rust de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-02 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de Rust de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcmbfb)
* 2025-09-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763848597)
* 2025-09-07 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002479)
* 2025-09-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361533)
* 2025-09-09 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**De las primeras líneas a los primeros clientes: Carol Nichols sobre la construcción de una carrera en Rust**](https://www.meetup.com/women-in-rust/events/310102318)
* 2025-09-11 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646019)
* 2025-09-11 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust/events/)
    * [**Reunión en línea de San Diego Rust de septiembre de 2025**](https://www.meetup.com/san-diego-rust/events/310326567)
* 2025-09-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002480)
* 2025-09-15 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [** Configuración de Tock OS en un entorno virtual (en línea) - preparación para el 17 de septiembre **](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-16 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)
* 2025-09-18 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-09-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361443)

### África
* 2025-09-09 | Johannesburgo, ZA | [Reunión de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Rust by Example - Primitivas y tipos personalizados**](https://www.meetup.com/johannesburg-rust-meetup/events/310714835)

### Asia
* 2025-09-13 | Hangzhou, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP aún está abierto)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)
* 2025-09-13 - 2025-09-14 | Hangzhou, CN | [GOSIM](https://hangzhou2025.gosim.org/schedule/)
    * [**GOSIM Hangzhou 2025**](https://dev.events/conferences/rust-global-china-and-rust-china-conf-2025-dscrf0e1)
* 2025-09-17 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust septiembre de 2025 en Varonis en Herzeliya**](https://www.meetup.com/rust-tlv/events/310708628)

### Europa
* 2025-08-27 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Interoperabilidad asíncrona de Rust y C++: Prueba de taller de RustConf con Aida Getoeva**](https://www.meetup.com/rust-london-user-group/events/310650028)
* 2025-08-28 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Encuentro de Rust #60 patrocinado por Bang & Olufsen**](https://www.meetup.com/copenhagen-rust-community/events/310591727)
* 2025-08-28 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/310438757)
* 2025-08-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-29 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/310438764)
* 2025-08-30 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**¿Quieres un lenguaje exprimible / moderno / útil / amplio? Elija cuatro**](https://www.meetup.com/rust-and-friends/events/310536614)
* 2025-09-03 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**De los errores al paralelismo y a la preparación para el futuro: lo que hace diferente a Rust**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-04 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Reunión de Rust Gdansk #10**](https://www.meetup.com/rust-gdansk/events/310610993)
* 2025-09-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 - 2025-09-18 | Berlín, DE | [Conferencia Oxidar](https://oxidizeconf.com/)
    * [**Conferencia Oxidize**](https://oxidizeconf.com/)
* 2025-09-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)
* 2025-09-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charlas en Mjolner Informatics**](https://www.meetup.com/rust-aarhus/events/310562343)
* 2025-09-24 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3 - híbrido**](https://www.meetup.com/rust-munich/events/307105978)

### América del Norte
* 2025-08-27 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-08-28 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/310602222)
* 2025-08-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust en Web3 Meetup**](https://www.meetup.com/rust-los-angeles/events/310618705)
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-04 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310547154)
* 2025-09-03 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310547154)
* 2025-09-04 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulación de sistemas retro (NES, Gameboy) en Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, 6 de septiembre **](https://www.meetup.com/bostonrust/events/310106310)
* 2025-09-10 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> JS**](https://www.meetup.com/desert-rustaceans/events/310669989)
* 2025-09-11 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Laberintos y gráficos en Rust**](https://www.meetup.com/utah-rust/events/310674937)
* 2025-09-11 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-14 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, 14 de septiembre **](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-17 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587) | [**Evento de configuración en línea 15 de septiembre **](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-18 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Serie 3 : Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333261)
* 2025-09-18 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310287849)

### Oceanía
* 2025-08-27 - 2025-08-30 | Wellington, Nueva Zelanda | [Forja de Rust](https://rustforgeconf.com/)
    * [**Forja de Rust**](https://rustforgeconf.com/)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> 'clono ()' todo ahora. El Comprobador de Préstamos permite esta pequeña rebelión, esta ineficiencia. Sabe que sufro más sabiendo que mi código no es idiomático. Cada '.clone()' es una confesión de mi fracaso. Cada 'Arco<Mutex>' es un monumento a mi insuficiencia.

– [/u/TheEldenLorrdd en /r/rust](https://reddit.com/comments/1mwmei6)

¡Gracias a [Colin Terry](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1709) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1n1znfa/this_week_in_rust_614/)</small>
