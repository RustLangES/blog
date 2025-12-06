---
title: "Esta semana en Rust #87"
number_of_week: 87
description: El crate de esta semana es corosensei, una caja que te permite escribir corutinas apiladas en Rust estable.
date: 2025-12-03
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
* [Actualización del Director de Proyecto Otoño 2025](https://blog.rust-lang.org/inside-rust/2025/12/02/project-director-update/)

### Fundación
* [Rustls finalista de dos premios OpenUK 2025](https://rustfoundation.org/media/rustls-shortlisted-for-two-2025-openuk-awards/)

### Boletines
* [Rust Trends Número #73: Del laboratorio a la fábrica: Rust en producción](https://rust-trends.com/newsletter/from-lab-to-factory-rust-in-production/)

### Actualizaciones de proyectos/herramientas
* [Wasmi 1.0 — Por fin estable el intérprete de Webassembly](https://wasmi-labs.github.io/blog/posts/wasmi-v1.0/)
* [Pools Componibles hiper-útiles](https://seanmonstar.com/blog/hyper-util-composable-pools/)
* [Actualizaciones de otoño: Soporte estándar de bibliotecas con vexide 0.8.0!](https://vexide.dev/blog/posts/thanksgiving-update-25/)
* [3DCF/doc2dataset v0.1.0 – Pipeline documento-conjunto de datos de Rust para ajuste fino de RAG y LLM](https://github.com/3DCF-Labs/doc2dataset/releases/tag/v0.1.0)
* [PGM-Extra: Estructuras de índice aprendido de alto rendimiento para Rust](https://github.com/itsfoxstudio/pgm-extra-rs/releases/tag/v1.2.2)

### Observaciones/Pensamientos
* [En defensa del envenenamiento por cerradura en Rust](https://sunshowers.io/posts/on-poisoning/)
* [Cómo los CRDT y Rust están revolucionando los sistemas distribuidos y las aplicaciones en tiempo real](https://kerkour.com/rust-crdt)
* [KCL parte 1: unidades](https://www.ncameron.org/blog/kcl-part-1-units/)
* [Nuevo pelusa de Rust: function_casts_as_integer](https://blog.guillaume-gomez.fr/articles/2025-11-28+New+rust+lint%3A+function_casts_as_integer)
* [audio] [Netstack.FM episodio 16 — WebRTC y Sans IO con Martin Algesten](https://netstack.fm/#episode-16)
* [audio] [Canónico con Jon Seager - Rust en Producción](https://corrode.dev/podcast/s05e05-canonical/)

### Guías de Rust
* [La Guía del Programador Impaciente para Bevy y Rust: Capítulo 3 - Deja que fluyan los datos](https://aibodh.com/posts/bevy-rust-game-development-chapter-3/)
* [Compilación cruzada de Rust para Raspberry Pi y creación de CI](https://sysdev.me/2025/11/27/cross-compiling-rust-for-raspberry-pi/)
* [Rootless pings en Rust](https://bou.ke/blog/rust-ping/)
* [Pruebas de mutación para librsvg](https://viruta.org/mutation-testing-librsvg.html)
* [vídeo] [impl Rust: Desafío de Mil Millones de Filas](https://www.youtube.com/watch?v=tCY7p6dVAGE)

### Miscelánea
* [El Hackathon de Rust Africa 2026](https://rustafrica.org/the-future-is-written-in-rust-rust-africa-hackathon-2026/)
* [Ferrous Systems obtiene la certificación IEC 61508 (SIL 2) para subconjunto de Rust Core Library](https://ferrous-systems.com/blog/ferrocene-libcore-news-release/)

## Crate de la semana

El crate de esta semana es [corosensei](https://github.com/Amanieu/corosensei), una caja que te permite escribir corutinas apiladas en Rust estable.

¡Gracias a [Christiaan](https://users.rust-lang.org/t/crate-of-the-week/2704/1497) por la sugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

* *No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos
  
Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [**hiper - Encuesta de usuarios 2025**](https://www.surveyhero.com/c/vvnhc7j7)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/) | CFP cierra el 24-11-2025 | Varsovia, Polonia | 2025-03-19 - 2025-03-2025 | [Página web del evento](https://www.rustikon.dev/)
* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp) | CFP cierra el 08-12-2025 | Portland, Oregón, EE. UU. | 2026-04-20
* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 31-12-2025 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

509 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-25..2025-12-02

#### Compilador
* [añadir 'Box::clone_from_ref' y similares bajo 'feature(clone_from_ref)'](https://github.com/rust-lang/rust/pull/149079)
* [añadir 'Command::get_env_clear'](https://github.com/rust-lang/rust/pull/149074)
* [añadir un atributo diagnóstico para errores limitados a const de mayúsculas especiales para impls no const](https://github.com/rust-lang/rust/pull/148641)
* [colapsar 'constness' consulta lógica de 'match'](https://github.com/rust-lang/rust/pull/149444)

#### Biblioteca
* [añadir 'impl TrustedLen' en iteradores 'BTree{Map,Set}'](https://github.com/rust-lang/rust/pull/149381)
* [constificar 'from_fn, try_from_fn, try_map,', mapa](https://github.com/rust-lang/rust/pull/147071)
* [implementa 'Iterator::{exactly_one, collect_array}'](https://github.com/rust-lang/rust/pull/149270)
* [implementar el método 'clamp_magnitude' para flotadores primitivos y enteros con signo](https://github.com/rust-lang/rust/pull/148690)
* [en 'BTreeMap::eq', no compares los elementos si los tamaños son diferentes](https://github.com/rust-lang/rust/pull/149125)
* [num: implementar la función 'uint_gather_scatter_bits' para enteros sin signo](https://github.com/rust-lang/rust/pull/149097)
* [descarga intrínseca](https://github.com/rust-lang/rust/pull/147936)
* [optimizar 'slice::Iter::next_chunk'](https://github.com/rust-lang/rust/pull/149131)
* [estabilizar 'asm_cfg'](https://github.com/rust-lang/rust/pull/147736)
* [estabilizar 'maybe_uninit_slice'](https://github.com/rust-lang/rust/pull/149102)
* [estabilizar 'maybe_uninit_write_slice'](https://github.com/rust-lang/rust/pull/148048)
* [estabilizar 'unchecked_neg' y 'unchecked_shifts'](https://github.com/rust-lang/rust/pull/149087)

#### Carga
* ['limpia': Anfitrionas limpias construyen con nuevo diseño](https://github.com/rust-lang/cargo/pull/16300)
* ['completación': Pon host-tuple antes de las tuplas reales](https://github.com/rust-lang/cargo/pull/16327)
* ['completaciones': incluir 'todos' en 'candidatos a árbol de carga --objetivo'](https://github.com/rust-lang/cargo/pull/16322)
* ['config-include': eliminar soporte para taquigrafía de una sola cadena](https://github.com/rust-lang/cargo/pull/16298)
* ['lints': mostrar número de error de pelusa](https://github.com/rust-lang/cargo/pull/16320)
* ['limpiar': añadir --soporte para espacio de trabajo](https://github.com/rust-lang/cargo/pull/16263)
* [no bloquear el director de artefactos para builds de comprobación + mejora de corrección](https://github.com/rust-lang/cargo/pull/16307)
* [validar correctamente los nombres de las cajas en 'instalación de carga'](https://github.com/rust-lang/cargo/pull/16314)

#### Rustdoc
* [corregir un mal preprocesamiento intra-doc-link](https://github.com/rust-lang/rust/pull/148169)
* [corregir la generación de enlaces inválida para métodos de alias de tipo](https://github.com/rust-lang/rust/pull/149274)
* [la búsqueda de fix rustdoc dice "Considera buscar "null" en su lugar." #149324](https://github.com/rust-lang/rust/pull/149332)

#### Clippy
* ['manual_ilog2': pelusa nueva](https://github.com/rust-lang/rust-clippy/pull/15865)
* ['equatable_if_let': no hacer pelusa si el patrón o el inicializador provienen de la expansión](https://github.com/rust-lang/rust-clippy/pull/15958)
* [añadir pelusa de 'ptr_offset_by_literal'](https://github.com/rust-lang/rust-clippy/pull/15606)
* [Mejoras y limpiezas de página de Clippy Lints](https://github.com/rust-lang/rust-clippy/pull/16112)
* [corregir macros 'implicit_hasher' mal desconfiguradas](https://github.com/rust-lang/rust-clippy/pull/16129)
* [corregir 'large_stack_frames' falsos positivos en objetivos generados por compilador](https://github.com/rust-lang/rust-clippy/pull/15101)
* [corregir la visualización de los "botones" del menú desplegable](https://github.com/rust-lang/rust-clippy/pull/16151)
* [arreglar: 'zero_repeat_side_effects' falla en rizos](https://github.com/rust-lang/rust-clippy/pull/15853)
* [nueva pelusa: 'decimal_bitwise_operands'](https://github.com/rust-lang/rust-clippy/pull/15215)
* [deja de insertar paréntesis redundantes alrededor de las expresiones de coincidencia desazúcaradas](https://github.com/rust-lang/rust-clippy/pull/16102)

#### Analizador de Rust
* [añadir múltiples genera para 'enum' generar es, como, 'try_into'](https://github.com/rust-lang/rust-analyzer/pull/20685)
* [compilación se libera con CRT estático para objetivos '-windows-msvc'](https://github.com/rust-lang/rust-analyzer/pull/21027)
* [completaciones: corregir completaciones sin tener en cuenta las capacidades de fragmentos](https://github.com/rust-lang/rust-analyzer/pull/21131)
* [característica: establecer campo 'enclosing_range' en la salida SCIP](https://github.com/rust-lang/rust-analyzer/pull/21141)
* [corregir Display scope inlay indica indicaciones tras cerrar el refuerzo para más tipos de bloques #18833](https://github.com/rust-lang/rust-analyzer/pull/21077)
* [corregir 'syntax_editor' duplicado elemento cambiado](https://github.com/rust-lang/rust-analyzer/pull/21023)
* [arreglo completado tras 'extern', añadir 'caja' completado](https://github.com/rust-lang/rust-analyzer/pull/21144)
* [corrección no completa después de inner-attr en el archivo fuente](https://github.com/rust-lang/rust-analyzer/pull/20976)
* [corrección no completa de tipo alias en patrón](https://github.com/rust-lang/rust-analyzer/pull/21028)
* [fijar skipiter no aplicable en autoderef](https://github.com/rust-lang/rust-analyzer/pull/21095)
* [no intentes conectarte mediante postal a proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21133)
* [no ejecutar la primación de caché cuando está desactivada en la configuración](https://github.com/rust-lang/rust-analyzer/pull/21151)
* [corregir proc-macro-srv pasando un extra no inválido al grupo de non-ningún a proc-macros](https://github.com/rust-lang/rust-analyzer/pull/21190)
* [Corregir la implementación del protocolo Rec-Macro-SRV Lectura](https://github.com/rust-lang/rust-analyzer/pull/21135)
* [pasa la edición correcta por token (no global) al expandir 'macro_rules'](https://github.com/rust-lang/rust-analyzer/pull/20164)
* [reescritura del rasgo dyn bajando para seguir a rustc](https://github.com/rust-lang/rust-analyzer/pull/21159)
* [soporte múltiples 'enable' en '#[target_feature]'](https://github.com/rust-lang/rust-analyzer/pull/21170)
* [usar edición por token, no global, en el analizador](https://github.com/rust-lang/rust-analyzer/pull/20163)
* [usar higiene radicular para resolución especulativa](https://github.com/rust-lang/rust-analyzer/pull/20217)
* [perf: usa una consulta por caja para los objetos lang, no una por objeto lang](https://github.com/rust-lang/rust-analyzer/pull/21149)
* [proc-macro-srv: fix '<TokenStream como Display>::fmt' impl produciendo espacio en blanco de retaguardia](https://github.com/rust-lang/rust-analyzer/pull/21145)
* [proc-macro-srv: fix '<TokenStream como Display>::fmt' impl rendering puncts as u8](https://github.com/rust-lang/rust-analyzer/pull/21146)
* [proc-macro-srv: corregir el envolvimiento innecesario de subárboles en el protocolo](https://github.com/rust-lang/rust-analyzer/pull/21154)
* [reintroducir reescritura de atributos](https://github.com/rust-lang/rust-analyzer/pull/20892)

### Triaje de rendimiento del compilador Rust

Una semana bastante tranquila en general, a pesar de un número ligeramente superior de marcas personales fusionadas.

Triaje hecho por **@simulacrum**.
Rango de revisión: [b64df9d1.. eca9d93f](https://perf.rust-lang.org/?start=b64df9d1012f2482b54a4d959548cf8fc67e820c&end=eca9d93f9057f9a48ff691bd65e7daf2f94c1b67&absolute=false&stat=instructions%3Au)

3 regresiones, 1 mejora, 4 mixtas; 3 de ellos en rollups
43 comparaciones de artefactos realizadas en total

Consulte el [informe completo](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-12-02.md) para más detalles.

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No hubo RFC entrando en el Periodo Final de Comentarios esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [no normalizan las cláusulas de donde al comprobar la buena formación](https://github.com/rust-lang/rust/pull/148477)
* [Estabilizar 'const_mul_add'](https://github.com/rust-lang/rust/pull/148052)
* [No propuéis restricciones de cierre innecesarias.](https://github.com/rust-lang/rust/pull/148329)
* [No forrando 'irrefutable_let_patterns' en las cadenas de alquiler](https://github.com/rust-lang/rust/pull/146832)
* [Haz que la captura de cierre tenga un comportamiento consistente y correcto alrededor de los patrones](https://github.com/rust-lang/rust/pull/138961)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Usa 'annotate-snippets' como emisor por defecto](https://github.com/rust-lang/compiler-team/issues/947)
* [Promocionar powerpc64-unknown-linux-musl a nivel 2 con herramientas anfitriona](https://github.com/rust-lang/compiler-team/issues/946)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Añadiendo una pestaña de seguridad crates.io](https://github.com/rust-lang/rfcs/pull/3872)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [dote: estabilizar '-Zconfig-include'](https://github.com/rust-lang/cargo/pull/16284)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Números complejos](https://github.com/rust-lang/rfcs/pull/3892)
* [RFC: Campos propios de const](https://github.com/rust-lang/rfcs/pull/3888)
* [Create 0000-pub_use_pub_glob.md](https://github.com/rust-lang/rfcs/pull/3887)

## Próximos eventos

Eventos Rusty entre el 3 de diciembre de 2025 y el 31 🦀 de diciembre de 2025

### Virtual
* 03-12-2025 | Virtual (Buffalo, NY, EE. UU.) [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 03-12-2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311886445/)
* 04-12-2025 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/306046643/)
* 05-12-2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [¡Inicio del Juego de Navidad de Rust & C++!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103307/)
* 2025-12-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763878687)
* 2025-12-07 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Final del Rust & C++ Christmas Game Jam**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103329/)
* 2025-12-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305361537/)
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/li5de4ts)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310728572/)
* 2025-12-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/6v2rorp3)
* 2025-12-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361448/)
* 2025-12-25 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046673/)

### Asia
* 2025-12-08 | Tokio, JP | [Rust Global: Tokio](https://rustfoundation.org/event/rust-global-tokyo/)
    * [**Rust Global: Tokyo**](https://rustfoundation.org/event/rust-global-tokyo/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de diciembre 2025**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)

### Europa
* 03-12-2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 03-12-2025 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Encuentro Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 04-12-2025 | Viena, AT | [Viena Oxidada](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna S2E2 - Diciembre | en metalab 🦀 **](https://www.meetup.com/rust-vienna/events/311680386/)
* 2025-12-06 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #20**](https://www.meetup.com/stockholm-rust/events/312195848/)
* 2025-12-08 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - diciembre de 2025**](https://www.meetup.com/rust-dortmund/events/312165912/)
* 2025-12-08 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**Rust LDN Habla: Fiesta de Navidad con los London Gophers y Red Badger**](https://www.meetup.com/rust-london-user-group/events/312264843/)
* 2025-12-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-15 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim)
    * [**Oxidación del Hackathon**](https://www.meetup.com/rust-trondheim/events/312278650/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)
* 2025-12-19 | Lyon, FR | [Lyon Oxidado](https://www.meetup.com/rust-lyon)
    * [**Reunión de Rust Lyon #11**](https://www.meetup.com/rust-lyon/events/312180836/)

### Norteamérica
* 04-12-2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 04-12-2025 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Dominando el estado, la seguridad y los manejadores escalables en Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 05-12-2025 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: ¡Nuestro mayor evento hasta la fecha!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Rust en el centro, 6 de diciembre**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-10 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/312289655/)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Robótica Competitiva con Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 13 de diciembre**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312076080/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Spokane, WA, EE. UU. [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro social de fin de año con grupos de usuarios locales de Python, Rust y otros**](https://www.meetup.com/spokane-rust/events/312292668/)
* 2025-12-20 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 20 de diciembre**](https://www.meetup.com/bostonrust/events/311917280/)

### Oceanía
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup dic 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> \[...\] simplemente devolver un error no es *gestión de errores*, es simplemente desenrollar el espacio de usuario.

– [Ddystopia en Rust-internals](https://internals.rust-lang.org/t/re-opening-deprecating-option-unwrap-and-result-unwrap/23734/45)

¡Gracias a [Aleksander Krauze](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1734) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1pdqeup/this_week_in_rust_628/)</small>
