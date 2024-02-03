---
title: "Esta semana en Rust #17"
number_of_week: 17
description: El crate de esta semana es Apache Iceberg Rust, una implementación de Rust de un formato de tabla para grandes conjuntos de datos analíticos.
date: 2024-01-31
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Actualizaciones de proyectos/herramientas
* [API del sistema de archivos Rust y C (proyecto Rust-for-Linux)](https://lwn.net/Articles/958072/)
* [Progreso hacia un compilador de Rust basado en GCC](https://lwn.net/Articles/954787/)
* [Paleta 0.7.4](https://ogeon.github.io/2024/01/28/palette-0.7.4.html)
* [Motor de juego Fyrox 0.33](https://fyrox.rs/blog/post/fyrox-game-engine-0-33/)
* [Dos meses en Servo: mejor diseño en línea, Rust estable y más!](https://servo.org/blog/2024/01/26/two-months-in-servo/)
* [Propiedad y flujo de datos en GPUI](https://zed.dev/blog/gpui-ownership)
* [Contratos de función para Kani](https://model-checking.github.io/kani-verifier-blog/2024/01/29/function-contracts.html)
* [Slint 1.4 lanzado con aspecto adicional y API mejoradas](https://slint.dev/blog/slint-1.4-released)
* [Esta semana en Fluvio #58 - El sistema de streaming de código abierto de Fluvio se puede implementar localmente como un solo binario](https://www.fluvio.io/news/this-week-in-fluvio-0058/)
* [Lanzamiento de Quickwit 0.7: compatibilidad con la API de Elasticsearch y ganancias de rendimiento del 30%](https://quickwit.io/blog/quickwit-0.7)

### Observaciones/Pensamientos
* [Cómo comparar el código de Rust con Criterion](https://bencher.dev/learn/benchmarking/rust/criterion/)
* [Jugando con combinadores de nom y analizadores](https://andreabergia.com/blog/2024/01/playing-with-nom-and-parser-combinators/)
* [¿A dónde se va el tiempo? El problema de Rust con las compilaciones lentas](https://thenewstack.io/where-does-the-time-go-rusts-problem-with-slow-compiles/)
* [ESP32 Embedded Rust at the HAL: I2C Scanner](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-i2c-scanner)
* [Construimos cadenas X.509 para que usted no tenga que hacerlo](https://blog.trailofbits.com/2024/01/25/we-build-x-509-chains-so-you-don't-have-to/)
* [Rendimiento de generación de procesos en Rust](https://kobzol.github.io/rust/2024/01/28/process-spawning-performance-in-rust.html)
* [Presentación de Foundations: nuestra biblioteca de bases de servicio Rust de código abierto](https://blog.cloudflare.com/introducing-foundations-our-open-source-rust-service-foundation-library)
* [Videojuegos gráficos vectoriales de alto rendimiento](https://simbleau.github.io/rust/graphics/2023/11/20/using-vello-for-video-games.html)
* [Algunos cambios recientes y notables en Rust](https://lwn.net/Articles/954033/)
* [Visualización de la programación dinámica con FireDBG](https://firedbg.sea-ql.org/blog/2024-01-31-visual-dynamic-program/)
* [video] [Nueve reglas para estructuras de datos en Rust](https://www.youtube.com/watch?v=09vg_GMftE8&t=9s)

### Tutoriales de Rust
* [Guías de diagnóstico de fugas de memoria de Rust mediante gráficos de llamas](https://www.greptime.com/blogs/2024-01-18-memory-leak)
* [WebSockets - La Guía para Principiantes](https://vaktibabat.github.io/posts/websockets/)
* [Escribir cronjobs en Rust](https://www.shuttle.rs/blog/2024/01/24/writing-cronjobs-rust)
* [Simultaneidad intrépida con Rust, gatos y algunos PI de Raspberry](https://manuel.bernhardt.io/posts/2024-01-26-rust-fearless-concurrency-cats-raspberry-pi/)
* [Macros de Rust que se encargan aún más de la plantilla de Lambda](https://medium.com/@sam.van.overmeire/rust-macros-taking-care-of-even-more-lambda-boilerplate-0c5cb6c4b63c)
* [Depuración de instrumentación de Tokio](https://hegdenu.net/posts/debugging-tokio-instrumentation/)

### Miscelánea
* [audio] [Arroyo - Micah Wylde, Co-Fundador y CEO](https://corrode.dev/podcast/s01e04-arroyo)

## Crate de la semana

El crate de esta semana es [Apache Iceberg Rust](https://github.com/apache/iceberg-rust/), una implementación de Rust de un formato de tabla para grandes conjuntos de datos analíticos.

¡Gracias a [Renjie Liu](https://users.rust-lang.org/t/crate-of-the-week/2704/1283) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [GreptimeTeam - Corrige un error menor en 'join_path' para un código más elegante](https://github.com/GreptimeTeam/greptimedb/issues/3212)
* [GreptimeTeam - Añadir pruebas para 'MetaPeerClientRef' para mejorar la estabilidad de GreptimeDB](https://github.com/GreptimeTeam/greptimedb/issues/3044)
* [Ockam - Resaltado de sintaxis para bloques de código cercados, en la salida de ayuda de comandos, en Linux funciona](https://github.com/build-trust/ockam/issues/7471)
* [Ockam - Se ha mejorado la salida de 'ticket de proyecto ockam' y la información no es opaca](https://github.com/build-trust/ockam/issues/7478)
* [Ockam - Se ha mejorado la salida tanto para 'ockam project ticket' como para 'ockam project enroll', con soporte para '--output json'](https://github.com/build-trust/ockam/issues/7473)
* [Hyperswitch - [FIX]: Añadir una validación de configuración para los trabajadores](https://github.com/juspay/hyperswitch/issues/3510)
* [Hyperswitch - [CARACTERÍSTICA]: Crear un punto final de eliminación para la tabla de configuración](https://github.com/juspay/hyperswitch/issues/3488)
* [Hyperswitch - [CARACTERÍSTICA]: Cobertura de código de configuración para pruebas locales y CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [FEATURE]: Tener get_required_value usar ValidationError en OptionExt](https://github.com/juspay/hyperswitch/issues/860)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

*Esta semana no se han presentado convocatorias de ponencias.*

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

Se presentaron 409 solicitudes de incorporación de cambios [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-23..2024-01-30

* ['pattern_analysis': deja que 'ctor_sub_tys' devuelvan cualquier Iterador que quieran](https://github.com/rust-lang/rust/pull/120317)
* ['pattern_analysis': reutiliza la mayor parte de la impl 'DeconstructedPat Debug'](https://github.com/rust-lang/rust/pull/120318)
* [añadir '#[coverage(off)]' a los cierres introducidos por '#[test]' y '#[bench]'](https://github.com/rust-lang/rust/pull/120183)
* [añadir la puerta de características 'min_exhaustive_patterns'](https://github.com/rust-lang/rust/pull/118803)
* [agregue la opción inestable para reducir el tamaño binario de la biblioteca dinámica...](https://github.com/rust-lang/rust/pull/118636)
* [normalizar siempre 'LoweredTy' en el nuevo solucionador](https://github.com/rust-lang/rust/pull/120378)
* [afirmar que se pasa un solo ámbito a 'for_scope'](https://github.com/rust-lang/rust/pull/120230)
* [evitar ICE en el rasgo sin pelusa 'dyn'](https://github.com/rust-lang/rust/pull/120275)
* [Patrones const en línea de comprobación prestada](https://github.com/rust-lang/rust/pull/120390)
* [clasificar los argumentos de cierre en un patrón refutable en el error de argumento](https://github.com/rust-lang/rust/pull/120382)
* [Const-eval interning: deshacerse del recorrido basado en tipos](https://github.com/rust-lang/rust/pull/119044)
* [cobertura: desmantelar 'Instrumentor' y aplanar el refinamiento del tramo](https://github.com/rust-lang/rust/pull/120292)
* [cobertura: no instrumentar las funciones '#[automatically_derived]']]](https://github.com/rust-lang/rust/pull/120185)
* [Cobertura: Nunca emitas regiones de cobertura mal ordenadas](https://github.com/rust-lang/rust/pull/119460)
* [no normalizar la firma de cierre al construir una corrección de compatibilidad 'FnOnce'](https://github.com/rust-lang/rust/pull/120139)
* [no llames a las funciones 'walk_' directamente si hay un método 'visit_' equivalente](https://github.com/rust-lang/rust/pull/120316)
* [no dispares 'OPAQUE_HIDDEN_INFERRED_BOUND' en el retorno de tamaño de AFIT](https://github.com/rust-lang/rust/pull/120360)
* [no resolver manualmente los cierres asíncronos en 'rustc_resolve'](https://github.com/rust-lang/rust/pull/120322)
* [emitir sugerencia al intentar escribir rangos exclusivos como '.. <'](https://github.com/rust-lang/rust/pull/119342)
* [arreglar asumir y aserción en subprocesos de salto](https://github.com/rust-lang/rust/pull/120171)
* [corrección: Corrección de la sugerencia arg para el rasgo impl](https://github.com/rust-lang/rust/pull/119957)
* [mejorar el manejo de expresiones en patrones](https://github.com/rust-lang/rust/pull/118625)
* [mejorar el manejo de números en 'IntoDiagnosticArg'](https://github.com/rust-lang/rust/pull/120398)
* [hacer '#! [allow_internal_unstable(..)] ' trabajar con 'stmt_expr_attributes'](https://github.com/rust-lang/rust/pull/117420)
* [implementar manualmente rasgos derivados 'NonZero'](https://github.com/rust-lang/rust/pull/120160)
* [modificar las estructuras GenericArg y Term para usar reglas de procedencia estrictas](https://github.com/rust-lang/rust/pull/119955)
* [condición de movimiento que habilita el pase a 'is_enabled'](https://github.com/rust-lang/rust/pull/120280)
* [normalizar los tipos de campo antes de comprobar la validez](https://github.com/rust-lang/rust/pull/120277)
* [solo ensamblar candidatos enlazados a alias para alias rígidos](https://github.com/rust-lang/rust/pull/119744)
* [recuperarse correctamente del ataque de arrastre en el cuerpo](https://github.com/rust-lang/rust/pull/118182)
* [proporcionar más contexto sobre el desbordamiento de evaluación 'impl' recursiva](https://github.com/rust-lang/rust/pull/119389)
* [riscv32im-risc0-zkvm-elf: add target](https://github.com/rust-lang/rust/pull/117958)
* [scopeTree: eliminar 'destruction_scopes' como no utilizado](https://github.com/rust-lang/rust/pull/120386)
* [split Diagnostics for Uncommon Codepoints: Add List to Display Characters Involved](https://github.com/rust-lang/rust/pull/120259)
* [Rasgo de división de tait e impl en la lógica de los elementos de la asociación](https://github.com/rust-lang/rust/pull/119766)
* [dejar de usar derivada en 'rustc_pattern_analysis'](https://github.com/rust-lang/rust/pull/120420)
* [sincronización de subárbol para 'rustc_codegen_cranelift'](https://github.com/rust-lang/rust/pull/120395)
* [sugerir 'array::from_fn' para la inicialización de array](https://github.com/rust-lang/rust/pull/119805)
* [use 'assert_unchecked' en lugar de 'asumir' intrínseco en la biblioteca estándar](https://github.com/rust-lang/rust/pull/119892)
* [interpretar: 'project_downcast': no hacer ICE para variantes deshabitadas](https://github.com/rust-lang/rust/pull/120367)
* [retorna un número finito de AllocIds por ConstAllocation en Miri](https://github.com/rust-lang/rust/pull/118336)
* [Miri: añadir '__cxa_thread_atexit_impl' en FreeBSD](https://github.com/rust-lang/miri/pull/3277)
* [Miri: añadir error portable-atomic-util a la lista de "errores encontrados"](https://github.com/rust-lang/miri/pull/3233)
* [Miri: FreeBSD añade *soporte de interceptación de llamadas STAT](https://github.com/rust-lang/miri/pull/3181)
* [solo usar conjuntos de bits densos en análisis de flujo de datos](https://github.com/rust-lang/rust/pull/116152)
* [eliminar todos los ConstPropNonsense](https://github.com/rust-lang/rust/pull/119627)
* [eliminar el rasgo StructuralEq](https://github.com/rust-lang/rust/pull/116167)
* [aumentar el rendimiento del iterador 'interscalar(_with)'](https://github.com/rust-lang/rust/pull/111379)
* [estabilizar métodos de matriz](https://github.com/rust-lang/rust/pull/103522)
* [std: hacer que el inicializador 'HEAP' nunca esté en línea](https://github.com/rust-lang/rust/pull/120205)
* [añadir familia de rasgos 'AsyncFn'](https://github.com/rust-lang/rust/pull/119305)
* [add 'ErrCode'](https://github.com/rust-lang/rust/pull/119972)
* [add 'NonZero*::count_ones'](https://github.com/rust-lang/rust/pull/118326)
* [añadir 'str::Lines::remainder'](https://github.com/rust-lang/rust/pull/107464)
* [ajustar el comportamiento de 'read_dir' y 'ReadDir' en la implementación de Windows: comprobar si existe la ruta de búsqueda](https://github.com/rust-lang/rust/pull/120373)
* [core: add 'From<core::ascii::Char>' implementaciones](https://github.com/rust-lang/rust/pull/120311)
* [manejar errores de memoria insuficiente en 'io:Read::read_to_end()'](https://github.com/rust-lang/rust/pull/117925)
* [impl 'De<&[T; N]>' por 'Vaca<[T]>'](https://github.com/rust-lang/rust/pull/113489)
* [rc,sync: no crear referencias a valores no inicializados](https://github.com/rust-lang/rust/pull/119433)
* [implementación inicial de 'str::from_raw_parts[_mut]'](https://github.com/rust-lang/rust/pull/119466)
* [eliminar el tratamiento de casos especiales de 'vec.split_off(0)'](https://github.com/rust-lang/rust/pull/119917)
* [reescribir la API del cursor 'BTreeMap' usando huecos](https://github.com/rust-lang/rust/pull/118208)
* [especialice 'Bytes' en 'StdinLock<'_>'](https://github.com/rust-lang/rust/pull/120053)
* [estabilizar 'slice_group_by'](https://github.com/rust-lang/rust/pull/117678)
* [cambiar la dirección del alias 'NonZero'](https://github.com/rust-lang/rust/pull/120165)
* [regex: hacer públicos los metadatos adicionales del prefiltro](https://github.com/rust-lang/regex/pull/1156)
* [cargo: 'docs(ref)': Intenta mejorar los documentos de autenticación de registro](https://github.com/rust-lang/cargo/pull/13351)
* [cargo: 'fix(cli)': Mejora de los errores relacionados con el script de carga](https://github.com/rust-lang/cargo/pull/13346)
* [cargo: 'fix(config)': Dejar de usar archivos que no son de extensión](https://github.com/rust-lang/cargo/pull/13349)
* [cargo: 'refactor(shell)': Usa una nueva y elegante API de estilo](https://github.com/rust-lang/cargo/pull/13368)
* [cargo: doc: reemplace la versión con 'latest' para el enlace del servidor de trabajo](https://github.com/rust-lang/cargo/pull/13366)
* [cargo: arreglar la descripción de la opción de lista que comienza con mayúsculas](https://github.com/rust-lang/cargo/pull/13344)
* [cargo: refactorizar: eliminar la opción innecesaria en 'Frescura::D irty'](https://github.com/rust-lang/cargo/pull/13361)
* [cargo: test: corrección de diseño de datos para 'x86_64-unknown-none-gnu'](https://github.com/rust-lang/cargo/pull/13362)
* [rustfmt: macro de envoltura que comienza con bloques de cuerpo anidados](https://github.com/rust-lang/rustfmt/pull/5582)
* [rustfmt: formatear la línea diff para que se pueda hacer clic fácilmente](https://github.com/rust-lang/rustfmt/pull/5971)
* [clippy: añadir pelusa 'to_string_trait_impl'](https://github.com/rust-lang/rust-clippy/pull/12122)
* [clippy: añadir nueva pelusa 'unnecessary_result_map_or_else'](https://github.com/rust-lang/rust-clippy/pull/12169)
* [clippy: falso positivo: 'needless_return_with_question_mark' con conversión de error implícita](https://github.com/rust-lang/rust-clippy/pull/12021)
* [clippy: 'redundant_closure_for_method_calls' Sugerir rutas relativas para módulos locales](https://github.com/rust-lang/rust-clippy/pull/11370)
* [clippy: 'multiple_crate_versions': añadir una opción de configuración para cajas duplicadas permitidas](https://github.com/rust-lang/rust-clippy/pull/12179)
* [clippy: 'never_loop': reconocer bloques 'try' desazucarados](https://github.com/rust-lang/rust-clippy/pull/12206)
* [clippy: evite el cierre redundante de linting cuando el destinatario está marcado como '#[track_caller]'](https://github.com/rust-lang/rust-clippy/pull/12202)
* [clippy: no adviertas sobre la aritmética de módulos cuando se compara con cero](https://github.com/rust-lang/rust-clippy/pull/12178)
* [clippy: assert* en varias condiciones después de desenrollar hará que lint 'nonminimal_bool' emita una advertencia](https://github.com/rust-lang/rust-clippy/pull/12083)
* [clippy: corrige las sugerencias incorrectas generadas por 'manual_retain' lint](https://github.com/rust-lang/rust-clippy/pull/12084)
* [clippy: falso positivo en 'redundant_closure_call' cuando los cierres se pasan a macros](https://github.com/rust-lang/rust-clippy/pull/12082)
* [clippy: sugerir la opción de configuración existente si se encuentra una](https://github.com/rust-lang/rust-clippy/pull/12180)
* [clippy: avisa si se utiliza un elemento que viene de una versión más reciente que MSRV](https://github.com/rust-lang/rust-clippy/pull/12160)
* [rust-analyzer: add postfix completion for let else](https://github.com/rust-lang/rust-analyzer/pull/15730)
* [Rust-analyzer: filtra los campos deshabilitados por CFG al reducir los patrones de registro](https://github.com/rust-lang/rust-analyzer/pull/16427)
* [rust-analyzer: reemplazado 'adjusted_display_range' por 'adjusted_display_range_new' en 'mismatched_arg_count'](https://github.com/rust-lang/rust-analyzer/pull/16431)

### Clasificación del rendimiento del compilador de Rust

Esta fue una semana muy tranquila en la que solo una solicitud de incorporación de cambios tuvo un impacto real en el rendimiento general del compilador. La eliminación del rasgo interno 'StructuralEq' supuso una mejora de aproximadamente el 0,4% de media en casi 50 puntos de referencia del mundo real. 

Triaje realizado por **@rylev**.
Rango de revisión: [d6b151fc7.. 5c9c3c7](https://perf.rust-lang.org/?start=d6b151fc77e213bf637db0f12c1965ace3ffe255&end=5c9c3c7871d603ba13d38372830eca0c9013e575&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.5% | [0.3%, 0.7%] | 5 |
| Regresiones ❌ <br /> (secundaria) | 0.5% | [0.2%, 1.4%] | 10 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-1,5%, -0,2%] | 48 |
| Mejoras ✅ <br /> (secundaria) | -2,3% | [-7,7%, -0,4%] | 36 |
| Todos ❌✅ (primario) | -0,4% | [-1,5%, 0,7%] | 53 |

0 regresiones, 4 mejoras, 4 mixtas; 3 de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/54a18b2515048a5695aa61e79cbf12b5ed9a118d/triage/2024-01-30.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [Evitar definiciones no locales en las funciones](https://github.com/rust-lang/rfcs/pull/3373)
* [RFC: constantes en patrones](https://github.com/rust-lang/rfcs/pull/3535)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Incluir 'Future' y 'IntoFuture' en el preludio de 2024](https://github.com/rust-lang/rfcs/pull/3509)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [mut estático: permitir referencia mutable a tipos arbitrarios, no solo rebanadas y matrices](https://github.com/rust-lang/rust/pull/117614)
* [disposición: fusionar] [Hacer que async-fn-in-trait sea compatible con un futuro concreto en la implementación](https://github.com/rust-lang/rust/pull/120103)
* [disposición: fusionar] [Decisión: semántica del atributo '#[expect]'](https://github.com/rust-lang/rust/issues/115980)
* [disposición: fusionar] [guía de estilo: Al romper binops se maneja mejor el primer operando de varias líneas](https://github.com/rust-lang/rust/pull/119838)
* [disposición: fusionar] [guía de estilo: Modificar el formato 'Cargo.toml' para no poner la descripción al final](https://github.com/rust-lang/rust/pull/120072)
* [disposición: fusionar] [style-guide: Formato de un solo tipo asociado donde las cláusulas en la misma línea](https://github.com/rust-lang/rust/pull/119515)
* [disposición: fusionar] [PartialEq, PartialOrd: actualizar y sincronizar el manejo de cadenas transitivas](https://github.com/rust-lang/rust/pull/115386)
* [disposition: merge] ['std::error::Error' -> Implementaciones de rasgos: mejora de la consistencia de las vidas por vida](https://github.com/rust-lang/rust/pull/113833)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Dejar de usar y luego eliminar el mut estático](https://github.com/rust-lang/rfcs/pull/3560)
* [RFC: El Rust tiene procedencia](https://github.com/rust-lang/rfcs/pull/3559)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2024-01-31 - 2024-02-28 🦀

### Virtual

* 31/01/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**¡Lanzamiento del Club de Lectura de Rustaceans!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298563633/)
* 01/02/2024 | Virtual + Presencial (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/) - [Transmisión](https://www.youtube.com/@bcnrust)
* 01/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457951/)
* 03/02/2024 | Virtual + Presencial (Bruselas, Bélgica) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM: Rust devroom - charlas**](https://fosdem.org/2024/schedule/track/rust/)
* 03/02/2024 | Virtual (Kampala, UG) | [Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 04/02/2024 | Virtual | [Especialista en Rust](https://meet-os.com/group/1)
    * [**Desarrollo web con Rocket - En Inglés**](https://meet-os.com/event/1)
* 07/02/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh - Cómo el Rust me salvó los ojos**](https://www.meetup.com/indyrs/events/298641965/)
* 08/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 08/02/2024 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 10/02/2024 | Virtual (Cracovia, PL) | [Stacja IT Kraków](https://www.meetup.com/stacja-it-krakow/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-krakow/events/298303129/)
* 10/02/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 13/02/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341575/)
* 15/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 15/02/2024 |  Virtual + Presencial (Praga, CZ) | [Rust República Checa](https://www.meetup.com/rust-czech-republic/)
    * [**Introducción y Rust en producción**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 22/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)

### Asia

* 10/02/2024 | Hyderabad, IN | [Idioma Rust Hyderabad](https://www.meetup.com/rust-hyderabad/)
    * [**Rust Language Develope BootCamp**](https://www.meetup.com/rust-hyderabad/events/298687498/)

### Europa

* 01/02/2024 | Híbrido (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12ª reunión de BcnRust**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 03/02/2024 | Bruselas, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM '24: Rust devroom - charlas**](https://fosdem.org/2024/schedule/track/rust/) | [**Reunión FOSDEM de Rust Aarhus**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 03/02/2024 | Nürnberg, BY, DE | [Campamento de Rust de Paessler 2024](https://www.meetup.com/paessler-rust-camp-2024/)
    * [**Paessler Rust Camp 2024**](https://www.meetup.com/paessler-rust-camp-2024/events/298603948)
* 05/02/2024 | Bruselas, BE | [Grupo de usuarios de Rust de Bélgica](https://www.meetup.com/fr-FR/belgium-rust-user-group/)
    * [**Encuentro post-FOSDEM Rust @ Vrije Universiteit Brussel**](https://www.meetup.com/fr-FR/belgium-rust-user-group/events/298754029/)
* 06/02/2024 | Bremen, DE | [Encuentro de Rust Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 07/02/2024 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/)
    * [**Abstracciones incrustadas**](https://www.meetup.com/rustcologne/events/298913201/) | [**Página del evento**](https://rust.cologne/2024/02/07/embedded-hal.html)
* 07/02/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 08/02/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀 **](https://www.meetup.com/rust-bern/events/298488858/)
* 15/02/2024 | Praga, CZ - Virtual + Presencial | [Rust República Checa](https://www.meetup.com/rust-czech-republic/)
    * [**Introducción y Rust en producción**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 22/02/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)

### América del Norte

* 07/02/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Coolidge Corner Brookline Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/297635028/)
* 08/02/2024 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/)
    * [**BEAST: Recreando un juego clásico de terminal de DOS en Rust**](https://www.meetup.com/utah-rust/events/298888955/)
* 12/02/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 13/02/2024 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Mezclador mensual Rust NYC**](https://www.meetup.com/rust-nyc/events/298593474/)
* 13/02/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)
* 15/02/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Back Bay Rust, 15 de febrero**](https://www.meetup.com/bostonrust/events/297635043/)
* 15/02/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 28/02/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)

### Oceanía

* 06/02/2024 | Perth, WA, AU | [Grupo de reunión de Perth Rust](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Reunión de Rust de febrero de 2024**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)
* 27/02/2024 | Canberra, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/297650401/)
* 27/02/2024 | Sídney, Nueva Gales del Sur, Australia | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [** 🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> La gran estabilidad de este programa es lo que me hizo usar Rust para todo en el futuro. El servicio social tiene un tiempo de actividad del 100% desde hace casi 2,5 años. Ha procesado 12,9 TB de tráfico y sigue usando 1,5 MB de RAM como el día que lo ejecutamos hace 2,5 años. El uso de recursos es tan bajo que se me llenan los ojos de lágrimas. Como alguien que viene de Java, la falta de errores OOM o problemas de GC ha sido un gran beneficio de Rust y nunca me veo usando ningún otro lenguaje de programación. Soy un gran admirador de la mentalidad de "constrúyelo una vez, pero constrúyelo de la manera correcta", por lo que el Rust es siempre mi elección.

– [/u/Tiflotin en /r/rust](https://reddit.com/r/rust/comments/1ach3ir/what_were_some_of_the_first_useful_applications/)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1519) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ag0grd/this_week_in_rust_532/)</small>
