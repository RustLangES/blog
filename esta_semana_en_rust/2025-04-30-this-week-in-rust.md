---
title: "Esta semana en Rust #58"
number_of_week: 58
description: El crate de esta semana es rust-sel4, una caja sin _std para vincular a las API del microkernel Se4L.
date: 2025-04-30
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

### Boletines
* [El Rustáceo Incrustado Edición #44](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-44)

### Actualizaciones de proyectos/herramientas
* [Bevy 0.16](https://bevyengine.org/news/bevy-0-16/)
* [Anuncio de lanzamiento de Yelken Alpha](https://bwqr.github.io/yelken-blog/first-announcement/)
* [BugStalker v0.3.0 - Depurador moderno para Linux x86-64. Escrito en Rust para programas de Rust.](https://www.reddit.com/r/rust/comments/1k6vni5/bugstalker_v030_released_async_debugging_new/)

### Observaciones/Pensamientos
* [Auditoría de la caja Rust p256](https://reports.zksecurity.xyz/reports/near-p256/)
* [¡Tenemos polimorfismo en casa 🦀!](https://medium.com/@alighahremani1377/we-have-polymorphism-at-home-d9f21f5565bf)
* [Migrando fuera de Rust](https://deadmoney.gg/news/articles/migrating-away-from-rust)
* [Reflexiones sintácticas sobre las expresiones de coincidencia](https://blog.yoshuawuyts.com/syntactic-musings-on-match-expressions/)

### Tutoriales de Rust
* [Liberando Gigabytes: Recuperando Espacio en Disco de Rust Cargo Builds](https://thisdavej.com/freeing-up-gigabytes-reclaiming-disk-space-from-rust-cargo-builds/)
* [Un viaje visual a través de Async Rust](https://github.com/alexpusch/rust-magic-patterns/blob/master/visual-journey-through-async-rust/Readme.md)
* [Video] [Envío de Rust a Python, Typescript y Ruby](https://www.youtube.com/watch?v=Zs6Uer3VAyQ)
* [Video] [De Rust a C y viceversa: una introducción a las "funciones extranjeras"](https://www.youtube.com/watch?v=B4yNqR0WgYQ) ([Versión larga](https://www.youtube.com/watch?v=LLAUzghhNHg))
* [Video] [Internos de SlateDB: Un almacén de clave-valor integrado construido sobre almacenamiento de objetos](https://www.youtube.com/watch?v=qqF_zFWqFYk)
* [video] [Generando 1 millón de PDFs en 10 minutos](https://www.ersteiger.com/posts/rendering-one-million-pdfs/)
* [Video] [RefinedRust - Verificación de alta seguridad de los programas de Rust](https://www.youtube.com/watch?v=XR8p9R1cPC4)

### Miscelánea
* [La empresa que apuesta por todo en Rust para la robótica](https://filtra.io/rust/interviews/matic-apr-25)
* [Cómo funciona el rolldown: carga de módulos, gráficos de dependencias y optimización explicados](https://www.atriiy.dev/blog/rolldown-module-loader-and-dependency-graph)
* [Video] [Por qué aprender Rust podría cambiar tu carrera](https://www.youtube.com/watch?v=_7va24sawyg)

## Crate de la semana

El crate de esta semana es [rust-sel4](https://github.com/seL4/rust-sel4/), una caja sin _std para vincular a las API del microkernel Se4L.

¡Gracias a [Robbie VanVossen](https://users.rust-lang.org/t/crate-of-the-week/2704/1432) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
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

* [rama - añadir un SubdominioTrieMatcher](https://github.com/plabayo/rama/issues/534)
* [Rama - Añadir fFi/Rama-Rhai: Capacidad de soporte para usar servicios y capas escritas en Rhai](https://github.com/plabayo/rama/issues/533)
* [rama - soporte (TLS) peetprint en huellas dactilares rama-net](https://github.com/plabayo/rama/issues/518)
* [Rama - Admite la huella digital pasiva Akamai H2 y la exposición en los servicios Echo + FP](https://github.com/plabayo/rama/issues/517)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->
*No se han presentado convocatorias ni presentaciones esta semana.* 

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 389 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-22..2025-04-29

#### Compilador

* ['rc""' mensaje de error más claro](https://github.com/rust-lang/rust/pull/140175)
* [permitir que los patrones de deref se muevan fuera de las cajas](https://github.com/rust-lang/rust/pull/140022)
* [código de caída asíncrona](https://github.com/rust-lang/rust/pull/123948)
* [evitar volver a hacer una pasantía en 'LateContext::get_def_path'](https://github.com/rust-lang/rust/pull/140345)
* [Implemente un lint para la referencia automática implícita de la desreferencia de puntero sin procesar - tome 2](https://github.com/rust-lang/rust/pull/123239)
* [Mejorar el mensaje de error para '||' (o) en cadenas de alquiler](https://github.com/rust-lang/rust/pull/140272)
* [Estabilizar las cadenas Let en la edición de 2024](https://github.com/rust-lang/rust/pull/132833)
* [denegar 'unsafe_op_in_unsafe_fn' por defecto](https://github.com/rust-lang/compiler-builtins/pull/801)

#### Biblioteca

* [añadir 'Arco::is_unique'](https://github.com/rust-lang/rust/pull/138939)
* [estabilizar 'std::ffi::c_str'](https://github.com/rust-lang/rust/pull/137439)
* [estabilizar 'proc_macro::Span::{inicio,fin,línea,columna}'](https://github.com/rust-lang/rust/pull/139865)
* [Estabilizar la función de biblioteca 'slice_as_chunks'](https://github.com/rust-lang/rust/pull/139656)
* [transmutabilidad: soporte char, NonZeroXxx](https://github.com/rust-lang/rust/pull/140215)

#### Carga

* [implementar RFC3695: Permitir literales booleanos como predicados cfg](https://github.com/rust-lang/cargo/pull/14649)
* [estabilizar la recolección automática de basura](https://github.com/rust-lang/cargo/pull/14287)
* ['feat(add/install)': comprueba si el argumento de caja dado sería válido con el símbolo @ insertado](https://github.com/rust-lang/cargo/pull/15441)

#### Rustdoc

* [mostrar correctamente stdout y stderr en caso de que un doctest esté fallando](https://github.com/rust-lang/rust/pull/140291)
* [Estabilizar banderas para la compilación cruzada de doctest](https://github.com/rust-lang/rust/pull/137096)

#### Clippy

* ['manual_div_ceil': corregir sugerencias cuando se trata de macros](https://github.com/rust-lang/rust-clippy/pull/14666)
* [Considere los efectos secundarios al reescribir los comportamientos de los iteradores](https://github.com/rust-lang/rust-clippy/pull/14490)
* [arreglar 'zombie_processes' falsos positivos dentro de los cierres](https://github.com/rust-lang/rust-clippy/pull/14696)
* [corrección: 'equatable_if_let' sugiere erróneamente cuando se trata de referencias](https://github.com/rust-lang/rust-clippy/pull/14504)
* [corrección: 'unnecessary_cast' sugiere corchetes adicionales cuando está en macro](https://github.com/rust-lang/rust-clippy/pull/14643)
* [corrección: 'unused_unit' sugiere erróneamente que la unidad nunca escribe fallback](https://github.com/rust-lang/rust-clippy/pull/14609)
* [restringir los casos en los que se activa 'ptr_eq'](https://github.com/rust-lang/rust-clippy/pull/14526)

#### Analizador de Rust

* [Agregar variante de modo de relleno de expresión para rellenar con expresiones de subrayado](https://github.com/rust-lang/rust-analyzer/pull/19704)
* [siempre error cuando no se puede analizar DiscoverProjectMessage](https://github.com/rust-lang/rust-analyzer/pull/19684)
* [arena asigna 'LifetimeRef's](https://github.com/rust-lang/rust-analyzer/pull/19678)
* [base-db: añadir más detalles a panic](https://github.com/rust-lang/rust-analyzer/pull/19710)
* [agregar dos nuevos diagnósticos: uno para la discordancia en el recuento de argumentos genéricos, y otro para la discordancia en su tipo](https://github.com/rust-lang/rust-analyzer/pull/19479)
* [añade una ayuda para eliminar los guiones bajos de las variables utilizadas](https://github.com/rust-lang/rust-analyzer/pull/19692)
* [¡Mejor soporte 'offset_of! ()»(https://github.com/rust-lang/rust-analyzer/pull/19657)
* [manejar correctamente las vidas al verificar los argumentos genéricos len](https://github.com/rust-lang/rust-analyzer/pull/19676)
* [error de corrección del sufijo 'raw_string' de las asistencias de IDE](https://github.com/rust-lang/rust-analyzer/pull/19622)
* [Escapar correctamente de los nombres sin procesar en las etiquetas](https://github.com/rust-lang/rust-analyzer/pull/19699)
* [Corregir diagnóstico incorrecto por discrepancia en el recuento de parámetros de por vida](https://github.com/rust-lang/rust-analyzer/pull/19672)
* [Se corrige la discordancia del argumento de tipo que se activa incorrectamente en los argumentos de rasgos inferidos](https://github.com/rust-lang/rust-analyzer/pull/19675)
* [pánico en las sugerencias de incrustación que producen ediciones de texto vacías para los tipos de retorno de cierre](https://github.com/rust-lang/rust-analyzer/pull/19647)

### Clasificación del rendimiento del compilador de Rust

Extraña semana con mucho ruido asomándose a través de las actuaciones. El único cambio realmente significativo fue una mejora en el rendimiento que proviene de permitir la codificación desordenada del gráfico dep.

Triaje realizado por **@rylev**.
Rango de revisión: [8f2819b0.. 25cdf1f6](https://perf.rust-lang.org/?start=8f2819b0e3428d0aee05fa60e91e0211c2aea053&end=25cdf1f67463c9365d8d83778c933ec7480e940b&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.1%, 3.0%] | 77 |
| Regresiones ❌ <br /> (secundaria) | 0.6% | [0.1%, 2.4%] | 77 |
| Mejoras ✅ <br /> (primario) | -0,7% | [-1.3%, -0.2%] | Artículo 106 |
| Mejoras ✅ <br /> (secundaria) | -0,7% | [-1.2%, -0.2%] | 29 |
| Todos ❌✅ (primarios) | -0,2% | [-1.3%, 3.0%] | Año 183 |

4 regresiones, 2 mejoras, 4 mixtas; 2 de ellos en rollups
38 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/748534344dceab1e8001a925cf84fa04a2c1c752/triage/2025-04-29.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Finalizar el comportamiento de inferencia de repetición expr con recuentos de repeticiones inferidos](https://github.com/rust-lang/rust/pull/139635)
* [Estabilizar parcialmente las características del objetivo LoongArch](https://github.com/rust-lang/rust/pull/135015)
* [Unificar los botones de la barra lateral para usar la misma imagen](https://github.com/rust-lang/rust/pull/140135)
* [Atributo de banco de desestabilización](https://github.com/rust-lang/rust/pull/134273)
* [Eliminar algunas implicaciones de tuplas sin tamaño ahora que ya no admitimos tuplas sin tamaño](https://github.com/rust-lang/rust/pull/138340)

#### Otras áreas

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Eliminar disculpas por la referencia](https://github.com/rust-lang/reference/pull/1792)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: nomenclatura de grupos de configuración con cfg_alias](https://github.com/rust-lang/rfcs/pull/3804)

## Próximos eventos

Eventos oxidados entre 2025-04-30 - 2025-05-28 🦀

### Virtual
* 01/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/clzsjmn0)
* 03/05/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 04/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307556965)
* 05/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592/)
* 07/05/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031663)
* 07/05/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #10**](https://www.meetup.com/bevy-game-development/events/307354690)
* 08/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/2lmcm9iq)
* 08/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 11/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbpb)
* 13/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020415)
* 15/05/2025 | Virtual (Encuentro Conjunto, Europa + Israel) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/), [Rust Paris](https://www.meetup.com/de-DE/rust-paris/), [London Rust Project Group](https://www.meetup.com/de-DE/london-rust-project-group/), [Rust Zürisee](https://www.meetup.com/de-DE/rust-zurich/), [Rust TLV](https://www.meetup.com/de-DE/rust-tlv/), [Rust Nürnberg](https://www.meetup.com/de-DE/rust-noris/), [Rust Munich](https://www.meetup.com/de-DE/rust-munich/), [Rust Aarhus]( https://www.meetup.com/de-de/rust-aarhus/), [lunch.rs](http://lunch.rs/)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 🦀 **](https://www.meetup.com/rust-berlin/events/307293317)
* 15/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 18/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbxb)
* 2025-05-20 | Virtual (Londres, Gran Bretaña) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Enhebrando a través de vidas de préstamos - a la manera de Rust**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170826)
* 21/05/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)
* 22/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 22/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 25/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbhc)
* 27/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361435)

### Asia
* 24/05/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de mayo de 2025**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)

### Europa
* 30/04/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Operador de Kubernetes en Rust**](https://www.meetup.com/rust-rhein-main/events/306772838)
* 01/05/2025 | Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Hackers Hike 0x0**](https://www.meetup.com/rust-noris/events/305522254)
* 04/05/2025 | Estambul, TR | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**Conexión de Rust**](https://kommunity.com/turkiyerust/events/rust-connect-91f7b3a6)
* 06/05/2025 | Cambridge, Gran Bretaña | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/307477191)
* 06/05/2025 - 07/05/2025 | París, FR | [WebAssembly y Rust Meetup](https://www.meetup.com/wasm-rust-meetup/)
    * [**GOSIM AI París 2025**](https://www.meetup.com/wasm-rust-meetup/events/306530699/)
* 06/05/2025 | París, FR | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Paris 2025 (Descuento disponible)**](https://www.meetup.com/wasm-rust-meetup/events/306530699)
* 07/05/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 05 2025**](https://lu.ma/k4nscy5q)
* 07/05/2025 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en mayo: FFI**](https://www.meetup.com/rustcologne/events/307548402)
* 07/05/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 07/05/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 08/05/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 08/05/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adoptando Rust (Alojado por Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 14/05/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 15/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust 10º aniversario @ Aparece**](https://www.meetup.com/rust-oslo/events/307427014)
* 16/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/307107584)
* 16/05/2025 | Utrecht, NL | [Grupo de Meetup de Rust NL](https://www.meetup.com/rust-nederland/)
    * [**Hackathon de RustWeek**](https://www.meetup.com/rust-nederland/events/307107584/)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 22/05/2025 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 22/05/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 27/05/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)

### América del Norte
* 01/05/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Reflexiones del Proyecto Capstone SIUE sobre el Rust**](https://www.meetup.com/stl-rust/events/304026152)
* 03/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Boston Common Rust, 3 de mayo**](https://www.meetup.com/bostonrust/events/306845368)
* 07/05/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/307557852)
* 08/05/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Tiempo de compilación vs Tiempo de ejecución. Introducción a uv**](https://www.meetup.com/rust-mx/events/307015601)
* 08/05/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: Un motor de consulta analítica rápido, extensible y modular en Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 11/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Porter Square, 11 de mayo**](https://www.meetup.com/bostonrust/events/306845728)
* 15/05/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 2 : Por qué tú, sí tú. ¡Debería usar Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 15/05/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 28/05/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhchblc)

### América del Sur
* 28/05/2025 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Primera meetup de Rust en 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Dado que Bevy es claramente un conjunto de pruebas extendido para el solucionador de rasgos de Rust, ¿cómo se te ocurrió la idea de convertirlo también en un motor de juego?

> Toda prueba suficientemente avanzada es indistinguible de un motor 🙂 de juego

– [/u/0x564A00 y /u/_cart en /r/rust](https://www.reddit.com/r/rust/comments/1k721w1/comment/moumd91)

¡Gracias a [Ludwig Stecher](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1681) y [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1682) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1kbx9de/this_week_in_rust_597/)</small>
