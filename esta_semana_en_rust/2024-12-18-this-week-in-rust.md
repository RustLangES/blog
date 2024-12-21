---
title: "Esta semana en Rust #40"
number_of_week: 40
description: El crate de esta semana es cmd_lib, una biblioteca de macros de línea de comandos y utilidades para escribir fácilmente tareas similares a shell-script en Rust.
date: 2024-12-18
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Actualización de los objetivos del proyecto en noviembre](https://blog.rust-lang.org/2024/12/16/project-goals-nov-update.html)
* [Este ciclo de desarrollo en carga: 1.84](https://blog.rust-lang.org/inside-rust/2024/12/13/this-development-cycle-in-cargo-1.84.html)
* [Actualización del Director del Proyecto de diciembre de 2024](https://blog.rust-lang.org/inside-rust/2024/12/17/project-director-update.html)

### Boletines
* [Este mes en Rust OSDev: noviembre de 2024](https://rust-osdev.com/this-month/2024-11/)

### Actualizaciones de proyectos/herramientas
* [Hoja de ruta hiper 2025](https://seanmonstar.com/blog/hyper-roadmap-2025/)
* [Sistemas falibles Bevy, componentes sin ataduras e inmutables](https://thisweekinbevy.com/issue/2024-12-09-fallible-systems-bindless-and-immutable-components)
* [Sequoia PGP: Un retoño madura: Conoce sq 1.0](https://sequoia-pgp.org/blog/2024/12/16/202412-sq-1.0/)
* [Versión 0.30.0 - egui_kittest y modales](https://github.com/emilk/egui/releases/tag/0.30.0)
* [fish-shell 4.0b1, ahora en Rust](https://fishshell.com/blog/fish-4b/)
* [Presentando Limbo: Una reescritura completa de SQLite en Rust](https://turso.tech/blog/introducing-limbo-a-complete-rewrite-of-sqlite-in-rust)
* [Análisis de datos de streaming, versión 0.14.0 de Fluvio](https://www.fluvio.io/news/this-week-in-fluvio-0067)
* [Anunciando la pantalla hexadecimal: una alternativa moderna 'xxd'](https://users.rust-lang.org/t/announcing-hex-display-a-modern-alternative-to-xxd/122523)
* [Diesel: Funciones de ventana](https://blog.weiznich.de/blog/diesel-nl-net-grant-window-functions/)
* [Repetición 0.21.0 - Vista de gráfico, cuadrícula 3D y mejoras en la interfaz de usuario / UX](https://rerun.io/blog/graphs)
* [Presentamos Fjädra — un port de Rust de 'd3-force' para diseños de gráficos](https://github.com/grtlr/fjadra)

### Observaciones/Pensamientos
* [¿Qué es la seguridad de la memoria temporal y espacial?](https://blog.yoshuawuyts.com/temporal-spatial-memory-safety/)
* [Reducción del tamaño binario de WASM](https://www.warp.dev/blog/reducing-wasm-binary-size)
* [Recuperación de fallos en 256 bytes](https://cliffle.com/blog/exhubris-super/)
* [Depuración de características de caja de Rust](https://rustunit.com/blog/2024/12-16-rust-feature-debugging/)
* [audio] [Building Rust: Una entrevista con Nell Shamrell-Harrington](https://timclicks.dev/podcast/nell-shamrell-harrington)

### Tutoriales de Rust
* [Lee el código: Usar Drop Safely en Rust](https://v5.chriskrycho.com/journal/read-the-code/using-drop-safely-in-rust/)
* [Pensamientos sobre el hashing de Rust](https://purplesyringa.moe/blog/thoughts-on-rust-hashing/)
* [Esquema de la Especificación Parte I: Colección de Ciclos Simultáneos](https://maplant.com/2024-12-13-Scheme-to-the-Spec-Part-I:-Concurrent-Cycle-Collection.html)
* [Resolviendo el advenimiento del código en tiempo de compilación con macros de Rust](https://doublefree.bearblog.dev/solving-advent-of-code-at-compile-time-with-rust-macros/)
* [video] [Lectura del código de Rust: La caja de los miles](https://www.youtube.com/watch?v=ITTj7ByNStE)
* [video] [Construyendo un administrador de paquetes Lua en Rust (árabe)](https://youtu.be/YsupdHTAKDw)

### Miscelánea
* [¿Qué es la programación shift-left ⬅️?](https://dev.to/szabgab/what-is-shift-left-programming-5601)
* [Actualización del estado social de Rust 2024.12](https://rust.code-maven.com/rust-update-2024-12-17)
* [Tantos tokens, tan poco tiempo: Presentamos un tokenizador de pares de bytes más rápido y flexible](https://github.blog/ai-and-ml/llms/so-many-tokens-so-little-time-introducing-a-faster-more-flexible-byte-pair-tokenizer/)

## Crate de la semana

El crate de esta semana es [cmd_lib](https://crates.io/crates/cmd_lib), una biblioteca de macros de línea de comandos y utilidades para escribir fácilmente tareas similares a shell-script en Rust.

¡Gracias a [Remo Senekowitsch](https://users.rust-lang.org/t/crate-of-the-week/2704/1382) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas

Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

<!-- Convocatorias para Pruebas vaya aquí, use este formato: * [<RFC Topic>](<RFC URL>)
        * [Problema de seguimiento](<URL del problema de seguimiento>)
        * [Pasos de prueba](<URL de los pasos de prueba>)
-->
<!-- o si no hay RFC nuevos o actualizados esta semana, use: -->
<!-- * *No se crearon RFC nuevas o actualizadas esta semana.* -->
<!-- Recuerde eliminar la etiqueta 'call-for-testing' del RFC para que el mantenedor pueda volver a enviar señales a los testers, si así lo desea. -->

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*Esta semana no se han presentado convocatorias para participar.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 437 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-10..2024-12-17

* [(Re)Implementar 'impl_trait_in_bindings'](https://github.com/rust-lang/rust/pull/134185)
* ['rustc_borrowck': Deja de sugerir la sintaxis inválida '&mut raw const'](https://github.com/rust-lang/rust/pull/134244)
* [Limpiezas de 'rustc_mir_dataflow', incluyendo algunos cambios de nombre](https://github.com/rust-lang/rust/pull/133938)
* ['rustc_target': correcciones de cadena de destino ppc64 para LLVM 20](https://github.com/rust-lang/rust/pull/134115)
* [agregar soporte AST para carpetas inseguras](https://github.com/rust-lang/rust/pull/134140)
* [añadir soporte experimental sin pulir para AFIDT (fn asíncrono en el rasgo dyn)](https://github.com/rust-lang/rust/pull/133122)
* [permitir 'symbol_intern_string_literal' lint en los módulos de prueba](https://github.com/rust-lang/rust/pull/134173)
* [Tipos propios arbitrarios v2: cambios en el compilador principal](https://github.com/rust-lang/rust/pull/132961)
* [cambie 'GetManyMutError' para que coincida con la decisión de T-libs-api](https://github.com/rust-lang/rust/pull/133598)
* [comprobar si hay longitudes de matriz que no sean realmente 'usize'](https://github.com/rust-lang/rust/pull/134371)
* [codegen '#[naked]' funciones usando ASM global](https://github.com/rust-lang/rust/pull/128004)
* [Manejar correctamente los comentarios en los atributos en el código fuente de DocTests](https://github.com/rust-lang/rust/pull/134260)
* [no ICE cuando se encuentre nunca en el patrón de rango](https://github.com/rust-lang/rust/pull/134103)
* [no hagas un def id para 'impl_trait_in_bindings'](https://github.com/rust-lang/rust/pull/134313)
* [no muestra los argumentos completos del enlazador a menos que se pase '--verbose'](https://github.com/rust-lang/rust/pull/133633)
* [no use los límites de 'AsyncFnOnce::CallOnceFuture' para la deducción de firmas](https://github.com/rust-lang/rust/pull/134017)
* [codificar cierres de corrutinas en SMIR](https://github.com/rust-lang/rust/pull/134295)
* [Ejercicio de la interacción del rasgo const con los campos predeterminados](https://github.com/rust-lang/rust/pull/134136)
* [arreglar ICE en el error de tipo en promovido](https://github.com/rust-lang/rust/pull/134010)
* [arreglar ICE cuando se necesitan varias sustituciones de superrasgos pero solo se proporciona una](https://github.com/rust-lang/rust/pull/133392)
* [arreglar 'trimmed_def_paths' ICE en la función ptr comparison lint](https://github.com/rust-lang/rust/pull/134357)
* [arreglar nuestra definición de tipo 'llvm::Bool' para que esté firmada, para que coincida con 'LLVMBool'](https://github.com/rust-lang/rust/pull/134204)
* [interpretar: reducir el uso de 'TypingEnv::fully_monomorphized'](https://github.com/rust-lang/rust/pull/134058)
* [jsondocck: analizar, no validar comandos](https://github.com/rust-lang/rust/pull/133478)
* [Realice un seguimiento de los errores de análisis en los 'mods' y no emita errores de resolución para las rutas que los involucran](https://github.com/rust-lang/rust/pull/133937)
* [lint al combinar '#[no_mangle]' y '#[export_name]'](https://github.com/rust-lang/rust/pull/131558)
* [hace que 'Copiar' sea inseguro de implementar para ADTs con campos 'inseguros'](https://github.com/rust-lang/rust/pull/134008)
* [asegúrese de usar ty normalizado para const no evaluado en el valor predeterminado de 'struct'](https://github.com/rust-lang/rust/pull/134314)
* [modifica la instrucción del índice de 'gep [0 x %Type]' a 'gep %Type'](https://github.com/rust-lang/rust/pull/134117)
* [considere correctamente los APIT para la corrección de la adscripción de reserva de nunca tipos](https://github.com/rust-lang/rust/pull/134144)
* [eliminar la dependencia de 'Lexer' en 'Parser'](https://github.com/rust-lang/rust/pull/134192)
* [Eliminar consultas de la interfaz del controlador](https://github.com/rust-lang/rust/pull/134302)
* [heurística rudimentaria para insertar paréntesis cuando sea necesario para RPIT sobrecaptura lint](https://github.com/rust-lang/rust/pull/134142)
* [¡Algunos ajustes de diagnóstico de ASM! y una corrección de PaperCut](https://github.com/rust-lang/rust/pull/134070)
* [Limpiezas diagnósticas de diferencias entre el método de rasgo y el método IMPL](https://github.com/rust-lang/rust/pull/134386)
* [sugerir el uso de deref en patrones](https://github.com/rust-lang/rust/pull/132939)
* [suprimir el campo expr con un mensaje de error genérico si es un método](https://github.com/rust-lang/rust/pull/134154)
* [intente evaluar las constantes en la manipulación heredada](https://github.com/rust-lang/rust/pull/134081)
* [Ajustar el renderizado multispan para reducir la longitud de salida](https://github.com/rust-lang/rust/pull/134181)
* [use SourceMap para cargar los archivos del visualizador del depurador](https://github.com/rust-lang/rust/pull/134041)
* [use un intervalo más preciso en 'placeholder_type_error_diag'](https://github.com/rust-lang/rust/pull/134256)
* [Usar excepciones recién agregadas a la advertencia de rama no predeterminada](https://github.com/rust-lang/rust/pull/134089)
* [validar las rutas '--skip' y '--exclude'](https://github.com/rust-lang/rust/pull/134209)
* [validarse a sí mismo en predicados de host correctamente](https://github.com/rust-lang/rust/pull/134105)
* [comprobación de límites con PtrMetadata en lugar de Len en MIR](https://github.com/rust-lang/rust/pull/133734)
* [miri: Optimización de TB: Omitir subárboles en función de los permisos del nodo raíz del subárbol](https://github.com/rust-lang/miri/pull/4008)
* [miri: 'localtime_r': deduplicar la asignación de nombres de zona horaria](https://github.com/rust-lang/miri/pull/4069)
* [miri: usar clap en miri-script](https://github.com/rust-lang/miri/pull/4036)
* [estabilizar 'const_nonnull_new'](https://github.com/rust-lang/rust/pull/134116)
* [estabilizar cierres asíncronos](https://github.com/rust-lang/rust/pull/132706) (RFC [#3668](https://rust-lang.github.io/rfcs/3668-async-closures.html))
* [estabilizar el preludio de Rust 2024](https://github.com/rust-lang/rust/pull/134178)
* [Implica el rasgo 'UniqueRc'](https://github.com/rust-lang/rust/pull/133223)
* ['std::net': Solaris también soporta 'SOCK_CLOEXEC' desde 11.4](https://github.com/rust-lang/rust/pull/130361)
* [agregar métodos de descriptor de acceso de valor a 'Mutex' y 'RwLock'](https://github.com/rust-lang/rust/pull/133406)
* [desduplicar y mejorar la definición de 'core::ffi::c_char'](https://github.com/rust-lang/rust/pull/132975)
* [ejecutar destructores TLS para subprocesos wasm32-wasip1](https://github.com/rust-lang/rust/pull/133472)
* [wasi/fs: mejora la condición de parada para '<ReadDir' como 'Iterator>::next'](https://github.com/rust-lang/rust/pull/133184)
* [codegen\_gcc: estabilizar la lógica de análisis de configuración 'lang_tests_common'](https://github.com/rust-lang/rustc_codegen_gcc/pull/576)
* [CodeGen\_gcc: use conversiones en lugar de transmisión de bits entre punteros y enteros para solucionar problemas al usar el enlazador LLD](https://github.com/rust-lang/rustc_codegen_gcc/pull/577)
* [cargo: build-script: Pass 'CARGO_CFG_FEATURE'](https://github.com/rust-lang/cargo/pull/14902)
* [cargo: SourceId: use hash estable de rustc-stable-hash](https://github.com/rust-lang/cargo/pull/14917)
* [cargo: base: Bases de soporte en parches en manifiestos virtuales](https://github.com/rust-lang/cargo/pull/14931)
* [cargo: build-rs: Informar implícitamente de rerun-if-env-changed para la entrada](https://github.com/rust-lang/cargo/pull/14911)
* [cargo: resolver: No reportar todas las versiones como rechazadas](https://github.com/rust-lang/cargo/pull/14921)
* [cargo: resolver: En errores, mostrar versiones rechazadas sobre versiones alternativas](https://github.com/rust-lang/cargo/pull/14923)
* [cargo: resolver: Reportar entradas de índice no válidas](https://github.com/rust-lang/cargo/pull/14927)
* [cargo: resolver: Reportar versiones no coincidentes, en lugar de decir que no hay paquete](https://github.com/rust-lang/cargo/pull/14897)
* [cargo: script: No anular el perfil de lanzamiento](https://github.com/rust-lang/cargo/pull/14925)
* [cargo: un hash más rápido para ActivationsKey](https://github.com/rust-lang/cargo/pull/14915)
* [cargo: implemente '--depth workspace' para el comando 'árbol de carga'](https://github.com/rust-lang/cargo/pull/14928)
* [cargo: 'emit_serialized_unit_graph' usa el shell configurado](https://github.com/rust-lang/cargo/pull/14926)
* [rustdoc-search: arreglar la ruta no coincidente cuando el padre se reexporta dos veces](https://github.com/rust-lang/rust/pull/134231)
* [rustdoc-search: manejar 'impl Into<X>' mejor](https://github.com/rust-lang/rust/pull/134277)
* [rustdoc: arreglarse a sí mismo cmp](https://github.com/rust-lang/rust/pull/134214)
* [clippy: permitir que 'needless_option_take' informe sobre más casos](https://github.com/rust-lang/rust-clippy/pull/13684)
* [clippy: mejor mensaje de ayuda para 'comparison_chain' lint](https://github.com/rust-lang/rust-clippy/pull/13762)
* [clippy: corregir la sugerencia de pelusa 'single_match'](https://github.com/rust-lang/rust-clippy/pull/13824)
* [clippy: sugerencia correcta para 'unnecessary_sort_by' en 'no_std'](https://github.com/rust-lang/rust-clippy/pull/13836)
* [clippy: manejar correctamente los índices de cadena en 'literal_string_with_formatting_arg'](https://github.com/rust-lang/rust-clippy/pull/13841)
* [clippy: detectar sombreado en el campo de patrón](https://github.com/rust-lang/rust-clippy/pull/13797)
* [clippy: no sugiero usar 'Error' en 'no_std' antes de Rust 1.81](https://github.com/rust-lang/rust-clippy/pull/13834)
* [clippy: arreglar la sugerencia de 'must_use_unit' cuando hay varios atributos](https://github.com/rust-lang/rust-clippy/pull/13830)
* [clippy: arreglar la pelusa 'single_match' que se emite cuando no debería](https://github.com/rust-lang/rust-clippy/pull/13765)
* [clippy: impl inicial de 'repr_packed_without_abi'](https://github.com/rust-lang/rust-clippy/pull/13398)
* [rust-analyzer: añadir una ayuda para extraer una expresión en una constante](https://github.com/rust-lang/rust-analyzer/pull/18652)
* [Rust-analyzer: agregue una corrección de diagnóstico para eliminar el envoltorio innecesario en la discrepancia de tipos](https://github.com/rust-lang/rust-analyzer/pull/18458)
* [rust-analyzer: conserva el orden de los parámetros en 'extract_functions'](https://github.com/rust-lang/rust-analyzer/pull/18656)
* [rust-analyzer: reportar identificaciones no resueltas para capturas implícitas en 'format_args! ()»(https://github.com/rust-lang/rust-analyzer/pull/18696)
* [Rust-analyzer: arreglar el enlace de publicación del flujo de trabajo en el manual](https://github.com/rust-lang/rust-analyzer/pull/18666)
* [rust-analyzer: las macros proc copiadas no se limpian al salir](https://github.com/rust-lang/rust-analyzer/pull/18660)
* [rust-analyzer: soluciona un pánico con una solución de diagnóstico cuando se usa una palabra clave como campo](https://github.com/rust-lang/rust-analyzer/pull/18700)
* [Rust-analyzer: arreglar las finalizaciones de importación automática calificadas de ruta que no funcionan con las reexportaciones](https://github.com/rust-lang/rust-analyzer/pull/18699)
* [rust-analyzer: arreglar los nombres de proc-macro dylib en Windows](https://github.com/rust-lang/rust-analyzer/pull/18693)
* [rust-analyzer: arreglar la construcción de sourceroot para manifiestos virtuales](https://github.com/rust-lang/rust-analyzer/pull/18668)
* [rust-analyzer: generar implementación con elementos incluso si la edición de texto de fragmentos está deshabilitada](https://github.com/rust-lang/rust-analyzer/pull/18667)
* [rust-analyzer: Sugerencia de nombre mejorado para 'destructure_tuple_binding'](https://github.com/rust-lang/rust-analyzer/pull/18695)
* [rust-analyzer: pánico al mostrar parámetros genéricos con valores predeterminados, de nuevo](https://github.com/rust-lang/rust-analyzer/pull/18675)
* [Rust-analyzer: Swallow rustfmt parsing panics](https://github.com/rust-lang/rust-analyzer/pull/18663)
* [rust-analyzer: use el contenido literal de la cadena como nombre al extraer en la variable](https://github.com/rust-lang/rust-analyzer/pull/18690)
* [rust-analyzer: elementos de finalización de hash para que coincidan correctamente durante /resolve](https://github.com/rust-lang/rust-analyzer/pull/18653)
* [Rust-analyzer: Maneja adecuadamente diferentes valores predeterminados para la gravedad de las pelusas](https://github.com/rust-lang/rust-analyzer/pull/18466)
* [Rust-analyzer: muestra errores de expansión en la función 'expand_macro'](https://github.com/rust-lang/rust-analyzer/pull/18674)

### Clasificación del rendimiento del compilador de Rust

<!-- resultados de Perf van aquí -->

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

<!-- RFC aprobados vaya aquí, use este formato: * [Tema](URL) -->
<!-- o si no se aprobó ninguno esta semana, use: * *No se aprobaron RFC esta semana.* -->
<!-- * []() -->

<!-- ### [Propuestas de Cambios Importantes Aprobadas (MCP)](https://forge.rust-lang.org/compiler/mcp.html) <!~~ Los MCP ocurren con poca frecuencia, por lo que esta sección está comentada de forma predeterminada. ~~>
<!~~ Los MCP que han sido aprobados o rechazados esta semana van aquí, usan este formato: * [cambio importante aceptado|rechazado] [Tema](URL) ~~>
-->

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
<!-- RFC que han ingresado a FCP van aquí, use este formato: * [disposición: fusionar|cerrar] [Tema](URL) -->
<!-- o si ninguno ingresó a FCP esta semana, use: * *Ninguna RFC ingresó al Período de Comentarios Final esta semana.* -->
<!-- * [disposición: ] []() -->

#### Seguimiento de problemas y relaciones públicas
<!-- Problemas de seguimiento que han ingresado a FCP vaya aquí, use este formato: * [disposición: fusionar|cerrar] [Tema](URL) -->
<!-- o si ninguno ingresó a FCP esta semana, use: -->
<!-- * *No hay problemas de seguimiento ni PR ingresaron al período final de comentarios esta semana.* -->
<!-- * [disposición: ] []() -->

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
<!-- RFC nuevos o actualizados vayan aquí, use este formato: * [nuevo|actualizado] [Tema](URL) -->
<!-- o si no hay RFC nuevos o actualizados esta semana, use: -->
<!-- * *No se crearon RFC nuevas o actualizadas esta semana.* -->
<!-- * [nuevo|actualizado] []() -->

## Próximos eventos

Eventos oxidados entre 2024-12-18 - 2025-01-15 🦀

### Virtual
* 19/12/2024 | Virtual | [Scandio GmBH](https://www.eventbrite.de/o/scandio-gmbh-75623231843)
    * [**Einführung in Rust: Für eine nachhaltige Zukunft / Introducción a Rust: Por un Futuro Sostenible**](https://www.eventbrite.com/e/einfuhrung-in-rust-fur-eine-nachhaltige-zukunft-tickets-1106203667949)
* 19/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633276/)
* 19/12/2024 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Despedida de Año 🎉🎉 **](https://www.meetup.com/rust-argentina/events/305095113)
* 19/12/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 20/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 22/12/2024 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Are We Embedded Yet? - Implementando un pequeño servidor HTTP en un microcontrolador**](https://www.meetup.com/rust-tlv/events/304937982)
* 24/12/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 26/12/2024 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898145)
* 2025-01-02| Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633277/)
* 04/01/2025 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 06/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**logger.info(f"Don't Give your {secrets} away") por Tamar Galer (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/305045436)
* 07/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Microdosing Rust a su organización con Aviram Hassan (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/304883841)
* 08/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implementado en Rust con Ohad Dahan (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/304951805)
* 09/01/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 14/01/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/302815031)
* 14/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Una introducción a WASM en Rust con Márk Tolmács (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/305064546)
* 15/01/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)

### Asia
* 2025-01-12 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust enero 2025 en Abra en Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europa
* 18/12/2024 | Gante, BE | [Programación de Sistemas Gante](https://sysghent.be)
    * [**Lanzamiento de una nueva comunidad para desarrolladores de Rust y C++*](https://sysghent.be)
* 08/01/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 09/01/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 09/01/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)

### América del Norte
* 22/12/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo de Rust en Harvard Square, 22 de diciembre**](https://www.meetup.com/bostonrust/events/304951457)
* 26/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)
* 2025-01-10 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo de Rust de Lechmere, 10 de enero**](https://www.meetup.com/bostonrust/events/304951467)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> ¡¡Ella dijo que sí!! (¡Y yo también!)

– [Amos on Mastodon](https://hachyderm.io/@fasterthanlime/113639047728482697) demostrando que los rustáceos *sí* tienen una vida fuera de Rust. ¡Felicidades, Amos!

¡Gracias a [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1642) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1hhlwbf/this_week_in_rust_578_this_week_in_rust/)</small>
