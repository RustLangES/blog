---
title: "Esta semana en Rust #72"
number_of_week: 72
description: El crate de esta semana es tur, un emulador de máquina de Turing con interfaz de usuario en modo texto.
date: 2025-08-20
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
* [Degradando x86_64-apple-darwin al Nivel 2 con herramientas de host](https://blog.rust-lang.org/2025/08/19/demoting-x86-64-apple-darwin-to-tier-2-with-host-tools/)
* [Selecciones de representantes del Consejo de Liderazgo de septiembre de 2025](https://blog.rust-lang.org/inside-rust/2025/08/15/leadership-council-repr-selection/)
* [Elección de nuevos directores de proyecto 2025](https://blog.rust-lang.org/inside-rust/2025/08/20/electing-new-project-directors-2025/)

### Boletines
* [Este mes en Rust OSDev: julio de 2025](https://rust-osdev.com/this-month/2025-07/)
* [El Problema de Rustacean Incrustado #52](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-52)

### Actualizaciones de proyectos/herramientas
* [Zed para Windows: ¡¿Por qué está tardando tanto?!](https://zed.dev/blog/windows-progress-report)
* [SeaQuery acaba de hacer que escribir SQL sin procesar sea más agradable](https://www.sea-ql.org/blog/2025-08-15-sea-query-raw-sql/)
* ['r3bl-cmdr' v0.0.22](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.22-cmdr)
* ['r3bl_tui' v0.7.4](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.7.4-tui)
* [Heapless v0.9.1 - estructuras de datos amigables 'estáticas' que no requieren asignación de memoria dinámica](https://blog.rust-embedded.org/heapless-091/)
* [Anuncio de Asterinas 0.16.0](https://asterinas.github.io/2025/08/04/announcing-asterinas-0.16.0.html)

### Observaciones/Pensamientos
* [Colocación de argumentos](https://blog.yoshuawuyts.com/placing-arguments/)
* [Actualización sobre nuestra defensa de la seguridad de la memoria - Tweede golf](https://tweedegolf.nl/en/blog/160/update-on-our-advocacy-for-memory-safety)
* [La velocidad gana al fuzzear el código de Rust con '#[derive(Arbitrary)]'](https://nnethercote.github.io/2025/08/16/speed-wins-when-fuzzing-rust-code-with-derive-arbitrary.html)
* [Reescribiendo el plano de datos de Numaflow: una base para el futuro](https://blog.numaproj.io/rewriting-numaflows-data-plane-a-foundation-for-the-future-a64fd2470cf0)
* [Sesiones de terminal que puede marcar: Construyendo el cliente web de Zellij](https://poor.dev/blog/building-zellij-web-terminal/)
* [Modos de falla de prueba mediante inyección de errores](https://forgestream.idverse.com/blog/20250814-testing-failure-modes/)
* [Múltiples puntos de interrupción en Rust: diseño de depurador basado en la propiedad](https://system.joekain.com/2025/08/17/ownership-driven-debugger-design.html)
* [Lecciones aprendidas de la reescritura de la caja UltraGraph](https://deepcausality.com/blog/lessons-learned-from-rewriting-ultragraph) 
* [Computación científica en Rust](https://ideas.reify.ing/en/blog/scientific-computing-in-rust-with-pytorch/)
* [RKL: una interfaz de línea de comandos similar a Docker construida en Rust](https://r2cn.dev/blog/rkl-a-docker-like-command-line-interface-built-in-rust)
* [kruci: Post-mortem de una biblioteca de interfaz de usuario](https://pwy.io/posts/kruci-post-mortem/)
* [Nueve reglas para generalizar su biblioteca de Rust: lecciones de extender RangeSetBlaze a mapas (Parte 2)](https://medium.com/@carlmkadie/nine-rules-for-generalizing-your-rust-library-part-2-92bb899d47ef)
* [audio] [Listas intrusivas por diversión y beneficio](https://sdr-podcast.com/episodes/intrusive-lists-for-fun-and-profit/)

### Tutoriales de Rust
* [Mejores prácticas del constructor en Rust](https://blog.cuongle.dev/p/constructor-best-practices-in-rust)
* [Escribamos una macro en Rust - Parte 1](https://hackeryarn.com/post/rust-macros-1/)
* [Análisis de memoria en Rust](https://rumcajs.dev/posts/memory-analysis-in-rust/)

### Miscelánea
* [Rust en Microsoft y presidiendo la Fundación Rust](https://filtra.io/rust/interviews/microsoft-aug-25)
* [Hablando con Zed Industries, creadores del editor de código colaborativo 100% Rust, de alto rendimiento](https://filtra.io/rust/interviews/zed-aug-25)
* [Todos los tutoriales de Rust](https://seanborg.tech/blog/huge-tutorial-list/)
* [Informe de empleos de Rust de julio de 2025](https://filtra.io/rust/jobs-report/jul-25)

## Crate de la semana

El crate de esta semana es [tur](https://github.com/rezigned/tur), un emulador de máquina de Turing con interfaz de usuario en modo texto.

A pesar de la falta de sugerencias, llogiq está muy satisfecho con su elección.

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

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y / o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*No hay convocatorias para participar esta semana*

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->
*No se enviaron convocatorias de artículos o presentaciones esta semana.* 

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 390 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-12..2025-08-19

#### Compilador
* [compilador: allow 'extern "interrupt" fn() → !'](https://github.com/rust-lang/rust/pull/143075)
* [const-eval: soporte completo para fragmentos de puntero](https://github.com/rust-lang/rust/pull/144081)
* [no adviertas nunca a ningún lanzamiento 'as' como inalcanzable](https://github.com/rust-lang/rust/pull/144804)
* [implementar macros de derivación declarativas 'macro_rules!'](https://github.com/rust-lang/rust/pull/145208) (RFC [#3698](https://rust-lang.github.io/rfcs/3698-declarative-derive-macros.html))
* [implementar '#[derivar(Desde)]'](https://github.com/rust-lang/rust/pull/144922)
* [más limpiezas de 'Impresora'](https://github.com/rust-lang/rust/pull/144949)
* [diagnóstico de llamada de cola para incluir información de duración](https://github.com/rust-lang/rust/pull/145012)
#### Biblioteca
* [agregar método Ref/RefMut 'try_map'](https://github.com/rust-lang/rust/pull/118087)
* [agregue 'Predeterminado' impls para 'Pin'ned 'Box', 'Rc', 'Arc'](https://github.com/rust-lang/rust/pull/143717)
* [agregue métodos relacionados con ASCII de 'u8' y 'MIN' / 'MAX' a 'core::ascii::Char'](https://github.com/rust-lang/rust/pull/143467)
* [cambiar el desazucarado de 'assert!' para una mejor salida de error](https://github.com/rust-lang/rust/pull/122661)
* [constificar métodos 'SystemTime'](https://github.com/rust-lang/rust/pull/144519)
* [implementar 'ptr_cast_array'](https://github.com/rust-lang/rust/pull/144515)
* [migrar de 'cfg_if' a 'cfg_select'](https://github.com/rust-lang/rust/pull/145489)
* [estabilizar 'as_array_of_cells'](https://github.com/rust-lang/rust/pull/144054)
* [función de estabilización 'const_exposed_provenance'](https://github.com/rust-lang/rust/pull/145462)
* [estabilizar 'núcleo::iter::cadena'](https://github.com/rust-lang/rust/pull/144963)
* [estabilizar 'ip_from'](https://github.com/rust-lang/rust/pull/141744)
* [función de estabilización de 'path_file_prefix'](https://github.com/rust-lang/rust/pull/144870)
* [estabilizar las características objetivo 'sse4a' y 'tbm'](https://github.com/rust-lang/rust/pull/144542)
* [hilo: devuelve un error si falla la configuración del tamaño de la pila de subprocesos](https://github.com/rust-lang/rust/pull/144210)
* [windows: reemplace 'GetThreadId'+'GetCurrentThread' por 'GetCurrentThreadId'](https://github.com/rust-lang/rust/pull/145412)
#### Carga
* [inestable: Añadida la función inestable '-Zbuild-dir-new-layout'](https://github.com/rust-lang/cargo/pull/15848)
* [inestable: agregar -Zbuild-analysis característica inestable](https://github.com/rust-lang/cargo/pull/15845)
* [paquete: Reutilice siempre el directorio de destino del espacio de trabajo](https://github.com/rust-lang/cargo/pull/15783)
* [agregar integración inicial para '--json=timings' detrás de '-Zsection-timings'](https://github.com/rust-lang/cargo/pull/15780)
* [corregir error al ejecutar el clip de carga --all-targets -- -advertencia D](https://github.com/rust-lang/cargo/pull/15843)
* [implementar la sustitución de 'host'-destino](https://github.com/rust-lang/cargo/pull/15838)
* [error más útil para 'cargo-features = []'](https://github.com/rust-lang/cargo/pull/15781)
* [estabilizar 'build.build-dir'](https://github.com/rust-lang/cargo/pull/15833)
#### Rustdoc
* [buscar: backend de búsqueda con árbol de sufijos particionado](https://github.com/rust-lang/rust/pull/144476)
* [permitir múltiples referencias a una sola nota al pie](https://github.com/rust-lang/rust/pull/140434)
* [visualización discriminante correcta de negativo a implícito](https://github.com/rust-lang/rust/pull/145216)
#### Clippy
* ['similar_names' deja de linting para nombres de 3 caracteres](https://github.com/rust-lang/rust-clippy/pull/15100)
* ['unnecessary_operation': añadir espacio entre los STMTs en la sugerencia](https://github.com/rust-lang/rust-clippy/pull/15432)
* ['{borrow,ptr}_as_ptr': no pelar dentro de proc-macros](https://github.com/rust-lang/rust-clippy/pull/15473)
* [ajustar la categoría de pelusa 'declare_interior_mutable_const'](https://github.com/rust-lang/rust-clippy/pull/15454)
* [no sugerir usar 'DerefMut' implícito en 'ManuallyDrop' alcanzado a través de uniones](https://github.com/rust-lang/rust-clippy/pull/14387)
* [corregir el falso positivo 'match_ref_pats' en el escrutinio de coincidencia de nunca tipo](https://github.com/rust-lang/rust-clippy/pull/15474)
* [arreglar 'unnecessary_semicolon': no pelar en los stmts con attrs](https://github.com/rust-lang/rust-clippy/pull/15466)
#### Analizador de Rust
* [sugerencia de cadenas no terminadas en errores de prefijo desconocidos](https://github.com/rust-lang/rust-analyzer/pull/20425)
* [corregir "Implementar miembros predeterminados" para resolver IdentPat](https://github.com/rust-lang/rust-analyzer/pull/20432)
* [añadir si.. else en LetStmt y ArgList](https://github.com/rust-lang/rust-analyzer/pull/20390)
* [corregir sangría para 'convert_match_to_let_else'](https://github.com/rust-lang/rust-analyzer/pull/20455)
* [hacer que la consulta de elementos de idioma filtre correctamente las raíces del sistema sobrescritas / excluidas](https://github.com/rust-lang/rust-analyzer/pull/20475)
* [solo importe el elemento en "Llamada al método de no calificación" si es necesario](https://github.com/rust-lang/rust-analyzer/pull/20442)
* [guardias de apoyo en 'replace_match_with_if_let'](https://github.com/rust-lang/rust-analyzer/pull/20456)
* [seguimiento de generaciones de diagnóstico por paquete](https://github.com/rust-lang/rust-analyzer/pull/20459)
* [Tiempo de diversión del próximo solucionador](https://github.com/rust-lang/rust-analyzer/pull/20446)
* [cambiar de tiza al siguiente solucionador de rasgos](https://github.com/rust-lang/rust-analyzer/pull/20329)
* [use un mensaje de error más específico cuando hable de los registros del servidor](https://github.com/rust-lang/rust-analyzer/pull/20467)

### Triaje de rendimiento del compilador de Rust

Mucho ruido / bimodalidad esta semana. En general, sin embargo, no se produjeron cambios importantes que afectaran el rendimiento.

Triaje realizado por **@simulacrum**.
Rango de revisión: [6355cd39.. 239e8b1b](https://perf.rust-lang.org/?start=6355cd39c81e9699b1925c58d2ed3165bcab1715&end=239e8b1b47b34120287ec36b33228c1e177f0c38&absolute=false&stat=instructions%3Au)

1 Regresiones, 3 Mejoras, 7 Mixtas; 4 de ellos en rollups
27 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-08-18.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar parcialmente los conceptos básicos de 'bigint_helper_methods'](https://github.com/rust-lang/rust/pull/144494)
* [corregir el alcance de caída para los enlaces 'super let' dentro de 'if let'](https://github.com/rust-lang/rust/pull/145342)
* [Asegúrese de tratar solo las cláusulas de param where como inherentes](https://github.com/rust-lang/rust/pull/145262)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [proponer objetivos 2025h2](https://github.com/rust-lang/rfcs/pull/3849)

## Próximos eventos

Rusty Eventos entre 2025-08-20 - 2025-09-17 🦀

### Virtual
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Híbrido (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Híbrido (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
* 2025-08-21 | Virtual (Londres, Reino Unido) | [Conf42: Eventos tecnológicos en línea](https://www.meetup.com/conf42/events/)
    * [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
* 2025-08-21 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
* 2025-08-24 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
* 2025-08-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361442)
* 2025-08-28 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-28 | Virtual (Los Ángeles, CA, EE. UU.) | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Taller de contratos inteligentes impulsados por IA**](https://www.meetup.com/rust-los-angeles/events/310603465)
* 2025-08-31 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Rust de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-02 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de Rust de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcmbfb)
* 2025-09-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763848597)
* 2025-09-07 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002479)
* 2025-09-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361533)
* 2025-09-09 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**De las primeras líneas a los primeros clientes: Carol Nichols sobre la construcción de una carrera en Rust**](https://www.meetup.com/women-in-rust/events/310102318)
* 2025-09-11 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646019)
* 2025-09-11 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust/events/)
    * [**Reunión en línea de San Diego Rust de septiembre de 2025**](https://www.meetup.com/san-diego-rust/events/310326567)
* 2025-09-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002480)
* 2025-09-16 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)

### Asia
* 2025-08-20 | Seúl, KR | [Rust de Seúl](https://www.meetup.com/rust-seoul-meetup)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310347685)
* 2025-08-21 | Kuala Lumpur, MY | [Rust Malasia](https://www.linkedin.com/company/rustmalaysia/)
    * [**Malaysia Rust Meetup**](https://www.eventbrite.sg/e/backend-webdev-with-axum-and-diesel-rust-meetup-aug-2025-tickets-1588476137889)
* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de agosto de 2025**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)
* 2025-09-13 | Hangzhou, ZH, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP aún está abierto)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)

### Europa
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Encuentro de Rust #60 patrocinado por Bang & Olufsen**](https://www.meetup.com/copenhagen-rust-community/events/310591727)
* 2025-08-28 | Edimburgo, GB | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/310438757)
* 2025-08-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-29 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/310438764)
* 2025-08-30 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #16**](https://www.meetup.com/stockholm-rust/events/310322522)
* 2025-09-03 | Edimburgo, GB | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**¿Quieres un lenguaje exprimible / moderno / útil / amplio? Elija cuatro**](https://www.meetup.com/rust-and-friends/events/310536614)
* 2025-09-03 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**De los errores al paralelismo y a la preparación para el futuro: lo que hace diferente a Rust**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-04 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Reunión de Rust Gdansk #10**](https://www.meetup.com/rust-gdansk/events/310610993)
* 2025-09-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 | Berlín, DE | [Conferencia Oxidar](https://oxidizeconf.com/)  
    * [**Conferencia Oxidize**](https://oxidizeconf.com/)  
* 2025-09-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)

### América del Norte
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Híbrido (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223/)
* 2025-08-21 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310321250)
* 2025-08-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 2: Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333117)
* 2025-08-23 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Somerville Union Square, 23 de agosto **](https://www.meetup.com/bostonrust/events/310106302)
* 2025-08-27 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-08-28 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/310602222)
* 2025-08-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust en Web3 Meetup**](https://www.meetup.com/rust-los-angeles/events/310618705)
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-04 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310547154)
* 2025-09-03 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulación de sistemas retro (NES, Gameboy) en Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, 6 de septiembre **](https://www.meetup.com/bostonrust/events/310106310)
* 2025-09-11 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-14 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, 14 de septiembre **](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-17 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)

### Oceanía
* 2025-08-26 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de agosto**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 - 2025-08-30 | Wellington, Nueva Zelanda | [Forja de Rust](https://rustforgeconf.com/)
    * [**Forja de Rust**](https://rustforgeconf.com/)

### América del Sur
* 2025-08-21 | Híbrido (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina) | [Rust Lang AR](https://rust-lang.ar) | [Oxidar](https://oxidar.org)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573) | [Transmisión en vivo](https://meet.google.com/pfw-hrqx-zhf)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Es sorprendente lo lejos que ha llegado la evaluación constante en #Rust. No hace mucho tiempo que incluso un simple si/si no estaba permitido. Ahora no estamos tan lejos de tener impls de rasgos constantes y cierres de const, lo que hará que casi todo sea capaz de const.

– [Jacob Pratt sobre Mastodon](https://hachyderm.io/@jhpratt@mastodon.social/115052212557381430)

LlogIQ ha mirado todas las sugerencias cero y se ha quedado vacío, así que eligió esta cita en su lugar.

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1mwuwdz/this_week_in_rust_613/)</small>
