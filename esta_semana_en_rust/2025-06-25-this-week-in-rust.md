---
title: "Esta semana en Rust #65"
number_of_week: 65
description: El crate de esta semana es primitive\_fixed\_point\_decimal, una caja de tipos decimales de punto fijo *reales*.
date: 2025-06-25
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
* [Anunciando la congelación de la función Clippy](https://blog.rust-lang.org/inside-rust/2025/06/21/announcing-the-clippy-feature-freeze/)

### Boletines
* [Tendencias de Rust Edición #67](https://rust-trends.com/newsletter/untangling-rust-errors-the-bzip2-rewrite/)

### Actualizaciones de proyectos/herramientas
* [Tantivy 0.24](https://quickwit.io/blog/tantivy-0.24)
* [Cómo escribir Rust en el kernel: parte 1](https://lwn.net/SubscriberLink/1024202/556fa7b3c51d7899/)
* [GlueSQL v0.17.0 - Añadido soporte de almacenamiento redb](https://github.com/gluesql/gluesql/releases/tag/v0.17.0)

### Observaciones/Pensamientos
* [La Efectividad Irrazonable del Fuzzing para Portar Programas](https://rjp.io/blog/2025-06-17-unreasonable-effectiveness-of-fuzzing)
* [¿Así que quieres serializar algo de DER?](https://alexgaynor.net/2025/jun/20/serialize-some-der/)
* [Por qué cambié de Flutter + Rust a Rust + egui](https://jdiaz97.github.io/greenblog/posts/flutter_to_egui/)
* [Expresiones extrañas en Rust](https://www.wakunguma.com/blog/rust-weird-expr)
* [Migrar de Tokio heredado a escala](https://www.okta.com/blog/2024/11/migrating-off-legacy-tokio-at-scale/)
* [Impulsando el compilador de Rust para compilar archivos individuales como shellcode](https://kirchware.com/Driving-the-Rust-Compiler-to-Compile-Single-Files-as-Shellcode)
* [Servicio de mostrador: Cómo lo reescribimos en Rust](https://engineering.grab.com/counter-service-how-we-rewrote-it-in-rust)
* [Defendiendo las democracias con Rust](https://filtra.io/rust/interviews/helsing-jun-25)
* [Rust: Un lenguaje que crece contigo, tu carrera y tus proyectos](https://kerkour.com/rust-grows-with-you)
* [lista de reproducción de vídeos] [Computación científica en Rust 2025](https://www.youtube.com/watch?v=XyXMKuclTcQ&list=PLrueqeouhcZNRW7H26DfscFjGSf0Pzd8c)

### Tutoriales de Rust
* [Portar sombreadores de GPU a Rust 30 veces más rápido con IA](https://rust-gpu.github.io/blog/2025/06/24/vulkan-shader-port/)
* [Compresión de ADN bit a bit en Rust: huella pequeña con complementos inversos rápidos](https://arianfarid.me/articles/dna-compression.html)
* [Escribir un controlador de dispositivo Linux básico cuando no sabe nada sobre controladores de Linux o USB](https://crescentro.se/posts/writing-drivers/)
* [Reescribiendo a Kafka en Rust Async: Perspectivas y lecciones aprendidas en Rust](https://wangjunfei.com/2025/06/18/Rewriting-Kafka-in-Rust-Async-Insights-and-Lessons-Learned/)
* [El manual completo de seguridad de Rust](https://yevh.github.io/rust-security-handbook/)

## Crate de la semana

El crate de esta semana es [primitive\_fixed\_point\_decimal](https://docs.rs/primitive_fixed_point_decimal), una caja de tipos decimales de punto fijo *reales*.

¡Gracias a [Wu Bingzheng](https://users.rust-lang.org/t/crate-of-the-week/2704/1445) por la autosugerencia!

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

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
* [Continuwuity - ACLs de sala predeterminadas](https://forgejo.ellis.link/continuwuation/continuwuity/issues/775)
* [Continuidad - Posibilidad de desactivar por completo la escritura y lectura de recibos](https://forgejo.ellis.link/continuwuation/continuwuity/issues/821)
* [Continuwuity - error: los usuarios de appservice no se crean al registrarse](https://forgejo.ellis.link/continuwuation/continuwuity/issues/813)
* [Continuwuity - Filtrado de invitaciones / deshabilitar invitaciones por cuenta](https://forgejo.ellis.link/continuwuation/continuwuity/issues/836)
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->
*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

448 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-17..2025-06-24

#### Compilador
* [perf: Almacenar en caché la *instanciación* canónica de param-envs](https://github.com/rust-lang/rust/pull/142316)
* [asyncDrop trait sin sync Drop genera un error](https://github.com/rust-lang/rust/pull/142606)
* [estabilizar 'generic_arg_infer'](https://github.com/rust-lang/rust/pull/141610)
* [Omitir pegamento de gota sin operación](https://github.com/rust-lang/rust/pull/142508)

#### Biblioteca
* [añadir los métodos 'trim_prefix' y 'trim_suffix' para los tipos 'slice' y 'str'](https://github.com/rust-lang/rust/pull/142331)
* [permitir comparaciones entre 'CStr', CString' y Cow<CStr>'](https://github.com/rust-lang/rust/pull/137268)
* [permitir almacenar 'format_args! ()' en variable](https://github.com/rust-lang/rust/pull/140748)
* [impl 'Default' for 'array::IntoIter'](https://github.com/rust-lang/rust/pull/141574)
* [cambiar la impl 'Default' de 'core::iter::Fuse' para hacer lo que sus documentos dicen que hace](https://github.com/rust-lang/rust/pull/140985)
* [deja que String pase '#[track_caller]' a sus llamadas Vec](https://github.com/rust-lang/rust/pull/142728)
* [implementación más segura de RepeatN](https://github.com/rust-lang/rust/pull/130887)
* [use una implementación 'ToString' distinta para 'u128' e 'i128'](https://github.com/rust-lang/rust/pull/142294)

#### Carga
* [cargo: 'feat(toml)': Soporte de análisis para múltiples scripts de compilación](https://github.com/rust-lang/cargo/pull/15630)
* [cargo: feat: introducir la opción perma unstable '--compile-time-deps' para 'cargo build'](https://github.com/rust-lang/cargo/pull/15674)
* [cargo: arreglar un posible punto muerto en 'CacheState::lock'](https://github.com/rust-lang/cargo/pull/15698)

#### Rustdoc
* [evite algunas asignaciones más en 'write_shared.rs'](https://github.com/rust-lang/rust/pull/142667)
* [rustdoc-json: mantener los argumentos genéricos vacíos si están entre paréntesis](https://github.com/rust-lang/rust/pull/142932)
* [rustdoc: hacer que srcIndex ya no sea una variable global](https://github.com/rust-lang/rust/pull/142100)

#### Clippy
* [usar jemalloc para Clippy](https://github.com/rust-lang/rust/pull/142286)
* [perf: No generes tantos compiladores (3/2) (19m → 250k)](https://github.com/rust-lang/rust-clippy/pull/15030)
* ['Sugg': no poner entre paréntesis un operador unario doble](https://github.com/rust-lang/rust-clippy/pull/14983)
* ['or_fun_call': lint más métodos](https://github.com/rust-lang/rust-clippy/pull/15071)
* [añadir el espacio que falta al expandir una variante similar a una estructura](https://github.com/rust-lang/rust-clippy/pull/15096)
* [verifique MSRV antes de sugerir aplicar 'const' a una función](https://github.com/rust-lang/rust-clippy/pull/15080)
* [emitir lint sobre el cierre redundante en el propio nodo de cierre](https://github.com/rust-lang/rust-clippy/pull/14791)
* [arreglar 'branches_sharing_code' sugiere engañosamente cuando está en la tarea](https://github.com/rust-lang/rust-clippy/pull/15076)
* [arreglar 'clippy::question_mark' en let-else con cfg](https://github.com/rust-lang/rust-clippy/pull/15082)
* [Corregir falso positivo 'exhaustive_structs' en estructuras con campo de valor predeterminado](https://github.com/rust-lang/rust-clippy/pull/15022)
* [arreglar 'manual_ok_err' sugiere erróneamente con referencias](https://github.com/rust-lang/rust-clippy/pull/15053)
* [arreglar 'non_copy_const' ICE](https://github.com/rust-lang/rust-clippy/pull/15083)
* [arreglar 'wildcard_enum_match_arm' sugiere erróneamente con identificadores sin procesar](https://github.com/rust-lang/rust-clippy/pull/15093)
* [corregir falso positivo de 'borrow_deref_ref'](https://github.com/rust-lang/rust-clippy/pull/14967)
* [arreglar sugerencia-causas-error de 'empty_line_after_outer_attr'](https://github.com/rust-lang/rust-clippy/pull/15078)
* [nueva pelusa: 'manual_is_multiple_of'](https://github.com/rust-lang/rust-clippy/pull/14292)

#### Analizador de Rust
* [rust-analyzer: add 'fn parent(self, db) → GenericDef' a 'hir::TypeParam'](https://github.com/rust-lang/rust-analyzer/pull/20046)
* [Rust-analyzer: limpie 'folding_ranges' y soporte más cosas](https://github.com/rust-lang/rust-analyzer/pull/20080)
* [rust-analyzer: no se establece por defecto en 'static para la vida útil de los objetos rasgo](https://github.com/rust-lang/rust-analyzer/pull/20036)
* [Rust-analyzer: Captura de cierre para LET exprs](https://github.com/rust-lang/rust-analyzer/pull/20039)
* [rust-analyzer: arreglar el manifiesto del proyecto cargo que no apunta a la raíz del espacio de trabajo](https://github.com/rust-lang/rust-analyzer/pull/20069)
* [rust-analyzer: en la asistencia "Tipo de retorno de envoltura", no envuelva los puntos de salida si ya tienen el tipo correcto](https://github.com/rust-lang/rust-analyzer/pull/20061)
* [rust-analyzer: respect '.cargo/config.toml build.target-dir'](https://github.com/rust-lang/rust-analyzer/pull/20072)
* [rust-analyzer: deshabilita temporalmente el controlador de escritura '+' ya que mueve la posición del cursor](https://github.com/rust-lang/rust-analyzer/pull/20042)
* [rust-analyzer: use la higiene 'ROOT' para 'args' dentro de la nueva expansión 'format_args!](https://github.com/rust-lang/rust-analyzer/pull/20073)
* [Rust-analyzer: Ocultar los privados importados si el privado editable está deshabilitado](https://github.com/rust-lang/rust-analyzer/pull/20025)
* [Rust-analyzer: Imita la nueva expansión 'format_args!' de Rustc](https://github.com/rust-lang/rust-analyzer/pull/20056)

### Clasificación del rendimiento del compilador de Rust

Una semana dominada por el aterrizaje de un gran parche que implementó [RFC # 3729](https://github.com/rust-lang/rfcs/pull/3729) que desafortunadamente introdujo regresiones de rendimiento bastante considerables (promedio de ~ 1% de recuento de instrucciones en 111 puntos de referencia primarios). Se consideró que valía la pena para que el parche pudiera aterrizar y el rendimiento pudiera recuperarse en las PR de seguimiento.

Triaje realizado por **@rylev**.
Rango de revisión: [45acf54e.. 42245d34](https://perf.rust-lang.org/?start=45acf54eea118ed27927282b5e0bfdcd80b7987c&end=42245d34d22ade32b3f276dcf74deb826841594c&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 1.1% | [0.2%, 9.1%] | 123 |
| Regresiones ❌ <br /> (secundaria) | 1.0% | [0.1%, 4.6%] | 86 |
| Mejoras ✅ <br /> (primario) | -3,8% | [-7.3%, -0.3%] | 2 |
| Mejoras ✅ <br /> (secundaria) | -2,3% | [-18.5%, -0.2%] | 44 |
| Todos ❌✅ (primarios) | 1.0% | [-7,3%, 9,1%] | 125 |

2 regresiones, 4 mejoras, 10 mixtas; 7 de ellos en rollups
40 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/a63db4d1799853b334e4106d914fba24e49c8782/triage/2025/2025-06-24.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Usar lld por defecto en x86_64-unknown-linux-gnu estable](https://github.com/rust-lang/rust/pull/140525)
* [Permitir #[must_use] en tipos asociados para advertir sobre valores no utilizados en contextos genéricos](https://github.com/rust-lang/rust/pull/142590)
* [Arreglar el manejo de proc_macro:Ident de $crate](https://github.com/rust-lang/rust/pull/141996)
* [Asegurar búferes no vacíos para E/S vectoriales grandes](https://github.com/rust-lang/rust/pull/138879)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: --crate-attr](https://github.com/rust-lang/rfcs/pull/3791)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-06-25 - 2025-07-23 🦀

### Virtual
* 25/06/2025 | Virtual (Lima, PE)| [Grupo de usuarios de Rust Perú](https://www.meetup.com/peru-rust-user-group/)
    * [**Interfaces y Costos en la nube con Rust**](https://www.meetup.com/peru-rust-user-group/events/308543965/)
* 2025-06-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/cgamfls6)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 29/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
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
* 2025-07-22 | Virtual (Londres, Gran Bretaña) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Cajas, Consejos y Trucos Charlas Relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/307560304)

### Asia
* 28/06/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de junio de 2025**](https://hasgeek.com/rustbangalore/june-2025-rustacean-meetup/)
* 02/07/2025 | Seúl, KR | [Encuentro de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Rust en Seúl**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europa
* 25/06/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Lecciones aprendidas de hacer un pequeño juego en nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)
* 25/06/2025 | París, FR | [Región de París sistemática](https://systematic-paris-region.org/)
    * [**Conferencia Rust Paris 2025**](https://my.weezevent.com/rust-paris-2025)
* 2025-06-26 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**18º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/308399403)
* 2025-06-26 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Encuentro de Rust #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 2025-06-26 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentro de Rust #77**](https://www.meetup.com/rust-paris/events/308416060)
* 30/06/2025 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Meetup 2025/06: Drink-up zatvaranje sezone**](https://www.meetup.com/zagreb-rust-meetup/events/308477879)
* 01/07/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #9**](https://www.meetup.com/rust-gdansk/events/308349712)
* 01/07/2025 | París, FR | [Fornido](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup en París - organizado por Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-1407389873999)
* 02/07/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
* 02/07/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**La (in)santa Trinidad de Flutter, Rust y C**](https://www.meetup.com/rust-rhein-main/events/308609465)
* 02/07/2025 | Londres, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y ACCU especial - Taller de codificación Vibe**](https://www.meetup.com/oxford-rust-meetup-group/events/308435063/)
* 02/07/2025 | Posnan, PL | [Rust Polonia](https://www.meetup.com/rust-poland-meetup/)
    * [**Rust Poland Meetup x Poznan**](https://www.meetup.com/rust-poland-meetup/events/308480357)
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

### América del Norte
* 25/06/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/308562608)
* 2025-06-26 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust en el grupo de desarrolladores de Web3**](https://www.meetup.com/rust-los-angeles/events/308401269)
* 2025-06-26 | Los Ángeles (Chino Hills), CA, ESTADOS UNIDOS | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Rust en Web3**](https://lu.ma/ek8jx2r3)
* 2025-06-26 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Elixir y Rust**](https://www.meetup.com/rust-mx/events/308579237)
* 2025-06-26 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Meetup mensual: ¡Haciendo una API CRUD con Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 28/06/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Back Bay Rust, 28 de junio**](https://www.meetup.com/bostonrust/events/307936269)
* 03/07/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Julio Social Mensual**](https://www.meetup.com/rust-montreal/events/308532058)
* 03/07/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Construcción de Servicios de Rust Resilientes y Observables con steady_state**](https://www.meetup.com/stl-rust/events/306345853)
* 06/07/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Alewife Rust, 6 de julio**](https://www.meetup.com/bostonrust/events/307936287)
* 09/07/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> AI**](https://www.meetup.com/desert-rustaceans/events/308507249/)
* 15/07/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307931266)
* 17/07/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Series 1 : Introducción al Desarrollo Embebido**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 17/07/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 23/07/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhckbfc)

### Oceanía
* 30/06/2025 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Reunión de junio de 2025 Mini Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/308546374)
* 01/07/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Rust en Christchurch en julio de 2025*](https://www.meetup.com/christchurch-rustlang-meetup-group/events/308605782)

### América del Sur
* 2025-07-12 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1knkfb6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Nuestra experiencia es que, independientemente de la cantidad de medidas de seguridad que se le pongan al código, no hay una panacea que impida una mala programación. Por supuesto, para tomar el argumento contrario, los cinturones de seguridad no detienen todas las muertes por accidentes de tráfico, pero simplemente podría optar por no tener accidentes. Así que tenemos cinturones de seguridad. Si Rust puede prevenir algunos errores o intenciones maliciosas, tal vez valga la pena incluso si no es perfecto.

– [Al Williams en hackaday](https://hackaday.com/2025/06/21/if-your-kernel-development-is-a-little-rusty/)

¡Gracias a [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1700) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1lknjc1/this_week_in_rust_605/)</small>
