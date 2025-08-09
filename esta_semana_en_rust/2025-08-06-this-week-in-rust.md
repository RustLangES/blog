---
title: "Esta semana en Rust #70"
number_of_week: 70
description: El crate de esta semana es saphyr, una bifurcación de yaml-rust que se mantiene activamente.
date: 2025-08-06
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

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabe qué categoría usar, no dude en enviar un PR de todos modos y simplemente pida a los editores que seleccionen la categoría. -->

### Boletines
* [El Problema de Rustacean Incrustado #51](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-51)
* [Bioinformática en Rust #2](https://dawnandrew100.github.io/seq.rs/posts/2025/august/)

### Actualizaciones de proyectos/herramientas
* [GCC Front-End For Rust - Informe mensual de julio de 2025](https://rust-gcc.github.io/2025/08/05/2025-07-monthly-report.html)
* [Anuncio de ChaCha12-BLAKE3: cifrado seguro, simple y rápido para cualquier CPU](https://kerkour.com/chacha12-blake3)
* ['r3bl_tui' v0.7.3 liberado](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.7.3-tui)
* ['r3bl-cmdr' v0.0.21 publicado](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.21-cmdr)
* [Rama 0.3.0-alpha.2 — Grupos de conexiones criptográficas, ACME y mejores](https://github.com/plabayo/rama/discussions/657)
* [Meilisearch 1.16 — incrustaciones multimodales y API de migración directa](https://www.meilisearch.com/blog/meilisearch-1-16)

### Observaciones/Pensamientos
* [Cómo hago juegos en 3D](https://www.youtube.com/watch?v=hAWv6AJ8M-Y)
* [Inyectar Java desde bibliotecas nativas en Android](https://octet-stream.net/b/scb/2025-08-03-injecting-java-from-native-libraries-on-android.html)
* [Usted es el BIOS ahora: Construyendo un hipervisor en Rust con KVM](https://yeet.cx/blog/you-are-the-bios-now)
* [Opsqueue: cola de procesamiento por lotes ligera para cargas pesadas](https://www.channable.com/tech/introducing-opsqueue)
* [Rust, Python y TypeScript: la nueva trifecta](https://smallcultfollowing.com/babysteps/blog/2025/07/31/rs-py-ts-trifecta/)
* [Cambios complejos de codificación de vibración en Rust](https://www.youtube.com/watch?v=EL7Au1tzNxE)
* [El patrón de generatividad en Rust](https://arhan.sh/blog/the-generativity-pattern-in-rust/)
* [Las complejidades de las transmisiones asíncronas de Rust](https://swatinem.de/blog/rust-async-streams/)
* [Cómo Rust me ayudó a escribir mejor código](https://forgestream.idverse.com/blog/20250805-how-rust-helped-me-write-better-code/)

### Tutoriales de Rust
* [construyendo un mapa hash simple](https://viniciusx.com/blog/building-a-hash-map/)
* [Un problema fácil hecho difícil: Rust y árboles binarios](https://mmhaskell.com/blog/2025/8/4/an-easy-problem-made-hard-rust-amp-binary-trees)
* [Enviar más datos que el dispositivo a través de Bluetooth LE con Rust](https://medium.com/@potto_94870/send-more-than-device-data-over-bluetooth-le-with-rust-97885316b42d)
* [Optimización de compilaciones de Rust con banderas de destino](https://ianwwagner.com/til/optimizing-rustc-target-features)
* [video] [Compilación con Naz: código Claude + perfiles y rendimiento de Rust](https://www.youtube.com/watch?v=7iLMdNc-zOs)

### Miscelánea
* [Construyendo una nueva nube pública con Rust](https://filtra.io/rust/interviews/fly-io-aug-25)

## Crate de la semana

El crate de esta semana es [saphyr](https://crates.io/crates/saphyr), una bifurcación de yaml-rust que se mantiene activamente.

¡Gracias a [Félix Saparelli](https://users.rust-lang.org/t/crate-of-the-week/2704/1458) por la sugerencia!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas de prueba esta semana por [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
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
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

* [rama - admite respuestas de aplicación / octeto-stream](https://github.com/plabayo/rama/issues/647)
* [rama - agregar rama-pac: implementación inicial con PacConnector como soporte principal de API](https://github.com/plabayo/rama/issues/566)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

406 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-29..2025-08-05

#### Compilador
* [descenso AST de tramo completo](https://github.com/rust-lang/rust/pull/144557)
* [expandir las obligaciones de WF al verificar las llamadas a métodos](https://github.com/rust-lang/rust/pull/144704)
* [Optimizar restricciones de región](https://github.com/rust-lang/rust/pull/144446)
* [realizar 'check_private_in_public' por módulo](https://github.com/rust-lang/rust/pull/144479)

#### Biblioteca
* [añadir 'core::mem::D ropGuard'](https://github.com/rust-lang/rust/pull/144236)
* [constificar 'with_exposed_provenance'](https://github.com/rust-lang/rust/pull/144539)
* [constificar algunas funciones más de 'Resultado'](https://github.com/rust-lang/rust/pull/143771)
* [marcar 'rebanada::swap_with_slice' inestablemente constante](https://github.com/rust-lang/rust/pull/142205)
* [implementar 'push_mut'](https://github.com/rust-lang/rust/pull/135975)
* [implementar macro 'hash_map'](https://github.com/rust-lang/rust/pull/144070)
* [implementación: '#[feature(sync_nonpoison)]', '#[feature(nonpoison_mutex)]'](https://github.com/rust-lang/rust/pull/144022)

#### Rustdoc
* [Mostrar el tiempo total y el tiempo de compilación de las pruebas de documentos fusionadas](https://github.com/rust-lang/rust/pull/144308)
* [Mostrar attrs inseguros con envoltorios 'unsafe()' edición 2024](https://github.com/rust-lang/rust/pull/143662)

#### Clippy
* ['{flat_,}map_identity': reconocer (tupla) 'struct' de- y reestructuración](https://github.com/rust-lang/rust-clippy/pull/15261)
* [extender 'implicit_clone' para manejar llamadas 'to_string'](https://github.com/rust-lang/rust-clippy/pull/14177)
* [corregir 'iter_on_single_items' falso positivo en punteros de función y let stmts](https://github.com/rust-lang/rust-clippy/pull/15013)
* [corregir 'min_ident_chars': ignorar en el rasgo impl](https://github.com/rust-lang/rust-clippy/pull/15275)
* [corregir 'search_is_some' sugiere erróneamente dentro de macro](https://github.com/rust-lang/rust-clippy/pull/15135)
* [corrección que no mostraba pelusas obsoletas](https://github.com/rust-lang/rust-clippy/pull/15407)
* [arreglar opción-si-de-otra pelusa](https://github.com/rust-lang/rust-clippy/pull/15394)
* ['let_with_type_underscore': no coma el paréntesis de cierre en 'let (i): _ = 0;'](https://github.com/rust-lang/rust-clippy/pull/15386)
* [optimizar 'broken_links' en un 99,77%](https://github.com/rust-lang/rust-clippy/pull/15385)
* [optimizar algunos usos de '!! ' y '--' en sugerencias](https://github.com/rust-lang/rust-clippy/pull/15366)
* [simplificar la expresión booleana en 'manual_assert'](https://github.com/rust-lang/rust-clippy/pull/15368)
* [dividir 'possible_missing_else' de 'suspicious_else_formatting'](https://github.com/rust-lang/rust-clippy/pull/15317)

#### Analizador de Rust
* [al cambiar el nombre de un parámetro a 'self', cambie los llamadores para que usen la sintaxis de llamada al método](https://github.com/rust-lang/rust-analyzer/pull/20351)
* [corregir la plantilla de documento de pánicos de generación para 'debug_assert'](https://github.com/rust-lang/rust-analyzer/pull/20300)
* [ir correctamente a 'From' impl cuando está en 'into ()' incluso cuando la llamada está dentro de una macro](https://github.com/rust-lang/rust-analyzer/pull/20382)
* [no requiere que todas las definiciones de cambio de nombre sean renombrables](https://github.com/rust-lang/rust-analyzer/pull/20333)
* [en 'generate_mut_trait_impl', no agregue una tabulación si el cliente no admite fragmentos de código](https://github.com/rust-lang/rust-analyzer/pull/20336)
* [al mostrar una proyección en un parámetro de tipo que tiene límites como 'rasgo impl', recopile solo los límites de esta proyección](https://github.com/rust-lang/rust-analyzer/pull/20337)
* [mejorar el título y las descripciones del árbol de configuración](https://github.com/rust-lang/rust-analyzer/pull/20154)
* [reorganizar proc-macro-srv más, agregar '--format' y '--version' args](https://github.com/rust-lang/rust-analyzer/pull/20342)

### Triaje de rendimiento del compilador de Rust

Semana positiva en general, pero la mayoría de los cambios se producen en escenarios incrementales muy pequeños.

Triaje realizado por **@panstromek**.
Rango de revisión: [e3514bde.. 07b7dc90](https://perf.rust-lang.org/?start=e3514bde96d2d13586337a48db77fa64b850d249&end=07b7dc90ee4df5815dbb91ef8e98cb93571230f5&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,2%, 1,0%] | 28 |
| Regresiones ❌ <br /> (secundaria) | 0,7% | [0,0%, 2,8%] | 29 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-2,7%, -0,1%] | 95 |
| Mejoras ✅ <br /> (secundario) | -0,9% | [-9,2%, -0,0%] | 62 |
| Todos ❌✅ (primarios) | -0,3% | [-2,7%, 1,0%] | 123 |

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/603527a3a03aeda30c72a4ce60b9999a0ecfc71c/triage/2025/2025-08-04.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* [Problema de seguimiento para 'derive_from' (RFC 3809)](https://github.com/rust-lang/rust/issues/144889)
* [RFC: '--crate-attr'](https://github.com/rust-lang/rfcs/pull/3791)

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Extraer TraitImplHeader en AST/HIR](https://github.com/rust-lang/rust/pull/144386)
* [implementar Suma y Producto para Saturar(u\*)](https://github.com/rust-lang/rust/pull/144275)
* [Estabilizar as_array_of_cells](https://github.com/rust-lang/rust/pull/144054)
* [Problema de seguimiento para 'array::repeat'](https://github.com/rust-lang/rust/issues/126695)
* [Problema de seguimiento para 'core::iter::chain'](https://github.com/rust-lang/rust/issues/125964)
* [Problema de seguimiento para const_exposed_provenance](https://github.com/rust-lang/rust/issues/144538)
* [disposición: cerrar] [Cambiar el nombre de bool::ok_or[_else] a bool::then_ok_or[_else] para evitar confusiones con Option::ok_or[_else]](https://github.com/rust-lang/rust/pull/144037)
* [Agregue 'Predeterminado' impls para 'Pin'ned 'Box', 'Rc', 'Arc'](https://github.com/rust-lang/rust/pull/143717)
* [impl 'PartialEq<{str,String}> for {Path,PathBuf}'](https://github.com/rust-lang/rust/pull/140956)
* [Estabilizar 'ip_from'](https://github.com/rust-lang/rust/pull/141744)
* [Problema de seguimiento para {BTreeMap,BTreeSet}::extract_if](https://github.com/rust-lang/rust/issues/70530)
* [Problema de seguimiento para const {OsString, PathBuf}::new](https://github.com/rust-lang/rust/issues/141520)
* [Estabilizar varargs de estilo C para System, Sysv64, Win64, EFIAPI, AAPCS](https://github.com/rust-lang/rust/pull/144066)
* [Rechazar límites relajados dentro de los límites de tipo asociados (ATB)](https://github.com/rust-lang/rust/pull/135331)
* [Comience a informar futuras roturas para 'ILL_FORMED_ATTRIBUTE_INPUT' en dependencias](https://github.com/rust-lang/rust/pull/144544)
* [Estabilizar las características objetivo 'sse4a' y 'tbm'](https://github.com/rust-lang/rust/pull/144542)
* [Agregar pelusa contra entero para transmutar el puntero](https://github.com/rust-lang/rust/pull/144531)
* [Estabilizar loongarch32 asm en línea](https://github.com/rust-lang/rust/pull/144402)

*Ningún artículo entró en el período de comentarios finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [transmutación de próxima generación](https://github.com/rust-lang/rfcs/pull/3844)
* [nuevo] [RFC: Etiquetas de seguridad](https://github.com/rust-lang/rfcs/pull/3842)

## Próximos eventos

Rusty Eventos entre 2025-08-06 - 2025-09-03 🦀

### Virtual

* 2025-08-06 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/309997055)
* 2025-08-10 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002457)
* 2025-08-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361531)
* 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
  - [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
  - [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820307)
* 2025-08-17 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002458)
* 2025-08-19 | Virtual (Santa Clara, CA, EE. UU.) | [Comunidad de Extensión de la UCSC](https://www.meetup.com/ucsc-extension-community/events/)
  - [**Programación con Rust**](https://www.meetup.com/ucsc-extension-community/events/310108013)
* 2025-08-19 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
  - [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757756)
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
  - [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
  - [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)
* 2025-08-21 | Virtual (Londres, Reino Unido) | [Conf42: Eventos tecnológicos en línea](https://www.meetup.com/conf42/events/)
  - [**Conf42 Rustlang 2025**](https://www.meetup.com/conf42/events/305437705)
* 2025-08-21 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
  - [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567875)
* 2025-08-24 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002461)
* 2025-08-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
  - [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361442)
* 2025-08-28 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305878943)
* 2025-08-31 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002471)
* 2025-09-02 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Rust de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304234)
* 2025-09-02 * 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)
* 2025-09-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcmbfb)

### Asia

* 2025-08-23 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
  * [**Reunión de Rustacean de agosto de 2025**](https://hasgeek.com/rustbangalore/august-2025-rustacean-meetup/)

### Europa

* 2025-08-06 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
  - [**Rust Girona Hack & Learn 08 2025**](https://lu.ma/eoydaar9)
* 2025-08-06 | Colonia, DE | [Colonia de Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en agosto: Introducción a Rust y herramientas de dependencia**](https://www.meetup.com/rustcologne/events/310279256)
* 2025-08-06 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
  - [**Alastair Harrison: Control de versiones para la era agéntica.**](https://www.meetup.com/oxford-rust-meetup-group/events/310101048)
* 2025-08-07 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️ 🌈 - Edición 005**](https://www.meetup.com/rust-berlin/events/310308105)
* 2025-08-13 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
  - [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/310014719)
* 2025-08-13 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
  - [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944036)
* 2025-08-16 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
  - [**Rust Embedded - Taller #4 @letsboot**](https://www.meetup.com/rust-basel/events/309894848)
* 2025-08-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
  - [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/310039453)
* 2025-08-19 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
  - [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592249)
* 2025-08-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
  - [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062129)
* 2025-08-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester August Code Night**](https://www.meetup.com/rust-manchester/events/307919168)
* 2025-08-30 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/)
  * [**Foro Fika de Ferris #16**](https://www.meetup.com/stockholm-rust/events/310322522/)

### América del Norte

* 2025-08-07 | Montreal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
  - [**Social mensual de agosto**](https://www.meetup.com/rust-montreal/events/310259905)
* 2025-08-07 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
  - [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/310030338)
* 2025-08-07 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
  - [**macros!**](https://www.meetup.com/stl-rust/events/306648747)
* 2025-08-08 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
  - [**Almuerzo de Rust del noreste, 8 de agosto **](https://www.meetup.com/bostonrust/events/310106298)
* 2025-08-12 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Rust + Gametank y una suave introducción a la optimización**](https://www.meetup.com/rust-nyc/events/310279438)
* 2025-08-12 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
  - [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284338)
* 2025-08-14 | Híbrido (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
  - [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-14 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust)
  - [**Programación de un robot de combate en Rust con Rex Magana**](https://www.meetup.com/utah-rust/events/310053631)
* 2025-08-14 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Agosto de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/307698880)
* 2025-08-18 | Denver, CO, EE. UU. | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
  - [**FOSS Rust Hack Night**](https://mobilizon.us/events/9092695a-89f0-40fa-b3d0-50072827b0ec)
* 2025-08-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
  - [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731032)
* 2025-08-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
  - [**Rust on Bare Metal Serie 2: Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333117)
* 2025-08-23 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
  - [**Almuerzo de Rust de Somerville Union Square, 23 de agosto **](https://www.meetup.com/bostonrust/events/310106302)
* 2025-08-27 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
  - [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310205991)
* 2025-08-28 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/308675976)
* 2025-09-02 * 2025-09-05 | Híbrido (Seattle, WA, EE. UU.) | [RustConf](https://rustconf.com/)
  - [**RustConf 2025**](https://rustconf.com/)

### Oceanía

* 2025-08-11 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group)
  - [**Reunión de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/308880707)
* 2025-08-26 | Barton, AC, AU | [Grupo de Usuarios de Rust de Canberra (CRUG)](https://www.meetup.com/rust-canberra)
  - [**Reunión de agosto**](https://www.meetup.com/rust-canberra/events/308746519)
* 2025-08-27 * 2025-08-30 | Wellington, Nueva Zelanda | [Forja de Rust](https://rustforgeconf.com/)
  - [**Forja de Rust**](https://rustforgeconf.com/)

### América del Sur

* 2025-08-07 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
  - [**Rust Uruguay meetup de Agosto**](https://www.meetup.com/rust-uruguay/events/310004109)
* 2025-08-21 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Agosto de Protocol Buffers!**](https://www.meetup.com/rust-argentina/events/310019573)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Además de reescribir las herramientas en Rust, un lenguaje más seguro, algunas características poco utilizadas de sudo no se implementaron para reducir la superficie de vulnerabilidad. Esto resultó ser significativo en julio de 2025 cuando se descubrieron dos vulnerabilidades (..) en funciones de sudo que no estaban implementadas en sudo-rs. En respuesta a uno de ellos, sudo ha quedado obsoleto y eliminará la característica que aloja la vulnerabilidad.

– [Josh Aas en el blog de prossimo](https://www.memorysafety.org/blog/sudo-rs-headed-to-ubuntu)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1708) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1mkayhe/this_week_in_rust_611/)</small>
