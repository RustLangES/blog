---
title: "Esta semana en Rust #98"
number_of_week: 98
description: El crate de esta semana es banish, una macro de activación para construir máquinas de estados basadas en reglas usando un DSL declarativo.
date: 2026-02-18
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
* [Anunciando Rust 1.93.1](https://blog.rust-lang.org/2026/02/12/Rust-1.93.1/)
* [crates.io: una actualización de la política de notificaciones maliciosas de cajas](https://blog.rust-lang.org/2026/02/13/crates.io-malicious-crate-update/)
* [Este ciclo de desarrollo en carga: 1,94](https://blog.rust-lang.org/inside-rust/2026/02/18/this-development-cycle-in-cargo-1.94/)

### Boletines
* [Computación Científica en Rust #15 (febrero de 2026)](https://scientificcomputing.rs/monthly/2026-02)
* [El Rustacean Incrustado Número #65](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-65)

### Actualizaciones de proyectos/herramientas
* [stochastic-rs: simulaciones estocásticas/cuantitativas (y más)](https://rust-dd.com/post/stochastic-rs-v1-stable)
* [Banish v1.1.4: DSL de máquina de estados basada en reglas](https://github.com/LoganFlaherty/banish/releases/tag/v1.1.4)
* [Surgiendo de Superficies de Volatilidad en Rust](https://volsurf-rs.github.io/posts/building-vol-surfaces-in-rust/)
* [Diesel-guard v0.6.0: comprobaciones personalizadas para migraciones de Postgres](https://github.com/ayarotsky/diesel-guard/releases/tag/v0.6.0)
* [El hipervisor Selium WebAssembly está en Alfa](https://selium.com/news/alpha-release)
* [FerroTunnel: túnel de reversa de alto rendimiento (https://users.rust-lang.org/t/ferrotunnel-high-performance-embeddable-reverse-tunnel-for-rust-applications/138214)
* [Compendio: strace como trazador](https://pker.xyz/posts/compendium)
* [Sesiones de shell contenedores con Shell-Cell](https://mr-leshiy-blog.web.app/blog/shell-cell/)
* [Presentando SurrealDB 3.0 - memoria de agentes de IA](https://surrealdb.com/blog/introducing-surrealdb-3-0--the-future-of-ai-agent-memory)
* [sighook 0.9.0: APIs de gancho preparcheadas](https://github.com/YinMo19/sighook/releases/tag/v0.9.0)

### Observaciones/Pensamientos
* [Cómo Rust y su compilador han revolucionado la ingeniería de software y la fiabilidad](https://kerkour.com/rust-software-engineering-reliability)
* [Async/await en la GPU](https://www.vectorware.com/blog/async-await-on-gpu/)
* [La evolución de Async Rust: de Tokio a aplicaciones de alto nivel](https://www.youtube.com/live/2aZaBZVJWm0?si=ienX-zcIBOtDxhj0)

### Guías de Rust
* [Introducción a la redacción de contratos RISC-V en Rust on Polkadot](https://dev.to/badery/introduction-to-writing-risc-v-contracts-in-rust-on-polkadot-29n7)
* [Enviando mi CLI de Rust a Windows: Lecciones aprendidas (con Windows 98 y APE Bonus)](https://ivaniscoding.github.io/posts/rustpackaging4/)
* [Visualizando vectores persistentes con Rust y WebAssembly](https://abishov.com/blog/pvec-rs-visualizing-structural-sharing/)
* [Recreando la pg_strict de PlanetScale en Rust: A Build Log](https://saybackend.com/blog/recreating-planetscale-pg-strict-in-rust/)
* [serie] [Parte 5: Un tonto sin sentido, construyendo un LLM desde cero en Rust](https://www.tag1.com/how-to/part5-a-witless-fool-building-an-llm-from-scratch/)

### Miscelánea
* [Informe de empleos de Rust de enero de 2026](https://filtra.io/rust/jobs-report/jan-26)
* [Encuesta sobre el ecosistema de desarrolladores de Rust 2025: Popularidad, tendencias y futuro](https://blog.jetbrains.com/rust/2026/02/11/state-of-rust-2025/)

## Crate de la semana

El crate de esta semana es [banish](https://github.com/LoganFlaherty/banish), una macro de activación para construir máquinas de estados basadas en reglas usando un DSL declarativo.

¡Gracias a [Logan Flaherty](https://users.rust-lang.org/t/crate-of-the-week/2704/1547) por la autosugerencia!

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

*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Conferencia Rust India 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP abierto hasta el 14-03-2026 | Bangalore, IN | 2026-04-18
* [**Conferencia Oxid**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

564 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-10..2026-02-17

#### Compilador
* [manejar la raza al colorear nodos simultáneamente como verde y rojo](https://github.com/rust-lang/rust/pull/151509)
* [implementar RFC 3678: Métodos de rasgos finales](https://github.com/rust-lang/rust/pull/151783)
* [sustituye 'box_new' por intrínsecos de menor nivel](https://github.com/rust-lang/rust/pull/148190)
* [resolución superficial de las variaciones ty y const a sus varas raíz](https://github.com/rust-lang/rust/pull/151380)
* [mostrar qué pelusa fue anulada](https://github.com/rust-lang/rust/pull/152452)

#### Biblioteca
* [implementar la característica 'float_exact_integer_constants'](https://github.com/rust-lang/rust/pull/152512)
* [implementa 'BinaryHeap::from_raw_vec'](https://github.com/rust-lang/rust/pull/152502)
* [implementa 'carryless_mul'](https://github.com/rust-lang/rust/pull/152132)
* [ADT de soporte escribe en la información de tipo reflection](https://github.com/rust-lang/rust/pull/151142)
* [optimizar la indexación de cortes y fibras con rangos inclusivos](https://github.com/rust-lang/rust/pull/145024)
* [estabilizar 'assert_matches'](https://github.com/rust-lang/rust/pull/137487)

#### Carga
* ['lints': No ejecutes lints activados por defecto cuando MSRV es demasiado antiguo](https://github.com/rust-lang/cargo/pull/16618)
* ['lockfile-path': Respeta la configuración en arreglar, instalar](https://github.com/rust-lang/cargo/pull/16617)
* ['script': Cargar la configuración relativa al script](https://github.com/rust-lang/cargo/pull/16620)
* ['script': Haz que el script de lockfile sea independiente del build-dir](https://github.com/rust-lang/cargo/pull/16619)
* [cambió el script de compilación ejecutando el director 'output' a 'stdout' en el nuevo diseño del directorio de compilación](https://github.com/rust-lang/cargo/pull/16644)
* [sugiere una entrada 'workspace.members' incluso desde fuera de la raíz del workspace](https://github.com/rust-lang/cargo/pull/16616)

#### Rustdoc
* [ordenar primero los elementos estables](https://github.com/rust-lang/rust/pull/149460)

#### Clippy
* [supone que cualquier función externa podría devolver un alias de tipo](https://github.com/rust-lang/rust-clippy/pull/16415)
* [no pelusas función principal en 'must_use_candidates'](https://github.com/rust-lang/rust-clippy/pull/16552)
* [extiende 'iter_kv_map' para cubrir 'flat_map' y 'filter_map'](https://github.com/rust-lang/rust-clippy/pull/16519)
* [corregir 'RustcCallbacks::config()' en 'clippy-driver'](https://github.com/rust-lang/rust-clippy/pull/16562)

#### Analizador de Rust
* [mejora y pasa el cursor lista de parámetros demasiado larga](https://github.com/rust-lang/rust-analyzer/pull/21591)
* [error de compilación de 'smol_str'](https://github.com/rust-lang/rust-analyzer/pull/21648)
* [fijar punto y coma completo en la expresión del array](https://github.com/rust-lang/rust-analyzer/pull/21402)
* [corregir la expansión incorrecta del camino del yo para 'inline_call'](https://github.com/rust-lang/rust-analyzer/pull/21381)
* [no resolver las macros de activación en valor ns (como funciones), solo en macro ns, fuera de su caja definitoria](https://github.com/rust-lang/rust-analyzer/pull/21633)
* [no asumas que los parámetros de 'extern fn son patrones](https://github.com/rust-lang/rust-analyzer/pull/21632)
* [nombre de encuadernaciones 'ref mut' en 'contains_explicit_ref_binding'](https://github.com/rust-lang/rust-analyzer/pull/21647)
* [usar 'ExprIsRead::Yes' para la derecha de las asignaciones ordinarias](https://github.com/rust-lang/rust-analyzer/pull/21649)
* [migrar 'covert_tuple_return_type' a 'struct' asistir al editor de sintaxis](https://github.com/rust-lang/rust-analyzer/pull/21619)
* [migrar 'generate_impl' asistir para usar AstNodeEdit](https://github.com/rust-lang/rust-analyzer/pull/21643)
* [migrar 'introduce_named_lifetime' asistencia a SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21507)
* [migrar la desestructuración de la asistencia de enlace de tuplas a syntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21618)
* [eliminar la edición mutable con 'edit::AstNodeEdit' en los gestores de asistencia migrados](https://github.com/rust-lang/rust-analyzer/pull/21636)

### Triaje de rendimiento del compilador Rust

Varias pull requests introdujeron regresiones (normalmente muy pequeñas) en todos los aspectos esta semana. En el
Por otro lado, [#151380](https://github.com/rust-lang/rust/pull/151380) proporcionó una buena victoria en el motor de inferencia.
También me gustaría llamar la atención sobre [#152375](https://github.com/rust-lang/rust/pull/152375),
lo que mejoró el frontend paralelo. No se muestra en este informe, porque aún no lo tenemos
muchos benchmarks para el frontend paralelo, pero esta imagen pública aparentemente mejoró el 'check' (tiempo de pared)
¡Rendimiento con múltiples hilos frontales en varias cajas reales del 5-10%!

Triaje hecho por **@kobzol**.
Rango de revisión: [39219ceb.. 3c9faa0d](https://perf.rust-lang.org/?start=39219ceb97d1b37dda72517daa9ebe8364ffe186&end=3c9faa0d037b9eecda4a440cc482ff7f960fb8a5&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,7% | [0,2%, 3,1%] | 96 |
| Regresiones ❌ <br /> (secundario) | 1,1% | [0,0%, 5,7%] | 62 |
| Mejoras ✅ <br /> (primaria) | -0,4% | [-0,9%, -0,2%] | 8 |
| Mejoras ✅ <br /> (secundario) | -2,6% | [-7,0%, -0,0%] | 45 |
| Todos ❌✅ (primario) | 0,6% | [-0,9%, 3,1%] | 104 |

2 regresiones, 0 mejoras, 9 mixtas; 4 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/a1128be07cc42d2bed7a65068f82dce36964386a/triage/2026/2026-02-17.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Inhibir la optimización de todas las variantes ausentes para todos los reprs de enum que inhiben la optimización de layout, no solo para repr(C).](https://github.com/rust-lang/rust/pull/146989)
* [estabilizar 'cfg_select!'](https://github.com/rust-lang/rust/pull/149783)
* [ptr::replace: hacer llamadas en ZST null ptr no UB](https://github.com/rust-lang/rust/pull/149169)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Añadir una bandera de línea de comandos '--min-recursion-limit'](https://github.com/rust-lang/compiler-team/issues/969)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Participación en la divulgación (dedicación de fondos)](https://github.com/rust-lang/leadership-council/issues/264)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [Equipo de subvenciones y programa de subvenciones 2026](https://github.com/rust-lang/rfcs/pull/3919)
* [RFC: Extender las dependencias del manifiesto con 'usado'](https://github.com/rust-lang/rfcs/pull/3920)

## Próximos eventos

Eventos Rusty entre el 18-02-2026 - el 18-03-2026 🦀

### Virtual
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
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313303094/)
* 05-03-2026 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Presentación: Tock OS Parte #3 - Cápsulas y controladores de hardware de nivel inferior**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 05-03-2026 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763908777)
* 2026-03-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (Londres, Reino Unido)| [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/312799450/)
* 2026-03-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjcfbwb/)
* 2026-03-18 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/dwnbwtyjcfbxb/)

### Asia
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de febrero de 2026**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**En persona Rust febrero 2026 en Nuvoton en Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europa
* 2026-02-18 - 2026-02-19 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-19 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en el DOJO HACKER**](https://www.meetup.com/hackerdojo/events/313139277/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Charla de febrero de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313172595/) | [**Página del Evento**](https://rustmanchester.co.uk/events/february-talks-2026/)
* 2026-02-25 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #65 Patrocinado por Factbird**](https://www.meetup.com/copenhagen-rust-community/events/313341944/)
* 2026-02-26 | Praga, CZ | [Rust República Checa](https://www.meetup.com/rust-czech-republic/events/)
    * [**Informační teorie vs. filtry: Proč filtrování bitcoinového mempoolu NEFUNGUJE**](https://www.meetup.com/rust-czech-republic/events/313323947/)
* 28-02-2026 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Fika Forum #24 de Ferris - edición crablings**](https://www.meetup.com/stockholm-rust/events/313367881/)
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**Rust en MWC Talent Arena — Talleres + Encuentro Comunitario**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburgo, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn marzo 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Discos, Destrozados sobre Hielo: Una introducción al parquet y el iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)
* 2026-03-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup - Introducción a Embedded Rust - Marzo**](https://www.meetup.com/rust-dortmund/events/313338784/)

### Norteamérica
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust 101: ¿Qué es Rust y cómo puedo usarlo?**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Union Square Somerville, 21 de febrero**](https://www.meetup.com/bostonrust/events/313208518/)
* 2026-02-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust como capa de pegamento- Infraestructura para aplicaciones nativas de IA**](https://www.meetup.com/rust-los-angeles/events/313097225/)
* 2026-02-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228648/)
* 2026-02-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Soluciones en tiempo de compilación**](https://www.meetup.com/rust-nyc/events/313196004/)
* 28-02-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de la Óxida de la Universidad de Boston, 28 de febrero**](https://www.meetup.com/bostonrust/events/313208529/)
* 05-03-2026 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**por determinar**](https://www.meetup.com/stl-rust/events/312654992/)
* 2026-03-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, 7 de marzo**](https://www.meetup.com/bostonrust/events/313208584/)
* 2026-03-14 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo North End Rust, 14 de marzo**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcfbwb/)

### Oceanía
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

> Claramente existe algo llamado demasiado azúcar sintáctico (como dijo uno de mis profesores, "el azúcar sintáctico causa cáncer semántico"), pero al mismo tiempo también está claro que algo de azúcar sintáctico merece la pena.

– [Ralf Jung sobre los internos de Rust](https://internals.rust-lang.org/t/pre-pre-rfc-splatting-for-named-arguments-and-function-overloading/24012/17)

¡Gracias a [robofinch](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1753) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1r8n7fm/this_week_in_rust_639/)</small>
