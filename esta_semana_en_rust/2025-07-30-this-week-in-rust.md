---
title: "Esta semana en Rust #69"
number_of_week: 69
description: El crate de esta semana es qop, una herramienta de migración de SQL independiente.
date: 2025-07-30
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de _This Semana en Rust_!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software confiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíenos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

_This Week in Rust_ se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabe qué categoría usar, no dude en enviar un PR de todos modos y simplemente pida a los editores que seleccionen la categoría. -->

### Fundación

* [RustConf 2025 ya casi está aquí - ¡Regístrese ahora!](https://rustfoundation.org/media/rustconf-2025-is-almost-here-register-now/)

### Actualizaciones de proyectos/herramientas

* [Biblioteca de sensores DHT integrados](https://rust-dd.com/post/building-a-rust-library-for-dht11-sensor-a-step-by-step-guide)
* [Rust eBooks Nightly: libros de Rust siempre actualizados en EPUB, AZW3, MOBI, PDF](https://artur-sulej.github.io/rust-ebooks/)
* [¡Git-cliff 2.10.0 ha sido lanzado! (un generador de registro de cambios altamente personalizable)](https://git-cliff.org/blog/2.10.0)
* [Nutype 0.6.2](https://github.com/greyblake/nutype/releases/tag/v0.6.2) - admite el atributo 'derive_unsafe' para derivar rasgos arbitrarios de terceros.

### Observaciones/Pensamientos

* [Avance en el rendimiento de Rust Async Web Framework (9247)](https://dev.to/member_8c78b76f/rust-async-web-framework-performance-breakthrough9247-4l22)

### Tutoriales de Rust

* [Uso de ESP32 como principiante de Rust](https://rust-dd.com/post/introduction-to-embedded-systems-with-rust-a-beginner-s-guide-using-esp32)
* [El protocolo de señal explicado #2: Implementación del algoritmo de doble trinquete en Rust](https://kerkour.com/signal-protocol-double-ratchet-rust)
* [Vibe codifica un proxy MCP de Rust en VSCode con GitHub Copilot](https://awakecoding.com/posts/vibe-coding-a-rust-mcp-proxy-in-vscode-with-github-copilot/)
* [Programación de tipos de datos extensibles en Rust con CGP - Parte 4](https://contextgeneric.dev/blog/extensible-datatypes-part-4/)

### Miscelánea

* [100 ejercicios para aprender Rust: RustRover Edition](https://blog.jetbrains.com/education/2025/07/28/rust-exercises-rustrover/)
* [Encuesta de Stackoverflow: Rust es una vez más el lenguaje de programación más admirado](https://survey.stackoverflow.co/2025/technology/#admired-and-desired)

## Crate de la semana

El crate de esta semana es [qop](https://github.com/cchexcode/qop), una herramienta de migración de SQL independiente.

¡Gracias a [Alexander Weber](https://users.rust-lang.org/t/crate-of-the-week/2704/1454) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas

Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

- _No convocatorias de pruebas fueron emitidas esta semana por [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing)._

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

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]: https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

428 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-22..2025-07-29

#### Compilador

- [evitar llamadas innecesarias 'new_adt'/'new_fn_def'](https://github.com/rust-lang/rust/pull/144425)
- ['loop_match': sugerir extraer a un elemento 'const'](https://github.com/rust-lang/rust/pull/143585)

#### Biblioteca

- [añadir 'Rev::into_inner'](https://github.com/rust-lang/rust/pull/144278)
- [str: marcar funciones de característica inestables 'round_char_boundary' como const](https://github.com/rust-lang/rust/pull/144472)

#### Carga

- [schema: Exponer 'IndexPackage', la descripción de un paquete dentro de un índice de registro](https://github.com/rust-lang/cargo/pull/15770)
- [permitir el uso de Cargo como biblioteca con el backend reqwest de gix](https://github.com/rust-lang/cargo/pull/15653)
- [corrección: 'no-proc-macro' es anulado por bordes posteriores](https://github.com/rust-lang/cargo/pull/15764)
- [tiempos: hacer que los gráficos sean escalables a la ventana del usuario](https://github.com/rust-lang/cargo/pull/15766)
- [use 'gix' para 'paquete de carga'](https://github.com/rust-lang/cargo/pull/15534)

#### Rustdoc

- [rustdoc: agregar formas de colapsar todos los bloques de impl](https://github.com/rust-lang/rust/pull/141663)

#### Clippy

- ['cast-lossless' no debe sugerir cuando el tipo de conversión es de entrada de macro](https://github.com/rust-lang/rust-clippy/pull/15358)
- [mensaje de ayuda correcto para 'arc_with_non_send_sync'](https://github.com/rust-lang/rust-clippy/pull/15332)
- [detectar atributos prefijados como duplicados](https://github.com/rust-lang/rust-clippy/pull/15212)
- [arreglar 'empty_structs_with_brackets' sugiriendo erróneamente sobre genéricos](https://github.com/rust-lang/rust-clippy/pull/15355)
- [corregir el falso positivo 'if_then_some_else_none' cuando se requiere coerción de tipos](https://github.com/rust-lang/rust-clippy/pull/15267)
- [corregir 'ip_constant' cuando la llamada se envuelve entre paréntesis adicionales](https://github.com/rust-lang/rust-clippy/pull/15339)
- [corregir 'let_unit_value' sugiriendo erróneamente para macros de formato](https://github.com/rust-lang/rust-clippy/pull/15085)
- [corregir 'match_single_binding' manejando mal el alcance](https://github.com/rust-lang/rust-clippy/pull/15060)
- [corregir el falso positivo 'module_name_repetitions' en las macros exportadas](https://github.com/rust-lang/rust-clippy/pull/15319)
- [arreglar 'unused_async' falso positivo en la función con 'todo!'](https://github.com/rust-lang/rust-clippy/pull/15308)
- ['unnecessary_map_or': no agregue paréntesis si el padre expr...](https://github.com/rust-lang/rust-clippy/pull/15345)

#### Analizador de Rust

- [agregar ide-assist: 'generate_impl_trait' por 'generate_impl'](https://github.com/rust-lang/rust-analyzer/pull/19938)
- [cambiar el nombre de sí mismo al parámetro usar el tipo 'Self'](https://github.com/rust-lang/rust-analyzer/pull/20285)
- [corregir el espacio en blanco 'generate_trait_from_impl' después de vis](https://github.com/rust-lang/rust-analyzer/pull/20297)
- [corregir el comentario del documento de plegado para la lista de parámetros multilínea fn](https://github.com/rust-lang/rust-analyzer/pull/20302)
- [considere todos los artefactos producidos para la búsqueda de proc-macro dylib](https://github.com/rust-lang/rust-analyzer/pull/20319)
- [corregir la comprobación incorrecta de la versión del script de compilación](https://github.com/rust-lang/rust-analyzer/pull/20317)
- [corregir runnables extra env que no sustituye a las variables de entorno](https://github.com/rust-lang/rust-analyzer/pull/20313)
- [ignorar los límites de 'Destruir' nuevamente](https://github.com/rust-lang/rust-analyzer/pull/20318)
- [analizar 'para<'a> [const]'](https://github.com/rust-lang/rust-analyzer/pull/20281)
- [use 'TempDir' para archivos de bloqueo copiados](https://github.com/rust-lang/rust-analyzer/pull/20290)

### Triaje de rendimiento del compilador de Rust

Una semana con muchos resultados mixtos, incluidos algunos puntos de referencia que parecen ser
nuevo bimodal, lo que hizo que algunos de los resultados parecieran más cercanos al ruido que a la señal.
En general, sin embargo, la semana terminó con una ligera mejora.

Triaje realizado por **@simulacrum**.
Rango de revisión: [3f9f20f7.. E3514BDE](https://perf.rust-lang.org/?start=3f9f20f71dd945fe7d044e274094a53c90788269&end=e3514bde96d2d13586337a48db77fa64b850d249&absolute=false&stat=instructions%3Au)

1 Regresiones, 2 Mejoras, 9 Mixtos; 2 de ellos en rollups
38 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-07-28.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

- _No RFC fueron aprobados este week._

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

- [Permitir que el asignador global use el almacenamiento local de subprocesos y std::thread::current()](https://github.com/rust-lang/rust/pull/144465)
- [Problema de seguimiento para str::{floor, ceil}\_char_boundary](https://github.com/rust-lang/rust/issues/93743)
- [Comprobar las variaciones ascendentes de corrutina en la restricción dtorck](https://github.com/rust-lang/rust/pull/144156)
- [Problema de seguimiento para aritmética y ciertas operaciones bit a bit en 'AtomicPtr'](https://github.com/rust-lang/rust/issues/99108)
- [Agregar pelusa contra punteros colgantes de variables locales](https://github.com/rust-lang/rust/pull/144322)
- ['apply_member_constraints': corregir la verificación de marcador de posición](https://github.com/rust-lang/rust/pull/142071)
- [Eliminar el atributo '#[no_sanitize]' a favor de '#[sanitize(xyz = "on|off")']](https://github.com/rust-lang/rust/pull/142681)
- [Puerto '#[should_panic]' a la nueva infraestructura de análisis de atributos](https://github.com/rust-lang/rust/pull/143808)
- [emita 'StorageLive' y programe 'StorageDead' para 'let' - enlaces de 'else' después de la coincidencia](https://github.com/rust-lang/rust/pull/143028)
- [enlaces de patrones inferiores en el orden en que están escritos y orden de caída base en el orden de los enlaces primarios](https://github.com/rust-lang/rust/pull/143764)
- [Actualizar semicolon_in_expressions_from_macros de advertencia a denegación](https://github.com/rust-lang/rust/pull/144369)
- [Estabilizar const TypeId::of](https://github.com/rust-lang/rust/pull/144133)
- [Marcar todas las pelusas de obsolescencia en la resolución de nombres como denegar por defecto e informar en deps](https://github.com/rust-lang/rust/pull/143929)
- [Problema de seguimiento para la aritmética que entra en pánico en el desbordamiento (operaciones 'strict_*')](https://github.com/rust-lang/rust/issues/118260)
- [[rustdoc] Mostrar attrs inseguros con envoltorios 'unsafe()' de la edición 2024.](https://github.com/rust-lang/rust/pull/143662)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),

- [Degradar 'x86_64-apple-darwin' del Nivel 1 al Nivel 2 con herramientas de host](https://github.com/rust-lang/rfcs/pull/3841)
- [RFC: habilitar 'derivar (Desde)' para estructuras de un solo campo](https://github.com/rust-lang/rfcs/pull/3809)

_No Items entraron en el período de comentarios finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)._

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

- [nuevo] [Degradar x86_64-apple-darwin de Nivel 1 a Nivel 2 con herramientas de host](https://github.com/rust-lang/rfcs/pull/3841)

## Próximos eventos

Rusty Eventos entre 2025-07-30 - 2025-08-27 🦀

### Virtual

- 2025-07-30 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
  - [**Jan Arends: Cómo convertir el código de espagueti en una arquitectura gourmet**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/310006696)
- 2025-07-31 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
  - [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
- 2025-08-02 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
  - [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
- 2025-08-03 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
- 2025-08-06 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
  - [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)
- 2025-08-10 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbnb)
- 2025-08-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361531)
- 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
  - [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
- 2025-08-14 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
  - [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
- 2025-08-17 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
- 2025-08-19 | Virtual (Santa Clara, CA, EE. UU.) | [Comunidad de Extensión de la UCSC](https://www.meetup.com/ucsc-extension-community/events/)
  - [**Programación con Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
- 2025-08-19 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
  - [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757756)
- 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
  - [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
- 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
  - [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
- 2025-08-21 | Virtual (Londres, Reino Unido) | [Conf42: Eventos tecnológicos en línea](https://www.meetup.com/conf42/events/)
  - [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
- 2025-08-21 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
  - [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
- 2025-08-24 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
- 2025-08-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361442)
- 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)

### Europa

- 2025-07-30 | Ámsterdam, Países Bajos | [Grupo de Desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
  - [**Reunión de Rust @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
- 2025-07-31 | Augsburgo, DE | [Encuentro de Rust en Augsburgo](https://rust-augsburg.github.io/meetup)
  - [**Rust Meetup #14: Prof. Dra. Claudia Meitinger - Embajada - Möglichkeiten und Herausforderungen im Modul "Proyecto Interdisciplinario"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)
- 2025-07-31 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
  - [**Reunión de Rust #59**](https://www.meetup.com/copenhagen-rust-community/events/310147999)
- 2025-08-06 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
  - [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
- 2025-08-06 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
  - [**Alastair Harrison: Control de versiones para la era agéntica.**](https://www.meetup.com/oxford-rust-meetup-group/events/310101048)
- 2025-08-13 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
  - [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
- 2025-08-13 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
  - [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944036)
- 2025-08-16 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
  - [**Rust Embedded - Taller #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
- 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
  - [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
- 2025-08-19 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
  - [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
- 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
  - [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)

### América del Norte

- 2025-07-31 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
  - [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)
- 2025-07-31 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
  - [**Rust en el grupo de desarrolladores de Web3**](https://www.meetup.com/rust-los-angeles/events/310240265)
- 2025-08-01 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
  - [**Rust Lunch - Loop Edition**](https://www.meetup.com/chicago-rust-meetup/events/310199297)
- 2025-08-02 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
  - [**Central Cambridge Rust Lunch, 2 de agosto**](https://www.meetup.com/bostonrust/events/310106288)
- 2025-08-05 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc)
  - [**Rust NYC: Validación/Optimización de consultas de base de datos con tipos y Rust en Enterprise AI**](https://www.meetup.com/rust-nyc/events/310107945)
- 2025-08-07 | Montreal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
  - [**Social mensual de agosto**](https://www.meetup.com/rust-montreal/events/310259905)
- 2025-08-07 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
  - [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310030338)
- 2025-08-07 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
  - [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
- 2025-08-08 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
  - [**Almuerzo de Rust del noreste, 8 de agosto **](https://www.meetup.com/bostonrust/events/310106298)
- 2025-08-12 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
  - [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)
- 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
  - [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
- 2025-08-14 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust)
  - [**Programación de un robot de combate en Rust con Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
- 2025-08-18 | Denver, CO, EE. UU. | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
  - [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
- 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
  - [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
- 2025-08-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
  - [**Rust on Bare Metal Serie 2: Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333117)
- 2025-08-23 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
  - [**Almuerzo de Rust de Somerville Union Square, 23 de agosto **](https://www.meetup.com/bostonrust/events/310106302)
- 2025-08-27 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
  - [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310205991)
- 2025-09-02 - 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)

### Oceanía

- 2025-08-11 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group)
  - [**Reunión de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/308880707)
- 2025-08-26 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
  - [**Reunión de agosto**](https://www.meetup.com/rust-canberra/events/308746519)
- 2025-08-27 - 2025-08-30 | Wellington, Nueva Zelanda | [Forja de Rust](https://rustforgeconf.com/)
  - [**Forja de Rust**](https://rustforgeconf.com/)

### América del Sur

- 2025-08-07 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
  - [**Rust Uruguay meetup de Agosto**](https://www.meetup.com/rust-uruguay/events/310004109)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> La misma lógica de cómputo se ejecuta en todos los destinos, escrita completamente en Rust normal. No se utilizan lenguajes de sombreador o kernel.

– [Christian Legnitto en el blog de rust-gpu](https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/) mostrando una demostración compilando Rust en todas las principales plataformas de GPU + web.

A pesar de la falta de sugerencias, llogiq está notablemente satisfecho con su elección.

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

_This Week in Rust está editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)_

_Email alojamiento de la lista está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)_

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1mec821/this_week_in_rust_610/)</small>
