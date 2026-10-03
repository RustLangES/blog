---
title: "Esta semana en Rust #128"
number_of_week: 128
description: El crate de esta semana es ying-profiler, un perfilador nativo de memoria de muestreo de Rust. 
date: 2026-09-30
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

* [Scientific Computing in Rust #22 (septiembre de 2026)](https://scientificcomputing.rs/monthly/2026-09)
* [El Rustacean Incrustado Número #81](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-81)

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [¿Puede Rust seguro vencer alguna vez al C Brotli de Google?](https://mnwa.hashnode.dev/can-safe-rust-ever-beat-google-s-c-brotli)
* [Construcción de un controlador basado en DMA para el RP2350 I2C (seguridad no incluida)](https://micro-rust.github.io/posts/001-i2c-dma-handler/)
* [Cuerpos blandos avanzados para juegos con el motor de física Rapier](https://dimforge.com/blog/2026/09/25/advanced-soft-bodies-for-games-in-the-rapier-physics-engine/)
* [Apoyando a Rust en Trabajadores nativos con el nuevo objetivo de Emscripten para wasm-bindgen](https://blog.cloudflare.com/rust-workers-emscripten-target/)
* [Pensamientos oxidados sobre "Analizar, no validar"](https://eli.thegreenplace.net/2026/rusty-thoughts-on-parse-don't-validate/)
* [El estado de SIMD en Rust en 2026](https://shnatsel.github.io/state-of-simd-rust-2026/)
* [¿Cómo dejas de ser un novato en Rust?](https://www.jochen.fyi/posts/how-do-you-stop-being-a-rust-novice)
* [Tenemos Argumentos Nombrados en Casa](https://corrode.dev/blog/named-arguments-at-home/): una respuesta a la entrada del blog *Discutiendo sobre los argumentos* mencionada en el número anterior
* [Informe de mantenimiento de Rust aguas arriba (agosto-septiembre 2026)](https://kobzol.github.io/rust/2026/09/30/stf-august-september-2026.html)
* [¿Rust en el núcleo? ¿Y el Rust sin el núcleo!](https://kerkour.com/rust-kernel)
* [Compilación del núcleo con gccrs](https://lwn.net/SubscriberLink/1095553/7f34252658f8b8d1/)
* [Escuchando la radio con Rust](https://lwn.net/SubscriberLink/1095721/e1d863e5fd827753/)
* [Soporte nativo para Rust en la GPU](https://lwn.net/SubscriberLink/1095731/a5ecc9da2388b8ec/)

### Guías de Rust

* [Cómo acelerar el compilador Rust en septiembre de 2026](https://nnethercote.github.io/2026/09/30/how-to-speed-up-the-rust-compiler-in-september-2026.html)
* [Creando notificaciones en tiempo real con SSE y Pub/Sub](https://oluseun.dev/blogs/real-time-notifications-sse-pubsub.html)
* [Hilos Verdes desde cero](https://dzania.github.io/green-threads-from-scratch/)
* [Un tipo más fuerte que la suma de sus componentes](https://www.schneems.com/2026/09/24/a-type-stronger-than-the-sum-of-its-components/)
* [Deser: Repensando la serialización de Rust](https://lucumr.pocoo.org/2026/9/29/deser/)
* [Dejando caer a Swift de nuestras cajas de Bevy iOS](https://rustunit.com/blog/2026/09-04-bevy-ios-crates-objc2/)
* [Anhelando el Arco Descendiendo en Rust](https://wolfgirl.dev/blog/2026-09-29-pining-for-arc-downcasting-in-rust/)
* [Topcoat está empujando los límites de las aplicaciones de servidor con Rust](https://tokio.rs/blog/2026-09-24-topcoat-server-applications)
* [Préstamo de Rust, Aliasing y Referencias Mutables](https://developerlife.com/2026/09/25/rust-reborrowing/)
* [Una introducción muy condensada de los conceptos básicos de Rust](https://liw.fi/distilled-rust/)
* [vídeo] [Cómo hacer que nuestra app GPUI sea interactiva con el estado y los eventos](https://youtu.be/bs8bpAZ10SM)
* [ES] [Domain–Flow–Effects (DFE): una arquitectura diseñada para Rust](https://codigolinea.com/domain-flow-effects-dfe-arquitectura-rust/)

### Miscelánea

* [DE][Evento Comunitario Rust & Linux – 20–21 de noviembre de 2026 @ TUXEDO, Augsburg – Ayúdanos a elegir el tema del taller](https://cryptpad.fr/form/#/2/form/view/ppm1DazKFfZxfLB8Fw6-7W0vBC1kFV9KgkHQWqY7UU0/)

## Crate de la semana

El crate de esta semana es [ying-profiler](https://github.com/velvia/ying-profiler), un perfilador nativo de memoria de muestreo de Rust. 

¡Gracias a [Evan Chan](https://users.rust-lang.org/t/crate-of-the-week/2704/1683) por la autosugerencia! 

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
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
* [Unsynced - Falla el frontend Strace en Pwritev2 con offset -1 (desplazamiento actual del archivo)](https://github.com/zaydmulani09/unsynced/issues/1)
* [sin sincronizar - Enlaces duros del modelo (enlace/linkat) en lugar de advertencia](https://github.com/zaydmulani09/unsynced/issues/2)
* [unsynced - Añadir un ext4 data=perfil de persistencia de escritura](https://github.com/zaydmulani09/unsynced/issues/3)
* [MemoryWhale - Cubrir errores amigables para argumentos incompletos de 'mw'](https://github.com/wuisabel-gif/MemWhale/issues/253)
* [MemoryWhale - Bloquear la salida 'mw --help'](https://github.com/wuisabel-gif/MemWhale/issues/254)
* [dataprof - Los mensajes de rechazo remoto de Parquet deberían indicar que hay que descargar el archivo cuando el servidor ignore el rango](https://github.com/AndreaBozzo/dataprof/issues/840)
* [dataprof - La documentación de 'ScoreBounds::d imension_scores' sigue listando los conteos estimados de claves como ilimitados](https://github.com/AndreaBozzo/dataprof/issues/827)
* [dataprof - El evento de progreso 'terminado' subcuenta las filas cuando un límite de filas detiene el motor incremental](https://github.com/AndreaBozzo/dataprof/issues/824)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

546 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-22..2026-09-29

#### Compilador
* [calculando 'crate_hash' a partir de la codificación de metadatos en lugar de HIR (implementa #94878)](https://github.com/rust-lang/rust/pull/154724)
* [detectar ausencia en la declaración let](https://github.com/rust-lang/rust/pull/156949)
* [devolver el noalias a las referencias en closures](https://github.com/rust-lang/rust/pull/162361)
* [implementar palabras clave forzadas ('k#')](https://github.com/rust-lang/rust/pull/161775)
* [usar SmallVec en LocalizedConstraintGraph](https://github.com/rust-lang/rust/pull/163190)

#### Biblioteca
* [añade 'Div' y 'Mul' para 'Complejo<{float}>'](https://github.com/rust-lang/rust/pull/162832)
* [alloc: estabilizar 'Allocator'](https://github.com/rust-lang/rust/pull/156882)
* [conversiones adicionales 'NonZero'](https://github.com/rust-lang/rust/pull/129036)
* [permitir vidas elididas (''estáticas') en 'thread_local!'](https://github.com/rust-lang/rust/pull/159564)
* [implementa 'ParcialEq<VecDeque<U>>' para 'Vec<T>', '&[T]', '&mut [T]', '[T; N]', '&[T; N]' y '&mut [T; N]'](https://github.com/rust-lang/rust/pull/152972)
* [haz que dejar caer un BTreeMap vacío sea gratis](https://github.com/rust-lang/rust/pull/161791)
* [estabilizar SyncView](https://github.com/rust-lang/rust/pull/163366)
* [estabilizar 'Box::take'](https://github.com/rust-lang/rust/pull/160436)
* [estabilizar 'Resultado::into_{ok,err}'](https://github.com/rust-lang/rust/pull/161712)
* [estabilizar 'funnel_shifts' (incluyendo 'const')](https://github.com/rust-lang/rust/pull/161015)
* [estabilizar 'mem::conjure_zst'](https://github.com/rust-lang/rust/pull/161710)
* [estabilizar 'vec_try_remove'](https://github.com/rust-lang/rust/pull/163459)
* [usar aritmética de envolver en 'from_str_radix'](https://github.com/rust-lang/rust/pull/163099)

#### Carga
* ['builtin-deps': añadir dependencias incorporadas sintaxis manifestada](https://github.com/rust-lang/cargo/pull/17498)
* ['config': añadir build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* ['metadatos': características de paquete espejo en 'features_v2'](https://github.com/rust-lang/cargo/pull/17517)
* ['builtin-deps': corregir la validación manifesta de dependencias incorporadas](https://github.com/rust-lang/cargo/pull/17499)
* ['diag': No reportar las dependencias normales no utilizadas cuando se saltan las bibliotecas estáticas](https://github.com/rust-lang/cargo/pull/17515)
* ['package': preservar metadatos de características en manifiestos normalizados](https://github.com/rust-lang/cargo/pull/17509)

#### Rustdoc
* [comprueba correctamente que un elemento no es 'doc(oculto)' con '--generar-enlace-a-definición'](https://github.com/rust-lang/rust/pull/163268)
* [corregir resolución de enlace intra doc cuando un comentario doc está compuesto tanto por comentarios internos como externos](https://github.com/rust-lang/rust/pull/162862)
* [corregir salto inválido a def link cuando está involucrado '#[rustc_allow_incoherent_impl]'](https://github.com/rust-lang/rust/pull/163133)
* [corregir la denominación cuadrática de enlaces duplicados en la barra lateral](https://github.com/rust-lang/rust/pull/162976)

#### Clippy
* ['while_let_loop': detectar el patrón cuando el bucle tiene una etiqueta](https://github.com/rust-lang/rust-clippy/pull/17614)
* [añadir nueva pelusa de 'try_from_instead_of_from_str'](https://github.com/rust-lang/rust-clippy/pull/17030)
* [no sugieres 'Box::leak' en 'nonnull_unchecked_on_box_ptr'](https://github.com/rust-lang/rust-clippy/pull/17752)
* [fijar 'collapsible_match' consumiendo/comprobando mutaciones](https://github.com/rust-lang/rust-clippy/pull/16951)
* [corregir la fuerza que coincide con 'match_str_case' dentro o patrones](https://github.com/rust-lang/rust-clippy/pull/17759)
* [mejora el seguimiento de DOC ATTR SPAN para proc-macro](https://github.com/rust-lang/rust-clippy/pull/17678)

#### Analizador de Rust
* [no fallar la activación de la extensión cuando el servidor falla al iniciar](https://github.com/rust-lang/rust-analyzer/pull/23386)
* [añadir 'type_match' relevancia para alias de tipo](https://github.com/rust-lang/rust-analyzer/pull/23406)
* [coerción segura de FN a insegura fn](https://github.com/rust-lang/rust-analyzer/pull/23416)
* ['falso' completo en el atributo cfg](https://github.com/rust-lang/rust-analyzer/pull/23382)
* [valor completo de atracción dentro de la cadena sin comillas](https://github.com/rust-lang/rust-analyzer/pull/23414)
* [elenco de evaluación constante de una sola variante 'enum'](https://github.com/rust-lang/rust-analyzer/pull/23185)
* [deduplicar macro de atributos 'derive' y 'test'](https://github.com/rust-lang/rust-analyzer/pull/23413)
* [no escribir tipo desconocido](https://github.com/rust-lang/rust-analyzer/pull/23408)
* [no borrar la caché de tokens semánticos al actualizar](https://github.com/rust-lang/rust-analyzer/pull/23410)
* [hover show impl encabezado cuando impl con rasgo](https://github.com/rust-lang/rust-analyzer/pull/23365)
* [pánico en cierres asíncronos con límites de rasgos de rango superior](https://github.com/rust-lang/rust-analyzer/pull/23409)
* [devolver UB en lugar de entrar en pánico al leer el discriminante de un 'enum' deshabitado](https://github.com/rust-lang/rust-analyzer/pull/23421)

### Triaje de rendimiento del compilador Rust

Esta semana fue bastante positiva. No tuvimos regresiones puras, y la mayoría de los resultados provienen de algunas mejoras arquitectónicas con un impacto mixto o mayormente positivo. Algunas mejoras también provienen de abordar regresiones previamente clasificadas causadas por la falta de anotación de no_alias para referencias en cierres. 

La mayor mejora de esta semana es en rustdoc, al abordar el comportamiento cuadrático al generar enlaces en la barra lateral. Esto fue reportado por un usuario, pero el efecto no apareció en nuestros benchmarks, así que añadimos una prueba de estrés especial para ello. 

Triaje hecho por **@panstromek**. 
Rango de revisión: [3670d253.. c1070d69](https://perf.rust-lang.org/?start=3670d2532bdf51abbe0b8fea22284d7ca340ffe3&end=c1070d69382b8d2f2eb65119c738a77d9e324c9e&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,6% | [0,2%, 0,8%] | 8 |
| Regresiones ❌ <br /> (secundario) | 1,4% | [0,1%, 5,8%] | 30 |
| Mejoras ✅ <br /> (primaria) | -0,6% | [-1,7%, -0,2%] | 192 |
| Mejoras ✅ <br /> (secundario) | -1,5% | [-82,5%, -0,1%] | 101 |
| Todos ❌✅ (primario) | -0,6% | [-1,7%, 0,8%] | 200 |


0 regresiones, 2 mejoras, 6 mixtas; 3 de ellas en rollups
26 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/7409c0adce96db29bbfa5030136401590f768577/triage/2026/2026-09-29.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Deja de usar dlltool para generar bibliotecas de importación en MinGW](https://github.com/rust-lang/rust/pull/157712)
* [Estabilizar 'ptr::try_cast_aligned'](https://github.com/rust-lang/rust/pull/154170)
* [disposición: cierre] [1.99 Regresión beta del cráter: desbordamiento evaluando el requisito](https://github.com/rust-lang/rust/issues/161916)
* [implementa FCW para los elementos 'rustc_allowed_through_unstable_modules'](https://github.com/rust-lang/rust/pull/163161)
* [corregir 'VisibleForLeakCheck' en el camino rápido 'RegionOutlives'](https://github.com/rust-lang/rust/pull/163267)
* [Estabilizar 'debug_closure_helpers'](https://github.com/rust-lang/rust/pull/146099)
* [ Soporte a rutas asociadas relativas a tipos en parámicas genéricas por defecto y tipos de parámetros const](https://github.com/rust-lang/rust/pull/161998)
* [FCW por '#[panic_handler]' en 'unsafe fn'.](https://github.com/rust-lang/rust/pull/162974)
* [Estabilizar el atributo 'optimizar'](https://github.com/rust-lang/rust/pull/157273)
* [Permitir que los tipos de operandos unarios se infieran más adelante](https://github.com/rust-lang/rust/pull/159744)
* [Dote - '#[inline(siempre)] + #[target_feature(habilitar = "....")]' #2](https://github.com/rust-lang/rust/pull/162460)
* [Problema de seguimiento para 'CStr::d isplay'](https://github.com/rust-lang/rust/issues/139984)
* [Rechazar sintácticamente listas de captura precisas entre paréntesis en tipos de objetos de rasgos desnudos ('(use<...>)+')](https://github.com/rust-lang/rust/pull/162652)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [tipo 'f16b'](https://github.com/rust-lang/rfcs/pull/3983)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [dote(recortes): estabilizar 'perfil.recortes'](https://github.com/rust-lang/cargo/pull/17488)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Crear nuevo objetivo de Nivel 3 para QTEE: 'aarch64-unknown-qtee'](https://github.com/rust-lang/compiler-team/issues/1038)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Haz que los métodos de rasgos sean llamables en contextos const, toma III](https://github.com/rust-lang/rfcs/pull/4008)

## Próximos eventos

Eventos Rusty entre el 30-09-2026 - el 28-10-2026 🦀

### Virtual
* 2026-09-30 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Segmentación e Introducción al Paginado**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-01 | Virtual | [Fundación Rust y JetBrains](https://rustfoundation.org/event/livestream-smarter-coding-agents-for-rust-with-symposium/)
    * [**Transmisión en directo: Agentes de codificación más inteligentes para Rust con el Simposio**](https://info.jetbrains.com/rustrover-livestream-october01-2026.html#form)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/yqxvguts)
* 2026-10-03 | Virtual (Ámsterdam, NL) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #14**](https://www.meetup.com/bevy-game-development/events/316736369/)
* 04-10-2026 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316134009/)
* 06-10-2026 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773044/)
* 07-10-2026 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-10 | Híbrido (Kuala Lumpur, Malasia) | [Reunión de Rust Malaysia](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup octubre 2026**](https://forms.gle/721DxqrPeHXY6omP9)
* 2026-10-13 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254772/)
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-18 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, EE.UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcnbbc/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-222 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-27 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Razonamiento con Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)

### Asia
* 2026-10-09 | Híbrido (Kuala Lumpur, MY) | [Encuentro de Rust Malaysia](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup agosto 2026**](https://forms.gle/721DxqrPeHXY6omP9)

### Europa
* 2026-09-30 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-09-30 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/316661690/)
* 01-10-2026 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin en localización 🏳️ 🌈 – Edición 018**](https://www.meetup.com/rust-berlin/events/316763107/)
* 2026-10-01 | Oxford, GB | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Rust incrustado para tontos**](https://www.meetup.com/oxford-rust-meetup-group/events/316708765/)
* 05-10-2026 | Múnich, DE | [Múnich Oxidado](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 08-10-2026 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Hack'n'Learn Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-08 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22ª sesión de bcnrust**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 20-10-2026 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)

### Norteamérica
* 2026-10-01 | Saint Louis, MO, EE. UU. | [STL Oxidación](https://www.meetup.com/stl-rust)
    * [**construyendo un contenedor mínimo y sin raíces en Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 03-10-2026 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 3 de octubre**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | Lehi, UT, EE.UU. [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Lightning habla y relajándose**](https://www.meetup.com/utah-rust/events/316708351/)
* 2026-10-08 | Nueva York, NY, EE.UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Pruebas de conocimiento cero y GPUs en renderizado**](https://www.meetup.com/rust-nyc/events/316698929/)
* 2026-10-08 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 10 de octubre**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Ángeles, CA, EE.UU. [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust con Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 20-10-2026 | San Francisco, CA, EE.UU. | [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Encuentro Incrustado**](https://luma.com/ur4pm34i)
* 2026-10-28 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316655403/)

### Sudamérica
* 2026-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**WebApp Ergonomics y Secretos Distribuidos.**](https://www.meetup.com/rust-argentina/events/316664266/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1wo5btb/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> La comunidad es insoportablemente servicial. Hice una pregunta sencilla en el servidor de Discord de la comunidad de Rust. ¿Cuál es la mejor manera de leer un archivo en Rust? Esperaba una respuesta directa. En su lugar, recibí un ensayo de 2.000 palabras sobre el funcionamiento interno de IO, buffering, manejo de errores y propiedad, además de enlaces a cuatro entradas diferentes del blog y tres enfoques distintos según el tamaño del archivo, y un ejemplo de código funcional. 
>
> ... 
>
> La comunidad Rust ha convertido la educación en mi contra. Ahora soy mejor ingeniero que ayer contra mi voluntad. 

– [tris en youtube](https://youtu.be/B2gmKy3pHkw?si=4QRLux5X55fTx8c8&t=196)

¡Gracias a [MusicalNinjaDad](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1806) por la sugerencia! 

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1wuo2o9/this_week_in_rust_671/)</small>