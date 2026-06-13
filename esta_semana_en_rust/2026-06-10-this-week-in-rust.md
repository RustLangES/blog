---
title: "Esta semana en Rust #113"
number_of_week: 113
description: El crate de esta semana es rustion, un servidor bastión SSH.
date: 2026-06-10
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *Esta Semana en Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquetanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[mándanos una solicitud de retirada](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* está desarrollado abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos pueden consultarse en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentras algún error en el número de esta semana, [por favor presenta un RP](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad Rust

<!--

Estimados colaboradores de la comunidad:
Por favor, lee README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe ser del siguiente tipo:

* [Título de la página enlazada](https://example.com/my_article)

Si añades un enlace a un contenido que no sea textual, por favor prefijadlo con '[vídeo]' o '[audio]':

* [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast)

Si no sabes qué categoría usar, siéntete libre de enviar una marca permanente de todas formas
Y simplemente pide a los editores que seleccionen la categoría.

-->

### Oficial
* [Cómo Josh ayuda a Rust a gestionar código en múltiples repositorios](https://blog.rust-lang.org/inside-rust/2026/06/04/how-josh-helps-rust-manage-code-across-multiple-repositories/)
* [Mantenedor destacado: Tiffany Pek Yuan (@tiif)](https://blog.rust-lang.org/inside-rust/2026/06/03/maintainer-spotlight-tiffany-pek-yuan-tiif/)

### Boletines
* [Este mes en Rust OSDev: mayo 2026](https://rust-osdev.com/this-month/2026-05/)
* [El Rustaceo Incrustado Número #73](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-73)

### Actualizaciones de proyectos/herramientas
* [Anunciando stdx, la biblioteca estándar extendida de Rust](https://kerkour.com/stdx)
* [OmniScope 0.2.0 lanzado:FFI herramienta de detección estática basada en LLVM IR](https://medium.com/p/dc57a4631f8b?postPublishedType=initial)
* [Anunciando Asterinas 0.18.0](https://asterinas.github.io/2026/06/04/announcing-asterinas-0.18.0.html)
* [Oryxis SSH 0.8: paneles separados](https://github.com/wilsonglasser/oryxis/releases/tag/v0.8.0)
* [Se lanza Ratatui 0.30.1 - una biblioteca Rust para crear interfaces de usuario de terminal](https://ratatui.rs/highlights/v0301/)
* [@utoo/pack: Una herramienta de construcción de nueva generación basada en Turbopack](https://utoo.land/en/docs/blog/utoopack-intro)
* [Pico de Gallo - un puente de protocolo conectado por USB para desarrollar controladores HAL embebidos en tu portátil](https://felipebalbi.github.io/pico-de-gallo/)
* [kache 0.5.0: diseñando una clave correcta de caché de compilación](https://kunobi.ninja/blog/kache-v0-5-0)
* [Anunciando smb2: un cliente SMB2/3 puro de Rust muy rápido](https://www.veszelovszki.com/a/smb2/)

### Observaciones/Pensamientos
* [Solo Límites](https://smallcultfollowing.com/babysteps/blog/2026/06/09/only-bounds/)
* [Portar nuestro backend de Django a Rust mejoró el uso de infraestructura en un 90%](https://wasmer.io/posts/ported-wasmer-backend-django-to-rust)
* [Comparación y Referencia de Cajas Decimales](https://wubingzheng.github.io/en/Decimal-Crates-Comparison.html) | [Versión china](https://wubingzheng.github.io/zh/Decimal-Crates-Comparison.html)
* [Tablero de tareas de robots TeaQL: una vitrina de Rust TUI para flujos de trabajo empresariales auditables](https://teaql.io/blog/robot-task-board-showcase/)
* [vídeo] [Rayon NO es para juegos - usa esto en su lugar](https://www.youtube.com/watch?v=QFQkqFSg8Z4)
* [audio] [Veo con Anders Hellerup Madsen y Gorm Casper](https://corrode.dev/podcast/s06e05-veo/)

### Guías de Rust
* [serie] [¿Quién dirige tu futuro de Rust? Introducción práctica a Async Rust](https://aibodh.com/posts/async-rust-chapter-1-hands-on-intro-to-async-rust/)
* [Extender MySQL usando Rust](https://villagesql.com/blog/rust/)
* [Aprende punteros inteligentes de Rust y mutabilidad interior construyendo el visor de grafos de commit de git](https://blog.sheerluck.dev/posts/learn-rust-smart-pointers-and-interior-mutability-by-building-git-commit-graph-viewer/)
* [heap underflow: soluciones clásicas de algoritmos en Rust idiomático, ejecutables en el navegador](https://rustarians.com/heap-underflow/)

## Crate de la semana
El crate de esta semana es [rustion](https://github.com/handewo/rustion), un servidor bastión SSH.

¡Gracias a [handewo](https://users.rust-lang.org/t/crate-of-the-week/2704/1610) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [cuda-oxide, bifurcación de Windows - prueba la versión de Windows MSVC en más configuraciones CUDA/Windows](https://github.com/ansidium/cuda-oxide-windows/issues/1)
* [openslate - añadir pruebas unitarias para slugify() en api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/38)
* [openslate - añadir pruebas de integración para Notes CRUD en API/src/Notes.rs](https://github.com/MrSheerluck/openslate/issues/70)
* [OpenSlate - Añadir pruebas de integración para el flujo de autenticación en API/SRC/USERS.RS](https://github.com/MrSheerluck/openslate/issues/96)
* [OpenSlate - Añadir pruebas unitarias para build_fts_query() en API/SRC/search.rs](https://github.com/MrSheerluck/openslate/issues/89)
* [OpenSlate - Añadir pruebas de integración para middleware de autenticación y cerrar sesión en API/SRC/Auth.rs](https://github.com/MrSheerluck/openslate/issues/106)
* [OpenSlate - Añadir pruebas de integración para puntos finales multimedia (capa de base de datos) en API/SRC/media.rs](https://github.com/MrSheerluck/openslate/issues/85)
* [openslate - añadir pruebas unitarias para ext_from_mime() y filename_from_url() en API/SRC/media.rs](https://github.com/MrSheerluck/openslate/issues/40)
* [reliakit - añadir un ejemplo typed_csv a la caja de paraguas](https://github.com/satyakwok/reliakit/issues/91)
* [reliakit - implementar CsvField para char](https://github.com/satyakwok/reliakit/issues/92)
* [reliakit - implementar CsvField para los tipos de dirección core::net](https://github.com/satyakwok/reliakit/issues/107)
* [reliakit - escribir una breve guía de "¿qué bloqueo de resiliencia uso?"(https://github.com/satyakwok/reliakit/issues/95)
* [reliakit - extraer un contador de ventana enrollable reutilizable de RollingBreaker](https://github.com/satyakwok/reliakit/issues/94)
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

526 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-02..2026-06-09

#### Compilador
* [añadir 'externa "tail"' llamando convención](https://github.com/rust-lang/rust/pull/157016)
* [añadir una implementación muy básica de "comptime" en la línea de datos](https://github.com/rust-lang/rust/pull/148820)
* [evitar 'unreachable_code' en los valores de retorno requeridos](https://github.com/rust-lang/rust/pull/157009)
* [limpieza y optimización de 'render_impls'](https://github.com/rust-lang/rust/pull/157540)
* [macros: informar directamente de metavariables no vinculadas](https://github.com/rust-lang/rust/pull/156155)
* [reescribir 'rustc_span::symbol::Interner' para evitar el doble hashing](https://github.com/rust-lang/rust/pull/157252)
* [staticlib oculta símbolos internos](https://github.com/rust-lang/rust/pull/155338)

#### Biblioteca
* [añadir APIs para el plegado de mayúsculas a la biblioteca estándar](https://github.com/rust-lang/rust/pull/154742)
* [añadir API '_value' para literales numéricos en proc-macro](https://github.com/rust-lang/rust/pull/154608)
* [optimizar aún más '<str>SliceIndex' para 'Range<usize>'](https://github.com/rust-lang/rust/pull/156119)
* [mejorar el código TLS marcando la ruta de pánico/init como fría](https://github.com/rust-lang/rust/pull/143511)
* [interpretación: usa 'get_unchecked' para 'TwoWaySearcher'](https://github.com/rust-lang/rust/pull/155607)
* [estabilizar 'PathBuf::into_string'](https://github.com/rust-lang/rust/pull/156840)
* [estabilizar 'Resultado::map_or_default' y 'Opción::map_or_default'](https://github.com/rust-lang/rust/pull/156222)

#### Carga
* [quitar CR de 'cargo:token-de-stdout'](https://github.com/rust-lang/cargo/pull/17081)

#### Rustdoc
* [IXCRE: conserva los límites de tamaño en los parámetros de tipo pertenecientes al elemento padre](https://github.com/rust-lang/rust/pull/157262)
* [no enlazar proyecciones de tipo asociadas a 'doc(hidden)'](https://github.com/rust-lang/rust/pull/157438)
* [corregir rasgo implica orden](https://github.com/rust-lang/rust/pull/157233)
* [render restricción 'impl'](https://github.com/rust-lang/rust/pull/157310)

#### Clippy
* [apoyo a 'iter_mut' en 'ITER_NEXT_SLICE'](https://github.com/rust-lang/rust-clippy/pull/17122)
* ['borrowed_box': mensaje de limpieza y mejora de sugerencia](https://github.com/rust-lang/rust-clippy/pull/17173)
* ['double_must_use': hacer que la máquina de pelusa sea aplicable en un caso de un solo atributo](https://github.com/rust-lang/rust-clippy/pull/17144)
* ['iter_cloned_collect': separar la sugerencia del mensaje principal](https://github.com/rust-lang/rust-clippy/pull/17174)
* [añadir pelusa de 'manual_isolate_lowest_one'](https://github.com/rust-lang/rust-clippy/pull/17037)
* [detectar más distancias en 'single_range_in_vec_init'](https://github.com/rust-lang/rust-clippy/pull/17146)
* [no activar 'inline_trait_bounds' en código auto-derivado](https://github.com/rust-lang/rust-clippy/pull/17131)
* [extiende 'extra_unused_lifetimes' por el espurio 'for<'a>'](https://github.com/rust-lang/rust-clippy/pull/17031)
* ['large_const_arrays': comprobar grandes matrices anidadas](https://github.com/rust-lang/rust-clippy/pull/17141)
* [corregir 'explicit_counter_loop' falso positivo cuando el contador solo se modifica dentro del bloque 'else' de 'deja... encuadernación de otra manera](https://github.com/rust-lang/rust-clippy/pull/17023)
* [arreglar 'result_large_err' y 'result_unit_err' no se activan en funciones asíncronas](https://github.com/rust-lang/rust-clippy/pull/17130)
* [sugerencias de corrección de 'unused_async_trait_impl' con sentencias de retorno](https://github.com/rust-lang/rust-clippy/pull/17181)
* [corregir duplicaciones de lints en 'unknown_attribute' y 'renamed_builtin_attr'](https://github.com/rust-lang/rust-clippy/pull/17164)
* [obtener los metadatos de un puntero const es una operación const](https://github.com/rust-lang/rust-clippy/pull/17121)
* [perf: evitar clonar objetos asociados en 'empty_line_after'](https://github.com/rust-lang/rust-clippy/pull/17135)
* [perf: salta el paseo 'boxed_local' para funciones sin parámetro Box](https://github.com/rust-lang/rust-clippy/pull/17168)
* [perf: saltarse la caminata de relevancia 'inline_always' para objetos sin el atributo](https://github.com/rust-lang/rust-clippy/pull/17137)

#### Analizador de Rust
* ['feat(diagnostics)': error de emisión para inferir vars en contextos no de inferencia](https://github.com/rust-lang/rust-analyzer/pull/22469)
* [adopta la política de IA de UV](https://github.com/rust-lang/rust-analyzer/pull/22505)
* [distribuir ventanas construidas con mimalloc](https://github.com/rust-lang/rust-analyzer/pull/22495)
* [el campo inferior se apunta por defecto a 'rustc_type_ir::Const](https://github.com/rust-lang/rust-analyzer/pull/22481)
* ['RunnableKind::Test' debería corresponder a 'project_json::RunnableKind::TestOne'](https://github.com/rust-lang/rust-analyzer/pull/22522)
* ['extract_function' no corresponde a '&mut' para 'container[i].mut_method()'](https://github.com/rust-lang/rust-analyzer/pull/22523)
* [no emitir un error de "anotaciones de tipo necesarias" en 'include_bytes! ()' donde no se puede inferir la longitud del array](https://github.com/rust-lang/rust-analyzer/pull/22520)
* [no generar parámetros genéricos no usados en el signo de rasgo](https://github.com/rust-lang/rust-analyzer/pull/22519)
* [analizar los tipos de patrón OR](https://github.com/rust-lang/rust-analyzer/pull/22524)
* [renombrar subItems del esquema con 'sub_items'](https://github.com/rust-lang/rust-analyzer/pull/22444)
* [implementar extensión LSP 'rust-analyzer/evaluatePredicate'](https://github.com/rust-lang/rust-analyzer/pull/22448)
* [analizar variantes 'enum' sin nombre](https://github.com/rust-lang/rust-analyzer/pull/22512)

### Triaje de rendimiento del compilador Rust

Una semana bastante ruidosa, con un montón de pequeñas regresiones contenidas,
lo que lleva a un ligero aumento promedio en el número de instrucciones. Esta semana tuvo un
Muchos rollups grandes, probablemente por problemas de CI, pero por suerte muchos
Esos venían con resultados de perf preclasificados para cuando llegaron (gracias a esos
¡triadores!). Regresiones ligeras más o menos similares en ciclos y tiempos de pared
La semana.

Triaje hecho por **@simulacrum**.
Rango de revisión: [4804ad7e.. f3EF3Bd8](https://perf.rust-lang.org/?start=4804ad7e93e1b31f4605b7083871d0d3d85a2afe&end=f3ef3bd882dd24a275a60701a67c3bb330edd8c1&absolute=false&stat=instructions%3Au)

2 regresiones, 0 mejoras, 10 mixtas; 5 de ellos en rollups
32 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-06-08.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [`#! [register_{atributo,lint}_tool]'](https://github.com/rust-lang/rfcs/pull/3808)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Pánico de documentos en 'RangeInclusive::from(legacy::RangeInclusive)'](https://github.com/rust-lang/rust/pull/155421)
* [Problema de seguimiento para la cuerda explicit-endian::from_utf16](https://github.com/rust-lang/rust/issues/116258)
* [Problema de seguimiento para 'substr_range' y métodos relacionados](https://github.com/rust-lang/rust/issues/126769)
* [Decidir y documentar dónde se permite que los intrínsecos de las stdarcas divergan del comportamiento asm](https://github.com/rust-lang/rust/issues/153990)
* [Documentar que la interacción de la caja de 'ManuallyDrop' ha sido corregida](https://github.com/rust-lang/rust/pull/155750)
* [Añadir alcance temporal a assert_eq y assert_ne](https://github.com/rust-lang/rust/pull/155739)
* [Limpiar los nombres de tipos de cajas para arreglar la confusión entre dylib y staticlib](https://github.com/rust-lang/rust/issues/153863)
* [Añadir límites 'T: ParcialEq' a 'EstructuralParcialEq' derivado implica la que se siente.](https://github.com/rust-lang/rust/pull/156807)
* [función de estabilización 'float_algebraic'](https://github.com/rust-lang/rust/pull/157029)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [¡Negar todo! () en orden](https://github.com/rust-lang/compiler-team/issues/999)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Oxidar a todos 2027](https://github.com/rust-lang/leadership-council/issues/300)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC para captura de cierre conveniente y explícita usando expresiones move($expr)](https://github.com/rust-lang/rfcs/pull/3968)

## Próximos eventos

Eventos Rusty entre el 10-06-2026 - el 08-07-2026 🦀

### Virtual
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/3bcnx1jb)
* 2026-06-12 | Virtual (Kenia, KE) | [RustaceansKenya](https://luma.com/user/rustaceanskenya)
    * [**ÓXIDO PARA LA TECNOLOGÍA CÍVICA**](https://luma.com/vuxir9w8)
* 2026-06-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/314985751/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ekws5nr4)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: ¿Qué demonios son las mónadas - y cómo las falsificamos en Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjckbcb/)
* 2026-07-02 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-05 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315060981/)
    
### Europa
* 2026-06-10 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en junio: Rust rápido**](https://www.meetup.com/rustcologne/events/315090338/)
* 2026-06-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin en localización 🏳️ 🌈 - Edición 014**](https://www.meetup.com/rust-berlin/events/315088919/)
* 2026-06-11 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Cracovia, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactivo: Todo es de código abierto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Planificación en tiempo real en Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla en Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-18 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**21º encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/315094938/)
* 2026-06-19 | Dresde, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Segundo encuentro**](https://pretix.eu/rust-dresden/on-location-2)
* 2026-06-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-23 | Varsovia, PL | [Varsovia Oxidada](https://luma.com/rust.in.warsaw)
    * [**Rust Warsaw Meetup: junio 2026**](https://luma.com/djs7ntfx)
* 2026-06-25 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-07-02 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Bevy, Bits, & Cats (Charlas de Rust July)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, OV, NL | [Reuniones de Tecnología de Baseflow](https://www.meetup.com/dutch-rust-meetup/events/)
    * [**Cumbre IA**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Únete a nosotros en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/315150327/)

### Norteamérica
* 2026-06-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de junio de Utah Rust**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314989012/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Solucionadores de restricciones basados en Rust en bocetos 2D con Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Gran Fiesta de Verano de Rust NYC**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-07-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**¿Git es fácil?**](https://www.meetup.com/stl-rust/events/315103359/)

### Oceanía
* 2026-06-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/events/)
    * [**Rust Brisbane • junio 2026**](https://www.meetup.com/rust-brisbane/events/315092980/)
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne junio 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### Sudamérica
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Es una pistola de pie, sí, pero es una pistola de pie.

– [Prof. Dr. Ralf Jung en github](https://github.com/rust-lang/rust/pull/155750#discussion_r3356323620)

¡Gracias a [Teemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1779) por la sugerencia!

[¡Por favor, enviad citas y votad para la semana que viene!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

Esta semana en el Rust está editado por:

* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [Marianne Goldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

*El alojamiento de la lista de correo está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1u2rz3a/this_week_in_rust_655/)</small>
