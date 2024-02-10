---
title: "Esta semana en Rust #18"
number_of_week: 18
description: El crate de esta semana es embedded-cli-rs, una biblioteca que facilita la creación de CLI en dispositivos integrados.
date: 2024-02-07
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
* [crates.io: Cambios en el código de estado de la API](https://blog.rust-lang.org/2024/02/06/crates-io-status-codes.html)

### Fundación
* [Google contribuye con 1 millón de dólares a la Fundación Rust para apoyar la "Iniciativa de interoperabilidad" de C++/Rust](https://foundation.rust-lang.org/news/google-contributes-1m-to-rust-foundation-to-support-c-rust-interop-initiative/)

### Actualizaciones de proyectos/herramientas
* [Anuncio del lanzamiento de la versión beta de Tauri v2](https://beta.tauri.app/blog/tauri-2-0-0-beta/)
* [Polars — Por qué hemos reescrito el tipo de datos de cadena](https://pola.rs/posts/polars-string-type/)
* [registro de cambios de rust-analyzer #219](https://rust-analyzer.github.io/thisweek/2024/02/05/changelog-219.html)
* [Ratatui 0.26.0 - una librería de Rust para cocinar interfaces de usuario de terminales](https://ratatui.rs/highlights/v026/)

### Observaciones/Pensamientos
* [¿Se bloqueará?](https://blog.yoshuawuyts.com/what-is-blocking/)
* [Rust incrustado en producción ..?](https://blog.lohr.dev/embedded-rust)
* [Que los futuros sean futuros](https://without.boats/blog/let-futures-be-futures/)
* [Compilar Rust es una prueba](https://kobzol.github.io/rust/2024/02/04/compiling-rust-is-testing.html)
* [Los frameworks web de Rust tienen informes de errores deficientes](https://www.lpalmieri.com/posts/rust-web-frameworks-have-subpar-error-reporting/)
* [video] [Probando el rendimiento - FOSDEM 2024 - Rust Dev Room](https://www.youtube.com/watch?v=P87C4jNakGs)
* [video] [Stefan Baumgartner - Pruebas, rasgos y tribulaciones](https://www.youtube.com/watch?v=DH9HIBbpktY)
* [video] [Rainer Stropek - Gestión de memoria en Rust](https://www.youtube.com/watch?v=5yy64sy2oSM)
* [video] [Shachar Langbeheim - Async & FFI - no es exactamente una historia de amor](https://www.youtube.com/watch?v=z3tpB94VKwU)
* [video] [Massimiliano Mantione - Programación Orientada a Objetos, y Rust](https://www.youtube.com/watch?v=XkCHjuF5Qps)
* [audio] [Desbloqueando el poder de Rust a través de la tutoría y la difusión del conocimiento, con Tim McNamara](https://rustacean-station.org/episode/tim-mcnamara/)
* [audio] [Asciinema con Marcin Kulik](https://rustacean-station.org/episode/marcin-kulik/)
* [Tipos no afines, caídas manuales y tiempos de vida invariantes en Rust - Primera parte](https://asyncmove.com/blog/2024/02/non-affine-types-manuallydrop-and-invariant-lifetimes-in-rust-part-one/)
* [Nueve reglas para acceder a archivos en la nube desde su código Rust: lecciones prácticas de la actualización de Bed-Reader, una biblioteca de bioinformática](https://towardsdatascience.com/nine-rules-for-accessing-cloud-files-from-your-rust-code-d456c1e2ceb4)

### Tutoriales de Rust
* [AsyncWrite y una historia de cuatro implementaciones](https://medium.com/@razieh.behjati/asyncwrite-and-a-tale-of-four-implementations-e63aef8397f7)
* [Recolección de basura sin código inseguro](https://fitzgeraldnick.com/2024/02/06/safe-gc.html)
* [Especificadores de fragmentos en macros de Rust](https://anoopelias.github.io/posts/fragment-specifiers-in-rust-macros/)
* [Escribir una API REST en Rust](https://www.shuttle.rs/blog/2024/01/31/write-a-rest-api-rust)
* [video] [Rasgos y operadores](https://www.youtube.com/watch?v=q6T5MsVmz7g)
* [Escribir un cliente y un servidor netcat simples en Rust](https://developerlife.com/2024/01/13/write-simple-netcat-in-rust/)

### Miscelánea
* [Anuncio de RustFest 2024](https://rustfest.ch/posts/2024-02-01/announcement/)
* [Preprocesamiento de billones de tokens con Rust (estudio de caso)](https://mainmatter.com/cases/aleph-alpha/)
* [Todas las charlas de EuroRust 2023 ordenadas por el número de visualizaciones](https://techtalksweekly.substack.com/p/all-eurorust-2023-talks-ordered-by)

## Crate de la semana

El crate de esta semana es [embedded-cli-rs](https://github.com/funbiscuit/embedded-cli-rs), una biblioteca que facilita la creación de CLI en dispositivos integrados.

¡Gracias a [Sviatoslav Kokurin](https://users.rust-lang.org/t/crate-of-the-week/2704/1286) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Fluvio - Construir un nuevo envoltorio de python para la caja del cliente fluvio](https://github.com/infinyon/fluvio/issues/3842)
* [Fluvio - Conector MQTT: Prefijo ID de cliente generado automáticamente para evitar caídas de conexión](https://github.com/infinyon/fluvio/issues/3825)
* [Ockam - Implementar eventos en 'SqlxDatabase'](https://github.com/build-trust/ockam/issues/7116)
* [Ockam - Se ha mejorado la salida tanto para 'ockam project ticket' como para 'ockam project enroll', con soporte para '--output json'](https://github.com/build-trust/ockam/issues/7473)
* [Ockam - Se ha mejorado la salida del ticket del proyecto ockam y la información no es opaca](https://github.com/build-trust/ockam/issues/7478)
* [Hyperswitch - [CARACTERÍSTICA]: Cobertura de código de configuración para pruebas locales y CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [FEATURE]: Tener get_required_value usar ValidationError en OptionExt](https://github.com/juspay/hyperswitch/issues/860)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador. 

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](enlace al CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se han presentado convocatorias de ponencias o presentaciones esta semana.* -->

* [RustNL 2024](https://2024.rustnl.org/) [CFP](https://cryptpad.fr/form/#/2/form/view/WtBXS48XjwAvYXIjhgEY6yJ0EOpULX+zByvkHds2oUA/) cierra 2024-02-19 | Delft, Países Bajos | Fecha del evento: 2024-05-07 & 2024-05-08
* [NDC Techtown](https://ndctechtown.com/call-for-papers) CFP cierra 2024-04-14 | Kongsberg, Noruega | Fecha del evento: 2024-09-09 hasta 2024-09-12

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

309 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-30..2024-02-06

* [Agregar AVX512FP16 a las características de destino x86](https://github.com/rust-lang/rust/pull/119543)
* [RISCV solo soporta 'split_debuginfo=off' por ahora](https://github.com/rust-lang/rust/pull/120518)
* [target: por defecto al modelo de código medio en los objetivos LoongArch](https://github.com/rust-lang/rust/pull/120661)
* ['#! [feature(inline_const_pat)]' ya no está incompleto](https://github.com/rust-lang/rust/pull/120547)
* [en realidad abortar en -Zpanic-abort-tests](https://github.com/rust-lang/rust/pull/120326)
* [añadir la falta de 'potential_query_instability' para las claves y los valores en el mapa hash](https://github.com/rust-lang/rust/pull/120485)
* [evite ICE cuando 'is_val_statically_known' no sea de un tipo compatible](https://github.com/rust-lang/rust/pull/120484)
* [Tenga más cuidado al interpretar una etiqueta/vida como un literal de caracteres mal escrito](https://github.com/rust-lang/rust/pull/120460)
* [marque 'RUST_BOOTSTRAP_CONFIG' en la prueba 'profile_user_dist'](https://github.com/rust-lang/rust/pull/120207)
* [Compruebe correctamente la compuerta de la función 'never_type'](https://github.com/rust-lang/rust/pull/120552)
* [Cobertura: Mejorar el manejo de los intervalos de función/cierre](https://github.com/rust-lang/rust/pull/120569)
* [Cobertura: usar 'edición' normal: encabezados en las pruebas de cobertura](https://github.com/rust-lang/rust/pull/120566)
* [deduplicar errores de mayor tamaño en las exprs de llamadas](https://github.com/rust-lang/rust/pull/120293)
* ['pattern_analysis': Abortar correctamente en caso de incompatibilidad de tipos](https://github.com/rust-lang/rust/pull/120313)
* ['pattern_analysis': manual de limpieza impls](https://github.com/rust-lang/rust/pull/120516)
* ['pattern_analysis': limpiar los contextos](https://github.com/rust-lang/rust/pull/120321)
* [arreglar la falta de solidez de BufReader añadiendo un check en 'default_read_buf'](https://github.com/rust-lang/rust/pull/120607)
* [arreglar ICE en el acceso de campo en un tipo contaminado después de una falla const-eval](https://github.com/rust-lang/rust/pull/120616)
* [hir: refactorizar getters para nodos propietarios](https://github.com/rust-lang/rust/pull/120346)
* [hir: eliminar el parámetro de tipo genérico de 'MaybeOwned'](https://github.com/rust-lang/rust/pull/120610)
* [Mejorar el diagnóstico de parámetros genéricos no utilizados](https://github.com/rust-lang/rust/pull/120556)
* [introducir soporte para el modificador enlazado 'async' en los rasgos 'Fn*'](https://github.com/rust-lang/rust/pull/120392)
* [hacer coincidir en NaN un error grave, y eliminar el resto de 'illegal_floating_point_literal_pattern'](https://github.com/rust-lang/rust/pull/116284)
* [Hacer que el ID de definición de corrutina de un cierre asíncrono sea el elemento secundario del ID de definición de cierre](https://github.com/rust-lang/rust/pull/120402)
* [Limpiezas de diagnósticos varios](https://github.com/rust-lang/rust/pull/120571)
* [mover las pruebas de problemas de la interfaz de usuario a los subdirectorios](https://github.com/rust-lang/rust/pull/120439)
* [mover cosas de predicado, región y const a sus propios módulos en el medio](https://github.com/rust-lang/rust/pull/120497)
* [nunca patrones: Es correcto bajar '!' a '_'](https://github.com/rust-lang/rust/pull/120517)
* [Normalizar la obligación de región en la resolución de regiones léxicas con el solucionador de próxima generación](https://github.com/rust-lang/rust/pull/119101)
* [solo sugiere la eliminación de los métodos de conversión 'as_*' y 'to_' en E0308](https://github.com/rust-lang/rust/pull/120473)
* [proporcionar más contexto sobre la etiqueta primaria de error de obligación derivada](https://github.com/rust-lang/rust/pull/120469)
* [sugerir cambiar el tipo a los parámetros const si encontramos un tipo en la posición ligada al rasgo](https://github.com/rust-lang/rust/pull/120570)
* [suprimir diagnósticos inútiles para atributos de nivel superior no resueltos](https://github.com/rust-lang/rust/pull/118533)
* [miri: normalizar la cola de 'estructura' en la comprobación de compatibilidad de ABI](https://github.com/rust-lang/rust/pull/120587)
* [Miri: Quitando la intercepción 'sched_getaffinity' de Linux'shim, FreeBSD su...](https://github.com/rust-lang/miri/pull/3283)
* [Miri: Cambiar a la caja de 'rastreo' de Rustc en lugar de usar nuestra propia caja de 'troncos'](https://github.com/rust-lang/miri/pull/2956)
* [revertir cambios de libcore poco sólidos](https://github.com/rust-lang/rust/pull/120562)
* [arreglar algunas fugas del asignador 'Arc'](https://github.com/rust-lang/rust/pull/120445)
* [use '<T, U>' para la igualdad de matriz/segmento 'impl's](https://github.com/rust-lang/rust/pull/120384)
* [mejorar el caso de error 'io::Read::read_buf_exact'](https://github.com/rust-lang/rust/pull/120523)
* [rechazar lecturas de tamaño infinito de 'io::Repeat'](https://github.com/rust-lang/rust/pull/119991)
* [Propuesta de corrección 'thread_local::register_dtor' para FreeBSD](https://github.com/rust-lang/rust/pull/120430)
* [añadir los tipos LocalWaker y ContextBuilder al núcleo, y el rasgo LocalWake a alllot](https://github.com/rust-lang/rust/pull/118960)
* [codegen\_gcc: mejora el iterador para la supresión de archivos](https://github.com/rust-lang/rustc_codegen_gcc/pull/416)
* [cargo: No entres en pánico en los tramos vacíos](https://github.com/rust-lang/cargo/pull/13375)
* [cargo: Mejorar el mensaje de error de mapa/secuencia](https://github.com/rust-lang/cargo/pull/13376)
* [cargo: aplicar '-Zpanic-abort-tests' a doctests también](https://github.com/rust-lang/cargo/pull/13388)
* [Cargo: no imprimir las líneas de comando de Rustdoc en caso de fallo por defecto](https://github.com/rust-lang/cargo/pull/13387)
* [Cargo: Estabilizar Lockfile v4](https://github.com/rust-lang/cargo/pull/12852)
* [cargo: arreglar el salto de línea de rebaja en cargo-add](https://github.com/rust-lang/cargo/pull/13400)
* [cargo: use el ID de especificación en lugar del nombre para que coincida con el paquete](https://github.com/rust-lang/cargo/pull/13335)
* [rustdoc: corregir el manejo de notas al pie](https://github.com/rust-lang/rust/pull/120443)
* [rustdoc: manejar correctamente la fusión de atributos si se trata de una reexportación global](https://github.com/rust-lang/rust/pull/120501)
* [rustdoc: evitar la inyección de JS desde localStorage](https://github.com/rust-lang/rust/pull/120250)
* [rustdoc: trait.impl, type.impl: ordena impls para que no dependa del orden de serialización](https://github.com/rust-lang/rust/pull/120641)
* [clippy: 'redundant_locals': tener en cuenta las capturas de cierre por valor](https://github.com/rust-lang/rust-clippy/pull/12227)
* [clippy: nueva pelusa: 'manual_c_str_literals'](https://github.com/rust-lang/rust-clippy/pull/11919)
* [clippy: añadir pelusa 'lint_groups_priority'](https://github.com/rust-lang/rust-clippy/pull/11832)
* [clippy: añadir nueva pelusa: 'ref_as_ptr'](https://github.com/rust-lang/rust-clippy/pull/12087)
* [clippy: añadir configuración para que 'wildcard_imports' ignore ciertas importaciones](https://github.com/rust-lang/rust-clippy/pull/11979)
* [clippy: evitar borrar bloques etiquetados](https://github.com/rust-lang/rust-clippy/pull/12219)
* [clippy: se corrigió FP en 'unused_io_amount' para Ok(lit), unrachable! y unwrap de...](https://github.com/rust-lang/rust-clippy/pull/12217)
* [rust-analyzer: Ayuda "Normalizar importación" y utilidades para normalizar árboles de uso](https://github.com/rust-lang/rust-analyzer/pull/16417)
* [rust-analyzer: habilitar la exclusión de resultados de búsqueda de referencias en la prueba](https://github.com/rust-lang/rust-analyzer/pull/16441)
* [rust-analyzer: soporte para GOTO def desde *dentro* de los archivos incluidos con la macro 'include!'](https://github.com/rust-lang/rust-analyzer/pull/16439)
* [rust-analyzer: emite un error de analizador sintáctico para la lista de argumentos que falta](https://github.com/rust-lang/rust-analyzer/pull/16493)
* [rust-analyzer: cambia 'Subtree::token_trees' de 'Vec' a box slice](https://github.com/rust-lang/rust-analyzer/pull/16482)

### Clasificación del rendimiento del compilador de Rust

El CI de Rust estuvo a la baja la mayor parte de la semana, lo que llevó a una colección mucho más pequeña de
confirmaciones de lo habitual. Los resultados son en su mayoría neutrales para la semana.

Triaje realizado por **@simulacrum**.
Rango de revisión: [5c9c3c78.. 0984bec](https://perf.rust-lang.org/?start=5c9c3c7871d603ba13d38372830eca0c9013e575&end=0984becf01112cbd3583c796541760b65340c8db&absolute=false&stat=instructions%3Au)

0 regresiones, 2 mejoras, 1 mixta; 1 de ellos en rollups
17 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-02-05.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Considerar los superrasgos auto-rasgos de la referencia del rasgo principal en la conversión de dyn](https://github.com/rust-lang/rust/pull/119338)
* [disposición: fusionar] [eliminar 'sub_relations' de 'InferCtxt'](https://github.com/rust-lang/rust/pull/119989)
* [disposición: fusionar] [Optimizar los protectores contra venenos cuando std se construye con panic=abort](https://github.com/rust-lang/rust/pull/100603)
* [disposición: fusionar] [Comprobar la firma de llamada normalizada para WF en mir typeck](https://github.com/rust-lang/rust/pull/118882)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Parámetros de tipo de ámbito de función anidada](https://github.com/rust-lang/rfcs/pull/3562)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2024-02-07 - 2024-03-06 🦀

### Virtual

* 07/02/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh - Cómo el Rust me salvó los ojos**](https://www.meetup.com/indyrs/events/298641965/)
* 08/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 08/02/2024 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 10/02/2024 | Virtual (Cracovia, PL) | [Stacja IT Kraków](https://www.meetup.com/stacja-it-krakow/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-krakow/events/298303129/)
* 10/02/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 13/02/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341575/)
* 15/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 15/02/2024 | Virtual + Presencial (Praga, CZ) | [Rust República Checa](https://www.meetup.com/rust-czech-republic/)
    * [**Introducción y Rust en producción**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 19/02/2024 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**Reunión de Rust Melbourne de febrero de 2024**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Virtual | [Rust para el almuerzo](https://lunch.rs/about/)
    * [**Almuerzo**](https://lunch.rs/meetups/2024-02-20/)
* 2024-02-21 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 2 - Tipos**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 22/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)

### Asia

* 10/02/2024 | Hyderabad, IN | [Idioma Rust Hyderabad](https://www.meetup.com/rust-hyderabad/)
    * [**Rust Language Develope BootCamp**](https://www.meetup.com/rust-hyderabad/events/298687498/)

### Europa

* 07/02/2024 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/)
    * [**Abstracciones incrustadas**](https://www.meetup.com/rustcologne/events/298913201/) | [**Página del evento**](https://rust.cologne/2024/02/07/embedded-hal.html)
* 07/02/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 08/02/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀 **](https://www.meetup.com/rust-bern/events/298488858/)
* 08/02/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Bromas basadas en Rust**](https://www.meetup.com/rust-oslo/events/298861296/)
* 13/02/2024 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Construyendo juegos con Rust: Sumérgete en el marco de Bevy**](https://www.meetup.com/rust-trondheim/events/298838682/)
* 15/02/2024 | Praga, CZ - Virtual + Presencial | [Rust República Checa](https://www.meetup.com/rust-czech-republic/)
    * [**Introducción y Rust en producción**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 22/02/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)

### América del Norte

* 07/02/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Coolidge Corner Brookline Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/297635028/)
* 08/02/2024 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/)
    * [**BEAST: Recreando un juego clásico de terminal de DOS en Rust**](https://www.meetup.com/utah-rust/events/298888955/)
* 12/02/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 13/02/2024 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Mezclador mensual Rust NYC**](https://www.meetup.com/rust-nyc/events/298593474/)
* 13/02/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)
* 15/02/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Back Bay Rust, 15 de febrero**](https://www.meetup.com/bostonrust/events/297635043/)
* 15/02/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 22/02/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 28/02/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)

### Oceanía

* 19/02/2024 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**Reunión de Rust Melbourne de febrero de 2024**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 27/02/2024 | Canberra, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/297650401/)
* 27/02/2024 | Sídney, Nueva Gales del Sur, Australia | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [** 🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Mi opinión sobre esto es que no se puede usar Rust asíncrono de forma correcta y fluida sin entender Arc, Mutex, la mutabilidad de las variables/referencias, y cómo se compila la sintaxis asíncrona y await al final. Rust te obliga a entender cómo y por qué las cosas son como son. Te da una abstracción mínima para hacer cosas que podrían haber sido tediosas de hacer tú mismo.
> 
> tuve la oportunidad de trabajar en dos proyectos que me obligaron drásticamente a entender cómo funciona async/await. La primera es transformar una biblioteca que esté completamente sincronizada y solo requiera un rasgo de sincronización para comunicarse con el servicio externo. Todo esto suena bien, ¿verdad? Bueno, esto se convierte en un problema cuando intentamos portarlo a los navegadores. El navegador es de un solo hilo y no puede bloquear el tiempo de ejecución de JavaScript en absoluto. Podría decirse que es el entorno más extraño para los usuarios de Rust. Es simplemente imposible reescribir toda la biblioteca, ya que ya se ha enviado a producción en otras plataformas.
> 
> Lo que hicimos en su lugar fue reescribir la parte de red usando sintaxis asíncrona, pero usando nuestro propio generador. La idea es simple: el generador produce un futuro cuando se le llama, y el futuro producido puede ser esperado. ¡Pero! El futuro producido contiene un puntero de arco al generador. Eso significa que podemos alimentar el generador con el valor que estamos esperando, luego la persona que llama que tiene la referencia al generador puede devolver el resultado a la función y reanudarlo. Para el navegador, utilizamos la API nativa del navegador para derivar las comunicaciones de red; Para otras plataformas, solo usamos llamadas de red de bloqueo regulares. La interfaz externa permanece sin cambios para otras plataformas.
> 
> Honestamente, no creo que ningún otro lenguaje pueda hacer esto. Tal vez C o C++, pero que nunca tendrán la misma velocidad de desarrollo y experiencia de desarrollador.
> 
> creo que la gente ya lo ha mencionado, pero el modelo asíncrono actual de Rust es la opción más razonable. Crea problemas para los desarrolladores, pero, por otro lado, no hay un mejor modelo asincrónico para Embedded o WebAssembly.
                    
– [/u/Top_Outlandishness78 en /r/rust](https://reddit.com/r/rust/comments/1ai1a97/let_futures_be_futures/korxtar/)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1521) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1alofun/this_week_in_rust_533/)</small>
