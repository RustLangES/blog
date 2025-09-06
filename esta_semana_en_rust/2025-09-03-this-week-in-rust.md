---
title: "Esta semana en Rust #74"
number_of_week: 74
description: El crate de esta semana es aehobak, un transcodificador para parches binarios bsdiff.
date: 2025-09-03
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

### Fundación
* [Dando la bienvenida al Laboratorio de Innovación de Rust | Blog de Rust](https://blog.rust-lang.org/2025/09/03/welcoming-the-rust-innovation-lab/)

### Boletines
* [El Problema de Rustacean Incrustado #53](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-53)
* [Rust Trends Issue #69: El año de avance empresarial de Rust](https://rust-trends.com/newsletter/rust-enterprise-breakthrough-2025)

### Actualizaciones de proyectos/herramientas
* [El camino a SeaQuery 1.0](https://www.sea-ql.org/blog/2025-08-30-sea-query-1.0/)
* [rkik v1.0.0 - CLI de inspección NTPv4/v6 sin estado en Rust](https://github.com/aguacero7/rkik)
* [Introducción a la publicación segura de la carga](https://blog.weiznich.de/blog/cargo-safe-publish/)
* [Seguimiento de la confianza con Rust en el kernel](https://lwn.net/SubscriberLink/1034603/5dcfecdd5e3af0c2/)
* [Slint 1.13 lanzado con vista previa en vivo para Rust y C ++](https://slint.dev/blog/slint-1.13-released)

### Observaciones/Pensamientos
* [Mordeduras de algoritmo de Rust - Recorrido de orden de nivel de árbol binario](https://d34dl0ck.me/rust-algorithm-bites-binary-tree-level-order-traversal/index.html)
* [Añadiendo derivar (De) a Rust](https://kobzol.github.io/rust/2025/09/02/adding-derive-from-to-rust.html)
* [Por qué construí TLQ (Tiny Little Queue)](https://nebjak.dev/blog/why-i-built-tlq-tiny-little-queue/)
* [Combinación de sintaxis literal de estructura con acceso a campos de solo lectura](https://kobzol.github.io/rust/2025/09/01/combining-struct-literal-syntax-with-read-only-field-access.html)
* [Tenga cuidado al copiar cadenas a cero con 'serde'](https://bd103.github.io/blog/2025-09-01-zero-copying-strings-serde)
* [Elefantes para el desayuno: probando funciones no comprobables, un bocado a la vez](https://bitfieldconsulting.com/posts/elephants-for-breakfast) 
* [Desenredando los detalles: resolución de símbolos en Rusty Trap](https://system.joekain.com/2025/08/31/demangling-the-details-symbol-resolution.html)
* [video] [Rust 1.89.0](https://youtu.be/C5RHSqYIR7w)

### Tutoriales de Rust
* [Escribamos una macro en Rust - Parte 2](https://hackeryarn.com/post/rust-macros-2/)
* [Cómo configurar el registro de Rust en AWS Lambda para AWS CloudWatch](https://forgestream.idverse.com/blog/20250902-cloudwatch-rust-logging/)
* [Creación de una aplicación de tareas pendientes en GPUI](https://blog.0xshadow.dev/posts/learning-gpui/gpui-todo-app/)

### Investigación
* [Compartir una referencia mutable con Python](https://blog.lilyf.org/posts/python-mutable-reference/)
* [Faster Rust se compila en Mac](https://nnethercote.github.io/2025/09/04/faster-rust-builds-on-mac.html)
* [Trucos de rendimiento de Rust](https://davidlattimore.github.io/posts/2025/09/02/rustforge-wild-performance-tricks.html)
* [video] [Taller de Rust incrustado](https://www.youtube.com/live/PZZfVAaYTP8?si=2nfis0-IrN9aMkti)
* [video] [RustCurious 1: Por qué Rust es seguro: una introducción novedosa a la propiedad y los préstamos](https://www.youtube.com/watch?v=lVWiHIVXG2c)

### Miscelánea
* [Elementos de Rust: Un mapa completo del sistema de tipos Rust](https://rustcurious.com/elements/)
* [filtra.io entrevista con el equipo de 'Tonari' | Abriendo portales con Rust](https://filtra.io/rust/interviews/tonari-aug-25)
* [audio] [Hyper con Sean McArthur](https://www.youtube.com/watch?v=aw9lvs3PhWQ)
* [video] [Seminario de Berkeley | La búsqueda del rendimiento](https://youtu.be/k_-6KI3m31M?si=JDZTHRDTs-unM34A)

## Crate de la semana

El crate de esta semana es [aehobak](https://crates.io/crates/aehobak), un transcodificador para parches binarios bsdiff.

¡Gracias a [David Michael Barr](https://users.rust-lang.org/t/crate-of-the-week/2704/1465) por la sugerencia!

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

*No se enviaron convocatorias de participación esta semana.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 383 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-08-26..2025-09-02

#### Compilador
* [detectar la falta 'if let' o 'let-else'](https://github.com/rust-lang/rust/pull/145582)
* [arreglar '#[loop_match]' en bucle divergente](https://github.com/rust-lang/rust/pull/144783)

#### Biblioteca
* [añadir 'Bound::copied'](https://github.com/rust-lang/rust/pull/145968)
* [añadir 'Duración::from_nanos_u128'](https://github.com/rust-lang/rust/pull/145969)
* [añadir 'Opción::reducir'](https://github.com/rust-lang/rust/pull/144274)
* [implementar Suma y Producto para 'Saturación(u*)](https://github.com/rust-lang/rust/pull/144275)
* [implementación: '#[feature(nonpoison_condvar)]'](https://github.com/rust-lang/rust/pull/144651)
* [optimizar '.ilog({2,10})' a '.ilog{2,10}()'](https://github.com/rust-lang/rust/pull/145776)
* [str: Función de estabilización 'round_char_boundary'](https://github.com/rust-lang/rust/pull/145756)

#### Carga
* ['fix(cli)': Mostrar la ruta de manifiesto incorrecta](https://github.com/rust-lang/cargo/pull/15896)
* [agregar más contexto al mensaje de error de publicación](https://github.com/rust-lang/cargo/pull/15879)
* [feat: no se detenga al primer error al emitir lints y advertencias](https://github.com/rust-lang/cargo/pull/15889)

#### Clippy
* ['map_identity': sugerir hacer que la variable sea mutable cuando sea necesario](https://github.com/rust-lang/rust-clippy/pull/15268)
* ['unit_cmp': no pelusa en la unidad explícitamente escrita expr](https://github.com/rust-lang/rust-clippy/pull/15562)
* [permitir '--print=niveles-de-pelusa-raíz-caja'](https://github.com/rust-lang/rust-clippy/pull/15567)
* ['assertions_on_result_states' evitar cambiar el tipo de retorno en más casos](https://github.com/rust-lang/rust-clippy/pull/15591)
* ['collapsible_match' sugerir ref/derefs cuando sea necesario](https://github.com/rust-lang/rust-clippy/pull/14221)
* [habilitar 'clippy::p anic' en contextos const](https://github.com/rust-lang/rust-clippy/pull/15565)
* [corregir el falso positivo de 'needless_range_loop' al reunirse con una matriz multidimensional](https://github.com/rust-lang/rust-clippy/pull/15486)
* [corregir 'alloc_instead_of_core' falso positivo cuando 'alloc' es un alias](https://github.com/rust-lang/rust-clippy/pull/15581)
* [corregir 'needless_for_each' sugiriendo erróneamente con tipos de entrada de cierre explícitos](https://github.com/rust-lang/rust-clippy/pull/15595)
* [corregir 'print_literal' sugiriendo erróneamente para un literal en línea después de un argumento numerado](https://github.com/rust-lang/rust-clippy/pull/15583)
* [arreglar 'redundant_closure' sugiere erróneamente con la sobrecarga de deref](https://github.com/rust-lang/rust-clippy/pull/15077)
* [suprimir 'excessive_precision' cuando las constantes son demasiado precisas](https://github.com/rust-lang/rust-clippy/pull/15193)

#### Analizador de Rust
* [agregar barras de progreso a más lugares en análisis-estadísticas](https://github.com/rust-lang/rust-analyzer/pull/20560)
* [adjuntar la base de datos en un lugar más en el resaltado](https://github.com/rust-lang/rust-analyzer/pull/20553)
* [evite que la opción '--target' se dé dos veces a 'rustc' cuando se invoca a través de 'cargo rustc' mientras obtiene el diseño de datos de destino](https://github.com/rust-lang/rust-analyzer/pull/20579)
* [deduplicar métodos en finalización por ID de función y no por nombre](https://github.com/rust-lang/rust-analyzer/pull/20587)
* [en 'highlight_related', cuando esté en un bloque inseguro, no resalte las operaciones inseguras de otros bloques inseguros](https://github.com/rust-lang/rust-analyzer/pull/20547)
* [al mapear el tipo 'dyn' de next-solver, agregue 'Self' (también conocido como var vinculado ^ 1.0) a las sustituciones de rasgos automáticos](https://github.com/rust-lang/rust-analyzer/pull/20563)
* [resolución de rasgos de caché en consultas en la misma revisión](https://github.com/rust-lang/rust-analyzer/pull/20527)

### Triaje de rendimiento del compilador de Rust

Una semana relativamente tranquila. [#144841](https://github.com/rust-lang/rust/pull/144841) agregó un
Optimización para construcciones incrementales que proporcionaron una muy buena victoria para la caja 'Nalgebra'. [#143290](https://github.com/rust-lang/rust/pull/143290) debería ayudar a evitar la creación de instancias de funciones asíncronas repetidamente en
cajas aguas abajo.

Triaje realizado por **@kobzol**.. -
Rango de revisión: [ee361e8f.. 75ee9ffd](https://perf.rust-lang.org/?start=ee361e8fca1c30e13e7a31cc82b64c045339d3a8&end=75ee9ffd5ed3649c0a09493057adaa8feebb2035&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,2%, 0,4%] | 7 |
| Regresiones ❌ <br /> (secundaria) | 2.0% | [0,1%, 13,6%] | 30 |
| Mejoras ✅ <br /> (primaria) | -1,9% | [-7,0%, -0,3%] | 17 |
| Mejoras ✅ <br /> (secundario) | -0,7% | [-1,7%, -0,1%] | 23 |
| Todos ❌✅ (primarios) | -1,2% | [-7,0%, 0,4%] | 24 |

1 Regresión, 3 Mejoras, 6 Mixto; 5 de ellos en rollups
45 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/27c08698d3d9cb15081459cf61385d52958e14ac/triage/2025/2025-09-02.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [no aplique reglas de extensión temporal de por vida a 'super let' no extendido](https://github.com/rust-lang/rust/pull/145838)
* [disposición: no especificada] [Estabilizar 'new_zeroed_alloc'](https://github.com/rust-lang/rust/pull/144091)
* [No materializar X en [X; 0] cuando X está desdimensionando una const](https://github.com/rust-lang/rust/pull/145277)
* [Rechazar sufijos literales no válidos en la indexación de tuplas, la indexación de estructuras de tuplas y la posición del nombre de campo de estructura](https://github.com/rust-lang/rust/pull/145463)
* [Estabilizar 'std::p anic::Location::file_as_c_str'](https://github.com/rust-lang/rust/pull/145664)
* [Corregir backtraces con '-C panic=abort' en linux; emitir tablas de desenredo por defecto](https://github.com/rust-lang/rust/pull/143613)
* [Iterador especializado::eq{_by} para iteradores TrustedLen](https://github.com/rust-lang/rust/pull/137122)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Rusty Eventos entre 2025-09-03 - 2025-10-01 🦀

### Virtual
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**¡Evento híbrido con Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/310730387)
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
* 2025-09-15 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [** Configuración de Tock OS en un entorno virtual (en línea) - preparación para el 17 de septiembre **](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-16 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)
* 2025-09-18 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-09-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361443)
* 2025-09-25 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-10-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)

### África
* 2025-09-09 | Johannesburgo, ZA | [Reunión de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Rust by Example - Primitivas y tipos personalizados**](https://www.meetup.com/johannesburg-rust-meetup/events/310714835)

### Asia
* 2025-09-13 | Hangzhou, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP aún está abierto)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)
* 2025-09-13 - 2025-09-14 | Hangzhou, CN | [GOSIM](https://hangzhou2025.gosim.org/schedule/)
    * [**GOSIM Hangzhou 2025**](https://dev.events/conferences/rust-global-china-and-rust-china-conf-2025-dscrf0e1)
* 2025-09-17 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust septiembre de 2025 en Varonis en Herzeliya**](https://www.meetup.com/rust-tlv/events/310708628)

### Europa
* 2025-09-03 | Dortmund, DE | [Rust, Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust para el desarrollo de software crítico para la seguridad y otros casos de uso de alto potencial**](https://www.meetup.com/rust-dortmund/events/308517658)
* 2025-09-03 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**¿Quieres un lenguaje exprimible / moderno / útil / amplio? Elija cuatro**](https://www.meetup.com/rust-and-friends/events/310536614)
* 2025-09-03 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**De los errores al paralelismo y a la preparación para el futuro: lo que hace diferente a Rust**](https://www.meetup.com/rust-rhein-main/events/310322369)
* 2025-09-03 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
  * [**Diversión de septiembre**](https://www.meetup.com/oxford-rust-meetup-group/events/310579981)
* 2025-09-04 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️ 🌈 - Edición 006**](https://www.meetup.com/rust-berlin/events/310800817)
* 2025-09-04 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Reunión de Rust Gdansk #10**](https://www.meetup.com/rust-gdansk/events/310610993)
* 2025-09-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 - 2025-09-18 | Berlín, DE | [Conferencia Oxidar](https://oxidizeconf.com/)
    * [**Conferencia Oxidize**](https://oxidizeconf.com/)
* 2025-09-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)
* 2025-09-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charlas en Mjolner Informatics**](https://www.meetup.com/rust-aarhus/events/310562343)
* 2025-09-24 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3 - híbrido**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Encuentro de Rust Moravia (¡En la capital!)**](https://www.meetup.com/rust-moravia/events/310743282)

### América del Norte
* 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
    * [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 1)**](https://www.meetup.com/desert-rustaceans/events/310345446)
* 2025-09-04 | Montreal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**Septiembre Mensual Social**](https://www.meetup.com/rust-montreal/events/310802460)
* 2025-09-04 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310547154)
* 2025-09-04 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**RustConf 2025 Watch Party (Día 2)**](https://www.meetup.com/desert-rustaceans/events/310345459)
* 2025-09-04 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**emulación de sistemas retro (NES, Gameboy) en Rust**](https://www.meetup.com/stl-rust/events/310116988)
* 2025-09-06 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Beacon Hill Rust Lunch, 6 de septiembre **](https://www.meetup.com/bostonrust/events/310106310)
* 2025-09-09 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Geometría en Rust at Motif + Rust en RISC-V/ESP32**](https://www.meetup.com/rust-nyc/events/310795569)
* 2025-09-10 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> JS**](https://www.meetup.com/desert-rustaceans/events/310669989)
* 2025-09-11 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Laberintos y gráficos en Rust**](https://www.meetup.com/utah-rust/events/310674937)
* 2025-09-11 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223)
* 2025-09-14 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, 14 de septiembre **](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Taller de Rust de Vara Network**](https://luma.com/1bii0kv7)
* 2025-09-17 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587) | [**Evento de configuración en línea 15 de septiembre **](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-18 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Serie 3 : Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333261)
* 2025-09-18 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-25 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl en la taberna Manuels**](https://www.meetup.com/rust-atl/events/308675983)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> ¡Errores como este son los peores! Es casi imposible detectarlos en el desarrollo, porque nunca hay suficiente carga en el sistema para obligar al programador a mover la ejecución a otro subproceso. Entonces, terminas con uno de estos errores "imposibles de reproducir, fallas a veces, pero nunca para ti".
>
> Es alucinantemente genial que el compilador de Rust pueda detectar algo como esto. Y que partes aparentemente no relacionadas del lenguaje, como las exclusiones mutuas, las vidas y las operaciones asíncronas, forman un sistema tan coherente.

– [Bernard Kolobara en su blog](https://lubeno.dev/blog/rusts-productivity-curve)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1711) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1n8du9a/this_week_in_rust_615/)</small>
