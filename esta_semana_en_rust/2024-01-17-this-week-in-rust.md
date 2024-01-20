---
title: "Esta semana en Rust #14"
number_of_week: 14
description: El crate de esta semana es fish.
date: 2024-01-17
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

### Actualizaciones de proyectos/herramientas
* [Planificación trimestral ESP-RS: 1T 2024](https://beta7.io/posts/esp-rs-quarterly-planning-q1-2024/)
* [Meilisearch 1.6 - Búsqueda con IA e indexación 2 veces más rápida](https://blog.meilisearch.com/meilisearch-1-6/)
* [Capnproto-rust: 0.19 - colocadores ergonómicos y reflexión más rápida](https://dwrensha.github.io/capnproto-rust/2024/01/14/0.19-release.html)
* [Hyper - 2023 en revisión](https://seanmonstar.com/blog/2023-in-review/)

### Observaciones/Pensamientos
* [Una estrategia de reducción universal para los efectos de control en Rust](https://www.abubalay.com/blog/2024/01/14/rust-effect-lowering)
* [No aprobar nada es sorprendentemente difícil](https://davidben.net/2024/01/15/empty-slices.html)
* [La memoria permanece: Memoria permanente con systemd y un asignador de Rust](https://darkcoding.net/software/rust-systemd-memory-remains/)
* [Lo más destacado de 2023 para los controles de carga](https://predr.ag/blog/highlights-of-2023-for-cargo-semver-checks/)
* [La perdición de mi existencia: Soportar código asíncrono y sincronizado en Rust](https://nullderef.com/blog/rust-async-sync/)
* [Una guía para los ORM de Rust en 2024](https://www.shuttle.rs/blog/2024/01/16/best-orm-rust)
* [Búsqueda semántica impulsada por WASM y WebGPU](https://aminediro.com/posts/semsearch_wasm/)
* [Embajada en ESP: UART Echo](https://apollolabsblog.hashnode.dev/embassy-on-esp-uart-echo)

### Tutoriales de Rust
* [Aventuras en la serialización binaria](https://blog.maguire.tech/posts/explorations/binary-serialisation/)
* [Creación de 2 asignadores 'simples'](https://blog.maguire.tech/posts/explorations/allocators/)
* [Macros de Rust y Lambda repetitivas](https://medium.com/@sam.van.overmeire/rust-macros-taking-care-of-some-lambda-boilerplate-96244d9e1924)
* [Hagamos una pantalla de información en rust Part 2: The Frontend](https://blog.stillinbeta.com/2024-01-06-let's-make-an-information-display-part-2.html)

### Miscelánea
* [Clap - subcomandos para aplicaciones de línea de comandos en Rust](https://rust.code-maven.com/clap-subcommand)
* [SurrealDB in-memory con demostración de SQL en Rust](https://rust.code-maven.com/surrealdb-in-memory-with-sql-demo)
* [Contador múltiple con base de datos SurrealDB integrada](https://rust.code-maven.com/surrealdb-cli-multi-counter)
* [Planes Xilem 2024](https://linebender.org/blog/xilem-2024/)
* [Rustáceos, digan adiós a los errores de por vida: anunciando nolife 0.3](https://blog.dureuill.net/articles/nolife/)
* [Servicio API de generación automática usando Rust, a TypeScript y Dart](https://www.polydelic.com/media/autogenerating-a-rust-api-to-typescript-and-dart)
* [De la comunidad de Rust en Reddit: Rust in Aviation](https://www.reddit.com/r/rust/comments/192q0vs/rust_in_aviation/)
* [audio] [Apollo - Rust in Production Podcast](https://corrode.dev/podcast/s01e03-apollo/)
* [audio] [Prossimo con Josh Aas](https://rustacean-station.org/episode/josh-aas/)

## Crate de la semana

El crate de esta semana es [fish](https://github.com/fish-shell/fish-shell), una *jodidamente *i*nteractiva *sh*ell que solía estar escrita en C++, pero que fue reescrita recientemente en Rust (aunque hay que admitir que tendrán que hacer algo de trabajo hasta que llegue a los repositorios de tu distribución).

A pesar de la lamentable falta de sugerencias, llogiq está razonablemente satisfecho con su elección.

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [ZeroCopy - Use cargo-semver-checks para asegurarse de que la función 'derivar' no cambie la superficie de la API 2](https://github.com/google/zerocopy/issues/422)
* [Ockam - La función 'remove_address_record' se llama dos veces por dirección durante el apagado del procesador](https://github.com/build-trust/ockam/issues/7401)
* [Ockam - Comando - refactorizar para usar interfaces con tipo para implementar comandos para 'servicios de kafka'](https://github.com/build-trust/ockam/issues/6706)
* [Ockam - Biblioteca - Validar estructuras CBOR de acuerdo con el esquema cddl para 'nodos/modelos/servicios'](https://github.com/build-trust/ockam/issues/6693)
* [Hyperswitch - [CARACTERÍSTICA]: Hacer que la configuración de caché sea configurable en tiempo de ejecución](https://github.com/juspay/hyperswitch/issues/3276)
* [Hyperswitch - [CARACTERÍSTICA]: Implementar Code cov para el sistema local usando makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - [CARACTERÍSTICA]: Cobertura de código de configuración para pruebas locales y CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [CARACTERÍSTICA]: Agregar tipo de dominio para el secreto de cliente](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - [FEATURE]: Tener get_required_value usar ValidationError en OptionExt](https://github.com/juspay/hyperswitch/issues/860)
* [Fluvio - conector: fluvio-http-source, añadir una opción para leer datos de un websocket](https://github.com/infinyon/fluvio/issues/3829)
* [Fluvio - Conector MQTT: Prefijo ID de cliente generado automáticamente para evitar caídas de conexión](https://github.com/infinyon/fluvio/issues/3825)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

418 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-09..2024-01-16

* [GNU/Hurd: usar incondicionalmente sondas de pila en línea](https://github.com/rust-lang/rust/pull/119758)
* ['rustc_codegen_ssa': Forzar 'rustc::p otential_query_instability' lint](https://github.com/rust-lang/rust/pull/119409)
* ['~const' rasgo y límites de proyección no implican sus contrapartes no const](https://github.com/rust-lang/rust/pull/119721)
* [add assume into 'NonZeroIntX::get'](https://github.com/rust-lang/rust/pull/119452)
* [añadir una variante de valor explícita 'none()' en check-cfg](https://github.com/rust-lang/rust/pull/119473)
* [añadir más información a 'visit_projection_elem'](https://github.com/rust-lang/rust/pull/119877)
* [permitir '~const' en los límites de tipo asociados de nuevo](https://github.com/rust-lang/rust/pull/119894)
* [evite silenciar los errores de seguimiento relevantes](https://github.com/rust-lang/rust/pull/117449)
* [Compruebe las pelusas oxidadas cuando se detecte una pelusa desconocida](https://github.com/rust-lang/rust/pull/119819)
* [Cobertura: agregar enumeraciones para acomodar otros tipos de asignaciones de cobertura](https://github.com/rust-lang/rust/pull/119842)
* [cobertura: simplifique la construcción del gráfico de cobertura con 'CoverageSuccessors'](https://github.com/rust-lang/rust/pull/119508)
* [Implementación de la delegación: Paso 1](https://github.com/rust-lang/rust/pull/118947)
* [no permitir la referencia a 'static mut' y agregar 'static_mut_ref' lint](https://github.com/rust-lang/rust/pull/117556)
* [no haga ICE al anotar los límites de GAT en 'report_no_match_method_error'](https://github.com/rust-lang/rust/pull/119944)
* [No reexportar 'atomic::ordering' a través de 'rustc_data_structures', usar STD Import](https://github.com/rust-lang/rust/pull/119527)
* [excluir nombres conocidos de mostrar una sugerencia en check-cfg](https://github.com/rust-lang/rust/pull/118924)
* [exhaustividad: abortar en caso de error de tipo](https://github.com/rust-lang/rust/pull/119715)
* [exhaustividad: rastree los rangos superpuestos con precisión](https://github.com/rust-lang/rust/pull/119396)
* [exhaustividad: use una 'Opción' en lugar de asignar patrones ficticios](https://github.com/rust-lang/rust/pull/119688)
* [corregir ICE al sugerir la desreferenciación de operandos binop](https://github.com/rust-lang/rust/pull/119361)
* [corrige los métodos 'all_trait*' para devolver todos los rasgos disponibles en StableMIR](https://github.com/rust-lang/rust/pull/119790)
* [corregir 'allow_internal_unstable' para '(min_)especialización'](https://github.com/rust-lang/rust/pull/119963)
* [arreglar el manejo de direcciones especiales 'is_global'](https://github.com/rust-lang/rust/pull/119006)
* [soluciona el problema de 'unused_parens' cuando se sigue el lanzamiento LT](https://github.com/rust-lang/rust/pull/117321)
* [arreglar un ICE que ocurre después de que ya se ha reportado un error](https://github.com/rust-lang/rust/pull/119772)
* [nuevo indicador para emitir todos los errores retrasados como errores (añadir '-Zeagerly-emit-delayed-bugs')](https://github.com/rust-lang/rust/pull/119872)
* [hacer que 'Usuario' <T>y 'Usuario<[T]> Enviar'](https://github.com/rust-lang/rust/pull/118241)
* [Fusionar la poda de BB muerta y la deduplicación de BB inalcanzable](https://github.com/rust-lang/rust/pull/119699)
* [nunca patrones: Comprobar las encuadernaciones con los patrones nunca](https://github.com/rust-lang/rust/pull/119610)
* [devolver el mensaje de error de LLVM al contenedor de paso](https://github.com/rust-lang/rust/pull/119637)
* [registrar incluso los impls erróneos](https://github.com/rust-lang/rust/pull/119868)
* [remove '-Zdont-buffer-diagnostics'](https://github.com/rust-lang/rust/pull/119723)
* [dejar de mencionar los elementos de idioma internos en los errores binarios 'no_std'](https://github.com/rust-lang/rust/pull/116343)
* [almacenar el nombre del segmento cuando falla la resolución](https://github.com/rust-lang/rust/pull/119925)
* [sugerir la actualización del compilador para las características cerradas](https://github.com/rust-lang/rust/pull/119088)
* [sugerir que se citen las identificaciones sin comillas en los attrs](https://github.com/rust-lang/rust/pull/119341)
* [Admite llamadas recursivas asíncronas (siempre que tengan direccionamiento indirecto)](https://github.com/rust-lang/rust/pull/117703)
* [taint '_' tipos de marcador de posición en firmas de método de impl de rasgo](https://github.com/rust-lang/rust/pull/119896)
* [unificar el modo de canonicalización de consultas](https://github.com/rust-lang/rust/pull/118968)
* [eliminar una gran cantidad de enteros codificados LEB128](https://github.com/rust-lang/rust/pull/119791)
* [varargs soporte para ABI del sistema](https://github.com/rust-lang/rust/pull/119587)
* [Estabilizar la función 'mutex_unpoison'](https://github.com/rust-lang/rust/pull/119804)
* [Una implementación de comparación de segmentos más eficiente para T: ! BytewiseEq](https://github.com/rust-lang/rust/pull/116846)
* [ajustar la inlinabilidad de 'unwrap'](https://github.com/rust-lang/rust/pull/119878)
* [metadatos de carga: Estabilizar el formato de identificación como PackageIDSpec](https://github.com/rust-lang/cargo/pull/12914)
* [Resolución de carga: no entres en pánico al clasificar resúmenes vacíos](https://github.com/rust-lang/cargo/pull/13287)
* [Cargo: Agregue orientación sobre la configuración de la página de inicio en el manifiesto](https://github.com/rust-lang/cargo/pull/13293)
* [cargo: añadir la opción inestable '--output-format' a 'cargo rustdoc'](https://github.com/rust-lang/cargo/pull/12252)
* [cargo: crates-io: set 'Content-Type: application/json' solo para solicitudes con una carga útil de cuerpo](https://github.com/rust-lang/cargo/pull/13264)
* [cargo: añadir errores de estilo 'rustc' para el análisis de manifiestos](https://github.com/rust-lang/cargo/pull/13172)
* [cargo: solo heredar la tabla de paquetes del espacio de trabajo si el nuevo paquete es miembro](https://github.com/rust-lang/cargo/pull/13261)
* [cargo: la implementación de libgit2 superficial se encuentra detrás de una bandera inestable](https://github.com/rust-lang/cargo/pull/13252)
* [cargo: introduce la bandera inestable '-Zprecise-pre-release'](https://github.com/rust-lang/cargo/pull/13296)
* [cargo: eliminar debuginfo cuando no se solicita debuginfo](https://github.com/rust-lang/cargo/pull/13257)
* [rustdoc-search: reutilizar tipos individuales en firmas de funciones](https://github.com/rust-lang/rust/pull/119756)
* [clippy: 'from_over_into': sugiere una conversión correcta a ()](https://github.com/rust-lang/rust-clippy/pull/12141)
* [clippy: 'useless_asref': comprueba que el receptor del clon es el parámetro](https://github.com/rust-lang/rust-clippy/pull/12136)
* [clippy: sugerir correctamente la ruta 'std' o 'core' dependiendo de si se trata de una caja 'no_std'](https://github.com/rust-lang/rust-clippy/pull/12149)
* [clippy: extender 'useless_asref' lint en 'map(clone)'](https://github.com/rust-lang/rust-clippy/pull/12105)
* [clippy: corrige falso positivo en la verificación 'PartialEq' en 'unconditional_recursion' lint](https://github.com/rust-lang/rust-clippy/pull/12137)
* [clippy: se corrige la sugerencia para 'map_clone' lint en los tipos que implementan 'Copy'](https://github.com/rust-lang/rust-clippy/pull/12129)
* [clippy: hacer que 'HirEqInterExpr::eq_block' tenga en cuenta los comentarios mientras se comprueba si dos bloques son iguales](https://github.com/rust-lang/rust-clippy/pull/12074)
* [rust-analyzer: añadir el predicado 'notable_trait' a 'CompletionRelevance'](https://github.com/rust-lang/rust-analyzer/pull/16274)
* [rust-analyzer: ayuda a fusionar si] anidado si](https://github.com/rust-lang/rust-analyzer/pull/16209)
* [rust-analyzer: reconoce las importaciones de 'pub(crate)' en las sugerencias de importación](https://github.com/rust-lang/rust-analyzer/pull/16265)
* [Rust-Analyzer: Diferenciar entre la carga de configuración de VFS y los eventos de cambio de archivo](https://github.com/rust-lang/rust-analyzer/pull/16319)
* [rust-analyzer: arreglar la implementación de 'ast::P ath::segments'](https://github.com/rust-lang/rust-analyzer/pull/16275)
* [rust-analyzer: corrige un error de análisis incorrecto en la llamada al método en el rango](https://github.com/rust-lang/rust-analyzer/pull/16310)
* [rust-analyzer: fix nested incluye resolver desde el archivo base incorrecto](https://github.com/rust-lang/rust-analyzer/pull/16348)
* [rust-analyzer: arreglar rust-analyzer-proc-macro-srv que no aparece en Windows](https://github.com/rust-lang/rust-analyzer/pull/16312)
* [rust-analyzer: conserva los comentarios para la expresión de bloque extraída en ''extract_function''](https://github.com/rust-lang/rust-analyzer/pull/16333)
* [rust-analyzer: eliminar el indicador de función sysroot-abi de proc-macro-test](https://github.com/rust-lang/rust-analyzer/pull/16271)
* [rust-analyzer: reemplace la salida del hashset SourceRootCrates con slice para un orden determinista](https://github.com/rust-lang/rust-analyzer/pull/16339)
* [rust-analyzer: resolver el pánico en 'generate_delegate_methods'](https://github.com/rust-lang/rust-analyzer/pull/16277)

### Clasificación del rendimiento del compilador de Rust

Esta semana hubo algunas pequeñas regresiones que no ameritaron una mayor investigación,
varios de los cuales fueron descartados por ser ruido/blips en los datos. Había
también una serie de ganancias. (No te entusiasmes con esa mejora del 20,6%, es una
artefacto de medición de un parpadeo temporal en el PR que precedió inmediatamente
el triaje de esta semana).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [76101eec.. F9C2421A](https://perf.rust-lang.org/?start=76101eecbe9aa80753664bbe637ad06d1925f315&end=f9c2421a2a6e34f3756900dd7d600704c08bfccb&absolute=false&stat=instructions%3Au)

3 regresiones, 5 mejoras, 5 mixtas; 3 de ellos en rollups
55 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/703f3ddf6f2777a4ab91e2a6f4e369b8f690cdfc/triage/2024-01-16.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC: Actualización precisa de la carga previa a la liberación](https://github.com/rust-lang/rfcs/pull/3493)
* [Agregar RFC que combina los equipos de Infraestructura y Lanzamiento](https://github.com/rust-lang/rfcs/pull/3533)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: fusionar] [Evitar definiciones no locales en funciones](https://github.com/rust-lang/rfcs/pull/3373)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Estabilizar offset_of de un solo campo](https://github.com/rust-lang/rust/pull/118799)
* [disposición: fusionar] [Dejar de usar lint 'unstable_features' y usarlo en el compilador](https://github.com/rust-lang/rust/pull/118639)
* [disposición: cerrar] [Corregir 'non_camel_case_types' para gritar palabras sueltas](https://github.com/rust-lang/rust/pull/116389)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Conjunto de pruebas fuera de árbol](https://github.com/rust-lang/rfcs/pull/3557)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2024-01-17 - 2024-02-14 🦀

### Virtual

* 17/01/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 23/01/2024 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 24/01/2024 | Virtual (Berlín, DE) | [Comunidad WeAreDevelopers](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 25/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 25/01/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 28/01/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 30/01/2024 | Virtual | [Desarrollo de juegos Bevy](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 30/01/2024 | Virtual (Búfalo, NY, EE. UU.) | [Grupo de usuarios de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 30/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
* 31/01/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**¡Lanzamiento del Club de Lectura de Rustaceans!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298563633/)
* 01/02/2024 | Virtual + Presencial (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/) - [Transmisión](https://www.youtube.com/@bcnrust)
* 01/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457951/)
* 03/02/2024 | Virtual + Presencial (Bruselas, Bélgica) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM: Rust devroom - charlas**](https://fosdem.org/2024/schedule/track/rust/)
* 03/02/2024 | Virtual (Kampala, UG) | [Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 04/02/2024 | Virtual | [Especialista en Rust](https://meet-os.com/group/1)
    * [**Desarrollo web con Rocket - En Inglés**](https://meet-os.com/event/1)
* 07/02/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 08/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 08/02/2024 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 10/02/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 13/02/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341575/)

### Europa

* 17/01/2024 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Que és Rust i els seus beneficis / What's Rust and its advantage**](https://www.meetup.com/rust-girona/events/294080437/)
* 17/01/2024 | Praga / Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Reloaded 2024**](https://www.meetup.com/rust-prague/events/298005196/) 
* 17/01/2024 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/rust-zurich/)
    * [**Cómo hacer comunidad - Encuentro de enero**](https://www.meetup.com/rust-zurich/events/298066842/)
* 23/01/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hackear y aprender**](https://www.meetup.com/rust-aarhus/events/297463730/)
* 23/01/2024 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #64**](https://mobilizon.fr/events/0fce31cd-3578-43f2-abf4-ffecd8d16da2)
* 24/01/2024 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/Zagreb-Rust-Meetup/)
    * [**Rust Meetup 2024/01: Introducción a WebGPU usando Rust**](https://www.meetup.com/zagreb-rust-meetup/events/298540606/)
* 25/01/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #5: Async Part2 y Async en acción**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/298008068/)
* 25/01/2024 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - Enero - Blockchains y Data Pipelines**](https://www.meetup.com/rust-vienna/events/298504153/)
* 01/02/2024 | Híbrido (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12ª reunión de BcnRust**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 03/02/2024 | Bruselas, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM '24: Rust devroom - charlas**](https://fosdem.org/2024/schedule/track/rust/) | [**Reunión FOSDEM de Rust Aarhus**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 06/02/2024 | Bremen, DE | [Encuentro de Rust Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 07/02/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 08/02/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀 **](https://www.meetup.com/rust-bern/events/298488858/)

### América del Norte

* 17/01/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 18/01/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 24/01/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Hackathon Aprovechando el Rust para problemas del mundo real: Día 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Hackathon de aprovechamiento de Rust para problemas del mundo real: Día 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 30/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 07/02/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Coolidge Corner Brookline Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/297635028/)
* 12/02/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 13/02/2024 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Mezclador mensual Rust NYC**](https://www.meetup.com/rust-nyc/events/298593474/)
* 13/02/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)

### Oceanía

* 06/02/2024 | Perth, WA, AU | [Grupo de reunión de Perth Rust](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Reunión de Rust de febrero de 2024**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Felicitaciones a la comunidad de #Rustlang y #Rust para #Linux: ¡el #LinuxKernel ahora contiene la primera cosa útil construida con Rust!

– [Thorsten Leemhuis en FOSStodon](https://fosstodon.org/@kernellogger/111741507899977461)

Al igual que con la caja de la semana, esta semana ha habido una falta total de sugerencias, por lo que llogiq quiere ofreceros esta buena noticia desde el punto de vista de Linux.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/199ctmk/this_week_in_rust_530/)</small>
