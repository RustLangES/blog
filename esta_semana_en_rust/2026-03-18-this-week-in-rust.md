---
title: "Esta semana en Rust #102"
number_of_week: 102
description: El crate de esta semana es grab, una herramienta de línea de comandos para convertir rápidamente CSV a JSON.
date: 2026-03-18
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

* [Anunciando Rustup 1.29.0](https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/)
* [Llamada de prueba: Diseño del Director de Construcción v2](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)

### Boletines

* [El Rustacean Incrustado Número #67](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-67)
* [Este mes en Rust OSDev: febrero de 2026](https://rust-osdev.com/this-month/2026-02/)

### Actualizaciones de proyectos/herramientas

* [loadgen-rs - cliente de referencia HTTP compatible con h2load escrito en Rust, compatible con HTTP/1.1, HTTP/2 y HTTP/3 (QUIC)](https://blog.none.at/blog/2026/2026-03-01-loadgen-rs/)
* [Presentando pgtui, un cliente de TUI de Postgres](https://kdwarn.net/programming/blog/227)
* [Física Aviar 0.6](https://joonaa.dev/blog/12/avian-0-6)
* [¡Vite 8.0 ha salido!](https://vite.dev/blog/announcing-vite8)
* [Construyendo macros procedurales de Rust sin comillas: Presentando zyn](https://aacebo.hashnode.dev/building-rust-procedural-macros-without-quote-introducing-zyn)
* [bnum v0.14.0: ¡muchas mejoras importantes!](https://github.com/isaacholt100/bnum/releases/tag/v0.14.0)
* [ClawShell: Asegurar el OpenClaw usando primitivas a nivel de sistema operativo](https://runta.com/blog/introducing-clawshell/)
* [Giff v1.1.0: Una interfaz de terminal para git diffs con soporte de rebase interactivo](https://github.com/bahdotsh/giff/releases/tag/v1.1.0)
* [mdterm v1.5.0: Un navegador Markdown basado en terminal](https://github.com/bahdotsh/mdterm/releases/tag/v1.5.0)
* [flodl - Un framework de deep learning nativo de Rust basado en libtorch](https://flodl.dev/blog/impl-drop-for-tensor)
* [Latilla v0.6: Perezoso por debajo](https://mackow.ski/blog/cot-v06-lazy-underneath/)

### Observaciones/Pensamientos

* [Resumen - Perspectivas del Proyecto Rust sobre la IA](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/feb27-summary.html)
* [Cómo usar la narración para encajar ensamblaje en línea en Rust](https://www.ralfj.de/blog/2026/03/13/inline-asm.html)
* [Por qué componentes WebAssembly](https://blog.yoshuawuyts.com/why-webassembly-components/)
* [sí, todas las coincidencias regulares más largas en tiempo lineal son posibles](https://iev.ee/blog/all-longest-regex-matches-in-linear-time/)
* [Accediendo al hardware en Rust](https://ferrous-systems.com/blog/hardware-access-rust/)
* [audio] [Netstack.FM episodio 31 — Protocolos cortos: Proxies MITM e Intercepción Transparente de L4](https://netstack.fm/#episode-31)
* [vídeo] [SpacetimeDB alimentado por Rust es mil veces más rápido? Fundador lo explica(https://www.youtube.com/watch?v=qfKBv3A0CVs)

### Guías de Rust
* [Construcción de imágenes Docker pequeñas y seguras para Rust: scratch vs alpine vs debian](https://kerkour.com/rust-docker-small-secure-images)
* [Parcheando LMDB: Cómo hicimos que el almacén vectorial de Meilisearch fuera un 333% más rápido](https://blog.kerollmops.com/patching-lmdb-how-we-made-meilisearch-s-vector-store-333-faster)
* [Creando un DAW en Rust - Reproduciendo audio](https://whoisryosuke.com/blog/2026/creating-a-daw-in-rust/)
* [Cómo comprobar la cobertura de códigos en Rust](https://barretts.club/posts/how-to-test-code-coverage-rust-2026/)
* [vídeo] [Lección 4 de RustCuriose: Estructuras y Recursos – Copiar vs Clonar vs Moverse](https://www.youtube.com/watch?v=r-Ag_21CKBI)

### Miscelánea
* [Tickets gratuitos de TokioConf para colaboradores y mantenedores de código abierto](https://tokio.rs/blog/2026-03-12-tokioconf-oss-tickets)

## Crate de la semana

El crate de esta semana es [grab](https://github.com/anwitars/grab), una herramienta de línea de comandos para convertir rápidamente CSV a JSON.

¡Gracias a [Gábor Maksa](https://users.rust-lang.org/t/crate-of-the-week/2704/1565) por la autosugerencia!

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

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Conferencia Oxid**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17
* [**NDC Techtown 2026**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-05-03 | Kongsberg, Noruega | 2026-09-21 - 2026-09-24

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

427 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-10..2026-03-17

#### Compilador
* [proporcionan mejores sugerencias para errores de inferencia en '.collect()?'](https://github.com/rust-lang/rust/pull/153925)

#### Biblioteca
* [añadir 'De' implica para tipos de envoltorio](https://github.com/rust-lang/rust/pull/146013)
* [en 'Option::get_or_insert_with()', olvida el 'Ninguno' en vez de eliminarlo](https://github.com/rust-lang/rust/pull/148562)
* [corregido 'VecDeque::splice()' no llenando correctamente el búfer al redimensionarlo en inicio = rango final](https://github.com/rust-lang/rust/pull/152258)

#### Carga
* ['CARGO_TARGET_DIR' no tiene por qué ser relativo](https://github.com/rust-lang/cargo/pull/16735)
* ['concha': Apoyo OSC 9; 4 Progreso en Ptyxis](https://github.com/rust-lang/cargo/pull/16730)
* ['compilar': Deja de negar advertencias sin ---seguir](https://github.com/rust-lang/cargo/pull/16725)
* [evitar el pánico para especificaciones del paquete con un fragmento vacío](https://github.com/rust-lang/cargo/pull/16754)
* [util: excluir de la sincronización de iCloud Drive en macOS](https://github.com/rust-lang/cargo/pull/16728)

#### Rustdoc
* ['rustdoc-json': Añadir soporte opcional para la (de)serialización de rkyv](https://github.com/rust-lang/rust/pull/153283)

#### Clippy
* [corregir 'match_same_arms' falso positivo con consts asociados](https://github.com/rust-lang/rust-clippy/pull/16701)
* [corrección: 'question_mark' sugerencia causada error](https://github.com/rust-lang/rust-clippy/pull/16656)
* [implementación refactorizada de 'unnecessary_{opción,resultado}_map_or_else'](https://github.com/rust-lang/rust-clippy/pull/15889)

#### Analizador de Rust
* [no activar GC en pruebas lentas](https://github.com/rust-lang/rust-analyzer/pull/21827)
* [La generación de SCIP debe activar las cachés en paralelo](https://github.com/rust-lang/rust-analyzer/pull/21828)
* [añadir validación de convención de nombres para tipos de 'unión'](https://github.com/rust-lang/rust-analyzer/pull/21794)
* [maneja identificadores UTF-8 de varios bytes en 'NameGenerator::suggest_name'](https://github.com/rust-lang/rust-analyzer/pull/21793)
* [inferir args genéricos para la referencia de rasgo y su tipo asociado](https://github.com/rust-lang/rust-analyzer/pull/21820)
* [eliminar corchetes angulares si todos los args de vida se eliminan en el tipo inline alias code assist](https://github.com/rust-lang/rust-analyzer/pull/21784)
* [reemplazar make usage por SyntaxFactory en algunos métodos utils de ayudas ide](https://github.com/rust-lang/rust-analyzer/pull/21826)

### Triaje de rendimiento del compilador Rust

Otra semana bastante tranquila, con pocos cambios y un rendimiento en general neutral.

Triaje hecho por **@simulacrum**.
Rango de revisión: [3945997a.. 5b61449e](https://perf.rust-lang.org/?start=3945997aabf6165261ef3419534c1ad59d9dc5c6&end=5b61449ed85a670f1dd3fca6a8c759ee0b451b66&absolute=false&stat=instructions%3Au)

1 regresión, 1 mejora, 2 mixta; 3 de ellos en rollups
35 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-03-16.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Mejoras en el formato de coincidencia](https://github.com/rust-lang/rust/issues/152763)
* [Corregir búsqueda retrasada de host SGX vía ToSocketAddr](https://github.com/rust-lang/rust/pull/152851)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Añadir 'homogeneous_try_blocks' RFC](https://github.com/rust-lang/rfcs/pull/3721)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [permitir 'incomplete_features' en pruebas de UI](https://github.com/rust-lang/compiler-team/issues/974)
* [Añadir '-Zsanitizer=kernel-hwaddress'](https://github.com/rust-lang/compiler-team/issues/975)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [[diseño de tipo] usize e isize tienen el mismo tamaño y alineación](https://github.com/rust-lang/reference/pull/2200)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Discutir proyecciones de subvenciones de viaje 2026](https://github.com/rust-lang/leadership-council/issues/276)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Permisos para eliminar cajas](https://github.com/rust-lang/rfcs/pull/3927)
* [¡Evita poner 'unreachable_code' en 'todo!' ()'](https://github.com/rust-lang/rfcs/pull/3928)
* [Propone el fondo de mantenimiento de la Fundación Rust](https://github.com/rust-lang/rfcs/pull/3931)

## Próximos eventos

Eventos Rusty entre el 18-03-2026 - el 15-04-2026 🦀

### Virtual
* 2026-03-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-18 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**¡Evento híbrido con Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313621933/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/45qqc2eo)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Adopción de Rust, Seguridad y Nube con Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Cajas, consejos y trucos Charlas relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-09 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)

### Asia
* 2026-03-19 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/313764176/)
* 222-03-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**Rust presencial marzo 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)
* 2026-03-28 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #13**](https://www.meetup.com/rustdelhi/events/313777790/)

### Europa
* 2026-03-18 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Introducción a Embedded Rust - Marzo**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-2026 | Varsovia, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Conferencia Rustikon**](https://www.rustikon.dev/)
* 2026-03-23 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #18**: Ludwig Weinzierl - Bevy: Spielentwicklung mit Rust](https://rust-augsburg.github.io/meetup/Meetup_18.html)
* 2026-03-23 | Ámsterdam, NL | [SecurityCon de código abierto](https://events.linuxfoundation.org/kubecon-cloudnativecon-europe/co-located-events/open-source-securitycon/)
    * [**SecurityCon de Código Abierto UE 2026**](https://rustfoundation.org/event/open-source-securitycon-eu-2026/)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack - Advenimiento del Código**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Marcha de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-24 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim)
    * [**Proyectos de Rust - mostrar y contar en marzo**](https://www.meetup.com/rust-trondheim/events/313537618/)
* 2026-03-25 | Dresde, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Primer encuentro**](https://github.com/rust-dresden/rust-dresden/discussions/7)
* 2026-03-26 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París**](https://www.rustinparis.com/)
* 2026-03-28 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #25**](https://www.meetup.com/stockholm-rust/events/313749232/)
* 2026-04-01 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | Londres, GB | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN habla de la Exposición Comunitaria de Primavera**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-07 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-09 | Ginebra, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust habla @ AutoStore – "Patrones para sistemas impulsados por eventos" y "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)

### Norteamérica
* 2026-03-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313569258/)
* 2026-03-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust aplicado - Aplicaciones de Rust en edificios**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-19 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Interoperabilidad Social - Rust, C++ y el Bien Mayor**](https://www.meetup.com/rust-nyc/events/313639707/)
* 2026-03-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Porter Square Rust, 21 de marzo**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Especial de Adopción de Activos Digitales de Rust NYC**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-04-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler con LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-09 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust April Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-14 | Charlottesville, VA, EE. UU. [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Afinando tus habilidades de Rust para entrevistas de trabajo**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)

### Oceanía
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Encuentro de marzo por determinar**](https://www.meetup.com/rust-melbourne/events/313471749/)

### Sudamérica
* 2026-03-21 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP (migrado pro Lumma)**](https://www.meetup.com/rust-sao-paulo-meetup/events/313446400/)
* 2026-04-11 | Argentina, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Lo que construimos colectivamente, más allá de los artefactos de código que son compiladores+herramientas, es un grupo de personas que regresan, que aprenden, que comparten su comprensión, que alinean sus gustos, que aceptan aportaciones de la comunidad, etcétera, etcétera. Fusionar un PR generado por un LLM solo alimenta la parte del Proyecto de "tenemos código que funciona"; No es participar en todos los demás ciclos de retroalimentación lo que hace que el proyecto tenga vida.

– [Nadrieril sobre las perspectivas del Proyecto Rust sobre la IA](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/feb27-summary.html#codebases-are-more-than-code)

A pesar de otra semana sin sugerencia, llogiq está satisfecho con su elección.

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1rxlv3m/this_week_in_rust_643/)</small>
