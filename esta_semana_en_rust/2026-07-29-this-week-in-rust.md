---
title: "Esta semana en Rust #120"
number_of_week: 120
description: El crate de esta semana es cargo-efmt, un reemplazo directo para cargo fmt para soportar '.editorconfig'. 
date: 2026-07-29
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

### Boletines

* [Scientific Computing in Rust #20 (julio 2026)](https://scientificcomputing.rs/monthly/2026-07)

### Actualizaciones de proyectos/herramientas

* [AFIRM 0.7.0: Un marco genérico de métodos de entrada](https://github.com/fodydev/afrim/releases/tag/v0.7.0)
* [Compartiendo trabajo de construcción de Rust entre árboles de trabajo de carga con cargo-reapi](https://github.com/TamedTornado/cargo-reapi/blob/main/docs/introducing-cargo-reapi.md)
* [exiftool-rs 0.7.0: localización de los valores PrintConv de ExifTool, no solo sus etiquetas](https://github.com/Le-Syl21/exiftool-rs/releases/tag/v0.7.0)
* [Anunciando SeaORM 2.0](https://www.sea-ql.org/blog/2026-07-27-sea-orm-2.0/)
* [kobe 0.37.0: más fácil de desplegar e instalar](https://github.com/kunobi-ninja/kobe/releases/tag/v0.37.0)
* [Kache 0.12.0: mandos enchufables, GC más inteligente, diagnósticos más precisos](https://github.com/kunobi-ninja/kache/releases/tag/v0.12.0)
* [Progreso hacia la compilación de Linux con gccrs](https://lwn.net/SubscriberLink/1083202/f1ba926cd57ac5c5/)
* [FLODL 0.7.0: una vista de panel, repetida en todos los niveles](https://flodl.dev/blog/then-i-looked-at-it)
* [samkhya 1.2.1 — el techo de cardinalidad de unión se vuelve demostrable](https://github.com/singhpratech/samkhya/releases/tag/v1.2.1)
* [BrewFS: un sistema de archivos distribuido similar a Rust y JuiceFS](https://brewfs.ai/en/blog/introducing-brewfs) 

### Observaciones/Pensamientos

* [Mejorando la etapa estándar::simd::swizzle_dyn](https://shnatsel.github.io/improving-std-simd-swizzle-dyn/)
* [Ciclos de consultas: Un misterio de asesinato con compilador](https://ferrous-systems.com/blog/query-cycles-a-compiler-murder-mystery/)
* [GDPatch: un cargador de mods versátil para Godot](https://notnite.com/blog/gdpatch)
* [Absolutistas de la Seguridad de la Memoria](https://itsallaboutthebit.com/memory-safety-absolutists/)
* [Migración de C++ a Rust](https://blog.jetbrains.com/rust/2026/07/27/cpp-to-rust-migration/)
* [Matrices planas 2D de alto rendimiento en Rust con SIMD, caché L1](https://developerlife.com/2026/07/14/build-high-performance-flat-2d-arrays-in-rust/)
* [Construcción de microservicios Java–Rust con TeaQL: Modelos, Eventos e Intención de Auditoría](https://teaql.io/blog/java-rust-microservice-integration-with-teaql/)
* [Cómo reducimos el tiempo de reacción de un bot de trading de ~2 segundos a milisegundos — moviendo solo el camino caliente hacia el Rust](https://www.99francs.agency/blog/python-to-rust-trading-bot-migration)
* [Servidor ESP32: Distribución de flujos HTTP/2 sobre TLS](https://c410-f3r.github.io/thoughts/esp32-server-distributing-http2-streams-over-tls)
* [vídeo] [Rust Berlin Talks · 23/07/2026](https://www.youtube.com/watch?v=ut5EHZ2FK0c)

### Guías de Rust

* [Que aún no haya tokens no significa que un stream de LLM Rust sea seguro para reintentar](https://ai-router.hashnode.dev/rust-llm-stream-retry-safety)
* [serie] [Rama 101.2: Conceptos Centrales](https://plabayo.tech/blog/rama-101-2-core-concepts)
* [vídeo] [serie] [¿Qué hay dentro de Axum?](https://www.youtube.com/watch?v=rBzPw6WurN0)

## Crate de la semana

El crate de esta semana es [cargo-efmt](https://codeberg.org/filmroellchen/cargo-efmt), un reemplazo directo para cargo fmt para soportar '.editorconfig'. 

¡Gracias a [kleines Filmröllchen](https://users.rust-lang.org/t/crate-of-the-week/2704/1632) por la autosugerencia! 

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
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
- *No se presentaron convocatorias para participar esta semana.* 

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->
- *No se presentaron convocatorias ni presentaciones esta semana.* 

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

570 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-21..2026-07-28

#### Compilador
* [aplica RemoveNoopLandingPads después de la monomorfización](https://github.com/rust-lang/rust/pull/143208)
* [los cierres heredan por defecto '#[optimizar]' de la función de encierro](https://github.com/rust-lang/rust/pull/158901)
* [corrigir la convención de llamadas 'bool' para aarch64, etc](https://github.com/rust-lang/rust/pull/159317)
* [optimizar 'escape_string_symbol()'](https://github.com/rust-lang/rust/pull/159609)
* ['proc_macro'': Corregir 'cfg_attr' atraciones internas en módulos de archivo](https://github.com/rust-lang/rust/pull/159695)
* [resolver: más trabajo de preparación para paralelizar el bucle de resolución de importación](https://github.com/rust-lang/rust/pull/159440)
* [estabilizar definiciones de funciones c-variádicas](https://github.com/rust-lang/rust/pull/155697)

#### Biblioteca
* [Constificar 'Vec! [1, 2, 3]' macro](https://github.com/rust-lang/rust/pull/155795)
* [núcleo: implementar 'Rng' para referencias](https://github.com/rust-lang/rust/pull/159435)
* [define un tipo 'Simd' en 'minicore'](https://github.com/rust-lang/rust/pull/159656)
* [implementar 'CovariantUnsafeCell'](https://github.com/rust-lang/rust/pull/159738)
* [implementa 'str::copy_from_str'](https://github.com/rust-lang/rust/pull/159846)
* [iter: extender la especialización 'step_by' para cubrir 'StepBy<RangeIter<{enter}>>'](https://github.com/rust-lang/rust/pull/159518)
* [mover 'std::io::buffered' a 'alloc::io'](https://github.com/rust-lang/rust/pull/158547)
* [num: mejorar los mensajes de error para 'TryFromIntError'](https://github.com/rust-lang/rust/pull/156225)
* [fuerza: añadir ASCII camino rápido a 'word_to_titlecase'](https://github.com/rust-lang/rust/pull/159271)
* [cambian las implementaciones de 'thread_local!' para WASI](https://github.com/rust-lang/rust/pull/159733)

#### Carga
* [añadir el camino de Dylib de Haiku](https://github.com/rust-lang/cargo/pull/17248)
* ['diag': recorrido transitivo de dependencias no utilizadas atado](https://github.com/rust-lang/cargo/pull/17251)
* ['git': Ocultar la salida de git fetch sin progreso](https://github.com/rust-lang/cargo/pull/17243)
* ['git': Sugiero libgit2 si git-cli falla](https://github.com/rust-lang/cargo/pull/17252)
* ['prueba': pruebas de rutas de recorte de puertas en soporte de debuginfo dividido](https://github.com/rust-lang/cargo/pull/17256)
* ['toml': advertencia sobre nombres y duplicados de pelusas con guion](https://github.com/rust-lang/cargo/pull/17051)
* [permitir establecer el valor '-Zembed-metadata' desde la configuración](https://github.com/rust-lang/cargo/pull/17266)
* [activar el diseño de la directora de construcción v2 por defecto por la noche](https://github.com/rust-lang/cargo/pull/17258)
* [completación de zsh: Añadir banderas '-p' y '--package' para 'añadir carga'](https://github.com/rust-lang/cargo/pull/17247)

#### Rustfmt
* [permitir errores de archivo no encontrado para mods externos anotados con '#[my_macro]'](https://github.com/rust-lang/rust/pull/159737)
* [descubre módulos vía 'cfg_select!'](https://github.com/rust-lang/rust/pull/158372)

#### Rustdoc
* [añadir rutas para los elementos asociados vinculados](https://github.com/rust-lang/rust/pull/156474)
* [Recuperar información 'cfg_attr' para impls derivados para la característica 'doc_cfg'](https://github.com/rust-lang/rust/pull/159722)
* [solo implica rasgo externo de build si es necesario](https://github.com/rust-lang/rust/pull/159623)
* [solo impls en línea para primitivos locales](https://github.com/rust-lang/rust/pull/159721)

#### Clippy
* [añadir 'EULER_GAMMA' y 'GOLDEN_RATIO' a 'approx_constant'](https://github.com/rust-lang/rust-clippy/pull/17441)
* [añadir pelusa de 'assert_is_empty'](https://github.com/rust-lang/rust-clippy/pull/17149)
* [aplicar comentario de seguridad a la declaración de asignación del compuesto](https://github.com/rust-lang/rust-clippy/pull/17044)
* ['blocks_in_conditions': No pongas pelusa si el bloqueo crea algo temporal...](https://github.com/rust-lang/rust-clippy/pull/17420)
* [llama a 'in_external_macro' tras hacer otras comprobaciones en varios lugares](https://github.com/rust-lang/rust-clippy/pull/17294)
* [no activar 'clippy::exit' cuando la expresión proviene de una macro externa](https://github.com/rust-lang/rust-clippy/pull/17105)
* ['duration_suboptimal_units': imprimir el nombre completo del método en la sugerencia](https://github.com/rust-lang/rust-clippy/pull/17002)
* [extiende 'branches_sharing_code' para que haga coincidir los brazos con cola compartida](https://github.com/rust-lang/rust-clippy/pull/17313)
* ['min_ident_chars' pelusas, identificadores cortos incluso si siguen el nombre de rasgos](https://github.com/rust-lang/rust-clippy/pull/16741)
* ['multiple_unsafe_ops_per_block': falso positivo en tomar una referencia a una estática, pero no leerla/escribirla](https://github.com/rust-lang/rust-clippy/pull/17461)
* [corregir 'four_forward_slashes' falso positivo en comentarios de Inner Doc](https://github.com/rust-lang/rust-clippy/pull/17448)
* ['lint-page': añadir etiquetas accesibles a los filtros](https://github.com/rust-lang/rust-clippy/pull/17434)
* [nueva pelusa: 'nonnull_unchecked_on_box_ptr'](https://github.com/rust-lang/rust-clippy/pull/17336)
* [perf: evitar el tipo y el trabajo de ruta por llamada en 'unnecessary_mut_passed'](https://github.com/rust-lang/rust-clippy/pull/17227)
* [perf: encontrar grupos de pestañas en los comentarios del documento sin asignar](https://github.com/rust-lang/rust-clippy/pull/17410)
* [reescribir 'EndianBytes' lint pass](https://github.com/rust-lang/rust-clippy/pull/17363)

#### Analizador de Rust
* [añadir diagnóstico para patrones 'struct' que no especifican subpatrones para sus campos](https://github.com/rust-lang/rust-analyzer/pull/22851)
* [añadir paréntesis para invertir expresión general](https://github.com/rust-lang/rust-analyzer/pull/22898)
* [adjuntar base de datos sobre hilos de trabajo en inferencia paralela de análisis-estadísticas](https://github.com/rust-lang/rust-analyzer/pull/22905)
* [cambiar la versión de la cadena de herramientas no soportada para que coincida con la realidad](https://github.com/rust-lang/rust-analyzer/pull/22876)
* [el protocolo Discover solo debería analizar stdout](https://github.com/rust-lang/rust-analyzer/pull/22903)
* [no detectar '#[rust_analyzer]' como '#[rust_analyzer::rust_fixture]](https://github.com/rust-lang/rust-analyzer/pull/22881)
* [no ofrezcas 'replace_qualified_name_with_use' en un camino no cualificado](https://github.com/rust-lang/rust-analyzer/pull/22919)
* [no te asustes por un camino cualificado cuyo rasgo no es un rasgo](https://github.com/rust-lang/rust-analyzer/pull/22930)
* [no elijas un tipo discriminante más grande que el de Typeck](https://github.com/rust-lang/rust-analyzer/pull/22932)
* [arreglar archivo de bloqueo obsoleto](https://github.com/rust-lang/rust-analyzer/pull/22908)
* [arreglar la llamada '.zip(Ninguno)'](https://github.com/rust-lang/rust-analyzer/pull/22924)
* [da un resultado de ciclo a 'impl_trait_with_diagnostics'](https://github.com/rust-lang/rust-analyzer/pull/22923)
* [haz que la barra de progreso de estadísticas de análisis sea segura en Unicode](https://github.com/rust-lang/rust-analyzer/pull/22909)
* [pánico merge_imports' en caminos inválidos](https://github.com/rust-lang/rust-analyzer/pull/22892)
* [pánico en estructuras macro-definidas con campos desconocidos](https://github.com/rust-lang/rust-analyzer/pull/22843)
* [prefiere 'alloc' sobre rutas 'std' cuando 'preferNoStd' está activado](https://github.com/rust-lang/rust-analyzer/pull/22918)
* [Registrar la cadena de obligación para diagnósticos de rasgos no implementados y mostrarlo](https://github.com/rust-lang/rust-analyzer/pull/22854)
* [reemplazar desacoplar por borrar para 'ast::IdentPat'](https://github.com/rust-lang/rust-analyzer/pull/22916)
* [ruta de resolución en todo el espacio de nombres en 'resolve_path'](https://github.com/rust-lang/rust-analyzer/pull/22743)
* [respetar 'references.exclude[Tests/Imports]' en la lente de referencias](https://github.com/rust-lang/rust-analyzer/pull/22660)
* [cebado perezoso con mira](https://github.com/rust-lang/rust-analyzer/pull/22587)
* [soporte diagnóstico de código inactivo en macros](https://github.com/rust-lang/rust-analyzer/pull/22306)
* [usa bool en lugar de pat ty en guardia](https://github.com/rust-lang/rust-analyzer/pull/22896)

### Triaje de rendimiento del compilador Rust

Varias mejoras importantes se lograron en la última semana: 

* Rustdoc es de media aproximadamente un 16% más rápido en todos nuestros benchmarks de documentos: 
  * [rustdoc: Solo impls en línea para primitivas locales](https://github.com/rust-lang/rust/pull/159721), builds de doc un 7% más rápidas
  * [rustdoc: Solo sintetiza auto/blanket impls para objetos documentados](https://github.com/rust-lang/rust/pull/159779), otros builds de doc un 7% más rápidos
  * [rustdoc: Solo construye implicaciones de rasgos externos si es necesario](https://github.com/rust-lang/rust/pull/159623), construcciones de doc un 10% más rápidas
* [Eliminación anticipada del manejo de pánico no-op en las builds de depuración](https://github.com/rust-lang/rust/pull/143208). Esto acelera la carga en un ~4% en el recuento cíclico. 
* [Optimizar escape_string_symbol()](https://github.com/rust-lang/rust/pull/159609) acelerado
  sube grandes 'include_bytes!'/'include_str!' mediante cambios en la escapada de cadenas, evitando una regresión en la próxima actualización de LLVM 23. 

¡Me alegra ver tantas mejoras! 

Triaje hecho por **@simulacrum**. 
Rango de revisión: [d527BC9B.. AD0C9DCE](https://perf.rust-lang.org/?start=d527bc9bfa297ca7fd7f5ae93781eeec42073170&end=ad0c9dce27a22416b65946bc0010edaf22ac6c83&absolute=false&stat=instructions%3Au)

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-07-27.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Resolución superficial ty y vars const a sus vars raíz, intento 2](https://github.com/rust-lang/rust/pull/158447)
* [Asegurarse de que los tipos de patrones inferidos estén bien formados](https://github.com/rust-lang/rust/pull/157841)
* [estabilizar 'c_variadic_naked_functions'](https://github.com/rust-lang/rust/pull/159746)
* [pelusa contra atributos repetidos de repr](https://github.com/rust-lang/rust/pull/157036)
* [Estabilizar los enteros de 128 bits pasando mediante registros vectoriales con 'asm!' en x86](https://github.com/rust-lang/rust/pull/159525)
* [Añadir nueva pelusa de 'invalid_markdown_table' de rustdoc](https://github.com/rust-lang/rust/pull/159583)
* [asignaciones: documenta que pueden ser de solo lectura](https://github.com/rust-lang/rust/pull/159503)
* [las asignaciones pueden crecer (pero no reducirse)](https://github.com/rust-lang/rust/pull/159729)
* [Problema de seguimiento para 'bool::toggle'](https://github.com/rust-lang/rust/issues/159298)
* [Problema de seguimiento para const_btree_len](https://github.com/rust-lang/rust/issues/71835)
* [Añadir pelusa de 'raw_borrows_via_references'](https://github.com/rust-lang/rust/pull/138230)

* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Wasm activa soporte de macros](https://github.com/rust-lang/compiler-team/issues/1017)
* [opciones de modificador de objetivo de grupo bajo -T](https://github.com/rust-lang/compiler-team/issues/980)

* [Optimizar los enums repr(Rust) omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Refactorizar el equipo liberal](https://github.com/rust-lang/rfcs/pull/3984)
* [Carga: 'hints.min-opt-level'](https://github.com/rust-lang/rfcs/pull/3924)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(profile): Añadir depuración de perfil incorporada](https://github.com/rust-lang/cargo/pull/17214)
* [feat(toml): permitir anular características predeterminadas heredadas en 2024](https://github.com/rust-lang/cargo/pull/17126)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Externref lang item para objetivos Wasm](https://github.com/rust-lang/rfcs/pull/3987)

## Próximos eventos

Eventos Rusty entre el 29-07-2026 - el 26-08-2026 🦀

### Virtual
* 30-07-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045928/)
* 31-07-2026 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/uo5ek1f4)
* 01-08-2026 | Virtual (Kampala, UG) | [Encuentro del Círculo Oxidado](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-08-02 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-03 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Workshop: Añadir pruebas a un proyecto Rust de código abierto**](https://luma.com/nwfmsdtf)
* 2026-08-04 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-08-04 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/315800760/)
* 2026-08-05 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Ejecución y Programación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315880365/)
* 2026-08-05 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-07 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/ii2jrwva)
* 2026-08-11 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-13 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-08-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/315604176/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, EE.UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #5 — Comunicación inalámbrica con el protocolo IEEE 802.15.4**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254775/)

### África
* 2026-08-11 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de agosto 2026**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Delhi Oxidado](https://www.meetup.com/rustdelhi)
    * [**Encuentro de Rust Delhi X SciPy India**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Computación científica en Rust y pitón**](https://scipy.in/sci-py-rs/)

### Europa
* 2026-07-29 | Polonia, PL | [Polonia Oxidada](https://www.meetup.com/rust-poland-meetup)
    * [**Polonia oxidada x Cracovia #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 30-07-2026 | Copenhague, DK | [Comunidad Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #70**](https://www.meetup.com/copenhagen-rust-community/events/315767999/)
* 2026-07-30 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Julio de Rust Manchester**](https://www.meetup.com/rust-manchester/events/315037685/)
* 06-08-2026 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Social ACCU/Rust Summer**](https://www.meetup.com/oxford-rust-meetup-group/events/315863373/)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Confía pero verifica el LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rhein-Meno Oxidado](https://www.meetup.com/rust-rhein-main)
    * [**Construcción de una cámara acústica con egui y embajada**](https://www.meetup.com/rust-rhein-main/events/315855368/)

### Norteamérica
* 30-07-2026 | Atlanta, GA, EE.UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539329/)
* 01-08-2026 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Chinatown, 1 de agosto**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Encuentro nocturno de Boston Rust en Red Hat, 4 de agosto**](https://www.meetup.com/bostonrust/events/314660176/)
* 2026-08-06 | Mountain View, CALI, EE.UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315590399/)
* 2026-08-06 | Saint Louis, MO, EE.UU. [Rust STL](https://www.meetup.com/stl-rust)
    * [**Envío temporal: Cómo un ecosistema global de Rust construyó la última API web de Chrome**](https://www.meetup.com/stl-rust/events/314701905/)
* 2026-08-13 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de agosto de Utah Rust**](https://www.meetup.com/utah-rust/events/314696652/)
* 2026-08-13 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust August Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601099/)
* 2026-08-15 | San Francisco, CA, EE. UU. [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 3**](https://partiful.com/e/juWAwRs3XMWP7s9wLNWK)
* 18-08-2026 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997215/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 19-08-2026 | San Francisco, CA, EE. UU. [Área de Rust Bay](https://luma.com/bayarearust)
    * [**Encuentro de agosto en el Área de Rust Bay**](https://luma.com/00f2s7q9)
* 2026-08-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 26-08-2026 | Austin, TX, EE.UU. [ATX Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/315171660/)

### Oceanía
* 30-07-2026 | Melbourne, AU | [Melbourne Oxidado](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne julio 2026**](https://www.meetup.com/rust-melbourne/events/315039480/)

### Sudamérica
* 2026-08-08 | São Paulo, SP | [Rust-SP](https://luma.com/calendar/cal-bif2oHITU1aVvsr)
    * [**Rust SP - Ago/2026**](https://luma.com/41oiyhtk)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Así que hablemos de cómo ha sido el proceso para Netstack3. Durante 11 meses, el equipo ha estado intensificando un programa de captura de perros en contacto. En su punto álgido, ese programa ha tenido alrededor de 60 dispositivos funcionando casi 24/7 en casas de desarrolladores. 
>
> De nuevo, si fuera cualquier otro netstack, habríamos esperado descubrir una enorme montaña de bugs en ese tiempo. Entonces, en el último año, ¿cuántos bugs descubrió el equipo en el campo? 
>
> Tres. 

– [Josh Liebow-Feeser en su blog](https://joshlf.com/posts/safety-unsafe-world/)

Llogiq de nuevo no tiene a quién agradecer una sugerencia, así que se agradece a sí mismo haber encontrado esa cita en su lugar. 

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

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1vaibge/this_week_in_rust_662/)</small>