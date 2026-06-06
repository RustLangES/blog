---
title: "Esta semana en Rust #112"
number_of_week: 112
description: El crate de esta semana es remyx, un marco para construir TUIs sobre Ratatui.
date: 2026-06-03
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

### Fundación
* [Ayuda a financiar a las personas que construyen Rust - La Fundación Rust](https://rustfoundation.org/media/help-fund-the-people-who-build-rust/)

### Oficial

* [Lanzamiento del Fondo de Mantenedores de la Fundación Rust](https://blog.rust-lang.org/2026/06/02/launching-the-rust-foundation-maintainers-fund)

### Actualizaciones de proyectos/herramientas

* [Un año de Roto, el lenguaje de scripting compilado para Rust](https://blog.nlnetlabs.nl/one-year-of-roto-the-compiled-scripting-language-for-rust/)
* [XA11Y: automatización de escritorio multiplataforma mediante APIs nativas de accesibilidad](https://crowecawcaw.github.io/general/2026/05/30/accessibility-for-computer-use.html)
* [¡halloy 2026.7 - ahora soporta respuesta IRCv3, redactores, metadatos, modo bot y más!](https://github.com/squidowl/halloy/releases/tag/2026.7)
* [Creando un visor nativo de markdown para documentos generados por IA con Rust y WebView](https://vorojar.github.io/md-preview/rust-webview-ai-docs.html)
* [BPF en la era agente](https://lwn.net/SubscriberLink/1075067/6e0bbea2010794b8/)
* [Hoja de ruta gRPC-Rust](https://grpc.io/blog/grpc-rust-roadmap/)
* [Anunciando a Zstandard en Rust](https://trifectatech.org/blog/announcing-zstandard-in-rust/)

### Observaciones/Pensamientos

* [Nueve formas de hacer herencia en Rust, un lenguaje sin herencia](https://medium.com/@carlmkadie/nine-ways-to-do-inheritance-in-rust-a-language-without-inheritance-14825bf1e215?v=1)
* [Async Rust: profundización en la planificación cooperativa y la arquitectura de Tokio](https://kerkour.com/async-rust-cooperative-scheduling-tokio)
* [La seguridad de la memoria es cuestión de vida o muerte | joshlf.com](https://joshlf.com/posts/memory-safety-life-and-death/)

### Guías de Rust

* [ZK responde sarcasmo a los desarrolladores de Rust: R1CS vs Plonkish vs AIR](https://rustarians.com/r1cs-plonkish-air)
* [Aprende cierres de Rust construyendo un pequeño linter basado en reglas](https://blog.sheerluck.dev/posts/learn-rust-closures-by-building-a-tiny-linter/)
* [Aprende estados de Bebida, Temporizadores y Movimiento de Cuadrícula construyendo Serpiente](https://blog.sheerluck.dev/posts/learn-bevy-states-timers-by-building-snake/)
* [vídeo] [Lección 8 de RustCuriose: Genéricos y monomorfización](https://www.youtube.com/watch?v=WTmjbKk1EIk)
* [Un enfoque grammatical primero para los combinadores de analizadores en Rust](https://blog.arnedebo.com/posts/a-grammar-first-approach-to-parser-combinators/)

### Investigación

* [Contrafactuales a través de la Mónada Causal en Rust](https://www.deepcausality.com/blog/counterfactuals-via-the-causal-monad/)

## Crate de la semana

El crate de esta semana es [remyx](https://github.com/manuelgdlvh/remyx), un marco para construir TUIs sobre Ratatui.

¡Gracias a [Manuel García de la Vega](https://users.rust-lang.org/t/crate-of-the-week/2704/1608) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
* [Vista previa MD - Vista previa del MD del paquete para barricas caseras](https://github.com/vorojar/md-preview/issues/19)
* [OpenSlate - Endpoint de comprobación de salud de prueba](https://github.com/MrSheerluck/openslate/issues/7)
* [OpenSlate - Endpoint de inicio de sesión de prueba](https://github.com/MrSheerluck/openslate/issues/8)
* [OpenSlate - Endpoint CRUD de notas de prueba](https://github.com/MrSheerluck/openslate/issues/9)
* [OpenSlate - Endpoint de búsqueda de pruebas](https://github.com/MrSheerluck/openslate/issues/10)
* [OpenSlate - Punto final de preferencia de prueba](https://github.com/MrSheerluck/openslate/issues/11)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Computación Científica en Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

500 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-26..2026-06-02

#### Compilador
* [expansión caída asíncrona durante la elaboración de caídas](https://github.com/rust-lang/rust/pull/156649)
* ['offload_kernel' macro expansión](https://github.com/rust-lang/rust/pull/156642)
* ['std::offload' sharedmem](https://github.com/rust-lang/rust/pull/154835)

#### Biblioteca
* [constify métodos y funciones relacionados con iteradores](https://github.com/rust-lang/rust/pull/156390)
* [mueve 'IoSlice' y 'IoSliceMut' a 'core::io'](https://github.com/rust-lang/rust/pull/155849)
* [especializar Clon del array IntoIter](https://github.com/rust-lang/rust/pull/156634)
* [estabilizar 'Path::is_empty'](https://github.com/rust-lang/rust/pull/157065)
* [deja de necesitar una asignación para 'catch_unwind'](https://github.com/rust-lang/rust/pull/156867)

#### Carga
* ['diag': Añadir el grupo ''carga::d efault''](https://github.com/rust-lang/cargo/pull/17033)
* ['diag': Resúmenes de informes para 'unused_deps'](https://github.com/rust-lang/cargo/pull/17034)
* [añadir '--output-format=json' a cargo doc como opción inestable](https://github.com/rust-lang/cargo/pull/17025)
* [añadir edición para scripts cada vez que mutamos el manifiesto](https://github.com/rust-lang/cargo/pull/17038)

#### Rustdoc
* [evitar ICE al renderizar consts de tipo sin cuerpo](https://github.com/rust-lang/rust/pull/156851)
* [propagar correctamente CFGs para reexportaciones de globos](https://github.com/rust-lang/rust/pull/157039)
* [clasificación determinista para las insignias de 'doc_cfg'](https://github.com/rust-lang/rust/pull/156401)
* [arreglar ICE en funciones asíncronas delegadas](https://github.com/rust-lang/rust/pull/157223)
* [optimizar ordenación impl](https://github.com/rust-lang/rust/pull/157179)
* [separar las cachés para rasgos automáticos sintéticos e implicaciones de manta](https://github.com/rust-lang/rust/pull/157171)

#### Clippy
* [añadir pelusa de 'unused_async_trait_impl'](https://github.com/rust-lang/rust-clippy/pull/16244)
* [añadir nueva pelusa: 'for_unbounded_range'](https://github.com/rust-lang/rust-clippy/pull/16257)
* [añadió nueva pelusa para 'map_or(..., identidad)'](https://github.com/rust-lang/rust-clippy/pull/16052)
* ['redundant_pattern_match': mejorar las sugerencias](https://github.com/rust-lang/rust-clippy/pull/17116)
* ['has_arg' más rápido](https://github.com/rust-lang/rust-clippy/pull/17112)
* [pliega todos los pases tempranos de pelusa en un solo pase combinado estáticamente](https://github.com/rust-lang/rust-clippy/pull/17132)
* [pliega todos los pases tardíos de lint en un solo pase combinado estáticamente](https://github.com/rust-lang/rust-clippy/pull/17124)
* [memorizar 'first_node_in_macro' para consultas consecutivas](https://github.com/rust-lang/rust-clippy/pull/17134)
* [saltar desactivado por defecto el documento repara](https://github.com/rust-lang/rust-clippy/pull/17126)

#### Analizador de Rust
* [siempre usar crates desde sysroot en proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/22500)
* [activar la función salsa para sintaxis puente](https://github.com/rust-lang/rust-analyzer/pull/22504)
* [también considera las características de la biblioteca como internas](https://github.com/rust-lang/rust-analyzer/pull/22498)
* [no rellenes tanto 'drop()' como 'pin_drop()' en la asistencia "rellenar miembros faltantes"(https://github.com/rust-lang/rust-analyzer/pull/22508)
* [corregir variable de extracción en el rango de reemplazo del árbol de tokens](https://github.com/rust-lang/rust-analyzer/pull/22447)
* [inferencia de bloque de puertos y bucles desde rustc](https://github.com/rust-lang/rust-analyzer/pull/22473)
* [intenta mejorar la clasificación de completación](https://github.com/rust-lang/rust-analyzer/pull/22503)
* [usa añadir deref en asignar en lugar de añadir '&mut' para valor](https://github.com/rust-lang/rust-analyzer/pull/22457)
* [Apagar procesos de activación-macro-SRV al apagarse](https://github.com/rust-lang/rust-analyzer/pull/22506)
* [eliminar el uso directo del constructor make con el editor make](https://github.com/rust-lang/rust-analyzer/pull/22477)
* [eliminar hacer de renombrar y embellecer la macro expansión](https://github.com/rust-lang/rust-analyzer/pull/22484)

### Triaje de rendimiento del compilador Rust

Esta semana hemos visto buenas victorias en todos los ámbitos gracias a la fusión de varias consultas de compilador ([#155678](https://github.com/rust-lang/rust/pull/155678)), y también mejoras sustanciales en el rendimiento de los 'documentos' gracias a
haciendo menos trabajo al ordenar implicaciones de rasgos ([#157179](https://github.com/rust-lang/rust/pull/157179)).

Triaje hecho por **@Kobzol**.
Rango de revisión: [783eb8c8.. 4804AD7E](https://perf.rust-lang.org/?start=783eb8c8682ddde0807c60ed8293670ef523794f&end=4804ad7e93e1b31f4605b7083871d0d3d85a2afe&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,1%, 0,7%] | 14 |
| Regresiones ❌ <br /> (secundario) | 0,4% | [0,1%, 0,9%] | 39 |
| Mejoras ✅ <br /> (primaria) | -0,9% | [-6,8%, -0,2%] | 111 |
| Mejoras ✅ <br /> (secundario) | -1,1% | [-2,9%, -0,1%] | 53 |
| Todos ❌✅ (primario) | -0,8% | [-6,8%, 0,7%] | 125 |

3 regresiones, 1 mejora, 2 mixtas; 4 de ellos en rollups
35 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/4a082d37cfd5006c8313e55bab306ea41f091714/triage/2026/2026-06-01.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Problema de seguimiento para 'strip_circumfix'](https://github.com/rust-lang/rust/issues/147946)
* [Problema de seguimiento para CommandExt::show_window](https://github.com/rust-lang/rust/issues/127544)
* [Problema de seguimiento para 'path_set_times'](https://github.com/rust-lang/rust/issues/147455)
* [Problema de seguimiento para 'nonzero_from_str_radix'](https://github.com/rust-lang/rust/issues/152193)
* [Problema de seguimiento para LoongArch CRC Intrinsecs](https://github.com/rust-lang/rust/issues/156908)
* [Problema de seguimiento para 'Vec::from_fn'](https://github.com/rust-lang/rust/issues/149698)
* [Añadir 'Step::forward/backward_overflowing' para habilitar optimizaciones de bucles RangeInclusive](https://github.com/rust-lang/rust/pull/155114)
* [Estabilizar 'core::rango::{legacy, RangeFull, RangeTo}'](https://github.com/rust-lang/rust/pull/156629)
* [Problema de seguimiento para box_as_ptr](https://github.com/rust-lang/rust/issues/129090)
* [Problema de seguimiento para la cuerda explicit-endian::from_utf16](https://github.com/rust-lang/rust/issues/116258)
* [Reducir el ciclo de 'código inalcanzable' después de 'todo! ()'](https://github.com/rust-lang/rust/pull/149543)
* [repr_transparent_non_zst_fields error grave](https://github.com/rust-lang/rust/pull/155299)
* [Problema de seguimiento para métodos de coma flotante algebraicos](https://github.com/rust-lang/rust/issues/136469)
* [riscv: promueve d, e y f target_features a CfgStableToggleUnstable](https://github.com/rust-lang/rust/pull/156188)
* [Problema de seguimiento para 'PathBuf::into_string'](https://github.com/rust-lang/rust/issues/156203)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Desazúcar bloquea asíncronos en HIR en lugar de MIR](https://github.com/rust-lang/compiler-team/issues/997)
* [Prueba el nuevo solucionador y la alfa de Polonio en CI](https://github.com/rust-lang/compiler-team/issues/996)
* [Añadir '-Zllvm-objetivo-objetivo' \*modificador\* para establecer directamente características objetivo a nivel LLVM, y desutilizar hacerlo con '-Ctarget-feature'](https://github.com/rust-lang/compiler-team/issues/994)
* [Establecer requisitos para windows-gnu](https://github.com/rust-lang/compiler-team/issues/993)
* [Crear un nuevo objetivo de Nivel 3: 'powerpc64le-desconocido-ninguno'](https://github.com/rust-lang/compiler-team/issues/988)
* [Añadir bandera para pasar MSRV/'package.rust-version' para uso por lints](https://github.com/rust-lang/compiler-team/issues/950)
* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Directrices del Código de Peligros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [¿Pueden alguna vez ser válidas las referencias a tipos deshabitados?](https://github.com/rust-lang/unsafe-code-guidelines/issues/413)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Reubicaciones de BTF](https://github.com/rust-lang/rfcs/pull/3966)
* ['---permite-banderas-inestables': Permitir banderas inestables en estable](https://github.com/rust-lang/rfcs/pull/3965)

## Próximos eventos

Eventos Rusty entre el 3-06-2026 y el 01-07-2026 🦀

### Virtual
* 2026-06-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-04 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Explorando FalkorDB - Aprendiendo a usar una base de datos de grafos en Rust**](https://www.meetup.com/code-mavens/events/314979560/)
* 2026-06-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-06-07 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-08 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Reflexiones de RustWeek**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315009040/)
* 2026-06-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-09 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315060947/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ekws5nr4)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: ¿Qué demonios son las mónadas - y cómo las falsificamos en Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjckbcb/)

### África
* 2026-06-09 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Oxidar con el Ejemplo - Vidas**](https://www.meetup.com/johannesburg-rust-meetup/events/315070733/)

### Europa
* 2026-06-03 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Únete en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Cracovia, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactivo: Todo es de código abierto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Planificación en tiempo real en Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla en Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-25 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396600/)

### Norteamérica
* 2026-06-04 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/314983693/)
* 2026-06-04 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Pruebas, Cobertura, Tracey y Mutaciones**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Común de Boston, 6 de junio**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de junio de Utah Rust**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314989012/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Solucionadores de restricciones basados en Rust en bocetos 2D con Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Gran Fiesta de Verano de Rust NYC**](https://www.meetup.com/rust-nyc/events/315014582/)

### Oceanía
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne junio 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### Sudamérica
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Si los errores de seguridad de memoria fueran Waldo (Wally): encontrarlos en programas C es un juego de "¿Dónde está Waldo?", y el 'inseguro' de Rust lo simplifica a "¿*este* es Waldo?"

– [kornel sobre usuarios de Rust](https://users.rust-lang.org/t/is-unsafe-rust-worse-than-c/140286/25)

¡Gracias a [Moy2010](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1776) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1twbzid/this_week_in_rust_654/)</small>
