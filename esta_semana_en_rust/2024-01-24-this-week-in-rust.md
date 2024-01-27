---
title: "Esta semana en Rust #16"
number_of_week: 16
description: El crate de esta semana es apistos, una herramienta de documentación de OpenAPI.
date: 2024-01-24
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

### Fundación
* [Resumen del cuarto trimestre de 2023 de Rebecca Rumbul](https://foundation.rust-lang.org/news/q4-2023-recap-from-rebecca-rumbul/)

### Actualizaciones de proyectos/herramientas
* [Revisión de Ruffle 2023](https://ruffle.rs/blog/2024/01/14/2023-in-review)
* [Cuatro desafíos que aún no ha abordado el control de cargas](https://predr.ag/blog/four-challenges-cargo-semver-checks-has-yet-to-tackle/)
* [rustc_codegen_gcc: Informe de Progreso #29](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-29)
* [Hoja de ruta para el backend de Xilem en 2024](https://linebender.org/blog/xilem-backend-roadmap/#2)
* [registro de cambios de rust-analyzer #217](https://rust-analyzer.github.io/thisweek/2024/01/22/changelog-217.html)
* [pq-sys 0.5.0](https://blog.weiznich.de/blog/pq-sys-05/)
* [Novedades de SeaORM 0.12.x](https://www.sea-ql.org/blog/2024-01-23-whats-new-in-seaorm-0.12.x/)
* [Rust en chips Espressif - 24 de enero de 2024](https://mabez.dev/blog/posts/esp-rust-24-01-2024/)

### Observaciones/Pensamientos
* [Hacer que los binarios de Rust sean más pequeños por defecto](https://kobzol.github.io/rust/cargo/2024/01/23/making-rust-binaries-smaller-by-default.html)
* [Mi mejor y peor punto muerto en Rust](https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust/)
* [¿Por qué SQL se cuelga exactamente durante 940s? TCP y Async Rust!](https://xuanwo.io/2024/01-why-sql-hang-for-exactly-940s/)
* [Hacer que el Rust asíncrono sea confiable](https://tmandry.gitlab.io/blog/posts/making-async-reliable/)
* [Identificando la pistola de fuga de memoria collect::() de Rust](https://blog.polybdenum.com/2024/01/17/identifying-the-collect-vec-memory-leak-footgun.html)
* [video] [La embajada ya está en crates.io](https://www.youtube.com/watch?v=o7okEkXPuIA)
* [video] [Los frameworks web full stack de Rust tienen un futuro brillante](https://www.youtube.com/watch?v=tq3-M7QJiWg)
* [video] [Rust Halifax - Rust & Tell #1](https://www.youtube.com/watch?v=MH-7xnv9CMI)
* [video] [Por qué Rust seguirá creciendo en 2024](https://www.youtube.com/watch?v=Q4VNRgxMQ6I)

### Tutoriales de Rust
* [Uso de 'mem::take' para reducir las asignaciones de montón](https://ferrous-systems.com/blog/rustls-borrow-checker-p1/)
* [Escribir tu propio linter de Rust](https://blog.guillaume-gomez.fr/articles/2024-01-18+Writing+your+own+Rust+linter)
* [Usando Serde en Rust](https://www.shuttle.rs/blog/2024/01/23/using-serde-rust)
* [Análisis de JSON en Rust](https://www.shuttle.rs/blog/2024/01/18/parsing-json-rust)
* [Desafío de mil millones de filas: tutorial de Rust](https://aminediro.com/posts/billion_row/)
* [Embajada en ESP: Temporizadores](https://apollolabsblog.hashnode.dev/embassy-on-esp-timers)
* [Soporte de LoRa en SparkFun expLoRaBLE Thing Plus con Rust](https://www.alistair23.me/2023/08/09/lora-on-sparkfun-board/)
* [Cómo trabajar con ! Tipos de tamaño en Rust](https://sgued.fr/blog/heapless-howto/)
* [Rocket - inicio de sesión en la aplicación web](https://rust.code-maven.com/rocket-logging)
* [Rocket - acceder a la configuración personalizada en las rutas](https://rust.code-maven.com/rocket-access-custom-configuration)
* [Pruebas con tempfiles y variables de entorno](https://rust.code-maven.com/testing-with-environment-variables)

### Investigación
* [Perfilado del aprendizaje de lenguajes de programación](https://arxiv.org/abs/2401.01257)
* [Rust-lancet: Corrección automatizada de la violación de la regla de propiedad con preservación del comportamiento](https://songlh.github.io/paper/lancet.pdf)

## Crate de la semana

El crate de esta semana es [apistos](https://github.com/netwo-io/apistos), una herramienta de documentación de OpenAPI.

¡Gracias a [Romain Lebran](https://users.rust-lang.org/t/crate-of-the-week/2704/1279) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->
* [Ockam - Tener una sola instancia de 'SqlxDatabase' por proceso](https://github.com/build-trust/ockam/issues/7313)
* [Ockam - Mejorar las migraciones de bases de datos para emparejar código de migración sql y rust](https://github.com/build-trust/ockam/issues/7311)
* [Ockam - Haz que install.sh no falle durante el proceso de actualización](https://github.com/build-trust/ockam/issues/7118)
* [Hyperswitch - [CARACTERÍSTICA]: Hacer que la configuración de caché sea configurable en tiempo de ejecución](https://github.com/juspay/hyperswitch/issues/3276)
* [Hyperswitch - [CARACTERÍSTICA]: Implementar Code cov para el sistema local usando makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - [CARACTERÍSTICA]: Cobertura de código de configuración para pruebas locales y CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [CARACTERÍSTICA]: Agregar tipo de dominio para el secreto de cliente](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - [FEATURE]: Tener get_required_value usar ValidationError en OptionExt](https://github.com/juspay/hyperswitch/issues/860)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador. 

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](enlace al CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

453 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-16..2024-01-23

* ['HashMap'/'HashSet': implementaciones 'plegadas' hacia adelante de iteradores](https://github.com/rust-lang/rust/pull/117756)
* ['dead_code' trata '#[repr(transparent)]' de la misma manera que '#[repr(C)]'](https://github.com/rust-lang/rust/pull/120107)
* ['fix(rust-analyzer)': use la nueva especificación pkgid para comparar](https://github.com/rust-lang/rust/pull/120084)
* ['large_assignments': Lint en argumentos grandes específicos pasados a funciones](https://github.com/rust-lang/rust/pull/116520)
* ['maybe_lint_impl_trait': separa 'is_downgradable' de 'is_object_safe'](https://github.com/rust-lang/rust/pull/120164)
* ['never_patterns': Cuenta los enlaces '!' como divergentes](https://github.com/rust-lang/rust/pull/120104)
* ['never_patterns': comprobación de tipos nunca patrones](https://github.com/rust-lang/rust/pull/120009)
* ['pat_analysis': No confíe en 'VariantId' contiguos fuera de rustc](https://github.com/rust-lang/rust/pull/120039)
* ['pattern_analysis': Eliminar 'Ty: Copiar' enlazado](https://github.com/rust-lang/rust/pull/120027)
* ['proc_macro': Añade el constructor 'Literal::c_string'](https://github.com/rust-lang/rust/pull/119651)
* ['single_use_lifetimes': No sugiera borrar vidas con límites](https://github.com/rust-lang/rust/pull/120148)
* [añadir '#[track_caller]' a la implación "From implica Into"](https://github.com/rust-lang/rust/pull/119807)
* [add 'Ipv6Addr::is_ipv4_mapped'](https://github.com/rust-lang/rust/pull/119081)
* [añadir 'PatKind::Err' a AST/HIR](https://github.com/rust-lang/rust/pull/119967)
* [añadir mensaje de ayuda para el error 'exclusive_range_pattern'](https://github.com/rust-lang/rust/pull/120152)
* [añadir alias privado de tipo 'NonZero<T>'](https://github.com/rust-lang/rust/pull/119990)
* [añadir forma de expresar que no se esperan valores con check-cfg](https://github.com/rust-lang/rust/pull/119930)
* [añadido 'NonZeroXxx::from_mut(_unchecked)?'](https://github.com/rust-lang/rust/pull/103730)
* [permitir cualquier bloque de expresión 'const' en 'thread_local!'](https://github.com/rust-lang/rust/pull/120181)
* [use siempre RevealAll para consultas constantes](https://github.com/rust-lang/rust/pull/119821)
* [evitar los ICE en los nombres de rasgos sin 'dyn'](https://github.com/rust-lang/rust/pull/119752)
* [consolidar la lógica en torno a la resolución de impls de rasgos de corrutina integrados](https://github.com/rust-lang/rust/pull/120143)
* [denegar invocaciones de macro entre llaves en let-else](https://github.com/rust-lang/rust/pull/119062)
* [detectar el error 'NulInCStr' antes](https://github.com/rust-lang/rust/pull/119172)
* [mejorar 'let_underscore_lock'](https://github.com/rust-lang/rust/pull/119710)
* [Atributo 'collapse_debuginfo' mejorado, se agregó una bandera de línea de comandos](https://github.com/rust-lang/rust/pull/119828)
* [hacer 'unsafe_op_in_unsafe_fn' migrado en la edición 2024](https://github.com/rust-lang/rust/pull/119948)
* [restringir el acceso al campo privado de los índices newtype](https://github.com/rust-lang/rust/pull/120134)
* [simplificar 'closure_env_ty' y 'closure_env_param'](https://github.com/rust-lang/rust/pull/119969)
* [sugerir '.swap()' cuando se encuentran préstamos conflictivos de 'mem::swap' en un segmento](https://github.com/rust-lang/rust/pull/120126)
* [desaprobar lint 'unstable_features' y hacer uso de él en el compilador](https://github.com/rust-lang/rust/pull/118639)
* [hacer que el nombre del pase MIR sea una constante en tiempo de compilación](https://github.com/rust-lang/rust/pull/120161)
* [hacer sonar 'stable_mir::with_tables'](https://github.com/rust-lang/rust/pull/120128)
* [SMIR: hacer que los campos "privados" restantes sean realmente privados](https://github.com/rust-lang/rust/pull/120135)
* [usar un intérprete en el subproceso de salto MIR](https://github.com/rust-lang/rust/pull/119461)
* [usar el modo de compatibilidad de límites implícitos en MIR borrowck](https://github.com/rust-lang/rust/pull/120123)
* [validar los tipos de AggregateKind en MIR](https://github.com/rust-lang/rust/pull/120137)
* [sándwich de optimizaciones MIR entre DSE](https://github.com/rust-lang/rust/pull/119672)
* [almacenar en caché las consultas locales con clave DefId sin hash](https://github.com/rust-lang/rust/pull/119977)
* [Empaquetar u128 en el compilador para mitigar la nueva alineación](https://github.com/rust-lang/rust/pull/120080)
* [use UnhashMap para algunos mapas más](https://github.com/rust-lang/rust/pull/120076)
* [plegar identidades aritméticas en GVN](https://github.com/rust-lang/rust/pull/119670)
* [optimizar la creación de matrices grandes en const-eval](https://github.com/rust-lang/rust/pull/120069)
* [Implementar rasgos de especialización de iterador en más adaptadores](https://github.com/rust-lang/rust/pull/85528)
* [optimizar la visualización de 'EscapeAscii' y la depuración de 'CStr'](https://github.com/rust-lang/rust/pull/113142)
* [estabilizar 'bound_map'](https://github.com/rust-lang/rust/pull/118361)
* [estabilizar 'round_ties_even'](https://github.com/rust-lang/rust/pull/120150)
* [estabilizar 'slice_first_last_chunk'](https://github.com/rust-lang/rust/pull/117561)
* [estabilizar 'offset_of!'](https://github.com/rust-lang/rust/pull/118799)
* [implementar operaciones enteras estrictas que entren en pánico en caso de desbordamiento](https://github.com/rust-lang/rust/pull/116090)
* [core: introduce 'split_at{,_mut}_checked'](https://github.com/rust-lang/rust/pull/118578)
* [un-hide 'iter::repeat_n'](https://github.com/rust-lang/rust/pull/120045)
* [corregida la desasignación con un asignador incorrecto en '(A)Rc::from_box_in'](https://github.com/rust-lang/rust/pull/119801)
* [use 'bool' en lugar de 'PartialOrd' como valor devuelto del cierre de comparación en '{slice,Iterator}::is_sorted_by'](https://github.com/rust-lang/rust/pull/118811)
* [regex: hacer que 'Input::new' se proteja contra implementaciones incorrectas de 'AsRef'](https://github.com/rust-lang/regex/pull/1154)
* [cargo-rustdoc: use la misma ruta por lógica de formato de salida en todas partes](https://github.com/rust-lang/cargo/pull/13325)
* [cargo: use la especificación pkgid en los mensajes JSON](https://github.com/rust-lang/cargo/pull/13311)
* [cargo: reasignar solo prefijo común](https://github.com/rust-lang/cargo/pull/13210)
* [cargo doc: agregue un encabezado para resaltar "Cómo encontrar funciones habilitadas en dependencias"](https://github.com/rust-lang/cargo/pull/13305)
* [cargo: heredar el servidor de trabajo de env para todo tipo de corredor](https://github.com/rust-lang/cargo/pull/12776)
* [Cargo: Arreglar el enlace de seguimiento preciso-prelanzamiento](https://github.com/rust-lang/cargo/pull/13320)
* [cargo: volver a pasar un 'values()' vacío cuando no se declaran características](https://github.com/rust-lang/cargo/pull/13316)
* [cargo: mejorar la configuración de CI de GitHub Actions](https://github.com/rust-lang/cargo/pull/13317)
* [rustdoc: Permite enlaces en encabezados](https://github.com/rust-lang/rust/pull/117662)
* [rustdoc: ocultar modales al cambiar el tamaño de la barra lateral](https://github.com/rust-lang/rust/pull/119746)
* [rustfmt: comprueba que un token puede comenzar un tipo no terminal antes de analizarlo como un macro arg](https://github.com/rust-lang/rust/pull/120218)
* [rustfmt: añadir la opción de configuración 'generated_marker_line_search_limit'](https://github.com/rust-lang/rustfmt/pull/5993)
* [clippy: 'blocks_in_conditions': no advertir si la condición proviene de la macro](https://github.com/rust-lang/rust-clippy/pull/12173)
* [clippy: 'default_numeric_fallback': mejora la detección de contexto const](https://github.com/rust-lang/rust-clippy/pull/12168)
* [clippy: 'no_effect_underscore_binding: _' se pueden usar variables con prefijo](https://github.com/rust-lang/rust-clippy/pull/12172)
* [clippy: 'unused_io_amount' captura 'Ok(_)'s](https://github.com/rust-lang/rust-clippy/pull/12005)
* [clippy: añadir pelusa 'suspicious_open_options'](https://github.com/rust-lang/rust-clippy/pull/11608)
* [clippy: maneja correctamente el tipo relativo en 'trait_duplication_in_bounds' lint](https://github.com/rust-lang/rust-clippy/pull/12155)
* [clippy: no emita 'derive_partial_eq_without_eq' lint si el tipo tiene el atributo 'non_exhaustive'](https://github.com/rust-lang/rust-clippy/pull/12153)
* [clippy: encuentra referencias de ruta de función al principio del mismo paso de lint](https://github.com/rust-lang/rust-clippy/pull/12147)
* [clippy: arreglar FP en 'semicolon_if_nothing_returned'](https://github.com/rust-lang/rust-clippy/pull/12167)
* [clippy: corrige 'multiple_crate_versions' para normalizar correctamente los nombres de los paquetes y evitar perder el local](https://github.com/rust-lang/rust-clippy/pull/12146)
* [clippy: se corrigió el intervalo de advertencia para 'no_effect_underscore_binding'](https://github.com/rust-lang/rust-clippy/pull/12125)
* [clippy: respeta los atributos '#[allow]' en 'single_call_fn' lint](https://github.com/rust-lang/rust-clippy/pull/12183)
* [clippy: mejorar la redacción y corregir el enlace muerto en la descripción de 'arc_with_non_send_sync' lint](https://github.com/rust-lang/rust-clippy/pull/11945)
* [rust-analyzer: añadir "una" granularidad de importación](https://github.com/rust-lang/rust-analyzer/pull/16372)
* [rust-analyzer: añadir una nueva configuración para permitir el cambio de nombre de las definiciones no locales](https://github.com/rust-lang/rust-analyzer/pull/16391)
* [Rust-Analyzer: Acciones de tipo Goto para desplazamientos de rasgos notables](https://github.com/rust-lang/rust-analyzer/pull/16375)
* [rust-analyzer: muestra información de valor adicional al pasar el cursor sobre literales](https://github.com/rust-lang/rust-analyzer/pull/16370)
* [Rust-analyzer: muestra rasgos implementados notables al pasar el mouse](https://github.com/rust-lang/rust-analyzer/pull/16374)
* [rust-analyzer: añadir recuperación de errores para el análisis 'use_tree_list'](https://github.com/rust-lang/rust-analyzer/pull/16349)
* [rust-analyzer: corrige el pánico al extraer 'struct' de la variante 'enum'](https://github.com/rust-lang/rust-analyzer/pull/16396)
* [rust-analyzer: arreglar el informe de progreso que se atasca](https://github.com/rust-lang/rust-analyzer/pull/16383)
* [rust-analyzer: maneja mejor 'SelfParam' en "Inline call"](https://github.com/rust-lang/rust-analyzer/pull/16378)
* [rust-analyzer: incluir la construcción 'for' en las condiciones de retorno convertidas a protegidas](https://github.com/rust-lang/rust-analyzer/pull/16405)
* [rust-analyzer: inferir 'OUT_DIR' cuando la raíz del espacio de trabajo contiene un enlace simbólico](https://github.com/rust-lang/rust-analyzer/pull/15868)
* [rust-analyzer: hacer que la consulta 'value_ty' sea falible](https://github.com/rust-lang/rust-analyzer/pull/16367)
* [rust-analyzer: analiza 'macro_rules' como nombre de macro](https://github.com/rust-lang/rust-analyzer/pull/16314)

### Clasificación del rendimiento del compilador de Rust

Esta semana se han producido un montón de regresiones causadas por correcciones de corrección y, en general, por hacer más trabajo
en el compilador. Estos fueron compensados por muchas mejoras (especialmente en torno al hash en el compilador)
Eso mejoró el rendimiento en ~2% en un gran número de puntos de referencia. No te entusiasmes demasiado con el
Sin embargo, grandes ganancias del 45+%, estas fueron solo para pequeños puntos de referencia como Helloworld. Fueron causados por un
[cambio en Carga](https://github.com/rust-lang/cargo/pull/13257) que introduce la eliminación de la depuración
símbolos de Rust liberan binarios de forma predeterminada y, a su vez, también mejora el tiempo de compilación para pequeños
Cajones.

Triaje realizado por **@kobzol**.
Rango de revisión: [f9c2421a.. d6b151fc](https://perf.rust-lang.org/?start=f9c2421a2a6e34f3756900dd7d600704c08bfccb&end=d6b151fc77e213bf637db0f12c1965ace3ffe255&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.7% | [0,2%, 1,5%] | 11 |
| Regresiones ❌ <br /> (secundaria) | 2,2% | [0.2%, 9.9%] | 26 |
| Mejoras ✅ <br /> (primaria) | -3,2% | [-47,5%, -0,2%] | 191 |
| Mejoras ✅ <br /> (secundaria) | -7,9% | [-46,5%, -0,1%] | 123 |
| Todos ❌✅ (primario) | -3.0% | [-47,5%, 1,5%] | 202 |

4 regresiones, 4 mejoras, 9 mixtas; 4 de ellos en rollups
48 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/dc3605e34203a3513f589868a161b8818b30adca/triage/2024-01-23.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: cerrar] [Añadir un indicador predeterminado para la documentación de enumeración](https://github.com/rust-lang/rust/pull/115575)
* [disposición: fusionar] [impl 'De<&[T; N]>' por 'Vaca<[T]>'](https://github.com/rust-lang/rust/pull/113489)
* [disposición: fusionar] [Problema de seguimiento para array_methods](https://github.com/rust-lang/rust/issues/76118)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [eRFC: Iterar y estabilizar la salida programática de libtest](https://github.com/rust-lang/rfcs/pull/3558)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2024-01-24 - 2024-02-21 🦀

### Virtual

* 24/01/2024 | Virtual (Berlín, DE) | [Comunidad WeAreDevelopers](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 25/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 25/01/2024 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 36ª Edición**](https://www.meetup.com/rust-linz/events/298687390/)
* 25/01/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 28/01/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 30/01/2024 | Virtual | [Desarrollo de juegos Bevy](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 30/01/2024 | Virtual (Búfalo, NY, EE. UU.) | [Grupo de usuarios de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 30/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
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
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 08/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 08/02/2024 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 10/02/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 13/02/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341575/)
* 15/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-21 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763497/)

### Europa

* 24/01/2024 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/Zagreb-Rust-Meetup/)
    * [**Rust Meetup 2024/01: Introducción a WebGPU usando Rust**](https://www.meetup.com/zagreb-rust-meetup/events/298540606/)
* 25/01/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #5: Async Part2 y Async en acción**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/298008068/)
* 25/01/2024 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - Enero - Blockchains y Data Pipelines**](https://www.meetup.com/rust-vienna/events/298504153/)
* 01/02/2024 | Híbrido (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12ª reunión de BcnRust**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 03/02/2024 | Bruselas, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM '24: Rust devroom - charlas**](https://fosdem.org/2024/schedule/track/rust/) | [**Reunión FOSDEM de Rust Aarhus**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 03/02/2024 | Nürnberg, BY, DE | [Campamento de Rust de Paessler 2024](https://www.meetup.com/paessler-rust-camp-2024/)
    * [**Paessler Rust Camp 2024**](https://www.meetup.com/paessler-rust-camp-2024/events/298603948)
* 06/02/2024 | Bremen, DE | [Encuentro de Rust Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 07/02/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 08/02/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀 **](https://www.meetup.com/rust-bern/events/298488858/)

### América del Norte

* 24/01/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Hackathon Aprovechando el Rust para problemas del mundo real: Día 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Hackathon de aprovechamiento de Rust para problemas del mundo real: Día 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 25/01/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/mv-rust-meetup/events/298645923/)
* 30/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 07/02/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Coolidge Corner Brookline Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/297635028/)
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

### Oceanía

* 06/02/2024 | Perth, WA, AU | [Grupo de reunión de Perth Rust](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Reunión de Rust de febrero de 2024**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Las raíces funcionales de ML del lenguaje, el primer compilador de Rust de Graydon que se escribió en OCaml, brillan e influyen en él desde el principio.
>
> No es "C++ pero mejor".
>
> Es Haskell parado sobre los hombros de Lisp, escondiéndose en el abrigo de C para colarse en PRDCTN. (El elegante club nocturno donde se reúnen todos los idiomas populares)

– [tris en su canal de Youtube "No Boilerplate"](https://www.youtube.com/watch?v=voRBS0r4EyI&t=317)

¡Gracias a [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1512) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/19f2pos/this_week_in_rust_531/)</small>
