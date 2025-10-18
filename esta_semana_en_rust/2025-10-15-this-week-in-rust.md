---
title: "Esta semana en Rust #80"
number_of_week: 80
description: El crate de esta semana es mitsein, una biblioteca de colecciones no vacías.
date: 2025-10-15
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
* [Actualización de la gestión del programa - septiembre de 2025](https://blog.rust-lang.org/inside-rust/2025/10/14/program-management-update-2025-09/)

### Fundación
* [Actualizaciones de la Junta de la Fundación Rust](https://rustfoundation.org/media/introducing-the-rust-foundations-newest-project-directors-october-2025/)

### Boletines
* [Este mes en Rust OSDev: septiembre de 2025](https://rust-osdev.com/this-month/2025-09/)

### Actualizaciones de proyectos/herramientas
* [Gccrs después de libcore](https://lwn.net/SubscriberLink/1040197/0733825193ca1f04/)
* [Una nueva API para spinlocks con reconocimiento de interrupciones](https://lwn.net/SubscriberLink/1039374/664ea18bb8a3c1a8/)
* [Anuncio de Heave 0.1.0: ¡una biblioteca de Rust de modelo de datos EAV que puede persistir estructuras personalizadas en una base de datos SQLite sin fricción alguna!](https://www.rustydonkey.dev/blog/2025.10.08_introduction_to_heave/)
* [GuardianDB 0.10.15 - Introducción: el nodo iroh incrustado](https://www.willsearch.com.br/?page_id=19)
* [Linebender en septiembre de 2025](https://linebender.org/blog/tmil-21/)
* [egui 0.33.0 - 'egui::P lugin', mejor kerning, visor kitdiff](https://github.com/emilk/egui/releases/tag/0.33.0)
* [Preparación de Slint para el escritorio](https://slint.dev/blog/making-slint-desktop-ready)
* [Física aviar 0.4](https://joonaa.dev/blog/09/avian-0-4)
* [rustc_codegen_gcc: Informe de progreso # 38](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-38)
* [Lanzamiento de CGP v0.5.0: despachadores automáticos, mejoras de tipos de datos extensibles, computación monádica, emulación RTN, serde modular y más](https://contextgeneric.dev/blog/v0-5-0-release/)
* [Diesel-Async 0.7](https://blog.weiznich.de/blog/diesel-async-0-7/)

### Observaciones/Pensamientos
* [Necesitamos (al menos) mangos ergonómicos y explícitos](https://smallcultfollowing.com/babysteps/blog/2025/10/13/ergonomic-explicit-handles/)
* [Entrar en pánico o no entrar en pánico](https://www.ncameron.org/blog/to-panic-or-not-to-panic/)
* [Estado de tipo recursivo en Rust](https://www.jessestuart.ca/posts/2025-10-10-recursive-type-state-in-rust/)
* [Charla sobre la seguridad de la memoria en la Conferencia ONE](https://tweedegolf.nl/en/blog/195/talk-about-memory-safety-at-one-conference)
* [Un poco de limitación del rasgo de Rust](https://www.thecodedmessage.com/posts/rust-trait-limitation/)
* [Efectos en Rust (y Koka)](https://aloso.foo/blog/2025-10-10-effects/)
* [video] [Conferencia Oxidize 2025](https://www.youtube.com/playlist?list=PLilpJp3WAOvcn5_VDv3VIkQzniMWl_BfO)
* [video] [Rust 2025: 400K salarios, IA, defensa y verificador de préstamos — Jon Gjengset sobre Rust y el futuro de la codificación](https://www.youtube.com/watch?v=nOSxuaDgl3s)
* [audio] [Netstack.FM Episodio 9 – gRPC con Lucio Franco](https://netstack.fm/#episode-9)

### Tutoriales de Rust
* [Construyendo extensiones SQLite en Rust](https://kerkour.com/sqlite-extension-rust)
* [Serie de backend de Axum: JWT con token de actualización](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-jwt-refresh-token/)
* [serie] [La guía del programador impaciente para Bevy y Rust: Capítulo 1 - Que haya un jugador](https://aibodh.com/posts/bevy-rust-game-development-chapter-1/)
* [serie] [La guía del programador impaciente para Bevy y Rust: Capítulo 2 - Que haya un mundo](https://aibodh.com/posts/bevy-rust-game-development-chapter-2/)
* [video] [Construyendo TUI integradas con Rust y Ratatui - Tokyo Rust Meetup 2025](https://www.youtube.com/watch?v=F04kQMKwrwQ)
* [video] [Construir con Naz: eliminar por uno los errores con el diseño del sistema de tipo Rust](https://www.youtube.com/watch?v=IkCUhGAyS9U)

### Investigación
* [Artículos sobre la traducción de C a Rust](https://hjaem.info/c-to-rust-papers)

### Miscelánea
* [Fondo de Mantenedores de Rust - RustNL](https://rustnl.org/fund/)
* [🦀 Paquete de 50+ Gratis (licencia CC0) Ilustraciones de Ferris con diferentes emociones, poses y situaciones en PNG y SVG 🦀 ](https://github.com/MariaLetta/free-ferris-pack)

## Crate de la semana

El crate de esta semana es [mitsein](https://github.com/olson-sean-k/mitsein), una biblioteca de colecciones no vacías.

¡Gracias a [Nik Revenco](https://users.rust-lang.org/t/crate-of-the-week/2704/1481) por la sugerencia!

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

* [Diesel - Ver soporte - Muéstrame tus definiciones de vista](https://github.com/diesel-rs/diesel/discussions/4805)
* [Diesel: agregue '#[diagnostic::d o_not_recommend]' a 'impl AsExpression for T: Expression'](https://github.com/diesel-rs/diesel/issues/4760)
* [Diésel - Mejorar la documentación para los modos de carga de Postgres](https://github.com/diesel-rs/diesel/issues/4764)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp) | CFP cierra 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

420 solicitudes de extracción se [fusionaron en la última semana] [fusionaron]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-07..2025-10-14

#### Compilador
* [agregar un nuevo objetivo 'wasm32-wasip3' a Rust](https://github.com/rust-lang/rust/pull/147205)
* [Denominación global de variables: evaluar constantes de forma diferida](https://github.com/rust-lang/rust/pull/146869)
* [Ajustes de 'DepNodeColor'](https://github.com/rust-lang/rust/pull/147423)
* [realizar InstSimplify antes de ReferencePropagation](https://github.com/rust-lang/rust/pull/147483)
* [refactorizar AddCallGuards en dos bucles](https://github.com/rust-lang/rust/pull/147477)
* [división 'overlapping_{inherent,trait}_impls'](https://github.com/rust-lang/rust/pull/147502)
* [valide mejor 'CopyForDeref' y 'DerefTemps' y elimínelos del MIR en tiempo de ejecución](https://github.com/rust-lang/rust/pull/145513)

#### Biblioteca
* [mover más código a 'RawVec::finish_grow'](https://github.com/rust-lang/rust/pull/147124)
* [transfiera la implementación de los intrínsecos de SIMD de Miri a const-eval](https://github.com/rust-lang/rust/pull/146568)
* [especialízate en 'slice::fill' para usar memset cuando sea posible](https://github.com/rust-lang/rust/pull/147457)
* [estabilizar 'NonZero<u*>::d iv_ceil'](https://github.com/rust-lang/rust/pull/147562)

#### Carga
* [Reorganizar el diseño del directorio de compilación](https://github.com/rust-lang/cargo/pull/15947)
* [agregar: Informar un error de origen faltante para las dependencias del espacio de trabajo](https://github.com/rust-lang/cargo/pull/16063)
* [script: Predeterminado bin.name a package.name](https://github.com/rust-lang/cargo/pull/16064)
* [script: Almacenar archivos de bloqueo de script de carga en build-dir](https://github.com/rust-lang/cargo/pull/16087)
* [árbol: Cambiar de '--depth public' a '--edges public'](https://github.com/rust-lang/cargo/pull/16081)
* [permitir que rustfix 'unused_variables' pelusa](https://github.com/rust-lang/cargo/pull/16082)
* [corregir la regresión que se tragó las explicaciones de diagnóstico de JSON](https://github.com/rust-lang/cargo/pull/16075)

#### Rustdoc
* [rustdoc: No serialice y deserialice datos que no pasan por el cable](https://github.com/rust-lang/rust/pull/147402)
* [rustdoc: una pequeña mejora de rendimiento: solo asigne una nueva cadena si hay líneas de fondo de DOS en highlight.rs](https://github.com/rust-lang/rust/pull/147443)

#### Clippy
* ['multiple_inherent_impl': Se ha añadido la opción de configuración para apuntar a un ámbito específico](https://github.com/rust-lang/rust-clippy/pull/15843)
* ['zero_repeat_side_effects': no sugiera aparatos ortopédicos innecesarios alrededor de los STMT](https://github.com/rust-lang/rust-clippy/pull/15826)
* ['clone_on_ref_ptr': solo nombra el tipo genérico si es posible](https://github.com/rust-lang/rust-clippy/pull/15740)
* ['collapsible_match': excluir los modos de enlace de las sugerencias de patrones de campo 'struct'](https://github.com/rust-lang/rust-clippy/pull/15608)
* ['zero_repeat_side_effects': no sugiera tipos no sugestionables](https://github.com/rust-lang/rust-clippy/pull/15815)
* ['legacy_numeric_constants': agregar verificación de ctxt para macro interna](https://github.com/rust-lang/rust-clippy/pull/15816)
* ['manual_unwrap_or': corregir el caso extremo de falso positivo](https://github.com/rust-lang/rust-clippy/pull/15812)
* ['get_unwrap': evite llamar a 'is_type_diagnostic_item' varias veces](https://github.com/rust-lang/rust-clippy/pull/15847)
* [agregar 'replace_box' pelusa](https://github.com/rust-lang/rust-clippy/pull/14953)
* [agregar pelusa 'unnecessary_option_map_or_else'](https://github.com/rust-lang/rust-clippy/pull/14662)
* [verifique las estructuras y enumeraciones para 'use_self'](https://github.com/rust-lang/rust-clippy/pull/15566)
* [corregir 'needless_continue' falso positivo cuando el tipo de coincidencia no es unidad o nunca](https://github.com/rust-lang/rust-clippy/pull/15547)
* [honrar los atributos 'allow'/'expect' en los nodos ADT e 'impl Clone'](https://github.com/rust-lang/rust-clippy/pull/15849)

#### Analizador de Rust
* [agregar ide-assist: generar rasgo general impl](https://github.com/rust-lang/rust-analyzer/pull/19771)
* [agregar autocompleciones de parámetros para rasgo assoc fn](https://github.com/rust-lang/rust-analyzer/pull/20812)
* [compilar rust-analyzer con '--target' para install/pgo xtask](https://github.com/rust-lang/rust-analyzer/pull/20804)
* [corregir que la finalización .let no funcione para let-chain](https://github.com/rust-lang/rust-analyzer/pull/20526)
* [corregir el tipo de retorno forzado de cierre para 'add_return_type'](https://github.com/rust-lang/rust-analyzer/pull/20816)
* [corregir análisis de finalización de cierre vacío](https://github.com/rust-lang/rust-analyzer/pull/20824)
* [Se corrige que c-str y byte-str no son aplicables para 'raw_string'](https://github.com/rust-lang/rust-analyzer/pull/20788)
* [corrección no aplicable en param en let-stmt para 'add_explicit_type'](https://github.com/rust-lang/rust-analyzer/pull/20817)
* [mejorar el error de análisis para 'static' y 'const'](https://github.com/rust-lang/rust-analyzer/pull/20805)
* [reemplace los valores predeterminados de la tarea '--show-output' con '--nocapture'](https://github.com/rust-lang/rust-analyzer/pull/20803)

### Triaje de rendimiento del compilador de Rust

Esta semana se vieron pequeñas ganancias en todos los ámbitos de algunas microoptimizaciones de la consulta incremental
sistema ([#147423](https://github.com/rust-lang/rust/pull/147423)). También ha habido un par de
Regresiones. [#142390](https://github.com/rust-lang/rust/pull/142390) introdujeron regresiones de 'cheque'
se construye en todos los ámbitos. La regresión más grande (18 %) proviene de una compilación de opción incremental de una base de datos secundaria
prueba de esfuerzo artificial, por lo que la consideramos aceptable.

Triaje realizado por **@kobzol**.

Rango de revisión: [1a3cdd34.. 956f47c3](https://perf.rust-lang.org/?start=1a3cdd34629306fa67624eaa60d73687e7fcf855&end=956f47c32f1bd97b22cd702d7ccf78f0f0d42c34&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,7% | [0,1%, 2,0%] | 65 |
| Regresiones ❌ <br /> (secundaria) | 0,8% | [0,1%, 18,6%] | 65 |
| Mejoras ✅ <br /> (primaria) | -0,6% | [-1,6%, -0,1%] | 119 |
| Mejoras ✅ <br /> (secundario) | -0,4% | [-1,6%, -0,1%] | 76 |
| Todos ❌✅ (primarios) | -0,1% | [-1,6%, 2,0%] | Artículo 184 |

2 regresiones, 7 mejoras, 3 mixtas; 3 de ellos en rollups
35 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/b4b30e719c7083141669f79edfdf20e685cf918f/triage/2025/2025-10-13.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [iter repeat: pánico al final](https://github.com/rust-lang/rust/pull/147258)
* [Problema de seguimiento (toma 2) para 'more_float_constants'](https://github.com/rust-lang/rust/issues/146939)
* [Extensión temporal de la vida útil de los bloques](https://github.com/rust-lang/rust/pull/146098)
* [Validez de bits del documento MaybeUninit](https://github.com/rust-lang/rust/pull/140463)
* [unused_must_use: No advertir en 'Resultado<(), Deshabitado>' o 'ControlFlow<Deshabitado, ()>'](https://github.com/rust-lang/rust/pull/147382)
* [Permitir pasar la metavariable 'expr' a 'cfg'](https://github.com/rust-lang/rust/pull/146961)
* [Eliminar el código actual para incrustar argumentos de línea de comandos en PDB](https://github.com/rust-lang/rust/pull/147022)
* ['-Znext-solver' instanciar el enlazador de predicados sin recanonicalizar el objetivo](https://github.com/rust-lang/rust/pull/146725)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [¿Deberían eliminarse los planos de construcción?](https://github.com/rust-lang/cargo/issues/7614)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Pasar punteros a 'const' en el ensamblador](https://github.com/rust-lang/rfcs/pull/3848)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)

* [Delegar el gasto de dinero del GSoC al equipo de tutoría t](https://github.com/rust-lang/leadership-council/issues/232)

*Ningún artículo entró en el período de comentarios finales esta semana para
  [Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Rusty Eventos entre 2025-10-15 - 2025-11-12 🦀

### Virtual
* 2025-10-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**indielinks**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk/Cracovia/Breslavia, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/) | [Espejo de Cracovia](https://www.meetup.com/stacja-it-krakow/events/310935157/) | [Espejo de Wroclaw](https://www.meetup.com/stacja-it-wroclaw/events/310935159/)
* 2025-10-19 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-22 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-22 | Virtual (Buenos Aires, AR) | [[Net-Baires] Comunidad de .NET en Buenos Aires](https://www.meetup.com/es-ES/net-baires/)
    * [**Rust para devs .NET | Standup comunitario #10**](https://www.meetup.com/es-ES/net-baires/events/311365783/)
* 2025-10-23 | Híbrido (Seattle/Bellevue, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
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
* 2025-11-06 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación / Weekly coding session**](https://luma.com/xkd84gfz)
* 2025-11-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-11 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (Londres, GB) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068632/)

### Asia
* 2025-10-20 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**En persona Rust octubre de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europa
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-23 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/311501254/)
* 2025-10-24 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/311501249/)
* 2025-10-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-29 | Dortmund, DE | [Rust, Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Encuentro de Rust Dortmund Octubre 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-30 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #62 patrocinada por Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga (octubre de 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-02 - 2025-11-04 | Florencia, IT | [Laboratorio de Rust 2025](https://rustlab.it/)
    * [**Rustlab 2025**](https://rustlab.it/)
* 2025-11-04 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Charla de noviembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión de Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Encuentro de Rust Gdansk #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944050/)

### América del Norte
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
* 2025-10-23 | Híbrido (Seattle/Bellevue, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2025 Reunión de SRUG (Seattle Rust User Group)**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Resumen del año**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro de Rust de octubre: ¡Una presentación especial y los encuentros mensuales están de vuelta!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Porter Square, 25 de octubre **](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-29 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Análisis estático escalable: enfrentando el problema de la detención**](https://www.meetup.com/rust-nyc/events/311541108/)
* 2025-10-30 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-10-30 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311273832/)
* 2025-11-01 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Chinatown, 1 de noviembre **](https://www.meetup.com/bostonrust/events/311039492/)
* 2025-11-06 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Estudiantes de SIUE sobre animaciones 3D wasm**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, 8 de noviembre **](https://www.meetup.com/bostonrust/events/311039501/)

### Oceanía
* 2025-10-22 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de octubre**](https://www.meetup.com/rust-canberra/events/311234237/)

### América del Sur
* 2025-10-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Rust Uruguay meetup de Octubre**](https://www.meetup.com/rust-uruguay/events/311475675/)
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

> Pointers son bastante difíciles.

– Tim McNamara

> Y, como su nombre lo indica, puntiagudo.

– [llogiq en LinkedIn](https://www.linkedin.com/feed/update/urn:li:activity:7381109081857724416?commentUrn=urn%3Ali%3Acomment%3A%28activity%3A7381109081857724416%2C7381113605926166528%29&dashCommentUrn=urn%3Ali%3Afsd_comment%3A%287381113605926166528%2Curn%3Ali%3Aactivity%3A7381109081857724416%29)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1721) por la autosugestión!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1o7upmn/this_week_in_rust_621/)</small>
