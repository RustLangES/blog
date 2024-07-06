---
title: "Esta semana en Rust #26"
number_of_week: 26
description: El crate de esta semana es asak, una TUI de grabación/reproducción de audio basada en terminal.
date: 2024-07-03
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (anteriormente Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Tipos: Actualización de equipo y hoja de ruta](https://blog.rust-lang.org/2024/06/26/types-team-update.html)

### Actualizaciones de proyectos/herramientas
* [Reescritura de crujidos](https://mo8it.com/blog/rustlings-rewrite/)
* [iroh 0.19.0 - Hazlo tuyo](https://iroh.computer/blog/iroh-0-19-make-it-your-own)
* [Anuncio de Polars 1.0](https://pola.rs/posts/announcing-polars-1/)
* [registro de cambios de rust-analyzer #240](https://rust-analyzer.github.io/thisweek/2024/07/01/changelog-240.html)
* [Publicado r3bl_cmdr v0.0.14](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v0014-2024-06-29)
* [Publicado r3bl_tui v0.5.6](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v056-2024-06-29)
* [Danubio - Corredor de mensajería Pub/Sub](https://dev-state.com/posts/danube_intro/)
* [what-the-time 1.0.0](https://github.com/sdball/what-the-time/releases/tag/v1.0.0)
* [Meilisearch versiones v1.9](https://blog.meilisearch.com/meilisearch-1-9/)
* [Derive-Deftly (la función de macro derivada basada en plantillas) se acerca a 1.x - Llamada para revisión/prueba](https://diziet.dreamwidth.org/18695.html)

### Observaciones/Pensamientos
* [Tipos ergonómicos autorreferenciales para Rust](https://blog.yoshuawuyts.com/self-referential-types/)
* [Más reflexiones sobre la afirmación](https://smallcultfollowing.com/babysteps/blog/2024/06/26/claim-followup-1/)
* [Cierres asíncronos](https://hackmd.io/@compiler-errors/async-closures)
* [¡Rust tiene tres tipos de referencia!](https://ssbr.xyz/blog/rust-has-three-reference-types/)
* [sans-IO: El secreto de un Rust efectivo para los servicios de red](https://www.firezone.dev/blog/sans-io)
* [audio] [OxidOS con Alexandru Radovici](https://corrode.dev/podcast/s02e05-oxidos/)

### Tutoriales de Rust
* [Resolviendo símbolos de Rust](https://blog.shrirambalaji.com/posts/resolving-rust-symbols/)
* [#! [doc = include_str! ()] con enlaces intra-doc](https://linebender.org/blog/doc-include/)
* [La configuración mínima de Rust-wasm](https://dzfrias.dev/blog/rust-wasm-minimal-setup/)
* [Build with Naz : Markdown parser en Rust y nom from r3bl_tui](https://developerlife.com/2024/06/28/md-parser-rust-from-r3bl-tui/)
* [Adición de seguridad en tiempo de compilación al SDK de AWS con el rasgo Visit de syn](https://medium.com/@sam.van.overmeire/adding-compile-time-safety-to-the-aws-sdk-with-syns-visit-trait-57bfbbac8677)
* [Adición de soporte de GraphQL a Loco con Seaography](https://www.sea-ql.org/blog/2024-07-01-graphql-support-with-loco-seaography/)
* [Patrones de Rust: No ponga ningún código en mod.rs o lib.rs archivos](https://kerkour.com/rust-patterns-don't-put-code-in-lib-mod-files)
* [serie] [Maestro de la arquitectura hexagonal en Rust (parte 3): 'Servicio', el corazón de la arquitectura hexagonal](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#service-the-heart-of-hexagonal-architecture)

### Miscelánea
* [FizzBuzz Multithreaded - sincronización con canales de encuentro](https://firedbg.sea-ql.org/blog/2024-06-30-fizzbuzz-multithread/)
* [video] [Zelda Hessler habla sobre el SDK de AWS para Rust](https://www.youtube.com/watch?v=-PTSJbUZ_Jo)

## Crate de la semana

El crate de esta semana es [asak](https://github.com/chaosprint/asak), una TUI de grabación/reproducción de audio basada en terminal.

A pesar de la lamentable falta de sugerencias esta semana, llogiq está razonablemente satisfecho con su elección.

[Por favor, envíe sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la prueba
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [diesel- Agrega soporte para operadores y métodos de rango actualmente no compatibles](https://github.com/diesel-rs/diesel/issues/4092)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (Anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

408 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-25..2024-07-02

* ['rustc_data_structures': Comprobar explícitamente la compatibilidad con atomics de 64 bits](https://github.com/rust-lang/rust/pull/127075)
* [añadir '()' a la macro 'marker_impls' para 'ConstParamTy'](https://github.com/rust-lang/rust/pull/127070)
* [añadir el archivo '.ignore' para hacer que 'config.toml' se pueda buscar en vscode](https://github.com/rust-lang/rust/pull/126876)
* [añadir más constantes, funciones y pruebas para 'f16' y 'f128'](https://github.com/rust-lang/rust/pull/126608)
* [AST: Estandarizar el orden de visita](https://github.com/rust-lang/rust/pull/126993)
* [ast: estandarizar el orden de visita para atributos e ID de nodo](https://github.com/rust-lang/rust/pull/125741)
* [contaminar automáticamente InferCtxt cuando se emiten errores](https://github.com/rust-lang/rust/pull/126996)
* [evite la hinchazón MIR en línea](https://github.com/rust-lang/rust/pull/127113)
* [evite clonar el estado de subproceso de salto cuando sea posible](https://github.com/rust-lang/rust/pull/127024)
* [Cobertura: evitar obtener información adicional de desexpansión cuando no la necesitamos](https://github.com/rust-lang/rust/pull/127157)
* [cobertura: hacer '#[cobertura(..)] ' aplicar recursivamente a funciones anidadas](https://github.com/rust-lang/rust/pull/126721)
* [de-duplicar todas las bibliotecas nativas consecutivas independientemente de sus opciones](https://github.com/rust-lang/rust/pull/126943)
* [Delegación: Refactorización de la reducción de AST](https://github.com/rust-lang/rust/pull/126947)
* [denegar el 'uso<>' para los RPITITs](https://github.com/rust-lang/rust/pull/126746)
* [detectar estructuras no utilizadas que derivaron de Default](https://github.com/rust-lang/rust/pull/126302)
* [no hacer ICE cuando sugiera desreferenciar el argumento de cierre](https://github.com/rust-lang/rust/pull/126884)
* [no ice durante el refinamiento de RPITIT la verificación de errores de resolución después de la normalización](https://github.com/rust-lang/rust/pull/126968)
* [no sugiera esperar en patrones de cierre](https://github.com/rust-lang/rust/pull/126915)
* [eliminar la distinción entre el nivel de precedencia 'PREC_POSTFIX' y 'PREC_PAREN'](https://github.com/rust-lang/rust/pull/126893)
* [habilitar la transmisión const para 'f16' y 'f128'](https://github.com/rust-lang/rust/pull/127032)
* [arreglar el código 'x86_64' que se producía para los objetivos LoongArch 'compiler_builtins'](https://github.com/rust-lang/rust/pull/127150)
* [corrige una sugerencia de error para E0121 cuando se usa el marcador de posición '_' como tipos de retorno en la firma de la función](https://github.com/rust-lang/rust/pull/127110)
* [Arreglar el mal reemplazo para la sugerencia de bloqueo externo inseguro](https://github.com/rust-lang/rust/pull/126973)
* [prefiera '(*p).clone' a 'p.clone' si la 'p' es un puntero sin procesar](https://github.com/rust-lang/rust/pull/127114)
* [ignorar 'llvm::Lld' si lld no está habilitado](https://github.com/rust-lang/rust/pull/126701)
* [implementar nuevos efectos de desazucarado](https://github.com/rust-lang/rust/pull/120639)
* [Mejorar los diagnósticos de bloqueos externos inseguros](https://github.com/rust-lang/rust/pull/127106)
* [introducir un atributo 'rustc_' para volcar todos los padres 'DefId' de un 'DefId'](https://github.com/rust-lang/rust/pull/127181)
* [menos 'maybe_whole_expr', toma 2](https://github.com/rust-lang/rust/pull/126571)
* [vamos a '#[esperar]' algunas pelusas: Estabilizar 'lint_reasons' (RFC 2383)](https://github.com/rust-lang/rust/pull/120924)
* [linker: refactorizar la interfaz para pasar argumentos al enlazador](https://github.com/rust-lang/rust/pull/126832)
* [hacer que 'feature(effects)' require '-Znext-solver'](https://github.com/rust-lang/rust/pull/127176)
* [hacer que el submódulo de carga sea opcional](https://github.com/rust-lang/rust/pull/126470)
* [Mark assoc tys live solo si el rasgo correspondiente está activo](https://github.com/rust-lang/rust/pull/126618)
* [mover el aglutinador y el análisis de polaridad a 'parse_generic_ty_bound'](https://github.com/rust-lang/rust/pull/127103)
* [no usar desplazamiento cuando no hay terminaciones con llave](https://github.com/rust-lang/rust/pull/126868)
* [patchable-function-entry: add unstable compiler flag and attribute](https://github.com/rust-lang/rust/pull/124741)
* [imprime 'TypeId' como 'u128' para 'Debug'](https://github.com/rust-lang/rust/pull/127134)
* [un-inseguro el rasgo 'StableOrd'](https://github.com/rust-lang/rust/pull/126326)
* [eliminar (obsoletos e inestables) los métodos de puntero '{a,desde}_bits'](https://github.com/rust-lang/rust/pull/127071)
* [eliminar las rutas ICE 'f16' y 'f128' de smir](https://github.com/rust-lang/rust/pull/126983)
* [eliminar más lanzamientos de 'PtrToPtr' en GVN](https://github.com/rust-lang/rust/pull/126844)
* [eliminar SeqCst innecesario en 'impl fmt::P ointer for AtomicPtr'](https://github.com/rust-lang/rust/pull/127073)
* [Resolver: modificar algunos nombres en torno a las ambigüedades de importación](https://github.com/rust-lang/rust/pull/126954)
* [muestra el tipo de atributo utilizado para el usuario cuando se encuentra que no se aplica a una variable 'estática'](https://github.com/rust-lang/rust/pull/127118)
* [detener la instancia de computación para eliminar la corrección de compatibilidad hasta que no tenga parámetros const sin sustituir](https://github.com/rust-lang/rust/pull/127068)
* [admite la obtención de 'Atributo' de elementos](https://github.com/rust-lang/rust/pull/127022)
* [Volver a cambiar 'non_local_definitions' lint a permitir por defecto](https://github.com/rust-lang/rust/pull/127015)
* [apretar 'fn_decl_span' para bloques asíncronos](https://github.com/rust-lang/rust/pull/127058)
* [Comprobación de tamaño de transmutación: tenga en cuenta correctamente la alineación](https://github.com/rust-lang/rust/pull/125740)
* [modificar 'FlatPat::new' para evitar un estado temporalmente inválido](https://github.com/rust-lang/rust/pull/126932)
* [modificar un comentario confuso en 'create_match_candidates'](https://github.com/rust-lang/rust/pull/126926)
* [unificar 'dylib' y 'bin_helpers' y crear 'shared_helpers::p arse_value_from_args'](https://github.com/rust-lang/rust/pull/127108)
* [use 'clang-format' en 'tidy' para verificar el estilo de código C++ en 'llvm-wrapper'](https://github.com/rust-lang/rust/pull/123918)
* [use full expr span para la sugerencia de devolución en error de tipo/ambigüedad](https://github.com/rust-lang/rust/pull/127129)
* [varias limpiezas de 'rustc_codegen_ssa'](https://github.com/rust-lang/rust/pull/123237)
* [varias refactorizaciones a 'rustc_interface'](https://github.com/rust-lang/rust/pull/126834)
* [Miri: 'iter_exported_symbols': también camina con estática usada en la caja local](https://github.com/rust-lang/miri/pull/3723)
* [miri: eliminar la comprobación 'frame_in_std' de GetCurrentProcessId](https://github.com/rust-lang/miri/pull/3716)
* [estabilizar 'PanicInfo::message()' y 'PanicMessage'](https://github.com/rust-lang/rust/pull/126732)
* [estabilizar 'duration_abs_diff'](https://github.com/rust-lang/rust/pull/127128)
* [marque 'Hasher::finish' como '#[must_use]'](https://github.com/rust-lang/rust/pull/127055)
* [std: creación de clave TLS separada del acceso TLS](https://github.com/rust-lang/rust/pull/126953)
* [cargo: permitir pelusa 'unexpected_builtin_cfgs' en la prueba 'user_specific_cfgs'](https://github.com/rust-lang/cargo/pull/14153)
* [Cargo: Gix: Eliminar la función de 'revisión' de Cargo](https://github.com/rust-lang/cargo/pull/14160)
* [cargo: dejar claro que 'CARGO_CFG_TARGET_FAMILY' es multivalor](https://github.com/rust-lang/cargo/pull/14165)
* [cargo: prueba: arreglar varias aserciones](https://github.com/rust-lang/cargo/pull/14167)
* [cargo: test: omitir el nombre del directorio objetivo](https://github.com/rust-lang/cargo/pull/14142)
* [cargo: prueba: reemplazar glob con llamadas explícitas no ordenadas](https://github.com/rust-lang/cargo/pull/14166)
* [rustdoc: comprueba si el desambiguador coincide con su sufijo](https://github.com/rust-lang/rust/pull/127016)
* [clippy: 'doc_lazy_continuation': línea de comentario en blanco para el espacio](https://github.com/rust-lang/rust-clippy/pull/13002)
* [clippy: agregar mensaje de error a 'manual_inspect' lint](https://github.com/rust-lang/rust-clippy/pull/13006)
* [clippy: no pelute 'assertions_on_constants' en ninguna afirmación const](https://github.com/rust-lang/rust-clippy/pull/12840)
* [clippy: arreglar el falso positivo de DevOps 'doc_markdown'](https://github.com/rust-lang/rust-clippy/pull/12995)
* [clippy: implementa una pelusa para reemplazar las rotaciones manuales de las brocas con 'rotate_left/rot...'](https://github.com/rust-lang/rust-clippy/pull/12983)
* [rust-analyzer: no normalizar 'use foo::{self}' a 'use foo'](https://github.com/rust-lang/rust-analyzer/pull/17494)
* [Rust-analyzer: agregue asistencia 'bool_to_enum' para los parámetros](https://github.com/rust-lang/rust-analyzer/pull/17467)
* [rust-analyzer: finalizaciones después de kw asíncrono](https://github.com/rust-lang/rust-analyzer/pull/17513)
* [rust-analyzer: se corrige el cálculo del alcance de la expresión cuando se está dentro de las expansiones de macros](https://github.com/rust-lang/rust-analyzer/pull/17518)
* [rust-analyzer: pasar args extra de carga al depurar](https://github.com/rust-lang/rust-analyzer/pull/17495)
* [Analizador de Rust: Mejoras en la calidad de vida de la búsqueda de términos](https://github.com/rust-lang/rust-analyzer/pull/17516)
* [rust-analyzer: use los 'ImplTraits' adecuados en 'insert_inference_vars_for_impl_trait'](https://github.com/rust-lang/rust-analyzer/pull/17505)

### Clasificación del rendimiento del compilador de Rust

# 2024-07-02 Registro de triaje

Vimos una gran regresión de un gran conjunto de puntos de referencia primarios, principalmente debido a PR #120924 ('lint_reasons' y '#[expect]') y PR #120639 (nuevos efectos
desazucarado). Aparte de esos, hay un par de PR acumulativos (#127076, #127096) con algunas regresiones que se limitaron a relativamente pocos puntos de referencia; pnkfelix no pudo aislar una inyección de PR que se puede identificar como una causa raíz (¡la asistencia externa es bienvenida!).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [c3d7fb39.. CF2DF68D](https://perf.rust-lang.org/?start=c3d7fb398569407350abe044e786bc7890c90397&end=cf2df68d1f5e56803c97d91e2b1a9f1c9923c533&absolute=false&stat=instructions%3Au)

4 regresiones, 3 mejoras, 11 mixtas; 7 de ellos en rollups
59 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/903bfae931250761eadc291a8fe34c7c52003f82/triage/2024-07-02.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:
* [RFC: Desbloquear metadatos de la función de carga](https://github.com/rust-lang/rfcs/pull/3416)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [RFC: #[derivar(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para conversiones de caracteres const](https://github.com/rust-lang/rust/issues/89259)
* [disposición: fusionar] [No hacer que la declaración nonterminals coincida con el patrón nonterminals](https://github.com/rust-lang/rust/pull/120221)
* [disposición: fusionar] [Permitir '#[denegar]' dentro de '#[prohibir]' como no-op](https://github.com/rust-lang/rust/pull/121560)
* [disposición: fusionar] [Problema de seguimiento para el atributo de función '#[cobertura]'](https://github.com/rust-lang/rust/issues/84605)
* [disposición: fusionar] [Golpea 'elided_lifetimes_in_associated_constant' para denegar](https://github.com/rust-lang/rust/pull/124211)
* [disposición: fusionar] [Problema de seguimiento para 'const_cstr_from_ptr'](https://github.com/rust-lang/rust/issues/113219)
* [disposición: fusionar] [Denegar la vida útil de las palabras clave antes de la expansión](https://github.com/rust-lang/rust/pull/126762)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hubo problemas de seguimiento de equipos lingüísticos ni relaciones públicas en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de referencias lingüísticas ni PR ingresados al período de comentarios finales esta semana.*

##### [Directrices para códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro o PR ingresados al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Cierres asíncronos](https://github.com/rust-lang/rfcs/pull/3668)

## Próximos eventos

Eventos de Rusty entre 2024-07-03 - 2024-07-31 🦀

### Virtual
* 03/07/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Construir aplicaciones web con Rust y Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 03/07/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/300328025/)
* 04/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488820/)
* 06/07/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 09/07/2024 | Virtual | [Rust para el almuerzo](https://lunch.rs/)
    * [**Julio 2024 Rust para el almuerzo**](https://lunch.rs/meetups/2024-07-09/)
* 09/07/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346976/)
* 10/07/2024 | Virtual | [Centro de Investigación Electrónica](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Investigación de la computación con el lenguaje de programación Rust**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 11/07/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 11/07/2024 | Híbrido - Virtual y Presencial (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 11/07/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 11/07/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Lectura de archivos JSON en Rust (inglés)**](https://www.meetup.com/code-mavens/events/301636580/)
* 16/07/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Desarrollo web en Rust usando Rocket - parte 2 (Inglés)**](https://www.meetup.com/code-mavens/events/301736709/)
* 17/07/2024 | Híbrido - Virtual y Presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488824/)
* 23/07/2024| Híbrido: virtual y presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
* 24/07/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Explorando los casos de uso de la API de Rust**](https://www.eventbrite.com/e/lunch-learn-exploring-rust-api-use-cases-tickets-928424531767)
* 25/07/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897865/)
* 27/07/2024 | Híbrido - Virtual y Presencial (Kiev, UA) | [Rust de UA](https://uarust.com/)
    * [**Conferencia UARust 2024**](https://uarust.com/)
* 30/07/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/301585665/)

### Asia
* 03/07/2024 | Tokio, JP | [Encuentro de Rust en Tokio](https://www.meetup.com/ja-JP/tokyo-rust-meetup/)
    * [**¡Estaba entendiendo mal el WASM!**](https://www.meetup.com/ja-JP/tokyo-rust-meetup/events/301807832/)

### Europa
* 10/07/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust - Julio**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 11/07/2024 | Praga, CZ | [Rust de Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Praga (julio de 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 16/07/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Construyendo una API REST en Rust usando Axum, SQLx y SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 16/07/2024 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Tus Bichos y Oxida Tus Motores #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)
* 23/07/2024 | Híbrido: virtual y presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
* 25/07/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288967/)
* 27/07/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8: Einführung in Machine Learning mit Rust**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)
* 30/07/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #9**](https://www.meetup.com/rust-basel/events/301459503/)

### América del Norte
* 05/07/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de la Universidad de Boston, 5 de julio**](https://www.meetup.com/bostonrust/events/301549737/)
* 11/07/2024 | Híbrido - Ciudad de México, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 11/07/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 17/07/2024 | Híbrido - Vancouver, Columbia Británica, CA | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: patrón de espera**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 18/07/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/301883176/)
* 24/07/2024 | Austin, TX, EE. UU. | [Oxidar ATC](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)
* 27/07/2024 | Híbrido - Virtual y Presencial (Kiev, UA) | [Rust de UA](https://uarust.com/)
    * [**Conferencia UARust 2024**](https://uarust.com/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> el compilador generalmente no se queja de que \[usted\] tenga un comportamiento indefinido porque no sabe que está teniendo un comportamiento indefinido.

– [nilstrieb en GitHub](https://github.com/rust-lang/rust/issues/125658#issuecomment-2135511362)

¡Gracias a [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1589) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1duvbdp/this_week_in_rust_554_this_week_in_rust/)</small>
