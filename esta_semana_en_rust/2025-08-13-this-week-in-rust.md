---
title: "Esta semana en Rust #71"
number_of_week: 71
description: El crate de esta semana es MOMA, un marco para la aritmética modular de origen móvil, con aplicaciones en teoría de números, criptografía y bioinformática.
date: 2025-08-13
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

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabe qué categoría usar, no dude en enviar un PR de todos modos y simplemente pida a los editores que seleccionen la categoría. -->

### Oficial
* [Anunciando Rust 1.89.0 | Blog de Rust](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/)

### Actualizaciones de proyectos/herramientas
* [TangleGuard: Monitoreo de arquitectura de software exclusivamente para Rust](https://tangleguard.com/)
* [redb v3.0.0 - almacén de clave-valor incrustado puro de Rust](https://github.com/cberner/redb/releases/tag/v3.0.0)
* [wrkflw v0.7.0 - ejecutar y validar acciones de GitHub localmente](https://github.com/bahdotsh/wrkflw/releases/tag/v0.7.0)
* [serde-ply - Serde (de)serializador moderno para archivos Ply](https://www.reddit.com/r/rust/comments/1mp147s/serdeply_modern_speed_convenience_for_a_90s_format/))
* [Quinto cumpleaños de Bevy](https://bevy.org/news/bevys-fifth-birthday/)
* [warp v0.4](https://seanmonstar.com/blog/warp-v04/)

### Observaciones/Pensamientos
* [Construyendo un sistema de archivos FUSE asíncrono en Rust](https://r2cn.dev/blog/building-an-asynchronous-fuse-filesystem-in-rust)
* [Nueve reglas para generalizar su biblioteca de Rust: lecciones de extender RangeSetBlaze a mapas (Parte 1)](https://medium.com/@carlmkadie/nine-rules-for-generalizing-your-rust-library-part-1-9f2b08fb5df4)
* [¡Espera! ¡No generes esa tarea! — Comparación de patrones para el estado mutable en aplicaciones simultáneas](https://taping-memory.dev/concurrency-patterns/)
* [Secreto de envío/sincronización que separa al profesional del aficionado](https://blog.cuongle.dev/p/this-sendsync-secret-separates-professional-and-amateur)
* [hyper HTTP/2 (no) MadeYouReset](https://seanmonstar.com/blog/hyper-http2-didn't-madeyoureset/)
* [¿Estamos enseñando a Rust de manera efectiva?](https://blog.kodewerx.org/2025/08/are-we-teaching-rust-effectively.html)
* [video] [BaM #29 - Mejora del firmware integrado de Rust](https://www.youtube.com/live/5Ca6pQQB-mg?si=yHFQMsDbHEXEfpig)
* [video] [David Sankel – Rust y C++ Interop](https://www.youtube.com/watch?v=xihX4RzStYk)

### Tutoriales de Rust
* [Lecciones aprendidas de la implementación de algoritmos acelerados por SIMD (ChaCha20 / ChaCha12) en Rust puro](https://kerkour.com/rust-simd)
* [Construir con Naz: Capturar el progreso de la construcción en tiempo real de la carga usando secuencias PTY y OSC](https://developerlife.com/2025/08/10/pty-rust-osc-seq/)
* [Conversión de FunctionTrace de C a Rust](https://programsareproofs.com/articles/functiontrace-rust-conversion/)
* [video] [Encuadre de mensajes en Rust e Iroh](https://www.youtube.com/watch?v=h6bBLbcj4Vg)

## Crate de la semana

El crate de esta semana es [MOMA](https://crates.io/crates/moma), un marco para la aritmética modular de origen móvil, con aplicaciones en teoría de números, criptografía y bioinformática.

¡Gracias a [Neil Crago](https://users.rust-lang.org/t/crate-of-the-week/2704/1462) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas de prueba esta semana por [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
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
* [Arcadia - Creación de collage](https://github.com/Arcadia-Solutions/arcadia/issues/71)
* [Arcadia - Recuperar datos de collage](https://github.com/Arcadia-Solutions/arcadia/issues/73)
* [arcadia - Raspador de API para TVDB](https://github.com/Arcadia-Solutions/arcadia/issues/6)
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

464 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-05..2025-08-12

#### Compilador
* [tener en cuenta las tuplas desnudas y los métodos 'Pin' en la lógica de búsqueda de campos](https://github.com/rust-lang/rust/pull/144649)
* [detectar construcción 'struct' con campo privado en campo con valor predeterminado](https://github.com/rust-lang/rust/pull/135846)
* [emita 'StorageLive' y programe 'StorageDead' para los enlaces de 'let'-'else' después de la coincidencia](https://github.com/rust-lang/rust/pull/143028)
* [el tipo de llamada de cola de aplicación está relacionado con el tipo de retorno del cuerpo en borrowck](https://github.com/rust-lang/rust/pull/144917)
* [fortificar comprobaciones predeterminadas de parámetros genéricos](https://github.com/rust-lang/rust/pull/144977)
* [implementar 'stability_implications' sin un visitante](https://github.com/rust-lang/rust/pull/144873)
* [implementar macros de atributos declarativas ('macro_rules!')](https://github.com/rust-lang/rust/pull/144579) (RFC [#3697](https://rust-lang.github.io/rfcs/3697-declarative-attribute-macros.html))
* [mejorar la sugerencia para el "argumento de función faltante" en una llamada multilínea](https://github.com/rust-lang/rust/pull/144966)
* [marcar todas las lints de obsolescencia en la resolución de nombres como denegar por defecto e informar en deps](https://github.com/rust-lang/rust/pull/143929)
* [analizador: recuperar de atributos aplicados a tipos y argumentos genéricos](https://github.com/rust-lang/rust/pull/144195)
* [recuperar 'para PAT = EXPR {}'](https://github.com/rust-lang/rust/pull/145124)
* [conservar la sección '.debug_gdb_scripts'](https://github.com/rust-lang/rust/pull/143679)
* [simplificar lint de código muerto](https://github.com/rust-lang/rust/pull/144863)
* [actualizar 'semicolon_in_expressions_from_macros' de advertir a denegar](https://github.com/rust-lang/rust/pull/144369)
 #### Biblioteca
* [función de estabilización de 'duration_constructors_lite'](https://github.com/rust-lang/rust/pull/145135)
* [función de estabilización 'panic_payload_as_str'](https://github.com/rust-lang/rust/pull/144861)
* [estabilizar 'strict_overflow_ops'](https://github.com/rust-lang/rust/pull/144682)
* [función de estabilización 'unsigned_signed_diff'](https://github.com/rust-lang/rust/pull/144900)
* [estabilizar const 'TypeId::of'](https://github.com/rust-lang/rust/pull/144133)
* [estabilizar loongarch32 asm en línea](https://github.com/rust-lang/rust/pull/144402)
* [constificar los rasgos/impls restantes para 'const_ops'](https://github.com/rust-lang/rust/pull/143949)
* [implementar 'continue_ok' y 'break_ok' para ControlFlow](https://github.com/rust-lang/rust/pull/140267)
* [optimizar 'char::is_alphanumeric'](https://github.com/rust-lang/rust/pull/145027)
* [imprimir ID de hilo en mensaje de pánico](https://github.com/rust-lang/rust/pull/115746)
* ['std::sys::io::io_slice': Agregar tipos UEFI](https://github.com/rust-lang/rust/pull/144350)

#### Carga
* [accediendo a la 'OUT_DIR' de cada script de compilación y en el orden correcto](https://github.com/rust-lang/cargo/pull/15776)

#### Rustdoc
* [buscar: preferir elementos estables en los resultados de búsqueda](https://github.com/rust-lang/rust/pull/141658)
* [corregir el almacenamiento en caché de enlaces intra-doc en las reexportaciones](https://github.com/rust-lang/rust/pull/144970)

#### Clippy
* [arreglar 'infinite_loop' positivo](https://github.com/rust-lang/rust-clippy/pull/15157)
* [no intente calcular el tamaño de un tipo con duraciones de escape](https://github.com/rust-lang/rust-clippy/pull/15434)
* [no pelar para 'wildcard_imports' en macro externa](https://github.com/rust-lang/rust-clippy/pull/15413)
* [corregir la verificación de tipo '&str' en 'from_str_radix_10'](https://github.com/rust-lang/rust-clippy/pull/15410)
* [corregir sugerencia para 'collapsible_if' y 'collapsible_else_if' cuando el 'if' interno está entre paréntesis](https://github.com/rust-lang/rust-clippy/pull/15304)
* [mover la pelusa 'cognitive_complexity' de 'vivero' a 'restricción'](https://github.com/rust-lang/rust-clippy/pull/15415)
* [mover 'crosspointer_transmute' de 'complejidad' a 'sospechoso'](https://github.com/rust-lang/rust-clippy/pull/15403)
* [optimizar la pelusa 'incompatible_msrv'](https://github.com/rust-lang/rust-clippy/pull/15422)
* [optimizar la pelusa 'needless_bool'](https://github.com/rust-lang/rust-clippy/pull/15423)
* [reutilizar la asignación anterior de 'Vec' en el bucle](https://github.com/rust-lang/rust-clippy/pull/15428)

#### Analizador de Rust
* [agregar análisis de tipo de asignación para la finalización de IDE](https://github.com/rust-lang/rust-analyzer/pull/20381)
* [agregar eliminar el literal dbg stmt para 'remove_dbg'](https://github.com/rust-lang/rust-analyzer/pull/20354)
* [agregar write! y writeln! a minicore](https://github.com/rust-lang/rust-analyzer/pull/20409)
* [¡Arregla 'extract_expressions_from_format_string' al escribir!](https://github.com/rust-lang/rust-analyzer/pull/20418)
* [corregir la definición de 'Respuesta' no compatible con LSP](https://github.com/rust-lang/rust-analyzer/pull/20393)
* [solucione el pánico al intentar borrar diagnósticos antiguos mientras no hay nada](https://github.com/rust-lang/rust-analyzer/pull/20434)
* [analizador: se corrige el análisis de la polaridad ligada a rasgos y los vinculadores for](https://github.com/rust-lang/rust-analyzer/pull/20417)

### Triaje de rendimiento del compilador de Rust

Esta semana casi no hubo regresiones, mientras que obtuvimos algunas buenas victorias. Uno de ellos fue [#143684](https://github.com/rust-lang/rust/pull/143684), que actualizó la versión LLVM utilizada por el compilador de Rust a 21.

Triaje realizado por **@kobzol**.
Rango de revisión: [07b7dc90.. 6355cd39](https://perf.rust-lang.org/?start=07b7dc90ee4df5815dbb91ef8e98cb93571230f5&end=6355cd39c81e9699b1925c58d2ed3165bcab1715&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,2%, 0,8%] | 3 |
| Regresiones ❌ <br /> (secundaria) | 0,7% | [0,1%, 1,5%] | 8 |
| Mejoras ✅ <br /> (primaria) | -1,5% | [-22,8%, -0,2%] | 219 |
| Mejoras ✅ <br /> (secundario) | -2,9% | [-18,8%, -0,1%] | 256 |
| Todos ❌✅ (primarios) | -1,5% | [-22,8%, 0,8%] | 222 |

2 regresiones, 2 mejoras, 9 mixtas; 5 de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/bd5a0abbedd81c0dcc604f1b79f7f9e1f02e8139/triage/2025/2025-08-12.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* [Degradar x86_64-apple-darwin de Nivel 1 a Nivel 2 con herramientas de host](https://github.com/rust-lang/rfcs/pull/3841)

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Corregir la duración excesivamente restrictiva en el tipo de retorno 'core::p anic::Location::file'](https://github.com/rust-lang/rust/pull/132087)
* [Problema de seguimiento para 'const_array_each_ref'](https://github.com/rust-lang/rust/issues/133289)
* [Requiere la aprobación de t-infra en lugar de t-release en los aumentos de nivel](https://github.com/rust-lang/rust/pull/144906)
* [const-eval: soporte completo para fragmentos de puntero](https://github.com/rust-lang/rust/pull/144081)
* [No adviertas nunca a ningún lanzamiento 'como' como inalcanzable](https://github.com/rust-lang/rust/pull/144804)
* [Puerto #[enlace] a la nueva infraestructura de análisis de atributos](https://github.com/rust-lang/rust/pull/143193)
* ['c_variadic' : Añadir advertencia de incompatibilidad futura para argumentos '...' sin un patrón fuera de los bloques extern](https://github.com/rust-lang/rust/pull/143619)
* [Reescribir el nuevo analizador de argumentos de atributo](https://github.com/rust-lang/rust/pull/144689)
* [Problema de seguimiento para array::repeat](https://github.com/rust-lang/rust/issues/126695)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Promover aarch64-pc-windows-msvc al nivel 1](https://github.com/rust-lang/rfcs/pull/3817)

*Ningún artículo entró en el período de comentarios finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).* o

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Pasar punteros a 'const' en el ensamblador](https://github.com/rust-lang/rfcs/pull/3848)
* [nuevo] [Incluir Clang en llvm-tools](https://github.com/rust-lang/rfcs/pull/3847)
* [nuevo] [repr(ordenado\_fields)](https://github.com/rust-lang/rfcs/pull/3845)

## Próximos eventos

Rusty Eventos entre 2025-08-13 - 2025-09-10 🦀

### Virtual
* 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
* 2025-08-17 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
* 2025-08-18 | Virtual (Kenia) | [RustaceansKenia](https://lu.ma/RustaceansKenya)
    * [**Rust Embedded Series: 02: Lectura de hojas de datos**](https://lu.ma/6vvg0s9y)
* 2025-08-19 | Virtual (Santa Clara, CA, EE. UU.) | [Comunidad de Extensión de la UCSC](https://www.meetup.com/ucsc-extension-community/events/)
    * [**Programación con Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
* 2025-08-19 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/306757756)
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Híbrido (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
* 2025-08-21 | Virtual (Londres, Reino Unido) | [Conf42: Eventos tecnológicos en línea](https://www.meetup.com/conf42/events/)
    * [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
* 2025-08-21 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
* 2025-08-24 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
* 2025-08-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361442)
* 2025-08-28 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-31 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
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

### Asia
* 2025-08-20 | Seúl, KR | [Rust de Seúl](https://www.meetup.com/rust-seoul-meetup)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310347685)
* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de agosto de 2025**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)

### Europa
* 2025-08-13 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
* 2025-08-13 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944036)
* 2025-08-16 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Embedded - Taller #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
* 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
* 2025-08-19 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-30 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**De los errores al paralelismo y a la preparación para el futuro: lo que hace diferente a Rust**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)

### América del Norte
* 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust)
    * [**Programación de un robot de combate en Rust con Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
* 2025-08-14 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Cross-Magic: proyectos personales, juegos de Rust y utilización de IA**](https://www.meetup.com/pdxrust/events/310364279)
* 2025-08-18 | Denver, CO, EE. UU. | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
    * [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Híbrido (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310321250)
* 2025-08-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 2: Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333117)
* 2025-08-23 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Somerville Union Square, 23 de agosto **](https://www.meetup.com/bostonrust/events/310106302)
* 2025-08-27 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulación de sistemas retro (NES, Gameboy) en Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, 6 de septiembre **](https://www.meetup.com/bostonrust/events/310106310)

### Oceanía
* 2025-08-26 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de agosto**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 - 2025-08-30 | Wellington, Nueva Zelanda | [Forja de Rust](https://rustforgeconf.com/)
    * [**Forja de Rust**](https://rustforgeconf.com/)

### América del Sur
* 2025-08-21 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> * rendimiento sólido y herramientas para optimizarlo aún más: debido a que lo fácil es generalmente lo suficientemente rápido, es rápido desarrollar funciones incluso en un proyecto sensible al rendimiento

– [Alice I Cecile en /r/rust](https://www.reddit.com/r/rust/comments/1mn9plk/bevys_fifth_birthday/n85mol9/)

A pesar de la falta de sugerencias, llogiq se siente bastante bien con su elección.

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1mplhs4/this_week_in_rust_612/)</small>
