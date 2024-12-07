---
title: "Esta semana en Rust #38"
number_of_week: 38
description: El crate de esta semana es augurs, un kit de herramientas de series temporales para Rust con enlaces a JS y Python.
date: 2024-12-04
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
* [Anunciando Rust 1.83.0](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html)
* [Actualización de la Iniciativa de Refactorización del Sistema de Rasgos de Rustc: Estabilizando '-Znext-solver=coherence' ](https://blog.rust-lang.org/inside-rust/2024/12/04/trait-system-refactor-initiative.html)
* [El objetivo wasm32-wasip2 ha alcanzado el soporte de nivel 2](https://blog.rust-lang.org/2024/11/26/wasip2-tier-2.html)
* [Encuesta Anual sobre el Estado de la Roya 2024](https://blog.rust-lang.org/2024/12/05/annual-survey-2024-launch.html)

### Actualizaciones de proyectos/herramientas
* [Medición y mejora del rendimiento multihilo de los susurros](https://rustls.dev/perf/2024-11-28-threading/)
* [Grupo: 0.15](https://bevyengine.org/news/bevy-0-15/)
* [Leptos 0.7.0](https://github.com/leptos-rs/leptos/releases/tag/v0.7.0)
* [Advenimiento de Rust 2024](https://www.rustfinity.com/advent-of-rust)
* [Presentación de Uniffi para React Native: módulos turbo impulsados por Rust](https://hacks.mozilla.org/2024/12/introducing-uniffi-for-react-native-rust-powered-turbo-modules/)
* [Revisando las configuraciones de aplicaciones de Hubris](https://cliffle.com/blog/exhubris/)
* [Enlaces RVKMS y Rust KMS](https://lwn.net/SubscriberLink/997850/8f1246199581a250/)
* [Discusión ininterrumpida sobre la adición de Rust a Git](https://lwn.net/SubscriberLink/998115/e9849d6de88348c6/)
* [Arquitectura del compilador incremental de Rust](https://lwn.net/SubscriberLink/997784/84e8aae50b88cca6/)
* [Novedades de SeaStreamer 0.5](https://www.sea-ql.org/blog/2024-11-30-whats-new-in-sea-streamer-0.5/)

### Observaciones/Pensamientos
* [¡Rotura! en el Cargo.toml — Cómo funcionan (y se rompen) las características del paquete Rust](https://predr.ag/blog/breakage-in-the-cargo-toml-how-rust-package-features-work/)
* [API de transmisión de audio en Rust pt. 4: El modelo](https://xd009642.github.io/2024/12/03/streaming-audio-APIs-the-model.html)
* [audio] [GitButler con Scott Chacon y Kiril Videlov](https://corrode.dev/podcast/s03e04-gitbutler/)
* [audio] [A different serde](https://sdr-podcast.com/episodes/a-different-serde/)

### Tutoriales de Rust
* [Desarrollo de un complemento de Zellij Rust](https://zellij.dev/tutorials/developing-a-rust-plugin/)
* [Aventuras de optimización: hacer que una carga de trabajo paralela de Rust sea aún más rápida con diseño orientado a datos (y otros trucos)](https://gendignoux.com/blog/2024/12/02/rust-data-oriented-design.html)
* [Diseñando un const 'array::from_fn' en Rust estable](https://13ros27.github.io/blog/const-array-from-fn/)
* [Implementación de APIs asíncronas para periféricos de microcontroladores](https://beaurivage.io/atsamd-hal-async/)
* [Rust 🦀 en el RP2040](https://baileytownsend.dev/articles/getting-started-with-rust-an-rp2040)
* [Creación de un chat en tiempo real usando WebSockets a través de flujos HTTP/2](https://c410-f3r.github.io/thoughts/building-a-real-time-chat-using-web-sockets-over-http2-streams)
* [¡Ejecutando Bevy en un Web Worker con Renderizado y Física!](https://allwright.io/#/blog/20241127-bevy-webworker.md)
* [Empaquetando una biblioteca de Rust como XCFramework para iOS](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-2/)

## Crate de la semana

El crate de esta semana es [augurs](https://github.com/grafana/augurs), un kit de herramientas de series temporales para Rust con enlaces a JS y Python.

¡Gracias a [Ben Sully](https://users.rust-lang.org/t/crate-of-the-week/2704/1379) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

* [Rama — soporta la capa del exportador HAR (http) en rama](https://github.com/plabayo/rama/issues/357)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

* [Semana del Rust 2025](https://www.papercall.io/rust-week) | Cierra 12/01/2025 | Utrecht, Países Bajos | Fecha del evento: 2025-05-13

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 488 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-26..2024-12-03

* ['rust_analyzer_settings': uso forzado de la cadena de herramientas 'nocturna'](https://github.com/rust-lang/rust/pull/133712)
* [añadir la directiva 'needs-target-has-atomic'](https://github.com/rust-lang/rust/pull/133736)
* [permitir inyectar un tiempo de ejecución del generador de perfiles en '#! [no_core]' cajas](https://github.com/rust-lang/rust/pull/133369)
* [Rescate de más errores en la reducción de la dinámica](https://github.com/rust-lang/rust/pull/133394)
* [Mejor diagnóstico para los ítems FN en funciones variádicas](https://github.com/rust-lang/rust/pull/133538)
* [cambia la antigua declaración intrínseca por una nueva declaración](https://github.com/rust-lang/rust/pull/133106)
* [marque 'xform_ret_ty' para WF en el nuevo solucionador para mejorar el aventado del método](https://github.com/rust-lang/rust/pull/133519)
* [verifique la fuente antes de sugerir una anotación](https://github.com/rust-lang/rust/pull/133691)
* [Comprobar la caché local incluso si se puede usar global](https://github.com/rust-lang/rust/pull/133626)
* [limpieza: eliminar la directiva '//@ bastante expandida'](https://github.com/rust-lang/rust/pull/133470)
* [constificar 'Soltar' y 'Destruir'](https://github.com/rust-lang/rust/pull/133402)
* [coverage: almacena las regiones de origen de cobertura como 'Span' hasta codegen](https://github.com/rust-lang/rust/pull/133418)
* [cobertura: use una consulta para identificar qué ID de contador/expresión se utilizan](https://github.com/rust-lang/rust/pull/133446)
* [retrasar un error al encontrar un impl con genéricos sin restricciones en 'codegen_select'](https://github.com/rust-lang/rust/pull/133368)
* [deshabilitar 'avr-rjmp-offset' en Windows por ahora](https://github.com/rust-lang/rust/pull/133481)
* [no llamar 'extern_crate' en el rasgo actual en errores de discordancia de caja](https://github.com/rust-lang/rust/pull/133585)
* [no restringir inferir vars en 'find_best_leaf_obligation'](https://github.com/rust-lang/rust/pull/133493)
* [No crear un tipo de objeto de rasgo si faltan tipos asociados](https://github.com/rust-lang/rust/pull/133660)
* [no unificar las desreferencias de préstamos compartidos en GVN](https://github.com/rust-lang/rust/pull/133474)
* [no escriba error si no logramos forzar 'Pin<T>' porque no contiene una referencia](https://github.com/rust-lang/rust/pull/133358)
* [eliminar los números mágicos de la precedencia de las expresiones](https://github.com/rust-lang/rust/pull/133603)
* [enable '-Zshare-generics' para funciones 'inline(never)'](https://github.com/rust-lang/rust/pull/123244)
* [asegúrese de que los objetivos definidos por JSON sean coherentes](https://github.com/rust-lang/rust/pull/133409)
* [Rechazo rápido: agregar verificación de profundidad](https://github.com/rust-lang/rust/pull/133566)
* [arreglar ICE cuando promocionado tiene un desbordamiento de tamaño de diseño](https://github.com/rust-lang/rust/pull/133704)
* [fix '-Zdump-mir-dataflow'](https://github.com/rust-lang/rust/pull/133732)
* [arreglar 'clobber_abi' en el ensamblaje en línea RV32E y RV64E](https://github.com/rust-lang/rust/pull/133422)
* [arreglar diagnóstico confuso para reservado '##'](https://github.com/rust-lang/rust/pull/133487)
* [arreglar el manejo de x18 en el ensamblaje en línea AArch64 en ohos/trusty o con -Zfixed-x18](https://github.com/rust-lang/rust/pull/133463)
* [GCE: arreglar el desajuste de 'typing_mode'](https://github.com/rust-lang/rust/pull/133471)
* [deshacerse del verificador de const de HIR](https://github.com/rust-lang/rust/pull/133321)
* [mejorar el manejo del tramo en 'parse_expr_bottom'](https://github.com/rust-lang/rust/pull/133623)
* [Mejoras en las lógicas iniciales de búsqueda de sysroot y libdir](https://github.com/rust-lang/rust/pull/132782)
* [hacer que 'adjust_fulfillment_errors' funcione con 'HostEffectPredicate' y 'const_conditions'](https://github.com/rust-lang/rust/pull/133403)
* [convertir 'compare_impl_item' en una consulta](https://github.com/rust-lang/rust/pull/133365)
* [solo error raw lifetime seguido de '\'' en la edición 2021+](https://github.com/rust-lang/rust/pull/133482)
* [solo ignorar windows-gnu en avr-jmp-offset](https://github.com/rust-lang/rust/pull/133513)
* [Imprimir rutas de documentos generadas](https://github.com/rust-lang/rust/pull/133550)
* [pasar correctamente los argumentos del enlazador que contienen comas](https://github.com/rust-lang/rust/pull/132974)
* [Respetar la opción verify-llvm-ir en el backend](https://github.com/rust-lang/rust/pull/133499)
* [robustificar y generar la resolución de notación de tipo de retorno en 'resolve_bound_vars'](https://github.com/rust-lang/rust/pull/132047)
* [mostrar 'forbidden_lint_groups' en informes de futuras comparaciones](https://github.com/rust-lang/rust/pull/133535)
* [soporte 'clobber_abi' en el ensamblaje en línea AVR](https://github.com/rust-lang/rust/pull/131323)
* [soporte de entrada/salida en registros vectoriales del ensamblaje en línea de PowerPC](https://github.com/rust-lang/rust/pull/131551)
* [admite registros de predicados (solo clobber) en el ensamblado en línea de Hexagon](https://github.com/rust-lang/rust/pull/133452)
* [apoyo que revela la definición de un post opaco prestado](https://github.com/rust-lang/rust/pull/133501)
* [target 'check_consistency': asegúrese de que la cadena de características de destino tenga algún sentido básico](https://github.com/rust-lang/rust/pull/133410)
* [el sistema operativo emscripten ya no existe en objetivos que no sean wasm](https://github.com/rust-lang/rust/pull/133411)
* [use la edición de 'macro_rules' al compilar la macro](https://github.com/rust-lang/rust/pull/133274)
* [use almacenes del tamaño correcto para establecer discriminantes](https://github.com/rust-lang/rust/pull/131698)
* [miri: implement 'TlsFree'](https://github.com/rust-lang/rust/pull/133457)
* [Miri: Soporte del sistema de archivos para Solarish: stat](https://github.com/rust-lang/miri/pull/4031)
* [miri: mover FdTable a una ubicación común y separar el comportamiento de Unix](https://github.com/rust-lang/miri/pull/4045)
* [Miri: eliminar ctrlc, sin usar](https://github.com/rust-lang/miri/pull/4064)
* [deja de clonar tanto 'Context'](https://github.com/rust-lang/rust/pull/133345)
* [recuperar algo de rendimiento perdido](https://github.com/rust-lang/rust/pull/133509)
* [estabilizar 'const_maybe_uninit_write'](https://github.com/rust-lang/rust/pull/131713)
* [estabilizar 'extended_varargs_abi_support'](https://github.com/rust-lang/rust/pull/116161)
* [estabilizar 'ptr::fn_addr_eq'](https://github.com/rust-lang/rust/pull/133678)
* [estabilizar las variantes sin signo y flotantes de la función 'num_midpoint'](https://github.com/rust-lang/rust/pull/131784)
* ['thread::available_parallelism' para wasm32-wasip1-threads](https://github.com/rust-lang/rust/pull/133496)
* [agregar API de entrada 'BTreeSet' para que coincidan con 'HashSet'](https://github.com/rust-lang/rust/pull/133548)
* [btree: add '{Entry,VacantEntry}::insert_entry'](https://github.com/rust-lang/rust/pull/133042)
* [arreglar el encadenamiento de 'carrying_add's](https://github.com/rust-lang/rust/pull/133674)
* [marque 'slice::copy_from_slice' de manera inestable const](https://github.com/rust-lang/rust/pull/131416)
* [std: exponer 'const_io_error!' como 'const_error!'](https://github.com/rust-lang/rust/pull/133449)
* [std: Refactorizar la sincronización basada en 'pthread'](https://github.com/rust-lang/rust/pull/128184)
* [arreglar y dejar de usar 'home_dir()'](https://github.com/rust-lang/rust/pull/132515)
* [rangos de soporte en '<[T]>::get_many_mut()'](https://github.com/rust-lang/rust/pull/133136)
* [cargo: toml: Permitir agregar/eliminar de los scripts de carga](https://github.com/rust-lang/cargo/pull/14857)
* [cargo: build-std: siempre enlazar a std al probar proc-macros](https://github.com/rust-lang/cargo/pull/14850)
* [cargo: fix: Migrar manifiestos de script de carga a través de ediciones](https://github.com/rust-lang/cargo/pull/14864)
* [cargo: build-std: descargar deps primero](https://github.com/rust-lang/cargo/pull/14861)
* [cargo: pgo: determinar la capacidad de ejecución de la prueba en tiempo de compilación](https://github.com/rust-lang/cargo/pull/14874)
* [cargo: pgo: asegúrese de que PGO funcione](https://github.com/rust-lang/cargo/pull/14859)
* [Cargo: PGO: solo se ejecuta todas las noches](https://github.com/rust-lang/cargo/pull/14887)
* [cargo: add future-incompat warning against keywords in cfgs and add raw-idens](https://github.com/rust-lang/cargo/pull/14671)
* [cargo: corrección: eliminar la referencia de registro predeterminada en los documentos cmd 'info'](https://github.com/rust-lang/cargo/pull/14880)
* [cargo: git-fetch-with-cli: set 'GIT_DIR' para compatibilidad con repositorios desnudos](https://github.com/rust-lang/cargo/pull/14860)
* [cargo: test: el atributo 'requires' acepta literales de cadena para cmds](https://github.com/rust-lang/cargo/pull/14875)
* [rustdoc-json: incluir la seguridad de 'estático's](https://github.com/rust-lang/rust/pull/133715)
* [rustdoc: Cambiar sangría de elementos impl](https://github.com/rust-lang/rust/pull/131718)
* [bindgen: añadir soporte para bloques externos inseguros](https://github.com/rust-lang/rust-bindgen/pull/3015)
* [bindgen: consolidar la gestión de versiones de dependencias](https://github.com/rust-lang/rust-bindgen/pull/3008)
* [bindgen: extender las devoluciones de llamada de análisis para exponer los tipos compuestos y alias descubiertos](https://github.com/rust-lang/rust-bindgen/pull/2658)
* [bindgen: generate C-String literals 'c"example"' en lugar de código inseguro](https://github.com/rust-lang/rust-bindgen/pull/2996)
* [bindgen: mejorar el generador de str de depuración](https://github.com/rust-lang/rust-bindgen/pull/3010)
* [bindgen: introduce '--rust-edition'](https://github.com/rust-lang/rust-bindgen/pull/3002)
* [Bindgen: Usar resolución de carga v2](https://github.com/rust-lang/rust-bindgen/pull/2999)
* [bindgen: envuelve la representación de la matriz de tipos opacos en una estructura '#[repr(C)]]](https://github.com/rust-lang/rust-bindgen/pull/2880)
* [rustfmt: use el símbolo de 'ruta' preinternado](https://github.com/rust-lang/rustfmt/pull/6404)
* [clippy: 'bad_bit_mask' Arreglar falsos positivos en macros proc](https://github.com/rust-lang/rust-clippy/pull/13736)
* [clippy: 'doc_nested_refdefs': nuevo lint para sintaxis de lista sospechosa](https://github.com/rust-lang/rust-clippy/pull/13707)
* [clippy: añadir más casos a la pelusa 'useless_conversion'](https://github.com/rust-lang/rust-clippy/pull/13756)
* [clippy: añadir nueva pelusa 'literal_string_with_formatting_args'](https://github.com/rust-lang/rust-clippy/pull/13410)
* [clippy: arreglar 'needless_match' FP en if-lets](https://github.com/rust-lang/rust-clippy/pull/13646)
* [clippy: arreglar el comportamiento de 'shadow_unrelated' con los cierres](https://github.com/rust-lang/rust-clippy/pull/13677)
* [clippy: arreglar la sugerencia de elisión de vidas en las cláusulas where](https://github.com/rust-lang/rust-clippy/pull/13752)
* [clippy: corrección: use 'multipart_suggestion' para 'derivable_impls'](https://github.com/rust-lang/rust-clippy/pull/13717)
* [clippy: maneja la repetición de la restricción constante asociada también](https://github.com/rust-lang/rust-clippy/pull/13723)
* [Rust-analyzer: Anunciar finalizaciones e inserciones Sugerencias resuelve las capacidades del servidor en función de las capacidades del cliente](https://github.com/rust-lang/rust-analyzer/pull/18589)
* [rust-analyzer: arreglar la consulta de configuración de depuración que no hereda el entorno](https://github.com/rust-lang/rust-analyzer/pull/18586)
* [rust-analyzer: arreglar la corrección de sintaxis insertando puntos y comas innecesarios](https://github.com/rust-lang/rust-analyzer/pull/18587)
* [rust-analyzer: re-add 'rust-analyzer.cargo.sysrootQueryMetadata'](https://github.com/rust-lang/rust-analyzer/pull/18511)
* [rust-analyzer: eliminar los límites de tipo asociados redundantes de 'dyn TypeFolder'](https://github.com/rust-lang/rust-analyzer/pull/18577)

### Clasificación del rendimiento del compilador de Rust

Semana ajetreada con más PR que afectan al rendimiento de lo habitual. Afortunadamente, las mejoras de rendimiento superaron las regresiones en los puntos de referencia del mundo real, y la mayor ganancia de rendimiento individual provino de un cambio para dejar de realizar incondicionalmente la verificación de LLVM IR en las compilaciones de depuración, lo que fue solo un trabajo desperdiciado.

Triaje realizado por **@rylev**.
Rango de revisión: [7db7489f.. 490b2cc0](https://perf.rust-lang.org/?start=7db7489f9bc274cb60c4956bfa56de0185eb1b9b&end=490b2cc09860dd62a7595bb07364d71c12ce4e60&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.2%, 1.9%] | 58 |
| Regresiones ❌ <br /> (secundaria) | 1.1% | [0.2%, 5.1%] | 85 |
| Mejoras ✅ <br /> (primario) | -2,3% | [-8.2%, -0.2%] | Artículo 116 |
| Mejoras ✅ <br /> (secundaria) | -2,5% | [-8.9%, -0.1%] | 55 |
| Todos ❌✅ (primarios) | -1,4% | [-8,2%, 1,9%] | 174 |

6 regresiones, 6 mejoras, 5 mixtas; 5 de ellos en rollups
49 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/fcd028e6e8117a881b7ffab448f549410c1c0dde/triage/2024-12-03.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [crates.io: Eliminar las eliminaciones de versiones del RFC de "eliminaciones de cajas"](https://github.com/rust-lang/rfcs/pull/3731)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [crates.io: Trusted Publishing Support](https://github.com/rust-lang/rfcs/pull/3691)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Arreglar ICE cuando se necesitan varias sustituciones de superrasgos pero solo se proporciona una](https://github.com/rust-lang/rust/pull/133392)
* [disposición: fusionar] [reelaborar el aventamiento para manejar con sensatez los límites globales de dónde](https://github.com/rust-lang/rust/pull/132325)
* [disposición: fusionar] [Mostrar siempre la primera línea de los bloques impl incluso cuando están colapsados](https://github.com/rust-lang/rust/pull/132155)
* [disposición: fusionar] [Problema de seguimiento para 'const_nonnull_new'](https://github.com/rust-lang/rust/issues/93235)
* [disposición: fusionar] [Lint al combinar '#[no_mangle]' y '#[export_name]'](https://github.com/rust-lang/rust/pull/131558)
* [disposición: fusionar] [Agregar implicaciones de extensión para tuplas de aridad 1 a 12](https://github.com/rust-lang/rust/pull/132187)
* [disposición: fusionar] [[discusión] 'ErrorKind::FilesystemQuotaExceeded' de 'io_error_more'](https://github.com/rust-lang/rust/issues/130190)
* [disposición: fusionar] [[discusión] 'ErrorKind::CrossesDevices' de 'io_error_more'](https://github.com/rust-lang/rust/issues/130191)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [[RFC] proyecciones de campo v2](https://github.com/rust-lang/rfcs/pull/3735)

## Próximos eventos

Eventos oxidados entre 2024-12-04 - 2025-01-01 🦀

### Virtual
* 05/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633275/)
* 05/12/2024 | Virtual (Miami, FL) | [Eventos del grupo de usuarios de Java en Miami](https://www.meetup.com/miami-java-user-group)
    * [**Introducción a Rust para desarrolladores de Java**](https://www.meetup.com/miami-java-user-group/events/304717903/)
* 06/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304369723/)
* 07/12/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 08/12/2024 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Leyendo archivos JSON en Rust - קריאת קבצי ג'ייסון בראסט**](https://www.meetup.com/rust-tlv/events/304685546/)
* 2024-12-10 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346988/)
* 11/12/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**egui**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Reunión de Rust en Seattle](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Encuentro de diciembre**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 12/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 12/12/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 13/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 17/12/2024 | Virtual (San Francisco, CA, EE. UU.) | [Centro Blockchain SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust en Web3: Serie para desarrolladores**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 17/12/2024 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Lectura del código fuente de Rust: La caja de los miles (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/304824684/)
* 17/12/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346972/)
* 17/12/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en códigos](https://www.meetup.com/code-mavens/)
    * [**Lectura del código fuente de Rust: La caja de los miles**](https://www.meetup.com/code-mavens/events/304824684/)
* 19/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633276/)
* 19/12/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 20/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 24/12/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 26/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)

### África
* 2024-12-10 | Johannesburgo, ZA | [Reunión de Rust en Johannesburgo](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hola Mundo... otra vez**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)

### Asia
* 14/12/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro/taller rustáceo de diciembre de 2024**](https://hasgeek.com/rustbangalore/december-2024-rustacean-meetup-workshop/)

### Europa
* 04/12/2024 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprenem junts Rust / Aprendan Rust Together**](https://lu.ma/pypwr0m7)
* 04/12/2024 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en diciembre: Advenimiento del código**](https://www.meetup.com/rustcologne/events/304760521/)
* 04/12/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 04/12/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #73**](https://www.meetup.com/rust-paris/events/304685955/)
* 05/12/2024 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #6**](https://www.meetup.com/rust-gdansk/events/304773705/)
* 05/12/2024 | Zlin, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Encuentro de Rust Moravia (diciembre de 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 06/12/2024 | Moscú, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Rusia**](https://rustcon.ru/)
* 11/12/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 12/12/2024 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 12/12/2024 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2024 / 4 - Noche de hacking**](https://www.meetup.com/rust-munich/events/304827279/)
* 12/12/2024 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/events/)
    * [**Rust Viena - Diciembre | en Sentry.io 🦀 **](https://www.meetup.com/rust-vienna/events/304815850/)
* 17/12/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Amigo invisible en Rust: Desenvolviendo pruebas de propiedades**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### América del Norte
* 05/12/2024 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Mensual de Diciembre**](https://www.meetup.com/rust-montreal/events/304778367/)
* 05/12/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cuerdas de Rust**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/)
    * [**Encuentro de diciembre**](https://www.meetup.com/join-srug/events/304806455/)
* 12/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 16/12/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 17/12/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 26/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)

### Oceanía
* 04/12/2024 | Sídney, Australia | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 08/12/2024 | Canberra, Australia | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Fiesta de Navidad CRUG**](https://www.meetup.com/rust-canberra/events/304282046/)
* 16/12/2024 | Collingwood, Australia | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Reunión de Rust Melbourne de diciembre de 2024**](https://www.meetup.com/rust-melbourne/events/304820598/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> "uno mismo" suena como una cosa de Rust

– [ionchy sobre Mastodon](https://types.pl/@ionchy/113567387219906256)

¡Gracias a [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1636) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1h7jacb/this_week_in_rust_576/)</small>
