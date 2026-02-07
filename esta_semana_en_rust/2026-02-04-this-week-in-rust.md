---
title: "Esta semana en Rust #96"
number_of_week: 96
description: El crate de esta semana es vortex, una biblioteca de BitTorrent basada solo en 'io_uring' para Linux y TUI.
date: 2026-02-04
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
* [Primer vistazo a los objetivos del Proyecto 2026](https://blog.rust-lang.org/inside-rust/2026/02/03/first-look-at-2026-project-goals/)

### Fundación
* [Informe Anual de la Fundación Rust 2025 + Estrategia a 3 años](https://rustfoundation.org/media/annual-report-strategy-2025/)

### Boletines
* [El Rustacean Incrustado Número #64](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-64)
* [Rust Trends Número #74: Cuando Meta y Anthropic eligen Rust](https://rust-trends.com/newsletter/when-meta-and-anthropic-choose-rust)
* [Este mes en Rust OSDev: enero 2026](https://rust-osdev.com/this-month/2026-01/)

### Actualizaciones de proyectos/herramientas
* [Compilando Rust a un Do legible con Eurídice](https://lwn.net/SubscriberLink/1055211/0c358474dee845ec/)
* [3DCF/doc2dataset v0.2.0](https://github.com/3DCF-Labs/doc2dataset/releases/tag/v0.2.0) – compresión de documentos embebible [crate](https://crates.io/crates/three-dcf-core) (codificador 3DCF + ayudantes de exportación JSONL) para convertir PDFs/markdown/HTML/etc. en bloques eficientes para tokens dentro de tus herramientas de Rust.
* [kinded v0.5.0 - proc-macro para generar enum compañero sin datos](https://github.com/greyblake/kinded/releases/tag/v0.5.0)
* [Versión CGP v0.6.1: Mejorando la ergonomía y la depuración](https://contextgeneric.dev/blog/v0-6-1-release/)
* [hotpath-rs 0.10 - nueva versión añade una interfaz MCP para la integración de LLMs](https://hotpath.rs/mcp)
* [s2-lite](https://github.com/s2-streamstore/s2?tab=readme-ov-file#s2-lite) - Una implementación de servidor de código abierto y auto-hostable de la API duradera S2 streams, respaldada por almacenamiento de objetos.

### Observaciones/Pensamientos
* [Rust para la programación en red](https://dev.to/godofgeeks/rust-for-network-programming-1en5)
* [vídeo] [Miri: Detección práctica de comportamientos indefinidos para Rust](https://www.youtube.com/watch?v=9A8ZeDIStAs)
* [audio] [Netstack.FM episodio 25 — especial FOSDEM 2026](https://netstack.fm/#episode-25)
* [audio] [Novedades en la edición Rust 2024](https://rustacean-station.org/episode/rust-2024-edition/)

### Guías de Rust
* [Cómo interconectamos C++ monohilo con Rust multihilo](https://antithesis.com/blog/2026/rust_cpp/)
* [Benchmark SIMD de Rust: std::simd vs NEON en Apple M4](https://github.com/Erio-Harrison/simd_benchmark/blob/master/BLOG.md)
* [Escribiendo XCTes para iOS en Rust](https://simlay.net/posts/2026-01-rust-xctesting/)
* ['post.explain_builders().build()'](https://hemomorphic.alexblood.net/posts/builders/)
* [Instaladores caseros y de una línea para mi CLI de Rust: Lecciones aprendidas](https://ivaniscoding.github.io/posts/rustpackaging2/)
* [serie] [La guía del programador impaciente para Bevy and Rust: Capítulo 7 - Que haya enemigos](https://aibodh.com/posts/bevy-rust-game-development-chapter-7/)

### Investigación
* [Análisis de calidad de código de traducciones de C a Rust](https://arxiv.org/abs/2602.00840)

## Crate de la semana

El crate de esta semana es [vortex](https://github.com/Nehliin/vortex), una biblioteca de BitTorrent basada solo en 'io_uring' para Linux y TUI.

¡Gracias a [Nehliin](https://users.rust-lang.org/t/crate-of-the-week/2704/1525) por la autosugerencia!

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
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [Spindalis - Factorización QR](https://github.com/lignum-vitae/spindalis/issues/56)
* [Spindalis - Añadir una función y una macro que puedan expandir polinomios](https://github.com/lignum-vitae/spindalis/issues/36)
* [Goombay-rs - Algoritmo de añadir Gotoh](https://github.com/lignum-vitae/goombay-rs/issues/8)
* [Goombay-rs - Añadir Waterman-Smith-Beyer](https://github.com/lignum-vitae/goombay-rs/issues/7)
* [Goombay-rs - Añadir funciones a LocalAlignmentModel](https://github.com/lignum-vitae/goombay-rs/issues/4)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.
* [**Conferencia Oxidar](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16
  
<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-11

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

530 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-27..2026-02-03

#### Compilador
* [pasa 'DepNode' por referencia a más lugares](https://github.com/rust-lang/rust/pull/151881)
* [modificar 'VecCache' para mejorar el rendimiento](https://github.com/rust-lang/rust/pull/138405)

#### Biblioteca
* [añadir 'Opción::get_or_try_insert_with'](https://github.com/rust-lang/rust/pull/143650)
* [añade 'shift_{izquierda, derecha}' en las porciones](https://github.com/rust-lang/rust/pull/151812)
* [constificar 'Iterador', tomar IV](https://github.com/rust-lang/rust/pull/151281)
* [constify 'fmt::from_fn'](https://github.com/rust-lang/rust/pull/150300)
* [corregir comportamientos indefinidos en 'VecDeque::splice'](https://github.com/rust-lang/rust/pull/151769)
* [implementa 'BinaryHeap::p op_if()'](https://github.com/rust-lang/rust/pull/151829)
* [implementa 'TryFrom<integer>' para bool](https://github.com/rust-lang/rust/pull/147400)
* [corte/ascii: optimizar 'eq_ignore_ascii_case' con auto-vectorización](https://github.com/rust-lang/rust/pull/147436)
* [estabilizar 'Feature(push_mut)'](https://github.com/rust-lang/rust/pull/151785)
* [estabilizar 'ptr_as_ref_unchecked'](https://github.com/rust-lang/rust/pull/151995)
* [modificar 'SlicePartialEq' para permitir el MIR-inlining de la llamada 'compare_bytes'](https://github.com/rust-lang/rust/pull/150945)

#### Carga
* ['lints': Añadir pelusa de 'redundant_homepage'](https://github.com/rust-lang/cargo/pull/16561)
* ['lints': Añadir pelusa de dependencia de espacio de trabajo sin usar](https://github.com/rust-lang/cargo/pull/16571)
* ['lints': Refinar lints de metadatos redundantes](https://github.com/rust-lang/cargo/pull/16564)
* ['guion': Estilo correcto de mensaje de ayuda](https://github.com/rust-lang/cargo/pull/16580)
* ['tiempos': Solo calcular 'y_ticks' cuando las 'unidades' no están vacías](https://github.com/rust-lang/cargo/pull/16575)
* [previene 'cargo init' en el directorio principal](https://github.com/rust-lang/cargo/pull/16566)

#### Rustdoc
* [Añadir un marcador para indicar a los usuarios que hay elementos ocultos (obsoletos) en los resultados de búsqueda](https://github.com/rust-lang/rust/pull/151559)

#### Clippy
* ['doc_paragraphs_missing_punctuation': permitir algunos párrafos sin puntos](https://github.com/rust-lang/rust-clippy/pull/16487)
* ['str_split': reducir dificultad de sugerencia](https://github.com/rust-lang/rust-clippy/pull/16418)
* [extiende 'question_mark' para cubrir 'else if'](https://github.com/rust-lang/rust-clippy/pull/16455)
* [corregir 'unwrap_used' y 'expect_used' falso negativo al usar sintaxis totalmente cualificada](https://github.com/rust-lang/rust-clippy/pull/16489)
* [corregir 'useless_attribute' falso positivo en atributos de pelusa 'exported_private_dependencies'](https://github.com/rust-lang/rust-clippy/pull/16470)
* [corregir la gramática en el documento comenta en 'conf.rs'](https://github.com/rust-lang/rust-clippy/pull/16479)
* [corregir 'allow_attributes' falso negativo en atributos con espacio en blanco](https://github.com/rust-lang/rust-clippy/pull/16497)
* ['duration_suboptimal_units': solo se cubren los constructores de 'Duración' que toman 'u64'](https://github.com/rust-lang/rust-clippy/pull/16465)

#### Analizador de Rust
* [dote: Retroceso permite que las finalizaciones del posfijo estén en condición](https://github.com/rust-lang/rust-analyzer/pull/21557)
* [dote: implementar soporte para 'feature(new_range)'](https://github.com/rust-lang/rust-analyzer/pull/21460)
* [corrección: tipo inferido completo en estática](https://github.com/rust-lang/rust-analyzer/pull/21542)
* [arreglo: no te pongas nervioso si el analizador de Rust no genera el comando descubrir](https://github.com/rust-lang/rust-analyzer/pull/21534)
* [corrección: arreglar un pánico donde un opaco estaba restringido a un tipo imposible en método autoderiff](https://github.com/rust-lang/rust-analyzer/pull/21533)
* [arreglar: arreglar que los diagnósticos se filtran cuando los diagnósticos se descontrolan](https://github.com/rust-lang/rust-analyzer/pull/21555)
* [corrección: corregir la coincidencia de macros de 'meta' y luego '=>' o '=='](https://github.com/rust-lang/rust-analyzer/pull/21527)
* [arreglar: arreglar más problemas de globos](https://github.com/rust-lang/rust-analyzer/pull/21561)
* [corrección: corregir análisis upvar de cierres anidados](https://github.com/rust-lang/rust-analyzer/pull/21564)
* [corrección: manejo 'Self::EnumVariant' y 'Self' sobre rasgos en doclinks](https://github.com/rust-lang/rust-analyzer/pull/21528)
* [implementar los nuevos bloques de try homogéneos y heterogéneos](https://github.com/rust-lang/rust-analyzer/pull/21572)
* [haz explícitos los protocolos json y postal, elimina códec genérico](https://github.com/rust-lang/rust-analyzer/pull/21548)
* [hacer segura la cancelación de llamadas bidireccionales proc-macro](https://github.com/rust-lang/rust-analyzer/pull/21410)
* [apoyo a la rama 'move_guard'](https://github.com/rust-lang/rust-analyzer/pull/21508)

### Triaje de rendimiento del compilador Rust

En general, cuenta una semana positiva para la instrucción (~1% de mejora en
compilaciones de comprobación/depuración/opt/doc). Los recuentos cíclicos y el uso de memoria siguen siendo en términos generales
Sin cambios a lo largo de la semana.

Triaje hecho por **@simulacrum**.
Rango de revisión: [ebf13cca.. a60d12cb](https://perf.rust-lang.org/?start=ebf13cca58b551b83133d4895e123f7d1e795111&end=a60d12cbccfbeaf153f3cecb90454aa696ea4b3b&absolute=false&stat=instructions%3Au)

0 regresión, 6 mejoras, 3 mixtas; 3 de ellos en rollups
En total se realizaron 33 comparaciones de artefactos

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-02-02.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [Campos peligrosos](https://github.com/rust-lang/rfcs/pull/3458)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Estabilizar guardias 'si dejamos' ( 'feature(if_let_guard)' )](https://github.com/rust-lang/rust/pull/141295)
* [Hacer que la semántica operativa de la correspondencia de patrones sea independiente de la caja y el módulo](https://github.com/rust-lang/rust/pull/150681)
* [desemplea 'Eq::assert_receiver_is_total_eq' y emite FCW en impls manuales](https://github.com/rust-lang/rust/pull/149978)
* [Estabilizar Frontmatter](https://github.com/rust-lang/rust/pull/148051)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Para 'nvptx64', elimina CPUs y ISAs antiguos - elimina el soporte para 'target-cpu' \< SM 7.0 y PTX ISA \< 7.0](https://github.com/rust-lang/compiler-team/issues/965)
* [Crear subequipo/Grupo de trabajo de enlaces](https://github.com/rust-lang/compiler-team/issues/964)
* [Extender la política de 'x.py' para 'TODO' y 'FIXME' a otros proyectos en el árbol](https://github.com/rust-lang/compiler-team/issues/963)

#### [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Resolución: preocupación sobre el "bloque recursivo"](https://github.com/rust-lang/lang-team/issues/365)

#### [Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [¿Se puede escribir un puntero obtenido lanzando '&UnsafeCell\<T\>' a '*mut T'?](https://github.com/rust-lang/unsafe-code-guidelines/issues/281)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 04-02-2026 - el 04-03-2026 🦀

### Virtual
* 2026-02-04 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 1: Conceptos Comunes de Programación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Lectura de código oxidado y contribución de código abierto (UTC 18:00; Inglés)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 2: Propiedad y Estructuras**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ir8s81ec)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254788/)
* 2026-02-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Almuerzo y aprendizaje: Patrón de Rust Coincidiendo Desempacado**](https://www.meetup.com/women-in-rust/events/312799411/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/fvcjjuv8)
* 2026-02-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455923/)
* 2026-03-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcfbgb/)

### Asia
* 2026-02-05 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Encuentro de Malasia Rust febrero 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de febrero de 2026**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**En persona Rust febrero 2026 en Nuvoton en Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europa
* 2026-02-04 | Darmstadt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo un servicio de suscripción a un boletín con axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-04 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en febrero: Acelera tu Python**](https://www.meetup.com/rustcologne/events/313111752/)
* 2026-02-04 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Más allá del código: Diseñando servicios que resisten la prueba del tiempo**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Hack Rust & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Hack y Aprendizaje de Karlsruhe Meetup en BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-18 - 2026-02-19 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Charla de febrero de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313172595/)
* 2026-03-04 | Hamburgo, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn marzo 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Discos, Destrozados sobre Hielo: Una introducción al parquet y el iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)

### Norteamérica
* 2026-02-05 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora Feliz Oxidada**](https://www.meetup.com/chicago-rust-meetup/events/313163092/)
* 2026-02-05 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust 101: ¿Qué es Rust y cómo puedo usarlo?**](https://www.meetup.com/music-city-rust-developers/events/313133786/)
* 2026-02-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Social mensual de febrero**](https://www.meetup.com/rust-montreal/events/313068358/)
* 2026-02-05 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en el DOJO HACKER**](https://www.meetup.com/hackerdojo/events/312859472/)
* 2026-02-05 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Renderizando el set de Mandelbrot en Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-11 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Rust ATX en Cloudflare**](https://www.meetup.com/rust-atx/events/313147803/)
* 2026-02-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Desarrollo web Full Stack en Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro y Saludo Comunitario**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust como capa de pegamento- Infraestructura para aplicaciones nativas de IA**](https://www.meetup.com/rust-los-angeles/events/313097225/)
* 2026-02-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228648/)

### Oceanía
* 2026-02-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane febrero 2026**](https://www.meetup.com/rust-brisbane/events/313087789/)
* 2026-02-11 | Sídney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
    * [**Bienvenidos 🦀 a 2026**](https://www.meetup.com/rust-sydney/events/313074935/)
* 2026-02-24 | Canberra, AU | [Canberra Oxidado](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/313199994/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> En C++, la memoria muscular que desarrollas con el tiempo es **evitativa**. Aprendes a no hacer ciertas cosas. Es un recuerdo negativo, no en un sentido peyorativo, sino en el sentido de que tienes que recordar qué no debes hacer en lugar de qué hacer: una lista de patrones que evitar, trampas que esquivar. Y esta lista sigue creciendo, porque el lenguaje no te impide caer en las trampas, solo tienes que recordar que existen.
>
> En Rust, la memoria muscular es **constructiva**. Aprendes patrones que son inherentemente correctos. No tienes que recordar qué evitar porque el compilador no te lo permite. En lugar de pensar "Debo recordar no dejar la puerta abierta", aprendes a construir una puerta que se cierra sola.

– [Marco Bollero en dev.to](https://dev.to/marco_bollero_ba3c38ddd27/switching-from-cc-to-rust-the-invisible-checklist-2p9n)

Dada la aguda falta de sugerencias, llogiq está bastante agradecido consigo mismo por haber encontrado una cita de todos modos.

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1qwh3hw/this_week_in_rust_637/)</small>
