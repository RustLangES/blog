---
title: "Esta semana en Rust #25"
number_of_week: 25
description: El crate de esta semana es cargo-binstall, un subcomando de carga para instalar cajas de binarios fuera de sus versiones de github.
date: 2024-06-26
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
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Actualizaciones de proyectos/herramientas
* [ratatui - v0.27.0](https://ratatui.rs/highlights/v027/)
* [Introducción - ChoRus](https://lsd-ucsc.github.io/ChoRus/)
* [uuid ahora soporta correctamente los contadores de la versión 7](https://kodraus.github.io/rust/2024/06/24/uuid-v7-counters.html)
* [Godot-Rust - Actualización de junio de 2024](https://godot-rust.github.io/dev/june-2024-update/)
* [piggui v0.2.0](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.2.0) 
* [¡Lanzamiento de Git-Cliff 2.4.0!](https://git-cliff.org/blog/2.4.0)

### Observaciones/Pensamientos
* [Reclamación, automática y de otro tipo](https://smallcultfollowing.com/babysteps/blog/2024/06/21/claim-auto-and-otherwise/)
* [Propiedad](https://without.boats/blog/ownership/)
* [Puzzle: Compartir argumentos declarativos entre el nivel superior y el subcomando usando Clap](https://gribnau.dev/posts/puzzle-sharing-declarative-args-between-top-level-and-subcommand/)
* [¿Estará vivo Rust en 10 años?](https://tweedegolf.nl/en/blog/128/rust-in-ten-years)
* [Por qué WebAssembly llegó al Backend (Wasm in the wild part 3)](https://www.jakobmeier.ch/wasm-road-2)
* [¿La construcción in situ parece sorprendentemente simple?](https://blog.yoshuawuyts.com/in-place-construction-seems-surprisingly-simple/)
* [Linealizador ígneo](https://domain-j.com/Igneous-Linearizer/uuid/9e30337c-b890-4fd9-a0bd-51a7aa6e65b0)
* [La vida en los carriles rápidos](https://blog.spiraldb.com/life-in-the-fastlanes/)
* [Modelo de simultaneidad de Rust vs modelo de simultaneidad de Go: corrutinas stackless vs stackfull](https://kerkour.com/rust-vs-go-concurrency-models-stackfull-vs-stackless-coroutines)

### Tutoriales de Rust
* [¡Domina el Rust jugando videojuegos!](https://jonte-osterberg.medium.com/master-rust-by-playing-video-games-cf5f7d8b1e28)
* [Instrumentación Tokio Waker](https://hegdenu.net/posts/tokio-waker-instrumentation/)
* [Build with Naz : Guía completa para el análisis sintáctico de nombres](https://developerlife.com/2023/02/20/guide-to-nom-parsing/)
* [Ejecución de un TLC5940 con un ESP32 utilizando el periférico RMT](https://wapl.es/esp32-tlc5940-rmt/)
* [Rust Data-Structures: ¿Qué es un trie CIDR y cómo puede ayudarte?](https://d34dl0ck.me/rust-bites-cidr-trie/index.html)
* [Patrones de Rust: Micro SDK](https://kerkour.com/rust-patterns-micro-sdks)
* [serie] [La guía definitiva para el manejo de errores en Rust (parte 1): Errores dinámicos](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling)

### Investigación
* [¿Cuándo el paralelismo es intrépido y de coste cero con Rust?](https://dl.acm.org/doi/10.1145/3626183.3659966)

### Miscelánea
* [Entrevista con Luca Palmieri de Mainmatter](https://filtra.io/rust-mainmatter-jun-24)

## Crate de la semana

El crate de esta semana es [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), un subcomando de carga para instalar cajas de binarios fuera de sus versiones de github.

¡Gracias a [Jiahao XU](https://users.rust-lang.org/t/crate-of-the-week/2704/1317) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [cargo-generate - RFC sobre la lectura de valores toml en marcadores de posición](https://github.com/cargo-generate/cargo-generate/issues/770)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí] [directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Cierra el 06/07/2024 | Online + Ucrania, Kiev | Fecha del evento: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

428 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-18..2024-06-25

* ['hir_typeck': ser más conservador al hacer que la persona que llama a la nota elija ty param" nota](https://github.com/rust-lang/rust/pull/126558)
* ['rustc_type_ir': Omitir algunos campos 'struct' de la salida de depuración](https://github.com/rust-lang/rust/pull/126656)
* [tener en cuenta las cosas que optimizan los costos de inserción](https://github.com/rust-lang/rust/pull/126578)
* [en realidad contamina 'InferCtxt' cuando se emite un error de cumplimiento](https://github.com/rust-lang/rust/pull/126620)
* [add '#[rustc_dump_{predicates,item_bounds}]'](https://github.com/rust-lang/rust/pull/126686)
* [Agregue error duro y lint de migración para attrs inseguros](https://github.com/rust-lang/rust/pull/126177)
* [permitir que "C-unwind" fn tenga variádicas C](https://github.com/rust-lang/rust/pull/126843)
* [permitir restringir los tipos opacos durante la subtipificación en el sistema de rasgos](https://github.com/rust-lang/rust/pull/125447)
* [permitir restringir los tipos opacos durante varios cambios de tamaño](https://github.com/rust-lang/rust/pull/125610)
* [permitir el rastreo a través de invocaciones de consulta 'item_bounds' en opacos](https://github.com/rust-lang/rust/pull/126674)
* [prohibir 'ArrayToPointer' y 'MutToConstPointer' de MIR en tiempo de ejecución](https://github.com/rust-lang/rust/pull/126308)
* [cambie 'DefineOpaqueTypes::No' a 'Sí' en el código de diagnóstico](https://github.com/rust-lang/rust/pull/126675)
* [Recopilar attrs en const block expr](https://github.com/rust-lang/rust/pull/126735)
* [cobertura: añadir el indicador de depuración '-Zcoverage-options=no-mir-spans'](https://github.com/rust-lang/rust/pull/126587)
* [cobertura: validación de revisión de la '#[cobertura(..)] ' atributo](https://github.com/rust-lang/rust/pull/126682)
* [No permitir seguro/inseguro en elementos estáticos y FN](https://github.com/rust-lang/rust/pull/126758)
* [no haga ICE cuando encuentre un campo de tipo externo durante la validación](https://github.com/rust-lang/rust/pull/126833)
* [Corrección: la ruptura dentro del cierre asíncrono tiene un intervalo incorrecto para encerrar el cierre](https://github.com/rust-lang/rust/pull/125078)
* [E0308: tipos no coincidentes, cuando expr está en el cuerpo de un brazo, no agregue punto y coma ';' al final](https://github.com/rust-lang/rust/pull/126455)
* [Mejorar la recuperación de marcadores de conflicto](https://github.com/rust-lang/rust/pull/126125)
* [Mejorar la propina para rasgos inaccesibles](https://github.com/rust-lang/rust/pull/125852)
* [interpretar: mejor error cuando nos quedamos sin memoria](https://github.com/rust-lang/rust/pull/126583)
* [Hacer que el código de colocación asíncrono sea más coherente con el código de colocación normal](https://github.com/rust-lang/rust/pull/126594)
* [hacer que el fragmento de macro ':expr' dependiente de la edición actúe como lo hace el fragmento ':p at' dependiente de la edición](https://github.com/rust-lang/rust/pull/126700)
* [Hacer que la impresión bonita para 'F16' y 'F128' sea consistente](https://github.com/rust-lang/rust/pull/126654)
* [Reducción de coincidencias: expande los candidatos OR mezclados con los candidatos anteriores](https://github.com/rust-lang/rust/pull/126553)
* [mostrar aviso sobre "nunca usado" de Depurar para 'enumeración'](https://github.com/rust-lang/rust/pull/124460)
* [deja de ordenar el 'SyntaxContext' de 'Span's, ya que es incompatible con incremental](https://github.com/rust-lang/rust/pull/123165)
* [sugerir bloques const en línea para la inicialización de matrices](https://github.com/rust-lang/rust/pull/126899)
* [sugerir eliminar los campos de tupla no utilizados si son los últimos campos](https://github.com/rust-lang/rust/pull/124580)
* [Eleva el siguiente solucionador de rasgos a 'rustc_next_trait_solver'](https://github.com/rust-lang/rust/pull/126614)
* [añadir 'F16' y 'F128'](https://github.com/rust-lang/chalk/pull/811)
* [miri: /miri: error más agradable cuando falla la construcción de miri-script](https://github.com/rust-lang/miri/pull/3700)
* [Miri: 'UNIX/foreign_items': mover getpid a la parte derecha del archivo](https://github.com/rust-lang/miri/pull/3705)
* [miri: no confíes en libc existente en Windows](https://github.com/rust-lang/miri/pull/3695)
* [miri: arreglar el ICE causado por buscar más allá de 'i64::MAX'](https://github.com/rust-lang/miri/pull/3689)
* [miri: implementar intrínsecos LLVM x86 adx](https://github.com/rust-lang/miri/pull/3690)
* [miri: implementar intrínsecos de IMC x86 de LLVM](https://github.com/rust-lang/miri/pull/3674)
* [Miri: Error de archivo por lotes más agradable cuando falla la construcción de Miri-script](https://github.com/rust-lang/miri/pull/3703)
* [Miri: usa operaciones estrictas en lugar de operaciones comprobadas](https://github.com/rust-lang/miri/pull/3694)
* [guardar 2 punteros en 'TerminatorKind' (96 → 80 bytes)](https://github.com/rust-lang/rust/pull/126784)
* [agregue 'SliceLike' a 'rustc_type_ir', úselo en el código genérico del solucionador (+ algunos otros cambios)](https://github.com/rust-lang/rust/pull/126813)
* ['std::unix::fs': simplificación de copia para Apple](https://github.com/rust-lang/rust/pull/126807)
* ['std::unix::os::home_dir': optimización de reserva](https://github.com/rust-lang/rust/pull/126854)
* [reemplace los códigos auxiliares de coincidencia de patrones 'F16' y 'F128' con implementaciones reales](https://github.com/rust-lang/rust/pull/123088)
* [add 'PidFd::'{'kill', 'wait', 'try_wait'}](https://github.com/rust-lang/rust/pull/124101)
* [también obtener 'add nuw' de 'uN::checked_add'](https://github.com/rust-lang/rust/pull/126852)
* [generalizar {'Rc', 'Arc'}'::make_mut()' a tipos sin tamaño](https://github.com/rust-lang/rust/pull/116113)
* [implementar 'array::repeat'](https://github.com/rust-lang/rust/pull/119127)
* [make 'Option::as_[mut_]slice' 'const'](https://github.com/rust-lang/rust/pull/126711)
* [cambiar el nombre de 'std::fs::try_exists' a 'std::fs::exists' y estabilizar 'fs_try_exists'](https://github.com/rust-lang/rust/pull/126140)
* [reemplazar implementaciones de ordenación](https://github.com/rust-lang/rust/pull/124032)
* [retorna el tipo opaco de 'PanicInfo::message()'](https://github.com/rust-lang/rust/pull/126330)
* [estabilizar 'c_unwind'](https://github.com/rust-lang/rust/pull/116088)
* [std: Refactorizar la implementación de almacenamiento local de subprocesos](https://github.com/rust-lang/rust/pull/126523)
* [hashbrown: implementar operaciones XxxAssign en HashSets](https://github.com/rust-lang/hashbrown/pull/529)
* [hashbrown: reemplace "ahash" por "default-hasher" en las características de Cargo](https://github.com/rust-lang/hashbrown/pull/533)
* [cargo toml: avisar cuando la edición no está configurada, incluso cuando MSRV no está configurado](https://github.com/rust-lang/cargo/pull/14110)
* [cargo: add 'CodeFix::apply_solution' e impl 'Clone'](https://github.com/rust-lang/cargo/pull/14092)
* [cargo: hacer que '-Cmetadata' sea consistente en todas las plataformas](https://github.com/rust-lang/cargo/pull/14107)
* [cargo: simplificar la sintaxis de las funciones de comprobación](https://github.com/rust-lang/cargo/pull/14106)
* [cargo: simplificar la comprobación de los ciclos de dependencia](https://github.com/rust-lang/cargo/pull/14089)
* [Prueba de carga: agregar redacción automática para el error no encontrado](https://github.com/rust-lang/cargo/pull/14124)
* [Prueba de carga: Número de archivo de redacción automática](https://github.com/rust-lang/cargo/pull/14121)
* [Rustdoc: Agrega soporte para la función 'missing_unsafe_on_extern'](https://github.com/rust-lang/rust/pull/126761)
* [implementar el formato 'use<>' en rustfmt](https://github.com/rust-lang/rust/pull/126754)
* [rustfmt: palabras clave de seguridad de formato en elementos estáticos](https://github.com/rust-lang/rustfmt/pull/6204)
* [eliminar la impresión perdida de 'rewrite_static' de rustfmt](https://github.com/rust-lang/rust/pull/126888)
* [resuelva clippy 'f16' y 'f128 unimplemented!'/'FIXME's](https://github.com/rust-lang/rust/pull/126636)
* [clippy: 'missing_const_for_fn': añadir sugerencia aplicable a la máquina](https://github.com/rust-lang/rust-clippy/pull/12930)
* [clippy: agregar filtro de aplicabilidad a la página de lista de lint](https://github.com/rust-lang/rust-clippy/pull/12655)
* [clippy: añadir más tipos a 'is_from_proc_macro'](https://github.com/rust-lang/rust-clippy/pull/12942)
* [clippy: no peluques 'implicit_return' en las macros proc](https://github.com/rust-lang/rust-clippy/pull/12963)
* [clippy: corrige sugerencia incorrecta para 'manual_unwrap_or_default'](https://github.com/rust-lang/rust-clippy/pull/12961)
* [clippy: resuelve 'clippy::invalid_paths' en 'bool::then'](https://github.com/rust-lang/rust-clippy/pull/12965)
* [clippy: llamada innecesaria al método min/max](https://github.com/rust-lang/rust-clippy/pull/12368)
* [rust-analyzer: palabra clave asíncrona completa](https://github.com/rust-lang/rust-analyzer/pull/17459)
* [rust-analyzer: comprueba que Expr no es ninguno antes de añadir la corrección](https://github.com/rust-lang/rust-analyzer/pull/17464)
* [rust-analyzer: add 'toggleLSPLogs' command](https://github.com/rust-lang/rust-analyzer/pull/17438)
* [rust-analyzer: añadir espacio después de palabras clave específicas en la finalización](https://github.com/rust-lang/rust-analyzer/pull/17431)
* [Rust-Analyzer: Filtro de expansión de macros incorporada](https://github.com/rust-lang/rust-analyzer/pull/17419)
* [rust-analyzer: no elimine los paréntesis para las llamadas de punteros similares a funciones que son miembros de una 'estructura' o unión](https://github.com/rust-lang/rust-analyzer/pull/17471)
* [rust-analyzer: asegúrese de que no haya ciclos en el 'source_root_parent_map'](https://github.com/rust-lang/rust-analyzer/pull/17457)
* [rust-analyzer: arreglar las características del IDE que se rompen en algunas macros de atr](https://github.com/rust-lang/rust-analyzer/pull/17462)
* [Rust-analyzer: arreglar el pánico de flycheck cuando se cancela](https://github.com/rust-lang/rust-analyzer/pull/17461)
* [rust-analyzer: maneja los límites de caracteres para caracteres anchos en 'extend_selection'](https://github.com/rust-lang/rust-analyzer/pull/17426)
* [Rust-analyzer: Mejorar el texto flotante en el diagnóstico de archivos desvinculados](https://github.com/rust-lang/rust-analyzer/pull/17411)
* [Rust-analyzer: solo muestra el diagnóstico de archivos no vinculados en la primera línea durante el inicio](https://github.com/rust-lang/rust-analyzer/pull/17415)
* [rust-analyzer: finalizaciones de patrones en let-stmt](https://github.com/rust-lang/rust-analyzer/pull/17481)
* [rust-analyzer: use 'ItemInNs::Macros' para convertir ModuleItem en ItemInNs](https://github.com/rust-lang/rust-analyzer/pull/17469)
* [Rust-analyzer: Eliminar la advertencia de extensión panicbit.cargo](https://github.com/rust-lang/rust-analyzer/pull/17456)
* [Rust-analyzer: simplifica algunas tácticas de búsqueda de términos](https://github.com/rust-lang/rust-analyzer/pull/17478)
* [Rust-Analyzer: Búsqueda de términos: nueva táctica para las constantes de elementos asociadas](https://github.com/rust-lang/rust-analyzer/pull/17449)

### Clasificación del rendimiento del compilador de Rust

En su mayoría, una serie de mejoras impulsadas por [mejoras en el revestimiento MIR], con un pequeño número
índices de referencia que tengan una regresión significativa debido a las mejoras en la
[algoritmos de ordenación], que son mejoras en el tiempo de ejecución a costa de
Regresiones neutrales en tiempo de compilación, con valores atípicos en algunos casos.

[Mejoras en el revestimiento MIR]: https://github.com/rust-lang/rust/pull/126578
[algoritmos de ordenación]: https://github.com/rust-lang/rust/pull/124032

Triaje realizado por **@simulacrum**.
Rango de revisión: [c2932aaf.. C3D7FB39](https://perf.rust-lang.org/?start=c2932aaf9d20acbc9259c762f1a06f8767c6f13f&end=c3d7fb398569407350abe044e786bc7890c90397&absolute=false&stat=instructions%3Au)

[Ver informe completo para más detalles](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-06-23.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:
* [Cambiar crates.io política para no ofrecer mediación de transferencia de cajas](https://github.com/rust-lang/rfcs/pull/3646)
* [UnsafePinned: permitir alias de referencias mutables ancladas](https://github.com/rust-lang/rfcs/pull/3467)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [RFC: Notación de tipo de retorno](https://github.com/rust-lang/rfcs/pull/3654)
* [disposición: fusionar] [Agregar un mecanismo general para configurar RUSTFLAGS en Cargo solo para la caja raíz](https://github.com/rust-lang/rfcs/pull/3310)
* [disposición: cerrar] [Permitir especificar dependencias para artefactos individuales](https://github.com/rust-lang/rfcs/pull/2887)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] ['#![ crate_name = EXPR]' semánticamente permite que 'EXPR' sea una llamada a macro, pero por lo demás la ignora en su mayoría](https://github.com/rust-lang/rust/issues/122001)
* [disposición: fusionar] [Añadir sección de guía de estilo nocturno para la sintaxis 'precise_capturing' 'use<>'](https://github.com/rust-lang/rust/pull/126753)
* [disposición: fusionar] [Problema de seguimiento para PanicInfo::message](https://github.com/rust-lang/rust/issues/66745)
* [disposición: fusionar] [Problema de seguimiento para Cell::update](https://github.com/rust-lang/rust/issues/50186)
* [disposición: \<sin especificar\>] [Problema de seguimiento para core::arch::{x86, x86_64}::has_cpuid](https://github.com/rust-lang/rust/issues/60123)
* [disposition: merge] [Sintaxis para capturas precisas: 'impl Trait + use<..>'](https://github.com/rust-lang/rust/issues/125836)
* [disposición: fusionar] [Eliminar la pelusa 'box_pointers'.](https://github.com/rust-lang/rust/pull/126018)
* [disposición: fusionar] [Volver a implementar un límite basado en el tamaño de tipo](https://github.com/rust-lang/rust/pull/125507)
* [disposición: fusionar] [Problema de seguimiento para 'duration_abs_diff'](https://github.com/rust-lang/rust/issues/117618)
* [disposición: fusionar] [Comprobar los argumentos de alias para WF incluso si tienen variables enlazadas de escape](https://github.com/rust-lang/rust/pull/123737)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR en el período de comentarios finales de esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ningún problema de seguimiento del equipo lingüístico o PR entró en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de referencias lingüísticas ni solicitudes de incorporación de cambios en el período final de comentarios de esta semana.*

##### [Directrices sobre códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro ni PR que hayan entrado en el período de comentarios finales esta semana.*

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Sintaxis estructurada de carga para dependencias de características en cajas](https://github.com/rust-lang/rfcs/pull/3663)
* [nuevo] [Información de caja cruzada de rustdoc fusionable](https://github.com/rust-lang/rfcs/pull/3662)
* [nuevo] [Añadir RFC "crates.io: Eliminación de cajas"](https://github.com/rust-lang/rfcs/pull/3660)

## Próximos eventos

Eventos oxidados entre 2024-06-26 - 2024-07-24 🦀

### Virtual
* 27/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 02/07/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 02/07/2024 | Híbrido - Virtual y Presencial (Los Ángeles, CA, EE. UU.) | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/)
    * [**Reinicio de Rust LA**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 03/07/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Crear aplicaciones web con Rust y Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 03/07/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/300328025/)
* 04/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488820/)
* 06/07/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 09/07/2024 | Virtual | [Rust para el almuerzo](https://lunch.rs/)
    * [**Julio 2024 Rust for Lunch**](https://lunch.rs/meetups/2024-07-09/)
* 09/07/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/299346976/)
* 10/07/2024 | Virtual | [Centro de Investigación Electrónica](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Investigación informática con el lenguaje de programación Rust**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 11/07/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 11/07/2024 | Híbrido - Virtual y Presencial (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 11/07/2024 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 11/07/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Lectura de archivos JSON en Rust (inglés)**](https://www.meetup.com/code-mavens/events/301636580/)
* 16/07/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Desarrollo web en Rust usando Rocket - parte 2 (Inglés)**](https://www.meetup.com/code-mavens/events/301736709/)
* 17/07/2024 | Híbrido - Virtual y Presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488824/)
* 2024-07-23| Híbrido - Virtual y Presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
* 24/07/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Explorando los casos de uso de la API de Rust**](https://www.eventbrite.com/e/lunch-learn-exploring-rust-api-use-cases-tickets-928424531767)

### Asia
* 30/06/2024 | Kioto, JP | [Rust de Kioto](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Aplicaciones multiplataforma**](https://www.meetup.com/kyoto-rust/events/301499550/)
* 03/07/2024 | Tokio, JP | [Reunión de Rust en Tokio](https://www.meetup.com/ja-JP/tokyo-rust-meetup/)
    * [**¡Estaba entendiendo mal el WASM!**](https://www.meetup.com/ja-JP/tokyo-rust-meetup/events/301807832/)

### Europa
* 27/06/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288965/)
* 27/06/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 patrocinado por Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)
* 10/07/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust - Julio**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 11/07/2024 | Praga, República Checa | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Praga (julio de 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 16/07/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Construyendo una API REST en Rust usando Axum, SQLx y SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 16/07/2024 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Your Bugs & Rust Your Engines #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)
* 2024-07-23| Híbrido - Virtual y Presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)

### América del Norte
* 26/06/2024 | Austin, TX, EE. UU. | [ATC de Rust](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 27/06/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613483/)
* 27/06/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 27/06/2024 | St. Louis, MO, EE. UU. | [STl Rust](https://www.meetup.com/stl-rust/)
    * [**Sesión de Meet and Greet Hacker**](https://www.meetup.com/stl-rust/events/301321974/)
* 02/07/2024 | Híbrido - Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/)
    * [**Reinicio de Rust LA**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 05/07/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de la Universidad de Boston, 5 de julio**](https://www.meetup.com/bostonrust/events/301549737/)
* 11/07/2024 | Híbrido - Ciudad de México, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 11/07/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 17/07/2024 | Híbrido - Vancouver, Columbia Británica, CA | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: patrón de espera**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 24/07/2024 | Austin, TX, EE. UU. | [ATC de Rust](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)

### Oceanía

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Rust no tiene inconsistencias teóricas... Un logro notable...

– [Simon Peyton-Jones en YouTube](https://youtu.be/UBgam9XUHs0?t=2756)

¡Gracias a [ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1579) por la sugerencia y a [Simon Farnsworth](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1580) por el enlace mejorado!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1dpgsgg/this_week_in_rust_553_this_week_in_rust/)</small>
