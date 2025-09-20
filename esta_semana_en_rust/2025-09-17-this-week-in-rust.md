---
title: "Esta semana en Rust #76"
number_of_week: 76
description: El crate de esta semana es asciinema, una conocida herramienta de línea de comandos para grabar, reproducir y transmitir sesiones de terminal recientemente reescritas en Rust.
date: 2025-09-17
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software confiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíenos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!--

Estimados colaboradores de la comunidad:
Lea README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe tener la siguiente forma:

* [Título de la página enlazada](https://example.com/my_article)

Si agrega un enlace a un contenido que no es de texto, prefije el prefijo '[video]' o '[audio]':

* [video] [Título del video vinculado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio vinculado](https://example.com/my_podcast)

Si no sabe qué categoría usar, no dude en enviar un PR de todos modos
y solo pida a los editores que seleccionen la categoría.

-->

### Oficial
* [crates.io campaña de phishing](https://blog.rust-lang.org/2025/09/12/crates-io-phishing-campaign/)

### Boletines
* [El Rustacean Incrustado Issue #54](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-54)
* [Rust Trends Issue #70: Movimientos de cimentación y avances en el rendimiento](https://rust-trends.com/newsletter/rust-foundation-performance-breakthroughs-2025)

### Actualizaciones de proyectos/herramientas
* [Cot v0.4: Particularmente perezoso](https://mackow.ski/blog/cot-v04-particularly-lazy/)
* [Anuncio de Diesel 2.3](https://diesel.rs/news/2_3_0_release.html)
* [Un adelanto de SeaORM 2.0](https://www.sea-ql.org/blog/2025-09-16-sea-orm-2.0/)
* [Anuncio de Pingoo: el equilibrador de carga / API Gateway rápido y seguro con descubrimiento de servicios y WAF integrados](https://kerkour.com/announcing-pingoo)
* [Rerun 0.25.0 - Resaltado de sintaxis, filtrado de tablas, objetos transparentes](https://github.com/rerun-io/rerun/releases/tag/0.25.0)
* [Introducción a Obelisco 0.24.1](https://obeli.sk/blog/introducing-obelisk-0-24-1/)
* [Presentación de CurveForge](https://smartnets.etrovub.be/posts/introducing-curveforge/)
* [Swiftide 0.31](https://blog.bosun.ai/swiftide-0-31/)
* [HotPath 0.2.5 - Un simple generador de perfiles de rendimiento y memoria](https://github.com/pawurb/hotpath)

### Observaciones/Pensamientos
* [La lógica basada en piratas de las referencias compartidas de Rust](http://ais523.me.uk/blog/logic-of-shared-references.html)
* [Depuración asíncrona integrada e inspección-embajada](https://tweedegolf.nl/en/blog/161/embedded-async-debugging-and-inspect-embassy)
* [Sé simple](https://corrode.dev/blog/simple/)
* [Por qué construimos nuestro propio analizador SQL desde cero: una historia de implementación de Rust](https://www.databend.com/blog/category-engineering/2025-09-10-query-parser/)
* [Comparación de la resolución de la versión de dependencia transitiva en Rust y Java](https://blog.frankel.ch/dependency-version-resolution-rust-java/)
* [Compensaciones en el diseño de DSL](https://forgestream.idverse.com/blog/20250916-dsl-trade-offs/)
* [Rust Algorithm Bites – Validación de un árbol de búsqueda binario](https://d34dl0ck.me/rust-algorithm-bites-validate-bst/index.html)
* [Rust en Android - Lecciones desde el borde](https://greptime.com/blogs/2025-04-14-rust-in-android-edge-based-practice)

### Tutoriales de Rust
* [Serie de backend de Axum: Docker, base de datos y agrupación de conexiones](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-database-setup-using-docker/)
* [Las reglas ocultas detrás de las funciones y cierres de Rust](https://blog.cuongle.dev/p/the-hidden-rules-behind-rust-functions)
* [Pruebas unitarias de Rust: código asíncrono](https://jorgeortiz.dev/posts/rust_unit_testing_async_code/)
* [Trazado de rayos en un fin de semana - En Rust](https://buttondown.com/dabeaz/archive/new-video-ray-tracing-in-one-weekend-in-rust/)
* [Nuevo esquema: cómo ahorrar $ 327.6 millones usando Rust](https://newschematic.org/blog/how-to-save-327-6-million-using-rust/)
* [serie] [Reloj en tiempo real - Libro de controladores integrados de Rust](https://red.implrust.com/rtc/index.html)

### Miscelánea
* [audio] [Netstack.FM — Un podcast sobre redes y Rust](https://netstack.fm/#episode-5)
* [video] [Entrevista de Jan David Nose, equipo de infraestructura de Rust (contenido del proyecto Rust @ RustConf 2025)](https://www.youtube.com/watch?v=r7i-2wHtNjw)
* [Informe de empleos de Rust de agosto de 2025](https://filtra.io/rust/jobs-report/aug-25)
* [La simbiosis de Rust y Arm: una conversación con David Wood](https://filtra.io/rust/interviews/arm-sep-25)

## Crate de la semana

El crate de esta semana es [asciinema](https://crates.io/crates/asciinema), una conocida herramienta de línea de comandos para grabar, reproducir y transmitir sesiones de terminal recientemente reescritas en Rust.

A pesar de la falta de sugerencias, llogiq está muy contento con su elección.

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háznoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Diésel - Inferir automáticamente las definiciones de enumeración](https://github.com/diesel-rs/diesel/issues/4759)
<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 379 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-09..2025-09-16

#### Compilador
* [implementar '#[rustc_align_static(N)]' en 'estáticos](https://github.com/rust-lang/rust/pull/146178)
* [mover más lints amortiguados tempranos a Dyn Lint Diagnostics](https://github.com/rust-lang/rust/pull/145881)
* [hacer que 'AssocItem' sea consciente de su tipo de implementación](https://github.com/rust-lang/rust/pull/145186)
* [Coincide con el ensamblaje 'va_arg' de Clang en objetivos de brazo](https://github.com/rust-lang/rust/pull/144549)
* [rechazar sufijos literales no válidos en la indexación de tuplas, la indexación de tuplas 'struct' y la posición del nombre de campo 'struct'](https://github.com/rust-lang/rust/pull/145463)
* [ordenar correctamente las sugerencias de implementación de rasgos de matriz](https://github.com/rust-lang/rust/pull/146403)
* [Strip Frontmatter en menos lugares](https://github.com/rust-lang/rust/pull/146340)
* [MIRI: Corregir la sincronización de release/scquire para cargas desde el búfer de almacenamiento](https://github.com/rust-lang/miri/pull/4577)
* [MIRI: Haz que un Hola Mundo Básico funcione en WASIP2](https://github.com/rust-lang/miri/pull/4582)

#### Biblioteca
* [constify Eq, Ord, PartialOrd](https://github.com/rust-lang/rust/pull/144847)
* [implemente 'Suma' y 'Producto' para 'f16' y 'f128'](https://github.com/rust-lang/rust/pull/146300)
* [incluido 'Rango': cambiar 'fin' por 'último'](https://github.com/rust-lang/rust/pull/144765)
* [hacer 'Barrera' 'RefUnwindSafe' de nuevo](https://github.com/rust-lang/rust/pull/146322)
* [estabilizar 'BTree{Map,Set}::extract_if'](https://github.com/rust-lang/rust/pull/145471)
* [admite literales enteros en '${concat()}'](https://github.com/rust-lang/rust/pull/146308)

#### Carga
* [cli: Permitir finalizaciones para nombres de subcomandos de terceros](https://github.com/rust-lang/cargo/pull/15961)
* [completer: Se ha añadido la finalización de la bandera '--features'](https://github.com/rust-lang/cargo/pull/15309)
* [completo: Mostrar cajas/características locales sobre otros miembros](https://github.com/rust-lang/cargo/pull/15956)
* [frontmatter: Pruebe vallas de código len alternativas](https://github.com/rust-lang/cargo/pull/15952)
* [manifiesto: Mostrar fuente de error a los usuarios](https://github.com/rust-lang/cargo/pull/15939)
* [publicar: Cambiar la línea 'ctrl-c en espera' a un mensaje de ayuda](https://github.com/rust-lang/cargo/pull/15942)

#### Rustdoc
* [Manejar correctamente la búsqueda literal en rutas](https://github.com/rust-lang/rust/pull/146448)

#### Clippy
* ['elidable_lifetime_names': evitar la superposición de tramos en las sugerencias](https://github.com/rust-lang/rust-clippy/pull/15667)
* ['len_zero': no llames ansiosamente a 'GenericArgs::type_at'](https://github.com/rust-lang/rust-clippy/pull/15660)
* ['multiple_unsafe_ops_per_block': ignorar las operaciones inseguras de '.await' desugaring](https://github.com/rust-lang/rust-clippy/pull/15654)
* ['needless_closure': no pelar en 'AsyncFn*'s](https://github.com/rust-lang/rust-clippy/pull/15649)
* ['needless_return': corregir el falso positivo con el código 'cfg'd después de la devolución](https://github.com/rust-lang/rust-clippy/pull/15669)
* ['ref_option': no pelar en macros externas y proc](https://github.com/rust-lang/rust-clippy/pull/15668)
* ['semicolon_inside_block': no pelar si el bloque está entre paréntesis](https://github.com/rust-lang/rust-clippy/pull/15626)
* ['use_self': no devuelve anticipadamente si el tipo externo no tiene vidas](https://github.com/rust-lang/rust-clippy/pull/15611)
* [añadir sugerencia a 'cast_sign_loss' y 'cast_possible_wrap' usando los métodos 'cast_{un,}signed()'](https://github.com/rust-lang/rust-clippy/pull/15384)
* [corregir 'as_underscore' para sugerir solo cuando sea sugestionable](https://github.com/rust-lang/rust-clippy/pull/15652)
* [corregir 'invalid_upcast_comparisons' macros mal destruídas](https://github.com/rust-lang/rust-clippy/pull/15663)
* [corregir 'useless_attribute' falso positivo en 'deprecated_in_future'](https://github.com/rust-lang/rust-clippy/pull/15645)
* [reconocer el patrón canónico '?' con 'Resultado'](https://github.com/rust-lang/rust-clippy/pull/15680)

#### Analizador de Rust
* [agregar más trucos de solución para diagnósticos de inicio incorrectos](https://github.com/rust-lang/rust-analyzer/pull/20402)
* [corregir 'LifetimeParam::lifetime_bounds' implemento no válido](https://github.com/rust-lang/rust-analyzer/pull/20624)
* [arreglar punto y coma adicional antes de else en let-stmt](https://github.com/rust-lang/rust-analyzer/pull/20657)
* [corregir sangría para correcciones 'unresolved_field'](https://github.com/rust-lang/rust-analyzer/pull/20613)
* [siempre coaccionar en un lanzamiento, incluso cuando hay tipos desconocidos](https://github.com/rust-lang/rust-analyzer/pull/20649)
* [no marque el tipo desconocido como implementando todos los rasgos notables](https://github.com/rust-lang/rust-analyzer/pull/20665)
* [no genere una lista de parámetros genéricos vacía en 'generate_function'](https://github.com/rust-lang/rust-analyzer/pull/20653)
* [no active dos flychecks al guardar archivos que forman parte de los objetivos](https://github.com/rust-lang/rust-analyzer/pull/20635)
* [corregir la expansión de macros recursivamente que no funcionaba correctamente para llamadas de macros anidadas](https://github.com/rust-lang/rust-analyzer/pull/20612)
* [corregir la normalización en el nuevo solucionador](https://github.com/rust-lang/rust-analyzer/pull/20647)
* [bucle infinito mientras se elaboran predicados](https://github.com/rust-lang/rust-analyzer/pull/20654)
* [hacer que '#[target_feature]' sea siempre seguro en WASM](https://github.com/rust-lang/rust-analyzer/pull/20642)
* [filtrado de cláusulas más preciso para 'explicit_*_predicates_of'](https://github.com/rust-lang/rust-analyzer/pull/20671)
* [solo calcule rutas inestables en cadenas de herramientas nocturnas para características IDE](https://github.com/rust-lang/rust-analyzer/pull/20517)
* [resolver rutas a bibliotecas de prueba de instantáneas absolutamente](https://github.com/rust-lang/rust-analyzer/pull/20639)
* [migrar 'InferenceTable' al siguiente solucionador](https://github.com/rust-lang/rust-analyzer/pull/20578)

### Triaje de rendimiento del compilador de Rust

Semana difícil de interpretar, porque un cambio positivo en [#145910](https://github.com/rust-lang/rust/pull/145910) funciona un poco peor en nuestros puntos de referencia de lo que lo haría en el mundo real. El resultado general probablemente siga siendo ligeramente negativo, porque hay más trabajo de características adicionales. Por otro lado, también tenemos una buena mejora en la reducción del número de dependencias de consulta en el sistema incremental del compilador en [#145186](https://github.com/rust-lang/rust/pull/145186).

Triaje realizado por **@panstromek**.
Rango de revisión: [f13ef0d7.. 52618eb3](https://perf.rust-lang.org/?start=f13ef0d75d834c826c9479a5d244bcfb9891df45&end=52618eb338609df44978b0ca4451ab7941fd1c7a&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,2%, 2,7%] | 72 |
| Regresiones ❌ <br /> (secundaria) | 0,7% | [0,0%, 3,5%] | 96 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-0,9%, -0,1%] | 10 |
| Mejoras ✅ <br /> (secundario) | -0,8% | [-2,9%, -0,1%] | 41 |
| Todos ❌✅ (primarios) | 0,4% | [-0,9%, 2,7%] | 82 |

1 Regresión, 1 Mejora, 6 Mixto; 3 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/888c0a24417c3883373ae0844f760f8300176b90/triage/2025/2025-09-15.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Permitir &raw '[mut | const]' para el campo de unión en código seguro](https://github.com/rust-lang/rust/pull/141469)
* [Denegar por defecto nunca escribir pelusas](https://github.com/rust-lang/rust/pull/146167)
* [Divida de manera oportunista '!=' para analizar correctamente nunca escribir](https://github.com/rust-lang/rust/pull/145536)
* [Permitir especificar varios límites para el mismo elemento asociado, excepto en objetos de característica](https://github.com/rust-lang/rust/pull/146593)
* [rustdoc: ocultar '#[repr]' si no es parte del ABI público](https://github.com/rust-lang/rust/pull/116882)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: '#[cfg(since(rust, "1.95"))]' para la compilación condicional de la versión de Rust](https://github.com/rust-lang/rfcs/pull/3857)
* [Aplicación de la mitigación](https://github.com/rust-lang/rfcs/pull/3855)
* [RFC para '#[stable(desde)]'](https://github.com/rust-lang/rfcs/pull/3854)

## Próximos eventos

Rusty Eventos entre 2025-09-17 - 2025-10-15 🦀

### Virtual
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)
* 2025-09-18 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-09-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361443)
* 2025-09-25 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-09-28 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311046301/)
* 2025-10-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)
* 2025-10-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763858627)
* 2025-10-05 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311062530/)
* 2025-10-07 | Virtual (Beijing, CN) | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Reunión mensual de la comunidad de WasmEdge, el tiempo de ejecución de LLM / AGI **](https://www.meetup.com/wasm-rust-meetup/events/310831771/)
* 2025-10-09 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/tsjcttyhcnbqb/)
* 2025-10-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)

### Asia
* 2025-09-17 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust septiembre de 2025 en Varonis en Herzeliya**](https://www.meetup.com/rust-tlv/events/310708628)
* 2025-10-02 | Seúl, KR | [Reunión de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310824483)
* 2025-10-04 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de octubre de 2025**](https://hasgeek.com/rustbangalore/october-2025-rustacean-meetup/)
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malasia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokio, JP | [Encuentro de Tokyo Rust](https://www.meetup.com/tokyo-rust-meetup/events/)
    * [**Creación de interfaces de usuario de terminal de bolsillo con Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)

### Europa
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)
* 2025-09-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charlas en Mjolner Informatics**](https://www.meetup.com/rust-aarhus/events/310562343)
* 2025-09-23 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #78**](https://www.meetup.com/rust-paris/events/310935603)
* 2025-09-24 | Gotemburgo, SE | [Rust, Göteborg](https://www.meetup.com/rustgbg/events/)
    * [**Rust Gbg — septiembre de 2025**](https://www.meetup.com/rustgbg/events/310866773)
* 2025-09-24 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3 - híbrido**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-09-25 | Augsburgo, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Reunión de Augsburg Rust #15**](https://rust-augsburg.github.io/meetup/Meetup_15.html)
* 2025-09-25 | Londres, Reino Unido | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Mujeres en Rust x Scala: Programación funcional en Rust & Streams con Aquascape**](https://www.meetup.com/women-in-rust/events/311056499/)
* 2025-09-27 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #18**](https://www.meetup.com/stockholm-rust/events/311027118/)
* 2025-09-30 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Septiembre 2025 Community Showcase**](https://www.meetup.com/rust-london-user-group/events/311070068/)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Encuentro de Rust Moravia (¡En la capital!)**](https://www.meetup.com/rust-moravia/events/310743282)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134)
* 2025-10-08 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #79**](https://www.meetup.com/rust-paris/events/310424476)
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944041)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)

### América del Norte
* 2025-09-18 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-20 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de la Universidad de Boston, 20 de septiembre **](https://www.meetup.com/bostonrust/events/311038454/)
* 2025-09-21 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/events/)
    * [**Detroit Rust Hacking Hangout**](https://www.meetup.com/detroitrust/events/311047467/)
* 2025-09-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-24 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)
* 2025-09-24 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Resumen de RustConf - ¡La primicia!**](https://www.meetup.com/chicago-rust-meetup/events/311006846)
* 2025-09-24 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Rust/Python Interop & DB Design**](https://www.meetup.com/rust-nyc/events/311006867/)
* 2025-09-25 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl en la taberna Manuels**](https://www.meetup.com/rust-atl/events/308675983)
* 2025-09-25 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Serie 3 : Final de la serie**](https://www.meetup.com/music-city-rust-developers/events/304333261/)
* 2025-09-27 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, 27 de septiembre **](https://www.meetup.com/bostonrust/events/311038485/)
* 2025-09-30 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Reunión nocturna de Boston Rust con Bevy e Isograph, 30 de septiembre **](https://www.meetup.com/bostonrust/events/310907806/)
* 2025-10-02 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [** 🚁 Rust en vuelo: lecciones del diseño de un cuadricóptero impreso en 3D con incrustación**](https://www.meetup.com/stl-rust/events/310279407)
* 2025-10-04 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**North End Rust Lunch, 4 de octubre **](https://www.meetup.com/bostonrust/events/310983705/)

### Oceanía:
* 2025-10-01 | Perth, WA, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> **Pregunta real:** ¿es una matriz una estructura/tupla, o es una enumeración?

– [Lokathor en github](https://github.com/rust-lang/rust/pull/146509#discussion_r2346807413)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1716) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1nkv205/this_week_in_rust_617/)</small>
