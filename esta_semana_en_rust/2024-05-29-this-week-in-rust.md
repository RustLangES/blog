---
title: "Esta semana en Rust #22"
number_of_week: 22
description: El crate de esta semana es pulso, un simple recopilador de métricas para TCP/IP.
date: 2024-05-29
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

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Oficial
* [Bienvenido James Munns al Consejo de Liderazgo](https://blog.rust-lang.org/inside-rust/2024/05/28/launching-pad-representative.html)

### Boletines informativos
* [Interfaz de usuario de Cyberpunk, simulación de Minecraft y destrucción volumétrica](https://thisweekinbevy.com/issue/2024-05-27-cyberpunk-ui-minecraft-simulation-and-volumetric-destruction)

### Actualizaciones de proyectos/herramientas
* [Motor de juego Fyrox 0.34](https://fyrox.rs/blog/post/fyrox-game-engine-0-34/)
* [El nuevo motor de ejecución de Wasmi - Más rápido que nunca](https://wasmi-labs.github.io/blog/posts/wasmi-v0.32/)
* [iroh@0.17.0 - Todo está un poco mejor](https://iroh.computer/blog/iroh-0-17-0-everything-is-a-little-better)
* [venndb@0.5.0 - admite opciones de filtro multidimensionales para filtrar en grupo](https://github.com/plabayo/venndb/releases/tag/0.5.0)

### Observaciones/Pensamientos
* [Diesel: Estudio de caso de evaluación comparativa continua](https://bencher.dev/learn/case-study/diesel/)
* [Visiones del futuro: verificación formal en Rust](https://xav.io/blog/rust-formal-verification/)
* [Evitar la dependencia excesiva de los canales 'mpsc' en Rust](https://blog.digital-horror.com/blog/how-to-avoid-over-reliance-on-mpsc/)
* [Cómo migramos nuestro analizador estático de Java a Rust](https://www.datadoghq.com/blog/engineering/how-we-migrated-our-static-analyzer-from-java-to-rust/)
* [Construyendo RAG Agentic con Rust, Qdrant y OpenAI](https://www.shuttle.rs/blog/2024/05/23/building-agentic-rag-rust-qdrant)
* [Hacer un chat seguro en Rust](https://vaktibabat.github.io/posts/Making_A_Secure_Chat_Rust_Crypto/)
* [Análisis de variables de entorno estructurado en Rust](https://blog.frankel.ch/structured-env-vars-rust/)
* [Tipos y código autodocumentado en Rust](https://ceronman.com/2024/05/28/types-and-self-documenting-code-in-rust/)
* [Iggy.rs — un año de construcción de la transmisión de mensajes](https://blog.iggy.rs/posts/one-year-of-building-the-message-streaming/)
* [Cuando la asignación de memoria no utilizada aumenta el rendimiento 2 veces](https://quickwit.io/blog/performance-investigation)
* [Intervalos de sondeo atómicos para cargas de trabajo altamente simultáneas](https://www.byronwasti.com/polling-atomics/)
* [Domar sumas de coma flotante](https://orlp.net/blog/taming-float-sums/)
* [Send & Mutex - Conceptos erróneos sobre Send](https://cryptical.xyz/rust/send-mutex)

### Tutoriales de Rust

* [Build with Naz : Linux io_uring y exploración tokio-uring con Rust](https://developerlife.com/2024/05/25/tokio-uring-exploration-rust/)
* [Introducción a Loco y SeaORM](https://www.sea-ql.org/blog/2024-05-28-getting-started-with-loco-seaorm/)

### Miscelánea
* [Reconstruyendo la sociedad del consumidor en Rust](https://filtra.io/rust-amo-may-24)
* [Informe sobre la discusión de genéricos variádicos en RustNL.](https://poignardazur.github.io/2024/05/25/report-on-rustnl-variadics/)
* [Cómo usar ChatGPT con Rust](https://www.onlycoiners.com/user/steadylearner/blog/how-to-use-chatgpt-with-rust)
* [video] [Programming Education: Tailoring Tools and Techniques for Rust (Will Crichton en la serie de transmisión en vivo de RustRover)](https://www.youtube.com/watch?v=u85bozA3bv0)

## Crate de la semana

El crate de esta semana es [pulso](https://github.com/guapodero/pulso), un simple recopilador de métricas para TCP/IP.

¡Gracias a [guapodero](https://users.rust-lang.org/t/crate-of-the-week/2704/1312) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria de pruebas
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

* [zerocopy - ¿La caché de CI no incluye las herramientas de carga instaladas?](https://github.com/google/zerocopy/issues/1312)
* [zerocopy - Debug GitHub Actions issue](https://github.com/google/zerocopy/issues/1276)
* [zerocopy - Error de paso 'jq' del documento de carga](https://github.com/google/zerocopy/issues/1228)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí] [directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [Rust Argentina junio 2024](https://sessionize.com/rust-argentina-june/) | Cierra el 31/05/2024 | Buenos Aires, AR | Fecha del evento: 2024-06-04
* [EuroRust 2024](https://www.papercall.io/eurorust-2024) | Cierra el 03/06/2024 | Viena, Austria y en línea | Fecha del evento: 2024-10-10
* [Computación científica en Rust 2024](https://scientificcomputing.rs/) | Cierra 14/06/2024 | En línea | Fecha del evento: 2024-07-17 - 2024-07-19
* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Cierra el 06/07/2024 | Online + Ucrania, Kiev | Fecha del evento: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

397 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-21..2024-05-28

* [Validación MIR: asegúrese de que la proyección descendente vaya seguida de una proyección de campo](https://github.com/rust-lang/rust/pull/125616)
* [en realidad usa TAIT en lugar de emularlo](https://github.com/rust-lang/rust/pull/125362)
* [permitir coercitar funciones cuya firma difiere en tipos opacos en su ámbito de definición en un tipo de puntero de función compartida](https://github.com/rust-lang/rust/pull/124297)
* [permitir errores de const eval de tiempo de monomorfización si la causa es un problema de diseño de tipos](https://github.com/rust-lang/rust/pull/124516)
* [un cierre asíncrono puede implementar 'FnMut'/'Fn' si no tiene autopréstamos](https://github.com/rust-lang/rust/pull/125259)
* [detectar estructuras no utilizadas que implementan rasgos privados](https://github.com/rust-lang/rust/pull/122382)
* [No permitir la conversión con macro reforzada final en let-else](https://github.com/rust-lang/rust/pull/125049)
* [No continúes sondeando el método si en la ambigüedad de los resultados de sugerencia y autoderef](https://github.com/rust-lang/rust/pull/125466)
* [No hacer detección de CC para objetivos sintéticos](https://github.com/rust-lang/rust/pull/125369)
* [No monomorfonice ansiosamente la caída de tipos que son imposibles de instanciar](https://github.com/rust-lang/rust/pull/125513)
* [no sugiera agregar los CFG inesperados al propio script de compilación](https://github.com/rust-lang/rust/pull/125412)
* [Eliminar restricciones de región para objetivos ambiguos](https://github.com/rust-lang/rust/pull/125413)
* [Salir del proceso poco tiempo después de ingresar a nuestro manejador Ctrl-C](https://github.com/rust-lang/rust/pull/125523)
* [expandir 'for_loops_over_fallibles' lint a lint en falibles detrás de referencias](https://github.com/rust-lang/rust/pull/125156)
* [f32: usar constantes en lugar de reasignar un valor ficticio como PI](https://github.com/rust-lang/rust/pull/125571)
* [falla relacionando constantes de diferentes tipos](https://github.com/rust-lang/rust/pull/125451)
* [corregir la sugerencia de error de OutsideLoop: agregar la etiqueta ''bloque' para el bloque ''si'](https://github.com/rust-lang/rust/pull/123623)
* [arreglar la pelusa 'unexpected_cfgs' en std](https://github.com/rust-lang/rust/pull/125296)
* [Se corrigió una sugerencia incorrecta para las duraciones de HRTB no declaradas en las cláusulas where](https://github.com/rust-lang/rust/pull/123122)
* [corregir el análisis sintáctico de puntos y comas colocados erróneamente](https://github.com/rust-lang/rust/pull/125276)
* [correcciones de seguimiento a 'report_return_mismatched_types'](https://github.com/rust-lang/rust/pull/123812)
* [forzar la corrutina interna de un cierre asíncrono para que se 'mueva' si el cierre externo es 'move' y 'FnOnce'](https://github.com/rust-lang/rust/pull/125306)
* [handle 'ReVar' en 'note_and_explain_region'](https://github.com/rust-lang/rust/pull/125054)
* [asegúrese de que la resolución del método coincida con 'note_source_of_type_mismatch_constraint'](https://github.com/rust-lang/rust/pull/124227)
* [mover '#[do_not_recommend]' al espacio de nombres '#[diagnóstico]'](https://github.com/rust-lang/rust/pull/125326)
* [solo permite estáticas inmutables con '#[linkage]'](https://github.com/rust-lang/rust/pull/125046)
* [solo suprimir el error de binop a favor de la sugerencia de punto y coma si estamos en una declaración de asignación](https://github.com/rust-lang/rust/pull/125467)
* [pánico directamente en 'Argumentos::nuevo*' en lugar de repetir](https://github.com/rust-lang/rust/pull/117804)
* [tipos de patrones: Prohibir argumentos genéricos en parámetros const](https://github.com/rust-lang/rust/pull/125015)
* [tratar adecuadamente los tipos faltantes/marcadores de posición dentro de los GAC](https://github.com/rust-lang/rust/pull/125457)
* [relajar las restricciones sobre desinfectantes múltiples](https://github.com/rust-lang/rust/pull/124676)
* [eliminar 'DefId' de 'EarlyParamRegion'](https://github.com/rust-lang/rust/pull/125468)
* [eliminar el formato del árbol de pruebas, hacerlo superficial](https://github.com/rust-lang/rust/pull/125510)
* [cambiar el nombre de 'FrameworkOnlyWindows' a 'RawDylibOnlyWindows'](https://github.com/rust-lang/rust/pull/125409)
* [resolver los predicados primarios de la const anónima para dirigir el padre en lugar del padre de opaque](https://github.com/rust-lang/rust/pull/125501)
* [dejar de SRoA 'DynMetadata' en MIR](https://github.com/rust-lang/rust/pull/125508)
* [soporta variádicas de C23 sin un parámetro nombrado](https://github.com/rust-lang/rust/pull/124048)
* [etiqueta más cosas con 'WG-trait-system-refactor'](https://github.com/rust-lang/rust/pull/125519)
* [convertir las pelusas restantes no estructurales en patrones en errores duros](https://github.com/rust-lang/rust/pull/124661)
* [use 'Backtrace::force_capture' en lugar de 'Backtrace::capture' en 'rustc_log'](https://github.com/rust-lang/rust/pull/125355)
* [validar la restricción de diseño especial en 'DynMetadata'](https://github.com/rust-lang/rust/pull/125479)
* [advertencia (o error) cuando se hace referencia al ctor 'Self' del elemento externo en el elemento anidado interno](https://github.com/rust-lang/rust/pull/124187)
* [wrap Context.ext en AssertUnwindSafe](https://github.com/rust-lang/rust/pull/125392)
* [interpretar: hacer que los binops desbordados sean binops normales](https://github.com/rust-lang/rust/pull/125359)
* [Miri: Añade algunas características de Tokio](https://github.com/rust-lang/miri/pull/3628)
* [miri: corrección de errores 'MiriAllocBytes' para garantizar diferentes direcciones](https://github.com/rust-lang/miri/pull/3625)
* [Miri: Refactorizar completamente la forma en que gestionamos el bloqueo y desbloqueo de hilos](https://github.com/rust-lang/miri/pull/3631)
* [perf: Retrasar la construcción de estructuras tempranas de diagnóstico de lint](https://github.com/rust-lang/rust/pull/125410)
* [estabilizar 'LazyCell' y 'LazyLock'](https://github.com/rust-lang/rust/pull/121377)
* [estabilizar 'div_duration'](https://github.com/rust-lang/rust/pull/124667)
* [estabilizar 'slice_flatten'](https://github.com/rust-lang/rust/pull/125561)
* [reescribir el almacenamiento local de subprocesos nativos](https://github.com/rust-lang/rust/pull/116123)
* [reescribir TLS en plataformas sin hilos](https://github.com/rust-lang/rust/pull/123724)
* [simplificar las configuraciones locales de subprocesos basadas en claves](https://github.com/rust-lang/rust/pull/122494)
* [añadir la bandera de la función de la biblioteca principal de opt-for-size](https://github.com/rust-lang/rust/pull/125011)
* [Utilice siempre el recuento general de caracteres de mayúsculas y minúsculas con 'optimize_for_size'](https://github.com/rust-lang/rust/pull/125609)
* [añadir 'IntoIterator' para 'Box<[T]>' + pelusas específicas de la edición 2024](https://github.com/rust-lang/rust/pull/124097)
* [añadir 'assert_unsafe_precondition' a 'unchecked_{add,sub,neg,mul,shl,shr}' métodos](https://github.com/rust-lang/rust/pull/121571)
* [añadir una ruta rápida a 'Depurar' ASCII '&str'](https://github.com/rust-lang/rust/pull/121150)
* [añadir la implicación manual de 'Sync' para 'ReentrantLockGuard'](https://github.com/rust-lang/rust/pull/125527)
* [arreglar 'VecDeque::shrink_to' UB cuando 'handle_alloc_error' se desenrolla](https://github.com/rust-lang/rust/pull/123803)
* [simplificar un poco las comprobaciones ub 'unchecked_sh[LR]']](https://github.com/rust-lang/rust/pull/125559)
* [menos llamadas al sistema para la sonda 'copy_file_range'](https://github.com/rust-lang/rust/pull/122079)
* [hacer 'abrazadera' en línea](https://github.com/rust-lang/rust/pull/125455)
* [corregir 'c_char' en AIX](https://github.com/rust-lang/rust/pull/122986)
* [pánico si 'PathBuf::set_extension' agregaría un separador de ruta](https://github.com/rust-lang/rust/pull/125070)
* [codegen\_llvm: agrega soporte para escribir código de bits de resumen](https://github.com/rust-lang/rust/pull/125345)
* [codegen\_gcc: simd: implementar intrínsecos de procedencia del puntero](https://github.com/rust-lang/rustc_codegen_gcc/pull/519)
* [rust-lld: reserva a la raíz del sistema de rustc si no hay ninguna ruta al enlazador en la raíz del sistema de destino](https://github.com/rust-lang/rust/pull/125263)
* [enlazador autónomo: reintente enlazar sin '-fuse-ld=lld' en CCs que no lo soportan](https://github.com/rust-lang/rust/pull/125417)
* [Cargo: añadir más trazas de alto nivel](https://github.com/rust-lang/cargo/pull/13951)
* [cargo: obtener confirmaciones específicas incluso si falla la ruta rápida de GitHub](https://github.com/rust-lang/cargo/pull/13946)
* [cargo: corrección: comprobar si Rev es SHA de confirmación completa para la ruta rápida de GitHub](https://github.com/rust-lang/cargo/pull/13969)
* [cargo: fix: eliminar el directorio de enlaces simbólicos en Windows](https://github.com/rust-lang/cargo/pull/13910)
* [cargo: mejorar la descripción del error al deserializar el campo parcial 'struct'](https://github.com/rust-lang/cargo/pull/13956)
* [Cargo: Prueba: Cambiar de 'Drop' a 'Let _' debido al cambio nocturno de Rustc](https://github.com/rust-lang/cargo/pull/13964)
* [Cargo: Mejora GIX de 0.62 a 0.63](https://github.com/rust-lang/cargo/pull/13948)
* [cargo: use 'i32' en lugar de 'usize' como "entero predeterminado" en la plantilla de biblioteca](https://github.com/rust-lang/cargo/pull/13939)
* [clippy: 'significant_drop_in_scrutinee': Activa la pelusa solo si la vida útil permite una caída significativa temprana](https://github.com/rust-lang/rust-clippy/pull/12740)
* [clippy: añadir nueva pelusa 'while_float'](https://github.com/rust-lang/rust-clippy/pull/12765)
* [clippy: añadir paréntesis a la sugerencia de 'let_and_return'](https://github.com/rust-lang/rust-clippy/pull/12842)
* [clippy: corrección de errores: error del mensaje 'numbered_fields' de lint](https://github.com/rust-lang/rust-clippy/pull/12398)
* [clippy: maneja correctamente los paréntesis de cierre en 'missing_backticks' doc lint](https://github.com/rust-lang/rust-clippy/pull/12809)
* [clippy: Solución rápida para URLs desnudas](https://github.com/rust-lang/rust-clippy/pull/12836)
* [clippy: corrige la interacción 'unnecessary_to_owned' con la expansión de macros](https://github.com/rust-lang/rust-clippy/pull/12843)
* [clippy: cumple con las expectativas en 'check_partial_eq_without_eq'](https://github.com/rust-lang/rust-clippy/pull/12841)
* [clippy: cumple con las expectativas en 'check_unsafe_derive_deserialize'](https://github.com/rust-lang/rust-clippy/pull/12804)
* [clippy: suprime 'iter_on_empty_collections' si se confía en el tipo concreto del iterador](https://github.com/rust-lang/rust-clippy/pull/12823)
* [rust-analyzer: añadir la acción de código de asistencia 'toggle_async_sugar'](https://github.com/rust-lang/rust-analyzer/pull/17258)
* [rust-analyzer: permitir que sysroots solo consista en el directorio raíz de la fuente](https://github.com/rust-lang/rust-analyzer/pull/17287)
* [Rust-Analyzer: borrar los diagnósticos solo después de recibir los nuevos](https://github.com/rust-lang/rust-analyzer/pull/17248)
* [rust-analyzer: más información invocable](https://github.com/rust-lang/rust-analyzer/pull/17268)
* [rust-analyzer: arreglar 'data_constructor' ignorando genéricos para 'struct'](https://github.com/rust-lang/rust-analyzer/pull/17291)
* [rust-analyzer: corrige el cwd inconsistente de los comandos 'run' y 'debug' en el cliente](https://github.com/rust-lang/rust-analyzer/pull/17275)
* [rust-analyzer: asegúrese de que los límites implícitos de los tipos asociados se tengan en cuenta en autocompletar](https://github.com/rust-lang/rust-analyzer/pull/17270)
* [rust-analyzer: arreglar la reducción de 'format_args' pasando parámetros incorrectos a 'rustc_parse_format'](https://github.com/rust-lang/rust-analyzer/pull/17279)
* [rust-analyzer: inferir el tipo de bloque asíncrono con retorno de cola expr](https://github.com/rust-lang/rust-analyzer/pull/17174)
* [Rust-analyzer: Resolver el preludio externo para mods locales en módulos de bloques](https://github.com/rust-lang/rust-analyzer/pull/17251)
* [Rust-Analyzer: use el canal correcto de la cadena de herramientas al generar enlaces de documentos de tipo incorporado](https://github.com/rust-lang/rust-analyzer/pull/17284)
* [Rust-Analyzer: varias correcciones de ruta de búsqueda](https://github.com/rust-lang/rust-analyzer/pull/17277)
* [rust-analyzer: maneja '{self}' al eliminar importaciones no utilizadas](https://github.com/rust-lang/rust-analyzer/pull/17140)
* [Rust-Analyzer: Implementa la asistencia para cambiar entre los comentarios doc y normales](https://github.com/rust-lang/rust-analyzer/pull/17253)

### Clasificación del rendimiento del compilador de Rust

Una semana relativamente tranquila, con pocos cambios importantes, el mayor impulsado por una mayor
Aumentar el alcance de la comprobación de condiciones previas inseguras.

Triaje realizado por **@simulacrum**.
Rango de revisión: [1d0e4afd.. A59072EC](https://perf.rust-lang.org/?start=1d0e4afd4cac09078e12a232508c3e9f8d42535d&end=a59072ec4fb6824213df5e9de8cae4812fd4fe97&absolute=false&stat=instructions%3Au)

2 regresiones, 3 mejoras, 5 mixtas; 3 de ellos en rollups
51 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-05-27.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [reorganizar el equipo compilador](https://github.com/rust-lang/rfcs/pull/3599)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento de 'Error' en 'core'](https://github.com/rust-lang/rust/issues/103765)
* [disposición: fusionar] [Hacer que 'WHERE_CLAUSES_OBJECT_SAFETY' sea una violación regular de la seguridad de los objetos](https://github.com/rust-lang/rust/pull/125380)
* [disposición: fusionar] [Mostrar archivos producidos por '--emit foo' en notificaciones de artefactos json](https://github.com/rust-lang/rust/pull/122597)
* [disposición: fusionar] [No intentes revelar tipos ocultos cuando intentes probar Congelar en el ámbito de definición](https://github.com/rust-lang/rust/pull/122192)
* [disposición: fusionar] [Los límites de los elementos pueden hacer referencia a proyecciones automáticas y seguir siendo seguros para los objetos](https://github.com/rust-lang/rust/pull/122804)
* [disposición: fusionar] [Usar una vida útil predeterminada de ''estático' en consts asociadas](https://github.com/rust-lang/rust/issues/125190)
* [disposición: fusionar] [Estabilizar la función 'custom_code_classes_in_docs'](https://github.com/rust-lang/rust/pull/124577)
* [disposición: fusionar] [Problema de seguimiento para integer_atomics](https://github.com/rust-lang/rust/issues/99069)
* [disposición: fusionar] [Constificación de la construcción de BinaryHeap](https://github.com/rust-lang/rust/issues/112353)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR en el período de comentarios finales de esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna RFC de equipo lingüístico entró en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

##### [Directrices sobre códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [[RFC] Grupos de captura de macros con nombre](https://github.com/rust-lang/rfcs/pull/3649)
* [nuevo] [Cambiar crates.io política para no ofrecer mediación de transferencia de cajas](https://github.com/rust-lang/rfcs/pull/3646)
* [nuevo] [Rasgos implementables externamente](https://github.com/rust-lang/rfcs/pull/3645)
* [nuevo] [[RFC] On_unimplemented_trait_use](https://github.com/rust-lang/rfcs/pull/3643)
* [nuevo] [[RFC] Gancho de generación de subprocesos (heredando locales de subprocesos)](https://github.com/rust-lang/rfcs/pull/3642)

## Próximos eventos

Eventos oxidados entre 2024-05-29 - 2024-06-26 🦀

### Virtual

* 29/05/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Enums, Structs, and Traits - Essential Building Blocks of Rust Programming**](https://www.eventbrite.com/e/enums-structs-and-traits-essential-building-blocks-of-rust-programming-tickets-904696681127)
* 30/05/2024 | Virtual + Presencial (Barcelona, ES) | [Materia principal](https://mainmatter.com/) y [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust para la web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/)
* 30/05/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 04/06/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: A Creative Thinker's Programming Language**](https://www.meetup.com/women-in-rust/events/300918713/)
* 04/06/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 05/06/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/299047896/)
* 06/06/2024 | Virtual | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Tu primera contribución a un proyecto de código abierto de Rust**](https://www.meetup.com/code-mavens/events/301156302/)
* 06/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477702/)
* 09/06/2024 | Virtual | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Páginas de GitHub para desarrolladores de Rust (inglés)**](https://www.meetup.com/code-mavens/events/301215326/)
* 11/06/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341709/)
* 13/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 13/06/2024 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 16/06/2024 | Virtual | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Taller: Desarrollo web en Rust usando Rocket (Inglés)**](https://www.meetup.com/code-mavens/events/301294669/)
* 18/06/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346963/)
* 19/06/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 20/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477705/)
* 25/06/2024 | Virtual (Dallas, TX, EE. UU.)| [Grupo de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)

### África

* 01/06/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Europa

* 2024-05-28 - 2024-05-30 | Berlín, DE | [Oxidar](https://oxidizeconf.com/)
    * [**Oxidar Conf 2024**](https://oxidizeconf.com/)
* 30/05/2024 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/)
    * [**Reunión de desarrolladores de Rust @ Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/301065548/)
* 30/05/2024 | Barcelona, ES | [Materia principal](https://mainmatter.com/) y [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust para la web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/)
* 30/05/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288963/)
* 30/05/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 patrocinado por Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 30/05/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 30/05/2024 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - Mayo - Rust Backend 101**](https://www.meetup.com/rust-vienna/events/301162548/)
* 05/06/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Junio 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 06/06/2025 | Vilnius, LT | [Vilna Rust](https://www.meetup.com/rust-in-vilnius/)
    * [**Disfruta de nuestro segundo evento de Rust y ZIG**](https://www.meetup.com/rust-in-vilnius/events/301012097/)
* 19/06/2024 - 24/06/2024 | Zúrich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 20/06/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)

### América del Norte

* 30/05/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 31/05/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Boston Common Rust, 31 de mayo**](https://www.meetup.com/bostonrust/events/300116786/)
* 08/06/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Porter Square, 8 de junio**](https://www.meetup.com/bostonrust/events/300116799/)
* 13/06/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/)
    * [**Reunión mensual: ¡Tema por determinar!**](https://www.meetup.com/spokane-rust/events/300020010/)
* 18/06/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 20/06/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)

### Oceanía

* 25/06/2024 | Canberra, ACt, AU | [Grupo de usuarios de Canberra Rust (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/300749371/)

### América del Sur

* 06/06/2024 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último [Who's Hiring thread en r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers)

# Frase de la semana

> lo he dicho antes y lo diré de nuevo: como hijo de OCaml y C++, Rust es actualmente el mejor lenguaje para cosas en forma de compilador de producción.

– [Alex Kladov en lobste.rs](https://lobste.rs/s/hjmrl1/how_we_migrated_our_static_analyzer_from#c_amxgiq)

¡Gracias a [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1570) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1d4aysp/this_week_in_rust_549/)</small>
