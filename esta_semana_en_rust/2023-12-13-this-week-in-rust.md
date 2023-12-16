---
title: "Esta semana en Rust #10"
number_of_week: 10
description: El crate de esta semana es io-adapters.
date: 2023-12-13
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

### Oficial
* [Anunciando Rust 1.74.1](https://blog.rust-lang.org/2023/12/07/Rust-1.74.1.html)
* [Limpieza de caché de carga](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html)

### Boletines informativos
* [Este mes en Rust OSDev: noviembre de 2023](https://rust-osdev.com/this-month/2023-11/)

### Actualizaciones de proyectos/herramientas
* [registro de cambios de rust-analyzer #211](https://rust-analyzer.github.io/thisweek/2023/12/11/changelog-211.html)
* [Generador de música para PC](https://hjvt.dev/posts/5)
* [Anuncio de mfio - E/S de finalización para todos](https://blaz.is/blog/post/mfio-release/)
* [Watchexec Library 3.0 y CLI 1.24](https://cohost.org/watchexec/post/3818997-watchexec-library-3)
* [serie] [Inspirado en Spotify: Elevando Meilisearch con Hybrid Search y Rust](https://blog.kerollmops.com/spotify-inspired-elevating-meilisearch-with-hybrid-search-and-rust)

### Observaciones/Pensamientos
* [Rust Is Beyond Object-Oriented, Part 3: Inheritance](https://www.thecodedmessage.com/posts/oop-3-inheritance/)
* [Ser Rusty: Descubriendo los axiomas de diseño de Rust](https://smallcultfollowing.com/babysteps/blog/2023/12/07/rust-design-axioms/)
* [¿Cuándo no se envían futuros?](https://matklad.github.io/2023/12/10/nsfw.html)
* ['por esperar' y la batalla de los arroyos amortiguados](https://tmandry.gitlab.io/blog/posts/for-await-buffered-streams/)
* [poll_progress](https://without.boats/blog/poll-progress/)
* [Rust y ThreadX - experimentos con un RTOS escrito en C, un antiguo componente de software certificado](https://ferrous-systems.com/blog/rust-and-threadx/)
* [Nueve reglas para la aceleración SIMD de su código Rust (Parte 1): Lecciones generales de aumentar la ingesta de datos en la caja range-set-fire en 7x](https://towardsdatascience.com/nine-rules-for-simd-acceleration-of-your-rust-code-part-1-c16fe639ce21)
* [Contribuyendo a Rust como novato](https://ochagavia.nl/blog/someone-interviewed-me/)
* [audio] [Explorando el impacto de Rust en la eficiencia y el ahorro de costes, con Stefan Baumgartner](https://rustacean-station.org/episode/stefan-baumgartner-rust-efficiency/)

### Tutoriales de Rust
* [Errores comunes con Rust Async](https://www.qovery.com/blog/common-mistakes-with-rust-async/)
* [Embajada en ESP: Transmisor UART](https://apollolabsblog.hashnode.dev/embassy-on-esp-uart-transmitter)
* [Escribir una herramienta CLI en Rust con Clap](https://www.shuttle.rs/blog/2023/12/08/clap-rust)
* [Memoria e iteración](https://yetanotherrustblog.net/memory-and-iteration/)
* [Primeros pasos con Axum - El framework web más popular de Rust](https://www.shuttle.rs/blog/2023/12/06/using-axum-rust)
* [Exploración del SDK de AWS Lambda en Rust](https://gruebelinchen.wordpress.com/2023/12/07/exploring-the-aws-lambda-sdk-in-rust/)
* [Práctico Rust del lado del cliente para Android, iOS y Web](https://www.mux.com/blog/practical-client-side-rust-for-android-ios-and-web)
* [video] [Advenimiento del Código 2023](https://www.youtube.com/playlist?list=PLWtPciJ1UMuD3_8Pb-EqrFhkYpastR2cn)

### Miscelánea
* [Turbofish ::<>](https://rust.code-maven.com/turbofish)
* [Rust Meetup y grupos de usuarios](https://rust.code-maven.com/meetups)
* [Adoptando Rust: el libro de jugadas que faltaba para gerentes y CTO](https://mainmatter.com/blog/2023/12/13/rust-adoption-playbook-for-ctos-and-engineering-managers/)
* [SemanticDiff 0.8.8 añade soporte para Rust](https://semanticdiff.com/blog/semanticdiff-0.8.8/)

## Crate de la semana

El crate de esta semana es [io-adapters](https://github.com/SUPERCILEX/io-adapters), una caja que te permite convertir entre diferentes API grabables ('io' vs. 'fmt').

¡Gracias a [Alex Saveau](https://users.rust-lang.org/t/crate-of-the-week/2704/1271) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [greptimedb - Mejora de la API para el resultado de la consulta sql de impresión bonita en la salida http 1](https://github.com/GreptimeTeam/greptimedb/issues/2877)
* [greptimedb - Unificar patrones de constructores](https://github.com/GreptimeTeam/greptimedb/issues/2853)
* [tokio - Ejecutar pruebas de telar en oss-fuzz 4](https://github.com/tokio-rs/tokio/issues/6208)
* [Ockam - Biblioteca - Validar estructuras CBOR de acuerdo con el esquema cddl para 'kafka/protocol_aware' y 'nodes/services'](https://github.com/build-trust/ockam/issues/6689)
* [Ockam - Comando - refactorizar para usar interfaces tipadas para implementar comandos para 'relés'](https://github.com/build-trust/ockam/issues/6704)
* [Ockam - Haz que install.sh no falle cuando la última versión ya está instalada](https://github.com/build-trust/ockam/issues/7118)
* [zerocopy - Use cargo-semver-checks para asegurarse de que la función 'derivar' no cambie la superficie de la API](https://github.com/google/zerocopy/issues/422)
* [zerocopy - Verifique que el trabajo de CI 'all-jobs-succeeded' dependa de todos los demás trabajos](https://github.com/google/zerocopy/issues/444)
* [Hyperswitch - [REFACTOR]: [Nuvei] Validación de metadatos MCA](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [Característica]: [Mediodía] Sincronización con Referencia de Hyperswitch](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [BUG] : Los errores de deserialización de metadatos MCA deben ser 4xx](https://github.com/juspay/hyperswitch/issues/2899)
* [Hyperswitch - [Característica]: [Zen] Sincronización con Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

391 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-04..2023-12-11

* [introducir soporte para bloques 'async gen'](https://github.com/rust-lang/rust/pull/118420)
* [implementar RFC de reglas de captura de por vida de la edición 2024](https://github.com/rust-lang/rust/pull/116952)
* [Soporte de plataforma 'RISCV32'](https://github.com/rust-lang/rust/pull/117874)
* [añadir teeos std impl](https://github.com/rust-lang/rust/pull/116565)
* ['never_patterns': Analizar los brazos sin cuerpo](https://github.com/rust-lang/rust/pull/118527)
* ['rustc_symbol_mangling,rustc_interface,rustc_driver_impl': Aplicar 'rustc::p otential_query_instability' lint](https://github.com/rust-lang/rust/pull/118637)
* [añadir información de la variante ADT a StableMIR y terminar de implementar 'TyKind::internal()'](https://github.com/rust-lang/rust/pull/118516)
* [añadir 'deeply_normalize_for_diagnostics', usarlo en coherencia](https://github.com/rust-lang/rust/pull/118346)
* [añadir comentario sobre mantener las banderas sincronizadas entre bootstrap.py y bootstrap.rs](https://github.com/rust-lang/rust/pull/118650)
* [añadir compatibilidad con TLS emulado](https://github.com/rust-lang/rust/pull/117873)
* [agregar evaluación de instancia y métodos para leer una asignación en StableMIR](https://github.com/rust-lang/rust/pull/118694)
* [añadir lint contra comparaciones ambiguas de puntero ancho](https://github.com/rust-lang/rust/pull/117758)
* [agregar método para obtener el tipo de un valor R en StableMIR](https://github.com/rust-lang/rust/pull/118688)
* [añadir más intrínsecos a la plataforma SIMD](https://github.com/rust-lang/rust/pull/117953)
* [añadir opciones de compilación segura](https://github.com/rust-lang/rust/pull/117966)
* [añadir soporte para 'gen fn'](https://github.com/rust-lang/rust/pull/118457)
* [Agregar soporte para hacer que las características de la biblioteca sean internas](https://github.com/rust-lang/rust/pull/118123)
* [Se ha añadido una sugerencia sombreada para la superposición de tipos asociados](https://github.com/rust-lang/rust/pull/117661)
* [evitar añadir funciones internas a 'symbols.o'](https://github.com/rust-lang/rust/pull/118568)
* [evite instanciar infer vars con infer](https://github.com/rust-lang/rust/pull/118710)
* [cambiar la captura previa para evitar el interbloqueo](https://github.com/rust-lang/rust/pull/118488)
* [Evaluación en tiempo de compilación: detectar escrituras a través de punteros inmutables](https://github.com/rust-lang/rust/pull/118324)
* [Cobertura: Sé más estricto con lo que cuenta como una "macro visible"](https://github.com/rust-lang/rust/pull/118595)
* [Cobertura: Fusionar intervalos refinados en una pasada final separada](https://github.com/rust-lang/rust/pull/118695)
* [Cobertura: Simplificar la heurística para ignorar los intervalos de retorno 'async Fn'](https://github.com/rust-lang/rust/pull/118666)
* [cobertura: use 'SpanMarker' para mejorar los intervalos de cobertura para las expresiones 'if!](https://github.com/rust-lang/rust/pull/118198)
* [desduplicado para sugerencias duplicadas](https://github.com/rust-lang/rust/pull/118057)
* [descartar intervalos no válidos en bloques externos](https://github.com/rust-lang/rust/pull/116420)
* [No poner entre paréntesis la 'estructura' exterior iluminada dentro de los protectores de partido](https://github.com/rust-lang/rust/pull/118726)
* [no incluir alcances de destrucción en THIR](https://github.com/rust-lang/rust/pull/116170)
* [no imprima el parámetro del efecto anfitrión en pretty 'path_generic_args'](https://github.com/rust-lang/rust/pull/118788)
* [no advertir que un patrón vacío es inalcanzable si no estamos seguros de que los datos sean válidos](https://github.com/rust-lang/rust/pull/118308)
* [forzar 'must_use' en los tipos asociados y RPITIT que tienen un rasgo de uso obligatorio en los límites](https://github.com/rust-lang/rust/pull/118504)
* [implementar explícitamente 'DynSync' y 'DynSend' para 'TyCtxt'](https://github.com/rust-lang/rust/pull/117681)
* [corregir 'is_foreign_item' para la instancia StableMIR](https://github.com/rust-lang/rust/pull/118681)
* [arreglar la comprobación de caída de const](https://github.com/rust-lang/rust/pull/118689)
* [arreglar la recopilación in situ que no se reasigna cuando es necesario](https://github.com/rust-lang/rust/pull/118460)
* [arreglar el ICE del analizador sintáctico al recuperar 'dyn'/'impl' después de 'for<...>'](https://github.com/rust-lang/rust/pull/118585)
* [corrección: corrija el arg para el diagnóstico 'sugerir usar sintaxis de función asociada'](https://github.com/rust-lang/rust/pull/118502)
* [generalizar el uso de LLD en bootstrap](https://github.com/rust-lang/rust/pull/116278)
* [Generalizar: el identificador produce un error de verificación en los alias](https://github.com/rust-lang/rust/pull/117088)
* [implementar el indicador del compilador '--env' (sin soporte para 'tracked_env')](https://github.com/rust-lang/rust/pull/118368)
* [implementar 'repr(packed)' para 'repr(simd)'](https://github.com/rust-lang/rust/pull/117116)
* [mejorar 'print_tts'](https://github.com/rust-lang/rust/pull/114571)
* [interpretar: hacer accesible 'numeric_intrinsic' desde Miri](https://github.com/rust-lang/rust/pull/118565)
* [Hacer que los generadores asíncronos se fusionen de forma predeterminada](https://github.com/rust-lang/rust/pull/118764)
* [asegúrese de que 'panic_nounwind_fmt' aún se pueda insertar completamente (por ejemplo, para 'panic_immediate_abort)'](https://github.com/rust-lang/rust/pull/118362)
* [solo verifique la referencia del rasgo principal para la seguridad del objeto](https://github.com/rust-lang/rust/pull/118686)
* [bonitas referencias de rasgos 'Fn<(..., ...)>' con paréntesis (casi) siempre](https://github.com/rust-lang/rust/pull/118268)
* [Privacidad: Visitar rasgo def id de proyecciones](https://github.com/rust-lang/rust/pull/118715)
* [proporcionar contexto cuando no se puede llamar a '?' debido a 'Result<_, E>'](https://github.com/rust-lang/rust/pull/116496)
* [reorganizar 'default_configuration' y 'CheckCfg::fill_well_known'](https://github.com/rust-lang/rust/pull/118494)
* [Recurre a las referencias cuando se comparan los TYS para el diagnóstico](https://github.com/rust-lang/rust/pull/118730)
* [eliminar 'PolyGenSig' ya que siempre es una carpeta ficticia](https://github.com/rust-lang/rust/pull/118684)
* [eliminar la puerta de características 'precise_pointer_size_matching'](https://github.com/rust-lang/rust/pull/118598)
* [Resolver enlaces de elementos asociados por espacio de nombres](https://github.com/rust-lang/rust/pull/118668)
* [optimizar los cursores de flujo de datos MIR](https://github.com/rust-lang/rust/pull/118230)
* [sugerencia estructurada de 'uso' sobre error de privacidad](https://github.com/rust-lang/rust/pull/118066)
* [decirle a MirUsedCollector que las comprobaciones de alineación del puntero llaman a su símbolo de pánico](https://github.com/rust-lang/rust/pull/118693)
* [sugerencia para definir el nombre de la macro después de 'macro_rules!'](https://github.com/rust-lang/rust/pull/118317)
* [modificar la sugerencia '.clone()' para que funcione en más casos](https://github.com/rust-lang/rust/pull/118076)
* [Modificar errores genéricos no cerrados](https://github.com/rust-lang/rust/pull/117922)
* [limpiezas sin escape](https://github.com/rust-lang/rust/pull/118734)
* [eleva el canonicalizador (nuevo solucionador) a 'rustc_next_trait_solver'](https://github.com/rust-lang/rust/pull/117586)
* [use 'immediate_backend_type' cuando lea de una asignación const](https://github.com/rust-lang/rust/pull/118791)
* [use los parámetros predeterminados hasta que los efectos se desazucaren](https://github.com/rust-lang/rust/pull/118608)
* [Miri: arreglar prometiendo una alineación muy grande](https://github.com/rust-lang/miri/pull/3211)
* [miri: fix x86 SSE4.1 ptestnzc](https://github.com/rust-lang/miri/pull/3216)
* [Miri: mover algo de código intrínseco x86 a funciones auxiliares en 'shims::x86'](https://github.com/rust-lang/miri/pull/3214)
* [miri: devuelve 'MAP_FAILED' cuando falla mmap](https://github.com/rust-lang/miri/pull/3219)
* [stablize 'arc_unwrap_or_clone'](https://github.com/rust-lang/rust/pull/116949)
* [add 'LinkedList::{retain,retain_mut}'](https://github.com/rust-lang/rust/pull/114136)
* [simplificar por defecto para tuplas](https://github.com/rust-lang/rust/pull/118350)
* [restore 'const PartialEq'](https://github.com/rust-lang/rust/pull/118661)
* [dividir 'Vec::d edup_by' en 2 ciclos](https://github.com/rust-lang/rust/pull/118273)
* [futuros: fillBuf: no igualar 'poll_fill_buf' dos veces](https://github.com/rust-lang/futures-rs/pull/2812)
* [futuros: 'FuturesOrdered': usa un índice de 64 bits](https://github.com/rust-lang/futures-rs/pull/2810)
* [futures: 'FuturesUnordered': corrige una implementación clara](https://github.com/rust-lang/futures-rs/pull/2809)
* [futuros: use 'cfg(target_has_atomic)' en objetivos sin ETS](https://github.com/rust-lang/futures-rs/pull/2811)
* [cargo: spec: Extend PackageIdSpec with source kind + git ref for unambigys specs](https://github.com/rust-lang/cargo/pull/12933)
* [cargo toml: no permitir la herencia de la condición de público de dependencia](https://github.com/rust-lang/cargo/pull/13125)
* [cargo toml: no permitir '[lints]' en espacios de trabajo virtuales](https://github.com/rust-lang/cargo/pull/13155)
* [cargo: schema: Eliminar la dependencia de los tipos de carga](https://github.com/rust-lang/cargo/pull/13154)
* [cargo: schemas: Pull out mod for proposed schemas package](https://github.com/rust-lang/cargo/pull/13097)
* [cargo: trim-paths: assert 'OSO' y 'SO' no pueden ser recortados](https://github.com/rust-lang/cargo/pull/13118)
* [cargo: evite escribir CACHEDIR. TAG si ya existe](https://github.com/rust-lang/cargo/pull/13132)
* [cargo: arreglar la finalización de bash en el directorio con espacios](https://github.com/rust-lang/cargo/pull/13126)
* [cargo: reasignar explícitamente el directorio actual usando '.'](https://github.com/rust-lang/cargo/pull/13114)
* [Cargo: Imprimir mensajes de RUSTC coloreados en Wincon](https://github.com/rust-lang/cargo/pull/13140)
* [cargo: limitar los lints de dependencias privadas exportadas a bibliotecas](https://github.com/rust-lang/cargo/pull/13135)
* [rustdoc-search: no tratar los nombres de tipos asociados como tipos](https://github.com/rust-lang/rust/pull/118812)
* [rustdoc: No generar el encabezado "Campos" si no se muestra ningún campo](https://github.com/rust-lang/rust/pull/118600)
* [rustdoc: Arreglar la visualización de características](https://github.com/rust-lang/rust/pull/118677)
* [rustdoc: no escapar de las comillas en el cuerpo del texto](https://github.com/rust-lang/rust/pull/118508)
* [rustdoc: eliminar el parámetro no utilizado 'reversed' de 'onEach(Lazy)'](https://github.com/rust-lang/rust/pull/118722)
* [bindgen: support float16](https://github.com/rust-lang/rust-bindgen/pull/2667)
* [rustfmt: añadir el rasgo 'StyleEdition enum' y 'StyleEditionDefault'](https://github.com/rust-lang/rustfmt/pull/5933)
* [clippy: 'fix(ptr_as_ptr)': maneja 'std::p tr::null{_mut}'](https://github.com/rust-lang/rust-clippy/pull/11913)
* [clippy: 'needless_borrows_for_generic_args': Maneja cuando el operando de campo impl Drop](https://github.com/rust-lang/rust-clippy/pull/11900)
* [clippy: 'uninhabited_reference': nueva pelusa](https://github.com/rust-lang/rust-clippy/pull/11878)
* [clippy: añadir una función para comprobar si los oprandos binarios no son triviales](https://github.com/rust-lang/rust-clippy/pull/11907)
* [clippy: arreglar patrones 'is_from_proc_macro'](https://github.com/rust-lang/rust-clippy/pull/11538)
* [Rust-analyzer: Compruebe si LHS también es un binexpr y use su RHS en Flip binexpr assist](https://github.com/rust-lang/rust-analyzer/pull/15515)
* [rust-analyzer: Recurso a la resolución del método en el acceso a campos no resueltos con el nombre del método coincidente](https://github.com/rust-lang/rust-analyzer/pull/16055)
* [rust-analyzer: añadir el diagnóstico 'trait_impl_reduntant_assoc_item'](https://github.com/rust-lang/rust-analyzer/pull/15990)
* [rust-analyzer: permite que los objetivos de navegación se dupliquen cuando el rango de enfoque se encuentra en el sitio de definición de macros](https://github.com/rust-lang/rust-analyzer/pull/16034)
* [rust-analyzer: soporte de args de formato implícito](https://github.com/rust-lang/rust-analyzer/pull/16027) (¡hurra!)
* [rust-analyzer: priorizar las sugerencias de importación basadas en el tipo esperado](https://github.com/rust-lang/rust-analyzer/pull/15627)
* [rust-analyzer: arreglar el cálculo de desplazamientos de WideChar en 'line-index'](https://github.com/rust-lang/rust-analyzer/pull/16041)
* [Rust-Analyzer: Arreglar el pánico con el cierre dentro de la matriz Len](https://github.com/rust-lang/rust-analyzer/pull/16045)
* [rust-analyzer: error en 'extract_function.rs'](https://github.com/rust-lang/rust-analyzer/pull/16009)
* [Rust-analyzer: no emita el diagnóstico de "elementos faltantes" para impls negativos](https://github.com/rust-lang/rust-analyzer/pull/16039)
* [rust-analyzer: no imprima trazas de retroceso de pánico proc-macro en los registros](https://github.com/rust-lang/rust-analyzer/pull/16037)
* [rust-analyzer: corrige la expansión 'concat_bytes!' que emite un identificador](https://github.com/rust-lang/rust-analyzer/pull/16048)
* [rust-analyzer: se corrige el error de finalización en 'format_args!' con plantilla no válida](https://github.com/rust-lang/rust-analyzer/pull/16060)
* [Rust-analyzer: Soluciona el pánico de diagnóstico al resolver en diferentes archivos debido a macros](https://github.com/rust-lang/rust-analyzer/pull/16035)
* [rust-analyzer: Se corrigió el árbol de elementos que bajaba 'pub(self)' a 'pub()'](https://github.com/rust-lang/rust-analyzer/pull/15486)
* [rust-analyzer: arreglar cwd ejecutable en Windows](https://github.com/rust-lang/rust-analyzer/pull/16024)
* [rust-analyzer: se corrige el mapeo descendente de tokens que es cuadrático](https://github.com/rust-lang/rust-analyzer/pull/16054)
* [Rust-analyzer: arreglar la función MIR, HIR y Eval de la vista que no funciona cuando el cursor está dentro de las macros](https://github.com/rust-lang/rust-analyzer/pull/16078)
* [rust-analyzer: inserte paréntesis de llamada fn solo si los paréntesis se insertan alrededor del nombre del campo](https://github.com/rust-lang/rust-analyzer/pull/16016)
* [Rust-analyzer: Hacer que la sugerencia de incrustación de gotas sea más legible](https://github.com/rust-lang/rust-analyzer/pull/16028)
* [rust-analyzer: resolver las referencias de tipo propio en la asistencia del método delegado](https://github.com/rust-lang/rust-analyzer/pull/15705)
* [Rust-Analyzer: Intervalos más pequeños para diagnósticos de campos y métodos no resueltos](https://github.com/rust-lang/rust-analyzer/pull/16058)
* [rust-analyzer: hacer accesible ParamLoweringMode](https://github.com/rust-lang/rust-analyzer/pull/16036)
* [rust-analyzer: consulta del bloque padre más cercano alrededor de la sugerencia a resolver](https://github.com/rust-lang/rust-analyzer/pull/16089)
* [rust-analyzer: reemplace 'doc_comments_and_attrs' por 'collect_attrs'](https://github.com/rust-lang/rust-analyzer/pull/16073)
* [rust-analyzer: show placeholder while run command gets runnables from server](https://github.com/rust-lang/rust-analyzer/pull/15896)
* [rustc-perf: agregar soporte para la evaluación comparativa del backend de código de Cranelift](https://github.com/rust-lang/rustc-perf/pull/1762)

### Clasificación del rendimiento del compilador de Rust

Una semana tranquila en general. Algunos puntos de referencia de cajas más pequeñas (por ejemplo, helloworld) vieron
mejoras significativas en la
[#118568](https://github.com/rust-lang/rust/pull/118568), sino que esto simplemente
Restaura el rendimiento que se había registrado anteriormente.

Triaje realizado por **@simulacrum**.
Rango de revisión: [9358642..5701093](https://perf.rust-lang.org/?start=9358642e3b8560eee89e6f40aa996c8394a3db31&end=57010939ed1d00076b4af0ed06a81ec69ea5e4a8&absolute=false&stat=instructions%3Au)

5 regresiones, 2 mejoras, 3 mixtas; 2 de ellos en rollups

69 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-12-11.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* [Merge RFC 3531: "Política de edición de especificadores de fragmentos de macros"](https://github.com/rust-lang/rfcs/commit/dbdb738e80d49cb127907e5b40986dacb46ac4b6)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para 'Bound::map'](https://github.com/rust-lang/rust/issues/86026)
* [disposición: fusionar] [Estabilizar THIR unsafeck](https://github.com/rust-lang/rust/pull/117673)
* [disposición: fusionar] [Exhaustividad: revelar correctamente los tipos opacos](https://github.com/rust-lang/rust/pull/116821)
* [disposición: fusionar] [Rechazar correctamente 'default' en elementos const libres](https://github.com/rust-lang/rust/pull/117818)
* [disposición: fusionar] [Hacer que los ciclos inductivos en coherencia sean siempre ambiguos](https://github.com/rust-lang/rust/pull/118649)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Caída asíncrona](https://github.com/rust-lang/rfcs/pull/3541)
* [Cierre ParcialEq y Eq](https://github.com/rust-lang/rfcs/pull/3538)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* [Limpieza de caché de carga](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html)

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-12-13 - 2024-01-10 🦀

### Virtual

* 14/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 14/12/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 17/12/2023 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**¡Que no cunda el pánico! - Nuestro viaje hacia el manejo de errores en Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 18/12/2023 | Virtual (Múnich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Espejo**](https://berline.rs/)
* 19/12/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/297128156/)
* 19/12/2023 | Virtual (Linz, AT) [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 35ª edición**](https://www.meetup.com/rust-linz/events/297909995/)
* 20/12/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Aventuras en el desarrollo de aplicaciones egui**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 26/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 28/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 03/01/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)

### Asia

* 16/12/2023 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europa

* 14/12/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 14/12/2023 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Pruebas: Aprende de los profesionales**](https://www.meetup.com/rust-basel/events/297599586/)
* 18/12/2023 | Múnich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Heidelberg, DE | [Elimina tus insectos y oxida tus motores](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 19/12/2023 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, una alternativa al electrón**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)
* 27/12/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs y peluches**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)

### América del Norte

* 13/12/2023 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/297671182/)
* 14/12/2023 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/)
    * [**Envío de datos con canales con Herbert Wolverson**](https://www.meetup.com/utah-rust/events/297720170/)
* 14/12/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628069/)
* 15/12/2023 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297633899/)
* 19/12/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)
* 27/12/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 09/01/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/182f6dv/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

Lamentablemente, la semana pasó sin una cita nominada.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18i1sov/this_week_in_rust_525/)</small>
