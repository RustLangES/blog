---
title: "Esta semana en Rust #68"
number_of_week: 68
description: El crate de esta semana es oxvg, un optimizador SVG.
date: 2025-07-16
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
* [crates.io: actualización de desarrollo | Blog de Rust](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/)
* [Llamado a Pruebas: Acelerando la compilación con 'hint-mostly-unused'](https://blog.rust-lang.org/inside-rust/2025/07/15/call-for-testing-hint-mostly-unused/)

### Boletines
* [Este mes en Rust OSDev: junio de 2025](https://rust-osdev.com/this-month/2025-06/)

### Actualizaciones de proyectos/herramientas
* [egui 0.32 - Átomos, popups, y mejor soporte SVG](https://github.com/emilk/egui/releases/tag/0.32.0)
* [reqwest retintentos](https://seanmonstar.com/blog/reqwest-retries/)
* [Presentación de Rudy: Una cadena de herramientas para Rust Debuginfo](https://www.samjs.io/blog/rudy)
* [RootAsRole 3.1.0 - Supera las características sudo, configurabilidad, -u, -g, -E](https://github.com/LeChatP/RootAsRole/releases/tag/v3.1.0)
* [Aspectos destacados de la versión 25.07 de Helix](https://helix-editor.com/news/release-25-07-highlights/)
* [UltraGraph 0.8: Análisis de gráficos 1.300 veces más rápido, no se necesita clúster](https://deepcausality.com/blog/announcement-ultragraph-0-8)

### Observaciones/Pensamientos
* [funciones de colocación](https://blog.yoshuawuyts.com/placing-functions/)
* [Rust es una gran opción para la era agentica](https://kerkour.com/rust-agentic-coding)
* [Aquí viene el sol: construyendo iterativamente un programa de Rust que obtiene las condiciones climáticas actuales](https://bitfieldconsulting.com/posts/here-comes-sun)
* [Pensando en Rust: Propiedad, Acceso y Seguridad de la Memoria](https://cocoindex.io/blogs/rust-ownership-access/)
* [Añadiendo lookbehinds a rust-lang/regex](https://systemf.epfl.ch/blog/rust-regex-lookbehinds/)
* [Publica todas tus cajas en todas partes de una sola vez](https://www.tweag.io/blog/2025-07-10-cargo-package-workspace/)
* [Ideas de genéricos variádicos que no funcionarán para Rust](https://poignardazur.github.io/2025/07/09/variadic-generics-dead-ends/)
* [audio] [Trazabilidad](https://sdr-podcast.com/episodes/traceability/)
* [KSAT con Vegard Sandengen](https://corrode.dev/podcast/s04e07-ksat/)

### Tutoriales de Rust
* [Programación de Tipos de Datos Extensibles en Rust con CGP - Parte 3: Implementación de Registros Extensibles](https://contextgeneric.dev/blog/extensible-datatypes-part-3/)
* [Axum: Optimizando el diseño de la API web con el Builder Pattern](https://medium.com/@adefemiadeoye/axum-optimizing-web-api-design-with-the-builder-pattern-08aa8e18a599)
* [Tipos de prueba de prueba unitaria de Rust](https://jorgeortiz.dev/posts/rust_unit_testing_test_types/)

### Investigación
* [Préstamo de árboles](https://plf.inf.ethz.ch/research/pldi25-tree-borrows.html)
* [Protección de Rust mixto con capacidades de hardware](https://arxiv.org/abs/2507.03344)

### Miscelánea
* [Informe de empleos de Rust de junio de 2025](https://filtra.io/rust/jobs-report/jun-25)

## Crate de la semana

El crate de esta semana es [oxvg](https://github.com/noahbald/oxvg), un optimizador SVG.

¡Gracias a [Noah Baldwin](https://users.rust-lang.org/t/crate-of-the-week/2704/1450) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [diesel: Designing '#[derive(QueryModel)]' Poll I - por defecto para #[diesel(check_for_backend()]](https://github.com/diesel-rs/diesel/discussions/4680)
* [Diesel: Designing '#[derive(QueryModel)]' Poll II - Comportamiento de unión para #[diesel(embed)]](https://github.com/diesel-rs/diesel/discussions/4681)
* [Diesel: Diseñando '#[deriva(QueryModel)]' Poll III - ¿Todo en uno derivar? ](https://github.com/diesel-rs/diesel/discussions/4682)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 421 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-08..2025-07-15

#### Compilador
* [use lld por defecto en el establo 'x86_64-unknown-linux-gnu'](https://github.com/rust-lang/rust/pull/140525)
* [aplicar efectos al borde 'de otro modo' en el análisis de flujo de datos](https://github.com/rust-lang/rust/pull/142707)
* [calcular todas las críticas de un rasgo](https://github.com/rust-lang/rust/pull/143783)
* [considere casos anidados para RPITIT duplicado](https://github.com/rust-lang/rust/pull/143570)
* [propagar desde locales prestados en CopyProp](https://github.com/rust-lang/rust/pull/143624)
* [resolver: refactorizar el mapa de macros en mapas externos y locales](https://github.com/rust-lang/rust/pull/143657)

#### Biblioteca
* [constificar los rasgos 'Fn*'](https://github.com/rust-lang/rust/pull/143640)
* [constificar 'Desde' y 'Hasta'](https://github.com/rust-lang/rust/pull/143774)
* [hacer const 'Default' y añadir algunos impls 'const Default'](https://github.com/rust-lang/rust/pull/134628)
* [rebanada: marque 'rotate_left', 'rotate_right' const de manera inestable](https://github.com/rust-lang/rust/pull/143554)
* [core: add 'BorrowedCursor::with_unfilled_buf'](https://github.com/rust-lang/rust/pull/142885)
* [implementar la función 'int_format_into'](https://github.com/rust-lang/rust/pull/142098)

#### Carga
* [añadir la tabla '[hints]' en 'Cargo.toml', y una sugerencia 'hints.mostly-unused'](https://github.com/rust-lang/cargo/pull/15673)
* [implementación y pruebas para 'multiple-build-scripts'](https://github.com/rust-lang/cargo/pull/15704)
* [perf: acelerar el análisis sintáctico de TOML actualizando toml](https://github.com/rust-lang/cargo/pull/15736)

#### Rustdoc
* [no marque '#[target_feature]' fns seguro como inseguro en rustdoc JSON](https://github.com/rust-lang/rust/pull/143555)

#### Clippy
* ['arithmetic_side_effects': no advertir en 'NonZeroU*.get() - 1'](https://github.com/rust-lang/rust-clippy/pull/15238)
* ['or_fun_call': llamadas al método lint dentro de 'map_or' primer argumento](https://github.com/rust-lang/rust-clippy/pull/15074)
* ['{flat_,}map_identity': reconocer '|[x, y]| [x, y]' como una función de identidad también](https://github.com/rust-lang/rust-clippy/pull/15229)
* [añadir 'uninlined_format_args' ejemplo para '{:?}«](https://github.com/rust-lang/rust-clippy/pull/15228)
* [no eliminar la llamada al método si se ajusta el tipo](https://github.com/rust-lang/rust-clippy/pull/15181)
* [arreglar 'approx_const' para algunos casos nuevos](https://github.com/rust-lang/rust-clippy/pull/15236)
* [arreglar 'expect_fun_call' produciendo sugerencias inválidas](https://github.com/rust-lang/rust-clippy/pull/15122)
* [Arreglar la sugerencia de 'legacy_numeric_constants' cuando la llamada está envuelta entre paréntesis](https://github.com/rust-lang/rust-clippy/pull/15191)
* [arreglar 'manual_abs_diff' sugiere erróneamente detrás de los árbitros](https://github.com/rust-lang/rust-clippy/pull/15265)
* [arreglar 'manual_assert' sugiere erróneamente para macros](https://github.com/rust-lang/rust-clippy/pull/15264)
* [corregir la generación de condiciones 'manual_is_variant_and'](https://github.com/rust-lang/rust-clippy/pull/15206)
* [corregir falso negativo de 'expect_used'](https://github.com/rust-lang/rust-clippy/pull/15253)
* [arreglar el manual es múltiplo de](https://github.com/rust-lang/rust-clippy/pull/15205)
* [arreglar múltiples problemas en #15063](https://github.com/rust-lang/rust-clippy/pull/15070)
* [la sugerencia de corrección causa el error de 'needless_for_each'](https://github.com/rust-lang/rust-clippy/pull/15262)
* [Saltar salida tardía lint pasar en las pruebas](https://github.com/rust-lang/rust-clippy/pull/15222)

#### Analizador de Rust
* [rust-analyzer: generate 'new' for tuple 'struct'](https://github.com/rust-lang/rust-analyzer/pull/20109)
* [Analizador de Rust: admite lista de arg multilínea plegable y cuerpo FN en un rango de plegado](https://github.com/rust-lang/rust-analyzer/pull/20054)
* [rust-analyzer: tipo de assoc donde la posición de la cláusula](https://github.com/rust-lang/rust-analyzer/pull/20235)
* [rust-analyzer: arreglar la visualización de la sintaxis 'use<>'](https://github.com/rust-lang/rust-analyzer/pull/20228)
* [rust-analyzer: correcciones para la sugerencia de incrustación 'dyn'](https://github.com/rust-lang/rust-analyzer/pull/20212)
* [Rust-Analyzer: correcciones de ASM en línea](https://github.com/rust-lang/rust-analyzer/pull/20210)
* [Rust-analyzer: normalizar los tipos de proyección antes de calcular los mapas de memoria](https://github.com/rust-lang/rust-analyzer/pull/20232)
* [rust-analyzer: perf: pon el material de expresión en el almacén de expresiones detrás de una 'Opción<Box>'](https://github.com/rust-lang/rust-analyzer/pull/20219)

### Clasificación del rendimiento del compilador de Rust

Una semana ocupada con muchos rollups que contienen perf. Regresiones y resultados mixtos. En general, las regresiones ganaron ligeramente, pero también hubo algunas victorias impresionantes en algunos puntos de referencia primarios y secundarios. Mucho rendimiento. Los efectos son causados por el rediseño actual del análisis de atributos, que se espera que resulte en un rendimiento ligeramente mejorado una vez que esté terminado.

Triaje realizado por **@kobzol**.
Rango de revisión: [0d11be5a.. A9FB6103](https://perf.rust-lang.org/?start=0d11be5aabe0cd49609fff5fce57c4691a22fe55&end=a9fb6103b05c6ad6eee6bed4c0bb5a2e8e1024c6&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.1%, 1.5%] | 62 |
| Regresiones ❌ <br /> (secundaria) | 0.5% | [0.1%, 1.8%] | 78 |
| Mejoras ✅ <br /> (primario) | -0,4% | [-3.9%, -0.1%] | 40 |
| Mejoras ✅ <br /> (secundaria) | -1,4% | [-11.6%, -0.0%] | 74 |
| Todos ❌✅ (primarios) | 0.1% | [-3.9%, 1.5%] | 102 |

5 regresiones, 4 mejoras, 8 mixtas; 5 de ellos en rollups
47 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/87d471ce81dd139cca60ee46377a4cf5c131f7cc/triage/2025/2025-07-15.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Garantía de 8 bytes de alineación en Thread::into_raw](https://github.com/rust-lang/rust/pull/143859)
* [rustdoc: añadir formas de colapsar todos los bloques impl](https://github.com/rust-lang/rust/pull/141663)
* [Estabilizar 'const_float_round_methods'](https://github.com/rust-lang/rust/pull/143604)
* [Problema de seguimiento para '#! [característica(const_float_round_methods)]'](https://github.com/rust-lang/rust/issues/141555)
* [Añadir 'target_env = "macabi"' y 'target_env = "sim"'](https://github.com/rust-lang/rust/pull/139451)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
* [Cerrando cuestiones relevantes para T-lang en este repositorio](https://github.com/rust-lang/rfcs/issues/3756)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [repr(escalable)](https://github.com/rust-lang/rfcs/pull/3838)

## Próximos eventos

Eventos oxidados entre 2025-07-16 - 2025-08-13 🦀

### Virtual
* 16/07/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307731031)
* 17/07/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 17/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Cajas, Consejos y Trucos Charlas Relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/307560304)
* 24/07/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874)
* 27/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhckbkc)
* 31/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
* 02/08/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
* 03/08/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
* 06/08/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)
* 2025-08-10 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbnb)
* 12/08/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305361531)

### Asia
* 19/07/2025 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #11**](https://www.meetup.com/rustdelhi/events/308666751)
* 26/07/2025 | Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de julio de 2025**](https://hasgeek.com/rustbangalore/july-2025-rustacean-meetup/)

### Europa
* 23/07/2025 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/)
    * [**Rust Dortmund Meetup - Enseñar y Hackear**](https://www.meetup.com/rust-dortmund/events/308517530/)
* 24/07/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Charlas de julio: Un cangrejo, un pez globo y una IA de ajedrez de última generación**](https://www.meetup.com/rust-and-friends/events/308687848)
* 24/07/2025 | Núremberg/Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874/)
* 26/07/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #15**](https://www.meetup.com/stockholm-rust/events/309275728)
* 29/07/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester)
    * [**Lightning Talks julio de 2025**](https://www.meetup.com/rust-manchester/events/308085035)
* 29/07/2025 | Praga, CZ | [Rust República Checa](https://www.meetup.com/rust-czech-republic)
    * [**Nix Meetup en Braiins :)**](https://www.meetup.com/rust-czech-republic/events/308963318)
* 30/07/2025 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
* 31/07/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #14: Prof. Dra. Claudia Meitinger - Embajada - Möglichkeiten und Herausforderungen im Modul "Proyecto Interdisciplinario"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)
* 06/08/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
* 13/08/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/308944036)

### América del Norte
* 16/07/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307731031)
* 17/07/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/308979091)
* 17/07/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 17/07/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 1 : Introducción al Desarrollo Embebido**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 23/07/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/308791385)
* 24/07/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhckbgc)
* 24/07/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Construyendo un Runtime Asíncrono desde Cero en Rust**](https://www.meetup.com/rust-mx/events/309687971)
* 31/07/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)
* 07/08/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
* 12/08/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)

### América del Sur
* 17/07/2025 | Florianópolis, BR | [Rust Brasil + Rust Floripa](https://lu.ma/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://lu.ma/p0umq6vm)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Desafortunadamente -∞ no cabe en 'usize'.

– [Tomek Czajka sobre los usuarios de Rust](https://users.rust-lang.org/t/enumerations-how-are-they-stored-and-other-questions/131667/31)

¡Gracias a [Kyllingene](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1703) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](REDDIT_LINK_HERE)</small>
