---
title: "Esta semana en Rust #117"
number_of_week: 117
description: El crate de esta semana es apis-saltans, una implementación en Zigbee que incluye una API coordinadora.
date: 2026-07-08
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
* [Destacado del mantenedor: Gen Li (@rami3l)](https://blog.rust-lang.org/inside-rust/2026/07/07/maintainer-spotlight-gen-li-rami3l/)
* [Juntos por un Clippy más saludable](https://blog.rust-lang.org/inside-rust/2026/07/06/unite-for-clippy/)

### Boletines
* [El Rustaceano Incrustado Número #75](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-75)

### Actualizaciones de proyectos/herramientas
* [copper-rs v1.0.0](https://www.copper-robotics.com/whats-new/copper-rs-v100): el sistema operativo de robótica determinista de código abierto es ahora estable.
* [Rayfish: Tu propia red privada. Sin servidores, sin configuración.](https://rayfish.xyz/blog/01-introducing-rayfish)
- [rama v0.3.0 — framework de servicios de red listo para ser utilizado por la comunidad más amplia de Rust](https://plabayo.tech/blog/rama-0-3)
* [kache 0.9.0: endurecimiento de la cadena de suministro + caché CI de solo lectura](https://github.com/kunobi-ninja/kache/releases/tag/v0.9.0)
* [GuardianDB - PostgreSQL y P2P/Local-First juntos](https://www.willsearch.com.br/blog/2026/07/04/meet-guardiandbs-new-postgresql-compatibility-layer/)
* [Nectar: un lenguaje similar a Rust que compila toda tu aplicación web a WebAssembly](https://buildnectar.com/)
* [logdrain: Minería rápida e incrustable de plantillas logarítmicas en Rust](https://thekeeper.io/blog/logdrain-log-template-mining-in-rust/)
* [funda: Empaquetando el vídeo del mundo en Rust puro](https://medium.com/@vbasky/packaging-the-worlds-video-in-pure-rust-ff1f6b884fec)
* [Wickra: Indicadores técnicos de streaming-first](https://docs.wickra.org/Quickstart-Rust)
* [Xcelerator Solver v0.1.0 -- regresión simbólica determinista](https://github.com/TeamXcelerator/xcelerator-solver/releases/tag/v0.1.0)
* [DLT-TUI 1.1.0 - un visor rápido de TUI para archivos DLT automotrices (AUTOSAR Diagnostic Log and Trace)](https://github.com/tkmsikd/dlt-tui/releases/tag/v1.1.0)
* [RSSH v0.2.11 — flujos de trabajo de terminal, importación de claves SSH más segura y operaciones de IA observables](https://github.com/shihuili1218/rssh/releases/tag/v0.2.11)
* [k8s-scale-app-rs: Escalar o reiniciar un despliegue de Kubernetes desde un CronJob](https://blog.none.at/blog/2026/2026-07-06-k8s-scale-app-rs/)
* [Actualización v0.5.0-rc1](https://dev.to/sicklefire/m-vis-v050-rc1-update-11cp)
* [FlareDB: Una base de datos nativa de streaming Apache Beam integrada en Rust](https://ganeshsivakumar.substack.com/p/flaredb)
* [MQTT-typed-client 0.2: un cliente asíncrono MQTT seguro para tipos en rumqttc](https://holovskyi.github.io/blog/typed-mqtt-topics-for-rust/)
* [RootAsRole: v4.0.0 Lanzamiento principal, ejecución segura, nuevo logo](https://github.com/LeChatP/RootAsRole/releases/tag/v4.0.0)
* [Un marco de interfaz de usuario Rust multiplataforma mediante la tecnología puente de Qt](https://www.qt.io/blog/rust-ui-framework-via-bridging-technology)
* [Lenguaje de programación Jam Planning](https://rapha.land/jam-programming-language/)
* [Sōzu 2.1.0: Balanceo de carga UDP para el borde programable](https://www.clever.cloud/blog/company/2026/07/01/sozu-2-1-0-udp-load-balancer-programmable-edge/)
* [b0nker: un tiempo de ejecución mínimo de contenedor escrito en Rust](https://op3kay.dev/writing/b0nker)

### Observaciones/Pensamientos
* [vídeo] [Reunión de Rust Berlin 25/06/2026 Transmisión en directo](https://www.youtube.com/watch?v=SGR5qBdwk30)
* [vídeo] [¿Cómo se reescriben proyectos en C/C++ para Rust? – Entrevista de JetBrains con Luca Palmieri, Mainmatter](https://www.youtube.com/live/_LtgHxuysUo)
* [Investigando por qué RustCrypto es lento: Análisis profundo de las instrucciones SIMD y la aceleración por hardware](https://kerkour.com/rustcrypto-slow-simd-rust)
* [Bool como U32](https://parsa.wtf/cast/)
* [Una cadena de verificación de Rust-to-Lean con pruebas de IA: Un informe de experiencia](https://arxiv.org/html/2605.30106)
* [Trabajo en Progreso Oxidado](https://blog.dureuill.net/articles/wip/)
* [vídeo] [OpenAI acaba de gastar 600.000 dólares en Rust](https://www.youtube.com/watch?v=Fk165jYfHpc)
* [audio] [Rising Adecemies con Dylan Brown - Podcast Rust in Production](https://corrode.dev/podcast/s06e07-rising-academies/)

### Guías de Rust
* [serie] [Tutorial de Bevy: Crea tu primer editor 3D - Crea un espacio 3D en una cuadrícula infinita](https://aibodh.com/posts/bevy-tutorial-build-your-first-3d-editor-in-rust/)
* [Aprende los conceptos básicos y el enrutamiento de Axum construyendo un acortador de URL](https://blog.sheerluck.dev/posts/learn-axum-basics-and-routing-by-building-a-url-shortener/)
* [serie] [Rama 101.1: Clientes HTTPS y capas de abstracción](https://plabayo.tech/blog/rama-101-1-https-clients-and-abstractions)

### Miscelánea
* [Diagrama de Euler clicable de todas las charlas sobre la semana de Rust](https://seanborg.tech/tiny-blog/rust-week-ven-diagram/)

## Crate de la semana

El crate de esta semana es [apis-saltans](https://crates.io/crates/apis-saltans-core), una implementación en Zigbee que incluye una API coordinadora.

¡Gracias a [Richard Neumann](https://users.rust-lang.org/t/crate-of-the-week/2704/1627) por la autosugerencia!

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
* [Protocolo - Extender pruebas de exactitud de bits a objetivos de reconstrucción f64](https://github.com/name970/Protocol/issues/4)                                                                            
* [Dofigen - No hay bandera de reemplazo de etiqueta de imagen para el comando generar](https://github.com/lenra-io/dofigen/issues/278)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

598 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-30..2026-07-07

#### Compilador
* [activar la norma 'param_env' de eager en el nuevo solver](https://github.com/rust-lang/rust/pull/156976)
* [pelusa en 'core::ffi::c_void' como tipo de retorno](https://github.com/rust-lang/rust/pull/156379)
* [pulir algo de código de macro analizable](https://github.com/rust-lang/rust/pull/158577)
* [resolver: no asignar en 'resolve_ident_in(_local)_module_*'](https://github.com/rust-lang/rust/pull/158604)
* [simplificar opción-iterador aplanando en el compilador](https://github.com/rust-lang/rust/pull/158627)
* [estabilizar '#[my_macro] mod foo;' (parte de 'proc_macro_hygiene')](https://github.com/rust-lang/rust/pull/157857)

#### Biblioteca
* [añadir 'std::io::cursor::WriteThroughCursor'](https://github.com/rust-lang/rust/pull/158537)
* [implementa 'Box::as_non_null()'](https://github.com/rust-lang/rust/pull/157347)
* [implementa 'DoubleEndedIterator::next_chunk_back'](https://github.com/rust-lang/rust/pull/156737)
* [implementa 'IntoIterator' para '[&[mut]] Box<[T; N], A>'](https://github.com/rust-lang/rust/pull/134021)
* [implementa 'ptr::{read,write}_unaligned' mediante 'repr(packed)'](https://github.com/rust-lang/rust/pull/158427)
* [mueve 'SizeHint' y 'IoHandle' a 'core::io'](https://github.com/rust-lang/rust/pull/158539)
* [mueve 'std::io::Seek' a 'core::io'](https://github.com/rust-lang/rust/pull/158540)
* [optimizar 'ArrayChunks::try_rfold' con 'DoubleEndedIterator::next_chunk_back'](https://github.com/rust-lang/rust/pull/158704)
* [estabilizar 'característica(atomic_from_mut)'](https://github.com/rust-lang/rust/pull/158573)

#### Carga
* ['bindeps': registros de objetivos de artefacto transitivos](https://github.com/rust-lang/cargo/pull/17135)
* [evitar clonar el manifiesto TOML analizado en 'ManifestErrorContext'](https://github.com/rust-lang/cargo/pull/17167)
* [evitar clon extra del manifiesto TOML analizado](https://github.com/rust-lang/cargo/pull/17176)
* [eliminar clonación innecesaria al analizar el índice de paquetes](https://github.com/rust-lang/cargo/pull/17178)
* [cambiar HashMaps y HashSets en Cargo para usar Fxhasher](https://github.com/rust-lang/cargo/pull/17169)
* [no pases banderas de Rust de pelusa cuando '--cap-lints=permite' está activado](https://github.com/rust-lang/cargo/pull/17174)
* [corregido 'Compilación::d eps_output' tomando solo la última dep](https://github.com/rust-lang/cargo/pull/17164)
* [preasignar algunos vectores](https://github.com/rust-lang/cargo/pull/17177)
* [estabilizar el layout 'build-dir' v2](https://github.com/rust-lang/cargo/pull/16807)
* [usar un conjunto al comprobar los miembros del espacio de trabajo visitados](https://github.com/rust-lang/cargo/pull/17180)

#### Rustdoc
* [corregir el fallo al intentar inlinear un elemento extranjero que no puede tener atributos](https://github.com/rust-lang/rust/pull/158751)
* [mostrar rutas de sitio de uso para longitudes de arreglos const no evaluadas](https://github.com/rust-lang/rust/pull/158334)

#### Clippy
* ['chunks_exact_to_as_chunks': No reportes expresiones con parámetros const](https://github.com/rust-lang/rust-clippy/pull/17319)
* ['chunks_exact_to_as_chunks': No reportes expresiones con parámetros de tipo](https://github.com/rust-lang/rust-clippy/pull/17360)
* ['missing_trait_methods': MSRV/conciencia inestable](https://github.com/rust-lang/rust-clippy/pull/17309)
* ['vec_init_then_push': no empujes pelusa de una expansión macro](https://github.com/rust-lang/rust-clippy/pull/17289)
* ['inline_modules': ignorar módulos 'cfg(test)' en compilaciones de prueba](https://github.com/rust-lang/rust-clippy/pull/17346)
* ['match_same_arms': mantén expectativas a nivel de brazo trabajando por debajo de un permiso externo](https://github.com/rust-lang/rust-clippy/pull/17345)
* ['unnecessary_operation': evitar malas sugerencias '!'](https://github.com/rust-lang/rust-clippy/pull/17341)
* ['unnecessary_unwrap_unchecked': no se active dentro del '_unchecked' fn](https://github.com/rust-lang/rust-clippy/pull/17351)
* [añadir paréntesis obligatorios cuando la sugerencia 'needless_bool' es un operando](https://github.com/rust-lang/rust-clippy/pull/17348)
* [arreglar ICE al resolver local en 'unnecessary_unwrap_unchecked'](https://github.com/rust-lang/rust-clippy/pull/17353)
* [corregir 'infinite_loop' falsos positivos dentro de bloques gens](https://github.com/rust-lang/rust-clippy/pull/17311)
* [sugerencia de corregir 'manual_c_str_literals' cuando se escapa la barra posterior hacia atrás](https://github.com/rust-lang/rust-clippy/pull/17358)
* [corregir 'strlen_on_c_strings' lógica incorrecta de sugerencias](https://github.com/rust-lang/rust-clippy/pull/17337)
* [corregir duplicaciones de 'suspicious_operation_groupings'](https://github.com/rust-lang/rust-clippy/pull/17323)
* [ancho de bit de pelusa](https://github.com/rust-lang/rust-clippy/pull/16902)
* [optimizar llamadas 'Msrv::meets'](https://github.com/rust-lang/rust-clippy/pull/17338)
* [salto de los escaneos de pelusa unicode cuando el fragmento es ASCII puro](https://github.com/rust-lang/rust-clippy/pull/17273)
* [saltarse el recorrido de los padres HIR en 'is_in_test_function' cuando no hay elementos de prueba](https://github.com/rust-lang/rust-clippy/pull/17224)
* [colocar el bloque impl generado después del bloque impl existente](https://github.com/rust-lang/rust-clippy/pull/17366)
* [refactorizar el pase de pelusa 'StringAdd'](https://github.com/rust-lang/rust-clippy/pull/17333)
* [refactorización 'suspicious_xor_used_as_pow'](https://github.com/rust-lang/rust-clippy/pull/17334)
* [eliminar 'lower_ty' de 'uninhabited_reference'](https://github.com/rust-lang/rust-clippy/pull/17293)
* [respetar el MSRV configur manual_is_variant_and ado en 'mapa() == reescritura de algunos(_)](https://github.com/rust-lang/rust-clippy/pull/17328)
* [reescribir 'mut_mut'](https://github.com/rust-lang/rust-clippy/pull/17332)
* [reescribir 'redundant_else' como un pase tardío](https://github.com/rust-lang/rust-clippy/pull/17329)
* [reescritura de 'tuple_array_conversions'](https://github.com/rust-lang/rust-clippy/pull/17354)

#### Analizador de Rust
* [SCIP: excluir trivial de inicio/final en los rangos de definición](https://github.com/rust-lang/rust-analyzer/pull/22595)
* [SCIP: eliminar campo 'inlay_hints' muerto](https://github.com/rust-lang/rust-analyzer/pull/22708)
* ['feat(ide-diagnostics)': añadir diagnósticos para patrones de unión inválidos (E0784)](https://github.com/rust-lang/rust-analyzer/pull/22433)
* ['internal(query-group-macro)': eliminar la prueba de aridad](https://github.com/rust-lang/rust-analyzer/pull/22704)
* [método añadir la parte superior del árbol al nodo Sintaxis](https://github.com/rust-lang/rust-analyzer/pull/22668)
* [añadir controlador para E0627](https://github.com/rust-lang/rust-analyzer/pull/22665)
* [soporta múltiples brazos para 'replace_match_with_if_let'](https://github.com/rust-lang/rust-analyzer/pull/22231)
* [corregir UB en casos de prueba 'smol_str borsh_non_utf8'](https://github.com/rust-lang/rust-analyzer/pull/22690)
* [corregir parámetro genérico para 'generate_default_from_enum_variant'](https://github.com/rust-lang/rust-analyzer/pull/20362)
* [archivo 'walkthrough_create_project' no empaquetado](https://github.com/rust-lang/rust-analyzer/pull/22703)
* [fallo de aserción al cerrar con función no vinculada](https://github.com/rust-lang/rust-analyzer/pull/22677)
* [evitar el pánico en 'convert_tuple_struct_to_named_struct' sobre el uso de patrones anidados](https://github.com/rust-lang/rust-analyzer/pull/22613)
* [sintaxis de configuración para nvim-lsp](https://github.com/rust-lang/rust-analyzer/pull/22649)
* [resolución correcta de valor cuando comparte el mismo nombre que el tipo](https://github.com/rust-lang/rust-analyzer/pull/22706)
* [excluir impls en el tipo de error de la enumeración impl](https://github.com/rust-lang/rust-analyzer/pull/22619)
* [corrección de bloqueo en 'extract_variable' al seleccionar llamada de macro no resuelta](https://github.com/rust-lang/rust-analyzer/pull/22705)
* [corregir el cierre al completar dentro de macros](https://github.com/rust-lang/rust-analyzer/pull/22715)
* [arreglar la gestión de parámetros de corutinas fns](https://github.com/rust-lang/rust-analyzer/pull/22673)
* [manejar más casos de CFGs en la reducción del almacenamiento de expansión](https://github.com/rust-lang/rust-analyzer/pull/22675)
* [no generar con el elemento asociado por defecto](https://github.com/rust-lang/rust-analyzer/pull/22488)
* [entra en pánico en 'unwrap_return_type', 'remove_underscore' y 'promote_local_to_const'](https://github.com/rust-lang/rust-analyzer/pull/22674)
* [colección de segmentos de calificadores de atributos de elevación](https://github.com/rust-lang/rust-analyzer/pull/22711)
* [reducir asignación conjunta de tokens del analizador](https://github.com/rust-lang/rust-analyzer/pull/22709)
* [proyecto-modelo: no pasar args extra de metadatos a sysroot](https://github.com/rust-lang/rust-analyzer/pull/22676)
* [modelo-proyecto: introduce cargo.configPath](https://github.com/rust-lang/rust-analyzer/pull/22679)
* [proporcionar tiempo de arranque para preparar el punto de registro y el benchmark asociado](https://github.com/rust-lang/rust-analyzer/pull/22581)

### Triaje de rendimiento del compilador Rust

Esta semana estuvo dominada por oscilaciones bruscas en los benchmarks del nuevo solucionador, que aún no está activado por defecto.
Aparte de eso, tuvimos muy pocos cambios notables, solo una aceleración inesperada por una corrección de errores en rustdoc.

Triaje hecho por **@panstromek**.
Rango de revisión: [7dc2c162.. 3659db0d](https://perf.rust-lang.org/?start=7dc2c162b9c197aaa76a6f9e7534569537830a01&end=3659db0d3e2cd634c766fcda79ed118eca31a9fd&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:------:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,2% | [0,2%, 0,2%] | 3 |
| Regresiones ❌ <br /> (secundario) | 162,1% | [0,2%, 1116,3%] | 20 |
| Mejoras ✅ <br /> (primaria) | -1,4% | [-8,4%, -0,1%] | 7 |
| Mejoras ✅ <br /> (secundario) | -1,1% | [-8,4%, -0,1%] | 11 |
| Todos ❌✅ (primario) | -0,9% | [-8,4%, 0,2%] | 10 |

1 regresión, 1 mejora, 4 mixta; 3 de ellos en rollups
17 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/9f1bc6e374b5ae202366df1cbef850b79be8c641/triage/2026/2026-07-06.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Pelusa contra definiciones inválidas de símbolos POSIX](https://github.com/rust-lang/rust/pull/158522)
* [Garantías de diseño de documentos NonNull](https://github.com/rust-lang/rust/pull/158325)
* [Problema de seguimiento para 'slice_split_once'](https://github.com/rust-lang/rust/issues/112811)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Deja que el sistema operativo gestione el crecimiento de la pila](https://github.com/rust-lang/compiler-team/issues/1011)
* [Añadir 'target_feature_available_at_call_site'](https://github.com/rust-lang/compiler-team/issues/1010)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Los enums vacíos de repr(Rust) son ZSTs](https://github.com/rust-lang/reference/pull/2293)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Actualizar plantilla RFC](https://github.com/rust-lang/rfcs/pull/3982)
* [RFC: Almacenar tokens del registro en el almacén de credenciales del sistema operativo por defecto](https://github.com/rust-lang/rfcs/pull/3981)

## Próximos eventos

Eventos Rusty entre el 8-07-2026 - el 05-08-2026 🦀

### Virtual
* 2026-07-08 | Virtual (Cardiff, GB) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Club de Lectura de Sistemas Operativos: Introducción + Procesos**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315506435/)
* 2026-07-08 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/jv9lom12)
* 2026-07-09 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315517604/)
* 2026-07-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/21k797xr)
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Aprendiendo el Rust como primer lenguaje de programación**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/315279653/)
* 222-07-2026 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/hd8mlw56)
* 2026-07-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254777/)
* 2026-07-29 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/uo5ek1f4)
* 30-07-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-08-02 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-04 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-07-29 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/ii2jrwva)
* 2026-08-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210367/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de julio de 2026**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### África:
* 2026-07-14 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Depurando una caja de Rust de código abierto de grado de producción**](https://www.meetup.com/johannesburg-rust-meetup/events/315573758/)

### Europa
* 2026-07-08 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Únete en directo e INPERSONAL para Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin en localización 🏳️ 🌈 - Edición 015**](https://www.meetup.com/rust-berlin/events/315585121/)
* 2026-07-09 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Construcción de aplicaciones multiplataforma con contrachapado**](https://www.meetup.com/rust-rhein-main/events/315366165/)
* 2026-07-09 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-07-15 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund/events/)
    * [**Enseña y Hackea en Projektspeicher**](https://www.meetup.com/rust-dortmund/events/315496876/)
* 2026-07-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Funciones de Supercharge Rust con argumentos implícitos y programación genérica de contexto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework para Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 30-07-2026 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Noche de Código de Julio de Rust Manchester**](https://www.meetup.com/rust-manchester/events/315037685/)

### Norteamérica
* 2026-07-09 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de Utah Rust July**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-09 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315338107/)
* 2026-07-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, 11 de julio**](https://www.meetup.com/bostonrust/events/315225865/)
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo North End Rust, 18 de julio**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 222-07-2026 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 222-07-2026 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: ¡Rust en sistemas distribuidos con ciencia del vuelo!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 2026-07-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Porter Square Rust, 25 de julio**](https://www.meetup.com/bostonrust/events/315582650/)
* 2026-07-25 | Brooklyn, NY, EE. UU. | [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)
* 30-07-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Chinatown, 1 de agosto**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Encuentro nocturno de Boston Rust en Red Hat, 4 de agosto**](https://www.meetup.com/bostonrust/events/314660176/)

### Oceanía
* 2026-07-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/events/)
    * [**Rust Brisbane • julio de 2026**](https://www.meetup.com/rust-brisbane/events/315563251/)
* 2026-07-21 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de julio**](https://www.meetup.com/rust-canberra/events/315307280/)
* 2026-07-23 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**¡Rust Perth: Encuentro de julio!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)
* 30-07-2026 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne julio 2026**](https://www.meetup.com/rust-melbourne/events/315039480/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> si un PTR es desreferenciado en un bosque y nadie lo oye, ¿es correcto?

– [Kornel sobre usuarios de Rust](https://users.rust-lang.org/t/does-the-indirection-of-a-pointer-immediately-create-a-reference/141071/10)

¡Gracias a [Cerber-Ursi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1785) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1ureq0r/this_week_in_rust_659/)</small>
