---
title: "Esta semana en Rust #55"
number_of_week: 55
description: El crate de esta semana es graft, un motor de almacenamiento transaccional optimizado para la replicación diferida, parcial y muy consistente.
date: 2025-04-09
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

### Oficial
* [Anunciando Rust 1.86.0 | Blog de Rust](https://blog.rust-lang.org/2025/04/03/Rust-1.86.0.html)
* [Ayúdanos a crear una visión para el futuro de Rust](https://blog.rust-lang.org/2025/04/04/vision-doc-survey.html)
* [C Cambios en ABI para 'wasm32-unknown-unknown'](https://blog.rust-lang.org/2025/04/04/c-abi-changes-for-wasm32-unknown-unknown.html)
* [Actualización de los objetivos del proyecto de marzo](https://blog.rust-lang.org/2025/04/08/Project-Goals-2025-March-Update.html)
* [Actualización del Director del Proyecto de marzo de 2025](https://blog.rust-lang.org/inside-rust/2025/04/03/project-director-update.html)
### Boletines
* [Este mes en Rust OSDev: marzo de 2025](https://rust-osdev.com/this-month/2025-03/)

### Actualizaciones de proyectos/herramientas
* [Informe de progreso de grafito (Q4 2024)](https://graphite.rs/blog/graphite-progress-report-q4-2024/)
* [Este mes en Redox - Marzo 2025](https://www.redox-os.org/news/this-month-250331/)
* [zxc: Un proxy mitm basado en terminal escrito en rust con Tmux y Vim como interfaz de usuario](https://hail-hydrant.github.io/zxc/)
* [rustc_codegen_gcc: Informe de Progreso #35](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-35)

### Observaciones/Pensamientos
* [Replicación de cambios de estado a través de barreras lingüísticas con Rust, UniFFI y macros proc](https://www.tantaluspath.com/tech/rust_to_swift_state_syncing/)
* [Pruebas de simulación determinista para Rust asíncrono](https://s2.dev/blog/dst)
* [Pensar como un compilador: lugares y valores en Rust](https://steveklabnik.com/writing/thinking-like-a-compiler-places-and-values-in-rust/)
* [Las cosas se desmoronan](https://bitfieldconsulting.com/posts/things-fall-apart)
* [Reflexiones sintácticas sobre los tipos de vista](https://blog.yoshuawuyts.com/syntactic-musings-on-view-types/)

### Tutoriales de Rust
* [Construyendo un motor de búsqueda desde cero, en Rust: parte 3](https://jdrouet.github.io/posts/202503231000-search-engine-part-3/)
* [Trampas de la oxidación segura](https://corrode.dev/blog/pitfalls-of-safe-rust/)
* [Cómo implementar claves API criptográficamente seguras en Rust](https://kerkour.com/api-keys)

### Investigación
* [Recolección de basura para Rust: The Finalizer Frontier](https://arxiv.org/abs/2504.01841)

### Miscelánea

## Crate de la semana

El crate de esta semana es [graft](https://github.com/orbitinghail/graft), un motor de almacenamiento transaccional optimizado para la replicación diferida, parcial y muy consistente.

¡Gracias a [Carl Sverre](https://users.rust-lang.org/t/crate-of-the-week/2704/1426) por la autosugestión!

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

*Esta semana no se han presentado convocatorias para participar.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

* [**Convocatoria de oradores de Rust Conf 2025**](https://rustfoundation.org/media/rustconf-2025-call-for-talk-proposals-open/) | Cierra 2025-04-29 11:59 PM PDT | Seattle, WA, EE. UU. | 2025-09-02 - 2025-09-05

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 451 solicitudes de extracción en la última semana[fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-01..2025-04-08

#### Compilador

* [implementar 'super let'](https://github.com/rust-lang/rust/pull/139112)
* [higiene: evitar la recursividad en la decodificación del contexto sintáctico](https://github.com/rust-lang/rust/pull/139281)
* [Ajustes de consulta miscelánea](https://github.com/rust-lang/rust/pull/139234)
* [experimento de carpeta: Micro-optimizar RegionEraserVisitor](https://github.com/rust-lang/rust/pull/139292)
* [experimento de carpeta: Resolución de región monomorfizada](https://github.com/rust-lang/rust/pull/139287)

#### Biblioteca

* [añadir 'slice::align_to_uninit_mut'](https://github.com/rust-lang/rust/pull/139072)
* [optimize slice '{Chunks,Windows}::nth'](https://github.com/rust-lang/rust/pull/138562)

#### Carga

* [cargo: feat: imprimir nombres de destinos y paquetes formateados como hipervínculos de archivo](https://github.com/rust-lang/cargo/pull/15405)

#### Rustfmt

* [rustfmt: añadir '#! [característica(ergonomic_clones)]' formato](https://github.com/rust-lang/rustfmt/pull/6532)
* [rustfmt: añadir la opción 'match_arm_indent'](https://github.com/rust-lang/rustfmt/pull/6525)
* [rustfmt: extiende el soporte de 'cfg_if!' a 'cfg_match!'](https://github.com/rust-lang/rustfmt/pull/6522)

#### Clippy

* [clippy: correcciones para 'missing_asserts_for_indexing'](https://github.com/rust-lang/rust-clippy/pull/14108)

#### Analizador de Rust

* [rust-analyzer: project-model: proporcionar bandera para no deps](https://github.com/rust-lang/rust-analyzer/pull/19519)
* [rust-analyzer: auto-import: Preferir importaciones de tipos coincidentes para listas de argumentos](https://github.com/rust-lang/rust-analyzer/pull/19541)
* [rust-analyzer: ide-assists: remove 'AssistKind::None'](https://github.com/rust-lang/rust-analyzer/pull/19509)
* [Rust-analyzer: añadir más finalización sobre "impl"](https://github.com/rust-lang/rust-analyzer/pull/19447)
* [Rust-analyzer: alinear el uso de 'predeterminado' y 'nuevo' con la guía de estilo](https://github.com/rust-lang/rust-analyzer/pull/19520)
* [Rust-analyzer: no elimines referencias con más de una definición](https://github.com/rust-lang/rust-analyzer/pull/19515)
* [rust-analyzer: arreglar corchetes de color en el contexto de la cadena](https://github.com/rust-lang/rust-analyzer/pull/19514)
* [Rust-analyzer: arreglar la disminución de 'format_args' para ≥1.87](https://github.com/rust-lang/rust-analyzer/pull/19531)
* [rust-analyzer: arregla un error en la expansión MBE que surgió de la corrección incorrecta de un error anterior en MBE](https://github.com/rust-lang/rust-analyzer/pull/19501)
* [Rust-analyzer: Otro diagnóstico de falso positivo de transmisión inválida](https://github.com/rust-lang/rust-analyzer/pull/19432)

### Clasificación del rendimiento del compilador de Rust

Una semana ajetreada con muchas mejoras de rendimiento. La mayor mejora del rendimiento provino de una reversión de la regresión de la semana anterior, justo a tiempo para el lanzamiento de la versión beta. Otra gran mejora se produjo con pequeños ajustes en el sistema de consultas, lo que muestra que todavía hay oportunidades para pequeñas mejoras de rendimiento específicas en el compilador.

Triaje realizado por **@rylev**.
Rango de revisión: [2ea33b59.. E643F59F](https://perf.rust-lang.org/?start=2ea33b591050c4ca1a3752830b29112638faecf6&end=e643f59f6da3a84f43e75dea99afaa5b041ea6bf&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.8% | [0.2%, 1.9%] | 11 |
| Regresiones ❌ <br /> (secundaria) | 8.4% | [0.2%, 38.5%] | 16 |
| Mejoras ✅ <br /> (primario) | -1.0% | [-35.1%, -0.2%] | 206 |
| Mejoras ✅ <br /> (secundaria) | -1,8% | [-8.6%, -0.1%] | 155 |
| Todos ❌✅ (primarios) | -0.9% | [-35.1%, 1.9%] | 217 |

2 regresiones, 9 mejoras, 5 mixtas; 4 de ellos en rollups
48 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/28767e0a09d14829c2930a7e0c22ee73b0b0a4e7/triage/2025-04-08.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [f*::NAN: garantiza que este es un NaN tranquilo](https://github.com/rust-lang/rust/pull/139483)
* [Estabilizar cadenas en la edición de 2024](https://github.com/rust-lang/rust/pull/132833)
* [Estabilizar la función cell_update](https://github.com/rust-lang/rust/pull/134446)
* [Estabilizar '-Zdwarf-version' como '-Cdwarf-version'](https://github.com/rust-lang/rust/pull/136926)
* [indirect-const-estabilizar el 'exact_div' intrínseco](https://github.com/rust-lang/rust/pull/139163)
* [La promesa 'array::from'_fn se genera en orden de índices crecientes](https://github.com/rust-lang/rust/pull/139099)

#### Otras áreas
* *No hay artículos ingresados al Período Final de Comentarios esta semana para
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-04-09 - 2025-05-07 🦀

### Virtual
* 10/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820298)
* 2025-04-15 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170698)
* 16/04/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/306231500)
* 17/04/2025 | Virtual y presencial (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361432)
* 23/04/2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Beyond embedded - Desarrollo de sistemas operativos en Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307036053)
* 24/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820299)
* 24/04/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parte 2: ¡Las computadoras cuánticas no pueden proteger esto contra el Rust!" **](https://www.meetup.com/charlottesville-rust-meetup/events/306679733)
* 03/05/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 05/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592/)
* 07/05/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031663)

### Asia
* 2025-04-22 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust abril de 2025 en Braavos en Tel Aviv en colaboración con StarkWare**](https://www.meetup.com/rust-tlv/events/306530984)

### Europa
* 09/04/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2025**](https://lu.ma/dlvfol30)
* 09/04/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045446)
* 10/04/2025 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/306682264)
* 2025-04-15 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741632)
* 2025-04-15 | Londres, Reino Unido | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**WIR x WCC: Encontrando tu voz en la tecnología**](https://www.meetup.com/women-in-rust/events/306774769)
* 2025-04-19 | Estambul, TR | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**Rust Konf Türkiye**](https://kommunity.com/turkiye-rust-community/events/rust-konf-turkiye-91f7b3a6)
* 23/04/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Fusionando Python con Rust usando enlaces C sin procesar**](https://www.meetup.com/london-rust-project-group/events/306644439)
* 24/04/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charla en MFT Energy**](https://www.meetup.com/rust-aarhus/events/305809344)
* 24/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/306911347)
* 24/04/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Code Night**](https://www.meetup.com/rust-manchester/events/306899063)
* 25/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/306911357)
* 2025-04-26 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #11**](https://www.meetup.com/stockholm-rust/events/307164617)
* 29/04/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #76**](https://www.meetup.com/rust-paris/events/306952202)
* 30/04/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Operador de Kubernetes en Rust**](https://www.meetup.com/rust-rhein-main/events/306772838)
* 01/05/2025 | Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Hackers Hike 0x0**](https://www.meetup.com/rust-noris/events/305522254)
* 06/05/2025 - 07/05/2025 | París, FR | [WebAssembly y Rust Meetup](https://www.meetup.com/wasm-rust-meetup/)
    * [**GOSIM AI París 2025**](https://www.meetup.com/wasm-rust-meetup/events/306530699/)
* 07/05/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 07/05/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)

### América del Norte
* 09/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust ATX en el brazo**](https://www.meetup.com/rust-atx/events/307039679)
* 10/04/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**TetaNES: Una vacuna contra la roya: sin aguja, solo el verificador de préstamos**](https://www.meetup.com/pdxrust/events/306714209)
* 14/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Coolidge Corner Brookline, 14 de abril**](https://www.meetup.com/bostonrust/events/306844334)
* 2025-04-15 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307181732)
* 17/04/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhcgbwb)
* 17/04/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 1 : Por qué HTMX es malo**](https://www.meetup.com/music-city-rust-developers/events/304333092)
* 17/04/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/307089940)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcgbfc)
* 24/04/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**3ª 3ª VEZ ¡DIOS MÍO SÍ!**](https://www.meetup.com/rust-atl/events/307152133)
* 25/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Ball Square Rust, 25 de abril**](https://www.meetup.com/bostonrust/events/306844343)
* 01/05/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Reflexiones del Proyecto Capstone SIUE sobre el Rust**](https://www.meetup.com/stl-rust/events/304026152)
* 03/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Boston Common Rust, 3 de mayo**](https://www.meetup.com/bostonrust/events/306845368)

### Oceanía
* 09/04/2025 | Sídney, NS, AU | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**Cangrejo 🦀 X 🕳️🐇 **](https://www.meetup.com/rust-sydney/events/306978026)
* 14/04/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306841248)
* 2025-04-22 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de abril**](https://www.meetup.com/rust-canberra/events/306425557)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> El momento en que congelé Doctest con un bucle en un comentario.

– [/u/HaMMeReD describiendo su primer momento Rust Whoa! en /r/rust](https://www.reddit.com/r/rust/comments/1jrmuj6/what_is_your_woah_moment_in_rust/mlg9ns4/)

A pesar de la falta de sugerencias, llogiq está satisfecho con su elección.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1jvgecc/this_week_in_rust_594/)</small>
