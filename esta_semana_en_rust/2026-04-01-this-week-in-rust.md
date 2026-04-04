---
title: "Esta semana en Rust #104"
number_of_week: 104
description: El crate de esta semana es tsastat de alta resolución para Linux.
date: 2026-04-01
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
* [Anunciando Rust 1.94.1](https://blog.rust-lang.org/2026/03/26/1.94.1-release/)
* [Actualización de gestión del programa — febrero de 2026 ](https://blog.rust-lang.org/inside-rust/2026/03/27/program-management-update-2026-02/)

### Fundación
* [La Fundación Rust se une al programa de código abierto de Datadog](https://rustfoundation.org/media/rust-foundation-joins-datadogs-open-source-program/)

### Actualizaciones de proyectos/herramientas
* [NTPD-rs: ¡ya era hora!](https://discourse.ubuntu.com/t/ntpd-rs-its-about-time/79154)
* [octopos: OS para RISC-V en Rust](https://www.boranseckin.com/projects/octopos)
* [Construcción de un entrenador de guitarra con Rust incrustado](https://blog.orhun.dev/introducing-tuitar/)
* [blogr v0.5.0 - bloguea sin salir de tu terminal](https://github.com/bahdotsh/blogr/releases/tag/v0.5.0)
* [alimentador v0.7.0 - lector de alimentación RSS/Atom basado en terminal](https://github.com/bahdotsh/feedr/releases/tag/v0.7.0)
* [mdterm v2.0.0 - navegador Markdown basado en terminal](https://github.com/bahdotsh/mdterm/releases/tag/v2.0.0)
* [RustGrep: búsqueda sencilla de 114 blogs de Rust](https://rustgrep.dev/)
* [Solucionador de rasgos de nueva generación de Rust](https://lwn.net/SubscriberLink/1063124/fcf747e51a5974f2/)
* [Rust Asíncrono Portátil](https://aimdb.dev/blog/building-aimdb-one-async-api)
* [jsongrep más rápido que {jq, jmespath, jsonpath-rust, jql}](https://micahkepe.com/blog/jsongrep/)
* [SeqPacker: 11 algoritmos de empaquetado de bins en Rust](https://alphakhaw.com/blog/seqpacker-bin-packing-algorithms-rust-llm)
* [flodl v0.2.2: Paridad de PyTorch en Rust](https://flodl.dev/blog/pytorch-parity)

### Observaciones/Pensamientos
* [filtra.io | Rompiendo el monopolio de la infraestructura artificial con Rust - Tracel AI](https://filtra.io/rust/interviews/tracel-mar-26)
* [Rust: Seguridad de memoria en espacio kernel | OSHub](https://oshub.org/users/OSHub/posts/rust-memory-safety-in-kernel-space-9178dd)
* [Solucionando nuestros propios problemas en el compilador de Rust](https://trifectatech.org/blog/fixing-our-own-problems-in-the-rust-compiler/)
* [Bugs que detecta el compilador Rust: La revolución de la corrección impuesta por compiladores](https://kerkour.com/rust-compiler-correctness-bugs)
* [Adapté el SDK de Python de OpenAI a Rust en 5 días con Claude Code](https://dev.to/fortunto2/squeezing-every-millisecond-from-the-openai-api-in-rust-4b11)
* [vídeo] [ 🦀 Compilador Rust (mir) morde: Closures — no nombrarás esta estructura](https://www.youtube.com/watch?v=OxK5pNvC20Y)
* [vídeo] [Cómo C++ finalmente vence a Rust en la serialización JSON](https://www.youtube.com/watch?v=Mcgk3CxHYMs)

### Guías de Rust
* [Añadiendo plugins WASM a tu app](https://blog.ar-ms.me/thoughts/adding-wasm-plugins-to-your-app/)
* [ZK responde sarcásticamente a Rust Developer parte 3/8](https://rustarians.com/execution-trace/)
* [Creando una cola de correo electrónico segura contra fallos en Rust](https://ferax564.github.io/rustqueue/blog/crash-safe-email-queue.html)
* [Añadiendo un motor de scripting a una CLI de Rust con Rhai](https://dev.to/ayarotsky/adding-a-scripting-engine-to-a-rust-cli-with-rhai-56g1)

## Crate de la semana

El crate de esta semana es [tsastat](https://github.com/AnkurRathore/tsastat), una herramienta de Análisis de Estado de Hilos (TSA) de alta resolución para Linux.

¡Gracias a [Ankur Rathore](https://users.rust-lang.org/t/crate-of-the-week/2704/1574) por la autosugerencia!

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

* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | CFP abierto hasta el 14-04-2024 | Kongsberg, Noruega | 2024-09-09 - 2026-09-12.
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

487 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-24..2026-03-31

#### Compilador
* [añadir el objetivo 'x86_64-unknown-linux-gnu{m,t}san' que habilita {M,T}San por defecto](https://github.com/rust-lang/rust/pull/152757)
* [añadir '-Zsanitize=kernel-hwaddress'](https://github.com/rust-lang/rust/pull/153049)

#### Biblioteca
* [constificar el rasgo 'Escalón' y todas sus 'impl'ementaciones](https://github.com/rust-lang/rust/pull/153821)
* [constifican comparaciones y 'Clone' para 'core::mem::Alignment'](https://github.com/rust-lang/rust/pull/154512)
* [constify const Fn*: Destruct](https://github.com/rust-lang/rust/pull/153874)
* [No dejes caer los temporales de argumentos en 'DBG!'](https://github.com/rust-lang/rust/pull/154074)
* [no fusionarse en 'MapWindows'](https://github.com/rust-lang/rust/pull/154190)
* [implementa 'unchecked_funnel_{shl,shr}'](https://github.com/rust-lang/rust/pull/154153)
* [reimplementar la macro 'hash_map!'](https://github.com/rust-lang/rust/pull/154322)
* [hacer que 'PinCoerceUnsized' requiera 'Deref'](https://github.com/rust-lang/rust/pull/149218)
* [estabilizar nuevo tipo RangeFrom e iterador](https://github.com/rust-lang/rust/pull/153380)
* ['trim_prefix' de caminos](https://github.com/rust-lang/rust/pull/154320)

#### Carga
* ['resolver': Mejor coincidir con Rustc en estilo de error](https://github.com/rust-lang/cargo/pull/16795)
* ['build': cubre más comportamiento de 'build.warnings'](https://github.com/rust-lang/cargo/pull/16785)
* ['build': facilitar la revisión del comportamiento de 'build.warnings'](https://github.com/rust-lang/cargo/pull/16788)

#### Rustdoc
* [rustdoc rechaza emisiones html con salida json](https://github.com/rust-lang/rust/pull/154421)

#### Rustfmt
* [prevenir el pánico al reescribir delegaciones de ítems asociadas](https://github.com/rust-lang/rust/pull/154454)

#### Clippy
* [añadir 'manual_option_zip' pelusa ('a.and_then(|x| b.map(|y| (x, y)))')](https://github.com/rust-lang/rust-clippy/pull/16600)
* [impl pelusa de 'manual_noop_waker'](https://github.com/rust-lang/rust-clippy/pull/16687)
* ['explicit_counter_loop': sugiere '.take(n)' para 'for _ in 0..n' co...](https://github.com/rust-lang/rust-clippy/pull/16658)
* ['iter_kv_map': asignar el mapa de identidad para 'mapa' y 'flat_map'](https://github.com/rust-lang/rust-clippy/pull/16743)
* ['manual_pop_if': Más casos de pelusa, aunque no proporcionemos una sugerencia](https://github.com/rust-lang/rust-clippy/pull/16683)
* [corregir 'collapsible_if' falso positivo cuando el if interno contiene cfg](https://github.com/rust-lang/rust-clippy/pull/16757)
* [conservar paréntesis en la sugerencia en presencia de moldes en cascada](https://github.com/rust-lang/rust-clippy/pull/16483)
* [perf: reducir el uso de 'matching_root_macro_call' (23b → 22,24b)](https://github.com/rust-lang/rust-clippy/pull/16756)

#### Analizador de Rust
* [corrección no aplicable en identificador ambiguo para 'merge_match_arms'](https://github.com/rust-lang/rust-analyzer/pull/21411)
* [Envs completos en 'env anidado! ()'](https://github.com/rust-lang/rust-analyzer/pull/21902)
* [índice de parámenos 'type_or_const' correcto acotado en 'debug_assert'](https://github.com/rust-lang/rust-analyzer/pull/21879)
* [corrección de mensajes de args perdidos para 'sched_getaffinity' y cuñas getenv](https://github.com/rust-lang/rust-analyzer/pull/21881)
* [no te asustes desintegrar brazo en el tubo trasero](https://github.com/rust-lang/rust-analyzer/pull/21904)
* [Bloqueo de corrección bajando en el mapa ID de AST](https://github.com/rust-lang/rust-analyzer/pull/21907)
* [mantener comentarios para 'Llenar armas de coincidencia'](https://github.com/rust-lang/rust-analyzer/pull/21744)
* [completaciones de posfijo incluyen nots prefijo-expr](https://github.com/rust-lang/rust-analyzer/pull/21903)
* [Saltar usos dentro de expansiones macro en la unión de estructura/tupla de desestructuración](https://github.com/rust-lang/rust-analyzer/pull/21838)
* [volver a 'TyLoweringContext.store' a sí mismo tras reducir los valores predeterminados de los padres](https://github.com/rust-lang/rust-analyzer/pull/21871)
* [envolver 'Option<>' por 'desugar_try_expr_let_else'](https://github.com/rust-lang/rust-analyzer/pull/21860)
* [envuelva 'Resulta<>' para 'desugar_try_expr_let_else'](https://github.com/rust-lang/rust-analyzer/pull/21865)
* [envuelve ty-anchor en un constuctor de tipo no-camino](https://github.com/rust-lang/rust-analyzer/pull/21876)
* [implementar completamente el soporte de expresiones 'VariantFields'](https://github.com/rust-lang/rust-analyzer/pull/21900)
* [conexión Inferencia de Firma en más lugares](https://github.com/rust-lang/rust-analyzer/pull/21859)
* [solo asigna bloques de ítems si realmente contienen elementos o macros de sentencia](https://github.com/rust-lang/rust-analyzer/pull/21901)
* [eliminar 'Arc' de 'GenericParams' y 'AstIdMap'](https://github.com/rust-lang/rust-analyzer/pull/21897)
* [eliminar generar rasgo impl texto intransitivo de utils](https://github.com/rust-lang/rust-analyzer/pull/21870)

### Triaje de rendimiento del compilador Rust

Esta semana tuvimos algunos problemas de infraestructura que impidieron que algunos RPs rollup generaran su
"desenrollado" construcciones, lo que complicaba la investigación de regresión de rollups, aunque nosotros sí
capaz de localizar y revertir las mayores regresiones de rollup al final. [#154304](https://github.com/rust-lang/rust/pull/154304) trajo algunas mejoras interesantes optimizando el sistema de consultas.

Triaje hecho por **@kobzol**.
Rango de revisión: [6f22f613.. cf7da0b7](https://perf.rust-lang.org/?start=6f22f61305478df09f9a4523743f85d9f558c3d7&end=cf7da0b7277cad05b79f91b60c290aa08a17a6f0&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,1%, 1,2%] | 4 |
| Regresiones ❌ <br /> (secundario) | 0,3% | [0,1%, 0,5%] | 12 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-6,2%, -0,2%] | 58 |
| Mejoras ✅ <br /> (secundario) | -0,4% | [-1,9%, -0,1%] | 28 |
| Todos ❌✅ (primario) | -0,8% | [-6,2%, 1,2%] | 62 |

3 regresiones, 4 mejoras, 2 mixtas; 2 de ellos en rollups
35 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/23c7a57ddd710830f9ae14d2676718587e9dc412/triage/2026/2026-03-31.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [Aplicación de mitigaciones](https://github.com/rust-lang/rfcs/pull/3855)
* [Añadir 'homogeneous_try_blocks'](https://github.com/rust-lang/rfcs/pull/3721)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Problema de seguimiento para 'isolate_most_least_significant_one'](https://github.com/rust-lang/rust/issues/136909)
* [Problema de seguimiento para 'int_lowest_highest_one'](https://github.com/rust-lang/rust/issues/145203)
* [Problema de seguimiento para 'uint_bit_width'](https://github.com/rust-lang/rust/issues/142326)
* [Problema de seguimiento para #138068: Añadir 'Resultado::map_or_default' y 'Opción::map_or_default'](https://github.com/rust-lang/rust/issues/138099)
* [No uses expectativas de entrada que no sean wf de fudge al revisar llamadas a funciones](https://github.com/rust-lang/rust/pull/150316)
* [Rechazar sintácticamente predicados de igualdad](https://github.com/rust-lang/rust/pull/153513)
* [Problema de seguimiento para tcp_deferaccept](https://github.com/rust-lang/rust/issues/119639)
* [estabilizar registros vectoriales S390X](https://github.com/rust-lang/rust/pull/154184)
* [Sustituyendo la autoescritura por una resolución adecuada](https://github.com/rust-lang/rust/pull/152996)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [build-std: siempre](https://github.com/rust-lang/rfcs/pull/3874)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Rasgos asociados](https://github.com/rust-lang/rfcs/pull/3938)
* [Añadir política de contribuciones \*estar presente](https://github.com/rust-lang/rfcs/pull/3936)
* [Borrador inicial para los objetivos del proyecto 2026](https://github.com/rust-lang/rfcs/pull/3935)

## Próximos eventos

Eventos Rusty entre el 01-04-2026 - el 29-04-2026 🦀

### Virtual
* 2026-03-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455925/)
* 31-03-2026 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 1**](https://www.meetup.com/code-mavens/events/313944077/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-05 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/313846136/)
* 2026-04-07 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 2**](https://www.meetup.com/code-mavens/events/313944233/)
* 2026-04-09 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 3**](https://www.meetup.com/code-mavens/events/314072969/)
* 2026-04-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/jia7wtfv)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/314007434/)
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: De Protobuf a Producción - Guía para gRPC en Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/8hi2xywi)

### Asia
* 2026-04-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Abril 2026/Encuentro previo a la conferencia de los Rustacean**](https://hasgeek.com/rustbangalore/april-2026-pre-conference-rustacean-meetup/)
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Taller de Rust India**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Conferencia Rust India**](https://rustindia.org/schedule)

### Europa
* 2026-04-01 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (pub vespertino)**](https://www.meetup.com/rust-and-friends/events/313898254/)
* 2026-04-01 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Oxidación en abril: De cero a Rust**](https://www.meetup.com/rustcologne/events/313947839/)
* 2026-04-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**¡Vino de otra dimensión!**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**LDN habla de la Exposición Comunitaria de Primavera**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-02 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Lanzamiento, Servidores y Ray Tracing Desmitificados**](https://www.meetup.com/rust-community-toulouse/events/313650892/)
* 2026-04-03 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café de día)**](https://www.meetup.com/rust-and-friends/events/313898258/)
* 2026-04-07 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-07 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Construyendo un dron desde cero con Rust (y algo de hardware)**](https://www.meetup.com/rust-rhein-main/events/314051660/)
* 2026-04-08 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2026**](https://luma.com/z8aoscr9)
* 2026-04-09 | Ginebra, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust habla @ AutoStore – "Patrones para sistemas impulsados por eventos" y "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**GUI nativas con Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla y fiesta de cumpleaños en MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)

### Norteamérica
* 2026-04-02 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313543900/)
* 2026-04-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler con LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Winter Hill Rust, 4 de abril**](https://www.meetup.com/bostonrust/events/313883689/)
* 2026-04-07 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: El Especial de Código Abierto**](https://www.meetup.com/rust-nyc/events/313946458/)
* 2026-04-09 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz Oxid**](https://www.meetup.com/chicago-rust-meetup/events/313987321/)
* 2026-04-09 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust April Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Brookline Rust, 11 de abril**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, EE. UU. [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Afinando tus habilidades de Rust para entrevistas de trabajo**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en la Plaza de Harvard, 18 de abril**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-23 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Oxida LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Estación Sur, 25 de abril**](https://www.meetup.com/bostonrust/events/313883704/)

### Oceanía
* 2026-04-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Oxidar Brisbane abr 2026**](https://www.meetup.com/rust-brisbane/events/313975190/)

### Sudamérica
* 2026-04-11 | Buenos Aires, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)
* 2026-04-17 | Río de Janeiro, BR | [Encuentros con Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Reunión Rust RJ**](https://luma.com/ce46pl7z)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Cuando haces cosas malditas, los problemas te encontran.

– [Folkert de Vries en el blog tecnológico trifecta](https://trifectatech.org/blog/fixing-our-own-problems-in-the-rust-compiler)

Llevamos cuatro semanas sin sugerencias de presupuestos. Llogiq sigue de acuerdo con su elección, pero estaría mucho más contento si alguno de vosotros le ayudara en su búsqueda.

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

<small>[Debate en r/rust](https://this-week-in-rust.org/blog/2026/04/01/this-week-in-rust-645/)</small>
