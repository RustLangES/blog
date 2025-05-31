---
title: "Esta semana en Rust #61"
number_of_week: 61
description: El crate de esta semana es boreal seguro y eficaz.
date: 2025-05-28
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
* [Actualización de Objetivos del Proyecto de Abril](https://blog.rust-lang.org/2025/05/26/april-project-goals-update/)
* [Degradando i686-pc-windows-gnu a Tier 2](https://blog.rust-lang.org/2025/05/26/demoting-i686-pc-windows-gnu/)

### Fundación
* [La Fundación Rust solicita aportaciones sobre su estrategia trienal (2026-2028)](https://docs.google.com/forms/d/e/1FAIpQLSca3ziiYWrTti6Ti2ki3Sv9Okmhzc4wGMxQgudUOmQrmh3pVg/viewform?usp=dialog)

### Actualizaciones de proyectos/herramientas
* [gitoxide mayo de 2025](https://github.com/GitoxideLabs/gitoxide/discussions/2021#discussion-8357816)
* [¡El backend del compilador GCC ahora puede arrancar completamente el compilador de Rust!](https://old.reddit.com/r/rust/comments/1ktph3c/media_the_gcc_compiler_backend_can_now_fully/)
* [Lanzamiento de Rust Coreutils 0.1 con grandes ganancias de rendimiento](https://www.phoronix.com/news/Rust-Coreutils-0.1-Released)
* [Presentación de Roto: Un lenguaje de scripting compilado para Rust](https://blog.nlnetlabs.nl/introducing-roto-a-compiled-scripting-language-for-rust/)
* [alpine-rustx: Compilación cruzada simple de Rust usando imágenes Docker personalizadas](https://github.com/tindzk/alpine-rustx)
* [Buscador de tareas 2.9.0](https://codeberg.org/kdwarn/taskfinder/src/commit/9d2779bfdd79826374bc5e77b85928c065b1094b/CHANGELOG.md#2-9-0-https-codeberg-org-kdwarn-taskfinder-compare-v2-8-0-v2-9-0-2025-05-22)
* [Lanzamiento de la segunda alfa de Yelken](https://bwqr.github.io/yelken-blog/second-alpha-release/)
* [Primer vistazo a Blinksy](https://blog.mikey.nz/first-look-at-blinksy/)
* [malai 0.2.5 ya está aquí: Comparta instantáneamente los servicios TCP locales (base de datos/SSH) con otros](https://www.malai.sh/hello-tcp/)

### Observaciones/Pensamientos
* [Enlaces profundos de iOS con Bevy](https://rustunit.com/blog/2025/05-18-bevy-ios-deep-linking/)
* [Sguaba: transformadas de cuerpo rígido difíciles de abusar para ingenieros con otras cosas de las que preocuparse además del álgebra lineal](https://blog.helsing.ai/sguaba-hard-to-misuse-rigid-body-transforms-for-engineers-with-other-things-to-worry-about-than-aeaa45af9e0d)
* [Haciendo que el decodificador de video rav1d sea un 1% más rápido](https://ohadravid.github.io/posts/2025-05-rav1d-faster/)
* [Asincrónico desde cero 3: Clavado contra la pared](https://natkr.com/2025-05-22-async-from-scratch-3/)
* [Unión de bifurcaciones: ¿más allá de OpenMP en C++ y Rust?](https://ashvardanian.com/posts/beyond-openmp-in-cpp-rust/)
* [Lenguaje de programación: Rust 2024 es la edición más completa hasta la fecha](https://www.heise.de/en/background/Programming-language-Rust-2024-is-the-most-comprehensive-edition-to-date-10393917.html)
* [Recursividad acotada a nivel de tipo en Rust](https://catgirl.ai/log/typelevel-bounded-recursion/)
* [Una historia de capacidad de prueba y envío de tipos que no son de envío en Rust](https://geo-ant.github.io/blog/2025/rust-testability-and-non-send-types/)
* [video] [Hot-reloading Rust Game Dev: Coding Flappy Bird in Bevy From Scratch](https://www.youtube.com/watch?v=fo6FXxeP0Wg)
* [SIMD en zlib-rs (parte 2): compare256](https://tweedegolf.nl/en/blog/155/simd-in-zlib-rs-part-2-compare256)

### Tutoriales de Rust
* [Secretos que los gestores consideran perjudiciales. Cómo cifrar de forma segura tus datos confidenciales con el cifrado de sobres y KMS en Rust](https://kerkour.com/rust-secrets-kms-envelope-encryption)
* [Rust, rendimiento de memoria y latencia](https://developerlife.com/2025/05/19/rust-mem-latency/)

### Miscelánea
* [Cómo Conseguir Un Trabajo De Rust Parte II: Introduciendo Rust En Su Empresa Actual](https://filtra.io/rust/career-help/how-to-get-a-rust-job-II)

## Crate de la semana

El crate de esta semana es [boreal](https://github.com/vthib/boreal), un evaluador de reglas [YARA](https://virustotal.github.io/yara/) seguro y eficaz.

¡Gracias a [Vincent Thiberville](https://users.rust-lang.org/t/crate-of-the-week/2704/1439) por la autosugerencia!

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

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Hyperswitch - Agregar implementación de verificación de integridad en Adyen](https://github.com/juspay/hyperswitch/issues/8149)
* [Hyperswitch - Agregar implementación de verificación de integridad en Authorize.net](https://github.com/juspay/hyperswitch/issues/8150)
* [Hyperswitch - Agregar implementación de verificación de integridad en ACI](https://github.com/juspay/hyperswitch/issues/8151)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

[EuroRust 2025](https://www.papercall.io/eurorust-2025)| La CFP cierra el 02/06/2025 | París, Francia | 2025-10-09

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 433 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-20..2025-05-27

#### Compilador

* [No repetir goles si ninguno de sus VAR ha cambiado](https://github.com/rust-lang/rust/pull/141500)
* [Fold predicate fast path in canonicalizer and eager resolver](https://github.com/rust-lang/rust/pull/141442)

#### Biblioteca

* [add 'std::os::unix::p rocess::CommandExt::chroot' para hacer chroot de forma segura un proceso hijo](https://github.com/rust-lang/rust/pull/137759)
* [corregir error de aliasing en la implementación del proceso UNIX](https://github.com/rust-lang/rust/pull/138896)
* [implementar 'ptr::try_cast_aligned' y 'NonNull::try_cast_aligned'](https://github.com/rust-lang/rust/pull/141222)
* [implementar 'advance_by' a través de 'try_fold' para iteradores 'Dimensionados'](https://github.com/rust-lang/rust/pull/141086)

#### Carga

* [toml: Eliminar la solución alternativa para el soporte de frontmatter de rustc](https://github.com/rust-lang/cargo/pull/15570)
* [añadir '-Zfix-edition'](https://github.com/rust-lang/cargo/pull/15596)
* [añadir la futura edición](https://github.com/rust-lang/cargo/pull/15595)
* [Extracción directa para fuentes de registro](https://github.com/rust-lang/cargo/pull/15514)
* [archivos de proveedor con sufijo .rej/.orig](https://github.com/rust-lang/cargo/pull/15569)

#### Rustdoc

* [Unificar la representación de alias de tipo con otro ADT](https://github.com/rust-lang/rust/pull/140863)
* [en dispositivos móviles, haz que la barra lateral sea de ancho completo y ajuste de línea](https://github.com/rust-lang/rust/pull/139831)
* [acelerar 'TypeAliasPart::get'](https://github.com/rust-lang/rust/pull/141421)

#### Clippy

* ['manual_flatten': arreglar con el patrón anidado 'Some' o 'Ok'](https://github.com/rust-lang/rust-clippy/pull/14846)
* ['needless_borrow': no contradice 'dangerous_implicit_autorefs'](https://github.com/rust-lang/rust-clippy/pull/14810)
* [considerar las consts en los patrones como refutables](https://github.com/rust-lang/rust-clippy/pull/14887)
* [Corregir 'assign_op_pattern' falso positivo en el rasgo const inestable](https://github.com/rust-lang/rust-clippy/pull/14886)
* [Arreglar 'manual_find' sugerencia incorrecta cuando el tipo de retorno necesita ajuste](https://github.com/rust-lang/rust-clippy/pull/14892)
* [Arreglar la sugerencia incorrecta de 'needless_for_each' cuando el cierre no tiene tirantes](https://github.com/rust-lang/rust-clippy/pull/14735)
* [arreglar 'manual_slice_size_computation' ICE y disparador en el contexto 'const'](https://github.com/rust-lang/rust-clippy/pull/14804)
* [hacer que 'trivial-copy-size-limit' sea consistente con el tamaño del puntero objetivo](https://github.com/rust-lang/rust-clippy/pull/13319)
* [varias correcciones de macros para lints de bucle](https://github.com/rust-lang/rust-clippy/pull/14631)

#### Analizador de Rust

* [cambiar el prefijo de importación predeterminado a por caja](https://github.com/rust-lang/rust-analyzer/pull/19819)
* [establecer correctamente el intervalo de los delimitadores de grupo de la caja 'proc_macro'](https://github.com/rust-lang/rust-analyzer/pull/19839)
* [arreglar la resolución del IDE de las macros de elementos](https://github.com/rust-lang/rust-analyzer/pull/19862)
* [Solucionar problemas de caché con el nivel de pelusas](https://github.com/rust-lang/rust-analyzer/pull/19824)
* [ide-assists, generate mut trait impl indent](https://github.com/rust-lang/rust-analyzer/pull/19792)
* [normalizar al comprobar los tipos deshabitados para comprobar la exhaustividad del patrón](https://github.com/rust-lang/rust-analyzer/pull/19851)
* [implementar correctamente 'might_be_inside_macro_call()' usando información semántica en lugar de hacks sintácticos](https://github.com/rust-lang/rust-analyzer/pull/19864)
* [ide-assists, la sangría 'generate_new' pierde](https://github.com/rust-lang/rust-analyzer/pull/19785)

### Clasificación del rendimiento del compilador de Rust

Una semana dominada por nuevos focos de ruido. En general, no hubo muchos cambios reales en el rendimiento del compilador. Algunos aspectos destacados del cambio real en el que centrarse son una mejora en rustdoc que obtuvo grandes victorias en algunos puntos de referencia clave y una mejora en la selección de rasgos que proviene de pasar de una cadena if/else a una coincidencia de patrones.

Triaje realizado por **@rylev**.
Rango de revisión: [59372f2c.. 2805e1dc](https://perf.rust-lang.org/?start=59372f2c81ba74554d9a71b12a4ed7f29adb33a2&end=2805e1dc4c18ed4c84d161502c48da870c56f68a&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.7% | [0.1%, 7.5%] | 73 |
| Regresiones ❌ <br /> (secundaria) | 1.4% | [0.1%, 6.8%] | 34 |
| Mejoras ✅ <br /> (primario) | -4.0% | [-78.5%, -0.1%] | 41 |
| Mejoras ✅ <br /> (secundaria) | -6,2% | [-22.1%, -0.1%] | 28 |
| Todos ❌✅ (primarios) | -1.0% | [-78.5%, 7.5%] | 114 |

5 regresiones, 7 mejoras, 8 mixtas; 9 de ellos en rollups
45 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/02eafc9ca0dda4c5851fb38850166b8af55eda91/triage/2025-05-27.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Permitir comparaciones entre 'CStr', 'CString' y 'Cow<CStr>'.](https://github.com/rust-lang/rust/pull/137268)
* [Problema de seguimiento para 'const_eq_ignore_ascii_case'](https://github.com/rust-lang/rust/issues/131719)
* [Estabilizar función result_flattening](https://github.com/rust-lang/rust/pull/141072)
* [Problema de seguimiento para la función 'breakpoint' ('core::arch::breakpoint')](https://github.com/rust-lang/rust/issues/133724)
* [Estabilizar 'sha512'. 'sm3' y 'sm4' para x86](https://github.com/rust-lang/rust/pull/140767)
* [Estabilizar bloqueador de teclas](https://github.com/rust-lang/rust/pull/140766)
* [terminología: objeto asignado → asignación](https://github.com/rust-lang/rust/pull/141224)
* [Problema de seguimiento para 'keylocker_x86'](https://github.com/rust-lang/rust/issues/134813)
* [Problema de seguimiento para 'sha512_sm_x86'](https://github.com/rust-lang/rust/issues/126624)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Use 'gix' para 'paquete de carga'](https://github.com/rust-lang/cargo/pull/15534)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Cambios en target_feature atributo](https://github.com/rust-lang/rfcs/pull/3820)
* [nuevo] [RFC: Promover aarch64-pc-windows-msvc a Nivel 1](https://github.com/rust-lang/rfcs/pull/3817)

## Próximos eventos

Eventos oxidados entre 2025-05-28 - 2025-06-25 🦀

### Virtual
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
* 04/06/2025 | Virtual | [Computación Científica en Rust](https://scientificcomputing.rs)
     * [**Computación científica en Rust 2025**](https://scientificcomputing.rs/2025)
* 05/06/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 07/06/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 08/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/307927093)
* 2025-06-10 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 La comunidad se pone al día**](https://www.meetup.com/women-in-rust/events/307560326)
* 2025-06-12 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 15/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308074808)
* 17/06/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170853)
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 22/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbdc)
* 24/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361436)
* 24/06/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Construyendo Raspadores Web Eficientes: Rust vs. Python para la Ingesta de Datos**](https://www.meetup.com/women-in-rust/events/306683025)

### Asia
* 08/06/2025 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Presencial Rust junio de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europa
* 28/05/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Tema sorpresa**](https://www.meetup.com/rust-rhein-main/events/307836400)
* 29/05/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 31/05/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 04/06/2025 | Gante, BE | [Programación de Sistemas Gante](https://www.sysghent.be/)
    * [**Vuélvete más inteligente con Rust incrustado**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 04/06/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Risc V - el nuevo retador para las CPU en IA y sistemas embebidos.**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 05/06/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-10 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/308080874)
* 2025-06-10 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw)
    * [**Rust Warsaw Meetup #5**](https://www.meetup.com/rust-warsaw/events/307955051)
* 11/06/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 17/06/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 19/06/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Encuentro de Rust Aarhus en Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 19/06/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 24/06/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 25/06/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Lecciones aprendidas de hacer un pequeño juego en nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)

### América del Norte
* 28/05/2025 | Albuquerque, Nuevo México, Estados Unidos | [En Ideas y Café](https://www.meetup.com/ideas-and-coffee)
    * [**Reunión de Rust a nivel de introducción**](https://www.meetup.com/ideas-and-coffee/events/307645653)
* 28/05/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 29/05/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 29/05/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307498676)
* 31/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Harvard Square, 31 de mayo**](https://www.meetup.com/bostonrust/events/307936097)
* 05/06/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/308091592)
* 05/06/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 08/06/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Universidad de Boston, 8 de junio**](https://www.meetup.com/bostonrust/events/307936165)
* 11/06/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 17/06/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Uso de Rust para Web Series 3: Presentaciones finales y redes sociales de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 19/06/2025 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro mensual: ¡Haciendo CRUD con Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-20 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Lechmere Rust, 20 de junio**](https://www.meetup.com/bostonrust/events/307936242)
* 25/06/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)

### Oceanía
* 16/06/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Encuentro de Rust en Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 24/06/2025 | Barton, Australia | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/307520854)

### América del Sur
* 31/05/2025 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 04/06/2025 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-06-12 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Junio de WebAssembly!**](https://www.meetup.com/rust-argentina/events/307990465)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1knkfb6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Esta es básicamente la versión de programación de "aprender japonés como hablante de inglés es difícil, por lo tanto, no es un buen idioma para que los bebés aprendan"

– [/u/Aaron1924 en /r/rust](https://www.reddit.com/r/programming/comments/1kqo2tc/comment/mt72ihj/) discutiendo si Rust podría ser un buen primer idioma o no.

¡Gracias a [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1688) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1kytv0p/this_week_in_rust_601_this_week_in_rust/)</small>
