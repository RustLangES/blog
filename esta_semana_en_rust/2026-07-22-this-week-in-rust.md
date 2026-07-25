---
title: "Esta semana en Rust #119"
number_of_week: 119
description: El crate de esta semana es xan, un kit de herramientas TUI para trabajar con archivos CSV.
date: 2026-07-22
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
* [Anunciando Rust 1.97.1](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/)

### Boletines
* [El Rustaceo Incrustado Número #76](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-76)

### Actualizaciones de proyectos/herramientas
* [Anunciando Topcoat: un framework para construir aplicaciones web reactivas full-stack con Rust](https://tokio.rs/blog/2026-07-22-announcing-topcoat)
* [Syn 3.0.0](https://github.com/dtolnay/syn/releases/tag/3.0.0)
* [Novedades en RustRover 2026.2](https://blog.jetbrains.com/rust/2026/07/22/whats-new-in-rustrover-2026-2/)
* [Kobe 0.35.0: Compuertas de Preparación y Reciclaje de Certificados](https://github.com/kunobi-ninja/kobe/releases/tag/v0.35.0)
* [Comhad v0.1.0: un sustituto de cyberduck estilo tui para navegar en la S3](https://github.com/Eoin-McMahon/comhad/releases/tag/v0.1.0)
* [Nova v0.2.1: servidor MCP de uso informático](https://github.com/bigduu/Nova/releases/tag/v0.2.1)
* [Winit ahora cuenta con soporte integral de arrastrar y soltar multiplataforma, exponiendo la mayor parte del poder de las APIs del sistema operativo subyacentes](https://github.com/rust-windowing/winit/pull/4571)
* [crimson-crab v0.1.0 - un SDK de Rust de grado de producción para la API de Claude (streaming, uso de herramientas, caché de prompts, lotes)](https://github.com/singhpratech/crimson-crab/releases/tag/v0.1.0)
* [ferrovec: búsqueda vectorial HNSW ligera de dependencias en Rust, compilado a WebAssembly para búsqueda semántica privada dentro del navegador](https://singhpratech.github.io/ferrovec/)
* [OrdoFP 0.1.0 lanzado — un conjunto de herramientas de programación funcional para Rust (HList, clases tipo GAT, óptica, efectos, transformadores mónadas)](https://github.com/ordokr/ordofp/releases/tag/v0.1.0)
* [Freya 0,4](https://freyaui.dev/posts/0.4)
* [Línea de construcción: Fusión de Carga y Perfil de Construcción de Ninja en una sola línea temporal](https://dev.to/nabsei/buildline-merging-cargo-and-ninjas-build-profiling-into-one-timeline-2373)
* [cochlea 0.3.0: lectura de melodía, timbre MFCC, un limitador maestro e importación MIDI para el motor de audio agente determinista](https://richer-richard.github.io/cochlea/determinism.html#030-additions-2026-07-22)
* [flodl 0.6.0: DDP heterogéneo multi-host - GPUs desajustadas entre hosts superan a la tarjeta más rápida por sí solas](https://flodl.dev/blog/then-the-cpu-died)
* [hwatu: un navegador WebKitGTK basado en demonios para tileado de WMs con ~13ms de aparición de ventanas](https://hongnoul.github.io/hwatu/)
* [Kache 0.11.0: Cobertura más amplia del compilador y claves compatibles con LIBC](https://github.com/kunobi-ninja/kache/releases/tag/v0.11.0)
* ['trazado-recarga' - capa de recarga sin pánico](https://mladedav.github.io/blog/blog/tracing-reload/)
* [Presentando OpenTypeless: Entrada de Voz que Realmente Funciona](https://www.opentypeless.com/en/blog/introducing-talkmore)
* [Leyendo las capacidades de una caja de Rust a partir de sus símbolos compilados](https://dev.to/booyaka101/reading-a-rust-crates-capabilities-out-of-its-compiled-symbols-58pb)

### Observaciones/Pensamientos
* [Batecos: Hablemos de cajas, cariño](https://smallcultfollowing.com/babysteps/blog/2026/07/15/battery-packs/)
* [cláusulas de captura como efectos](https://blog.yoshuawuyts.com/capture-clauses-as-effects)
* [Código de Endurecimiento de Rust para Producción](https://corrode.dev/blog/hardening-rust/)
* [Tokio da progreso, no orden: programación de 1M de tareas](https://pranitha.dev/posts/tokio-gives-progress-not-ordering/)
* [Endurecimiento de servicio contra Rust y lista de verificación de producción](https://kerkour.com/rust-service-hardening-and-production-checklist)
* [audio] [The Rust Foundation con Rebecca Rumbul, Lori Lorusso y David Wood, dirección y consejo de la Rust Foundation](https://corrode.dev/podcast/s06e08-rust-foundation/)
* [vídeo] [Jon Gjengset: Mantenimiento de código abierto 2026-07-18](https://www.youtube.com/watch?v=bAINppA0BSU)
* [vídeo] [Registro de cambios de lanzamiento de Rust - 1.97.0](https://www.youtube.com/watch?v=lUoQ3uGSQA0)
* [vídeo] [Retransmisión en directo: Rust en Ubuntu](https://www.youtube.com/live/Doqwh1b4QyA)

### Guías de Rust
* [He encadenado el registro de auditoría de mi agente. Luego encontré 13 fracturas en él — todas mías, todas benignas.](https://kriyanative.com/blog/13-chain-breaks/)
* [Dos bichos complicados en un demonio de Rust](https://dev.to/scripthpp/two-bugs-i-only-found-by-running-my-rust-sync-daemon-against-real-infrastructure-4278)
* [vídeo] [Conceptos de backend en Rust: Gestión segura de los secretos de la aplicación](https://www.youtube.com/watch?v=u91eX3J6lPU)
* [vídeo] [Build con Naz - Ep 21: Matrices planas 2D de alto rendimiento en Rust (SIMD, caché L1)](https://www.youtube.com/watch?v=tIrSvJFRxAg)

## Crate de la semana

El crate de esta semana es [xan](https://github.com/medialab/xan), un kit de herramientas TUI para trabajar con archivos CSV.

¡Gracias a [Simeon H.K. Fitch](https://users.rust-lang.org/t/crate-of-the-week/2704/1630) por la sugerencia!

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

576 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-14..2026-07-21

#### Compilador
* [cuenta los cierres asíncronos al apuntar a la vida útil en el tipo de retorno](https://github.com/rust-lang/rust/pull/159256)
* [comptime inherente implica](https://github.com/rust-lang/rust/pull/157824)
* ['dep_graph': lecturas de tareas de deduplicación con un grabador de índice filtrado por época](https://github.com/rust-lang/rust/pull/159115)
* [comprueba con entusiasmo ambigüedades en el análisis macro](https://github.com/rust-lang/rust/pull/158976)
* [implementa el atributo '#[diagnostic::opaque]' para ocultar retrocesos de macros](https://github.com/rust-lang/rust/pull/158608)
* [shrink 'ast::Expr64'](https://github.com/rust-lang/rust/pull/158720)

#### Biblioteca
* [añadir expl 'Iterator::count' impl para 'str::EncodeUtf16'](https://github.com/rust-lang/rust/pull/159467)
* [implementa 'bool::toggle'](https://github.com/rust-lang/rust/pull/159296)
* [implementa 'const_binary_search'](https://github.com/rust-lang/rust/pull/159528)
* [implementa ayudantes de 'Depurar' mediante 'Cell'](https://github.com/rust-lang/rust/pull/159302)
* [implementa 'VecDeque::truncate_to_range'](https://github.com/rust-lang/rust/pull/156220)
* [¡haz un alfiler! ()' más infalible](https://github.com/rust-lang/rust/pull/158061)
* [mover 'std::io::BufRead' a 'alloc::io'](https://github.com/rust-lang/rust/pull/158546)
* [mover 'std::io::Read' a 'alloc::io'](https://github.com/rust-lang/rust/pull/158544)
* [mover 'std::io::read_to_string' a 'alloc::io'](https://github.com/rust-lang/rust/pull/158545)

#### Carga
* [usar PGO para carga](https://github.com/rust-lang/rust/pull/159149)
* ['tiempos': solo reporta unidades que la cola de trabajos realmente ejecutó](https://github.com/rust-lang/cargo/pull/17238)
* [no incluir deps proc-macro en los args de ruta de búsqueda de rustc](https://github.com/rust-lang/cargo/pull/17236)
* [incluir salidas SBOM en huellas dactilares](https://github.com/rust-lang/cargo/pull/17216)
* [inicializar perezosamente transportes de búsqueda git2](https://github.com/rust-lang/cargo/pull/17226)

#### Rustdoc
* [corregir la normalización automática de rasgos](https://github.com/rust-lang/rust/pull/159194)
* [usar PGO para rustdoc](https://github.com/rust-lang/rust/pull/159091)

#### Clippy
* [añadir pelusa de 'block_scrutinee'](https://github.com/rust-lang/rust-clippy/pull/16855)
* [evitar sugerencias inválidas de 'ref_as_ptr' en inicializadores const/estáticos](https://github.com/rust-lang/rust-clippy/pull/17415)
* [detectar '== 0' en tipos sin signo como un límite inferior 'manual_clamp'](https://github.com/rust-lang/rust-clippy/pull/16800)
* [corregir 'if_not_else' linting en condiciones macro expandidas](https://github.com/rust-lang/rust-clippy/pull/17405)
* [corregir 'needless_collect' sugiere una sugerencia que no puede ser mecanografiada](https://github.com/rust-lang/rust-clippy/pull/17383)
* ['non_zero_suggestions': no lint con signo entero div/rem como NonZero](https://github.com/rust-lang/rust-clippy/pull/17385)
* ['manual_filter': no comas los comentarios en la sugerencia 'and_then'](https://github.com/rust-lang/rust-clippy/pull/17377)
* [requieren el uso de 'como _' para los rasgos usados indirectamente en fuentes de clippy](https://github.com/rust-lang/rust-clippy/pull/17369)
* [reescritura de 'min_ident_chars'](https://github.com/rust-lang/rust-clippy/pull/17362)
* [usar la determinación '#[must_use]' del compilador](https://github.com/rust-lang/rust-clippy/pull/16633)

#### Analizador de Rust
* [evitar el pánico del índice cuando la lista de flycheck está vacía](https://github.com/rust-lang/rust-analyzer/pull/22634)
* [añadir pistas de captura a las corutinas](https://github.com/rust-lang/rust-analyzer/pull/22811)
* [añadir manejador para E0572](https://github.com/rust-lang/rust-analyzer/pull/22813)
* [no se supone que las asignaciones de desestructuración de arrays con patrón reposo son de tamaño constante](https://github.com/rust-lang/rust-analyzer/pull/22483)
* [normalizan con entusiasmo 'IntoFuture::Output' de '.await'](https://github.com/rust-lang/rust-analyzer/pull/22852)
* [activar inferencia automática de rasgos](https://github.com/rust-lang/rust-analyzer/pull/22791)
* [extraer variable preservando el espacio en blanco de la entrada de macro](https://github.com/rust-lang/rust-analyzer/pull/22792)
* [corregir corutinas que no registran correctamente a los propietarios de la vinculación](https://github.com/rust-lang/rust-analyzer/pull/22832)
* [corrección se cierra en asistencias debido a llamadas '.unwrap()' en SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/22759)
* [corregir 'hir' variables ligadas de la caja que filtra desde carpetas saltadas](https://github.com/rust-lang/rust-analyzer/pull/22810)
* [corregir 'InferenceContext:identity_args' usando el DefID incorrecto](https://github.com/rust-lang/rust-analyzer/pull/22855)
* [arreglar sintaxis del puente de pánico al derramar flotación](https://github.com/rust-lang/rust-analyzer/pull/22849)
* [manejar variantes 'enum' en los 'genéricos' del siguiente solucionador](https://github.com/rust-lang/rust-analyzer/pull/22857)
* [implementar reducción de HRTB](https://github.com/rust-lang/rust-analyzer/pull/22818)
* ['pattern_matching_variant' inválido bajando debido a la recuperación](https://github.com/rust-lang/rust-analyzer/pull/22789)
* [fusionar 'WherePredicate::ForLifetimes' con 'WherePredicate::TypeBound'](https://github.com/rust-lang/rust-analyzer/pull/22867)
* [solo escribe anon const ty en el resultado de inferencia de padre si no tiene su propia inferencia](https://github.com/rust-lang/rust-analyzer/pull/22804)
* [pánico con un elemento de función y un objeto de macro de activación con nombre duplicado](https://github.com/rust-lang/rust-analyzer/pull/22822)
* [analizador a error en el límite de tipo de macro](https://github.com/rust-lang/rust-analyzer/pull/22827)
* [activan servidores de macro en solicitudes que borran la caché del cliente](https://github.com/rust-lang/rust-analyzer/pull/22865)
* [usa cita! dentro de 'ast::make::expr_call()'](https://github.com/rust-lang/rust-analyzer/pull/22782)
* [usar 'Result' para el tipo de carga útil 'Response' del servidor lsp](https://github.com/rust-lang/rust-analyzer/pull/22793)
* [grabar expresiones en tipos en 'ExprScope'](https://github.com/rust-lang/rust-analyzer/pull/22861)

### Triaje de rendimiento del compilador Rust

Los dos cambios más destacados esta semana fueron [#159115](https://github.com/rust-lang/rust/pull/159115),
Lo que resultó en victorias bastante buenas en el conteo de instrucciones para builds incrementales completas en varios benchmarks,
y [#159091](https://github.com/rust-lang/rust/pull/159091), que habilitó PGO para rustdoc, que
Lo hace ~3-4% más rápido en general.

Había dos grandes rollups con pequeñas regresiones de rendimiento, lo que dificultaba localizarlo
los PRs ofensivos.

Triaje hecho por **@Kobzol**.
Rango de revisión: [5503df87.. d527BC9B](https://perf.rust-lang.org/?start=5503df87342a73d0c29126a7e08dc9c1255c46ad&end=d527bc9bfa297ca7fd7f5ae93781eeec42073170&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,2%, 1,0%] | 40 |
| Regresiones ❌ <br /> (secundario) | 0,7% | [0,2%, 4,6%] | 69 |
| Mejoras ✅ <br /> (primaria) | -2,0% | [-6,2%, -0,2%] | 136 |
| Mejoras ✅ <br /> (secundario) | -2,6% | [-8,4%, -0,2%] | 119 |
| Todos ❌✅ (primario) | -1,4% | [-6,2%, 1,0%] | 176 |

2 regresiones, 3 mejoras, 6 mixtas; 4 de ellos en rollups
34 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/189822607d8d09acd85c234b2c245e817591ca67/triage/2026/2026-07-21.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Problema de seguimiento para 'bool::toggle'](https://github.com/rust-lang/rust/issues/159298)
* [Problema de seguimiento para vec_try_remove](https://github.com/rust-lang/rust/issues/146954)
* [Evitar calcular la disposición de enums con discriminantes no int](https://github.com/rust-lang/rust/pull/157562)
* [Problema de seguimiento para const_btree_len](https://github.com/rust-lang/rust/issues/71835)
* [Añadir pelusa de 'raw_borrows_via_references'](https://github.com/rust-lang/rust/pull/138230)
* [estabilizar size_of_val_raw, align_of_val_raw, Disposición::for_value_raw](https://github.com/rust-lang/rust/pull/157572)
* [rustc_passes: atributos '#[path]' de pelusas no utilizados en módulos en línea](https://github.com/rust-lang/rust/pull/158835)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Emitir 'nota' al llamar 'rustc' sin especificar una edición](https://github.com/rust-lang/compiler-team/issues/1019)
* [Deja que el sistema operativo gestione el crecimiento de la pila](https://github.com/rust-lang/compiler-team/issues/1011)
* [Añadir 'target_feature_available_at_call_site'](https://github.com/rust-lang/compiler-team/issues/1010)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Deallocate fondos posteriores a 2026 de PM y compiladores](https://github.com/rust-lang/leadership-council/pull/314)

##### [Directrices del Código de Peligros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [¿Tienen que permanecer los bytes de un puntero en el mismo orden?](https://github.com/rust-lang/unsafe-code-guidelines/issues/558)
  
*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
  [RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Refactorizar el equipo liberal](https://github.com/rust-lang/rfcs/pull/3984)

## Próximos eventos

Eventos Rusty entre 2026-07-22 - 2026-08-19 🦀

### Virtual
* 2026-07-24 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/hd8mlw56)
* 2026-07-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254777/)
* 2026-07-28 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/315279653/)
* 30-07-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-07-31 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/uo5ek1f4)
* 2026-08-01 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-08-02 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-04 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-08-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-07 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/ii2jrwva)
* 2026-08-11 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-13 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-08-18 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/315604176/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)

### África
* 2026-08-11 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-07-25 | Mumbai, IN | [Rust Mumbai](https://luma.com/mumbai)
    * [**Rust Mumbai — Encuentro 🦀 de julio **](https://luma.com/7ksabwbm/)
* 2026-07-26 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**Rust Pune: julio 2026**](https://www.meetup.com/rust-pune/events/315651505/)

### Europa
* 2026-07-23 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**LDN habla: Adquisición de Antítesis en julio de 2026 (https://www.meetup.com/rust-london-user-group/events/315612916/)
* 2026-07-23 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework para Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 2026-07-25 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #28**](https://www.meetup.com/stockholm-rust/events/315749994/)
* 27-07-2026 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #20: Julian Dickert - Seguridad de la cadena de suministro en Rust: Evaluando cajas para producción**](https://rust-augsburg.github.io/meetup/Meetup_20.html)
* 2026-07-29 | Polonia, PL | [Polonia Oxidada](https://www.meetup.com/rust-poland-meetup)
    * [**Polonia oxidada x Cracovia #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 30-07-2026 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #70**](https://www.meetup.com/copenhagen-rust-community/events/315767999/)
* 30-07-2026 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Julio de Rust Manchester**](https://www.meetup.com/rust-manchester/events/315037685/)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Confía pero verifica el LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)

### Norteamérica
* 222-07-2026 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 222-07-2026 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: ¡Rust en sistemas distribuidos con ciencia del vuelo!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 222-07-2026 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Escribe un agente de codificación personalizado y wasm_zero**](https://www.meetup.com/rust-nyc/events/315636854/)
* 2026-07-23 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315418155/)
* 2026-07-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Porter Square Rust, 25 de julio**](https://www.meetup.com/bostonrust/events/315582650/)
* 2026-07-25 | Brooklyn, NY, EE. UU. | [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)
* 30-07-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Chinatown, 1 de agosto**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Encuentro nocturno de Boston Rust en Red Hat, 4 de agosto**](https://www.meetup.com/bostonrust/events/314660176/)
* 2026-08-06 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Envío temporal: Cómo un ecosistema global de Rust construyó la última API web de Chrome**](https://www.meetup.com/stl-rust/events/314701905/)
* 2026-08-13 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de agosto de Utah Rust**](https://www.meetup.com/utah-rust/events/314696652/)
* 2026-08-13 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust August Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601099/)
* 2026-08-15 | San Francisco, CA, EE. UU. [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 3**](https://partiful.com/e/juWAwRs3XMWP7s9wLNWK)
* 2026-08-18 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997215/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)

### Oceanía
* 2026-07-23 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**¡Rust Perth: Encuentro de julio!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)
* 30-07-2026 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
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

> Teníamos pensado publicar una entrada en el blog anunciando esto al mismo tiempo que hacíamos público el repositorio, pero se nos quedó sin uso 😭 de CI de repositorios privados.

– [Carl Lerche en r/rust](https://www.reddit.com/r/rust/comments/1uzknzl/tokiorstopcoat_a_batteriesincluded_framework_for/oy8k2nn/) sobre el lanzamiento de la capa superior

A pesar de la lamentable falta de sugerencias, llogiq se alegra de haber encontrado esta cita.

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1v41dgv/this_week_in_rust_661/)</small>
