---
title: "Esta semana en Rust #59"
number_of_week: 59
description: El crate de esta semana es structstruck, una caja proc-macro para habilitar definiciones de struct/enum anidadas.
date: 2025-05-07
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
* [Anunciando rustup 1.28.2](https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/)

### Actualizaciones de proyectos/herramientas
* [Anunciando Malai - Comparte tu servidor de desarrollo (y más) a través de P2P](https://malai.sh/announcing-malai/)
* [Análisis de datos de streaming, versión 0.17.2 de Fluvio](https://www.fluvio.io/news/this-week-in-fluvio-0074)
* [Leptos v0.8.0](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
* [Este mes en Redox - Abril 2025](https://www.redox-os.org/news/this-month-250430/)

### Observaciones/Pensamientos
* [Entrelazado automático de operaciones concurrentes de alto nivel](https://blog.yoshuawuyts.com/automatic-interleaving-of-high-level-concurrent-operations/)
* [Aplanando la curva de aprendizaje de Rust](https://corrode.dev/blog/flattening-rusts-learning-curve/)
* [La evolución del Rust](https://ranger-ross.github.io/blog/evolution-of-rust/)
* [std::mem es... Interesante](https://blog.veeso.dev/blog/en/std-mem-is-interesting/)
* [audio] [Svix con Tom Hacohen](https://corrode.dev/podcast/s04e02-svix/)

### Tutoriales de Rust
* [Autenticación con Axum](https://mattrighetti.com/2025/05/03/authentication-with-axum)
* [Los índices newtyped son pruebas](https://eikopf.bearblog.dev/newtyped-indices-are-proofs/)
* [¿Qué está haciendo mi fuzzer?](https://tweedegolf.nl/en/blog/154/what-is-my-fuzzer-doing)
* [Una API de Rust inspirada en Python, impulsada por Serde](https://ohadravid.github.io/posts/2025-05-serde-reflect/)
* [Cómo crear imágenes Docker pequeñas y seguras para Rust (DESDE cero)](https://kerkour.com/rust-docker-from-scratch)
* [video] [Rust + SQLite: Tutorial completo (esquema, CRUD, JSON y asíncrono)](https://www.youtube.com/watch?v=VlyXm7bwq6k)

### Investigación
* [Un depurador interactivo para errores de rasgos de Rust](https://cel.cs.brown.edu/blog/an-interactive-debugger-for-rust-trait-errors/)
* [RustAssistant: Uso de LLMs para corregir errores de compilación en el código Rust](https://www.microsoft.com/en-us/research/publication/rustassistant-using-llms-to-fix-compilation-errors-in-rust-code/)

### Miscelánea
* [Sudo seguro para memoria se convertirá en el predeterminado en Ubuntu](https://trifectatech.org/blog/memory-safe-sudo-to-become-the-default-in-ubuntu/)
* [Cómo conseguir un trabajo de Rust Parte I: Empresas que ya utilizan Rust](https://filtra.io/rust/career-help/how-to-get-a-rust-job-I)
* [Finalistas de GOSIM Spotlight en RustWeek](https://rustweek.org/gosim-spotlight-speakers/)

## Crate de la semana

El crate de esta semana es [structstruck](https://crates.io/crates/structstruck), una caja proc-macro para habilitar definiciones de struct/enum anidadas.

¡Gracias a [Julius Michaelis](https://users.rust-lang.org/t/crate-of-the-week/2704/1433) por la auto-sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
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
* [Hyperswitch - Mover las funciones de utilidad específicas del conector a los módulos de conector respectivos](https://github.com/juspay/hyperswitch/issues/7926)
* [Hyperswitch - Refactorizar el conector ACI para reutilizar utilidades de 'utils.rs'](https://github.com/juspay/hyperswitch/issues/7927)
* [Hyperswitch - Analice y elimine las funciones no utilizadas en 'connector/utils.rs'](https://github.com/juspay/hyperswitch/issues/7928)
* [Rama - Añadir fFi/Rama-Rhai: Capacidad de soporte para usar servicios y capas escritas en Rhai](https://github.com/plabayo/rama/issues/533)
* [rama - soporte (TLS) peetprint en huellas dactilares rama-net](https://github.com/plabayo/rama/issues/518)
* [Rama - Admite la huella digital pasiva Akamai H2 y la exposición en los servicios Echo + FP](https://github.com/plabayo/rama/issues/517)
* [rama - añadir into_stream al rasgo BodyExtractExt](https://github.com/plabayo/rama/issues/536)
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->
*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 447 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-29..2025-05-06

#### Compilador

* [handle paren in macro expand for let-init-else expr](https://github.com/rust-lang/rust/pull/134034)
* [implementar patrones o para tipos de patrones](https://github.com/rust-lang/rust/pull/139909)
* [Soporte inicial para cajas conectadas dinámicamente](https://github.com/rust-lang/rust/pull/134767)
* [mir-opt: ejecutar MatchBranchSimplification después de GVN](https://github.com/rust-lang/rust/pull/140115)
* [refactorizar el analizador de filtros de 'rustc_on_unimplemented's](https://github.com/rust-lang/rust/pull/140307)
* [perf: optimiza el codegen para 'Span::from_expansion'](https://github.com/rust-lang/rust/pull/140485)
* [perf: comprobación de retraso de '#[rustc_no_implicit_autorefs]' en autoref lint](https://github.com/rust-lang/rust/pull/140406)
* [perf: simplify 'LazyAttrTokenStream'](https://github.com/rust-lang/rust/pull/127516)
* [perf: use un cierre en lugar de tres iteradores encadenados](https://github.com/rust-lang/rust/pull/140464)
* [Transmutabilidad: fusionar tramos contiguos con un destino común](https://github.com/rust-lang/rust/pull/140509)
* [Transmutabilidad: la transición uninit coincide solo con el byte unitario](https://github.com/rust-lang/rust/pull/140380)

#### Biblioteca

* [evitar comprobaciones redundantes de WTF-8 en 'PathBuf'](https://github.com/rust-lang/rust/pull/140159)
* [delegar a inner 'vec::IntoIter' desde 'env::ArgsOs'](https://github.com/rust-lang/rust/pull/139847)
* [implementar 'Iterator::last' para 'vec::IntoIter'](https://github.com/rust-lang/rust/pull/139773)
* [estabilizar 'ptr::swap_nonoverlapping' en const](https://github.com/rust-lang/rust/pull/137280)
* [estabilizar 'select_unpredictable'](https://github.com/rust-lang/rust/pull/140550)
* [simplificar la macro 'formato'](https://github.com/rust-lang/rust/pull/140188)

#### Carga

* [cargo add: sugerir características con nombres similares](https://github.com/rust-lang/cargo/pull/15438)
* [en package-workspace, mantener dev-dependencies si tienen una versión](https://github.com/rust-lang/cargo/pull/15470)

#### Rustdoc

* [Arreglar la heurística de DocTest para el envoltura principal de FN](https://github.com/rust-lang/rust/pull/140420)

#### Rustfmt

* [también permitir literales bool como primer elemento de la cadena let](https://github.com/rust-lang/rust/pull/140486)

#### Clippy

* [No avises sobre cajas descargadas](https://github.com/rust-lang/rust-clippy/pull/14733)
* [Arreglar 'collapsible_if' falso positivo en el bloque stmt antes de expr](https://github.com/rust-lang/rust-clippy/pull/14730)
* [corregir el falso positivo 'manual_unwrap_or_default' en el enlace de referencias](https://github.com/rust-lang/rust-clippy/pull/14731)
* [corrección: 'manual_slice_fill' falso positivo en la sobrecarga de 'IndexMut'](https://github.com/rust-lang/rust-clippy/pull/14719)
* [corrección: 'unused_async' falso positivo en impl predeterminado](https://github.com/rust-lang/rust-clippy/pull/14720)
* [puerta 'collapsible_if let_chains' pelusas en la edición 2024 y MSRV](https://github.com/rust-lang/rust-clippy/pull/14723)

#### Analizador de Rust

* [agregar soporte PGO para instalar](https://github.com/rust-lang/rust-analyzer/pull/19685)
* [Mejor manejo del paralelismo en el cebado de caché](https://github.com/rust-lang/rust-analyzer/pull/19721)
* [desactivar temporalmente el punto de fijación para el cálculo de la varianza](https://github.com/rust-lang/rust-analyzer/pull/19739)
* [añadir una ayuda para desenvolver un tipo con un arg genérico](https://github.com/rust-lang/rust-analyzer/pull/19740)
* [índice de inicio de var ty ligado a la asociación correcta](https://github.com/rust-lang/rust-analyzer/pull/19732)
* [información de intervalo correcta para 'mir::Operand'](https://github.com/rust-lang/rust-analyzer/pull/19247)
* [que no cunda el pánico con algún código extraño](https://github.com/rust-lang/rust-analyzer/pull/19738)
* [arreglar las asistencias de 'move_bounds' que no funcionan de por vida](https://github.com/rust-lang/rust-analyzer/pull/19747)
* [Se corrige el manejo incorrecto de las importaciones no resueltas que no son módulos en la resolución de nombres](https://github.com/rust-lang/rust-analyzer/pull/19742)
* [arreglar la API proc-macro que crea literales negativos mal formados](https://github.com/rust-lang/rust-analyzer/pull/19746)
* [implementar mut a const ptr cast para la resolución del método](https://github.com/rust-lang/rust-analyzer/pull/19733)
* [mejorar un poco la recuperación del analizador](https://github.com/rust-lang/rust-analyzer/pull/19723)
* [Números negativos en la expansión 'Concat!](https://github.com/rust-lang/rust-analyzer/pull/19434)
* [eliminar la verificación innecesaria de la longitud del token para macros en el cambio de nombre](https://github.com/rust-lang/rust-analyzer/pull/19750)
* [mejorar el fragmento de código LET](https://github.com/rust-lang/rust-analyzer/pull/19735)
* [renderizar más vidas](https://github.com/rust-lang/rust-analyzer/pull/19581)
* [Variable de entorno de soporte 'CARGO_MANIFEST_PATH'](https://github.com/rust-lang/rust-analyzer/pull/19751)

### Clasificación del rendimiento del compilador de Rust

Una semana relativamente ruidosa debido a la incorporación de nuevos puntos de referencia como parte de nuestro [2025
benchmark update], y una serie de grandes regresiones en un rollup que aterriza tarde
en la semana (y por lo tanto aún no se ha investigado).

[Actualización de referencia 2025]: https://github.com/rust-lang/rustc-perf/issues/2024

Triaje realizado por **@simulacrum**.
Rango de revisión: [25cdf1f6.. 62c5f58f](https://perf.rust-lang.org/?start=25cdf1f67463c9365d8d83778c933ec7480e940b&end=62c5f58f57670ce65e7fec34f8c4ba00c27da2d9&absolute=false&stat=instructions%3Au)

2 regresiones, 2 mejoras, 6 mixtas; 3 de ellos en rollups
31 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-05-04.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Extensión temporal de la vida útil a través de constructores de estructura de tupla y variantes de tupla](https://github.com/rust-lang/rust/pull/140593)
* [Estabilizar las características del objetivo avx512](https://github.com/rust-lang/rust/pull/138940)
* [Hacer de 'missing_fragment_specifier' un error incondicional](https://github.com/rust-lang/rust/pull/128425)
* [Error en la opacidad recursiva en el tipo HIR](https://github.com/rust-lang/rust/pull/139419)
* [Añadir 'std::io::Seek instance' para 'std::io::Take'](https://github.com/rust-lang/rust/pull/138023)
* [eliminar intrínsecos::d rop_in_place](https://github.com/rust-lang/rust/pull/140151)
* [Estabilizar 'tcp_quickack'](https://github.com/rust-lang/rust/pull/129121)
* [Cambiar la eliminación de azúcar de 'assert!' para una mejor salida de error](https://github.com/rust-lang/rust/pull/122661)
* [Problema de seguimiento para 'non_null_from_ref]'(https://github.com/rust-lang/rust/issues/130823)
* [Hacer que los predicados de buena formación ya no sean coductivos](https://github.com/rust-lang/rust/pull/140208)
* [Se corrigió el orden de los parámetros para las variantes '_by()' de 'min' / 'max' / 'minmax' en 'std::cmp'](https://github.com/rust-lang/rust/pull/139357)
* [Finalizar el comportamiento de inferencia de repetición expr con recuentos de repeticiones inferidos](https://github.com/rust-lang/rust/pull/139635)
* [Implementar (parte de) ACP 429: agregar 'DerefMut' a 'Lazy[Cell/Lock]'](https://github.com/rust-lang/rust/pull/129334)

#### Otras áreas

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar doctest-xcompile](https://github.com/rust-lang/cargo/pull/15462)

#### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: map_or_default en opción y resultado](https://github.com/rust-lang/rfcs/pull/3148)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: enable derive(From) para estructuras de un solo campo](https://github.com/rust-lang/rfcs/pull/3809)
* [`#! [register_{attribute,lint}_tool]'](https://github.com/rust-lang/rfcs/pull/3808)
* [RFC: Agregar un atributo para elevar la alineación de varios elementos](https://github.com/rust-lang/rfcs/pull/3806)

## Próximos eventos

Eventos oxidados entre 2025-05-07 - 2025-06-04 🦀

### Virtual
* 07/05/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031663)
* 07/05/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #10**](https://www.meetup.com/bevy-game-development/events/307354690)
* 08/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 08/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/2lmcm9iq)
* 08/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307635288)
* 08/05/2025 | Virtual (Zürich, CH) | [Rust Zürisee](https://www.meetup.com/rust-zurich/events/)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 (co-evento con berline.rs) 🦀 **](https://www.meetup.com/rust-zurich/events/307696718)
* 2025-05-10 | Virtual | [Comunidad Leptos](https://lu.ma/3b7solx0)
    * [**Leptos Meetup: Versión 0.8 y demo de Server Fn Websockets**](https://www.youtube.com/watch?v=CTIeERU1hns)
* 11/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307648682)
* 11/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbpb)
* 13/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020415)
* 15/05/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 15/05/2025 | Virtual (Encuentro Conjunto, Europa + Israel) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/), [Rust Paris](https://www.meetup.com/de-DE/rust-paris/), [London Rust Project Group](https://www.meetup.com/de-DE/london-rust-project-group/), [Rust Zürisee](https://www.meetup.com/de-DE/rust-zurich/), [Rust TLV](https://www.meetup.com/de-DE/rust-tlv/), [Rust Nürnberg](https://www.meetup.com/de-DE/rust-noris/), [Rust Munich](https://www.meetup.com/de-DE/rust-munich/), [Rust Aarhus]( https://www.meetup.com/de-de/rust-aarhus/), [lunch.rs](http://lunch.rs/)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 🦀 **](https://www.meetup.com/rust-berlin/events/307293317)
* 15/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 18/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbxb)
* 19/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592)
* 2025-05-20 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Enhebrando a través de vidas de préstamos - a la manera de Rust**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    [**Rust at Work: una conversación con Ran Reichman, cofundador y CEO de Flarion**](https://www.meetup.com/code-mavens/events/307635734/)
* 2025-05-20 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170826)
* 21/05/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)
* 22/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 22/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 25/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307668643)
* 25/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbhc)
* 27/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361435)
* 27/05/2025 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversación con Eli Shalom e Igal Tabachnik de Eureka Labs**](https://www.meetup.com/code-mavens/events/307673680/)
* 29/05/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820285/)
* 01/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbcb)
* 04/06/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031665)

### Asia
* 17/05/2025 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #10**](https://www.meetup.com/rustdelhi/events/307657584)
* 24/05/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de mayo de 2025**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)

### Europa
* 07/05/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 05 2025**](https://lu.ma/k4nscy5q)
* 07/05/2025 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en mayo: FFI**](https://www.meetup.com/rustcologne/events/307548402)
* 07/05/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 07/05/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 08/05/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 08/05/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adoptando Rust (Alojado por Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 2025-05-12 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Boliche en Rust Week**](https://www.meetup.com/rust-nederland/events/307676003)
* 2025-05-12 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**¡Crea tu logotipo de Rust de acero oxidado!**](https://www.meetup.com/rust-nederland/events/307679174)
* 2025-05-12 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Recorrido a pie por Utrecht - Lunes (tarde)**](https://www.meetup.com/rust-nederland/events/307661412)
* 2025-05-12 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Recorrido a pie por Utrecht - Lunes**](https://www.meetup.com/rust-nederland/events/307521994)
* 13/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Anuncio de RustWeek 2025**](https://www.meetup.com/rust-nederland/events/305227330)
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 14/05/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 15/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust 10º aniversario @ Aparece**](https://www.meetup.com/rust-oslo/events/307427014)
* 16/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/307107584)
* 16/05/2025 | Utrecht, NL | [Grupo de Meetup de Rust NL](https://www.meetup.com/rust-nederland/)
    * [**Hackathon de RustWeek**](https://www.meetup.com/rust-nederland/events/307107584/)
* 17/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Recorrido a pie por Utrecht - Sábado**](https://www.meetup.com/rust-nederland/events/307522004)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 22/05/2025 | Augsburgo, DE | [Rust Augsburgo](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Encuentro de Rust #13**](https://rust-augsburg.github.io/meetup/Meetup_13.html)
* 22/05/2025 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 22/05/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 22/05/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/307653223)
* 27/05/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)
* 29/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 04/06/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105443)
* 04/06/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)

### América del Norte
* 07/05/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/307557852)
* 08/05/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Tiempo de compilación vs Tiempo de ejecución. Introducción a uv**](https://www.meetup.com/rust-mx/events/307015601)
* 08/05/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: Un motor de consulta analítica rápido, extensible y modular en Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 11/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Porter Square, 11 de mayo**](https://www.meetup.com/bostonrust/events/306845728)
* 13/05/2025 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Aplicación multiplataforma en Rust @ Warp.dev && Verificando Stdlib @ CMU**](https://www.meetup.com/rust-nyc/events/307675512)
* 15/05/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307488039/)
* 15/05/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 2 : Por qué tú, sí tú. ¡Debería usar Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 15/05/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 21/05/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)
* 28/05/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhchblc)
* 29/05/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)

### América del Sur
* 28/05/2025 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 31/05/2025 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Bueno, la respuesta es básicamente sí. Nuestro firmware es todo Rust. Todos los componentes de nuestra pila de autonomía son Rust. Nuestra aplicación está 50% en Rust. Y nuestras herramientas de visualización están en Rust. Nuestras herramientas de producción están oxidadas. El software de control de calidad de producción, que enviamos a China, está oxidado. Nuestros sitios web internos están oxidados. Está oxidado por todas partes. Hemos bebido el Rust Kool-Aid. De hecho, no hay Python instalado en los robots. Esto no es para desprestigiar a Python en absoluto, pero simplemente no está allí.
>
> Usamos Python para el entrenamiento de redes neuronales. Pero Python está encasillado en eso. Todo lo demás es Rust. Y la ventaja de usar Rust se acumula exponencialmente.

– [Vivek Bagaria en filtra.io](https://filtra.io/rust/interviews/matic-apr-25)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1683) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1khfde3/this_week_in_rust_598/)</small>
