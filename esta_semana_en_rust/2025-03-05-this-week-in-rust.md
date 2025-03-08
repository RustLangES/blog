---
title: "Esta semana en Rust #50"
number_of_week: 50
description: El crate de esta semana es wild, un enlazador bastante rápido escrito en Rust.
date: 2025-03-05
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Rust participa en Google Summer of Code 2025](https://blog.rust-lang.org/2025/03/03/Rust-participates-in-GSoC-2025.html)
* [Actualización de Objetivos del Proyecto de Febrero](https://blog.rust-lang.org/2025/03/03/Project-Goals-Feb-Update.html)
* [Anunciando Rustup 1.28.0](https://blog.rust-lang.org/2025/03/02/Rustup-1.28.0.html)

### Boletines
* [Este mes en Rust OSDev: febrero de 2025](https://rust-osdev.com/this-month/2025-02/)
* [Tendencias de Rust Edición #60](https://rust-trends.com/newsletter/from-dns-servers-to-json-speed-what-s-new-in-rust/)
* [El Rustáceo Incrustado Edición #40](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-40)

### Actualizaciones de proyectos/herramientas
* [Anunciando Wiremocket: Wiremock para Websockets](https://xd009642.github.io/2025/03/03/announcing-wiremocket-wiremock-for-websockets.html)
* [Un reqwest] más modular](https://seanmonstar.com/blog/modular-reqwest/)
* [MemberList 0.6 - Protocolo de chismes para la gestión de la membresía del clúster](https://github.com/al8n/memberlist/releases/tag/v0.6)
* [Maelstrom Clustered Test Runner v0.13: nuevo modo de observación y soporte de flujo de trabajo de GitHub](https://maelstrom-software.com/blog/0.13.0/)
* [Código binario 2.0.0](https://github.com/bincode-org/bincode/releases/tag/v2.0.0)

### Observaciones/Pensamientos
* [El problema con los alias de tipo](https://blog.polybdenum.com/2025/03/01/the-problem-with-type-aliases.html)
* [Tómate un descanso: la partida de Rust ha fracasado](https://huonw.github.io/blog/2025/03/rust-fallthrough/)
* [Decodificación rápida de JSON en columnas con arrow-rs](https://www.arroyo.dev/blog/fast-arrow-json-decoding)
* [Algunas cosas que hacen que la vida de Rust sea difícil de aprender](https://ntietz.com/blog/rust-lifetimes-learning/)
* [Optimización del rendimiento y cómo hacerlo mal](https://genna.win/blog/convolution-simd/)
* [No ejecutar ningún comando de Cargo en proyectos que no sean de confianza](https://shnatsel.medium.com/do-not-run-any-cargo-commands-on-untrusted-projects-4c31c89a78d6)
* [Garantías de estabilidad faltantes de la carga](https://blog.weiznich.de/blog/cargo-instablity/)
* [video] [Rust bajo el capó](https://www.youtube.com/watch?v=L8caNpK3Shs)
* [video] [9 reglas para portar Rust al navegador](https://www.youtube.com/watch?v=i6dahKSnuUg)

### Tutoriales de Rust
* [El poder de la pasantía: hacer una base de datos de series temporales 2000 veces más pequeña en Rust](https://gendignoux.com/blog/2025/03/03/rust-interning-2000x.html)
* [Bajar elementos de nivel superior](https://thunderseethe.dev/posts/lowering-items/)
* [Construyendo un servidor DNS en Rust: Parte 1 de 2](https://rust-trends.com/posts/building-a-dns-server-in-rust/)

### Miscelánea
* [video] [Asignador global de Rust](https://www.youtube.com/watch?v=TlkDwWGVZKg)
* [video] [Vulkanizado 2025: Envío de un juego con Vulkan y Rust en 100 días](https://www.youtube.com/watch?v=EB-ARcAnZY4)
* [video] [Creación de un sitio web en páginas de GitHub con mdBook](https://www.youtube.com/watch?v=x3vF9YiWBMQ)
* [Se anuncia EuroRust 2025 París](https://eurorust.eu/)
* Por favor, [nomina](https://cfp.gosim.org/spotlight-rust) nuevos proyectos innovadores para [GOSIM Rust Spotlight](https://spotlight.gosim.org/rust2025) lo antes posible!

## Crate de la semana

El crate de esta semana es [wild](https://crates.io/crates/wild-linker), un enlazador bastante rápido escrito en Rust.

¡Gracias a [Mateusz Mikuła](https://users.rust-lang.org/t/crate-of-the-week/2704/1418) por la (especie de auto)sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características
y desea que su RFC aparezca en esta lista, agregue una etiqueta de 'llamada para pruebas' a su RFC junto con
con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

Háganos saber si desea que se realice un seguimiento de su función como parte de esta lista.

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
[**EuroRust 2025**](https://www.papercall.io/eurorust-2025)| 15/05/2025 | París | 2025-10-09–2025-10-10
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

502 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-02-25..2025-03-04

#### Compilador

* [introducir 'característica(generic_const_parameter_types)'](https://github.com/rust-lang/rust/pull/137617)
* [Arreglar el análisis de rangos después de operadores unarios](https://github.com/rust-lang/rust/pull/134900)
* [implementar '#[cfg]' en las cláusulas 'where'](https://github.com/rust-lang/rust/pull/132388)
* [Optimizar las comprobaciones de rangos de procedencia vacíos](https://github.com/rust-lang/rust/pull/137704)

#### Biblioteca

* [add 'IntoBounds::intersect' y 'RangeBounds::is_empty'](https://github.com/rust-lang/rust/pull/137304)
* [arreglar el error de la ruta de búsqueda de 'Comando' de Windows](https://github.com/rust-lang/rust/pull/137673)
* [estabilizar 'core::str::from_utf8_mut' como 'const'](https://github.com/rust-lang/rust/pull/136668)
* [estabilizar 'extract_if'](https://github.com/rust-lang/rust/pull/137109)
* [estabilizar 'hash_extract_if'](https://github.com/rust-lang/rust/pull/134655)

#### Carga

* [cargo: agregar soporte SBOM](https://github.com/rust-lang/cargo/pull/13709) (RFC [#3553](https://github.com/arlosi/rfcs/blob/sbom/text/3553-cargo-sbom.md))
* [cargo: cli: Finalizaciones de bash hacia adelante de subcomandos de terceros](https://github.com/rust-lang/cargo/pull/15247)
* [cargo: añadir terminaciones para '--lockfile-path'](https://github.com/rust-lang/cargo/pull/15238)
* [cargo: reinicia $CARGO si el programa en ejecución es real 'cargo[.exe]'](https://github.com/rust-lang/cargo/pull/15208)
* [cargo: obtener todos los miembros como 'destinos disponibles' aunque se haya especificado default-members](https://github.com/rust-lang/cargo/pull/15199)
* [cargo: implementó la opción de configuración 'build.build-dir'](https://github.com/rust-lang/cargo/pull/15104)

#### Rustdoc

* ['librustdoc': devuelve 'impl fmt::D isplay' en más lugares en lugar de escribir en cadenas](https://github.com/rust-lang/rust/pull/137425)
* [califique completamente 'Resultado' en el código doctest generado](https://github.com/rust-lang/rust/pull/137807)

#### Rustfmt

* [use 'semver' para que coincida con la versión requerida](https://github.com/rust-lang/rustfmt/pull/6066)

#### Clippy

* nuevas pelusas: ['manual_midpoint'](https://github.com/rust-lang/rust-clippy/pull/13851),
  [añadir pelusa 'unnecessary_debug_formatting'](https://github.com/rust-lang/rust-clippy/pull/13893)
* [mover 'comparison_chain' de 'estilo' a 'pedante'](https://github.com/rust-lang/rust-clippy/pull/14219)
* ['macro_use_import': No compruebes si el atributo proviene de la expansión](https://github.com/rust-lang/rust-clippy/pull/14317)
* ['manual_strip': use el identificador existente en lugar del marcador de posición](https://github.com/rust-lang/rust-clippy/pull/14188)
* ['needless_collect': evita advertir si se utilizan métodos no iteradores](https://github.com/rust-lang/rust-clippy/pull/14147)
* [comprobar los atributos MSRV en las pasadas tardías utilizando el HIR](https://github.com/rust-lang/rust-clippy/pull/13821)
* [opción de configuración para lint 'incompatible_msrv' en el código de prueba](https://github.com/rust-lang/rust-clippy/pull/14279)
* [extender {'implicit','inverted'}}'_saturating_sub' a las expresiones](https://github.com/rust-lang/rust-clippy/pull/14310)
* [arreglar ICE en la verificación de 'doc_nested_refdefs' al verificar el rango](https://github.com/rust-lang/rust-clippy/pull/14308)
* [arreglar ICE en la pelusa 'manual_map'](https://github.com/rust-lang/rust-clippy/pull/14326)
* [corrección: 'map_entry' falso positivo dentro del cierre](https://github.com/rust-lang/rust-clippy/pull/14307)
* [corrección: 'map_entry' sugiere erróneamente cuando la clave no es 'Copiar'](https://github.com/rust-lang/rust-clippy/pull/14314)
* [pelusa más casos con 'ptr_eq'](https://github.com/rust-lang/rust-clippy/pull/14339)
* [dividir las sugerencias de 'needless_lifetime '_' en 'elidable_lifetime_names'](https://github.com/rust-lang/rust-clippy/pull/13960)

#### Analizador de Rust

* [rust-analyzer: agregue 'identificador' para extraer las capacidades de diagnóstico de LSP](https://github.com/rust-lang/rust-analyzer/pull/19266)
* [Rust-analyzer: Añadir ancla para enlaces intra-doc a elementos asociados](https://github.com/rust-lang/rust-analyzer/pull/19246)
* [Rust-analyzer: Añadir Flip or Pattern Assist](https://github.com/rust-lang/rust-analyzer/pull/19259)
* [rust-analyzer: permitir la bandera de característica de formato "paquete/característica"](https://github.com/rust-lang/rust-analyzer/pull/19204)
* [rust-analyzer: permitir a rust-project.json especificar el espacio de trabajo sysroot](https://github.com/rust-lang/rust-analyzer/pull/19096)
* [Rust-analyzer: permitir desconfigurar CFG predeterminados](https://github.com/rust-lang/rust-analyzer/pull/19243)
* [Rust-analyzer: cofigurar el objetivo de OHOS para evitar bloqueos de compilación](https://github.com/rust-lang/rust-analyzer/pull/19239)
* [rust-analyzer: completion-ref-matching](https://github.com/rust-lang/rust-analyzer/pull/19226)
* [Analizador de Rust: pruebas DOC](https://github.com/rust-lang/rust-analyzer/pull/19237)
* [Rust-analyzer: Doc: Eliminar liendre de setup.md](https://github.com/rust-lang/rust-analyzer/pull/19220)
* [Rust-analyzer: Arreglar sugerencias de ajuste de prefijos introduciendo paréntesis innecesariamente](https://github.com/rust-lang/rust-analyzer/pull/19249)
* [rust-analyzer: arreglar la construcción del gráfico de caja de sysroot que no mapea los identificadores de caja para proc-macros](https://github.com/rust-lang/rust-analyzer/pull/19241)
* [rust-analyzer: haz que 'inline_local_variable' use el cálculo de precedencia para los paréntesis](https://github.com/rust-lang/rust-analyzer/pull/19250)
* [rust-analyzer: eliminar la edición de sintaxis del cálculo de paréntesis](https://github.com/rust-lang/rust-analyzer/pull/19251)
* [rust-analyzer: soporta patrones de tupla 'struct' para asistencia 'expand_rest_pattern'](https://github.com/rust-lang/rust-analyzer/pull/19261)
* [Rust-Analyzer: Avisar cuando la cadena de herramientas usada parece demasiado vieja para Rust-Analyzer](https://github.com/rust-lang/rust-analyzer/pull/19244)

### Clasificación del rendimiento del compilador de Rust

Una semana bastante tranquila, con cambios mínimos en el rendimiento (positivos o negativos).

Triaje realizado por **@simulacrum**.
Rango de revisión: [f5729cfe.. daf59857](https://perf.rust-lang.org/?start=f5729cfed3c45e061e8a443677fc1d5ef9277df7&end=daf59857d6d2b87af4b846316bf1561a6083ed51&absolute=false&stat=instructions%3Au)

1 Regresiones, 4 Mejoras, 1 Mixto; 2 de ellos en rollups
29 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-03-03.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Denote 'ControlFlow' como '#[must_use]'](https://github.com/rust-lang/rust/pull/137449)
* [Convertir los objetos de rasgo dependientes del orden en una advertencia de incompatibilidad futura en un error grave](https://github.com/rust-lang/rust/pull/136968)
* [Estabilizar 'const_vec_string_slice'](https://github.com/rust-lang/rust/pull/137319)
* [añadir una edición "futura"](https://github.com/rust-lang/rust/pull/137606)
* [Problema de seguimiento para const_sockaddr_setters](https://github.com/rust-lang/rust/issues/131714)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Deseche el campo 'edition' por destino de compilación en 'Cargo.toml'](https://github.com/rust-lang/rfcs/pull/3772)
* [RFC: Degradar i686-pc-windows-gnu a Tier 2](https://github.com/rust-lang/rfcs/pull/3771)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat(package): add --exclude-lockfile flag](https://github.com/rust-lang/cargo/pull/15234)

#### Otras áreas
* *No hay artículos ingresados al Período Final de Comentarios esta semana para
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Límites predeterminados locales para ayudar a 'Olvidar' y otros '? Rasgo'.](https://github.com/rust-lang/rfcs/pull/3783)
* [Rasgo de marcador 'Olvidar'](https://github.com/rust-lang/rfcs/pull/3782)
* [RFC: Campo de registro de cambios de cajas](https://github.com/rust-lang/rfcs/pull/3779)

## Próximos eventos

Eventos de Rusty entre 2025-03-05 - 2025-04-02 🦀

### Virtual
* 05/03/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031659)
* 06/03/2025 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820281/)
* 06/03/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #9**](https://www.meetup.com/bevy-game-development/events/306131557)
* 06/03/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Ratatui - Interfaces de usuario de terminal en Rust con Orhun Parmaksız**](https://www.meetup.com/code-mavens/events/305750365/)
* 09/03/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Creación de una cadena de bloques simulada en Rust con Sourav Mishra**](https://www.meetup.com/code-mavens/events/305587087/)
* 09/03/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Convertirse en un Campeón de Rust: Llevando a tu equipo al éxito con Yoni Peleg**](https://www.meetup.com/rust-tlv/events/306396549)
* 11/03/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/303522529)
* 11/03/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 La comunidad se pone al día**](https://www.meetup.com/women-in-rust/events/305716839)
* 13/03/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820296)
* 18/03/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**crum: Números Complejos y Matrices Complejas en Rust con Frans Slabber**](https://www.meetup.com/code-mavens/events/305823397/)
* 18/03/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170694)
* 19/03/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Tocino y Benchmarking de Rendimiento**](https://www.meetup.com/vancouver-rust/events/305470139)
* 2025-03-20 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Rust y programación integrada con Leon Vak (en línea en inglés)**](https://www.meetup.com/code-mavens/events/306357728)
* 25/03/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361431)
* 25/03/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: SKIing into Rust - elaborando un intérprete sencillo**](https://www.meetup.com/women-in-rust/events/305988711)
* 27/03/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820297)
* 01/04/2025 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304228)
* 02/04/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031661)

### África
* 11/03/2025 | Johannesburgo, ZA | [Reunión de Rust en Johannesburgo](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Las Etapas de la Gestión de Errores**](https://www.meetup.com/johannesburg-rust-meetup/events/306437452)

### Asia
* 15/03/2025 | Pekín, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**KCD Beijing 2025**](https://www.meetup.com/wasm-rust-meetup/events/303112196)
* 19/03/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust March 2025 en Jit en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)
* 2025-03-28 | Kowloon Tong, HK | [Rust de Asia](https://www.rustasiaconf.com/)
    * [**Rust Asia 2025**](https://www.rustasiaconf.com/)

### Europa
* 05/03/2025 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**17º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/305887675)
* 05/03/2025 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en marzo: Edición Rust 2024**](https://www.meetup.com/rustcologne/events/306446366)
* 05/03/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Hackathon**](https://www.meetup.com/oxford-rust-meetup-group/events/306541493/)
* 07/03/2025 | Praga, CZ | [Rust República Checa](https://www.meetup.com/rust-czech-republic/events/)
    * [**Encuentro de Rust en las oficinas de Braiins**](https://www.meetup.com/rust-czech-republic/events/306237623)
* 12/03/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 13/03/2025 | Biel, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #2 @ BFH Biel**](https://www.meetup.com/rust-bern/events/306169573)
* 14/03/2025 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París 2025**](https://www.rustinparis.com/schedule)
* 18/03/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #10 @ MDPI Basel**](https://www.meetup.com/rust-basel/events/306121044)
* 18/03/2025 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729673)
* 2025-03-20 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**¡Conversaciones de marzo! (Dos)**](https://www.meetup.com/rust-and-friends/events/306524042)
* 2025-03-20 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (marzo de 2025)**](https://www.meetup.com/rust-prague/events/306512157)
* 25/03/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/306478352)
* 25/03/2025 | Eindhoven, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Rust x Julia Meetup Eindhoven**](https://www.meetup.com/rust-nederland/events/306434865)
* 2025-03-26 | Varsovia, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon**](https://www.rustikon.dev/)
* 27/03/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #12: Probando en Rust**](https://rust-augsburg.github.io/meetup/Meetup_12.html)
* 02/04/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 1 - híbrido**](https://www.meetup.com/rust-munich/events/306097261)
* 02/04/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541535)

### América del Norte
* 06/03/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Marzo Social Mensual**](https://www.meetup.com/rust-montreal/events/306518386)
* 06/03/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/306183467)
* 06/03/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**CRDTs en Rust**](https://www.meetup.com/stl-rust/events/305187783)
* 2025-03-10 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en Davis Square, 10 de marzo**](https://www.meetup.com/bostonrust/events/305805192)
* 13/03/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306484710)
* 18/03/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638264)
* 18/03/2025 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Encuentro mensual: Introducción a Rust y Python; ¡Usando Rustup, Cargo y Rust!**](https://www.meetup.com/spokane-rust/events/306368210)
* 2025-03-20 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/306473234)
* 2025-03-20 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Marzo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658448)
* 21/03/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y AWS Lambda. Preparando el camino para desplegar ML/AI**](https://www.meetup.com/rust-mx/events/306406018)
* 2025-03-26 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcfbjc)
* 27/03/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/306470345)
* 31/03/2025 | Boulder, CO, EE. UU. | [Depósito de estado sólido](https://www.meetup.com/solidstatedepot/?eventOrigin=event_home_page)
    * [**Boulder Rust: Bryan presenta Rusted Hardware**](https://www.meetup.com/solidstatedepot/events/306573447)

### Oceanía
* 11/03/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Rust en Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306262384)

### América del Sur
* 15/03/2025 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na CloudWalk**](https://www.meetup.com/rust-sao-paulo-meetup/events/306034427)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ivrkhs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> El impacto en el rendimiento de pasar a Rust, y este es un tema común en todo lo que se hace cuando pasamos de C/C++ a Rust, vimos una mejora del rendimiento del 5 al 15%.
>
> diré que una de las formas en que podrías atacar ese tipo de estadística es decir, bueno, la reescribiste, así que cada vez que reescribas algo, lo vas a mejorar, y si lo hubieras reescrito en C o C++, también habrías visto una mejora como esa, pero el hecho es que no teníamos la intención de obtener una mejora en el rendimiento. Esto fue puramente un ejercicio de portabilidad y lo vimos ahora.
>
> Y el otro aspecto de esto es que nunca vemos regresiones de rendimiento cuando estamos haciendo nuestros ports [...]

– [Mark Russinovich en RustNationUK '25'](https://youtu.be/1VgptLwP588?feature=shared&t=414)

A pesar de la falta de sugerencias, llogiq está bastante satisfecho con su elección.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1j5bpz8/this_week_in_rust_589/)</small>
