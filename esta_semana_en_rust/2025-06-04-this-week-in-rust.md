---
title: "Esta semana en Rust #62"
number_of_week: 62
description: El crate de esta semana es context-logger, una biblioteca ligera y ergonómica para agregar contexto estructurado a sus registros.
date: 2025-06-04
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
* [Anunciando cinco nuevos miembros del equipo compilador](https://blog.rust-lang.org/inside-rust/2025/05/30/compiler-team-new-members/)
* [Rediseño de la secuencia inicial de arranque](https://blog.rust-lang.org/inside-rust/2025/05/29/redesigning-the-initial-bootstrap-sequence/)

### Actualizaciones de proyectos/herramientas
* [¡Lanzamiento de ICU4X 2.0!](https://blog.unicode.org/2025/05/icu4x-20-released.html)
* [Freya 0.3](https://freyaui.dev/posts/0.3)
* [godot-rust mayo de 2025 dev update](https://godot-rust.github.io/dev/may-2025-update/)
* [Novedades de SeaORM 1.1.12](https://www.sea-ql.org/blog/2025-06-01-whats-new-in-sea-orm-1.1/)
* [¡Lanzamiento de Git-Cliff 2.9.0!](https://git-cliff.org/blog/2.9.0/)
* [Desafío "Rata en la naturaleza" de Ratatui](https://github.com/ratatui/ratatui/discussions/1886)
* [registro de cambios de rust-analyzer #288](https://rust-analyzer.github.io/thisweek/2025/06/02/changelog-288.html)
* [Fusión de la frontera del bloque Ratatui](https://jslazak.com/ratatui-border-merging/)

### Observaciones/Pensamientos
* [parking_lot: ffffffffff...](https://fly.io/blog/parking-lot-ffffffffffffffff/)
* [Cómo lidiar con las dependencias de Rust](https://notgull.net/rust-dependencies/)
* [Elogio de Shuttle: Oxidando la API web de Capibara](https://justinwoodring.com/blog/rewriting-the-capibara-web-api-in-rust/)
* [Reducción del tamaño del directorio de destino de Cargo con -Zno-embed-metadata](https://kobzol.github.io/rust/rustc/2025/06/02/reduce-cargo-target-dir-size-with-z-no-embed-metadata.html)
* [Diseño de tipos de error en bibliotecas de Rust](https://d34dl0ck.me/rust-bites-designing-error-types-in-rust-libraries/index.html)
* [¿Por qué usar errores estructurados en aplicaciones de Rust?](https://home.expurple.me/posts/why-use-structured-errors-in-rust-applications/)
* [video] [La virtud de unsynn](https://www.youtube.com/watch?v=YtbUzIQw-so)
* [audio] [El proxy es solo un enrutamiento tonto](https://sdr-podcast.com/episodes/proxying-is-just-dumb-routing/)
* [audio] [David Lattimore - Enlazador más rápido, compilaciones más rápidas](https://timclicks.dev/podcast/david-lattimore-faster-linker-faster-builds)
* [audio] [Rust con Niko Matsakis](https://corrode.dev/podcast/s04e04-rust/)
* [audio] [SWC con DongYoon Kang](https://rustacean-station.org/episode/dongyoon-kang/)
* [audio] [AccessKit con Matt Campbell y Arnold Loubriat](https://rustacean-station.org/episode/accesskit-with-matt-campbell-and-arnold-loubriat/)

### Tutoriales de Rust
* [Libro de frases de C++ a Rust](https://cel.cs.brown.edu/crp/)
* [Los rasgos asincrónicos pueden ser respaldados directamente por implementaciones futuras manuales](https://blog.yoshuawuyts.com/async-traits-can-be-directly-backed-by-manual-future-impls/)
* [Cómo estructuramos nuestro build.rs en Rust](https://www.evolvebenchmark.com/blog-posts/how-we-wrap-external-c-and-cpp-libraries-in-rust)
* [video] [Iniciar un nuevo juego Bevy 2d con el CLI de Bevy](https://www.youtube.com/watch?v=ez4oGeM3X2o)
* [video] [Build with Naz : Analizar entrada sin corte con nom](https://www.youtube.com/watch?v=3IzAweJGdZU)

### Investigación
* [Un primer vistazo a las aplicaciones de ROS 2 escritas en Rust asíncrono](https://arxiv.org/abs/2505.21323)

### Miscelánea
* [Repensando la transmisión de datos con Rust e InfinyOn](https://filtra.io/rust/interviews/infinyon-jun-25)

## Crate de la semana

El crate de esta semana es [context-logger](https://github.com/alekseysidorov/context-logger), una biblioteca ligera y ergonómica para agregar contexto estructurado a sus registros.

¡Gracias a [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1440) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.
### [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing)
* [Problema de seguimiento para -zrustdoc-depinfo](https://github.com/rust-lang/cargo/issues/15370)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/15370#issuecomment-2932749085)
* [Problema de seguimiento para la finalización nativa](https://github.com/rust-lang/cargo/issues/14520)
  * [Instrucciones de prueba](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#native-completions)
* [Redefinir 'CARGO_TARGET_DIR' para que sea solo un directorio de artefactos](https://github.com/rust-lang/cargo/issues/14125)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/14125#issuecomment-2802967106)
* [instalación de carga --funcionamiento en seco](https://github.com/rust-lang/cargo/issues/11123)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/11123#issuecomment-2460846663)
* [Paquete de carga --el espacio de trabajo no es muy útil](https://github.com/rust-lang/cargo/issues/10948)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/10948#issuecomment-2540365225)
* [Cargo publica varios paquetes a la vez](https://github.com/rust-lang/cargo/issues/1169)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/1169#issuecomment-2540367442)

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*Esta semana no se han presentado convocatorias para participar.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 529 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-27..2025-06-03

#### Compilador

* [Corregir intervalos para carpetas inseguras](https://github.com/rust-lang/rust/pull/141781)
* [Nuevo solucionador de PGO](https://github.com/rust-lang/rust/pull/141453)
* [añadir rutas rápidas adicionales de 'TypeFlags'](https://github.com/rust-lang/rust/pull/141581)
* [agregar ruta rápida para maybe-initializationness en liveness](https://github.com/rust-lang/rust/pull/141667)

#### Biblioteca

* [Etapa de rediseño 0 std](https://github.com/rust-lang/rust/pull/119899)
* [implementar los métodos '((un)checked_)exact_div' para números enteros](https://github.com/rust-lang/rust/pull/141237)
* [agregar el parámetro Range a 'BTreeMap::extract_if' y 'BTreeSet::extract_if'](https://github.com/rust-lang/rust/pull/140825)
* [añadir 'CStr::d isplay'](https://github.com/rust-lang/rust/pull/139994)
* [añadir 'Resultado::map_or_default' y 'Opción::map_or_default'](https://github.com/rust-lang/rust/pull/141659)
* [añadir 'From<TryLockError>' para 'io::Error'](https://github.com/rust-lang/rust/pull/141312)
* [agregar soporte 'const' para métodos de redondeo flotante](https://github.com/rust-lang/rust/pull/141521)
* [agregar el método 'data_ptr' a Mutex y RwLock](https://github.com/rust-lang/rust/pull/140369)

#### Carga

* [trim-paths: remapear todas las rutas a 'build.build-dir'](https://github.com/rust-lang/cargo/pull/15614)
* [arreglar la carga agregar sobrescribir archivos Cargo.toml enlazados simbólicamente](https://github.com/rust-lang/cargo/pull/15281)

#### Rustdoc

* [limpiezas relacionadas con asignaciones](https://github.com/rust-lang/rust/pull/141573)
* [mostrar 'doc(cfg(false))' correctamente](https://github.com/rust-lang/rust/pull/141747)
* [enlazar a una macro de proc local ya no avisa](https://github.com/rust-lang/rust/pull/141411)
* [use información sobre herramientas descriptiva si doctest se ignora condicionalmente](https://github.com/rust-lang/rust/pull/141517)

#### Clippy

* ['explicit_deref_methods': no pelar en las cadenas de métodos](https://github.com/rust-lang/rust-clippy/pull/14921)
* ['needless_return': mira dentro de las partes 'else if' también](https://github.com/rust-lang/rust-clippy/pull/14798)
* ['while_let_loop': Incluir la tarea 'let' en la sugerencia](https://github.com/rust-lang/rust-clippy/pull/14756)
* [añadir pelusa 'infallible_try_from'](https://github.com/rust-lang/rust-clippy/pull/14813)
* [limpieza 'modulo_arithmetic'](https://github.com/rust-lang/rust-clippy/pull/14898)
* [extender 'manual_is_variant_and lint' para comprobar si hay comparaciones de mapas booleanos](https://github.com/rust-lang/rust-clippy/pull/14646)
* [arreglar que 'dbg_macro' falle al manejar la desazúcar de corrutina asíncrona](https://github.com/rust-lang/rust-clippy/pull/14937)
* [arreglar 'semicolon_outside_block' sugiere erróneamente cuando está dentro de macros](https://github.com/rust-lang/rust-clippy/pull/14954)
* [mejorar la redacción de los documentos de 'manual_contains'](https://github.com/rust-lang/rust-clippy/pull/14917)
* [nueva restricción lint: 'pointer_format'](https://github.com/rust-lang/rust-clippy/pull/14792)
* [optimizar 'unit_return_expecting_ord'](https://github.com/rust-lang/rust-clippy/pull/14905)
* [use símbolos internos en lugar de cadenas en más lugares](https://github.com/rust-lang/rust-clippy/pull/14855)

#### Analizador de Rust

* [tenga en cuenta las acciones de 'Generar' al filtrar las permitidas](https://github.com/rust-lang/rust-analyzer/pull/19899)
* [agregar una solución rápida para acceder a un campo privado de una 'estructura'](https://github.com/rust-lang/rust-analyzer/pull/19869)
* [agregar soporte para 'Semantics<'db, dyn HirDatabase>' borrado de tipos, mediante el uso de 'DB: ? Tamaño'](https://github.com/rust-lang/rust-analyzer/pull/19850)
* [habilitar la edición de asistencia para 'tupla<->estructura con nombre' para las palabras clave 'struct' y visibility](https://github.com/rust-lang/rust-analyzer/pull/19901)
* [desugar assist para 'let pat = expr?; ' → 'dejar lo demás'](https://github.com/rust-lang/rust-analyzer/pull/19881)
* [mejorar el cambio de nombre para incluir variaciones de identificadores generadas por macros](https://github.com/rust-lang/rust-analyzer/pull/19893)
* [renderizar información de relleno al pasar el cursor sobre estructuras](https://github.com/rust-lang/rust-analyzer/pull/19876)
* [manejadores de ciclo para 'HirDatabase::infer, const_param_ty_with_diagnostics'](https://github.com/rust-lang/rust-analyzer/pull/19894)
* [arreglar la capa IDE que no resuelve algunas llamadas de macro](https://github.com/rust-lang/rust-analyzer/pull/19879)
* [arreglar la inserción de importación que no es totalmente consciente de cfg](https://github.com/rust-lang/rust-analyzer/pull/19890)
* [corregir la inferencia del tipo de retorno 'AsyncFnX'](https://github.com/rust-lang/rust-analyzer/pull/19872)
* [manejar mejor los archivos incluidos en la capa IDE](https://github.com/rust-lang/rust-analyzer/pull/19880)
* [reconocer los ciclos de salsa en 'thread_result_to_response'](https://github.com/rust-lang/rust-analyzer/pull/19888)
* [Omitir análisis de patrones en discrepancias de tipos](https://github.com/rust-lang/rust-analyzer/pull/19875)

### Clasificación del rendimiento del compilador de Rust

Una semana bastante ajetreada, con muchos cambios en el rendimiento. La mayoría de los cambios
(al menos en cantidad de índices de referencia) son atribuibles a una actualización de nuestro PGO
a los puntos de referencia más nuevos como parte de la actualización de 2025.

Triaje realizado por **@simulacrum**.
Rango de revisión: [2805e1dc.. 2fc3deed](https://perf.rust-lang.org/?start=2805e1dc4c18ed4c84d161502c48da870c56f68a&end=2fc3deed9fcb8762ad57191e0195f06f7543e4a5&absolute=false&stat=instructions%3Au)

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-06-02.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Agregar (atrás) unsupported_calling_conventions lint para rechazar más convenciones de llamada no válidas](https://github.com/rust-lang/rust/pull/141435)
* [Estabilizar los resguardos 'if let' ('feature(if_let_guard)')](https://github.com/rust-lang/rust/pull/141295)
* [Añadida la implementación de 'Clone' para 'ChunkBy'](https://github.com/rust-lang/rust/pull/138016)
* [Hacer que la pelusa 'dangerous_implicit_autorefs' se deniegue por defecto](https://github.com/rust-lang/rust/pull/141661)
* [Hacer que el carácter distinto de cero< > posible](https://github.com/rust-lang/rust/pull/141001)
* [Problema de seguimiento para nonnull_provenance](https://github.com/rust-lang/rust/issues/135243)
* [disposición: cerrar] [Implementar operaciones para envolver< T > donde Rhs = T](https://github.com/rust-lang/rust/pull/140567)
* [Divide la pelusa 'unknown_or_malformed_diagnostic_attributes'](https://github.com/rust-lang/rust/pull/140717)
* [Lint en comparaciones de punteros fn en macros externas](https://github.com/rust-lang/rust/pull/134536)
* [Especifique el comportamiento de 'file!'](https://github.com/rust-lang/rust/pull/134442)
* [Representación del documento de 'Opción<insegura fn()>'](https://github.com/rust-lang/rust/pull/141447)
* [Estabilizar 'característica(generic_arg_infer)'](https://github.com/rust-lang/rust/pull/141610)
* [Permitir '#! [doc(test(attr(..)))]' en todas partes](https://github.com/rust-lang/rust/pull/140560)
* [Problema de seguimiento para la API de bloqueo de archivos](https://github.com/rust-lang/rust/issues/130994)
* [disposición: no especificada] [Problema de seguimiento para 'unsigned_signed_diff'](https://github.com/rust-lang/rust/issues/126041)
* [Estabilizar 'os_string_pathbuf_leak'](https://github.com/rust-lang/rust/pull/137992)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC] Añadir el atributo '#[export_ordinal(n)]'](https://github.com/rust-lang/rfcs/pull/3641)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [de-RFC: Eliminar unsized_locals](https://github.com/rust-lang/rfcs/pull/3829)
* [nuevo] [RFC: Macros de procedimiento en el mismo paquete que la aplicación](https://github.com/rust-lang/rfcs/pull/3826)
* [nuevo] [RFC: Permitir implicaciones genéricas usando límites de rasgos locales](https://github.com/rust-lang/rfcs/pull/3821)

## Próximos eventos

Eventos oxidados entre 2025-06-04 - 2025-07-02 🦀

### Virtual
* 04/06/2025 | Virtual | [Computación Científica en Rust](https://scientificcomputing.rs)
    * [**Computación científica en Rust 2025**](https://scientificcomputing.rs/2025)
* 04/06/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031665)
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
* 11/06/2025 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversación con Herbert Wolverson de Ardan Labs y LibreQoS**](https://www.meetup.com/code-mavens/events/308234298/)
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
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308246353)
* 24/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361436)
* 24/06/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Construyendo Raspadores Web Eficientes: Rust vs. Python para la Ingesta de Datos**](https://www.meetup.com/women-in-rust/events/306683025)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 29/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
* 02/07/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031667)

### Asia
* 08/06/2025 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Presencial Rust junio de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europa
* 04/06/2025 | Gante, BE | [Programación de Sistemas Gante](https://www.sysghent.be/)
    * [**Vuélvete más inteligente con Rust incrustado**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 04/06/2025 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in June: A Decade of Rust**](https://www.meetup.com/rustcologne/events/308158241)
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
* 2025-06-12 | Berlín, DE | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️ 🌈 - Edition 003**](https://www.meetup.com/rust-berlin/events/308131380)
* 17/06/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 18/06/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/308129156)
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
* 2025-06-26 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Encuentro de Rust #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 02/07/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)

### América del Norte
* 05/06/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/308091592)
* 05/06/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 08/06/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Universidad de Boston, 8 de junio**](https://www.meetup.com/bostonrust/events/307936165)
* 11/06/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 2025-06-12 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/308019627)
* 17/06/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**programación asíncrona en Rust usando Tokio**](https://www.meetup.com/rust-mx/events/308248260)
* 19/06/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Uso de Rust para Web Series 3: Presentaciones finales y redes sociales de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 2025-06-20 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Lechmere Rust, 20 de junio**](https://www.meetup.com/bostonrust/events/307936242)
* 25/06/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Meetup mensual: ¡Haciendo una API CRUD con Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 28/06/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Back Bay Rust, 28 de junio**](https://www.meetup.com/bostonrust/events/307936269)

### Oceanía
* 16/06/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Encuentro de Rust en Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 24/06/2025 | Barton, Australia | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/307520854)

### América del Sur
* 04/06/2025 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
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

> Los verificadores de tipos de Python basados en Rust son como autobuses: esperas años para que llegue uno y luego aparecen dos a la vez.

– [u/fiddle_n en r/rustjerk](https://www.reddit.com/r/rustjerk/comments/1kx8uk8/comment/muns9to)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1695) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1l3q7u1/this_week_in_rust_602_this_week_in_rust/)</small>
