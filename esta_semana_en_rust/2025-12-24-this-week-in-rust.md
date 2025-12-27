---
title: "Esta semana en Rust #90"
number_of_week: 90
description: El crate de esta semana es arcshift, un sustituto de Arc para cargas de trabajo con mucha lectura que soporta reemplazo atómico sin bloqueo.
date: 2025-12-24
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
* [¿Qué es lo que le gusta a la gente de Rust?](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/)
* [Por favor, envíe propuestas de objetivos del proyecto 2026](https://blog.rust-lang.org/inside-rust/2025/12/16/please-submit-2026-project-goal-proposals/)
* [Actualización del Director de Proyecto de diciembre de 2025](https://blog.rust-lang.org/inside-rust/2025/12/19/project-director-update/)
* [Actualización de gestión del programa — Finales de 2025](https://blog.rust-lang.org/inside-rust/2025/12/19/program-management-update--end-of-2025/)
* [Rustup 1.29.0 beta: ¡Llamada para pruebas!](https://blog.rust-lang.org/inside-rust/2025/12/20/rustup-1.29.0-beta-cft/)

### Boletines
* [El Rustaceano Incrustado Número #61](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-61)
* [Computación Científica en Rust #13 (diciembre de 2025)](https://scientificcomputing.rs/monthly/2025-12)

### Actualizaciones de proyectos/herramientas
* [¿Qué hay de "nuevo" en Miri (¡y además, hay un periódico de Miri!)](https://www.ralfj.de/blog/2025/12/22/miri.html)
* [acoplamiento de carga: Visualización del acoplamiento en proyectos Rust](https://syu-m-5151.hatenablog.com/entry/2025/12/21/152559)
* [Anunciando Asterinas 0.17.0](https://asterinas.github.io/2025/12/19/announcing-asterinas-0.17.0.html)
* [Tuitar - Herramienta portátil de entrenamiento de guitarra y kit DIY](https://github.com/orhun/tuitar/releases/tag/v0.1.0)
* [GitRust en diciembre](https://github.com/GitoxideLabs/gitoxide/discussions/2300)
* [Anunciando GotaTun, el futuro de WireGuard en Mullvad VPN](https://mullvad.net/en/blog/announcing-gotatun-the-future-of-wireguard-at-mullvad-vpn)
* [wgpu v28.0.0 - Shaders de malla, inmediatos y más!](https://github.com/gfx-rs/wgpu/releases/tag/v28.0.0)
* [rustc_codegen_gcc: Informe de progreso #39](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-39)

### Observaciones/Pensamientos
* [Reflexiones sintácticas sobre el efecto de falibilidad](https://blog.yoshuawuyts.com/syntactic-musings-on-the-fallibility-effect/)
* [Patrón de bloques de Rust](https://notgull.net/block-pattern/)
* [audio] [Netstack.FM episodio 19 — Firezone y seguridad de redes de confianza cero con Thomas Eizinger](https://netstack.fm/#episode-19)

### Guías de Rust
* [Rust Unit Testing: Servidor HTTP básico](https://jorgeortiz.dev/posts/rust_unit_testing_basic_http_srvr/)
* [Fontanería Bluetooth Rust Assíncrona: Dónde va el rendimiento](https://medium.com/@potto_94870/async-rust-bluetooth-plumbing-where-the-throughput-goes-d2cf21430a90)
* [serie] [Parte 2: Operaciones de Tensores, Construyendo un LLM desde cero en Rust](https://www.tag1.com/how-to/part2-tensor-operations-building-an-llm-from-scratch/)

## Crate de la semana

El crate de esta semana es [arcshift](https://docs.rs/arcshift), un sustituto de Arc para cargas de trabajo con mucha lectura que soporta reemplazo atómico sin bloqueo.

¡Gracias a [Rustkins](https://users.rust-lang.org/t/crate-of-the-week/2704/1510) por la sugerencia!

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
<!-- * [ - ]() -->
*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 18-01-2026 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-10

<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

475 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-16..2025-12-23

#### Compilador
* [añadir 'target_feature = "gc"' para Wasm](https://github.com/rust-lang/rust/pull/150111)
* [mejor propagación de requisitos de cierre](https://github.com/rust-lang/rust/pull/148329)
* [codificar correctamente metadatos de atributos doc](https://github.com/rust-lang/rust/pull/149919)
* [no trates las afirmaciones como una llamada en línea cruzada](https://github.com/rust-lang/rust/pull/117192)
* [mejora la codificación y miscelánea de nombres de archivo](https://github.com/rust-lang/rust/pull/149989)
* [hacer que la captura de cierre tenga un comportamiento consistente y correcto alrededor de los patrones](https://github.com/rust-lang/rust/pull/138961)
* [soportar delegación recursiva](https://github.com/rust-lang/rust/pull/150024)

#### Biblioteca
* [añadir 'try_as_dyn' y 'try_as_dyn_mut'](https://github.com/rust-lang/rust/pull/150033)
* [añadir const por defecto para OnceCell y OnceLock](https://github.com/rust-lang/rust/pull/149812)
* [expandir 'str_as_str' a más tipos](https://github.com/rust-lang/rust/pull/145933)
* [hacer que 'const BorrowMut' requiera 'const Borrow' y que 'const Fn' requiera 'const FnMut'](https://github.com/rust-lang/rust/pull/147939)
* [hashbrown: añadir 'hash_map::{OccupiedEntry::into_entry', 'VacantEntryRef::insert_entry_with_key}', hacer que 'EntryRef' use 'ToOwned' de nuevo](https://github.com/rust-lang/hashbrown/pull/670)
* [hashbrown: añadir 'hash_table::OccupiedEntry::replace_entry_with' para imitar la API HashMap](https://github.com/rust-lang/hashbrown/pull/669)
* [hashbrown: añadir el método 'hash_table::UnsafeIter', 'iter()' a varios iteradores](https://github.com/rust-lang/hashbrown/pull/667)

#### Rustdoc
* [Añadir etiquetas de cierre faltantes en las reexportaciones de cajas externas](https://github.com/rust-lang/rust/pull/150185)
* [Corregir la gestión inválida del campo seguida de la llamada de macro negada](https://github.com/rust-lang/rust/pull/150099)
* [generar expansión de macro para compiladores de Rust crates documentación](https://github.com/rust-lang/rust/pull/150022)
* [manejar expansiones de macros en tipos](https://github.com/rust-lang/rust/pull/150221)

#### Clippy
* ['transmuting_null': Comprobar casts enteros const](https://github.com/rust-lang/rust-clippy/pull/16227)
* [permitir sugerencias multilínea en 'map-unwrap-or'](https://github.com/rust-lang/rust-clippy/pull/16114)
* [no intentar usar 'nth' con argumento que no sea usize](https://github.com/rust-lang/rust-clippy/pull/16272)
* [no emitir pelusa 'collapsible_else_if' cuando todos los brazos contienen solo expresiones 'si {} si no, {}'](https://github.com/rust-lang/rust-clippy/pull/16286)
* [corregir paréntesis faltantes de 'cmp_null' en el ejemplo](https://github.com/rust-lang/rust-clippy/pull/16282)
* [corregir 'empty_enum_variants_with_brackets' falla al quitar los paréntesis en los patrones](https://github.com/rust-lang/rust-clippy/pull/16160)
* [corrige 'if_then_some_else_none' sugiere erróneamente cuando termina con comentario](https://github.com/rust-lang/rust-clippy/pull/16278)
* [corregir 'needless_type_cast' sugiriendo código inválido para inicializadores no literales](https://github.com/rust-lang/rust-clippy/pull/16248)
* [error causado por la sugerencia 'println_empty_string' de corrección](https://github.com/rust-lang/rust-clippy/pull/16201)
* [corregir 'use_self' falso positivo en el tipo en genéricos de const](https://github.com/rust-lang/rust-clippy/pull/16172)
* [corregir un mensaje de error incorrecto respecto al tamaño de 'usize' y 'isize' en 'cast_precision_loss'](https://github.com/rust-lang/rust-clippy/pull/14966)
* [mueve 'collapsible_else_if' a 'pedante'](https://github.com/rust-lang/rust-clippy/pull/16211)
* [nueva pelusa - 'same_length_and_capacity'](https://github.com/rust-lang/rust-clippy/pull/15656)

#### Analizador de Rust
* [añadir la sección 'Uso de herramientas de IA' a CONTRIBUTING.md](https://github.com/rust-lang/rust-analyzer/pull/21314)
* [añadir BreakExpr completación sugerir](https://github.com/rust-lang/rust-analyzer/pull/20521)
* [añadir una extensión LSP para obtener obligaciones fallidas para una función dada](https://github.com/rust-lang/rust-analyzer/pull/21309)
* [añadir varname por defecto para la finalización del postfijo TryEnum](https://github.com/rust-lang/rust-analyzer/pull/21212)
* [añadir brackets, adivina doctor 'T! []' por 'T_'](https://github.com/rust-lang/rust-analyzer/pull/20439)
* [añadir ayuda de ayuda: 'add_explicit_method_call_deref'](https://github.com/rust-lang/rust-analyzer/pull/20996)
* [referencia completa '&T' → '&&T'](https://github.com/rust-lang/rust-analyzer/pull/21289)
* [introduce el campo 'crate_attrs' en 'rust-project.json'](https://github.com/rust-lang/rust-analyzer/pull/21282)
* [atributos de impresión bonita hasta 'cfg(false)'](https://github.com/rust-lang/rust-analyzer/pull/21298)
* [corrección aplicable en no desnudo si para 'move_guard' asistencia](https://github.com/rust-lang/rust-analyzer/pull/21293)
* [corrigir suposición renombrada como corchetes de macro](https://github.com/rust-lang/rust-analyzer/pull/20438)
* [corregir sangría para 'convert_iter_for_each_to_for'](https://github.com/rust-lang/rust-analyzer/pull/20595)
* [corregir sangría para 'merge_nested_if'](https://github.com/rust-lang/rust-analyzer/pull/20577)
* [arreglar match arm anidado cuerpo inválido tipo esperado](https://github.com/rust-lang/rust-analyzer/pull/21291)
* [arreglar if-let anidado para 'merge_nested_if'](https://github.com/rust-lang/rust-analyzer/pull/20576)
* [arreglar generaciones de flycheck que no se sincronizan para múltiples espacios de trabajo](https://github.com/rust-lang/rust-analyzer/pull/21326)
* [más mejoras en las interpretaciones, posibles tras internamientos sin Salsa](https://github.com/rust-lang/rust-analyzer/pull/21307)
* [tipos de solucionadores no internos en Salsa - con GC para ellos](https://github.com/rust-lang/rust-analyzer/pull/21295)
* [eliminar consejos contradictorios](https://github.com/rust-lang/rust-analyzer/pull/20472)
* [Apoyo undoted-self para 'este' cierre de Param](https://github.com/rust-lang/rust-analyzer/pull/21166)

### Triaje de rendimiento del compilador Rust

Semana muy tranquila, prácticamente sin cambios en el rendimiento.

Triaje hecho por **@simulacrum**.
Rango de revisión: [21ff67df.. e1212ea7](https://perf.rust-lang.org/?start=21ff67df15329dd7548ccba54b6c6ae9a562124f&end=e1212ea79b38d51954625291c04d2797c4bb8ec5&absolute=false&stat=instructions%3Au)

1 regresión, 1 mejora, 3 mixta; 2 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-12-22.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?
* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Exportar explícitamente macros de núcleo y estándar](https://github.com/rust-lang/rust/pull/139493)
* [Estabilizar 29 características objetivo RISC-V ('riscv_ratified_v2')](https://github.com/rust-lang/rust/pull/145948)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat(toml): TOML 1.1 soporte de análisis sintáctico](https://github.com/rust-lang/cargo/pull/16415)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Permitir combinar '--ayuda -C ayuda -Z ayuda -W ayuda' en una sola invocación](https://github.com/rust-lang/compiler-team/issues/954)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Subir el límite de subvenciones de viaje a 100.000 dólares para 2026](https://github.com/rust-lang/leadership-council/pull/254)
* [Programa de gestión de fondos para 2026](https://github.com/rust-lang/leadership-council/issues/255)
* [Subir la suba automática de viaje a 2000 dólares](https://github.com/rust-lang/leadership-council/pull/257)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Punteros delgados con metadatos en línea](https://github.com/rust-lang/rfcs/pull/3898)

## Próximos eventos

Eventos Oxidados entre el 24-12-2025 - 21-01-2026 🦀

### Virtual
* 30-12-2025 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Contribución al proyecto Live Open Source Rust**](https://www.meetup.com/code-mavens/events/312554028/)
* 2026-01-03 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 07-01-2026 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/xov10pef)
* 2026-01-15 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-20 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)

### Asia
* 07-01-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**Rust en persona enero de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europa
* 07-01-2026 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 07-01-2026 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-08 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por confirmar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | París, FR | [París Rust](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #82**](https://www.meetup.com/rust-paris/events/312364675/)

### Norteamérica
* 2025-12-27 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Lechmere, 27 de diciembre**](https://www.meetup.com/bostonrust/events/312483556/)
* 2026-01-03 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Allston Rust, 3 de enero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-01-08 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-10 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Central Cambridge Rust, 10 de enero**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-15 | Seattle, WA, EE. UU. | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust Común de Boston, 17 de enero**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-20 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/312185794/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> deberían simplemente renombrar 'inseguro' a 'C' para que la gente se calle

– [/u/thisismyfavoritename en /r/rust](https://www.reddit.com/r/rust/comments/1pp3y9e/comment/nukdfn4/)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1739) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1pv24hh/this_week_in_rust_631/)</small>
