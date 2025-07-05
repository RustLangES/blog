---
title: "Esta semana en Rust #66"
number_of_week: 66
description: El crate de esta semana es ansic, una macro proc que proporciona un DSL para generar cadenas de escape ANSI sin sobrecarga de tiempo de ejecución.
date: 2025-07-02
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Anunciando Rust 1.88.0 | Blog de Rust](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/)
* [Se aceptan propuestas de Objetivos del Proyecto para 2025H2](https://blog.rust-lang.org/inside-rust/2025/06/23/project-goals-2025h2-call-for-submissions/)
* [Encuesta del Consejo de Liderazgo 2025](https://blog.rust-lang.org/inside-rust/2025/06/30/2025-leadership-council-survey/)
* [Actualización de la gestión del programa — junio de 2025](https://blog.rust-lang.org/inside-rust/2025/06/30/program-management-update-2025-06/)

### Boletines
* [Tendencias de Rust Edición #68 Especial](https://rust-trends.com/newsletter/join-the-rust-programming-contest-win-a-keychron-q1-airpods-pro-2-or-oura-ring-4/)

### Actualizaciones de proyectos/herramientas
* [Anunciando TokioConf 2026](https://tokio.rs/blog/2025-06-19-announcing-tokio-conf)
* [registro de cambios del analizador de Rust #292](https://rust-analyzer.github.io/thisweek/2025/06/30/changelog-292.html)
* [Cómo escribir Rust en el kernel: parte 2](https://lwn.net/SubscriberLink/1025232/4a7776eb2f0379cf/)
* [Asterinas: un nuevo proyecto de kernel compatible con Linux](https://lwn.net/SubscriberLink/1022920/14dfdc76df0f1b96/)
* [Diésel asíncrono 0.6.0](https://blog.weiznich.de/blog/diesel-async-0-6/)
* [Kiorg - un nuevo gestor de archivos multiplataforma ultrarrápido con combinaciones de teclas inspiradas en VIM](https://github.com/houqp/kiorg/releases/tag/v0.1.1)
* [Informe de situación sobre rustc_codegen_cranelift (junio de 2025)](https://bjorn3.github.io/2025/06/30/progress-report-june-2025.html)

### Observaciones/Pensamientos
* [¿Cuánto código genera esa macro proc?](https://nnethercote.github.io/2025/06/26/how-much-code-does-that-proc-macro-generate.html)
* [Leaktracer: Un asignador de Rust para rastrear asignaciones de memoria](https://blog.veeso.dev/blog/en/leaktracer-a-rust-allocator-to-trace-memory-allocations/)
* [Compilación cruzada de 10,000+ cajas CLI de Rust estáticamente](https://blog.pkgforge.dev/cross-compiling-10000-rust-cli-crates-statically)
* ["¿Por qué el compilador de Rust es tan lento?"](https://sharnoff.io/blog/why-rust-compiler-slow)
* [La primera contribución de un novato a (Rust para) Linux](https://blog.buenzli.dev/rust-for-linux-first-contrib/)
* [Manejo de excepciones en rustc_codegen_cranelift](https://tweedegolf.nl/en/blog/157/exception-handling-in-rustc-codegen-cranelift)
* [Por qué elijo Rust para crear un sistema de administración full-stack](https://idaibin-blog.vercel.app/blog/why-rust-admin)
* [video] [Los 10 mejores juegos de Bevy Jam 6](https://www.youtube.com/watch?v=wvVbsQCgbGk)
* [video] [Hilos - Parte 15 de Idiotic Rust in Simple Steps](https://www.youtube.com/watch?v=04PZPs7fbuo)
* [audio] [1Contraseña con Andrew Burkhart](https://corrode.dev/podcast/s04e06-1password/)
* [audio] [Dioxus con Jonathan Kelley](https://rustacean-station.org/episode/jonathan-kelley/)
* [audio] [Malaquita con Adi Seredinschi](https://rustacean-station.org/episode/adi-seredinschi/)

### Tutoriales de Rust
* [Implementaciones generales alternativas para un solo rasgo de Rust](https://www.greyblake.com/blog/alternative-blanket-implementations-for-single-rust-trait/)

### Miscelánea
* [Reflexiones sobre Haskell y Rust](https://academy.fpblock.com/blog/rust-haskell-reflections/)

## Crate de la semana

El crate de esta semana es [ansic](https://crates.io/crates/ansic), una macro proc que proporciona un DSL para generar cadenas de escape ANSI sin sobrecarga de tiempo de ejecución.

¡Gracias a [Zeon](https://users.rust-lang.org/t/crate-of-the-week/2704/1448) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->
*Esta semana no se han presentado convocatorias para participar.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->
*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 429 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-24..2025-07-01

#### Compilador
* [añadir '#[loop_match]' para mejorar el código DFA](https://github.com/rust-lang/rust/pull/138780)
* [agregar verificación de tiempo de ejecución para evitar sobrescribir arg en 'Diag'](https://github.com/rust-lang/rust/pull/142724)
* [marque la validez implícita de 'CoerceUnsized' antes de coaccionar](https://github.com/rust-lang/rust/pull/142976)
* [calcular errores graves sin diagnóstico en 'impl_intersection_has_impossible_obligation'](https://github.com/rust-lang/rust/pull/142647)
* [Análisis de colapso|Prestado|PostBorrowckAnalysis cuando no hay opacos](https://github.com/rust-lang/rust/pull/142802)
* [codificar correctamente los atributos de HIR en la caja cruzada](https://github.com/rust-lang/rust/pull/142777)
* [ruta rápida para los objetivos de WF en el nuevo solucionador](https://github.com/rust-lang/rust/pull/142223)
* [implementar el análisis sintáctico de préstamos anclados](https://github.com/rust-lang/rust/pull/135731)
* [mejorar la recuperación cuando los usuarios escriben 'dónde':](https://github.com/rust-lang/rust/pull/143065)
* [introduce 'ByteSymbol'](https://github.com/rust-lang/rust/pull/141875)
* [nueva sintaxis de rasgos const](https://github.com/rust-lang/rust/pull/139858)
* [solo calcular llamadas recursivas una vez](https://github.com/rust-lang/rust/pull/142625)
* [Abandono superficial de 'coerce_unsized' más](https://github.com/rust-lang/rust/pull/142941)
* [simplificar 'ObligationCauseCode::IfExpression'](https://github.com/rust-lang/rust/pull/139594)

#### Biblioteca
* [agregue el cambio de embudo SIMD y los intrínsecos de ronda a par](https://github.com/rust-lang/rust/pull/142078)
* [hacer que RefCell conste de manera inestable](https://github.com/rust-lang/rust/pull/137843)
* [hacer 'Sub', Mul', Div' y 'Rem const_traits'](https://github.com/rust-lang/rust/pull/143000)

#### Carga
* [añadir la configuración 'http.proxy-cainfo' para certificados de proxy](https://github.com/rust-lang/cargo/pull/15374)
* [expandir los mensajes de error en torno a la dependencia de la ruta en 'paquete de carga' y 'publicación de carga'](https://github.com/rust-lang/cargo/pull/15705)
* [anular las sumas de comprobación de 'Cargo.lock' al hacer una 'publicación' de simulacro](https://github.com/rust-lang/cargo/pull/15711)
* [rehacer 'cargo-test-support' y 'testsuite' para usar 'CARGO_BIN_EXE_*' para Cargo](https://github.com/rust-lang/cargo/pull/15692)

#### Rustdoc
* [rustdoc: mostrar atributos en variantes de 'enumeración'](https://github.com/rust-lang/rust/pull/142987)

#### Clippy
* ['missing_panics_doc': Permitir 'unwrap()' y 'expect()' dentro de contextos const-only](https://github.com/rust-lang/rust-clippy/pull/15170)
* ['zero_ptr': lint en el contexto 'const' también](https://github.com/rust-lang/rust-clippy/pull/15152)
* [considerar el argumento deref'ed como no temporal](https://github.com/rust-lang/rust-clippy/pull/15172)
* ['cast_possible_truncation' no debería sugerir dentro del contexto const](https://github.com/rust-lang/rust-clippy/pull/15164)
* [arreglar 'coerce_container_to_any' falso positivo en autoderef](https://github.com/rust-lang/rust-clippy/pull/15057)
* [arreglar 'disallowed_script_idents' FP en identificadores con '_'](https://github.com/rust-lang/rust-clippy/pull/15123)

#### Analizador de Rust
* [Consulta de elementos de rasgo de desarco](https://github.com/rust-lang/rust-analyzer/pull/20088)
* [no añadir '--compile-time-deps' a los comandos de script de compilación sobrescritos](https://github.com/rust-lang/rust-analyzer/pull/20121)
* [Eliminar el error de carga del espacio de trabajo de Rustc, si no necesitamos sus fuentes](https://github.com/rust-lang/rust-analyzer/pull/20092)
* [resaltado de los valores devueltos mientras el cursor está en 'match' / 'if' / '=>'](https://github.com/rust-lang/rust-analyzer/pull/19546)
* [Se corrige la finalización al escribir 'entero.|«](https://github.com/rust-lang/rust-analyzer/pull/20110)
* [embellece AST en 'PathTransform' si proviene de una macro](https://github.com/rust-lang/rust-analyzer/pull/20103)
* [analizar la nueva sintaxis de rasgos const](https://github.com/rust-lang/rust-analyzer/pull/20105)
* [eliminar el último uso de 'rustc_pattern_analysis::Captures'](https://github.com/rust-lang/rust-analyzer/pull/20124)
* [eliminar los paréntesis innecesarios en el cierre](https://github.com/rust-lang/rust-analyzer/pull/20122)
* [salsa idiomize 'VariantFields' consulta](https://github.com/rust-lang/rust-analyzer/pull/20106)

### Clasificación del rendimiento del compilador de Rust
Muchos cambios esta semana, con resultados dominados por las mejoras del 1-5% de
[#142941](https://github.com/rust-lang/rust/pull/142941) a través de muchos
Puntos de referencia en la suite.

Triaje realizado por **@simulacrum**.
Rango de revisión: [42245d34.. AD3B7257](https://perf.rust-lang.org/?start=42245d34d22ade32b3f276dcf74deb826841594c&end=ad3b7257615c28aaf8212a189ec032b8af75de51&absolute=false&stat=instructions%3Au)

3 regresiones, 6 mejoras, 5 mixtas; 4 de ellos en rollups
39 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-06-30.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: cerrar] [Borrador: Hacer into_parts métodos en las funciones asociadas a Vec](https://github.com/rust-lang/rust/pull/141509)
* [Implementar 'Debug' para 'EncodeWide'](https://github.com/rust-lang/rust/pull/140153)
* [Si 'HOME' está vacío, use la alternativa en su lugar](https://github.com/rust-lang/rust/pull/141840)
* [Problema de seguimiento para duration_constructors_lite](https://github.com/rust-lang/rust/issues/140881)
* [Imprimir ID de hilo en mensaje de pánico](https://github.com/rust-lang/rust/pull/115746)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: habilitar 'derivar(De)' para estructuras de un solo campo](https://github.com/rust-lang/rfcs/pull/3809)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Añadir la tabla '[hints]' en 'Cargo.toml', y una pista 'hints.mostly-unused'](https://github.com/rust-lang/cargo/pull/15673)
* [feat(publish): Estabilizar la publicación de múltiples paquetes](https://github.com/rust-lang/cargo/pull/15636)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-07-02 - 2025-07-30 🦀

### Virtual
* 02/07/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031667)
* 03/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820304)
* 03/07/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #11**](https://www.meetup.com/bevy-game-development/events/308463394)
* 05/07/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 06/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308298511)
* 08/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305361452)
* 13/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308298512)
* 15/07/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Puesta al día de la comunidad**](https://www.meetup.com/women-in-rust/events/307560349)
* 15/07/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/306757755)
* 16/07/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307731031)
* 17/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Cajas, Consejos y Trucos Charlas Relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/307560304)
* 27/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhckbkc)

### Asia
* 02/07/2025 | Seúl, KR | [Encuentro de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Rust en Seúl**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europa
* 02/07/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
* 02/07/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**La (in)santa Trinidad de Flutter, Rust y C**](https://www.meetup.com/rust-rhein-main/events/308609465)
* 02/07/2025 | Londres, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y ACCU especial - Taller de codificación Vibe**](https://www.meetup.com/oxford-rust-meetup-group/events/308435063/)
* 02/07/2025 | Posnan, PL | [Rust Polonia](https://www.meetup.com/rust-poland-meetup/)
    * [**Rust Poland Meetup x Poznan**](https://www.meetup.com/rust-poland-meetup/events/308480357)
* 03/07/2025 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/308328739/)
* 05/07/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #13**](https://www.meetup.com/stockholm-rust/events/308530949)
* 08/07/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Recolección de basura para Rust: la frontera del finalizador**](https://www.meetup.com/london-rust-project-group/events/308443710)
* 09/07/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 07 2025**](https://lu.ma/hismn492)
* 09/07/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtyhckbmb)
* 15/07/2025 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592246)
* 15/07/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**TUI Power: Simulación y visualización de datos de sensores con Rust**](https://www.meetup.com/london-rust-project-group/events/308434768)
* 23/07/2025 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/)
    * [**Rust Dortmund Meetup - Enseñar y Hackear**](https://www.meetup.com/rust-dortmund/events/308517530/)
* 24/07/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Charlas de julio: Un cangrejo, un pez globo y una IA de ajedrez de última generación**](https://www.meetup.com/rust-and-friends/events/308687848)
* 24/07/2025 | Núremberg/Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874/)
* 29/07/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Lightning Talks julio de 2025**](https://www.meetup.com/rust-manchester/events/308085035)
* 30/07/2025 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)

### América del Norte
* 03/07/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Julio Social Mensual**](https://www.meetup.com/rust-montreal/events/308532058)
* 03/07/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Construcción de Servicios de Rust Resilientes y Observables con steady_state**](https://www.meetup.com/stl-rust/events/306345853)
* 06/07/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Alewife Rust, 6 de julio**](https://www.meetup.com/bostonrust/events/307936287)
* 07/07/2025 | Denver, CO, EE. UU. | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
    * [**Noche de hackeo de Rust incrustado**](https://mobilizon.us/events/e9d6fd47-6120-4789-ad04-313d6a04f572)
* 08/07/2025 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Las ventajas y desventajas de usar Rust para codegen**](https://www.meetup.com/rust-nyc/events/308679186)
* 09/07/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> AI**](https://www.meetup.com/desert-rustaceans/events/308507249/)
* 10/07/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/308277549)
* 15/07/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307931266)
* 17/07/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Series 1 : Introducción al Desarrollo Embebido**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 17/07/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 23/07/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhckbfc)

### América del Sur
* 2025-07-12 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 17/07/2025 | Florianópolis, BR | [Rust Brasil + Rust Floripa](https://lu.ma/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://lu.ma/p0umq6vm)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Me encanta Rust, así que ya estaba sesgado a ser positivo sobre el proyecto Rust para Linux, incluso antes de incursionar en él yo mismo. Estoy realmente sorprendido de ser aún más optimista ahora que antes. La parte de codificación fue mucho más fácil de lo que imaginaba, gracias al uso del conteo de referencias en el kernel.
>
> ¿Y los beneficios prometidos de Rust sobre C? Son absolutamente reales. La versión Rust del controlador se siente mucho más robusta que el código C, no solo en lo que respecta a la seguridad de la memoria. No tenía ni un solo error: una vez que se compilaba, funcionaba. Eso no es un gran problema teniendo en cuenta que fue una reescritura directa, pero cuenta para algo.

– [Remo Senekowitsch blogueando sobre su aventura en Rust 4 Linux](https://blog.buenzli.dev/rust-for-linux-first-contrib/)

A pesar de la lamentable falta de sugerencias, llogiq está razonablemente satisfecho con su elección.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1lqe66f/this_week_in_rust_606/)</small>
