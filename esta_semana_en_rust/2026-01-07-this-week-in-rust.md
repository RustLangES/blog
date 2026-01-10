---
title: "Esta semana en Rust #92"
number_of_week: 92
description: El crate de esta semana es kameo, un marco de actores asíncrono con abstracciones claras basadas en rasgos para actores y mensajes tipificados.
date: 2026-01-07
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *Esta Semana en Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquetanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[mándanos una solicitud de retirada](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* está desarrollado abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos pueden consultarse en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentras algún error en el número de esta semana, [por favor presenta un RP](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad Rust

<!--

Estimados colaboradores de la comunidad:
Por favor, lee README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe ser del siguiente tipo:

* [Título de la página enlazada](https://example.com/my_article)

Si añades un enlace a un contenido que no sea textual, por favor prefijadlo con '[vídeo]' o '[audio]':

* [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast)

Si no sabes qué categoría usar, siéntete libre de enviar una marca permanente de todas formas
Y simplemente pide a los editores que seleccionen la categoría.

-->

### Oficial
* [Este ciclo de desarrollo en carga: 1,93](https://blog.rust-lang.org/inside-rust/2026/01/07/this-development-cycle-in-cargo-1.93/)
* [Actualización de objetivos del proyecto — diciembre de 2025](https://blog.rust-lang.org/2026/01/05/project-goals-2025-december-update/)

### Boletines
* [Este mes en Rust OSDev: diciembre de 2025](https://rust-osdev.com/this-month/2025-12/)
* [El Rustacean Incrustado Número #62](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-62)

### Actualizaciones de proyectos/herramientas
* [Mensajería del Danubio v0.6 - Introduce el Registro de Esquemas](https://danube-docs.dev-state.com/architecture/schema_registry_architecture/)
* [Lanzamiento de Fjall 3.0: motor de almacenamiento clave-valor estructurado en log](https://fjall-rs.github.io/post/fjall-3/)

### Observaciones/Pensamientos
* [1160 marcas personales para mejorar Rust en 2025](https://kobzol.github.io/rust/rustc/2026/01/05/my-rust-contributions-in-2025.html)
* [serie] [¿Quién es el dueño del recuerdo? Parte 3: ¿Qué tamaño tiene tu tipo?](https://lukefleed.xyz/posts/who-owns-the-memory-pt3/)
* [Rust aún más seguro con Miri](https://ianwwagner.com/even-safer-rust-with-miri.html)
* [ \[uv\] OnceMap: patrón de Rust para ejecutar trabajo simultáneo exactamente una vez](https://codepointer.substack.com/p/uv-oncemap-rust-pattern-for-running)
* [Rust a escala: La gran apuesta de Scaleway para convertirse en EL hiperescalador europeo](https://filtra.io/rust/interviews/scaleway-jan-26)
### Guías de Rust
* [Introducción a la programación SIMD en Rust puro](https://kerkour.com/introduction-rust-simd)
* [Dejar de reenviar errores, empezar a diseñarlos](https://fast.github.io/blog/stop-forwarding-errors-start-designing-them/)
* [Diseñando APIs para el Pozo del Éxito](https://www.rodriguez.today/articles/designing-rusty-paseto)
* [Oxidado CDK, un experimento de infraestructura como código](https://medium.com/@sam.van.overmeire/rusty-cdk-an-infrastructure-as-code-experiment-c10ed7804a2a)
* [Objetos de rasgo Asincrónico ergonómico en Rust](https://ebadf.me/blog/ergonomic-async-trait-objects-rust)
* [vídeo] [desbloqueando carga. Hacia construcciones concurrentes de carga y caché entre espacios de trabajo](https://www.youtube.com/watch?v=0QpBeQTJjeo)
* [audio] [Netstack.FM episodio 21 — GraphQL y Rust con Tom Houlé](https://netstack.fm/#episode-21)
* [Ese ruiseñor no canta: un servidor API simulado en Rust](https://bitfieldconsulting.com/posts/mockingbird-wont-sing)
* [ES] [Patrones de diseño de GoF en Rust: ¿necesarios u opcionales?](https://codigolinea.com/patrones-de-diseno-gof-en-rust/)
* [vídeo] [Tock, un sistema operativo integrado en Rust, resumen y demo (3 vídeos en lista de reproducción)](https://www.youtube.com/watch?v=cd10qCP-ciU&list=PLOsuNMzzhUHVTdS0XZYlaN6btnM6CMyuh)

### Investigación
* [¿Qué tan seguro es el ecosistema Rust? Una inmersión profunda en crates.io](https://mr-leshiy-blog.web.app/blog/crates_io_analysis/)

## Crate de la semana

El crate de esta semana es [kameo](https://github.com/tqwewe/kameo), un marco de actores asíncrono con abstracciones claras basadas en rasgos para actores y mensajes tipificados.

¡Gracias a [edgimar](https://users.rust-lang.org/t/crate-of-the-week/2704/1513) por la sugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

* *No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing) o
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 18-01-2026 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

341 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-30..2026-01-06

#### Compilador
* [hacer que la detección de ciclos inliners sea un proceso falible](https://github.com/rust-lang/rust/pull/147361)
* [eliminar 'Span' de los segmentos de 'AtPath'](https://github.com/rust-lang/rust/pull/149790)

#### Biblioteca
* [canal 'oneshot'](https://github.com/rust-lang/rust/pull/143741)
* [añadir 'VecDeque::splice'](https://github.com/rust-lang/rust/pull/147247)
* [añadir especialización para 'deque1.prepend(deque2.drain(range)) (VecDeque::p repend' y 'extend_front)'](https://github.com/rust-lang/rust/pull/150595)
* [evitar la comprobación de índice en 'char::to_lowercase' y 'char::to_uppercase'](https://github.com/rust-lang/rust/pull/150520)
* [hacer que la especialización de 'Vec::extend' y 'VecDeque::extend_front' funcione para 'vec::IntoIter' con cualquier 'Allocador', no solo 'Global'](https://github.com/rust-lang/rust/pull/150597)
* [implementa 'TryFrom<char>' para 'usize'](https://github.com/rust-lang/rust/pull/146792)
* [mejora la actuación de 'Vec::retain_mut'](https://github.com/rust-lang/rust/pull/149784)

#### Carga
* [''Feat(informe)': añadir informe de carga reconstrucciones](https://github.com/rust-lang/cargo/pull/16456)
* ['feat(test-support)': Usar nombre de prueba para el director al ejecutar pruebas](https://github.com/rust-lang/cargo/pull/16121)
* ['fix(log)': añadir el campo 'dependencias' a 'UnitRegistered'](https://github.com/rust-lang/cargo/pull/16448)
* [cualquier script de compilación ahora puede usar 'cargo::metadata=KEY=VALUE'](https://github.com/rust-lang/cargo/pull/16436)
* [implementar bloqueo de grano fino para 'build-dir'](https://github.com/rust-lang/cargo/pull/16155)
* [refactorización: migrar algunos casos para esperar/razonar](https://github.com/rust-lang/cargo/pull/16461)

#### Clippy
* ['manual_div_ceil': Añadido comprobación de variante 'x.next_multiple_of(y) / y'](https://github.com/rust-lang/rust-clippy/pull/16221)
* ['transmuting_null': Comprobar bloques const y bloques de expresión única](https://github.com/rust-lang/rust-clippy/pull/16260)
* [no hacer sugerencias aplicables a la máquina si puede cambiar la semántica](https://github.com/rust-lang/rust-clippy/pull/16324)
* [corrige 'bool_assert_comparison' sugiere erróneamente para macros](https://github.com/rust-lang/rust-clippy/pull/16280)
* [corrige 'implicit_saturating_sub' sugiere erróneamente en int literal sin tipificar](https://github.com/rust-lang/rust-clippy/pull/16309)
* [corregir falsos negativos 'multiple_inherent_impl' para bloques impl genéricos](https://github.com/rust-lang/rust-clippy/pull/16284)
* [corregir 'needless_for_each' falso negativo cuando 'for_each' está en el expr de un bloque](https://github.com/rust-lang/rust-clippy/pull/16295)
* [corrige 'new_without_default' falla la cláusula de dónde en 'new'](https://github.com/rust-lang/rust-clippy/pull/16268)
* [corregir 'redundant_pattern_matching' falla ')' en el alcance de sugerencia](https://github.com/rust-lang/rust-clippy/pull/16084)
* [corregir macros 'cmp_owned' mal desordenadas](https://github.com/rust-lang/rust-clippy/pull/16331)
* [mueve 'multiple_bound_locations' a estilo](https://github.com/rust-lang/rust-clippy/pull/16302)

#### Analizador de Rust
* [añadir el prefijo inútil 'try_into_' para 'suggest_name'](https://github.com/rust-lang/rust-analyzer/pull/21361)
* [permite encontrar referencias en comentarios de documentos](https://github.com/rust-lang/rust-analyzer/pull/21376)
* [añadir el atributo '#[rust_analyzer::macro_style()]' para controlar el estilo de enlace de completación de la macro](https://github.com/rust-lang/rust-analyzer/pull/21370)
* [añadir enlaces de ubicación para pistas genéricas de tipo de parámetro](https://github.com/rust-lang/rust-analyzer/pull/21393)
* [corregir pista de dinano incorrecta en 'impl Rasgo para'](https://github.com/rust-lang/rust-analyzer/pull/21375)
* [corregir texto fuente](https://github.com/rust-lang/rust-analyzer/pull/21397)
* [no dispares pelusa 'non_camel_case_types' para estructuras/enums marcados con 'repr(C)'](https://github.com/rust-lang/rust-analyzer/pull/21374)
* [tienen una consulta 'upvars_mentioned()' que solo calcula qué upvars captura un cierre](https://github.com/rust-lang/rust-analyzer/pull/21367)
* [suprimir falso positivo faltante de diagnosme de ítem asociado en especialización](https://github.com/rust-lang/rust-analyzer/pull/21403)
* [implementa 'Span::line()' y 'Span::column()' para el servidor proc-macro](https://github.com/rust-lang/rust-analyzer/pull/21405)
* [migrar 'move_arm_cond_to_match_guard' asiste para usar 'SyntaxEditor'](https://github.com/rust-lang/rust-analyzer/pull/21369)
* [comprimir árboles de tokens para mejor uso de memoria](https://github.com/rust-lang/rust-analyzer/pull/21363)
* [Solo calcular objetos de lenguaje para '#! [feature(lang_items)] cajas (https://github.com/rust-lang/rust-analyzer/pull/21396)
* [reutilizar asignaciones de scratch para 'try_evaluate_obligations'](https://github.com/rust-lang/rust-analyzer/pull/21407)
* [preasignar almacenes internos con 64kb de datos / 1024 elementos](https://github.com/rust-lang/rust-analyzer/pull/21390)
* [proc-macro-srv: archivo de soporte y 'local_file' mediante callbacks bidireccionales](https://github.com/rust-lang/rust-analyzer/pull/21377)

### Triaje de rendimiento del compilador Rust

No se fusionaron muchos registros permanentes, ya que seguía siendo mayormente una semana festiva. [#149681](https://github.com/rust-lang/rust/pull/149681) causó pequeñas regresiones en todos los aspectos, esto está pendiente de investigación.

Triaje hecho por **@kobzol**.
Rango de revisión: [112a2742.. 7c04f5d2](https://perf.rust-lang.org/?start=112a274275d77ebc2b892f056a1e2fad141f4f08&end=7c04f5d216b5dcfff0a55fc20327a1c519004699&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,1%, 1,4%] | 146 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,0%, 3,5%] | 91 |
| Mejoras ✅ <br /> (primaria) | -3,1% | [-4,7%, -1,5%] | 2 |
| Mejoras ✅ <br /> (secundario) | -0,7% | [-6,4%, -0,1%] | 15 |
| Todos ❌✅ (primario) | 0,4% | [-4,7%, 1,4%] | 148 |

2 regresiones, 0 mejoras, 7 mixtas; 4 de ellos en rollups
51 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/255bac429c3cc0a39f4332d8241af2e95e6d375f/triage/2026/2026-01-06.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?
* [build-std: contexto](https://github.com/rust-lang/rfcs/pull/3873)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [refactor: eliminar el acotado Ord de BinaryHeap::new etc](https://github.com/rust-lang/rust/pull/149408)
* [regresión: "el parámetro tipo 'T' puede no vivir lo suficiente" en 'offset_of!'](https://github.com/rust-lang/rust/issues/150465)
* [Problema de seguimiento para 'peekable_next_if_map'](https://github.com/rust-lang/rust/issues/143702)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Banderas de código de pelusa sin soporte](https://github.com/rust-lang/compiler-team/issues/957)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Privacidad de implementación de rasgos con permisos](https://github.com/rust-lang/rfcs/pull/3903)

## Próximos eventos

Eventos Rusty entre el 07-01-2026 - el 04-02-2026 🦀

### Virtual
* 07-01-2026 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/4p6rxjc5)
* 07-01-2026 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/xov10pef)
* 2026-01-13 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Contribución al proyecto Live Open Source Rust**](https://www.meetup.com/code-mavens/events/312641560/)
* 2026-01-15 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-15 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-18 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Contribución al proyecto Live Open Source Rust**](https://www.meetup.com/code-mavens/events/312710291/)
* 2026-01-20 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/e2ea7hxo)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-27 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/9h9n0dau)
* 2026-01-29 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Encuentro de Bevy #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #2 - Procesos oxidados, límites de memoria y cápsulas básicas**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312187422/)

### Asia
* 07-01-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**Rust en persona enero de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)
* 2026-01-08 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312645994/)
* 2026-01-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #12**](https://www.meetup.com/rustdelhi/events/312584516/)

### Europa
* 07-01-2026 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 2026-01-08 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-14 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-16 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café de día)**](https://www.meetup.com/rust-and-friends/events/312662987/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por confirmar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #82**](https://www.meetup.com/rust-paris/events/312364675/)
* 2026-01-21 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 26-01-2026 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Encuentro de Rust #17**](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-28 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - enero 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-02-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Más allá del código: Diseñando servicios que resisten la prueba del tiempo**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)

### Norteamérica
* 2026-01-08 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Trayendo el data lake al navegador con WASM**](https://www.meetup.com/utah-rust/events/312565472/)
* 2026-01-08 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-08 | Portland, OR, EE. UU. [PDXRust](https://www.meetup.com/pdxrust)
    * [**Hoty: Un mapeo objeto-relacional fácil de usar para el Rust**](https://www.meetup.com/pdxrust/events/312694009/)
* 2026-01-08 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust Enero Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/312675582/)
* 2026-01-10 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Central Cambridge Rust, 10 de enero**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-13 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Panel de Rust NYC: Agentes de codificación de IA en Rust: Qué funciona, qué se rompe**](https://www.meetup.com/rust-nyc/events/312632598/)
* 2026-01-14 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/312722481/)
* 2026-01-15 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust Común de Boston, 17 de enero**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-17 | Herndon, VA, EE. UU. | [NoVaLUG](https://mobilizon.us/@novalug)
    * [**Reunión mensual - Enfadar a Brian Lunduke, programa en Rust**](https://mobilizon.us/events/140c5c7c-01f3-4aaa-b218-58289c6b4449)
* 2026-01-20 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-22 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Encuentro nocturno de Boston Rust con Jiff, 22 de enero**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Davis Square Rust, 24 de enero**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Ángeles: Construyendo reemplazos de Git-LFS en Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Programación Rust 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust del Noreste, 31 de enero**](https://www.meetup.com/bostonrust/events/312483767/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> me parece increíble que usando Rust y Miri esté utilizando herramientas que están al borde de la investigación fundamental en lenguajes de programación. Herramientas prácticamente útiles que cualquiera pueda usar, no experimentos de código arcano que se comparten entre académicos.

– [ZiCog sobre usuarios de Rust](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1744)

¡Gracias a [Kyllingene](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1745) por la sugerencia!

[¡Por favor, enviad citas y votad para la semana que viene!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

Esta semana en el Rust está editado por:

* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [Marianne Goldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

*El alojamiento de la lista de correo está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1q6zd6n/this_week_in_rust_633/)</small>

