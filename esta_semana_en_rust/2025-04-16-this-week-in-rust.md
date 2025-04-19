---
title: "Esta semana en Rust #56"
number_of_week: 56
description: El crate de esta semana es wgpu.
date: 2025-04-16
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Actualización de los objetivos del proyecto de marzo](https://blog.rust-lang.org/2025/04/08/Project-Goals-2025-March-Update/)

### Boletines
* [El Rustáceo Incrustado Edición #43](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-43)

### Actualizaciones de proyectos/herramientas
* [Shadertoys portado a la GPU Rust](https://rust-gpu.github.io/blog/2025/04/10/shadertoys/)
* [Meilisearch 1.14 - incrustadores compuestos, caché de incrustación, atributos filtrables granulares y recuperación de documentos por lotes por ID](https://www.meilisearch.com/blog/meilisearch-1-14)
* [Consulta de Rust 0.4: Tipos estructurales y otras características nuevas](https://blog.lucasholten.com/rust-query-0-4/)

### Observaciones/Pensamientos
* [Reconstrucción de la interfaz de usuario de Prime Video con Rust y WebAssembly](https://www.infoq.com/presentations/prime-video-rust/)
* [ALP Rust es más rápido que C++](https://spiraldb.com/post/alp-rust-is-faster-than-c)
* [¿Y si el veneno fuera Rust?](https://flak.tedunangst.com/post/what-if-the-poison-were-rust)
* [Una sorprendente optimización del tamaño de la enumeración en el compilador de Rust](https://jpfennell.com/posts/enum-type-size/)
* [Dos años de Rust](https://borretti.me/article/two-years-of-rust)
* [Una arquitectura ECS lite](https://prideout.net/blog/layout_viewer/#progress-reporting-and-state-machine)
* [Una encuesta de 2025 sobre las bibliotecas GUI de Rust](https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html)
* [BTrees, índices invertidos y un modelo para la búsqueda de texto completo](https://ohadravid.github.io/posts/2025-04-08-btrees-and-mental-models/)
* [Reducción de los tiempos de compilación de Rust de 30 a 2 minutos con mil cajas](https://www.feldera.com/blog/cutting-down-rust-compile-times-from-30-to-2-minutes-with-one-thousand-crates)
* [SIMD en zlib-rs (parte 1): Autovectorización y características objetivo](https://tweedegolf.nl/en/blog/153/simd-in-zlib-rs-part-1-autovectorization-and-target-features)
* [Evitando la fragmentación de memoria en Rust con jemalloc](https://kerkour.com/rust-jemalloc)
* [video] [Bevy Basics: Who Observes the Observer](https://www.youtube.com/watch?v=8dJtmt19D_s)

### Tutoriales de Rust
* [Inmersión profunda en el sistema de tipo de Rust desde los GAT hasta el borrado de tipos](https://minikin.me/blog/rust-type-system-deep-dive)
* [Asincrónico desde cero 1: ¿Qué hay en el futuro, de todos modos? | Las divagaciones de Natkr](https://natkr.com/2025-04-10-async-from-scratch-1/)
* [Async from scratch 2: Wake me maybe | natkr's divblings](https://natkr.com/2025-04-15-async-from-scratch-2/)
* [Construyendo un motor de búsqueda desde cero, en Rust: parte 4](https://jdrouet.github.io/posts/202503311500-search-engine-part-4/)
* [Bonitos patrones de máquina de estado en Rust](https://hoverbear.org/blog/rust-state-machine-pattern/)
* [video] [Build with Naz : Macros declarativas en Rust](https://www.youtube.com/watch?v=ZLl2G8tx83s)

### Miscelánea
* [Informe de empleo de marzo de 2025](https://filtra.io/rust/jobs-report/mar-25)
* [Recursos de Rust](https://ongardie.net/misc/rust/)

## Crate de la semana

El crate de esta semana es [wgpu](https://docs.rs/wgpu), una biblioteca de gráficos y computación multiplataforma basada en [WebGPU](https://gpuweb.github.io/gpuweb/).

A pesar de la falta de sugerencias, llogiq está satisfecho con su elección.

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* No se emitieron convocatorias para pruebas esta semana por [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

* [rama - añadir comando de servicio a rama-cli](https://github.com/plabayo/rama/issues/508)
* [rama - añadir soporte para include_dir para ServeDir y relacionados](https://github.com/plabayo/rama/issues/507)
* [rama - añadir módulo curl a rama-http-types](https://github.com/plabayo/rama/issues/509)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

480 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-08..2025-04-15

#### Compilador

* [detectar y proporcionar sugerencias para '&raw EXPR'](https://github.com/rust-lang/rust/pull/139392)
* [no sugiera el uso de 'impl Trait' en el parámetro de cierre](https://github.com/rust-lang/rust/pull/138998)
* [hacer que el compilador sugiera rutas reales en lugar de rutas visibles si las rutas visibles están a través de cualquier ruta oculta del documento](https://github.com/rust-lang/rust/pull/139364)
* [dígale a LLVM sobre etiquetas de nicho imposibles](https://github.com/rust-lang/rust/pull/139098)
* [eliminar 'Nonterminal' y 'TokenKind::Interpolated'](https://github.com/rust-lang/rust/pull/124141)
* [reutilizar la ruta rápida 'dimensionada'](https://github.com/rust-lang/rust/pull/139577)

#### Biblioteca

* [add 'core::intrinsics::simd::{simd_extract_dyn, simd_insert_dyn}'](https://github.com/rust-lang/rust/pull/137447)
* [implementación inicial de 'UnsafePinned' (Parte 1: Libs)](https://github.com/rust-lang/rust/pull/137043)
* [polymorphize 'array::IntoIter's iterator impl](https://github.com/rust-lang/rust/pull/139430)
* [acelera 'String::p ush' y 'String::insert'](https://github.com/rust-lang/rust/pull/124810)
* [std: add 'Output::exit_ok'](https://github.com/rust-lang/rust/pull/139554)

#### Carga

* [añadida resolución de enlace simbólico para 'workspace-path-hash'](https://github.com/rust-lang/cargo/pull/15400)
* [Mensaje de error mejorado cuando la plantilla build-dir var no es válida](https://github.com/rust-lang/cargo/pull/15418)

#### Rustdoc

* [search: add unbox flag to Result aliases](https://github.com/rust-lang/rust/pull/139688)
* [habilitar extensiones de Markdown al buscar doctests](https://github.com/rust-lang/rust/pull/139592)

#### Clippy

* ['arbitrary_source_item_ordering' debería ignorar los módulos de prueba](https://github.com/rust-lang/rust-clippy/pull/14585)
* ['implicit_return': mejor manejo del código asíncrono](https://github.com/rust-lang/rust-clippy/pull/14446)
* [aceptar 'self.cmp(other).into()' como 'PartialOrd' canónico impl](https://github.com/rust-lang/rust-clippy/pull/14573)
* [añadir pelusa 'manual_abs_diff'](https://github.com/rust-lang/rust-clippy/pull/14482)
* [los retornos consecutivos ya no disminuyen el nivel de complejidad cognitiva](https://github.com/rust-lang/rust-clippy/pull/14460)
* [considere las vidas anidadas en 'mut_from_ref'](https://github.com/rust-lang/rust-clippy/pull/14471)
* [manejar correctamente el tipo entre corchetes en 'default_constructed_unit_struct'](https://github.com/rust-lang/rust-clippy/pull/14367)
* [desaprobar 'match_on_vec_items' lint](https://github.com/rust-lang/rust-clippy/pull/14217)
* [no proponga autoderivar 'Clonar' en presencia de campos inseguros](https://github.com/rust-lang/rust-clippy/pull/14559)
* [corrección: 'iter_cloned_collect' falso positivo con impl personalizado 'From'/'IntoIterator'](https://github.com/rust-lang/rust-clippy/pull/14473)
* [corrección: 'map_entry': no emitir lint antes de que se hayan realizado las comprobaciones](https://github.com/rust-lang/rust-clippy/pull/14568)
* [corrección: 'redundant_clone' falso positivo en la superposición de vidas](https://github.com/rust-lang/rust-clippy/pull/14237)
* [varias correcciones para 'manual_is_power_of_two'](https://github.com/rust-lang/rust-clippy/pull/14463)

#### Analizador de Rust

* [ast: devuelve los tipos correctos para los métodos 'make::expr_*'](https://github.com/rust-lang/rust-analyzer/pull/19569)
* [Función de agregar módulos secundarios](https://github.com/rust-lang/rust-analyzer/pull/19255)
* [add normalizeDriveLetter](https://github.com/rust-lang/rust-analyzer/pull/19578)
* [distribuya compilaciones de Linux x64 y aarch64 con optimizaciones PGO](https://github.com/rust-lang/rust-analyzer/pull/19582)
* [arreglar el código de compatibilidad de dyn omitiendo la consulta 'callable_item_signature'](https://github.com/rust-lang/rust-analyzer/pull/19566)
* [Arreglar un pequeño error con efectos catastróficos](https://github.com/rust-lang/rust-analyzer/pull/19558)
* [arreglar un 'ExpressionStore' incorrecto que se pasó](https://github.com/rust-lang/rust-analyzer/pull/19570)
* [prevenir pánicos cuando hay una dependencia cíclica entre cierres](https://github.com/rust-lang/rust-analyzer/pull/19579)
* [Tipo de sombra por módulo](https://github.com/rust-lang/rust-analyzer/pull/19461)
* [ignorar los errores de rustfmt que pueden desencadenar la notificación de errores](https://github.com/rust-lang/rust-analyzer/pull/19576)
* [inferencia de cierre de puerto de rustc](https://github.com/rust-lang/rust-analyzer/pull/19536)

### Clasificación del rendimiento del compilador de Rust

Cambios relativamente pequeños esta semana, nada terriblemente impactante (positivo o negativo).

Triaje realizado por **@simulacrum**.
Rango de revisión: [e643f59f.. 15f58c46](https://perf.rust-lang.org/?start=e643f59f6da3a84f43e75dea99afaa5b041ea6bf&end=15f58c46da79399961a09db0c650a2f90f442e6b&absolute=false&stat=instructions%3Au)

1 Regresiones, 3 Mejoras, 3 Mixtas; 2 de ellos en rollups
35 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-04-14.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Dividir elided_lifetime_in_paths en atado y desatado](https://github.com/rust-lang/rust/pull/120808)
* [Comprobar los tipos de valores predeterminados de los parámetros const](https://github.com/rust-lang/rust/pull/139646)
* [Estabilizar banderas para la compilación cruzada doctest](https://github.com/rust-lang/rust/pull/137096)
* [No eliminar el trivial 'SwitchInt' en el análisis MIR](https://github.com/rust-lang/rust/pull/139042)
* [Implementar un lint para la referencia automática implícita de la desreferencia de puntero sin procesar - tome 2](https://github.com/rust-lang/rust/pull/123239)
* [Implementar 'Predeterminado' para punteros sin procesar](https://github.com/rust-lang/rust/pull/139535)
* [hacer abi_unsupported_vector_types un error grave](https://github.com/rust-lang/rust/pull/139309)
* [Estabilizar cadenas en la edición de 2024](https://github.com/rust-lang/rust/pull/132833)
* [Hacer que la captura de cierre tenga un comportamiento consistente y correcto en torno a los patrones](https://github.com/rust-lang/rust/pull/138961)
* [Estabilizar la función 'cell_update'](https://github.com/rust-lang/rust/pull/134446)

#### Otras áreas
* *No hay artículos ingresados al Período Final de Comentarios esta semana para
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-04-16 - 2025-05-14 🦀

### Virtual
* 16/04/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/306231500)
* 17/04/2025 | Virtual y presencial (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361432)
* 23/04/2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Beyond embedded - Desarrollo de sistemas operativos en Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307036053)
* 24/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820299)
* 24/04/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parte 2: ¡Las computadoras cuánticas no pueden proteger esto contra el Rust!" **](https://www.meetup.com/charlottesville-rust-meetup/events/306679733)
* 03/05/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 05/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592/)
* 07/05/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031663)
* 08/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 13/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020415)

### Asia
* 2025-04-22 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust abril de 2025 en Braavos en Tel Aviv en colaboración con StarkWare**](https://www.meetup.com/rust-tlv/events/306530984)

### Europa
* 2025-04-19 | Estambul, TR | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**Rust Konf Türkiye**](https://kommunity.com/turkiye-rust-community/events/rust-konf-turkiye-91f7b3a6)
* 23/04/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Fusionando Python con Rust usando enlaces C sin procesar**](https://www.meetup.com/london-rust-project-group/events/306644439)
* 24/04/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charla en MFT Energy**](https://www.meetup.com/rust-aarhus/events/305809344)
* 24/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/306911347)
* 24/04/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Code Night**](https://www.meetup.com/rust-manchester/events/306899063)
* 25/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/306911357)
* 2025-04-26 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #11**](https://www.meetup.com/stockholm-rust/events/307164617)
* 29/04/2025 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks abril 2025 Community Showcase**](https://www.meetup.com/rust-london-user-group/events/307212039)
* 29/04/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #76**](https://www.meetup.com/rust-paris/events/306952202)
* 30/04/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Operador de Kubernetes en Rust**](https://www.meetup.com/rust-rhein-main/events/306772838)
* 01/05/2025 | Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Hackers Hike 0x0**](https://www.meetup.com/rust-noris/events/305522254)
* 06/05/2025 - 07/05/2025 | París, FR | [WebAssembly y Rust Meetup](https://www.meetup.com/wasm-rust-meetup/)
    * [**GOSIM AI París 2025**](https://www.meetup.com/wasm-rust-meetup/events/306530699/)
* 06/05/2025 | París, FR | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Paris 2025 (Descuento disponible)**](https://www.meetup.com/wasm-rust-meetup/events/306530699)
* 07/05/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 07/05/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 08/05/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 08/05/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adoptando Rust (Alojado por Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 13/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Anuncio de RustWeek 2025**](https://www.meetup.com/rust-nederland/events/305227330)
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://www.meetup.com/Rust-Nederland/)
    * [**RustWeek 2025**](https://dev.events/conferences/rust-week-2025-utcccotp)
* 14/05/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045447)

### América del Norte
* 17/04/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhcgbwb)
* 17/04/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 1 : Por qué HTMX es malo**](https://www.meetup.com/music-city-rust-developers/events/304333092)
* 17/04/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-22 | Detroit, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/events/)
    * [**Informe de la Reunión y Conferencia de la Comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/307221924)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/307089940)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcgbfc)
 23/04/2025 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Mostrar y contar a la comunidad en Fuel Coworking**](https://www.meetup.com/spokane-rust/events/307228157)
* 24/04/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**3ª 3ª VEZ ¡DIOS MÍO SÍ!**](https://www.meetup.com/rust-atl/events/307152133)
* 25/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Ball Square Rust, 25 de abril**](https://www.meetup.com/bostonrust/events/306844343)
* 01/05/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Reflexiones del Proyecto Capstone SIUE sobre el Rust**](https://www.meetup.com/stl-rust/events/304026152)
* 03/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Boston Common Rust, 3 de mayo**](https://www.meetup.com/bostonrust/events/306845368)
* 08/05/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Tiempo del compilador vs Tiempo de ejecución**](https://www.meetup.com/rust-mx/events/307015601)
* 08/05/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: Un motor de consulta analítica rápido, extensible y modular en Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 11/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Porter Square, 11 de mayo**](https://www.meetup.com/bostonrust/events/306845728)

### Oceanía
* 2025-04-22 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de abril**](https://www.meetup.com/rust-canberra/events/306425557)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> punto flotante IEEE 754, que se enorgullece de proporcionar contraejemplos desde 1985.

– [Johannes Dahlström sobre las partes internas del Rust](https://internals.rust-lang.org/t/highlight-differences-in-assert-eq/22722/4)

¡Gracias a [Ralf Jung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1665) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1k107ip/this_week_in_rust_595/)</small>
