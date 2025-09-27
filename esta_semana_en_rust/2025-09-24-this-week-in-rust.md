---
title: "Esta semana en Rust #77"
number_of_week: 77
description: El crate de esta semana es faer, una biblioteca de álgebra lineal de propósito general para Rust, con un enfoque en el alto rendimiento para operaciones algebraicas en matrices medianas/grandes, así como descomposiciones de matrices.
date: 2025-09-24
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
* [Anuncio de Rust 1.90.0 | Blog de Rust](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/)
* [Microencuesta de genéricos variádicos](https://blog.rust-lang.org/inside-rust/2025/09/22/variadic-generics-micro-survey/)
* [Selecciones de representantes del Consejo de Liderazgo de septiembre de 2025](https://blog.rust-lang.org/inside-rust/2025/09/23/leadership-council-repr-selection/)
* [crates.io: Cajas maliciosas faster_log y async_println](https://blog.rust-lang.org/2025/09/24/crates.io-malicious-crates-fasterlog-and-asyncprintln/)

### Actualizaciones de proyectos/herramientas
* [¡Temporal_rs está aquí! La biblioteca de fecha y hora que alimenta Temporal en Boa y V8 ](https://boajs.dev/blog/2025/09/24/temporal-release)
* [Actualización de Wild Linker - 0.6.0](https://davidlattimore.github.io/posts/2025/09/23/wild-update-0.6.0.html)
* [Lucha contra la trata de personas con aplicaciones autónomas](https://lwn.net/SubscriberLink/1036916/8fa1fd58807543b6/)
* [CHERI con un Linux en la parte superior](https://lwn.net/SubscriberLink/1037974/7860e9a3612d70fb/)
* [SeaORM 2.0: Una mirada más cercana](https://www.sea-ql.org/blog/2025-09-24-sea-orm-2.0/)
* [GuardianDB: La implementación de OrbitDB en Rust. Una base de datos peer-to-peer para la Web Descentralizada.](https://www.willsearch.com.br/)
* [Styx Emulator: Un nuevo marco de emulación para DSP, SoC extraños y sistemas integrados](https://stumbl.ing/posts/styx-emulator-release/)
* [GlueSQL v0.18.0 agrega soporte para Send/Sync y una nueva macro de derivación para el mapeo de filas con tipo](https://github.com/gluesql/gluesql/releases/tag/v0.18.0)
* [Implementación automática de TLS para Pingoo: esto es lo que se necesita para construir una Internet segura](https://kerkour.com/pingoo-automatic-tls)

### Observaciones/Pensamientos
* [Un ecosistema de Rust más estable](https://ranger-ross.github.io/blog/more-stable-ecosystem/)
* [Comparando el Rust con el carbono](https://lwn.net/SubscriberLink/1036912/ecf2235a9ef774d9/)
* [Cancelación de Rust asíncrono](https://lwn.net/SubscriberLink/1036924/83af62ecb5f74c06/)
* [Experiencia de contribución de Rust: de un extraño curioso a un defensor de GreptimeDB: mi viaje hacia la contribución de código abierto](https://greptime.com/blogs/2025-09-23-greptimedb-submission-rust-contribute-guide)
* [¿Por qué Rust?](https://roland.fly.dev/posts/why-rust/)

### Tutoriales de Rust
* [Pruebas unitarias de Rust: herramientas integradas](https://jorgeortiz.dev/posts/rust_unit_testing_tools_builtin/)
* [Del Rust a la realidad: el viaje oculto de fetch_max](https://questdb.com/blog/rust-fetch-max-compiler-journey/)
* [Reducción del tamaño binario de los programas (Rust) con debuginfo](https://kobzol.github.io/rust/2025/09/22/reducing-binary-size-of-rust-programs-with-debuginfo.html)
* [Serie de backend de Axum: modelos, migración, DTO y patrón de repositorio](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-model-setup/)
* [video] [charlando sobre Rust / concurrencia](https://www.youtube.com/live/kJnrbbtYfhI?si=9T62OcP3DbpaR4B6)
* [video] [Introducción a micro:bit y Embedded Rust](https://www.youtube.com/watch?v=IjlbuPZTluU)
* [video] [(Kernel) Cambio de tareas en Rust](https://www.youtube.com/watch?v=JP4-JJefY_A)

### Miscelánea
* [Liderando el camino para la seguridad certificada por Rust: una conversación con Espen Albrektsen de Sonair](https://filtra.io/rust/interviews/sonair-sep-25)

## Crate de la semana

El crate de esta semana es [faer](https://docs.rs/faer), una biblioteca de álgebra lineal de propósito general para Rust, con un enfoque en el alto rendimiento para operaciones algebraicas en matrices medianas/grandes, así como descomposiciones de matrices.

A pesar de que pasó otra semana sin una caja semanal sugerida, llogiq está satisfecho con su elección.

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

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

* [this-week-in-rust - Faltan atributos lang HTML para sitios web](https://github.com/rust-lang/this-week-in-rust/issues/6879)
* [GreptimeDb - Admite señal de perfil para OLTP](https://github.com/GreptimeTeam/greptimedb/issues/6760)
* [GreptimeDb - Actualmente KILL no puede terminar consultas como INSERT INTO SELECT](https://github.com/GreptimeTeam/greptimedb/issues/6334)
* [GreptimeDb - Admite la exportación de archivos CSV o JSON comprimidos](https://github.com/GreptimeTeam/greptimedb/issues/6286)

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

430 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-16..2025-09-23

#### Compilador
* ['-Znext-solver' allow 'ExprKind::Call' para opacos aún no definidos](https://github.com/rust-lang/rust/pull/145993)
* [destinoPropagación: evitar la creación de asignaciones superpuestas](https://github.com/rust-lang/rust/pull/146516)
* [detectar intento de usar var-args en el cierre](https://github.com/rust-lang/rust/pull/146581)
* [no aplique reglas de extensión temporal de por vida a 'super let' no extendido](https://github.com/rust-lang/rust/pull/145838)
* [habilitar DestinationPropagation de forma predeterminada](https://github.com/rust-lang/rust/pull/142915)
* [pelusa más asignaciones superpuestas en MIR](https://github.com/rust-lang/rust/pull/146566)
* [eliminar 'Rvalue::Len' de nuevo](https://github.com/rust-lang/rust/pull/146564)
* [sugerir eliminar 'Box::new' en lugar de desempaquetarlo](https://github.com/rust-lang/rust/pull/146259)

#### Biblioteca
* [agregue '[const] PartialEq' vinculado a 'PartialOrd'](https://github.com/rust-lang/rust/pull/146690)
* [repetición del iterador: no hay bucle infinito para 'último' y 'recuento'](https://github.com/rust-lang/rust/pull/146410)
* [hacer que 'PeekMut' sea genérico sobre el asignador](https://github.com/rust-lang/rust/pull/146621)
* [especialízate en 'Iterator::eq{_by}' para iteradores 'TrustedLen'](https://github.com/rust-lang/rust/pull/137122)
* [función de estabilización 'btree_entry_insert'](https://github.com/rust-lang/rust/pull/144871)
* [estabilizar 'new_zeroed_alloc'](https://github.com/rust-lang/rust/pull/144091)
* [estabilizar 'std::p anic::Ubicación::file_as_c_str'](https://github.com/rust-lang/rust/pull/145664)
* [corregir la implementación WASI de 'remove_dir_all'](https://github.com/rust-lang/rust/pull/146691)
* [fusionar definiciones de 'StdioPipes'](https://github.com/rust-lang/rust/pull/146639)
* [simplificar la búsqueda de host](https://github.com/rust-lang/rust/pull/146541)

#### Carga
* ['fix(frontmatter)': Mejorar la calidad de los errores](https://github.com/rust-lang/cargo/pull/15972)
* [feat: agregar lint para el uso global de 'hint-mostly-unused'](https://github.com/rust-lang/cargo/pull/15995)

#### Rustdoc
* [rustdoc-search: optimización de javaScript basada en la salida de Firefox Profiler](https://github.com/rust-lang/rust/pull/146484)

#### Clippy
* ['match_as_ref': no pelusa si el otro brazo no es 'Ninguno => Ninguno'](https://github.com/rust-lang/rust-clippy/pull/15693)
* ['redundant_clone': el iterador dividido se verifica en 'redundant_iter_cloned'](https://github.com/rust-lang/rust-clippy/pull/15277)
* ['transmute_ptr_to_ref': no sugiera '.cast' cuando to-type es DST](https://github.com/rust-lang/rust-clippy/pull/15621)
* [agregue 'clippy::self_only_used_in_recursion' lint](https://github.com/rust-lang/rust-clippy/pull/14787)
* [no reemplace '.unwrap_or(vec![])' por '.unwrap_or_default()'](https://github.com/rust-lang/rust-clippy/pull/15699)
* ['nonstandard_macro_braces': sugerir punto y coma final cuando sea necesario](https://github.com/rust-lang/rust-clippy/pull/15593)
* [corregir 'option_if_let_else' cuando se ignora la variante 'Err'](https://github.com/rust-lang/rust-clippy/pull/14429)
* [corregir 'question_mark' falso positivo en las variables utilizadas después](https://github.com/rust-lang/rust-clippy/pull/15644)
* [corregir 'unnecessary_semicolon' falso negativo en '#[feature(stmt_expr_attributes)]'](https://github.com/rust-lang/rust-clippy/pull/15481)
* [arreglar 'unnecessary_unwrap' falso negativo](https://github.com/rust-lang/rust-clippy/pull/15689)
* [tenga en cuenta que el uso de 'enumerate()' intercambiará los argumentos](https://github.com/rust-lang/rust-clippy/pull/14969)
* [reelaboración 'module_inception'](https://github.com/rust-lang/rust-clippy/pull/14753)
* [sugerencia para 'rest_pat_in_fully_bound_structs'](https://github.com/rust-lang/rust-clippy/pull/15648)

### Analizador de Rust
* ['hover': unificar el formato de la regla horizontal a '---'](https://github.com/rust-lang/rust-analyzer/pull/20379)
* [agregue 'rust-analyzer.semanticHighlighting.comments.enable'](https://github.com/rust-lang/rust-analyzer/pull/20583)
* [corregir las ramas 'IfExpr' sugiere](https://github.com/rust-lang/rust-analyzer/pull/20661)
* [corregir la finalización de 'else' antes de la palabra clave 'else'](https://github.com/rust-lang/rust-analyzer/pull/20702)
* [arreglar 'extract_variable' en 'LetExpr'](https://github.com/rust-lang/rust-analyzer/pull/20700)
* [corregir el campo de registro abreviado 'unused_variables'](https://github.com/rust-lang/rust-analyzer/pull/20710)
* [arreglar aplicar en el if interno para 'pull_assignment_up'](https://github.com/rust-lang/rust-analyzer/pull/20722)
* [corregir literales enteros genéricos const negativos](https://github.com/rust-lang/rust-analyzer/pull/20697)
* [corrección no aplicable a la coma final para 'remove_dbg'](https://github.com/rust-lang/rust-analyzer/pull/20714)
* [corregir pánicos en 'Foo{mut x}' para 'destructure_struct_binding'](https://github.com/rust-lang/rust-analyzer/pull/20708)
* [corrección para implementar 'stdx::replace'](https://github.com/rust-lang/rust-analyzer/pull/20706)
* [corregir el manejo de elisión de por vida para los límites de rasgos de estilo 'Fn'](https://github.com/rust-lang/rust-analyzer/pull/20725)
* [hacer que el borrado de cheques tenga en cuenta la dependencia](https://github.com/rust-lang/rust-analyzer/pull/20689)
* [portar un montón de cosas de rustc y corregir un montón de discrepancias de tipos/diagnósticos](https://github.com/rust-lang/rust-analyzer/pull/20664)

### Triaje de rendimiento del compilador de Rust

Mover el argumento de la línea de comandos que cita de C++ a Rust ([#146700](https://github.com/rust-lang/rust/pull/146700)) dio como resultado un buen rendimiento
Gane cuando se trata de muchas dependencias y grandes espacios de trabajo. Una propagación de destino algo costosa
el paso del compilador estaba habilitado de forma predeterminada ([#142915](https://github.com/rust-lang/rust/pull/142915)), lo que resultó en algunas regresiones de tiempo de compilación,
pero debería dar lugar a un mejor rendimiento en tiempo de ejecución. El resto de cambios fueron pequeños.

Triaje realizado por **@kobzol**.
Rango de revisión: [52618eb3.. ce4beebe](https://perf.rust-lang.org/?start=52618eb338609df44978b0ca4451ab7941fd1c7a&end=ce4beebecb77821734079cff47d8af08f9f27f11&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,1%, 1,9%] | 61 |
| Regresiones ❌ <br /> (secundaria) | 0,6% | [0,1%, 3,4%] | 90 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-1,9%, -0,2%] | 29 |
| Mejoras ✅ <br /> (secundario) | -1,3% | [-22,8%, -0,1%] | 71 |
| Todos ❌✅ (primarios) | 0,0% | [-1,9%, 1,9%] | 90 |

1 Regresión, 4 Mejoras, 4 Mixto; 4 de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/8e7c75c12a21eb9c8c86cbfc75eff144a017f6b2/triage/2025/2025-09-23.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [la corrección de compatibilidad '#[track_caller]' no debe heredar '#[no_mangle]'](https://github.com/rust-lang/rust/pull/145724)
* [Permitir el préstamo de elementos de matriz de estructuras empaquetadas con alineación ABI <= alineación empaquetada](https://github.com/rust-lang/rust/pull/145419)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: ¡Agregar iter! macro](https://github.com/rust-lang/rfcs/pull/3861)

## Próximos eventos

Rusty Eventos entre 2025-09-24 - 2025-10-22 🦀

### Virtual
* 2025-09-25 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-09-28 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311046301/)
* 2025-10-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)
* 2025-10-02 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/ekgdex6j)
* 2025-10-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763858627)
* 2025-10-05 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311062530/)
* 2025-10-07 | Virtual (Beijing, CN) | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Reunión mensual de la comunidad de WasmEdge, el tiempo de ejecución de LLM / AGI **](https://www.meetup.com/wasm-rust-meetup/events/310831771/)
* 2025-10-09 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/tsjcttyhcnbqb/)
* 2025-10-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-10-19 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109167)
* 2025-10-21 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068625)
* 2025-10-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/310002307)

### Asia
* 2025-10-02 | Seúl, KR | [Reunión de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310824483)
* 2025-10-04 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de octubre de 2025**](https://hasgeek.com/rustbangalore/october-2025-rustacean-meetup/)
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malasia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokio, JP | [Encuentro de Tokyo Rust](https://www.meetup.com/tokyo-rust-meetup/events/)
    * [**Creación de interfaces de usuario de terminal de bolsillo con Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust octubre de 2025 en AWS en Tel Aviv **](https://www.meetup.com/rust-tlv/events/310628902)

### Europa
* 2025-09-24 | Gotemburgo, SE | [Rust, Göteborg](https://www.meetup.com/rustgbg/events/)
    * [**Rust Gbg — septiembre de 2025**](https://www.meetup.com/rustgbg/events/310866773)
* 2025-09-24 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Múnich 2025 / 3**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-09-25 | Augsburgo, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Reunión de Augsburg Rust #15**](https://rust-augsburg.github.io/meetup/Meetup_15.html)
* 2025-09-25 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Reunión de Rust #61**](https://www.meetup.com/copenhagen-rust-community/events/311100221)
* 2025-09-25 | Londres, Reino Unido | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Mujeres en Rust x Scala: Programación funcional en Rust & Streams con Aquascape**](https://www.meetup.com/women-in-rust/events/311056499/)
* 2025-09-27 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #18**](https://www.meetup.com/stockholm-rust/events/311027118/)
* 2025-09-30 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Septiembre 2025 Community Showcase**](https://www.meetup.com/rust-london-user-group/events/311070068/)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Encuentro de Rust Moravia (¡En la capital!)**](https://www.meetup.com/rust-moravia/events/310743282)
* 2025-10-01 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Construyendo chatbots de IA con Webassembly, Rust y Leptos**](https://www.meetup.com/oxford-rust-meetup-group/events/311170808)
* 2025-10-01 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134)
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #79**](https://www.meetup.com/rust-paris/events/310424476)
* 2025-10-08 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944041)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141)
* 2025-10-21 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252)
* 2025-10-21 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952)

### América del Norte
* 2025-09-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-24 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)
* 2025-09-24 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Resumen de RustConf - ¡La primicia!**](https://www.meetup.com/chicago-rust-meetup/events/311006846)
* 2025-09-24 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Rust/Python Interop & DB Design**](https://www.meetup.com/rust-nyc/events/311006867/)
* 2025-09-25 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl en la taberna Manuels**](https://www.meetup.com/rust-atl/events/308675983)
* 2025-09-25 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Serie 3 : Final de la serie**](https://www.meetup.com/music-city-rust-developers/events/304333261/)
* 2025-09-27 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, 27 de septiembre **](https://www.meetup.com/bostonrust/events/311038485/)
* 2025-09-30 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Reunión nocturna de Boston Rust con Bevy e Isograph, 30 de septiembre **](https://www.meetup.com/bostonrust/events/310907806/)
* 2025-10-02 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311004898)
* 2025-10-02 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [** 🚁 Rust en vuelo: lecciones del diseño de un cuadricóptero impreso en 3D con incrustación**](https://www.meetup.com/stl-rust/events/310279407)
* 2025-10-04 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**North End Rust Lunch, 4 de octubre **](https://www.meetup.com/bostonrust/events/310983705/)
* 2025-10-09 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Aya the Beholder: Escribir un cortafuegos eBPF con la caja de Aya**](https://www.meetup.com/utah-rust/events/311145663)
* 2025-10-16 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Resumen del año**](https://www.meetup.com/music-city-rust-developers/events/304333267)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Vara & Equipo](https://luma.com/events-by-vara-gear)
    * [**Taller de Rust de Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307)

### Oceanía:
* 2025-10-01 | Perth, WA, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Estamos aquí para aprender. Lo haremos sin descanso.

– [Jon Gjengset en YouTube](https://youtu.be/Wnb_n5YktO8?feature=shared&t=5645)

¡Gracias a [John Arundel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1718) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1npwe4i/this_week_in_rust_618/)</small>
