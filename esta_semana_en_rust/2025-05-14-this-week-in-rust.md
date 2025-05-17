---
title: "Esta semana en Rust #60"
number_of_week: 60
description: El crate de esta semana es brush, un shell compatible con bash implementado completamente en Rust.
date: 2025-05-14
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
* [Anuncio de los proyectos seleccionados de Google Summer of Code 2025](https://blog.rust-lang.org/2025/05/08/gsoc-2025-selected-projects/)

### Fundación
* [10 años de Rust estable: una historia de infraestructura](https://rustfoundation.org/media/10-years-of-stable-rust-an-infrastructure-story/)

### Boletines
* [Este mes en Rust OSDev: abril de 2025 | Rust OSDev](https://rust-osdev.com/this-month/2025-04/)
* [El Rustacean Incrustado Edición #45](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-45)

### Actualizaciones de proyectos/herramientas
* [Física Aviar 0.3](https://joonaa.dev/blog/08/avian-0-3)
* [Dos meses en Servo: anidamiento de CSS, Shadow DOM, API de portapapeles y más](https://servo.org/blog/2025/05/09/this-month-in-servo/)
* [Cuna v0.3: Aún más Lazier](https://mackow.ski/blog/cot-v03-even-lazier/)
* [Análisis de datos de streaming, versión 0.17.3 de Fluvio](https://www.fluvio.io/news/this-week-in-fluvio-0075)
* [CGP v0.4 está aquí: desbloqueando una depuración más fácil, ajustes preestablecidos extensibles y más](https://contextgeneric.dev/blog/v0-4-0-release/)
* [Rama v0.2](https://github.com/plabayo/rama/discussions/544)

### Observaciones/Pensamientos
* [Patrones de Tipos Malos - El Pato Duplicado](https://www.schneems.com/2025/05/07/bad-type-patterns-the-duplicate-duck/)
* [Características nocturnas de Rust a las que debes estar atento](https://www.wakunguma.com/blog/interesting-rust-nightly-features)
* [Rust sin bloqueos: cómo construir una montaña rusa mientras está en llamas](https://yeet.cx/blog/lock-free-rust/)
* [Localización simple y con seguridad de tipos en Rust](https://aarol.dev/posts/rust-simple-i18n/)
* [De Rust al ensamblaje AVR: Diseccionando un programa de parpadeo mínimo](https://n-eq.github.io/blog/2025/05/13/rust-avr-arduino-blink)
* [Semana de la Velocidad de las Lonas](https://xd009642.github.io/2025/05/08/Tarpaulins-Week-of-Speed.html)
* [Rendimiento del lado del servidor de Rustls](https://www.memorysafety.org/blog/rustls-server-perf/)
* [¿Es Rust el futuro de la programación?](https://blog.jetbrains.com/rust/2025/05/13/is-rust-the-future-of-programming/)

### Tutoriales de Rust
* [Rust asíncrono funcional](https://willemvanhulle.tech/blog/func-async/)
* [El poder de la arquitectura ECS en tiempo de compilación en Rust](https://minikin.me/blog/entity-component-systems-reimagined)
* [video] [Build with Naz : Animación Spinner, contención de bloqueos, manejo de Ctrl+C para TUI y CLI](https://www.youtube.com/watch?v=iIMYzczF11c)

### Miscelánea
* [Informe de empleos de Rust de abril de 2025](https://filtra.io/rust/jobs-report/apr-25)

## Crate de la semana

El crate de esta semana es [brush](https://github.com/reubeno/brush/), un shell compatible con bash implementado completamente en Rust.

¡Gracias a [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/1434) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
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

* [Rama - Añadir fFi/Rama-Rhai: Capacidad de soporte para usar servicios y capas escritas en Rhai](https://github.com/plabayo/rama/issues/533)
* [Rama - Admite la huella digital pasiva Akamai H2 y la exposición en los servicios Echo + FP](https://github.com/plabayo/rama/issues/517)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

* *No se presentaron convocatorias para trabajos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 397 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-06..2025-05-13

#### Compilador

* [corrección de caída asíncrona para <T>el diseño 'async_drop_in_place' para T no especificado](https://github.com/rust-lang/rust/pull/140902)
* [Mejor mensaje de error para la discordancia de parámetros de vida tardía/temprana](https://github.com/rust-lang/rust/pull/140523)
* [perf: hacer la aserción en 'Ident::new' debug-only](https://github.com/rust-lang/rust/pull/140880)
* [perf: fusionar el bucle typeck con el bucle de evaluación de elementos estáticos/const](https://github.com/rust-lang/rust/pull/140854)

#### Biblioteca

* [implementar (parte de) ACP 429: agregar 'DerefMut' a 'Lazy[Cell/Lock]'](https://github.com/rust-lang/rust/pull/129334)
* [implementar 'VecDeque::truncate_front()'](https://github.com/rust-lang/rust/pull/140668)

#### Carga

* [network: use el encabezado Retry-After para respuestas HTTP 429](https://github.com/rust-lang/cargo/pull/15463)
* [rustc: No entres en pánico en contenedores desconocidos](https://github.com/rust-lang/cargo/pull/15497)
* [agregar soporte de patrón de globo para 'known_hosts'](https://github.com/rust-lang/cargo/pull/15508)
* [añadir soporte para '-Zembed-metadata'](https://github.com/rust-lang/cargo/pull/15378)
* [Arreglar enlace de plantilla de problema de seguimiento](https://github.com/rust-lang/cargo/pull/15494)
* [hacer que el script de carga ignore los espacios de trabajo](https://github.com/rust-lang/cargo/pull/15496)

#### Rustdoc

* [rustdoc-json: eliminar nuevas líneas de los atributos](https://github.com/rust-lang/rust/pull/140762)
* [Asegúrese de que la carpeta temporal de doctest se elimine correctamente incluso si doctests falló](https://github.com/rust-lang/rust/pull/140706)

#### Clippy

* [clippy: 'item_name_repetitions': excluir variantes de 'enumeración' con componentes de ruta idénticos](https://github.com/rust-lang/rust-clippy/pull/14619)
* [clippy: 'return_and_then': solo lint devuelve expresiones](https://github.com/rust-lang/rust-clippy/pull/14783)
* [clippy: 'unwrap_used', 'expect_used': aceptar el resultado de la macro como receptor](https://github.com/rust-lang/rust-clippy/pull/14575)
* [clippy: añadir la configuración 'allow_unused' a 'missing_docs_in_private_items'](https://github.com/rust-lang/rust-clippy/pull/14453)
* [clippy: agregar nueva pelusa 'confusing_method_to_numeric_cast'](https://github.com/rust-lang/rust-clippy/pull/13979)
* [clippy: añadir nueva pelusa: 'cloned_ref_to_slice_refs'](https://github.com/rust-lang/rust-clippy/pull/14284)
* [clippy: arreglar ICE en 'missing_const_for_fn'](https://github.com/rust-lang/rust-clippy/pull/14776)
* [clippy: corrige el falso negativo 'integer_division' para denominadores distintos de cero](https://github.com/rust-lang/rust-clippy/pull/14664)
* [clippy: corrige el falso negativo 'manual_let_else' cuando diverge en la variante simple 'enum'](https://github.com/rust-lang/rust-clippy/pull/14732)
* [clippy: arreglar 'unnecessary_unwrap' emitido dos veces en el cierre](https://github.com/rust-lang/rust-clippy/pull/14770)
* [clippy: arreglar rutas de diagnóstico impresas por la prueba de comida para perros](https://github.com/rust-lang/rust-clippy/pull/14746)
* [clippy: arreglar falso negativo para 'unnecessary_unwrap'](https://github.com/rust-lang/rust-clippy/pull/14758)
* [clippy: convertir el mensaje de ayuda 'let_with_type_underscore' en una sugerencia](https://github.com/rust-lang/rust-clippy/pull/14749)
* [clippy: resolver a través de reexportaciones locales en 'lookup_path'](https://github.com/rust-lang/rust-clippy/pull/14772)

#### Analizador de Rust

* [arreglar fragmentos de postfijo duplicando derefs](https://github.com/rust-lang/rust-analyzer/pull/19764)
* [Resuelva la ruta del documento desde el módulo principal si existen comentarios externos en el módulo](https://github.com/rust-lang/rust-analyzer/pull/19507)
* [aún completando paréntesis y argumentos de llamada al método si hay paréntesis existentes, pero están después de una nueva línea](https://github.com/rust-lang/rust-analyzer/pull/19763)

### Clasificación del rendimiento del compilador de Rust

Muchos cambios esta semana. El resultado general es positivo, con una gran victoria en el chequeo de tipo.

Triaje realizado por **@panstromek**.
Rango de revisión: [62c5f58f.. 718ddf66](https://perf.rust-lang.org/?start=62c5f58f57670ce65e7fec34f8c4ba00c27da2d9&end=718ddf660e6a1802c39b4962cf7eaa4db57025ef&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.2%, 1.4%] | 113 |
| Regresiones ❌ <br /> (secundaria) | 0.5% | [0.1%, 1.5%] | 54 |
| Mejoras ✅ <br /> (primario) | -2,5% | [-22.5%, -0.3%] | 45 |
| Mejoras ✅ <br /> (secundaria) | -0.9% | [-2.3%, -0.2%] | 10 |
| Todos ❌✅ (primarios) | -0.3% | [-22.5%, 1.4%] | 158 |

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/521ad9b18768d7c9890dbc6e6685e38b8d4c0164/triage/2025-05-12.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para 'non_null_from_ref'](https://github.com/rust-lang/rust/issues/130823)
* [Añadir std::io::Seek instancia para 'std::io::Take'](https://github.com/rust-lang/rust/pull/138023)
* [aarch64-softfloat: prohibir la habilitación de la función de objetivo de neón](https://github.com/rust-lang/rust/pull/135160)
* [Estabilizar las características del objetivo avx512](https://github.com/rust-lang/rust/pull/138940)
* [hacer que las funciones std::intrínsecas sean realmente intrínsecas](https://github.com/rust-lang/rust/pull/139916)
* [Error en la opacidad recursiva en el tipo HIR](https://github.com/rust-lang/rust/pull/139419)
* [Eliminar 'i128' y 'u128' de 'improper_ctypes_definitions'](https://github.com/rust-lang/rust/pull/137306)
* [Comportamiento de garantía de transmutación de 'Opción:<T>:::Ninguna' sujeta a NPO](https://github.com/rust-lang/rust/pull/137323)
* [Extensión temporal de la vida útil a través de constructores de estructura de tupla y variantes de tupla](https://github.com/rust-lang/rust/pull/140593)
* [Estabilizar 'tcp_quickack'](https://github.com/rust-lang/rust/pull/129121)
* [Cambiar la eliminación de azúcar de 'assert!' para una mejor salida de error](https://github.com/rust-lang/rust/pull/122661)
* [Hacer que los predicados de buena formación ya no sean coductivos](https://github.com/rust-lang/rust/pull/140208)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Biblioteca Estándar Extendida (ESL)](https://github.com/rust-lang/rfcs/pull/3810)

## Próximos eventos

Eventos oxidados entre 2025-05-14 - 2025-06-11 🦀

### Virtual
* 15/05/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 15/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 15/05/2025 | Virtual (Encuentro Conjunto, Europa + Israel) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/) + [Rust Paris](https://www.meetup.com/de-DE/rust-paris/) + [London Rust Project Group](https://www.meetup.com/de-DE/london-rust-project-group/) + [Rust Zürisee](https://www.meetup.com/de-DE/rust-zurich/) + [Rust TLV](https://www.meetup.com/de-DE/rust-tlv/) + [Rust Nürnberg](https://www.meetup.com/de-DE/rust-noris/) + [Rust Munich](https://www.meetup.com/de-DE/rust-munich/) + [Rust Aarhus]( https://www.meetup.com/de-de/rust-aarhus/) + [lunch.rs](http://lunch.rs/)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 🦀 **](https://www.meetup.com/rust-berlin/events/307293317)
* 15/05/2025 | Virtual (Zürich, CH) | [Rust Zürisee](https://www.meetup.com/rust-zurich)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 (co-evento con berline.rs) 🦀 **](https://www.meetup.com/rust-zurich/events/307696718)
* 18/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307796049)
* 19/05/2025 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592)
* 2025-05-20 | Híbrido (UE/Reino Unido) | [Rust y C++ Dragons (antes Cardiff)](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Talk and Connect - Fullstack - con Goetz Markgraf y Ben Wishovich**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307836345)
* 2025-05-20 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Enhebrando a través de vidas de préstamos - a la manera de Rust**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    [**Rust at Work: una conversación con Ran Reichman, cofundador y CEO de Flarion**](https://www.meetup.com/code-mavens/events/307635734/)
* 2025-05-20 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170826)
* 21/05/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Enlace**](https://www.meetup.com/vancouver-rust/events/307184332)
* 22/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 22/05/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 25/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307668643)
* 27/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361435)
* 27/05/2025 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversación con Eli Shalom e Igal Tabachnik de Eureka Labs**](https://www.meetup.com/code-mavens/events/307673680/)
* 29/05/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820285)
* 29/05/2025 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307730629)
* 01/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307795210)
* 03/06/2025 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**¿Por qué Rust? למה ראסט? -**](https://www.meetup.com/rust-tlv/events/307801358)
* 04/06/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031665)
* 05/06/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 07/06/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 08/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjblb)
* 2025-06-10 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 La comunidad se pone al día**](https://www.meetup.com/women-in-rust/events/307560326)

### Asia
* 17/05/2025 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #10**](https://www.meetup.com/rustdelhi/events/307657584)
* 24/05/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de mayo de 2025**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)
* 08/06/2025 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Presencial Rust junio de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europa
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 14/05/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 15/05/2025 | Berlín, DE | [Rust Berlín](https://berline.rs/)
    * [**10 aniversario de Rust 1.0**](https://www.c-base.org/calendar/#view=month&date=2025-05-15&event=92df14e3-c21c-477a-a150-84be085ed411)
* 15/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust 10º aniversario @ Aparece**](https://www.meetup.com/rust-oslo/events/307427014)
* 16/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/307107584)
* 16/05/2025 | Utrecht, NL | [Grupo de Meetup de Rust NL](https://www.meetup.com/rust-nederland/)
    * [**Hackathon de RustWeek**](https://www.meetup.com/rust-nederland/events/307107584/)
* 17/05/2025 | Ámsterdam, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam)
    * [**Recorrido a pie por Utrecht - Sábado**](https://www.meetup.com/rust-nederland/events/307522004)
* 2025-05-20 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Talk and Connect - Fullstack - con Goetz Markgraf y Ben Wishovich**](https://www.meetup.com/rust-dortmund/events/307505881)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 22/05/2025 | Augsburgo, DE | [Rust Augsburgo](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Rust meetup #13: Una guía práctica para la telemetría en Rust**](https://rust-augsburg.github.io/meetup/Meetup_13.html)
* 22/05/2025 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 22/05/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 22/05/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/307653223)
* 27/05/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)
* 27/05/2025 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna)
    * [**Rust Viena - mayo en Bitcredit 🦀 **](https://www.meetup.com/rust-vienna/events/307825439)
* 29/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 31/05/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 04/06/2025 | Gante, BE | [Programación de Sistemas Gante](https://www.sysghent.be)
    * [**Vuélvete más inteligente con Rust incrustado**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 04/06/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105443)
* 04/06/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 05/06/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105443)
* 11/06/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045448)

### América del Norte
* 15/05/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 15/05/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307488039/)
* 15/05/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Usando Rust para Web Series 2 : Por qué tú, sí tú. ¡Debería usar Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 15/05/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 18/05/2025 | Albuquerque, Nuevo México, Estados Unidos | [**Ideas y café**](https://www.meetup.com/ideas-and-coffee/)
    * [**Reunión de Rust a nivel de introducción**](https://www.meetup.com/ideas-and-coffee/events/307645653/)
* 2025-05-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 21/05/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Enlace**](https://www.meetup.com/vancouver-rust/events/307184332)
* 28/05/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 29/05/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 05/06/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)

### América del Sur
* 28/05/2025 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 31/05/2025 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
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

> Si un "Pin" cae en una habitación, y nadie alrededor lo entiende, ¿hace un mal sonido? #rustlang

– [Josh Triplett en fedi](https://social.joshtriplett.org/notice/AtrAtfNxi0bcypcBDk)

¡Gracias a [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1684) por la autosugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ko2g0n/this_week_in_rust_599/)</small>
