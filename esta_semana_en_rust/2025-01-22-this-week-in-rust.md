---
title: "Esta semana en Rust #45"
number_of_week: 45
description: La comunidad de esta semana es resvg, una biblioteca de renderizado SVG.
date: 2025-01-22
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenido a otro número de *esta semana en Rust*! [Rust](https://www.rust-lang.org/)
en lenguaje programación que permite todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o envíanos un [solicitud extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan la contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).


Esta semana en Rust se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición en esta semana, [por favor envía un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandej e entrada? [Suscríbet quí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunida de Rust 🥰

### Oficial
* [Este ciclo de desarrollo en Cargo: 1.85](https://blog.rust-lang.org/inside-rust/2025/01/17/this-development-cycle-in-cargo-1.85.html)

### Boletines
* [El Rustáceo Incrustado Edición #37](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-37)

### Actualizaciones de proyectos/herramientas
* [gitoxide - Enero 2025](https://github.com/GitoxideLabs/gitoxide/discussions/1791)
* [Lanzamiento de Musi Lili 0.2](https://codeberg.org/vivi-ui/lili/src/branch/main/CHANGELOG.md#0-2)

### Observaciones/Pensamientos
* [Comparación de las bibliotecas actor de Rust: Actix, Coerce, Kameo, Ractor y Xtra](https://theari.dev/blog/comparing-rust-actor-libraries/)
* [Mejorar el tiempo de compilación de Rust en 108X](https://burn.dev/blog/improve-rust-compile-time-by-108x)
* [Codificación UTF-8 sin ramificaciones](https://cceckman.com/writing/branchless-utf8-encoding/)
* [La caza del error -22](https://tweedegolf.nl/en/blog/145/the-hunt-for-error--22)
* [Recarga automática del servidor en Rust: ¿Qué son listenfd/systemfd?](https://lucumr.pocoo.org/2025/1/19/what-is-systemfd/)
* [Investigando un extraño error de falta de memoria](https://www.qovery.com/blog/rust-investigating-a-strange-out-of-memory-error/)
* [Comparación de 13 crates de Rust para extraer texto de HTML](https://emschwartz.me/comparing-13-rust-crates-for-extracting-text-from-html/)
* [Enrutamiento frontend type-safe en Rust/Leptos](https://dnaaun.github.io/posts/typesafe-routing-in-rust-leptos/)
* [Entrevista con un apasionado desarrollador de Rust, Radu Marias](https://blog.rust.careers/post/radu_interview/)
* [Presentación de RealtimeSanitizer para Rust](https://steck.tech/posts/rtsan-in-rust/)
* [El stack HARM (HTMX, Axum/AlpineJS, Rust, Maud) considerado inofensivo](https://nguyenhuythanh.com/posts/the-harm-stack-considered-unharmful/)
* [Inferencia de tipos en Rust y C++](https://herecomesthemoon.net/2025/01/type-inference-in-rust-and-cpp/)
* [La canonicalización de rutas no es ergonómica](https://tesujimath.org/path-canonicalization-is-unergonomic/)
* [Video] [Rust Not Complicated - Una perspectiva de arquitecto](https://www.youtube.com/watch?v=bYKPKBPo7EY)


### Tutoriales de Rust
* [Una amable introducción a WebAssembly en Rust (edición 2025)](https://dev.to/marktolmacs/a-gentle-introduction-to-webassembly-in-rust-2025-edition-1iac)
* [Añadiendo una nueva falsificación al crate fake](https://xd009642.github.io/2025/01/15/adding-a-new-fake-to-the-fake-crate.html)
* [API de audio en streaming: el servidor Axum](https://xd009642.github.io/2025/01/20/streaming-audio-APIs-the-axum-server.html)
* [Prototipado en Rust](https://corrode.dev/blog/prototyping/)

### Miscelánea
* [Resumen de 2024 - Seanmonstar](https://seanmonstar.com/blog/2024-in-review/)
* ["Nunca actualizamos a menos que nos obliguen" - Resumen 2024 de cargo-semver-checks](https://predr.ag/blog/cargo-semver-checks-2024-year-in-review/)
* [Resumen anual: Lo destacado de 2024 y un vistazo a 2025 - Grafito](https://graphite.rs/blog/year-in-review-2024-highlights-and-a-peek-at-2025)
* [Por qué Rust no es un lenguaje funcional](https://serokell.io/blog/rust-is-not-a-functional-language)
* [Cómo pienso sobre Zig y Rust](https://lewiscampbell.tech/blog/250117.html)
* [Video] [Rompiendo la seguridad de memoria de Rust en 1 línea de código](https://youtu.be/HGERkwG0ViI)
* [Audio] [Crates que amamos | Rust y amigos](https://share.transistor.fm/s/d3cea637)

## `Crate` de la semana

La `Crate` de esta semana es [resvg](https://crates.io/crates/resvg), una biblioteca de renderizado SVG.

¡Gracias a [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/1389) por sugerir su propia crate!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamada a pruebas
Un paso importante en la implementación de una RFC es que las personas experimenten con la
implementación y brinden retroalimentación, especialmente antes de su estabilización.
Las siguientes RFCs se beneficiarían de pruebas por parte de los usuarios antes de continuar avanzando:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue una nueva 'call-for-testing'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan ser evaluados.

## Convocatorio a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyecto de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tarea en la comunida de Rust para que elijas y comiences!

Alguna de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [rama - primera versión de Rama-Unix](https://github.com/plabayo/rama/issues/370)
* [rama - añadir servicio de router web a rama-http](https://github.com/plabayo/rama/issues/396)
* [rama - Añadir Benchmark de rama de Pila Completa](https://github.com/plabayo/rama/issues/374)
* [rama - añadir rama al "The-Benchmarker"](https://github.com/plabayo/rama/issues/398)
* [rama - Añadir Rama del Server Benchmark a Sharkbench](https://github.com/plabayo/rama/issues/390)
* [rama - añadir rama al FrameworkBenchmark de TechEmpower](https://github.com/plabayo/rama/issues/389)

Si eres propietario de un proyect de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que está aceptando presentaciones par unirse a su evento como orador.

Si eres organizador de un evento y esperas ampliar su alcance, envía un enlace a la página web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o contactándonos en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust).!

## Actualizacione el Proyecto Rust

Se [fusionaron 397 solicitude e extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-14..2025-01-21

* [`cfg_match`: Mejorar sintaxis](https://github.com/rust-lang/rust/pull/133720)
* [`rustc_resolve`: Usar campos estructurados en los logs de seguimiento](https://github.com/rust-lang/rust/pull/135676)
* [Agregar anulación de perfil para fuentes que no son Git](https://github.com/rust-lang/rust/pull/135433)
* [Añadir caché a `AmbiguityCausesVisitor`](https://github.com/rust-lang/rust/pull/135618)
* [Agregar convención de llamada GPU-kernel](https://github.com/rust-lang/rust/pull/135047)
* [Añadir `license-metadata.json` al tarball `rustc-src`](https://github.com/rust-lang/rust/pull/135588)
* [Permitir coerción de funciones `target_feature` seguras a punteros de función seguros](https://github.com/rust-lang/rust/pull/135504)
* [Forzar recorte de rutas en el lint `unreachable_patterns`](https://github.com/rust-lang/rust/pull/135310)
* [Limpiar verificación de movimiento promovido](https://github.com/rust-lang/rust/pull/134455)
* [Mejorar manejo de errores de diseño inválido](https://github.com/rust-lang/rust/pull/135264)
* [Consolidar lints MIR ad-hoc en lints MIR formales](https://github.com/rust-lang/rust/pull/135705)
* [Trait const: Eliminar falsos positivos conocidos](https://github.com/rust-lang/rust/pull/135523)
* [const-eval: Detectar más punteros definitivamente no nulos](https://github.com/rust-lang/rust/pull/133700)
* [Convertir `FromBytesWithNulError` de struct a enum](https://github.com/rust-lang/rust/pull/134143)
* [Cobertura: Revisar asignación de contadores usando CFG](https://github.com/rust-lang/rust/pull/135481)
* [Detectar cadena `else if` faltante en errores de tipo](https://github.com/rust-lang/rust/pull/135558)
* [Prohibir `A { .. }` cuando `A` no tiene campos](https://github.com/rust-lang/rust/pull/135703)
* [Ignorar traits con condiciones no satisfechas en const-checking](https://github.com/rust-lang/rust/pull/135425)
* [Optimizar análisis de argumentos en ejecuciones sin parámetros](https://github.com/rust-lang/rust/pull/135716)
* [Mejorar manejo de drops ansiosos para structs con vidas](https://github.com/rust-lang/rust/pull/135313)
* [Codificar restricciones de ubicación en Polonius](https://github.com/rust-lang/rust/pull/135290)
* [Reforzar estabilidad sintáctica de traits const en HIR](https://github.com/rust-lang/rust/pull/135423)
* [Corregir ICE en resolución de elementos asociados sin vinculantes](https://github.com/rust-lang/rust/pull/135663)
* [Corregir desbordamientos en el lint `overflowing_literals`](https://github.com/rust-lang/rust/pull/135249)
* [Mejorar sugerencia para desreferenciación de punteros sin format](https://github.com/rust-lang/rust/pull/135601)
* [Optimizar búsqueda de funciones basadas en tipo de retorno](https://github.com/rust-lang/rust/pull/135302)
* [Desestabilizar completamente los atributos internos personalizados](https://github.com/rust-lang/rust/pull/134276)
* [Mejorar `panic_immediate_abort` eliminando mensajes redundantes](https://github.com/rust-lang/rust/pull/135446)
* [Implementar elementos de trait asociados en bloques `use`](https://github.com/rust-lang/rust/pull/134754)
* [Mejorar validación de `DispatchFromDyn` y `CoerceUnsized`](https://github.com/rust-lang/rust/pull/135228)
* [Prevenir monomorfización de instancias imposibles](https://github.com/rust-lang/rust/pull/135466)
* [Finalizar implementación de Polonius sensible a ubicación](https://github.com/rust-lang/rust/pull/134980)
* [Hacer `missing_abi` lint warn por defecto](https://github.com/rust-lang/rust/pull/132397)
* [Asegurar manejo de regiones en Borrowck](https://github.com/rust-lang/rust/pull/134940)
* [Manejar correctamente sustituciones de vidas triviales](https://github.com/rust-lang/rust/pull/135520)
* [Producir errores `ConstArgHasWrongType` para valores const](https://github.com/rust-lang/rust/pull/135380)
* [Hacer métodos de traits const como const](https://github.com/rust-lang/rust/pull/135541)
* [Limpiar manejo de regiones en Mir Borrowck](https://github.com/rust-lang/rust/pull/135479)
* [Nuevo solver: Preferir implementaciones triviales](https://github.com/rust-lang/rust/pull/135639)
* [Tratar solo patrones literales simples como cortos](https://github.com/rust-lang/rust/pull/135251)
* [Esbozar código de pánico para `LocalKey::with`](https://github.com/rust-lang/rust/pull/135224)
* [Reemplazar `extern "rust-intrinsic"` con `#[rustc_intrinsic]`](https://github.com/rust-lang/rust/pull/135333)
* [Preferir candidatos menores en `TraitUpcasting`](https://github.com/rust-lang/rust/pull/135498)
* [Sugerencia estructurada para `#![feature(..)]` inválido](https://github.com/rust-lang/rust/pull/134858)
* [Reexportar `likely`/`unlikely` en `std::hint`](https://github.com/rust-lang/rust/pull/133695)
* [Respetar `--sysroot` en `rustc -vV` y `-Cpasses=list`](https://github.com/rust-lang/rust/pull/135330)
* [Ignorar HirIds duplicados en hash estable](https://github.com/rust-lang/rust/pull/135329)
* [Mejorar búsqueda basada en tipos](https://github.com/rust-lang/rust/pull/131806)
* [Tratar funciones `target_feature` como unsafe por defecto](https://github.com/rust-lang/rust/pull/134353)
* [Usar tipos C-safe para intrínsecos `__rust_[ui]128_*`](https://github.com/rust-lang/rust/pull/134338)
* [Retorno indirecto para `i128`/`f128` en wasm32](https://github.com/rust-lang/rust/pull/135534)
* [Detectar definiciones de traits para alias de traits](https://github.com/rust-lang/rust/pull/134504)
* [Manejar discriminadores LLVM excedidos](https://github.com/rust-lang/rust/pull/135643)
* [Estabilizar `float_next_up_down`](https://github.com/rust-lang/rust/pull/135661)
* [Asignar ID de hilo principal perezosamente](https://github.com/rust-lang/rust/pull/132654)
* [Refinar `Path::name` para mostrar solo el elemento final](https://github.com/rust-lang/rust/pull/134880)
* [Mover `std::pe` a `std::io`](https://github.com/rust-lang/rust/pull/135583)
* [Reducir código unsafe en `dangling`/`without_provenance`](https://github.com/rust-lang/rust/pull/135344)
* [Crear función para alias definidos por usuario](https://github.com/rust-lang/cargo/pull/15076)
* [Corregir `Benchsuit` con versiones nuevas de Git](https://github.com/rust-lang/cargo/pull/15069)
* [Corregir concatenación y nombres de variables](https://github.com/rust-lang/cargo/pull/15074)
* [Integrar funcionalidad de terceros en `list_commands`](https://github.com/rust-lang/cargo/pull/15075)
* [Eliminar trait `AttributesExt` obsoleto](https://github.com/rust-lang/rust/pull/135428)
* [Reemplazar listas de módulos con elementos semánticos](https://github.com/rust-lang/rust/pull/135641)
* [Nuevo lint: `repeat().take()` → `repeat_n()`](https://github.com/rust-lang/rust-clippy/pull/13858)
* [Mover `literal_string_with_formatting_args` a nursery](https://github.com/rust-lang/rust-clippy/pull/14014)
* [Emitir `missing_const_for_fn` para `CONST_MUT_REFS`](https://github.com/rust-lang/rust-clippy/pull/13839)
* [Corregir sugerencia para `significant_drop_in_scrutinee`](https://github.com/rust-lang/rust-clippy/pull/14019)
* [Nuevo lint: `useless-non-zero-new_unchecked`](https://github.com/rust-lang/rust-clippy/pull/13993)
* [Nuevo lint: `unnecessary_semicolon`](https://github.com/rust-lang/rust-clippy/pull/14032)
* [Soporte para elisión de autotipos en Rust 1.81+](https://github.com/rust-lang/rust-clippy/pull/13992)
* [Mejorar sugerencia para `manual_div_ceil`](https://github.com/rust-lang/rust-clippy/pull/13951)
* [Mejorar sugerencia multipar para `unnecessary_map_or`](https://github.com/rust-lang/rust-clippy/pull/13998)
* [Añadir anotaciones faltantes `#[rust_analyzer::rust_fixture]`](https://github.com/rust-lang/rust-analyzer/pull/18951)
* [Añadir feature `Win32_Foundation` faltante](https://github.com/rust-lang/rust-analyzer/pull/18963)
* [Mejorar persistencia de variables en dispatchers](https://github.com/rust-lang/rust-analyzer/pull/18982)
* [Autocompletado de desreferenciación](https://github.com/rust-lang/rust-analyzer/pull/18917)
* [Finalización inteligente para `await`/`iter()`](https://github.com/rust-lang/rust-analyzer/pull/18927)
* [Navegación entre definiciones `From`/`Into`](https://github.com/rust-lang/rust-analyzer/pull/18934)
* [Completado raw y palabras clave `const`](https://github.com/rust-lang/rust-analyzer/pull/18952)
* [Límites de renderizado de tipos en proyecciones](https://github.com/rust-lang/rust-analyzer/pull/18925)
* [Acción `go-to-type-def` para parámetros y trait bounds](https://github.com/rust-lang/rust-analyzer/pull/18946)
* [Requerir `unstable-options` en `cargo rustc --print`](https://github.com/rust-lang/rust-analyzer/pull/18968)
* [Detectar errores de llaves faltantes en `let...else`](https://github.com/rust-lang/rust-analyzer/pull/18908)
* [Manejar correctamente variantes de enums en resolución de tipos](https://github.com/rust-lang/rust-analyzer/pull/18976)
* [Manejar desbordamiento de pila en expansión de macros](https://github.com/rust-lang/rust-analyzer/pull/18929)
* [Corregir navegación ascendente en impls de traits](https://github.com/rust-lang/rust-analyzer/pull/18977)
* [Almacenar correctamente raíces de archivos](https://github.com/rust-lang/rust-analyzer/pull/18940)
* [Hacer `stdout` opcional en pruebas de cargo](https://github.com/rust-lang/rust-analyzer/pull/18897)
* [Mejorar configuración de escritura opcional](https://github.com/rust-lang/rust-analyzer/pull/18939)
* [Generalizar sugerencias de tipos en hover](https://github.com/rust-lang/rust-analyzer/pull/18950)
* [Optimizar envío de mensajes LSP](https://github.com/rust-lang/rust-analyzer/pull/18972)
* [Hacer `RTLD_DEEPBIND` portable en proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/18981)
* [Registrar reexports significativos correctamente](https://github.com/rust-lang/rust-analyzer/pull/18967)

### Clasificación del rendimiento de compilador de Rust

Una semana muy tranquila para el rendimiento, con pequeña mejoras esencialmente en todos los benchmarks.

Triage de realizado por **@simulacrum**.
Range de revisión: [1ab85fbd.. 9a1d156f](https://perf.rust-lang.org/?start=1ab85fbd7474e8ce84d5283548f21472860de3e2&end=9a1d156f38c51441ee51e5a068f1d0caf4bb0f27&absolute=false&stat=instructions%3Au)

0 Regresión, 1 Mejora, 2 Mixto; 0 de ellos en rollups
40 comparacione  rtefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-01-20.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitu  omentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se probaron para la implementación esta semana:

* [Sombreado de elementos de supertrait v2](https://github.com/rust-lang/rfcs/pull/3624)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC] Modificadores de objetivo](https://github.com/rust-lang/rfcs/pull/3716)

#### Seguimiento de problemas y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Eliminar los atributos obsoletos `#![start]` y `crate_id`](https://github.com/rust-lang/rust/pull/134300)
* [Estabilizar `target_feature_11`](https://github.com/rust-lang/rust/pull/134090)
* [Agregar documentación de error 'El archivo ya existe' a la función `hard_link`](https://github.com/rust-lang/rust/pull/135415)
* [Problema de seguimiento para `once_wait`](https://github.com/rust-lang/rust/issues/127527)
* [Corregir(libtest): Dejar de usar `--logfile`](https://github.com/rust-lang/rust/pull/134283)
* [Windows: Eliminar archivos de solo lectura](https://github.com/rust-lang/rust/pull/134679)
* [Hacer que la feature `rustc_encodable_decodable` sea correctamente nestable](https://github.com/rust-lang/rust/pull/134272)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de Cargo en período final de comentarios esta semana*

##### [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna propuesta del equipo de lenguaje en período final de comentarios*

##### [Referencia del Lenguaje](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFCs de referencia en período final de comentarios*

##### [Directrices de Código Inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay actualizaciones en directrices de código inseguro*

#### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Metas del proyecto para 2025H1](https://github.com/rust-lang/rfcs/pull/3764)

## Próximos eventos

Evento Rust entre 2025-01-22 - 2025-02-19 🦀

## Eventos Virtuales de Rust

### Virtual
* 2025-01-22 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos con Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)
* 2025-01-23 y 2025-01-24 | Virtual | [Taller de Rust por Mainmatter](https://ti.to/mainmatter/)
    * [**Taller remoto: Pruebas para proyectos de Rust: yendo más allá de lo básico**](https://ti.to/mainmatter/rust-testing-jan-2025)
* 24/01/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Club de Codificadores Elegante y Curioso Cooperativo](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305425309/)
* 26/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Rust y programación integrada con Leon Vak (en línea en hebreo)**](https://www.meetup.com/rust-tlv/events/304971264)
* 27/01/2025 | Virtual (Londres, Reino Unido) | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/)
    * [**Usando rasgos en Rust para flexibilidad, simulación/pruebas unitarias, y más**](https://www.meetup.com/london-rust-project-group/events/305211672/)
* 28/01/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/305361243)
* 30/01/2025 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468340)
* 30/01/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡La computadora cuántica no puede proteger esto contra el Rust!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 30/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
*   * [**Are We Embedded Yet? - Implementación de un pequeño servidor HTTP en microcontrolador**](https://www.meetup.com/code-mavens/events/305382647)
* 31/01/2025 | Virtual (Delhi, IN) | [Asociación de Hackathon Raptors](https://www.meetup.com/hackathon-raptors-association/)
    * [**Hackathon de Rust increíblemente rápido**](https://www.meetup.com/hackathon-raptors-association/events/305435372/)
* 31/01/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Club de Codificadores Elegante y Curioso Cooperativo](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305560416/)
* 01/02/2025 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 04/02/2025 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
* 04/02/2025 | Virtual (Londres, Gran Bretaña) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Rust Nation UK Talks**](https://www.meetup.com/women-in-rust/events/305647334)
* 05/02/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - co-instanciamiento social**](https://www.meetup.com/indyrs/events/302031658)
* 07/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Club de Codificadores Elegante y Curioso Cooperativo](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbkb/)
* 2025-02-11 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/302815036)
* 2025-02-11 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Conoce Elusion: Nueva biblioteca de arco de datos impulsada por Rust 🦀 con Borivoj Grujicic**](https://www.meetup.com/code-mavens/events/305513416)
* 13/02/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 18/02/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidación mediada por meses**](https://www.meetup.com/rustdc/events/305170682)
* 19/02/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust en Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Procedencia del puntero**](https://www.meetup.com/vancouver-rust/events/304051783)

### Europa
* 2025-01-22 | Londres, Gran Bretaña | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Fiesta de Año Nuevo de Londres y Lanzamiento de Swag de la Comunidad**](https://www.meetup.com/rust-london-user-group/events/305588703)
* 2025-01-22 | Oberursel, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Edición 2024 de Rust y más allá**](https://www.meetup.com/rust-rhein-main/events/305330873)
* 23/01/2025 | Barcelona, ES | [Barcelona Software Libre](https://www.meetup.com/barcelona-free-software/events/)
    * [**¿Por qué construir un nuevo motor de navegador en Rust?**](https://www.meetup.com/barcelona-free-software/events/305179554)
* 23/01/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentro de Rust #74**](https://www.meetup.com/rust-paris/events/305455221)
* 24/01/2025 | Edimburgo, Gran Bretaña | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust y Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/305204279)
* 27/01/2025 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Praga (enero 2025)**](https://www.meetup.com/rust-prague/events/305455153)
* 27/01/2025 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Meetup 2025/01: Optimización de compilación de C++ y comparación con Rust (GUI tool)**](https://www.meetup.com/zagreb-rust-meetup/events/305624480)
* 28/01/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Aventura del Código**](https://www.meetup.com/rust-aarhus/events/304487851)
* 28/01/2025 | Manchester, Gran Bretaña | [Rust en Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester January Code Night**](https://www.meetup.com/rust-manchester/events/305496243)
* 28/01/2025 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/events/)
    * [**Rust Warsaw Meetup #4**](https://www.meetup.com/rust-warsaw/events/305628338)
* 30/01/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Desarrollo impulsado por hipermedios en Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 30/01/2025 | Berlín, DE | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421383)
* 30/01/2025 | Copenhague, Dinamarca | [Comunidad de Rust en Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Meetup #54 patrocinado por Google**](https://www.meetup.com/copenhagen-rust-community/events/305453880)
* 01/02/2025 | Bruselas, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 01/02/2025 | Helsinki, FI | [Grupo Rust-lang de Finlandia](https://www.meetup.com/finland-rust-meetup/events/)
    * [**Encuentro de febrero**](https://www.meetup.com/finland-rust-meetup/events/305666104)
* 01/02/2025 | Nürnberg, DE | [Rust en Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 05/02/2025 | Oxford, Gran Bretaña | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)
* 06/02/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #7**](https://www.meetup.com/rust-gdansk/events/305742562)
* 2025-02-12 | Lectura, GB | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 18/02/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introducción a la Programación Contextual-Genérica en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 2025-02-19 - 2025-02-20 | Londres, Reino Unido | [Nación Rust Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation Reino Unido 2025**](https://www.rustnationuk.com/)

### América del Norte
* 2025-01-22 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/305325657)
* 23/01/2025 | Mountain View, CA, EE. UU. | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305414182) | [**Encuentro de Rust en Hacker Dojo - Página de Encuentro de Rust en Mountain View**](https://www.meetup.com/mv-rust-meetup/events/305564600)
* 28/01/2025 | Boulder, CO, EE. UU. | [Encuentro de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/events/)
    * [**De lo Básico a lo Avanzado: Pruebas**](https://www.meetup.com/boulder-rust-meetup/events/305597961)
* 06/02/2025 | Vista a la Montaña, CA, EE. UU. | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305517476)
* 06/02/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Async, el futuro de los futuros**](https://www.meetup.com/stl-rust/events/304959018)
* 2025-02-11 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/305720765)
* 18/02/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Rust en San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)61)

### Oceanía
* 04/02/2025 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust AKL: Cómo aprendemos Rust**](https://www.meetup.com/rust-akl/events/305583693)

Si estás organizando un evento de Rust, agrégalo al [calendario] para que se mencione aquí.
Por favor, recuerda agregar un enlace al evento también.
Envía un correo electrónico para solicitar acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, consulte el último hilo en [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Fras e la semana

> Los problemas de seguridad de memoria significan que ya no puede confiar en lo que ves en tu código fuente.

– [Alguien en Antítesis en el blog de la lanzadera](https://www.shuttle.dev/blog/2025/01/14/the-appeal-of-rust)

¡Gracias a [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1651) por la sugerencia!

[¡Por favor, envíe su citas y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista  correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1i81bmu/this_week_in_rust_583/)</small>
