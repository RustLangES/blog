---
title: "Esta semana en Rust #64"
number_of_week: 64
description: El crate de esta semana es RobustMQ, una cola de mensajes multiprotocolo de alta generación y de próxima generación.
date: 2025-06-18
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
* [Encuesta de rendimiento del compilador Rust 2025](https://blog.rust-lang.org/2025/06/16/rust-compiler-performance-survey-2025/)
* [Actualización del Consejo de Liderazgo de junio de 2025 | Dentro del blog de Rust](https://blog.rust-lang.org/inside-rust/2025/06/11/leadership-council-update/)

### Fundación
* [Anuncio de la línea de altavoces RustConf 2025](https://rustfoundation.org/media/announcing-the-rustconf-2025-speaker-lineup/)

### Boletines
* [Este mes en Rust OSDev: mayo de 2025](https://rust-osdev.com/this-month/2025-05/)

### Actualizaciones de proyectos/herramientas
* [registro de cambios de rust-analyzer #290](https://rust-analyzer.github.io/thisweek/2025/06/16/changelog-290.html)
* [Linebender en mayo de 2025](https://linebender.org/blog/tmil-17/)
* [La caja bzip2 cambia de C a 100% Rust](https://trifectatech.org/blog/bzip2-crate-switches-from-c-to-rust/)
* [Hypershell: Un DSL de nivel de tipo para shell-scripting en Rust](https://contextgeneric.dev/blog/hypershell-release/)
* [Slint 1.12 lanzado con soporte WGPU, puerto iOS e integración de variables Figma](https://slint.dev/blog/slint-1.12-released)
* [Glues v0.7.0 – Aplicación para tomar notas de TUI con un nuevo motor de temas y paletas de colores](https://github.com/gluesql/glues/releases/tag/v0.7.0)

### Observaciones/Pensamientos
* [Rust retrobootstrapping por alguna razón](https://graydon2.dreamwidth.org/317484.html)
* [La difícil situación del mal entendido ordenamiento de la memoria](https://www.grayolson.me/blog/posts/misunderstood-memory-ordering/)
* [¿No te atreves a ordenar tus campos de estructura cuando usas ? Tamaño](https://blog.veeso.dev/blog/en/dont-you-dare-to-sort-your-struct-fields-when-using-sized/)
* [audio] [Tembo con Adam Hendel](https://corrode.dev/podcast/s04e05-tembo/)
* [audio] [Rust at Work - conversación con Eli Shalom e Igal Tabachnik de Eureka Labs](https://rustacean-station.org/episode/eli-shalom-and-igal-tabachnik/)
* [video] [sans-io: meh](https://sdr-podcast.com/episodes/sans-io/)
* [video] [Guillaume Gomez - Rustdoc como caso de estudio de herramientas para desarrolladores](https://timclicks.dev/podcast/guillaume-gomez-rustdoc)
* [video] [10th Bevy Meetup - Tristan - De cero a demostración: la experiencia de un recién llegado aprendiendo Bevy](https://www.youtube.com/watch?v=_FIDuLV0ZsA)

### Tutoriales de Rust
* [Sometiendo los cálculos de escaños en el software electoral holandés a la prueba (difusa)](https://tweedegolf.nl/en/blog/156/putting-seat-calculations-in-dutch-election-software-to-the-fuzz-test)
* [Registro de datos en Rust](https://github.com/frankmcsherry/blog/blob/master/posts/2025-06-03.md)
* [video] [Conducción de una matriz de LED usando Rust integrado asíncrono - moxi Ep2](https://www.youtube.com/watch?v=uZDcWA8cCsw)

### Investigación
* [Asterinas: Un sistema operativo Framekernel basado en Rust compatible con ABI de Linux con un TCB pequeño y sólido](https://arxiv.org/abs/2506.03876)

### Miscelánea
* [Hacer que la carga de imágenes GdkPixbuf de GNOME sea más segura](https://blogs.gnome.org/sophieh/2025/06/13/making-gnomes-gdkpixbuf-image-loading-safer/)
* [Informe de empleo de mayo de 2025](https://filtra.io/rust/jobs-report/may-25)
* [Actualización del estado social de Rust 2025.06](https://rust.code-maven.com/rust-update-2025-06-17)
* [Cómo funciona el rolldown: Vinculación de símbolos, resolución CJS/ESM y análisis de exportación explicados](https://www.atriiy.dev/blog/rolldown-link-stage-symbol-linking-resolution)

## Crate de la semana

El crate de esta semana es [RobustMQ](https://github.com/robustmq/robustmq), una cola de mensajes multiprotocolo de alta generación y de próxima generación.

¡Gracias a [Yu Liu](https://users.rust-lang.org/t/crate-of-the-week/2704/1443) por la autosugerencia!

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

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 461 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-10..2025-06-17

#### Compilador

* [caché 'param_env' canonicalización](https://github.com/rust-lang/rust/pull/141451)
* [Alineación temprana: evite llamadas redundantes a 'check_id'](https://github.com/rust-lang/rust/pull/142398)
* [mover rechazo rápido al interior](https://github.com/rust-lang/rust/pull/142355)
* [use 'MixedBitSet' para el análisis de flujo de datos de préstamos en el ámbito](https://github.com/rust-lang/rust/pull/142471)
* [Miri: añadir bandera para suprimir el no determinismo de flotación](https://github.com/rust-lang/rust/pull/142337)
* [Miri: podemos usar 'mul_add' de apfloat ahora](https://github.com/rust-lang/rust/pull/142340)

#### Biblioteca

* [Estabilizar la función '"file_lock"'](https://github.com/rust-lang/rust/pull/142125)
* [Estabilizar bloqueador de teclas](https://github.com/rust-lang/rust/pull/140766)
* [add 'Vec::p eek_mut'](https://github.com/rust-lang/rust/pull/142046)
* [añadida la implementación de 'Clone' para 'ChunkBy'](https://github.com/rust-lang/rust/pull/138016)
* ['fmt::D isplay' más rápido de enteros de 128 bits, sin puntero inseguro](https://github.com/rust-lang/rust/pull/136594)
* [añadir 'bit_width' para tipos enteros sin signo](https://github.com/rust-lang/rust/pull/142328)
* [eliminar el límite de vida innecesario de la firma de 'BTreeSet::extract_if'](https://github.com/rust-lang/rust/pull/142484)

#### Carga

* [agregar completador personalizado para 'eliminación de carga<TAB>'](https://github.com/rust-lang/cargo/pull/15662)
* [resalte las palabras correctas](https://github.com/rust-lang/cargo/pull/15659)
* [refactorizar: reemplazar InternedString con Cow en IndexPackage](https://github.com/rust-lang/cargo/pull/15559)

#### Rustdoc

* [Proporcione más información en la información extraída de doctest](https://github.com/rust-lang/rust/pull/141399)
* [rustdoc\_json: reducir asignaciones](https://github.com/rust-lang/rust/pull/142335)

#### Rustfmt

* [no intentes reparar autoimportaciones inválidas](https://github.com/rust-lang/rustfmt/pull/6573)

#### Clippy

* [Optimizar la 3ª función más pesada, (81b → 10m)](https://github.com/rust-lang/rust-clippy/pull/15043)
* [Agregar pelusa para enlaces de documentos rotos](https://github.com/rust-lang/rust-clippy/pull/13696)
* [Documentos: añadir enlace a 'span_lint' en diagnostics.rs](https://github.com/rust-lang/rust-clippy/pull/15065)
* [Documentos: Hacer que los documentos de 'unbuffered_bytes' sean más consistentes](https://github.com/rust-lang/rust-clippy/pull/15019)
* [arreglar FP de 'identity_op' al encontrar 'Default::d efault()'](https://github.com/rust-lang/rust-clippy/pull/15028)
* [arreglar 'collapsible_else_if' FP en stmt compilado condicionalmente](https://github.com/rust-lang/rust-clippy/pull/14906)
* [Arreglar el pánico 'needless_doctest_main' cuando doctest no es válido](https://github.com/rust-lang/rust-clippy/pull/15052)
* [arreglar 'unit_arg' sugiere erróneamente para 'Default::d efault'](https://github.com/rust-lang/rust-clippy/pull/14881)
* [arreglar sugerencia-causas-error de 'manual_swap'](https://github.com/rust-lang/rust-clippy/pull/14978)
* [corrige 'manual_flatten' elimina el inútil if let](https://github.com/rust-lang/rust-clippy/pull/14861)
* [eliminar el pase 'ClippyCtfe'](https://github.com/rust-lang/rust-clippy/pull/14712)
* [eliminar la vida útil innecesaria](https://github.com/rust-lang/rust-clippy/pull/15040)

#### Analizador de Rust

* ['ItemTree's 'ItemVisibilities' no tiene identidad, así que deduplicar](https://github.com/rust-lang/rust-analyzer/pull/19980)
* [Agregar soporte para excluir importaciones de la búsqueda de símbolos](https://github.com/rust-lang/rust-analyzer/pull/19996)
* [Limpieza de pruebas incrementales y verificación de ejecuciones de consultas](https://github.com/rust-lang/rust-analyzer/pull/20006)
* [Agregar la revisión rápida para aumentar la visibilidad de un campo privado al diagnóstico de campo privado](https://github.com/rust-lang/rust-analyzer/pull/19945)
* [en "Rellenar brazos de coincidencia", permite a los usuarios preferir 'Self' al nombre 'enum' cuando sea posible](https://github.com/rust-lang/rust-analyzer/pull/19939)
* [inserte los paréntesis necesarios al escribir '+' en el tipo de rasgo dyn](https://github.com/rust-lang/rust-analyzer/pull/20015)
* [mostrar lo que están haciendo los metadatos de la carga en el estado](https://github.com/rust-lang/rust-analyzer/pull/20014)
* [copiar los archivos de bloqueo en el directorio de destino antes de invocar los 'metadatos de cargo'](https://github.com/rust-lang/rust-analyzer/pull/20018)
* [no forzar el descenso en derivadas para las características del IDE de goto](https://github.com/rust-lang/rust-analyzer/pull/19981)
* [Corregir la comparación de macros proc](https://github.com/rust-lang/rust-analyzer/pull/19983)
* [arreglar la finalización con algunas macros de atributos](https://github.com/rust-lang/rust-analyzer/pull/19942)
* [arreglar el manejo del servidor de macros proc de cadenas con desventajas](https://github.com/rust-lang/rust-analyzer/pull/19970)
* [ocultar sugerencias de incrustaciones de dyn para 'impl's" incompletos](https://github.com/rust-lang/rust-analyzer/pull/19973)
* [nunca haga estable el diagnóstico de discordancia de tipo, incluso cuando hay una corrección](https://github.com/rust-lang/rust-analyzer/pull/20022)
* [Recargar espacios de trabajo cuando cambien las configuraciones de carga](https://github.com/rust-lang/rust-analyzer/pull/20020)
* [Rangos de soporte con servidores de macros PROC desde antes de que cambie el ID de AST](https://github.com/rust-lang/rust-analyzer/pull/19985)
* [generar anotaciones para elementos definidos por macros si su nombre está en la entrada](https://github.com/rust-lang/rust-analyzer/pull/19990)
* [uso idiomático de la salsa para la consulta de variantes 'enum'](https://github.com/rust-lang/rust-analyzer/pull/20007)
* [mejorar las terminaciones en las condiciones de expresión if/while](https://github.com/rust-lang/rust-analyzer/pull/20023)
* [optimizar la resolución de visibilidad de 'pub(crate)' y 'pub(self)](https://github.com/rust-lang/rust-analyzer/pull/20009)
* [perf: traer de vuelta la deduplicación del árbol de elementos 'VACÍO'](https://github.com/rust-lang/rust-analyzer/pull/19991)
* [proporcionar una mejor incrementalidad cuando se cambian los elementos](https://github.com/rust-lang/rust-analyzer/pull/19837)
* [simplificar y optimizar 'ItemTree'](https://github.com/rust-lang/rust-analyzer/pull/19982)
* [convierte 'BlockId' en un '#[salsa::tracked]'](https://github.com/rust-lang/rust-analyzer/pull/19995)
* [use 'ThinVec' en 'ItemScope' en un par de lugares](https://github.com/rust-lang/rust-analyzer/pull/19992)

### Clasificación del rendimiento del compilador de Rust

Semana relativamente tranquila, con algunas mejoras en los índices de referencia aprovechando la nueva
Solucionador de rasgos.

Triaje realizado por **@kobzol**.
Rango de revisión: [c31cccb7.. 45ACF54E](https://perf.rust-lang.org/?start=c31cccb7b5cc098b1a8c1794ed38d7fdbec0ccb0&end=45acf54eea118ed27927282b5e0bfdcd80b7987c&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.3% | [0.1%, 0.5%] | 14 |
| Regresiones ❌ <br /> (secundaria) | 0.3% | [0.1%, 0.5%] | 52 |
| Mejoras ✅ <br /> (primario) | -0,5% | [-4.8%, -0.1%] | 68 |
| Mejoras ✅ <br /> (secundaria) | -4,3% | [-56.5%, -0.1%] | 85 |
| Todos ❌✅ (primarios) | -0,4% | [-4.8%, 0.5%] | 82 |

3 regresiones, 7 mejoras, 4 mixtas; 4 de ellos en rollups
51 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d40fd2e4bd715c7d350d23e0c894f97f915998b6/triage/2025/2025-06-17.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Establecer MSG\_NOSIGNAL para UnixStream](https://github.com/rust-lang/rust/pull/140005)
* [Rechazar 'extern "{abi}"'s no soportados consistentemente en todas las posiciones](https://github.com/rust-lang/rust/pull/142134)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Agregar la configuración 'http.proxy-cainfo' para certificados de proxy](https://github.com/rust-lang/cargo/pull/15374)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Macros de derivación declarativas 'macro_rules!](https://github.com/rust-lang/rfcs/pull/3698)
* [Macros de atributo declarativas 'macro_rules!](https://github.com/rust-lang/rfcs/pull/3697)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: '#[export_visibility = ...]' atributo.](https://github.com/rust-lang/rfcs/pull/3834)

## Próximos eventos

Eventos oxidados entre 2025-06-18 - 2025-07-16 🦀

### Virtual
* 18/06/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Pánicos al Rust y límites de FFI**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 19/06/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/vna66he1)
* 22/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308246353)
* 24/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361436)
* 24/06/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Construyendo Raspadores Web Eficientes: Rust vs. Python para la Ingesta de Datos**](https://www.meetup.com/women-in-rust/events/306683025)
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

### Asia
* 28/06/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de junio de 2025**](https://hasgeek.com/rustbangalore/june-2025-rustacean-meetup/)
* 02/07/2025 | Seúl, KR | [Encuentro de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Rust en Seúl**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europa
* 18/06/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/308129156)
* 19/06/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Encuentro de Rust Aarhus en Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 19/06/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 23/06/2025 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London: Rust Hack & Learn junio de 2025**](https://www.meetup.com/rust-london-user-group/events/308529202)
* 24/06/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
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
* 02/07/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
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
* 15/07/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**TUI Power: Simulación y visualización de datos de sensores con Rust**](https://www.meetup.com/london-rust-project-group/events/308434768)

### América del Norte
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**programación asíncrona en Rust usando Tokio**](https://www.meetup.com/rust-mx/events/308248260)
* 19/06/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Uso de Rust para Web Series 3: Presentaciones finales y redes sociales de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 19/06/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-20 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Lechmere Rust, 20 de junio**](https://www.meetup.com/bostonrust/events/307936242)
* 25/06/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust en el grupo de desarrolladores de Web3**](https://www.meetup.com/rust-los-angeles/events/308401269)
* 2025-06-26 | Los Ángeles (Chino Hills), CA, ESTADOS UNIDOS | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Rust en Web3**](https://lu.ma/ek8jx2r3)
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

### Oceanía
* 24/06/2025 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/307520854)
* 30/06/2025 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Reunión de junio de 2025 Mini Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/308546374)

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

> Pero después de unas semanas, se compiló y los resultados nos sorprendieron. El código fue 10 veces más rápido que nuestra implementación de Kotlin cuidadosamente ajustada, a pesar de que no se intentó hacerlo más rápido. Para poner esto en perspectiva, habíamos pasado años mejorando gradualmente la versión de Kotlin de 2,000 a 3,000 transacciones por segundo (TPS). La versión de Rust, escrita por desarrolladores de Java que eran nuevos en el lenguaje, registró 30,000 TPS.
>
> Este fue uno de esos momentos que cambia fundamentalmente tu forma de pensar. De repente, el par de semanas que pasamos aprendiendo Rust ya no parecía un gran problema, en comparación con el tiempo que nos habría llevado obtener los mismos resultados en la JVM. Dejamos de preguntarnos: "¿Deberíamos usar Rust?" y comenzamos a preguntar: "¿Dónde más podría Rust ayudarnos a resolver nuestros problemas?"

– [Dr. Werner Vogels en su blog](https://www.allthingsdistributed.com/2025/05/just-make-it-scale-an-aurora-dsql-story.html)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1697) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://this-week-in-rust.org/blog/2025/06/19/this-week-in-rust-604/)</small>
