---
title: "Esta semana en Rust #85"
number_of_week: 85
description: El crate de esta semana es cargo cat, un subcomando de carga para poner una cara aleatoria de gato ascii en tu terminal.
date: 2025-11-19
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
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

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
* [Lanzamiento de la Encuesta sobre el Estado del Rust 2025](https://blog.rust-lang.org/2025/11/17/launching-the-2025-state-of-rust-survey)
* [Resultados de Google Summer of Code 2025](https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/)
* [Actualización de objetivos del proyecto — octubre de 2025](https://blog.rust-lang.org/2025/11/19/project-goals-update-october-2025/)
* [Actualización de objetivos del proyecto — septiembre de 2025](https://blog.rust-lang.org/2025/11/19/Project-Goals-2025-September-Update/)

### Boletines
* [Scientific Computing in Rust #12 (noviembre de 2025)](https://scientificcomputing.rs/monthly/2025-11)
* [Desarrollo de firmware seguro por diseño con Wasefire](https://opensource.googleblog.com/2025/11/secure-by-design-firmware-development-with-wasefire.html)
* [Rust Trends Número #72: De lo experimental a lo empresarial: El momento de producción de Rust](https://rust-trends.com/newsletter/experimental-to-enterprise-rust-production)

### Actualizaciones de proyectos/herramientas
* [GuardianDB v0.11.28](https://www.willsearch.com.br/blog/2025/11/18/whats-new-in-guardiandb-v0-11-18/)
* [Estado actual del soporte para arquitectura Linux](https://lwn.net/SubscriberLink/1045363/60611dc5ec3f7099/)

### Observaciones/Pensamientos
* [audio] [Netstack.FM Episodio 14 – Roto And Cascade con Terts y Arya de NLnet Labs](https://netstack.fm/#episode-14)
* [Mejorando el sistema incremental en el compilador Rust](https://blog.goose.love/posts/improving-the-incremental-system-in-the-rust-compiler/)
* [Punteros Inteligentes Personalizados Verdaderamente de Primera Clase](https://nadrieril.github.io/blog/2025/11/11/truly-first-class-custom-smart-pointers.html)
* [Clavar es una especie de préstamo estático](https://nadrieril.github.io/blog/2025/11/12/pinning-is-a-kind-of-static-borrow.html)
* [Rust en Android: muévete rápido y arregla cosas](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)
* [Iguala de nuevo Sam](https://www.sminez.dev/match-it-again-sam/)
* [La humanidad está manchada por los pecados de C y ningún LLM puede reescribirlos hasta convertirlos en Rust](https://kirancodes.me/posts/log-sins-of-c.html)
* [UV y Ruff: Desarrollo turboalimentado de Python con herramientas alimentadas por Rust](https://www.devtoolsacademy.com/blog/uv-and-ruff-turbocharging-python-development-with-rust-powered-tools/)
* [Un inliner funcional para wasmtime y Cranelift](https://fitzgen.com/2025/11/19/inliner.html)

### Guías de Rust
- [Pruebas unitarias de Rust: bibliotecas de aserciones](https://jorgeortiz.dev/posts/rust_unit_testing_assertion_libraries/)
- [Pruebas unitarias de Rust: Uso de una biblioteca de mocking](https://jorgeortiz.dev/posts/rust_unit_testing_mocking_library/)
* [Guía práctica para la transición a lenguajes con seguridad en la memoria (https://queue.acm.org/detail.cfm?id=3773096)
* [Construcción del protocolo WebSocket en Apache Iggy usando arquitectura de E/S basada en io_uring y completación](https://iggy.apache.org/blogs/2025/11/17/websocket-io-uring/)
* [Construcción de aplicaciones serverless con Rust en AWS Lambda](https://aws.amazon.com/blogs/compute/building-serverless-applications-with-rust-on-aws-lambda/)
* [Prohibir el uso de códigos con un 'clippy.toml' personalizado](https://www.schneems.com/2025/11/19/find-accidental-code-usage-with-a-custom-clippytoml/)

### Miscelánea
* [¿Rust absurdo? ¡Nunca!](https://academy.fpblock.com/blog/absurd-rust-never/?share=1)
* [vídeo] [Linus Torvalds — Habla sobre la División del Rust y dice NO](https://www.youtube.com/watch?v=amyKC9lJe3Q)
* [Informe de Empleos de Rust de octubre 2025](https://filtra.io/rust/jobs-report/oct-25)
* [Ventaja Estratégica de Rust](https://sysid.github.io/rusts-strategic-advantage/)

## Crate de la semana

El crate de esta semana es [cargo cat](https://crates.io/crates/cat-ascii-faces), un subcomando de carga para poner una cara aleatoria de gato ascii en tu terminal.

¡Gracias a [Alejandra Gonzáles](https://users.rust-lang.org/t/crate-of-the-week/2704/1490) por la autosugerencia!

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

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si eres un implementador de funciones y quieres que tu RFC aparezca en la lista anterior, añade la nueva 'llamada para pruebas'
etiqueta a tu RFC junto con un comentario que ofrezca instrucciones de prueba y/o orientación sobre qué aspecto(s) de la funcionalidad
Necesito pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

* [GuardianDB - Crear y traducir documentación al inglés](https://github.com/wmaslonek/guardian-db/issues/3)
* [GuardianDB - Aumentar la cobertura de pruebas (actualmente 13%)](https://github.com/wmaslonek/guardian-db/issues/4)
* [GuardianDB - Crear ejemplos de uso cohesivos](https://github.com/wmaslonek/guardian-db/issues/5)
* [GuardianDB - Nodo IPFS de Iroh de backend](https://github.com/wmaslonek/guardian-db/issues/6)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.
* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/) \| Cierre del CFP: 24-11-2025 23:59 \| Varsovia, Polonia \| Evento: 2025-03-19-2025-03-2025 [Sitio web del evento](https://www.rustikon.dev/)<!-- Los CFPs van aquí, usen este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP cierra el 08-12-2025 | Portland, Oregón, EE. UU. | 2026-04-20
* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/)| CFP cierra el 31-12-2025 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

427 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-11..2025-11-18

#### Compilador
* [añadir nueva pelusa de 'function_casts_as_integer'](https://github.com/rust-lang/rust/pull/141470)
* [MIRI: Implementación inicial de la procedencia comodín para préstamos de árboles](https://github.com/rust-lang/miri/pull/4630)
#### Biblioteca
* [¡nuevo 'format_args! ()' y 'fmt::Argumentos' implementación](https://github.com/rust-lang/rust/pull/148789)
* ['vec_recycle': implementación](https://github.com/rust-lang/rust/pull/148416)
* [implementa 'Leer::read_array'](https://github.com/rust-lang/rust/pull/148850)
* [estabilizar 'char_max_len'](https://github.com/rust-lang/rust/pull/145610)
* [estabilizar 'duration_from_nanos_u128'](https://github.com/rust-lang/rust/pull/148587)
* [estabilizar 'extern_system_varargs'](https://github.com/rust-lang/rust/pull/145954)
* [estabilizar 'vec_into_raw_parts'](https://github.com/rust-lang/rust/pull/148827)
* [constify 'ManuallyDrop::take'](https://github.com/rust-lang/rust/pull/148752)
* [constifica 'mem::take'](https://github.com/rust-lang/rust/pull/148757)
* [eliminar 'rustc_inherit_overflow_checks' de 'posición()' en iteradores de cortes](https://github.com/rust-lang/rust/pull/148944)
#### Carga
* ['cli': añadir soporte para completar valores de '--config' en Bash](https://github.com/rust-lang/cargo/pull/16245)
* ['árbol': soporte para formas largas para variables --format](https://github.com/rust-lang/cargo/pull/16204)
* ['config': recurso a ruta no canónica para workspace-path-hash](https://github.com/rust-lang/cargo/pull/16248)
* ['manifest': señalar cuándo una clave pertenece a config](https://github.com/rust-lang/cargo/pull/16256)
* ['paquete': todas las entradas de alquitrán tienen la misma marca de tiempo](https://github.com/rust-lang/cargo/pull/16242)
* [no bloquear el director de artefactos para builds de tirada](https://github.com/rust-lang/cargo/pull/16230)
* [añadir bandera rustc-unicode inestable](https://github.com/rust-lang/cargo/pull/16243)
#### Rustdoc
* [Corregir generación de enlaces macro saltos a defensa inválida](https://github.com/rust-lang/rust/pull/148080)
* [no ignorar la distancia de ruta para los alias de los documentos](https://github.com/rust-lang/rust/pull/147701)
* [no pases 'RenderOptions' a 'DocContext'](https://github.com/rust-lang/rust/pull/147832)
* [microoptimizar 'render_item', mover cosas fuera del camino común](https://github.com/rust-lang/rust/pull/148877)
* [cambios en la calidad de vida](https://github.com/rust-lang/rust/pull/148466)
#### Clippy
* ['ok_expect': añadir autofixación](https://github.com/rust-lang/rust-clippy/pull/15867)
* [{'innecesario', 'pánico'}'_unwrap': acceso a campos de pelusa](https://github.com/rust-lang/rust-clippy/pull/15949)
* ['equatable_if_let': no sugieres '=' en contexto const](https://github.com/rust-lang/rust-clippy/pull/16092)
* ['rc_buffer': no toques el camino hacia 'Rc'/'Arc' en la sugerencia](https://github.com/rust-lang/rust-clippy/pull/15803)
* ['incompatible_msrv': no compruebes el contenido de ninguna macro 'STD'](https://github.com/rust-lang/rust-clippy/pull/16083)
* [añadir una pelusa de 'doc_paragraphs_missing_punctuation'](https://github.com/rust-lang/rust-clippy/pull/15758)
* [corregir 'single_range_in_vec_init' falso positivo para 'Rango' explícito](https://github.com/rust-lang/rust-clippy/pull/16043)
* [corregir 'sliced_string_as_bytes' falso positivo con un 'RangeFull'](https://github.com/rust-lang/rust-clippy/pull/15873)
* [arreglar interacciones en el historial de la web](https://github.com/rust-lang/rust-clippy/pull/16060)
* [reelaboración de 'missing_docs_in_private_items'](https://github.com/rust-lang/rust-clippy/pull/14741)
#### Analizador de Rust
* [corrección eliminada de la función 'doc_auto_cfg' para 'smol_str' lib](https://github.com/rust-lang/rust-analyzer/pull/21021)

### Triaje de rendimiento del compilador Rust

¡Semana positiva, sobre todo por la nueva format_args! () y fmt::Arguments implementación de [#148789](https://github.com/rust-lang/rust/pull/148789). Otra mejora notable vino de mover algunos cálculos de una etapa del compilador a otra para ahorrar memoria y recorridos innecesarios de árboles en [#148706](https://github.com/rust-lang/rust/pull/148706)

Triaje hecho por **@panstromek**.
Rango de revisión: [055d0d6a.. 6159a440](https://perf.rust-lang.org/?start=055d0d6aaf937cc11b3d2a5b5725972723b7f3c6&end=6159a44067ebce42b38f062cc7df267a1348e092&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 1,6% | [0,2%, 5,6%] | 11 |
| Regresiones ❌ <br /> (secundario) | 0,3% | [0,1%, 1,1%] | 26 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-4,5%, -0,1%] | 161 |
| Mejoras ✅ <br /> (secundario) | -1,4% | [-38,1%, -0,1%] | 168 |
| Todos ❌✅ (primario) | -0,6% | [-4,5%, 5,6%] | 172 |

2 regresiones, 4 mejoras, 10 mixtas; 4 de ellos en rollups
En total se realizaron 48 comparaciones de artefactos

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/8cb481daaea8c43b1d694184b0a58fa93001ece6/triage/2025/2025-11-19.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Estabilizar '-Zremap-path-scope'](https://github.com/rust-lang/rust/pull/147611)
* [coacción variada limpia y gestiona correctamente la seguridad](https://github.com/rust-lang/rust/pull/148602)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Contratos: afirmaciones de propiedad primitivas: 'poseído' y 'bloquear'](https://github.com/rust-lang/compiler-team/issues/942)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o 
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 19-11-2025 - el 17-12-2025 🦀

### Virtual
* 2025-11-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/l2xukapz)
* 2025-11-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Noviembre de 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock, un sistema operativo basado en Rust Parte #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de Lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Boulder, CO, EE.UU.) | [Elixir de Roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrando Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-25 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Sistemas intensivos en datos en Rust: Seguridad, Velocidad, Concurrencia**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/q5tjirkt)
* 27-11-2025 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Meet de Noviembre - ¡Runtimes asíncronos, parte 2!**](https://www.meetup.com/rust-argentina/events/312061828/)
* 30-11-2025 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de Rust Readers: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Advento del código - ¡Arranca!**](https://www.meetup.com/women-in-rust/events/311068648/)
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

### Asia
* 2025-11-20 | Tokio, JP | [Encuentro de Tokyo Rust](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Rastreando el Rust a escala**](https://www.meetup.com/tokyo-rust-meetup/events/311817069/)

### Europa
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**Círculo de QA**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche Social**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Lucerna, CH | [Rust Luzern](https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Guías de cajas @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Berna, CH | [Bern Oxidado](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #5 @Source Ingenieros**](https://www.meetup.com/rust-bern/events/311792568/)
* 27-11-2025 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #16: Christian Meusel - Oxidando paso a paso**](https://rust-augsburg.github.io/meetup/Meetup_16.html)
* 27-11-2025 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**19º encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/311787736/)
* 27-11-2025 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Tipos de tamaño exótico, y Rust en el espacio en la aguja!**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Praga, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 03-12-2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 03-12-2025 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Encuentro Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 2025-12-08 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)

### Norteamérica
* 2025-11-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Noviembre de 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, EE. UU. [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro mensual de Rust: Imprimación incrustada y codificación libre**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-23 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo y Hackathon Inman Rust, 23 de noviembre**](https://www.meetup.com/bostonrust/events/311917854/)
* 2025-11-26 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-26 | Phoenix, AZ, EE. UU. | [Rust del Desierto](https://www.meetup.com/desert-rustaceans)
    * [**Booze.rs**](https://www.meetup.com/desert-rustaceans/events/312000222/)
* 27-11-2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Harvard Square, 29 de noviembre**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Talk diciembre**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 04-12-2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 04-12-2025 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Dominando el estado, la seguridad y los manejadores escalables en Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 05-12-2025 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: ¡Nuestro mayor evento hasta la fecha!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Rust en el centro, 6 de diciembre**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Robótica Competitiva con Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 13 de diciembre**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)

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

> Adoptamos Rust por su seguridad y estamos viendo una reducción de mil veces en la densidad de vulnerabilidades de seguridad de memoria en comparación con el código C y C++ de Android. Pero la mayor sorpresa fue el impacto de Rust en la entrega de software. Con los cambios en Rust teniendo una tasa de rollback 4 veces menor y dedicando un 25% menos de tiempo a la revisión de código, el camino más seguro ahora es también el más rápido.

– [Jeff Vander Stoep en el blog de Google Android](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)

¡Gracias a [binarycat](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1728) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1p1q0zt/this_week_in_rust_626/)</small>
