---
title: "Esta semana en Rust #79"
number_of_week: 79
description: El crate de esta semana es tokio-netem, una caja de herramientas de adaptadores Tokio AsyncRead / AsyncWrite para emular latencia, estrangulamiento, corte, terminación, apagado forzado, inyección de datos y corrupción de datos.
date: 2025-10-08
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

### Actualizaciones de proyectos/herramientas
* [Seaography 2.0: Un marco GraphQL potente y extensible](https://www.sea-ql.org/blog/2025-10-08-seaography/)
* [Anuncio de la versión candidata a redis-rs 1.0.0](https://github.com/redis-rs/redis-rs/blob/redis-1.0.0-rc.0/version1.md)
* [BlazeSym 0.2 Stable Release: Baterías incluidas simbolización de dirección](https://github.com/libbpf/blazesym/discussions/1318)
* [Hackers del kernel en Cauldron, edición 2025](https://lwn.net/SubscriberLink/1039784/d2548814efb78046/)
* [Progreso en la derrota del zapping de puntero de fin de vida](https://lwn.net/SubscriberLink/1038757/d613acbb48f20a20/)
* [Próximas características del lenguaje Rust para el desarrollo del kernel](https://lwn.net/SubscriberLink/1039073/abf96f38b178f988/)
* [utsuru: "Go Live" en Discord usando OBS, FFmpeg o cualquier cosa que admita WHIP.](https://github.com/VincentVerdynanta/utsuru/releases/tag/v0.2.1)
* [Aladin Lite - navega en TBs de datos astronómicos provenientes de varias misiones espaciales](https://aladin.cds.unistra.fr/AladinLite/doc/)

### Observaciones/Pensamientos
* [El rasgo Mango](https://smallcultfollowing.com/babysteps/blog/2025/10/07/the-handle-trait/)
* [Genéricos variádicos](https://www.wakunguma.com/blog/variadic-generics)
* [Pensamientos sobre la palabra especificación en Rust](https://tritium.legal/blog/word)
* [Por qué apostamos por Rust para potenciar la tienda de funciones en Agoda](https://medium.com/agoda-engineering/why-we-bet-on-rust-to-supercharge-feature-store-at-agoda-ed4a70d2efb7)
* [morsa: ingesta de datos a velocidades de memoria](https://nubskr.com/2025/10/06/walrus.html)
* [Primitivas de simultaneidad en tiempo real](https://blog.inkreas.ing/realtime-concurrency)
* [Por qué no reescribimos nuestro controlador de feeds en Rust](https://databento.com/blog/why-we-didn't-rewrite-our-feed-handler-in-rust)
* [audio] [Netstack.FM Episodio 7 – Susurros con Dirkjan Ochtman](https://netstack.fm/#episode-7)
* [audio] [Netstack.FM Episodio 8 – Fuchsia's Netstack3 con Bruno Dal Bo Silva](https://netstack.fm/#episode-8)

### Tutoriales de Rust
* [Serie de backend de Axum: Implementación del restablecimiento de contraseña](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-reset-password/)
* [Serie de backend de Axum: verificación de correo electrónico después del registro](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-email-verification/)
* [Diversión con HyperLogLog y SIMD](https://vaktibabat.github.io/posts/hyperloglog/)
* [Cómo perfilar automáticamente el rendimiento de las aplicaciones de Rust](https://pawelurbanek.com/rust-optimize-performance)
* [Rust asíncrono con flujos de E/S de Tokio: contrapresión, concurrencia y ergonomía](https://biriukov.dev/docs/async-rust-tokio-io/1-async-rust-with-tokio-io-streams-backpressure-concurrency-and-ergonomics/)
* [Prevención del acceso no válido a la base de datos en tiempo de compilación](https://www.svix.com/blog/preventing-db-misuse-at-compile-time/)
* [Pruebas unitarias de Rust: dobles y talones de prueba](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_stub/)
* [Cancelación de Rust asíncrono](https://sunshowers.io/posts/cancelling-async-rust/)
* [Detrás de escena de Pingoo: Reducción de asignaciones con mimalloc y heapless para construir el proxy inverso más rápido](https://kerkour.com/rust-pingoo-high-performance-allocations-mimalloc-heapless)
* [Cálculos numéricos genéricos en Rust](https://michaelmauderer.com/blog/generic-numeric-computations/)
* [Escribamos una macro en Rust - Parte 3](https://hackeryarn.com/post/rust-macros-3/)
* [Interoperabilidad de Rust/C++ Parte 5 - Interoperabilidad en 2025: Un ejemplo completo](https://tylerjw.dev/posts/20251003-rust-cpp-interop-2025-update/)

### Miscelánea
* [videos] [RustConf 2025](https://www.youtube.com/playlist?list=PL2b0df3jKKiRFEuVNk76ufXagOgEJ9sBZ)
* [audio] [Prime Video - Alexandru Ene, Ingeniero Principal](https://corrode.dev/podcast/s05e01-prime-video/)

## Crate de la semana

El crate de esta semana es [tokio-netem](https://crates.io/crates/tokio-netem), una caja de herramientas de adaptadores Tokio AsyncRead / AsyncWrite para emular latencia, estrangulamiento, corte, terminación, apagado forzado, inyección de datos y corrupción de datos.

¡Gracias a [Viacheslav Biriukov](https://users.rust-lang.org/t/crate-of-the-week/2704/1478) por la autosugestión!

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

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
*No se enviaron convocatorias de participación esta semana.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP cierra 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 398 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-30..2025-10-07

#### Compilador
* [agregar un backend de codegen ficticio](https://github.com/rust-lang/rust/pull/146596)
* [no normalice las suposiciones de mayor rango si no se usan](https://github.com/rust-lang/rust/pull/147299)
* [extender '#[rustc_force_inline]' para que sea aplicable a los métodos inherentes](https://github.com/rust-lang/rust/pull/147231)
* [arreglar el truco de límites implícitos de bevy para el próximo solucionador](https://github.com/rust-lang/rust/pull/147184)
* [Numeración global de valores: sindicatos de apoyo](https://github.com/rust-lang/rust/pull/146355)
* [Numeración de valores globales: use un VnIndex en la proyección de direcciones](https://github.com/rust-lang/rust/pull/144477)
* [miri: agregar soporte para la mezcla temporal de accesos atómicos y no atómicos en modo GenMC](https://github.com/rust-lang/miri/pull/4611)

#### Biblioteca
* [agregue 'CloneFromCell' y 'Cell::get_cloned'](https://github.com/rust-lang/rust/pull/145685)
* [agregue 'Ruta::has_trailing_sep' y métodos relacionados](https://github.com/rust-lang/rust/pull/142506)
* [añadir 'mem::conjure_zst'](https://github.com/rust-lang/rust/pull/146479)
* [agregar ruta rápida para acceder al ID de subproceso actual](https://github.com/rust-lang/rust/pull/143069)
* [implementar 'Box::take'](https://github.com/rust-lang/rust/pull/147227)
* [implementar 'Mutex::with_mut', 'RwLock::with' y 'RwLock::with_mut'](https://github.com/rust-lang/rust/pull/147328)
* [hashbrown: reconocer y usar asignaciones de gran tamaño](https://github.com/rust-lang/hashbrown/pull/523)

#### Carga
* ['fix(run)': Anular arg0 para scripts de carga](https://github.com/rust-lang/cargo/pull/16027)
* ['fix(timings)': calcule la hora de inicio de codegen para dibujar líneas de dep](https://github.com/rust-lang/cargo/pull/16055)
* ['fix(toml)': Evitar campos que no sean scripts en scripts de carga](https://github.com/rust-lang/cargo/pull/16026)
* [accediendo a la 'OUT_DIR' de cada script de compilación](https://github.com/rust-lang/cargo/pull/15891)
* [agregar pánico = soporte de aborto inmediato a Cargo](https://github.com/rust-lang/cargo/pull/16041)
* [considere las dependencias públicas al elegir una versión en cargo add (# 1 ...](https://github.com/rust-lang/cargo/pull/15966)
* [convertir un diagnóstico de varias partes en un informe](https://github.com/rust-lang/cargo/pull/16035)
* [feat (publish): obsoleta la opción '--token'](https://github.com/rust-lang/cargo/pull/16046)
* [corregir el seguimiento de la ruta de FileLock después de cambiar el nombre en la operación del paquete](https://github.com/rust-lang/cargo/pull/16036)
* [arreglar 'unsafe_op_in_unsafe_fn' para Windows](https://github.com/rust-lang/cargo/pull/16058)
* [corrección: eliminar el comentario FIXME que ya no es un problema](https://github.com/rust-lang/cargo/pull/16025)
* [limpieza de errores de esquemas de archivo de bloqueo](https://github.com/rust-lang/cargo/pull/16039)
* [errores de manifiesto público en privado](https://github.com/rust-lang/cargo/pull/16002)
* [recomendar 'package.rust-version' en la sección de la versión de Rust de 'reference/semver.md'](https://github.com/rust-lang/cargo/pull/15806)
* [prueba: ruta terminada en nulo para la detección de nombres reservados de Windows](https://github.com/rust-lang/cargo/pull/16052)

#### Rustdoc
* [reemplace 'rustc_span::Span' con una versión simplificada para el resaltador de librustdoc](https://github.com/rust-lang/rust/pull/147189)

#### Clippy
* ['assertions_on_constants': Sugerir el uso de un bloque const cuando se utiliza una constante con nombre](https://github.com/rust-lang/rust-clippy/pull/15774)
* ['zero_repeat_side_effects': identificar mejor los EXPR con efectos secundarios](https://github.com/rust-lang/rust-clippy/pull/15814)
* [cambios constantes](https://github.com/rust-lang/rust-clippy/pull/15773)
* [no sugiera usar una cadena 'if let' si no es compatible](https://github.com/rust-lang/rust-clippy/pull/15746)
* [no activar 'inefficient_to_string' después de Rust 1.82](https://github.com/rust-lang/rust-clippy/pull/15729)
* [extender 'while_let_loop' a 'loop { let else }'](https://github.com/rust-lang/rust-clippy/pull/15701)
* [corregir 'if_then_some_else_none' falso positivo cuando existe retorno en el bloque expr](https://github.com/rust-lang/rust-clippy/pull/15783)
* [corregir 'let_unit_value' sugiere erróneamente para la abreviatura de inicio de campo](https://github.com/rust-lang/rust-clippy/pull/15791)
* [corregir 'mem_replace_with_default' macros mal destruídas](https://github.com/rust-lang/rust-clippy/pull/15786)
* [implementar pelusa 'volatile_composites'](https://github.com/rust-lang/rust-clippy/pull/15686)

#### Analizador de Rust
* [hacer que rust-analyzer use un directorio de compilación dedicado](https://github.com/rust-lang/rust/pull/141839)
* [deduplicar llamadas de ordenación+desduplicación](https://github.com/rust-lang/rust-analyzer/pull/20794)
* [registro flycheck stdout y stderr en archivos](https://github.com/rust-lang/rust-analyzer/pull/20806)
* [corregir paréntesis faltantes para 'missing_unsafe'](https://github.com/rust-lang/rust-analyzer/pull/20793)
* [corregir el pánico al usar análisis-estadísticas](https://github.com/rust-lang/rust-analyzer/pull/20777)
* [corregir el diagnóstico erróneo 'incorrect_generics_len' cuando hay genéricos en la variante 'enumeración' utilizada a través del alias de tipo](https://github.com/rust-lang/rust-analyzer/pull/20787)
* [ignorar los errores de seguridad de la característica impl cuando la característica no está resuelta](https://github.com/rust-lang/rust-analyzer/pull/20770)

### Triaje de rendimiento del compilador de Rust

En gran medida una semana positiva. Gran victoria proveniente de evitar trabajo innecesario para el inicio de sesión de depuración en [#147293](https://github.com/rust-lang/rust/pull/147293), y otra para rustdoc de la representación de intervalo optimizada para el resaltador [#147189](https://github.com/rust-lang/rust/pull/147189). De lo contrario, hay muchos resultados ruidosos.

Triaje realizado por **@panstromek**.
Rango de revisión: [8d72d3e1.. 1a3cdd34](https://perf.rust-lang.org/?start=8d72d3e1e96f58ca10059a6bb6e8aecba4a0e9cd&end=1a3cdd34629306fa67624eaa60d73687e7fcf855&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,2%, 2,0%] | 10 |
| Regresiones ❌ <br /> (secundaria) | 0,4% | [0,0%, 0,8%] | 50 |
| Mejoras ✅ <br /> (primaria) | -1,3% | [-5,3%, -0,2%] | 147 |
| Mejoras ✅ <br /> (secundario) | -1,3% | [-12,7%, -0,1%] | 111 |
| Todos ❌✅ (primarios) | -1,2% | [-5,3%, 2,0%] | 157 |

6 regresiones, 3 mejoras, 6 mixtas; 8 de ellos en rollups
40 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/33d1d2f6103c22772c45562aa159d1e1257c228e/triage/2025/2025-10-06.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Eleva y extiende 'clippy::needless-maybe-sized' en rustc](https://github.com/rust-lang/rust/pull/145924)
* [preferir candidatos de alias para objetivos de tamaño + rasgo automático](https://github.com/rust-lang/rust/pull/144064)
* [implementar Extend<{Group, Literal, Punct, Ident}> para TokenStream](https://github.com/rust-lang/rust/pull/145722)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Agregue 'target = "host"' que significa el host actual](https://github.com/rust-lang/cargo/issues/13051)

*Ningún artículo entró en el período de comentarios finales esta semana para
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
  [Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Permitir '#[ignorar(Trait)]' en el campo para ignorarlo al derivar 'Trait'](https://github.com/rust-lang/rfcs/pull/3869)

## Próximos eventos

Rusty Eventos entre 2025-10-08 - 2025-11-05 🦀

### Virtual
* 2025-10-08 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-09 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-09 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust)
    * [**Reunión en línea de San Diego Rust de octubre de 2025**](https://www.meetup.com/san-diego-rust/events/311359525/)
* 2025-10-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109160/)
* 2025-10-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/)
* 2025-10-19 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-23 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2025 Reunión de SRUG (Seattle Rust User Group)**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361444/)
* 2025-10-30 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-05 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcpbhb/)

### Asia
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malasia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokio, JP | [Encuentro de Tokyo Rust](https://www.meetup.com/tokyo-rust-meetup)
    * [**Creación de interfaces de usuario de terminal de bolsillo con Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**En persona Rust octubre de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europa
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | París, FR | [Rust París](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #79**](https://www.meetup.com/rust-paris/events/310424476/)
* 2025-10-08 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944041/)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Bergen, No | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-30 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #62 patrocinada por Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga (octubre de 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-02 - 2025-11-04 | Florencia, IT | [Laboratorio de Rust 2025](https://rustlab.it/)
    * [**Rustlab 2025**](https://rustlab.it/)
* 2025-11-04 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Charla de noviembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión de Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)

### América del Norte
* 2025-10-08 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> C++**](https://www.meetup.com/desert-rustaceans/events/311365791/)
* 2025-10-09 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust)
    * [**Aya the Beholder: Escribiendo un cortafuegos eBPF con la caja de Aya**](https://www.meetup.com/utah-rust/events/311145663/)
* 2025-10-14 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Rust in Robotics & Isograph**](https://www.meetup.com/rust-nyc/events/311328171/)
* 2025-10-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311012947/)
* 2025-10-16 | San Francisco, CA, EE. UU. | [Svix](https://luma.com/calendar/cal-Cnmn4RR2n4fRUNZ)
    * [**Encuentro de San Francisco Rust**](https://luma.com/tp6w7tc9)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Vara & Equipo](https://luma.com/events-by-vara-gear)
    * [**Taller de Rust de Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343/)
* 2025-10-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2025 Reunión de SRUG (Seattle Rust User Group)**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Resumen del año**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro de Rust de octubre: ¡Una presentación especial y los encuentros mensuales están de vuelta!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Porter Square, 25 de octubre **](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-30 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-11-01 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Chinatown, 1 de noviembre **](https://www.meetup.com/bostonrust/events/311039492/)

### Oceanía
* 2025-10-22 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de octubre**](https://www.meetup.com/rust-canberra/events/311234237/)

### América del Sur
* 2025-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Octubre Async - Escribimos un Runtime desde Cero!**](https://www.meetup.com/rust-argentina/events/311276950/)
* 2025-10-25 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)
* 2025-10-30 | Florianópolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Para mí, personalmente, lo mejor de tener éxito en cualquier cosa es que obtienes la capacidad de levantar a los demás.

– [Nell Shamrell-Harrington en RustConf](https://youtu.be/nEHLIUWO78I?t=1175) (enlace de video de YouTube, ¡el resto de la charla también es genial!)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1720) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1o1uqur/this_week_in_rust_620/)</small>
