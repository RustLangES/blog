---
title: "Esta semana en Rust #107"
number_of_week: 107
description: El crate de esta semana es farben, una caja macro de nombre alemán para los colores terminales. 
date: 2026-04-22
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

<!-- Queridos colaboradores de la comunidad: Por favor, leed README.md para orientarse sobre las aportaciones. Cada enlace enviado debe ser de la forma: * [Título de la página enlazada](https://example.com/my_article) Si añades un enlace a un contenido no textual, por favor prefijadlo con '[vídeo]' o '[audio]': * [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article) * [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todas formas y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [crates.io: Ayuda a probar nuestro nuevo frontend web](https://blog.rust-lang.org/inside-rust/2026/04/17/crates-io-svelte-public-testing/)
* [Anunciando Rust 1.95.0 | Blog Rust](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/)

### Fundación
* ¡RustConf 2026 [horario](https://rustconf.com/schedule/) y [inscripción](https://rustconf.com/register) están en activo! Los precios de las entradas para anticipación están disponibles hasta el 29 de abril. 

### Actualizaciones de proyectos/herramientas
* [axum-harness: plantilla de arquitectura backend nativa de agentes para Axum — semántica-primero, topología-tardía, multi-agente harness](https://github.com/openclosed-org/axum-harness/blob/main/docs/launch.md)
* [decimal lean: 2~6 veces más rápido que 'rust_decimal'](https://github.com/WuBingzheng/lean-decimal/blob/main/benches/README.md)
* [Construcción de control de versiones semántica en Rust](https://therohansharma.com/semantic-version-control-rust)
* [Oxanus v1.0 - Biblioteca de procesamiento de trabajos](https://github.com/pragmaplatform/oxanus/releases/tag/v1.0)
* [flodl 0.5.2: HuggingFace, en Rust](https://flodl.dev/blog/huggingface)
* [El rasgo de tamaño único no sirve para todos](https://lwn.net/SubscriberLink/1067220/f4b7acbc7ce7d1fa/)
* [tinyboot v0.4.0 Publicado — La API es estable](https://aaronqian.com/log/2026-04-22-tinyboot-v040-released/)
* [Slint 1.16 Lanzado](https://slint.dev/blog/slint-1.16-released)
* [Mensajería del Danubio añade suscripciones de Key-Share](https://danube-docs.dev-state.com/architecture/key_shared_architecture/)
* [Anunciando montaje mtp: montaje fusible puro de Rust para dispositivos MTP](https://www.veszelovszki.com/a/mtp-mount/)
* [wrkflw v0.8.0 - Validar y ejecutar acciones de GitHub localmente.](https://github.com/bahdotsh/wrkflw/releases/tag/v0.8.0)

### Observaciones/Pensamientos
* [Respuestas Criptográficas Correctas: Edición Post Quantum and Rust](https://kerkour.com/post-quantum-cryptography-recommendations-rust)
* [Aprendiendo Rust a través de un LLM para desarrollar un lector RSS TUI (y lo que cuento a mis estudiantes)](https://github.com/christo-auer/eilmeldung/blob/main/docs/llm-development.md)
* [Qué ocurre cuando construyes un vector estilo inodo en Rust](https://sot.dev/inode-style-vector-in-rust.html)
* [Propiedad y Préstamo
frente a Conteo de Referencias](https://slicker.me/rust/ownership_and_borrowing_vs_reference_counting.html)
* [El borde del Rust seguro](https://kyju.org/blog/tokioconf-2026/)
* [vídeo] [Tercer Func Prog Online Suecia 2026](https://www.youtube.com/watch?v=fboHzVVfknU&t=340s)

### Guías de Rust
* [vídeo] [Construye una aplicación web Full Stack Clon de Twitter en Rust (Axum & Leptos)](https://www.youtube.com/watch?v=WmGv-lZgr7M)
* [La Guía del Programador Impaciente para Bevy y Rust: Capítulo 12 - Que haya Networking](https://aibodh.com/posts/bevy-rust-game-development-chapter-12/)
* [vídeo] [RustCurious lección 6: Enums y polimorfismo](https://www.youtube.com/watch?v=7dXQLr014JU)
* [Un VMM mínimo en Rust con Apple Hypervisor](https://gigapotential.dev/blog/minimal-vmm-in-rust-with-apple-hypervisor/)

* [Almacenamiento en caché de funciones caras en Rust con 'caché'](https://kocharhook.com/post/5/caching-expensive-functions-in-rust/)

## Crate de la semana
El crate de esta semana es [farben](https://github.com/razkar-studio/farben), una caja macro de nombre alemán para los colores terminales. 

¡Gracias a [Nik Revenco](https://users.rust-lang.org/t/crate-of-the-week/2704/1597) por la sugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

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
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->

* [rust-cookbook - Añadir sección asíncrona con recetas de ejecución de tokio](https://github.com/rust-lang-nursery/rust-cookbook/issues/759) ([otros ejemplos de alto impacto](https://github.com/rust-lang-nursery/rust-cookbook/issues?q=is%3Aissue%20state%3Aopen%20label%3Aexample))
* [wacp-platform - Corregir los clippy drifts solo de prueba en 'wacp-runtime/tests.rs' + 'console-db/queries/tests.rs'](https://github.com/AAkil98/wacp-platform/issues/2) ([otros buenos primeros números](https://github.com/AAkil98/wacp-platform/labels/good%20first%20issue))

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | 2026-04-27 | Barcelona, España | 2026-10-14 - 2026-10-17
* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | 2026-05-03 | Kongsberg, Noruega | 2026-09-21 al 23. 

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

542 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-14..2026-04-21

#### Compilador
* [no hash 'DelayedLints'](https://github.com/rust-lang/rust/pull/155248)
* [refactorizar los campos no tipo FnDecl y FnSig en un nuevo tipo de envoltorio](https://github.com/rust-lang/rust/pull/155223)
* [sugiero eliminar 'y' cuando se espera una referencia a un futuro](https://github.com/rust-lang/rust/pull/154933)
* [sugiere devolver una referencia para un lugar no dimensionado desde un cierre](https://github.com/rust-lang/rust/pull/152162)

#### Biblioteca
* [abortar en el núcleo](https://github.com/rust-lang/rust/pull/154604)
* [constificar 'Index'('Mut'), 'Deref'('Mut') por 'Vec'](https://github.com/rust-lang/rust/pull/155054)
* [núcleo/num: implementar la característica 'integer_cast_extras'](https://github.com/rust-lang/rust/pull/154664)
* ['core::unicode': Sustituye la tabla 'Cased' por 'Lt'](https://github.com/rust-lang/rust/pull/154699)
* [libtest: usar búsqueda binaria para --filtro de prueba exacto](https://github.com/rust-lang/rust/pull/154865)
* [mover 'std::io::ErrorKind' a 'core::io'](https://github.com/rust-lang/rust/pull/154654)

#### Rustdoc
* [corregir 'redundant_explicit_links' disparando incorrectamente (o sin disparar) bajo ciertos escenarios](https://github.com/rust-lang/rust/pull/155435)
* [preservar 'doc(cfg)' en alias de tipo reexportados localmente](https://github.com/rust-lang/rust/pull/154970)

#### Clippy
* [añadir comprobación MSRV para 'manual_noop_waker'](https://github.com/rust-lang/rust-clippy/pull/16850)
* [añadir pelusa de 'useless_borrows_in_formatting'](https://github.com/rust-lang/rust-clippy/pull/16523)
* [no proponen refactorizar cuando no se utiliza ningún constructor variante](https://github.com/rust-lang/rust-clippy/pull/16867)
* [no actives 'let_and_return' en 'deja que si no'](https://github.com/rust-lang/rust-clippy/pull/16829)
* [extender 'byte_char_slices' para cubrir matrices](https://github.com/rust-lang/rust-clippy/pull/16770)
* [extender la pelusa 'zst_offset' para detectar <T>cálculos de desplazamiento 'NonNull'](https://github.com/rust-lang/rust-clippy/pull/16888)
* [arreglar un caso donde 'collapsible_match' sugirió una transformación que cambia el comportamiento en tiempo de ejecución](https://github.com/rust-lang/rust-clippy/pull/16878)
* [corregir 'cloned_ref_to_slice_refs' falso negativo en 'to_owned()'](https://github.com/rust-lang/rust-clippy/pull/16329)
* [corrige 'expect_fun_call' sugiere erróneamente para cortar cuerdas](https://github.com/rust-lang/rust-clippy/pull/16752)
* [corregir 'for_kv_map' falso negativo al usar 'iter' y 'iter_mut'](https://github.com/rust-lang/rust-clippy/pull/16830)
* [paréntesis 'AssocOp::Cast' en sugerencia cuando el operador de reemplazo está '<' para evitar errores de análisis sintáctico](https://github.com/rust-lang/rust-clippy/pull/16848)
* ['useless_conversion': no poner pelusa '(a.. b).into_iter()' (para migración de la edición)](https://github.com/rust-lang/rust-clippy/pull/16891)

#### Analizador de Rust
* ['completación': reducir relevancia para elementos obsoletos](https://github.com/rust-lang/rust-analyzer/pull/22085)
* [eliminar lints duplicados](https://github.com/rust-lang/rust-analyzer/pull/22054)
* [permiten a los autores de cajas declarar que su rasgo prefiere ser importado 'como _'](https://github.com/rust-lang/rust-analyzer/pull/21740)
* [no completar elementos inestables que usan una función interna](https://github.com/rust-lang/rust-analyzer/pull/22044)
* [excluir referencias(encuentra todas las referencias) de deps y stdlib](https://github.com/rust-lang/rust-analyzer/pull/21906)
* [soporte para extraer variable en llamada de macro](https://github.com/rust-lang/rust-analyzer/pull/21487)
* [añadir paréntesis en el registro expr para 'replace_let_with_if_let'](https://github.com/rust-lang/rust-analyzer/pull/22067)
* [ajustar el nombre de 'extract_type_alias'](https://github.com/rust-lang/rust-analyzer/pull/22070)
* [permitir ambigüedad en la abreviatura de tipo asociado si resuelven al mismo tipo asociado, entre superrasgos esta vez](https://github.com/rust-lang/rust-analyzer/pull/22032)
* [comprobación de tipo de puerto y inferencia de cierre de UPVAR de llamada a puerto desde RUSTC](https://github.com/rust-lang/rust-analyzer/pull/22101)
* [respeta '#[desprecado]' al decidir si una finalización de 'ModuleDef' está 'obsoleta'](https://github.com/rust-lang/rust-analyzer/pull/22083)
* [algunas correcciones para 'upvars_mentioned()'](https://github.com/rust-lang/rust-analyzer/pull/22055)
* [usar 'ProofTreeVisitor' para coacción sin tamaño](https://github.com/rust-lang/rust-analyzer/pull/22096)
* [analizar elementos de 'tipo const'](https://github.com/rust-lang/rust-analyzer/pull/22046)
* [perf: no comprobar la validez de la caché del solver en cada acceso](https://github.com/rust-lang/rust-analyzer/pull/22104)
* [Sync function call args check fadging with rustc](https://github.com/rust-lang/rust-analyzer/pull/22092)

### Triaje de rendimiento del compilador Rust

Esta semana ha sido un poco desordenada, pero las mayores regresiones han sido o bien
Ya se ha arreglado o están siendo investigados. También hubo un par de buenas prestaciones. Victorias. 

Triaje hecho por **@Kobzol**. 
Rango de revisión: [dab8d9d1.. 9ab01ae5](https://perf.rust-lang.org/?start=dab8d9d1066c4c95008163c7babf275106ce3f32&end=9ab01ae53c416f89fe256b79588a76dcbcdc9290&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,7% | [0,2%, 4,6%] | 39 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,2%, 1,4%] | 31 |
| Mejoras ✅ <br /> (primaria) | -0,6% | [-4,8%, -0,1%] | 70 |
| Mejoras ✅ <br /> (secundario) | -0,7% | [-4,1%, -0,0%] | 93 |
| Todos ❌✅ (primario) | -0,1% | [-4,8%, 4,6%] | 109 |

3 regresiones, 4 mejoras, 6 mixtas; 4 de ellas en rollups
41 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/9c2cedf21859ce1404fe1265ab518ca243d1d20b/triage/2026/2026-04-21.md). 

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Error en el especificador de sección macho inválido](https://github.com/rust-lang/rust/pull/155065)
* [Permitir el 'yo' posterior en más contextos](https://github.com/rust-lang/rust/pull/155137)
* [Añadir FCW para prohibir '$crate' en el emparejador de macros](https://github.com/rust-lang/rust/pull/155121)
* [Pelusa de artículos de pub no usados en cajas binarias](https://github.com/rust-lang/rust/pull/149509)
* [const-estabilizar 'char::is_control()'](https://github.com/rust-lang/rust/pull/155528)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Estabilizar el diseño 'build-dir' v2](https://github.com/rust-lang/cargo/pull/16807)
* [dote(compilar): Estabilizar 'build.warnings'](https://github.com/rust-lang/cargo/pull/16796)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Promocionar riscv64gc-unknown-linux-musl a Nivel 2 (con herramientas)](https://github.com/rust-lang/compiler-team/issues/982)
* [Hacer que los nombres de hash estables sean consistentes](https://github.com/rust-lang/compiler-team/issues/983)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),  
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [Añadir política de contribución para trabajo generado por IA](https://github.com/rust-lang/rfcs/pull/3950)
* [Lanzamiento de rasgo acotado](https://github.com/rust-lang/rfcs/pull/3952)
* [Apoyo a bloqueos heterogéneos ('try_blocks_heterogeneous') RFC](https://github.com/rust-lang/rfcs/pull/3953)

## Próximos eventos

Eventos Rusty entre el 22-04-2026-2026 🦀

### Virtual
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Ámsterdam, NL) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-24 | Virtual (Nairobi, KE) | [RustaceansKenya](http://luma.com/RustaceansKenya)
    * [**Transición a Rust: La curva de aprendizaje**](https://luma.com/f4qezpay)
* 2026-04-28 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: De Protobuf a Producción - Guía para gRPC en Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-28 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 4**](https://www.meetup.com/code-mavens/events/314401473/)
* 2026-04-29 | Virtual (Girona, ES) | [Girona Oxidada](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/8hi2xywi)
* 01-05-2026 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Caminata de Hacker 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Encuentro de Rust Circle](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Cardiff, GB) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Introducción práctica a SIMD**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314301861/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/314323890/)
* 2026-05-07 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-12 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-13 | Virtual (Girona, ES) | [Girona Oxidada](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ooub1kt0)
* 2026-05-17 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, EE.UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Virtual (Girona, ES) | [Girona Oxidada](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/548kbqhl)
* 2026-05-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
    
### Asia
* 2026-05-13 | Malasia, MI | [Rust Meetup Malasia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup mayo 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)

### Europa
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla y fiesta de cumpleaños en MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-04-23 | París, FR | [París Oxidado](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #85**](https://www.meetup.com/rust-paris/events/314283634/)
* 2026-04-24 - 2026-04-26 | Augsburgo, DE | [Encuentro Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Semana del Futuro Augsburgo: Camino al Game Jam – Spielend Bevy und Rust lernen bei Tuxedo Computers**](https://www.tuxedocomputers.com/de/Road-to-Game-Jam-2026-Bevy-Workshop.tuxedo)
* 2026-04-25 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #26**](https://www.meetup.com/stockholm-rust/events/314227099/)
* 2026-04-29 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Reunión de Rust #67**](https://www.meetup.com/copenhagen-rust-community/events/314279730/)
* 2026-04-29 | París, FR | [Rustaceos de París](https://www.eventbrite.fr/o/74289178383)
    * [**Rust Meetup en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1984135342220)
* 30-04-2026 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/314292918/)
* 30-04-2026 | Manchester, GB | [Manchester Oxidado](https://www.meetup.com/rust-manchester/events/)
    * [**Charla de abril de Rust Manchester**](https://www.meetup.com/rust-manchester/events/314229892/)
* 2026-05-02 | Augsburgo, DE | [Rust Múnich](https://rust-munich.de/) y [Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Augsburger Linux-Infotag 2026: Gemeinschaftsstand Rust Augsburg und Rust München**](https://www.luga.de/static/LIT-2026/)
* 04-05-2026 | Ámsterdam, NH, NL | [Grupo Rust Developers Amsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/314268909/)
* 04-05-2026 | Frankfurt, DE | [Rhein-Meno Oxidado](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo una simulación de cartera de acciones en Rust con Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)
* 2026-05-05 | Olomouc, CZ | [Moravia Oxidada](https://www.meetup.com/rust-moravia/events/)
    * [**5. Rust Moravia Meetup (¡Ukaž irritable!) **](https://www.meetup.com/rust-moravia/events/314218493/)
* 2026-05-07 | Edimburgo, GB | [Rust y Amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust May habla: Aetherus + TBA**](https://www.meetup.com/rust-and-friends/events/314300802/)
* 2026-05-13 | Girona, ES | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Suiza, CH | [Tras TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 | Milán, MI, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano/events/)
    * [**SemanaOxidación 2026**](https://www.meetup.com/rust-language-milan/events/314329200/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de Hacks**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**anuncio de RustWeek 2026**](https://www.meetup.com/rust-nederland/events/312861992/)
* 2026-05-19 | Leipzig, SN, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Construcción y Prueba Cruzada**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | Londres, Reino Unido | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Comida de la Semana del Rust**](https://www.meetup.com/women-in-rust/events/314313054/)

### Norteamérica
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-22 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-22 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Solucionadores de Rust y SAT Verificados formalmente**](https://www.meetup.com/rust-nyc/events/314167944/)
* 2026-04-22 | Portland, OR | [**Apache DataFusion Meetup**](https://luma.com/dsp3ud82)
    * [**Encuentro de Portland Apache DataFusion**](https://luma.com/dsp3ud82)
* 2026-04-23 | Los Ángeles, CA, EE.UU. [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Oxida LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Estación Sur, 25 de abril**](https://www.meetup.com/bostonrust/events/313883704/)
* 2026-04-28 | Nueva York, NY, EE.UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC x OpenAI: Más seguro 'inseguro' & Barnum: El motor de flujo de trabajo agente.**](https://www.meetup.com/rust-nyc/events/314180711/)
* 30-04-2026 | Atlanta, GA, EE.UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228662/)
* 2026-05-07 | Saint Louis, MO, EE. UU. | [Rust STL](https://www.meetup.com/stl-rust/events/)
    * [**Noche de Proyecto Abierto**](https://www.meetup.com/stl-rust/events/313807225/)
* 2026-05-14 | Portland, OR, EE.UU. [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**De ondas de radio a píxeles - Visualizaciones en tiempo real con Rust y WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, EE.UU. [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust May Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 19-05-2026 | San Francisco, CA, EE.UU. | [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 20-05-2026 | San Francisco, CA, EE. UU. | [Encuentro de Rust en el Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Rust en el Área de la Bahía**](https://luma.com/9j3q5ejl)

### Oceanía
* 2026-05-14 | Melbourne, AU | [Melbourne Oxidado](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne - mayo 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
    
Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién contrata en r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> en Rust pagamos el precio de la composición por adelantado

– [Nadieril sobre zulip de Rust](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/broken.20and.20un-fixable.20parts.20of.20Rust/near/587758938)

¡Gracias a [Nadieril](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1763) por la autosugerencia! 

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1su40pd/this_week_in_rust_648/)</small>