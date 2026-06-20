---
title: "Esta semana en Rust #114"
number_of_week: 114
description: El crate de esta semana es marser, una biblioteca de combinadores de analizadores sintácticos con un giro.
date: 2026-06-17
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
### Actualizaciones de proyectos/herramientas
* [cuTile Rust - Concurrencia intrépida en la GPU, kernels de GPU seguros en memoria y libres de carreras de datos, benchmarks B200](https://arxiv.org/abs/2606.15991)
* [Iroh 1.0 - Teclas de marca, no IPs](https://www.iroh.computer/blog/v1)
* [Diplomat - FFI multilingüe para bibliotecas de Rust](https://manishearth.github.io/blog/2026/06/14/diplomat-multi-language-ffi-for-rust-libraries/)
* [Construí EVM desde cero. Otra vez.](https://sergey-melnychuk.github.io/2026/05/23/yevm/)
* [ProcessKit 1.0 - Gestión de árboles de procesos asíncronos](https://zelanton.github.io/processkit/)
* [litchee: cliente API de Rust Lichess](https://github.com/obazin/litchee/releases/tag/v0.1.0)
* [Cuenca - Optimización numérica en Rust](https://jolars.co/blog/2026-06-10-basin/)
* [Carboxyl 0.1.0-rc - Un navegador servo para el terminal](https://github.com/carboxyl-rs/carboxyl/releases/tag/v0.1.0-servo-rc.1)
* [kache 0.6.0 - una caché de compilación Rust + C/C++ compartida](https://github.com/kunobi-ninja/kache/releases/tag/v0.6.0)
* [numax v0.1.0 - primera versión estable del runtime distribuido WASM de Numax](https://github.com/GianIac/numax/releases/tag/v0.1.0)
* [ZamSync - motor Rust sync desconectado](https://dev.to/etoile_bleu/-i-built-a-sync-engine-for-clinics-that-run-on-2g-and-lose-power-mid-transfer-here-is-why-and-18od)
* [Ktav - un formato de configuración sin comillas](https://dev.to/phpcraftdream/ktav-i-got-fed-up-with-every-config-format-so-i-built-one-with-no-quotes-no-commas-no-54an)

### Observaciones/Pensamientos
* [zlib-rs en Firefox](https://trifectatech.org/blog/zlib-rs-in-firefox/)
* [Oxidación impide carreras de datos, no condiciones de carrera](https://corrode.dev/blog/rust-prevents-data-races-not-race-conditions/)
* [Construye tu proyecto al estilo Zig](https://fnordig.de/2026/06/16/build-your-project-zig-style/)
* [Cómo difieren los CVEs de seguridad de memoria entre Rust y C/C++](https://kobzol.github.io/rust/2026/06/15/how-memory-safety-cves-differ-between-rust-and-c-cpp.html)
* [Por qué no está en crates.io](https://kerkour.com/stdx-cratesio)
* [vídeos] [RustWeek 2026 por RustNL, lista de reproducción de charlas](https://www.youtube.com/watch?v=PrfMpCaIh0k&list=PL8Q1w7Ff68DBpmF38rcIAf8Z9Gj2TnlgM)
* [El iPad estaba en Tailscale](https://www.p2claw.com/blog/2026-06-09-the-ipad-was-on-tailscale/)

### Guías de Rust
* [Aprende la concurrencia de Rust construyendo un pool de hilos](https://blog.sheerluck.dev/posts/learn-rust-concurrency-by-building-a-thread-pool/)
* [Hay vida antes que el principal en Rust](https://grack.com/blog/2026/06/11/life-before-main/)
* [Locales de tareas asincrónicas desde cero](https://wolfgirl.dev/blog/2026-06-16-async-task-locals-from-scratch/)
* [Fearless Embedded Rust: Conduciendo un coche de Lego con una Pico W](https://dystroy.org/blog/picomobile/)
* [Construcción de una capa LLM independiente del proveedor en Rust con Rig](https://smista.ai/blog/how-we-built-a-provider-agnostic-llm-layer-in-rust-with-rig)

### Miscelánea
* [vídeo] [grabaciones de charla de RustWeek 2026](https://2026.rustweek.org/blog/2026-06-10-rustweek-recordings-published/)

## Crate de la semana

El crate de esta semana es [marser](https://github.com/ArneCode/marser), una biblioteca de combinadores de analizadores sintácticos con un giro.

¡Gracias a [Arne Code](https://users.rust-lang.org/t/crate-of-the-week/2704/1611) por la autosugerencia!

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
* [solana-infra-doctor - Lista de códigos de salida en 'sol-doctor --help'](https://github.com/satyakwok/solana-infra-doctor/issues/77)
* [solana-infra-doctor - Hacer que el error de URL inválida sugiera el esquema esperado](https://github.com/satyakwok/solana-infra-doctor/issues/78)
* [solana-infra-doctor - Añadir un glosario de términos de preparación RPC](https://github.com/satyakwok/solana-infra-doctor/issues/79)
* [openslate - añadir pruebas unitarias para slugify() en api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/38)
* [openslate - añadir pruebas de integración para Notes CRUD en API/src/Notes.rs](https://github.com/MrSheerluck/openslate/issues/70)
* [OpenSlate - Añadir pruebas de integración para el flujo de autenticación en API/SRC/USERS.RS](https://github.com/MrSheerluck/openslate/issues/96)
* [OpenSlate - Añadir pruebas unitarias para build_fts_query() en API/SRC/search.rs](https://github.com/MrSheerluck/openslate/issues/89)
* [OpenSlate - Añadir pruebas de integración para middleware de autenticación y cerrar sesión en API/SRC/Auth.rs](https://github.com/MrSheerluck/openslate/issues/106)
* [OpenSlate - Añadir pruebas de integración para puntos finales multimedia (capa de base de datos) en API/SRC/media.rs](https://github.com/MrSheerluck/openslate/issues/85)
* [openslate - añadir pruebas unitarias para ext_from_mime() y filename_from_url() en API/SRC/media.rs](https://github.com/MrSheerluck/openslate/issues/40)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

527 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-09..2026-06-16

#### Compilador
* ['obligations_for_self_ty': saltar objetivos irrelevantes (recalcular 'sub_root' desde 'stalled_vars)'](https://github.com/rust-lang/rust/pull/156187)
* ['codegen_ssa': Pelar envoltorios de trans. en VECs escalables](https://github.com/rust-lang/rust/pull/157768)
* [añadir una comprobación de predicados imposibles a 'trivial_const'](https://github.com/rust-lang/rust/pull/156934)
* [añadir bucle inestable desenrollando atributos de pista](https://github.com/rust-lang/rust/pull/156816)
* [mejorar la polimorfización del formato de punteros en bruto](https://github.com/rust-lang/rust/pull/157714)
* [introduce '#[diagnostic::on_type_error(mensaje)]'](https://github.com/rust-lang/rust/pull/155200)
* [PERF: reutilizar el recorrido de borde de marcado verde al promocionar un nodo](https://github.com/rust-lang/rust/pull/157781)

#### Biblioteca
* [añadir variantes 'or_try_*' para las APIs de entrada 'HashMap' y 'BTreeMap'](https://github.com/rust-lang/rust/pull/157355)
* [hacer que 'BorrowedBuf' y 'BorrowedCursor' sean genéricos sobre los datos](https://github.com/rust-lang/rust/pull/149749)
* [sustituye la tabla de imprimibles por tablas 'unicode_data.rs'](https://github.com/rust-lang/rust/pull/155527)
* [estabilizar '#! [feature(box_as_ptr)]'](https://github.com/rust-lang/rust/pull/157876)
* [estabilizar 'núcleo::rango::{legacy, RangeFull, RangeTo}'](https://github.com/rust-lang/rust/pull/156629)
* [función de estabilizar 'int_format_into'](https://github.com/rust-lang/rust/pull/152544)
* [estabilizar 'nonzero_from_str_radix'](https://github.com/rust-lang/rust/pull/157877)
* [función de estabilización 'float_algebraic'](https://github.com/rust-lang/rust/pull/157029)

#### Carga
* ['trim-paths': emitir 'CARGO_TRIM_PATHS_REMAP' para build.rs](https://github.com/rust-lang/cargo/pull/17104)
* ['diag': Da a los diagnósticos el mismo comportamiento de ruta de visualización que rustc](https://github.com/rust-lang/cargo/pull/17101)
* ['diag': Informar de todos los errores, en orden](https://github.com/rust-lang/cargo/pull/17095)
* ['publicar': evitar un bloqueo falso cuando 'to_confirm' no está vacío](https://github.com/rust-lang/cargo/pull/17071)
* ['resolver': mover política de retiro a capa de resolver](https://github.com/rust-lang/cargo/pull/17083)

#### Rustdoc
* [también pasa pelusa 'unused_doc_comments'](https://github.com/rust-lang/rust/pull/141000)
* [limpieza y (micro)optimización 'print_where_clause'](https://github.com/rust-lang/rust/pull/157874)
* [doctest span correcto para punto y coma final después del ítem](https://github.com/rust-lang/rust/pull/157740)
* [no despojar los objetos ocultos en 'AliasedNonLocalStripper'](https://github.com/rust-lang/rust/pull/157838)
* [un poco más de formato perezoso](https://github.com/rust-lang/rust/pull/157796)

#### Rustfmt
* [añadir 'doc_comment_code_block_small_heuristics', para anular 'use_small_heuristics' en el código de documento](https://github.com/rust-lang/rustfmt/pull/6616)
* [estabilizar 'hex_literal_case'](https://github.com/rust-lang/rustfmt/pull/6935)

#### Clippy
* [nueva pelusa de 'by_ref_peekable_peek'](https://github.com/rust-lang/rust-clippy/pull/17042)
* [añadir pelusa de 'with_capacity_zero'](https://github.com/rust-lang/rust-clippy/pull/17192)
* ['mem_replace_with_default': también emiten macros internos](https://github.com/rust-lang/rust-clippy/pull/17191)
* ['infallible_destructuring_match': limpieza, separa la sugerencia del mensaje principal](https://github.com/rust-lang/rust-clippy/pull/17175)
* ['manual_is_variant_and': pelusa 'resultado.ok().is_some_and(f)'](https://github.com/rust-lang/rust-clippy/pull/17184)
* ['needless_borrow': métodos del mismo nombre falsos positivos](https://github.com/rust-lang/rust-clippy/pull/17171)
* ['unnecessary_lazy_evaluations': cierre de manilla '->'](https://github.com/rust-lang/rust-clippy/pull/17216)
* [menospreciar la pelusa 'from_iter_instead_of_collect'](https://github.com/rust-lang/rust-clippy/pull/17208)
* [eliminar 'is_integer_const'](https://github.com/rust-lang/rust-clippy/pull/17204)
* [no activar 'ref_patterns' lint en código derivado automáticamente](https://github.com/rust-lang/rust-clippy/pull/17250)
* [bucle de mejora nunca](https://github.com/rust-lang/rust-clippy/pull/17145)
* [añadir configuración específica de perfil para métodos y tipos no permitidos](https://github.com/rust-lang/rust-clippy/pull/15779)
* [corrige 'collapsible_match' sugiere erróneamente cuando el cuerpo del partido no lleva brackets](https://github.com/rust-lang/rust-clippy/pull/16749)
* [corregir la sugerencia inversa 'unnecessary_sort_by' usando el nombre incorrecto del parámetro de cierre](https://github.com/rust-lang/rust-clippy/pull/16868)
* [corregir llamada de cierre redundante falsa positiva asíncrona](https://github.com/rust-lang/rust-clippy/pull/17107)
* [INTERPRETACIÓN: Marca 'is_in_test' por última vez en 'incompatible_msrv'](https://github.com/rust-lang/rust-clippy/pull/17218)
* [perf: comprueba el tipo de token antes de extraer la fuente en lints literales tempranos](https://github.com/rust-lang/rust-clippy/pull/17219)
* [perf: coincidir la forma de expresión antes de la comprobación MSRV en 'cloned_ref_to_slice_refs'](https://github.com/rust-lang/rust-clippy/pull/17220)
* [PERF: saltarse la recogida de texto de 'doc_markdown' y escanear palabras cuando se permite la pelusa](https://github.com/rust-lang/rust-clippy/pull/17217)
* [PERF: Salta el módulo de 'single_component_path_imports' cuando no hay nada que lutter](https://github.com/rust-lang/rust-clippy/pull/17225)

#### Analizador de Rust
* [crear directorio para 'cargo xtask metrics rustc_tests'](https://github.com/rust-lang/rust-analyzer/pull/22562)
* [no cuenten C-variádicos '...' como parámetro para punteros fn](https://github.com/rust-lang/rust-analyzer/pull/22575)
* [soporte flyimport excluir variantes](https://github.com/rust-lang/rust-analyzer/pull/22549)
* [arreglar tareas de desestructuración sin introducir movimientos](https://github.com/rust-lang/rust-analyzer/pull/22566)
* [macro de oferta en línea en macro de llamada y proc macro](https://github.com/rust-lang/rust-analyzer/pull/22584)
* [prefiere el comando de banco cuando el objetivo está en banco para evitar el paso de carga](https://github.com/rust-lang/rust-analyzer/pull/22591)
* [soporta variable en línea en macro](https://github.com/rust-lang/rust-analyzer/pull/22551)
* [usar el id del paquete como argumento para '--package' si el package no es único](https://github.com/rust-lang/rust-analyzer/pull/22574)
* [asistir al trabajo de 'inline_type_alias' en definiciones de TDA](https://github.com/rust-lang/rust-analyzer/pull/22545)
* [perf: posponer la revisión inicial del espacio de trabajo hasta que se complete el primado de caché](https://github.com/rust-lang/rust-analyzer/pull/22579)
* [eliminar documentos sobre eliminar el comando 'analysis-bench'](https://github.com/rust-lang/rust-analyzer/pull/22561)
* [eliminar banderas de características innecesarias de las pruebas](https://github.com/rust-lang/rust-analyzer/pull/22571)
* [usar ASCII minúscula para comprobar extensiones de dilibación](https://github.com/rust-lang/rust-analyzer/pull/22585)

### Triaje de rendimiento del compilador Rust

Esta semana hemos tenido bastantes cambios, algunas pequeñas regresiones que fueron un poco difíciles de diagnosticar, pero la semana ha sido mayormente positiva, en general.
Cabe destacar que obtuvimos una mejora enorme en el benchmark next-solver en #[156187](https://github.com/rust-lang/rust/pull/156187),
y una buena aceleración para incremental en [#157781](https://github.com/rust-lang/rust/pull/157781).

Triaje hecho por **@panstromek**.
Rango de revisión: [f3ef3bd8.. b5d46ecb](https://perf.rust-lang.org/?start=f3ef3bd882dd24a275a60701a67c3bb330edd8c1&end=b5d46ecb51c3e4134b82570cfe718f093daa6390&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,2%, 0,6%] | 22 |
| Regresiones ❌ <br /> (secundario) | 0,5% | [0,1%, 2,0%] | 40 |
| Mejoras ✅ <br /> (primaria) | -1,8% | [-5,9%, -0,1%] | 125 |
| Mejoras ✅ <br /> (secundario) | -3,8% | [-69,4%, -0,1%] | 90 |
| Todos ❌✅ (primario) | -1,5% | [-5,9%, 0,6%] | 147 |

1 regresión, 4 mejoras, 8 mixtas; 5 de ellos en rollups
28 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d36b1ad8679b65efbb98252fbb93f72a7d90d4c6/triage/2026/2026-06-16.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Corregir resolución del método de rasgos en un tipo nunca ajustado](https://github.com/rust-lang/rust/pull/156047)
* [Problema de seguimiento para atomic_from_mut](https://github.com/rust-lang/rust/issues/76314)
* [estabilizar nunca escribe](https://github.com/rust-lang/rust/pull/155499)
* [Pelusa contra funciones iteradoras que entran en pánico cuando N es cero](https://github.com/rust-lang/rust/pull/153563)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Soporte de contador de un solo byte en la instrumentación de cobertura](https://github.com/rust-lang/compiler-team/issues/1002)
* [Renombrar los archivos del compilador que contienen diagnósticos de estructuras como 'diagnostics.rs'](https://github.com/rust-lang/compiler-team/issues/1003)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Delegar subvenciones de proyectos al equipo de financiación](https://github.com/rust-lang/leadership-council/issues/301)
* [Asignar presupuesto al equipo de financiación](https://github.com/rust-lang/leadership-council/issues/304)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Parámetros de rasgos Fn nombrados](https://github.com/rust-lang/rfcs/pull/3955)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Las estructuras sin campos o todos campos ZST son ZSTs](https://github.com/rust-lang/reference/pull/2262)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 17-06-2026 y el 15-07-2026 🦀

### Virtual
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ekws5nr4)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: ¿Qué demonios son las mónadas - y cómo las falsificamos en Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Aprendiendo desarrollo de juegos por las malas con Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-05 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
    
### Europa
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla en Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-18 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends llega a Glasgow! (café de día)**](https://www.meetup.com/rust-and-friends/events/315093492/)
* 2026-06-18 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends llega a Glasgow! (pub de la tarde)**](https://www.meetup.com/rust-and-friends/events/315093500/)
* 2026-06-18 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**21º encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/315094938/)
* 2026-06-19 | Dresde, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Segundo encuentro**](https://pretix.eu/rust-dresden/on-location-2)
* 2026-06-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-23 | Varsovia, PL | [Varsovia Oxidada](https://luma.com/rust.in.warsaw)
    * [**Rust Warsaw Meetup: junio 2026**](https://luma.com/djs7ntfx)
* 2026-06-24 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-06-25 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-06-25 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Reunión de Rust #69**](https://www.meetup.com/copenhagen-rust-community/events/315214426/)
* 2026-07-02 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Bevy, Bits, & Cats (Charlas de Rust July)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, OV, NL | [Reuniones de Tecnología de Baseflow](https://www.meetup.com/dutch-rust-meetup/events/)
    * [**Cumbre IA**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Únete a nosotros en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)

### Norteamérica
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Encuentro Comunitario**](https://www.meetup.com/music-city-rust-developers/events/315213927/)
* 2026-06-20 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust del Noreste, 20 de junio**](https://www.meetup.com/bostonrust/events/315225854/)
* 2026-06-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Lugar de Adiós**](https://www.meetup.com/rust-atx/events/315105633/)
* 2026-06-24 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Solucionadores de restricciones basados en Rust en bocetos 2D con Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Gran Fiesta de Verano de Rust NYC**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-06-27 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Union Square en Somerville, 27 de junio**](https://www.meetup.com/bostonrust/events/315225857/)
* 2026-07-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**¿Git es fácil?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Rust de la Universidad de Boston, 4 de julio**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Encuentro de Utah Rust July**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, 11 de julio**](https://www.meetup.com/bostonrust/events/315225865/)

### Oceanía
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne junio 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### Sudamérica
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> "El tipo nunca se nombra según la fecha de su estabilización" fue un buen chiste mientras duró.

– [Sergey "Shnatsel" Davidoff en /r/rust](https://www.reddit.com/r/rust/comments/1u1v53c/the_never_type_is_likely_to_stabilize_soon/oqss8ii/)

¡Gracias a [Dos Moonen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1780) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1u8wwfp/this_week_in_rust_656/)</small>
