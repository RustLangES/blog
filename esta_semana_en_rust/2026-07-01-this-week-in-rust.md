---
title: "Esta semana en Rust #116"
number_of_week: 116
description: El crate de esta semana es deconvolución, una biblioteca de deconvolución y restauración de imágenes.
date: 2026-07-01
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
* [Anunciando Rust 1.96.1 | Blog Rust](https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/)
* [Los muchos viajes de aprendizaje Rust | Blog Rust](https://blog.rust-lang.org/2026/06/25/vision-doc-journeys-to-learning-rust/)

### Fundación
* [Se lanza el programa de formación confiable de la Fundación Rust, dando a los alumnos una marca de calidad para confiar](https://rustfoundation.org/media/rust-foundation-trusted-training-program-launches-giving-learners-a-mark-of-quality-to-trust/)

### Boletines
* [Computación científica en Rust #19 (junio 2026)](https://scientificcomputing.rs/monthly/2026-06)

### Actualizaciones de proyectos/herramientas
* [Slint 1.17 Lanzado](https://slint.dev/blog/slint-1.17-released)
* [rustc_codegen_gcc: Informe de progreso #42](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-42)
* [¡Presentando Test That!](https://hovinen.me/announcements/2026/06/24/introducing-test-that.html)
* [Dentro de RSSH: una caja de Rust, tres binarios y las lecciones de tauri por el camino](https://github.com/shihuili1218/rssh/blob/main/docs/article_arch_en.md)
* [Rustty 1.38 – accesibilidad y navegación con teclado](https://github.com/Aleixenandros/Rustty/releases/tag/v1.38.0)
* [GuardianDB 0.17.0: Espacios de nombres seguros, Iroh 1.0 y la llegada del ODM](https://www.willsearch.com.br/blog/2026/06/25/guardiandb-0-17-0-secure-namespaces-iroh-1-0-and-the-arrival-of-the-odm/)
* [Creando un entorno de ejecución de agente de voz en tiempo real en Rust: sin GIL, un binario, 2.000 llamadas por caja](https://dev.to/iam_suriyan_b9078a5b3a553/building-a-real-time-voice-agent-runtime-in-rust-no-gil-one-binary-2000-calls-a-box-12ko)
* [AimDB: Trae tu propio conector](https://aimdb.dev/blog/aimdb-bring-your-own-connector)
* [kache 0.8.0: restauraciones en copia cero en Windows (ReFS)](https://github.com/kunobi-ninja/kache/releases/tag/v0.8.0)
* [Warbell — un RPG de acción de defensa de castillos construido con Bevy 0.19](https://miskibin.github.io/warbell/)
* [He creado un cliente FTP para macOS completamente en Rust - sin Electron, sin webview](https://dev.to/gregorymc86/i-built-a-macos-ftp-client-entirely-in-rust-no-electron-no-webview-2a8i)

### Observaciones/Pensamientos
* [Levantando expresiones](https://blog.yoshuawuyts.com/hoisting-expressions)
* [El lado poco glamuroso del desarrollo web de Rust](https://blog.jetbrains.com/rust/2026/06/25/rust-web-development-2026/)
* [Cómo descubrí que el 52% de mi grafo de conocimiento eran duplicados (y lo que hice al respecto)](https://dev.to/ernesto_arias_148b35bc25d/-how-i-found-out-52-of-my-knowledge-graph-was-duplicates-and-what-i-did-about-it-3coh)
* [Un enfoque novedoso para el manejo de errores de Rust](https://jtjlehi.github.io/2026/06/25/novel-rust-error-handling.html)
* [Hemos puesto un servidor Redis dentro de nuestro runtime](https://encore.dev/blog/redis-runtime)
* [Rust de alto rendimiento: Comprender y eliminar la fragmentación de la memoria](https://kerkour.com/rust-high-performance-memory-fragmentation-allocations)
* [IA y árboles de trabajo están llenando nuestros discos: almacenamiento kache, medido](https://kunobi.ninja/blog/kache-storage-worktrees)
* [Diseñando un visualizador de memoria terminal multiplataforma en Rust](https://dev.to/sicklefire/designing-a-cross-platform-terminal-memory-visualizer-in-rust-2365)
* [Tu servicio de Rust no está perdiendo — podría ser el asignador](https://pranitha.dev/posts/rust-and-memory-allocators)

### Guías de Rust
* [Medir, No Adivinar: Construir viser, un optimizador de codificación de vídeo adaptativo al contenido en Rust](https://medium.com/@vbasky/measure-don't-guess-building-viser-a-content-adaptive-video-encoding-optimizer-in-rust-7675edd6943a)
* [Aprende SQL y SQLx construyendo una CLI de biblioteca de libros en Rust](https://blog.sheerluck.dev/posts/learn-sql-and-sqlx-by-building-a-book-library-cli-in-rust/)
* [serie] [Razonamiento sobre el Rust asíncrono con máquinas de estados](https://aibodh.com/posts/async-rust-chapter-2-what-async-fn-compiles-into/)
* [El libro de migración del C al Rust](https://mainmatter.com/c-to-rust-migration-book/)

## Crate de la semana

El crate de esta semana es [deconvolución](https://github.com/pbkx/deconvolution), una biblioteca de deconvolución y restauración de imágenes.

¡Gracias a [pbkx](https://users.rust-lang.org/t/crate-of-the-week/2704/1621) por la autosugerencia!

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

[multicálculo - buenos primeros números](https://github.com/kmolan/multicalc-rust/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
* [AimDB - Añadir ejemplo mínimo: hola-single-latest](https://github.com/aimdb-dev/aimdb/issues/93)
* [AimDB - Conectar '.transform()' y '.transform_join()' al perfilado de escenario](https://github.com/aimdb-dev/aimdb/issues/109)
* [edid-info - Aumentar la cobertura de pruebas con datos reales de EDID](https://github.com/SzilvasiPeter/edid-info/issues/1)
* [edid-info - Finalizar la implementación de la extensión CTA-861](https://github.com/SzilvasiPeter/edid-info/issues/2)
* [edid-info - Soporte para tipos adicionales de bloques de extensión EDID](https://github.com/SzilvasiPeter/edid-info/issues/3)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

426 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-23..2026-06-30

#### Compilador
* [suelta la caminata AST de caja llena en 'check_unused'](https://github.com/rust-lang/rust/pull/157996)
* [hacer que 'stable_crate_ids' se lea libre de cerradura tras cargar la caja](https://github.com/rust-lang/rust/pull/158185)
* [rework lint pass running](https://github.com/rust-lang/rust/pull/158239)
* [simplificar algunas cosas 'proc_macro'](https://github.com/rust-lang/rust/pull/157271)

#### Biblioteca
* [añadir 'io::ErrorKind::TooManyOpenFiles'](https://github.com/rust-lang/rust/pull/158326)
* [expandir los métodos iteradores de 'OptionFlatten'](https://github.com/rust-lang/rust/pull/153097)
* [mover 'std::io::Error' a 'core'](https://github.com/rust-lang/rust/pull/155625)
* [optimizar analizador de direcciones de red](https://github.com/rust-lang/rust/pull/158053)

#### Carga
* [añadir bandera '-Zhint-msrv'](https://github.com/rust-lang/cargo/pull/17106)

#### Clippy
* ['filter_map_next': sugerencias de limpieza y revisión](https://github.com/rust-lang/rust-clippy/pull/17237)
* ['chunks_exact_to_as_chunks': Prevenir sugerencias sintácticamente inválidas](https://github.com/rust-lang/rust-clippy/pull/17318)
* ['chunks_exact_to_as_chunks': Usar el nombre correcto del método en el mensaje](https://github.com/rust-lang/rust-clippy/pull/17317)
* ['chunks_exact_to_as_chunks': Elige el método iter, dependiendo de la mut-idad](https://github.com/rust-lang/rust-clippy/pull/17316)
* ['non_ascii_literal', 'invisible_characters': no sugieres arreglar cuerdas crudas](https://github.com/rust-lang/rust-clippy/pull/17302)
* [crear un sencillo 'ConstEvalCtxt' en 'expr_eagerness'](https://github.com/rust-lang/rust-clippy/pull/17228)
* [detectar nuevos tipos de rango en 'superior::Rango'](https://github.com/rust-lang/rust-clippy/pull/17299)
* [no activar 'manual_option_zip' cuando el receptor de mapas es una expresión evaluada de forma perezosa](https://github.com/rust-lang/rust-clippy/pull/17270)
* [mejorar 'needless_late_init' para cubrir tareas agrupadas](https://github.com/rust-lang/rust-clippy/pull/16746)
* [corrección: 'borrow_as_ptr' se activa en código generado](https://github.com/rust-lang/rust-clippy/pull/17257)

#### Analizador de Rust
* [añadir diagnóstico para E0596](https://github.com/rust-lang/rust-analyzer/pull/22466)
* [añadir correcciones añadir '.await' para 'type_mismatch'](https://github.com/rust-lang/rust-analyzer/pull/22645)
* [crash al bajar consts con tipos asociados](https://github.com/rust-lang/rust-analyzer/pull/22646)
* [bloqueo al pasar el cursor sobre consts anónimos](https://github.com/rust-lang/rust-analyzer/pull/22640)
* [solo ejecuta 'Drop::d rop' cuando está implementado](https://github.com/rust-lang/rust-analyzer/pull/22582)
* [marca 'inline_convert_while_ascii()' como 'inseguro'](https://github.com/rust-lang/rust-analyzer/pull/22633)
* [cambiar los tipos LSP por tipos gen-lsp](https://github.com/rust-lang/rust-analyzer/pull/22115)

### Triaje de rendimiento del compilador Rust

En general, la semana fue bastante neutral, sin ningún cambio significativo en la mayoría de los indicadores de nuestras estadísticas.

Triaje hecho por **@simulacrum**.
Rango de revisión: [8b6558a0.. 7dc2c162](https://perf.rust-lang.org/?start=8b6558a02b2774acfb25cf15e199467c37ba7490&end=7dc2c162b9c197aaa76a6f9e7534569537830a01&absolute=false&stat=instructions%3Au)

2 regresiones, 1 mejora, 7 mixtas; 5 de ellos en rollups
34 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-06-29.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Problema de seguimiento para LocalKey/Cell::update](https://github.com/rust-lang/rust/issues/143989)
* [Problema de seguimiento para '{fuerza, [T], Ruta}::trim_prefix' y '{fuerza, [T]}::trim_suffix'](https://github.com/rust-lang/rust/issues/142312)
* [Estabilizar definiciones de funciones c-variádicas](https://github.com/rust-lang/rust/pull/155697)
* [Problema de seguimiento para la información de diseño detrás de punteros](https://github.com/rust-lang/rust/issues/69835)
* [Arreglar la puerta de características para 'repr(simd)'](https://github.com/rust-lang/rust/pull/158523)
* [reat no_mangle_generic_items como error difícil en lugar de advertencia de pelusa](https://github.com/rust-lang/rust/pull/154585)
* [Pelusa contra definiciones inválidas de símbolos POSIX](https://github.com/rust-lang/rust/pull/158522)
* [Corregir la pelusa de 'overflowing_literals' con repetidas negaciones](https://github.com/rust-lang/rust/pull/158302)
* [estabilizar 'extern "custom"'](https://github.com/rust-lang/rust/pull/158504)
* [No escapes de U+FF9E y U+FF9F en 'escape_debug_ext'](https://github.com/rust-lang/rust/pull/158057)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Desacoplar 'BackendRepr' de la alineación ABI](https://github.com/rust-lang/compiler-team/issues/1007)
* [MCP: Estrategia de estabilización para frontend paralelo rustc](https://github.com/rust-lang/compiler-team/issues/1005)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Los campos deben encajar en el tipo, incluso para repr(Rust)](https://github.com/rust-lang/reference/pull/2166)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Subguión de const asociado](https://github.com/rust-lang/rfcs/pull/3527)
* [Añadir 'externo "personalizado"'](https://github.com/rust-lang/rfcs/pull/3980)

##### [Directrices del Código de Peligros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Propuesta de extensión Opsem: accesos a volátiles atómicos](https://github.com/rust-lang/unsafe-code-guidelines/issues/615)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Cadena de métodos como objeto](https://github.com/rust-lang/rfcs/pull/3977)
* [Añadir 'externo "personalizado"'](https://github.com/rust-lang/rfcs/pull/3980)

## Próximos eventos

Eventos Rusty entre el 01-07-2026 - 29-07-2026 🦀

### Virtual
* 2026-07-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Aprendiendo desarrollo de juegos por las malas con Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-07-05 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254778/)
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
* 2026-07-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254777/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de julio de 2026**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### Europa
* 2026-07-01 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en julio: Vecs, cuerdas y cortes, ¡Dios mío!**](https://www.meetup.com/rustcologne/events/315404678/)
* 2026-07-01 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-07-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Construyendo un sistema de archivos desde cero**](https://www.meetup.com/oxford-rust-meetup-group/events/315409335/)
* 2026-07-02 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Bevy, Bits, & Cats (Charlas de Rust July)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, NL | [Reuniones de Tecnología de Baseflow](https://www.meetup.com/dutch-rust-meetup)
    * [**Cumbre IA**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Únete en directo e INPERSONAL para Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-07-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Funciones de Supercharge Rust con argumentos implícitos y programación genérica de contexto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework para Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)

### Norteamérica
* 2026-07-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**¿Git es fácil?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Rust de la Universidad de Boston, 4 de julio**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de Utah Rust July**](https://www.meetup.com/utah-rust/events/314696647/)
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
* 2026-07-25 | Brooklyn, NY, EE. UU. | [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)

### Oceanía
* 2026-07-21 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de julio**](https://www.meetup.com/rust-canberra/events/315307280/)
* 2026-07-23 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**¡Rust Perth: Encuentro de julio!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> *sí* espero que cualquiera que use '-Zllvm-objetivo-características' o cualquier forma estabilizada de ellas sepa que está conversando directamente con el dragón y que tenga cuidado con sus palabras si no quiere que él le haga una barbacoa y se sirva sobre un buen plato de limaduras de hierro.

– [Jubileo de trabajo en Rust Zulip](https://rust-lang.zulipchat.com/#narrow/channel/233931-t-compiler.2Fmajor-changes/topic/Add.20.60-Zllvm-target-feature.60.20target.20.2Amodif.E2.80.A6.20compiler-team.23994/near/606147265)

¡Gracias a [Tomáš Šedovič](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1784) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1ul6xfl/this_week_in_rust_658/)</small>
