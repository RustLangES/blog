---
title: "Esta semana en Rust #7"
number_of_week: 7
description: El crate de esta semana es rocket, un marco web obstinado que pretende ser realmente ergonómico sin dejar de ser rápido.
date: 2023-11-22
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Oficial
* [Anunciando Rust 1.74.0](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)

### Actualizaciones de proyectos/herramientas
* [hyper v1](https://seanmonstar.com/blog/hyper-v1/)
* [Rocket v0.5: Estable, asíncrono, Centinelas, Streams, SSE, Formularios, WebSockets y mucho más](https://rocket.rs/v0.5/news/2023-11-17-version-0.5/)
* [GlueSQL v0.15 - Soporte para Python, Redis y CSV](https://github.com/gluesql/gluesql/releases/tag/v0.15.0)
* [Meilisearch lanza v1.5 - indexación más rápida, instantáneas bajo demanda e informes de Puffin](https://blog.meilisearch.com/meilisearch-1-5/)
* [Nutype v0.4.0 - newtype con restricciones](https://github.com/greyblake/nutype/releases/tag/v0.4.0)
* [Anuncio de open-ai-safe: un cliente de Rust de la API de OpenAI con seguridad de tipos](https://youtu.be/x11tBhokFNc)

### Observaciones/Pensamientos
* [Una JVM en Rust parte 8 - Retrospectiva](https://andreabergia.com/blog/2023/11/a-jvm-in-rust-part-8-retrospective/)
* [Rust 1.74.0: ¡Los 45 cambios en 19 minutos!](https://www.youtube.com/watch?v=MOzuShpnUm8)
* [Diversión con el análisis léxico y Rust](https://blog.blotato.com/fun-with-lexical-analysis-rust/)
* [Wasmtime y Cranelift en 2023](https://bytecodealliance.org/articles/wasmtime-and-cranelift-in-2023)
* [Señales vs. servidores](https://blog.adamchalmers.com/signals-vs-servers/)
* [Interposición de funciones en Rust con upgrayedd](https://blog.yossarian.net/2023/11/19/Function-interposition-in-Rust-with-upgrayedd)
* [Un encuentro cercano con el falso compartir](https://morestina.net/blog/1976/a-close-encounter-with-false-sharing)
* [Edge IoT con Rust en ESP: MQTT Publisher](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-mqtt-publisher)
* [Comprobación de semver en presencia de elementos doc(ocultos)](https://predr.ag/blog/checking-semver-for-doc-hidden-items/)
* [Empujar los "si" hacia arriba y los "fors" hacia abajo](https://matklad.github.io/2023/11/15/push-ifs-up-and-fors-down.html)
* [Construcción de troncos segmentados en Rust: ¡de la teoría a la producción!](https://arindas.github.io/blog/segmented-log-rust/)
* [Escribir una biblioteca genérica en tiempo de ejecución asíncrono](https://www.sea-ql.org/blog/2023-11-22-async-runtime-generic/)
* [Ferrostar: Creación de un SDK de navegación multiplataforma en Rust (Parte 1)](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-1/)

### Tutoriales de Rust
* [¡Así es como hice el tiempo de ejecución en el que se ejecuta este sitio web!](https://aandreba.com/article/this-is-how-i-made-the-runtime-this-website-runs-on)
* [video] [Tokenización y análisis sintáctico de un lenguaje de programación en Rust, por Adam Chalmers](https://www.youtube.com/watch?v=LUcI6KkM-PE)

### Miscelánea
* [Construyendo una mejor base para el futuro de Rocket](https://rocket.rs/v0.5/news/2023-11-17-rwf2-prelaunch/)
* [audio] [Rust Digger con Gabor Szabo](https://rustacean-station.org/episode/gabor-szabo/)
* [video] [Haciendo que el cliente BitTorrent (parcial) de Rust sea más razonable](https://www.youtube.com/watch?v=r0srf3kfZbs)
* [video] [Rust Release Train 1.74](https://www.youtube.com/watch?v=Ciuhriopc00)
* [video] [EuroRust 2023](https://www.youtube.com/playlist?list=PLH6-VpZ3SvUUKFSEPEWiHQi4JqebBj9Tq)

## Crate de la semana

El crate de esta semana es [rocket](https://rocket.rs), un marco web obstinado que pretende ser realmente ergonómico sin dejar de ser rápido.

¡Gracias a [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/1265) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [ Hyperswitch - [REFACTOR]: [Zen] Validación de metadatos MCA ](https://github.com/juspay/hyperswitch/issues/2913)
* [ Hyperswitch - [CARACTERÍSTICA] Estandarizar el campo de estado en la dirección de facturación y envío ](https://github.com/juspay/hyperswitch/issues/2939)
* [ Hyperswitch - [BUG]: Los errores de deserialización de metadatos MCA deben ser 4xx ](https://github.com/juspay/hyperswitch/issues/2899)
* [ Hyperswitch - [Característica]: [NMI] Sincronización con referencia de Hyperswitch](https://github.com/juspay/hyperswitch/issues/2905)
* [ Hyperswitch - [Característica]: [Zen] Sincronización con Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)
* [Ockam - La gestión de características 'Cargo.toml' debería permitir la construcción de cajas individuales con un conjunto de características predeterminado](https://github.com/build-trust/ockam/issues/5491)
* [Ockam - Mejorar el manejo de errores de múltiples llamadas 'ockam tcp-outlet create'](https://github.com/build-trust/ockam/issues/5897)
* [Ockam - Biblioteca - Adelgazar el 'NodeManagerWorker' para 'node / tcp'](https://github.com/build-trust/ockam/issues/6708)
* [Ockam - Biblioteca - Adelgazar el 'NodeManagerWorker' para 'nodo / credenciales'](https://github.com/build-trust/ockam/issues/6709)
* [Ockam - Comando - refactorizar para usar interfaces con tipo para implementar comandos para 'canal seguro' y 'oyente de canal seguro'](https://github.com/build-trust/ockam/issues/6699)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

369 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-13..2023-11-20

* [añadir objetivos arm64e-apple-ios y arm64e-apple-darwin](https://github.com/rust-lang/rust/pull/115526)
* [eliminar asmjs](https://github.com/rust-lang/rust/pull/117338)
* [add -Z 'llvm_module_flag'](https://github.com/rust-lang/rust/pull/116555)
* [añadir una estructura más rica para las proyecciones MIR estables](https://github.com/rust-lang/rust/pull/117517)
* [ajustar la IP del fotograma en las trazas inversas en relación con la base de la imagen para el objetivo SGX](https://github.com/rust-lang/rust/pull/117895)
* [siempre apunte al intervalo de índice en caso de error de obligación de índice](https://github.com/rust-lang/rust/pull/117856)
* [Evite iterar sobre mapas hash en astconv](https://github.com/rust-lang/rust/pull/117828)
* [compilar el cuerpo de la corrutina pre-co-co-transform en caso de error](https://github.com/rust-lang/rust/pull/117686)
* [garantizar la integridad de todas las ABI calculadas](https://github.com/rust-lang/rust/pull/117500)
* [Se corrigió la inserción de instrucciones que se ejecutarán a lo largo del borde de retorno en la inserción](https://github.com/rust-lang/rust/pull/117783)
* [handle intenta tener múltiples expresiones de cola 'cfg'd](https://github.com/rust-lang/rust/pull/117988)
* [ignorar pero no asumir las obligaciones de la región de unificar encabezados en coherencia negativa](https://github.com/rust-lang/rust/pull/117994)
* [ignorar los límites implícitos con marcadores de posición](https://github.com/rust-lang/rust/pull/112422)
* [lint fijó punteros '#[must_use]' (en particular, 'Box' <T>donde 'T' es '#[must_use]') en 'unused_must_use'](https://github.com/rust-lang/rust/pull/118054)
* [hacer que 'LayoutError::Cycle' lleve 'ErrorGuaranteed'](https://github.com/rust-lang/rust/pull/117849)
* [hacer que Regionck se preocupe por los marcadores de posición en los componentes de Outlives](https://github.com/rust-lang/rust/pull/118000)
* [más detalles cuando se espera una expresión pero se encuentra con un mal argumento de macro](https://github.com/rust-lang/rust/pull/114292)
* [Nuevas mejoras en la normalización del solucionador](https://github.com/rust-lang/rust/pull/117278)
* [patrones: no ICE cuando se encuentre con un corte de str sin procesar](https://github.com/rust-lang/rust/pull/117807)
* [recuperar 'dyn' e 'impl' después de 'for<...>'](https://github.com/rust-lang/rust/pull/117891)
* [eliminar la transformación incorrecta de RemoveZsts](https://github.com/rust-lang/rust/pull/117801)
* [eliminar los valores predeterminados de código de bits heredados de todas las especificaciones de Apple](https://github.com/rust-lang/rust/pull/117364)
* [manejar mejor los errores de tipo que involucran literales 'Self'](https://github.com/rust-lang/rust/pull/117959)
* [Agregue algunas advertencias adicionales para elementos de diagnóstico duplicados](https://github.com/rust-lang/rust/pull/117742)
* [sugerir la desreferenciación del LHS para binops como '&T == T'](https://github.com/rust-lang/rust/pull/117893)
* [al resolver el error de '[rest..] ', sugerir '[descanso @ ..] «](https://github.com/rust-lang/rust/pull/117998)
* [intente usar regiones de marcador de posición aproximadas al generar un error AscribeUserType en borrowck](https://github.com/rust-lang/rust/pull/116097)
* [Cuando un enlace local sombrea un FN, apunte a FN def en caso de error de llamada](https://github.com/rust-lang/rust/pull/117924)
* [Al encontrar el literal de llamada fn 'struct' con campos privados, sugiera todos los constructores](https://github.com/rust-lang/rust/pull/117683)
* [Cuando use FN existente como módulo, no afirme que no existe](https://github.com/rust-lang/rust/pull/117964)
* [Interpretar: Simplifique el manejo de turnos al ya no intentar manejar cantidades de turnos firmadas y no firmadas en la misma sucursal](https://github.com/rust-lang/rust/pull/117832)
* [MIR personalizado: Bloques de limpieza de soporte](https://github.com/rust-lang/rust/pull/117330)
* [emitir sonrisa](https://github.com/rust-lang/rust/pull/117745)
* [agregar CoroutineWitness a los tipos cubiertos en smir](https://github.com/rust-lang/rust/pull/117787)
* [miri: cargo-miri: cuando se verbose, imprime dónde se está construyendo el sysroot](https://github.com/rust-lang/miri/pull/3175)
* [Miri: deshazte de nuestros últimos usos de 'set_var'](https://github.com/rust-lang/miri/pull/3168)
* [miri: implementar los 16 operadores de comparación AVX para vectores SIMD de 128 bits](https://github.com/rust-lang/miri/pull/3176)
* [Miri: Propuesta de soporte para ReallocArray Shim Linux/FreeBSD](https://github.com/rust-lang/miri/pull/3166)
* [rehabilitar efectos en libcore](https://github.com/rust-lang/rust/pull/117825)
* [si está disponible, use el pidfd de un niño para matar/esperar](https://github.com/rust-lang/rust/pull/117957)
* [Solucionado el problema de redondeo con exponentes en 'FMT'](https://github.com/rust-lang/rust/pull/116301)
* [añadir 'Buscar::seek_relative'](https://github.com/rust-lang/rust/pull/116750)
* [impl más rasgos para 'ptr::Alignment,' add mask method](https://github.com/rust-lang/rust/pull/115249)
* [feat: implementar 'DoubleEndedSearcher' para 'CharArray[Ref]Searcher'](https://github.com/rust-lang/rust/pull/111922)
* [hashbrown: evite el uso de 'ptr::invalid_mut'](https://github.com/rust-lang/hashbrown/pull/481)
* [futuros: fillBuf: no sondees por segunda vez en EOF](https://github.com/rust-lang/futures-rs/pull/2801)
* [futuros: eliminar el redundante 'impl Unpin'](https://github.com/rust-lang/futures-rs/pull/2800)
* [cargo-credential-1password: agregue el argumento '--account' que falta al comando 'op signin'](https://github.com/rust-lang/cargo/pull/12985)
* [cargo: añadir salida de color para 'cargo --list'](https://github.com/rust-lang/cargo/pull/12992)
* [cargo resolver: No hacer recuperaciones de git al actualizar los miembros del espacio de trabajo](https://github.com/rust-lang/cargo/pull/12975)
* [resolución de carga: Preferir MSRV, en lugar de ignorar incompatibles](https://github.com/rust-lang/cargo/pull/12950)
* [cargo: arreglar las invocaciones '--check-cfg' con cero características](https://github.com/rust-lang/cargo/pull/13011)
* [cargo: corregir mensaje de error para enlaces duplicados](https://github.com/rust-lang/cargo/pull/12973)
* [cargo: handle '$message_type' en diagnósticos JSON](https://github.com/rust-lang/cargo/pull/13016)
* [cargo: si el único camino es un bucle, entonces se cuenta como el camino más corto](https://github.com/rust-lang/cargo/pull/12977)
* [cargo: ignorar 'changing_spec_relearns_crate_types' en windows-gnu](https://github.com/rust-lang/cargo/pull/12972)
* [cargo: solo filtrar el objetivo si está en la raíz del paquete](https://github.com/rust-lang/cargo/pull/12944)
* [eliminar la creación innecesaria de variables 'desconocidas' y 'símbolos' al iterar sobre elementos en el renderizado de rustdoc](https://github.com/rust-lang/rust/pull/118051)
* [rustdoc-search: optimize unifyFunctionTypes](https://github.com/rust-lang/rust/pull/118024)
* [rustdoc-search: simplifica la ruta rápida de checkTypes](https://github.com/rust-lang/rust/pull/117955)
* [rustfix: arreglar la inserción al principio](https://github.com/rust-lang/rustfix/pull/224)
* [clippy: 'impl_trait_in_params': evita ICE cuando la función con el tipo 'impl Trait' no tiene parámetros](https://github.com/rust-lang/rust-clippy/pull/11804)
* [clippy: 'needless_return_with_question_mark' ignorar let-else](https://github.com/rust-lang/rust-clippy/pull/11802)
* [clippy: cambia 'if_same_then_else' para que sea una pelusa de 'estilo'](https://github.com/rust-lang/rust-clippy/pull/11809)
* [clippy: extiende 'maybe_misused_cfg' lint sobre 'cfg(test)'](https://github.com/rust-lang/rust-clippy/pull/11821)
* [clippy: 'manual_memcpy' reduce las sugerencias de indexación cuando la longitud de la matriz es igual al rango del bucle](https://github.com/rust-lang/rust-clippy/pull/11778)
* [clippy: implementar la nueva pelusa 'iter_over_hash_type'](https://github.com/rust-lang/rust-clippy/pull/11791)
* [clippy: mejorar tal vez mal usado cfg](https://github.com/rust-lang/rust-clippy/pull/11840)
* [clippy: lint 'flatten()' under 'lines_filter_map_ok'](https://github.com/rust-lang/rust-clippy/pull/11691)
* [clippy: new lint 'clippy::join_absolute_paths'](https://github.com/rust-lang/rust-clippy/pull/11453)
* [clippy: enseñar a 'eager_or_lazy' sobre operaciones aritméticas de pánico](https://github.com/rust-lang/rust-clippy/pull/11002)
* [clippy: verificar <T>la semántica 'Borrow' para los tipos que implementan Hash, 'Borrow' <str>y 'Borrow<[u8]>'](https://github.com/rust-lang/rust-clippy/pull/11781)
* [Rust-Analyzer: Diagnosticar la inseguridad incorrecta para las implicaciones de rasgos](https://github.com/rust-lang/rust-analyzer/pull/15893)
* [Rust-Analyzer: Diagnosticar elementos de asociación faltantes en el rasgo Impls](https://github.com/rust-lang/rust-analyzer/pull/15895)
* [Rust-analyzer: diagnosticar algunos casos de impl de rasgos huérfanos](https://github.com/rust-lang/rust-analyzer/pull/15891)
* [rust-analyzer: corrige la gramática de 'PathSegment'](https://github.com/rust-lang/rust-analyzer/pull/15875)
* [Rust-Analyzer: Fix builtin line! expansion](https://github.com/rust-lang/rust-analyzer/pull/15903)
* [Rust-analyzer: diagnostica todo en elementos anidados, no solo diagnósticos def](https://github.com/rust-lang/rust-analyzer/pull/15901)
* [rust-analyzer: maneja los valores constantes predeterminados en el diagnóstico 'trait_impl_missing_assoc_item'](https://github.com/rust-lang/rust-analyzer/pull/15911)

### Clasificación del rendimiento del compilador de Rust

Semana bastante tranquila, con solo un pequeño número de cambios estadísticamente significativos.

Triaje realizado por **@simulacrum**.
Rango de revisión: [173b6e68.. 4f3da90](https://perf.rust-lang.org/?start=173b6e686b158dbad7d072c64bef3ced2052312b&end=4f3da903a43f22ea33d2ca4435a24b42fc1f842a&absolute=false&stat=instructions%3Au)

1 Regresiones, 1 Mejoras, 1 Mixto; 0 de ellos en rollups
60 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-11-21.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC: Sustitución de dependencias públicas/privadas](https://github.com/rust-lang/rfcs/pull/3516)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [[edición estilo 2024] Combinar todas las exprs delimitadas como último argumento](https://github.com/rust-lang/rust/pull/114764)
* [disposición: fusionar] [Problema de seguimiento para 'ptr::addr_eq'](https://github.com/rust-lang/rust/issues/116324)
* [disposición: fusionar] [Estabilizar literales de cadena C](https://github.com/rust-lang/rust/pull/117472)
* [disposición: fusionar] [Problema de seguimiento para mutex_unpoison](https://github.com/rust-lang/rust/issues/96469)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Política de edición de especificadores de fragmentos de macros](https://github.com/rust-lang/rfcs/pull/3531)
* [eRFC: Implementar delegación de funciones en rustc](https://github.com/rust-lang/rfcs/pull/3530)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-11-22 - 2023-12-20 🦀

### Virtual

* 23/11/2023 | Virtual (Edmonton, AB, CA) | [Grupo de usuarios de Edmonton R - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Reunión del grupo de usuarios de Edmonton R: R y Rust, como una pareja hecha en el cielo**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 28/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 28/11/2023 | Virtual (Europa / África) | [Rust para el almuerzo](https://lunch.rs/)
    * [**Encuentro de Rust**](https://lunch.rs/meetups/2023-11-28/)
* 29/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [** ¡Capítulo final del Club de Lectura de Atomics & Locks! (Capítulo 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 30/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 30/11/2023 | Virtual (Dublín, IE) | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Automatización de la experiencia con comprobaciones de carga de carga**](https://www.meetup.com/rust-dublin/events/296346693/)
* 01/12/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 02/12/2023 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 05/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Espejo**](https://berline.rs/)
* 05/12/2023 | Virtual (Búfalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)
* 06/12/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/297172140)
* 12/12/2023 | Virtual | [Materia principal](https://mainmatter.com)
    * [**Taller: Telemetría para aplicaciones de Rust**](https://rust-telemetry-workshop.mainmatter.com)
* 12/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/fvdtgtyfcqbqb/)
* 14/12/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 18/12/2023 | Virtual (Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 20/12/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763506/)

### Europa

* 23/11/2023 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)
* 28/11/2023 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**LDN conversa en noviembre de 2023 con Helsing.ai**](https://www.meetup.com/rust-london-user-group/events/297257712/)
* 30/11/2023 | Bruselas, BE | [Grupo de usuarios de Rust de Bélgica](https://www.meetup.com/fr-FR/belgium-rust-user-group/events/297538601/)
    * [**Lambda Bruselas**](https://lambda-brussels.glitch.me/)
* 30/11/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #42 patrocinado por Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/297405705/)
* 30/11/2023 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - Noviembre**](https://www.meetup.com/rust-vienna/events/297382145/)
* 30/11/2023 | Zúrich, CH| [Rust Zúrich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-zurich/events/297312190/)
* 06/12/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Encuentro de diciembre**](https://www.meetup.com/rustcologne/events/297100007/)
* 07/12/2023 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 07/12/2023 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 14/12/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 18/12/2023 | Múnich, DE + Virtual | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, una alternativa al electrón**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### América del Norte

* 22/11/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 28/11/2023 | Pasadena, CA, EE. UU. | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Grupo mensual de Rust**](https://www.meetup.com/thursday-go/events/297062186/)
* 29/11/2023 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296657831/)
* 12/12/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 19/12/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)

### Oceanía

* 28/11/2023 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/296391733/)
* 05/12/2023 | Aukland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Trucos asíncronos avanzados + software interrumpible**](https://www.meetup.com/rust-akl/events/297271684/)
* 11/12/2023 | Perth, WA, AU | [Grupo de Meetup de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Evento de fin de año de Rust**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Si lo requieres, mídelo. Esa es la respuesta simple. Todo lo demás son conjeturas.

– [Johannes Lade sobre los usuarios de Rust](https://users.rust-lang.org/t/rusts-forcing-of-using-pointers-when-writing-a-variable-printing-it/102627/12)

¡Gracias a [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1489) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/181s96q/this_week_in_rust_522/)</small>
