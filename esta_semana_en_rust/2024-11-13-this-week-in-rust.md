---
title: "Esta semana en Rust #35"
number_of_week: 35
description: El crate de esta semana es struct-split, una macro proc para implementar préstamos parciales.
date: 2024-11-13
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
* [gccrs: Un compilador alternativo para Rust](https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust.html)
* [Resultados Google Summer of Code 2024](https://blog.rust-lang.org/2024/11/07/gsoc-2024-results.html)

### Fundación
* [Rust Foundation publica una declaración de problema sobre la interoperabilidad de C++/Rust](https://foundation.rust-lang.org/news/rust-foundation-releases-problem-statement-on-c-rust-interoperability/)

### Boletines
* [Linebender en octubre de 2024: resvg stewardship](https://linebender.org/blog/tmix-10/)
* [El Rustáceo Incrustado Edición #32](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-32)

### Actualizaciones de proyectos/herramientas
* [Presentación de Hyperlight: Seguridad basada en máquinas virtuales para funciones a escala](https://opensource.microsoft.com/blog/2024/11/07/introducing-hyperlight-virtual-machine-based-security-for-functions-at-scale/)
* [Presentamos Sled, una biblioteca de Rust para crear efectos de iluminación de tiras de LED espaciales](https://davjcosby.github.io/all-published/miscellaneous-tech/Introducing%20Sled,%20a%20Rust%20Library%20for%20Creating%20Spatial%20LED%20Strip%20Lighting%20Effects.html)
* [Redis Shield: Un módulo limitador de velocidad de alto rendimiento en Rust que utiliza el algoritmo Token Bucket](https://github.com/ayarotsky/redis-shield)
* [Cohen: gccrs: Un compilador alternativo para Rust](https://lwn.net/Articles/997483/)
* [Progreso en las características de seguridad de la cadena de herramientas](https://lwn.net/SubscriberLink/996344/a68070fd6ffe56e9/)
* [Lanzamiento de macro de creación de próxima generación Bon 3.0](https://bon-rs.com/blog/bon-v3-release)

### Observaciones/Pensamientos
* [Quizás Rust necesita "aplazar"](https://gaultier.github.io/blog/perhaps_rust_needs_defer.html)
* [Rust necesita una especificación oficial](https://tweedegolf.nl/en/blog/140/rust-needs-an-official-specification)
* [¿Por qué std::p in::P in es tan raro?](https://sander.saares.eu/2024/11/06/why-is-stdpinpin-so-weird/)
* [Trayendo excepciones más rápidas a Rust](https://purplesyringa.moe/blog/bringing-faster-exceptions-to-rust/)
* [Explorando el código ensamblador generado por Rust Recursive Tree Traversal](https://www.eventhelix.com/rust/rust-to-assembly-recursive-tree-fold/)
* [ID mecanografiados con SeaORM](https://klacan.sk/posts/typed-ids-with-sea-orm/)
* [Procesos de desove en Linux](https://maelstrom-software.com/blog/spawning-processes-on-linux/)
* [video] [Actualización de los objetivos del proyecto Rust 2024 y Rust 1.80.1](https://www.youtube.com/watch?v=DQ2XqNB-0Qg)
* [video] [Río: Emulador de terminal de próxima generación escrito en Rust](https://www.youtube.com/watch?v=bxcYj78UNPU)

### Tutoriales de Rust
* [Analizando argumentos en Rust sin dependencias](https://ntietz.com/blog/parsing-arguments-rust-no-deps/)
* [Uso de SIMD portátil en Rust estable](https://pythonspeed.com/articles/simd-stable-rust/)
* [Tutorial de Rust Syn Crate: Automatice los patrones de creación con macros personalizadas](https://packetandpine.com/blog/rust-syn-crate-tutorial/)
* [Tutorial: Implementación del análisis sintáctico JSON](https://blog.davimiku.com/tutorials/json-parsing-rust-1)
* [Impl Snake For Micro:bit - Incrustado Rust asíncrono en BBC Micro:bit con Embassy](https://gitlab.com/cyril-marpaud/impl_snake_for_microbit/-/blob/main/README.md)

### Miscelánea
* [Informe de empleos de Rust de octubre de 2024](https://filtra.io/rust/jobs-report/oct-24)

## Crate de la semana

El crate de esta semana es [struct-split](https://github.com/wdanilo/struct-split), una macro proc para implementar préstamos parciales.

¡Gracias a [Felix](https://users.rust-lang.org/t/crate-of-the-week/2704/1374) por la sugerencia!

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

* [Rama — múltiples credenciales básicas/de portador para soporte de servidor de 'Autorización'](https://github.com/plabayo/rama/issues/352)
* [Rama — implementar 'tomar' y 'reemplazar' para Contexto y Extensiones](https://github.com/plabayo/rama/issues/353)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

403 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-05..2024-11-12

* [eliminar el objetivo 'wasm32-wasi' de rustc](https://github.com/rust-lang/rust/pull/132562)
* [añadir una nueva característica de 'aritmética amplia' para WebAssembly](https://github.com/rust-lang/rust/pull/132077)
* [agregar soporte de salida del compilador de dibujo de bloques Unicode](https://github.com/rust-lang/rust/pull/126597)
* [add '{ignore,needs}-{rustc,std}-debug-assertions' soporte de directiva](https://github.com/rust-lang/rust/pull/131913)
* [añadir una implementación predeterminada para 'CodegenBackend::link'](https://github.com/rust-lang/rust/pull/132820)
* [añadir discriminadores a DILocations cuando múltiples funciones están alineadas en un solo punto](https://github.com/rust-lang/rust/pull/132613)
* [Agregue la función de destino V9, V8Plus y LeonCasa a SPARC y use V8Plus en 'create_object_file'](https://github.com/rust-lang/rust/pull/132552)
* [Pruebas adicionales para asegurar que LET se rechaza durante el análisis](https://github.com/rust-lang/rust/pull/132828)
* [tipos de sí mismo arbitrarios v2: (sin usar) rasgo de receptor](https://github.com/rust-lang/rust/pull/132144)
* [soporte básico de ensamblaje en línea para SPARC y SPARC64](https://github.com/rust-lang/rust/pull/132472)
* [cobertura: extraiga las funciones seguras del envoltorio FFI a 'llvm_cov'](https://github.com/rust-lang/rust/pull/132452)
* [cobertura: restringir la expansión de intervalo vacío para cubrir solo '{' y '}'](https://github.com/rust-lang/rust/pull/132675)
* [Cobertura: Simplificar partes de la creación de gráficos de cobertura](https://github.com/rust-lang/rust/pull/132389)
* [no filtrar los pasos de pelusa vacíos y rehacer el paso de CTFE](https://github.com/rust-lang/rust/pull/132637)
* [no reveles opacos en el param-env, en su lugar tenemos la norma perezosa](https://github.com/rust-lang/rust/pull/132755)
* [no confíe en download-rustc=if-unchanged en CI por ahora](https://github.com/rust-lang/rust/pull/132852)
* [no sugiera '.into_iter()' en los iteradores](https://github.com/rust-lang/rust/pull/132760)
* [no use 'maybe_unwrap_block' al buscar llamadas de macro en un bloque expr](https://github.com/rust-lang/rust/pull/132653)
* [no sugiera 'use<impl Trait>' cuando tengamos un problema de borrowck relacionado con la edición de 2024](https://github.com/rust-lang/rust/pull/132816)
* [elimine "gnu" en el entorno de destino para FreeBSD armv6/7](https://github.com/rust-lang/rust/pull/132764)
* [emitir advertencia al llamar/declarar funciones con vectores no disponibles](https://github.com/rust-lang/rust/pull/132173)
* [Imponer que las vidas sin procesar deben ser identificadores sin procesar válidos](https://github.com/rust-lang/rust/pull/132363)
* [Asegúrese de que la expansión de cola reciba una extensión de vida útil](https://github.com/rust-lang/rust/pull/129627)
* [arreglar paréntesis destrozados en la sugerencia de lint estática de mut compartido](https://github.com/rust-lang/rust/pull/132095)
* [deshacerse de 'check_opaque_type_well_formed'](https://github.com/rust-lang/rust/pull/132757)
* [hacer 'RustString' un tipo extern para evitar advertencias de 'improper_ctypes'](https://github.com/rust-lang/rust/pull/132549)
* [hacer que 'Ty::p rimitive_symbol' reconozca 'str'](https://github.com/rust-lang/rust/pull/132799)
* [hacer 'fn_abi_sanity_check' un poco más estricto](https://github.com/rust-lang/rust/pull/132729)
* [asegúrese de que sugerimos la turbopesca del tipo correcto arg para nunca sugerencia](https://github.com/rust-lang/rust/pull/132933)
* [marque algunas características de destino como 'prohibidas' para que no puedan ser (des)establecidas con -Ctarget-feature](https://github.com/rust-lang/rust/pull/129884)
* [solo deshabilita la caché si el predicado tiene opacos dentro de él](https://github.com/rust-lang/rust/pull/132625)
* [passWrapper: adaptar para nuevo parámetro en LLVM](https://github.com/rust-lang/rust/pull/132600)
* [preferir 'pub(super)' en 'unreachable_pub' sugerencia de lint](https://github.com/rust-lang/rust/pull/132426)
* [sugerir correctamente 'E::assoc' cuando nos encontramos con 'E::Variant::assoc'](https://github.com/rust-lang/rust/pull/132567)
* [proporcionar marcadores de posición genéricos para rasgos en las sugerencias de "no se encontró ningún método para el parámetro de tipo"](https://github.com/rust-lang/rust/pull/132487)
* [rechaza el tiempo de vida bruto seguido de ''', como lo hacen los tiempos de vida regulares](https://github.com/rust-lang/rust/pull/132341)
* [eliminar los restos de ABI 'intrínsecos a la plataforma'](https://github.com/rust-lang/rust/pull/132734)
* [eliminar 'rustc_session::config::rustc_short_optgroups'](https://github.com/rust-lang/rust/pull/132891)
* [eliminar el soporte para el atributo 'rustc_safe_intrinsic'; usar las funciones 'rustc_intrinsic' en su lugar](https://github.com/rust-lang/rust/pull/132717)
* [eliminar las importaciones innecesarias de glob-imports de pub 'enum' de 'rustc_middle::ty'](https://github.com/rust-lang/rust/pull/132580)
* [requiere puerta 'const_impl_trait' para todas las llamadas const condicionales y de rasgo](https://github.com/rust-lang/rust/pull/132823)
* [revertir usando la estática 'HEAP' en la asignación de Windows](https://github.com/rust-lang/rust/pull/131888)
* [establecer el "nombre del símbolo" en las bibliotecas de importación raw-dylib con el nombre representativo](https://github.com/rust-lang/rust/pull/130586)
* [simplificar las llamadas FFI para '-Ztime-llvm-passes' y '-Zprint-codegen-stats'](https://github.com/rust-lang/rust/pull/132590)
* [simplificar algunos lugares que se ocupan de los parámetros genéricos predeterminados](https://github.com/rust-lang/rust/pull/132912)
* [simplificar la API interna para declarar opciones de línea de comandos](https://github.com/rust-lang/rust/pull/132754)
* [sugerir cambiar LHS y RHS cuando RHS implique 'PartialEq<lhs_ty>'](https://github.com/rust-lang/rust/pull/132404)
* [modificar la redacción del error de desbordamiento E0320](https://github.com/rust-lang/rust/pull/132663)
* [Se ha ajustado la detección de múltiples versiones de cajas para que sean más abarcadoras](https://github.com/rust-lang/rust/pull/128849)
* [use 'download-rustc="if-unchanged"' como un valor predeterminado global](https://github.com/rust-lang/rust/pull/132772)
* [use un directorio separado para compilaciones de R-A de manera consistente en Helix Config](https://github.com/rust-lang/rust/pull/132794)
* [Usar detallado para la sugerencia de separador de rutas](https://github.com/rust-lang/rust/pull/132780)
* ['pointee_info_at': arreglar la lógica para recursar en enumeraciones](https://github.com/rust-lang/rust/pull/132745)
* ['rustc_codegen_llvm': Añadir una nueva opción 'pc' a branch-protection](https://github.com/rust-lang/rust/pull/132259)
* ['rustc_target': más correcciones de cadenas de destino para LLVM 20](https://github.com/rust-lang/rust/pull/132785)
* [interpretar: 'get_alloc_info': también devuelve mutabilidad](https://github.com/rust-lang/rust/pull/132801)
* [StableMIR: Algunas correcciones a la impresión bonita](https://github.com/rust-lang/rust/pull/132161)
* [StableMIR: API para recuperar definiciones de cajas](https://github.com/rust-lang/rust/pull/132131)
* [Miri: arreglar la prueba de linux-futex que se desactivaba accidentalmente](https://github.com/rust-lang/miri/pull/4022)
* [Miri: get/set thread name shims return errors for invalid handles](https://github.com/rust-lang/miri/pull/4004)
* [Miri: Preparándose para la fusión de rustc](https://github.com/rust-lang/miri/pull/4023)
* [Miri: pthread-sync test: evita errores confusos al ejecutar con adelantamiento](https://github.com/rust-lang/miri/pull/4020)
* [miri: eliminar la lista de MutexID](https://github.com/rust-lang/miri/pull/4002)
* [Miri: renombró estos argumentos a ecx](https://github.com/rust-lang/miri/pull/4029)
* [miri: pruebas de préstamos apilados: agregue las que fallan bajo TB](https://github.com/rust-lang/miri/pull/4028)
* [miri: nombres de variables estandarizadas para InterpCx](https://github.com/rust-lang/miri/pull/4018)
* [Miri: Almacene los futexes en datos por asignación en lugar de globalmente](https://github.com/rust-lang/miri/pull/3971)
* [Miri: Soporte de sincronización: no clonar implícitamente dentro de la maquinaria de sincronización general](https://github.com/rust-lang/miri/pull/4027)
* [estabilizar 'const_char_encode_utf16'](https://github.com/rust-lang/rust/pull/132153)
* [estabilizar el ensamblaje en línea Arm64EC](https://github.com/rust-lang/rust/pull/131781)
* [estabilizar las características de destino 'multivalor', tipos de referencia y llamadas de cola de WebAssembly](https://github.com/rust-lang/rust/pull/131080)
* [estabilizar 'UnsafeCell::from_mut'](https://github.com/rust-lang/rust/pull/131261)
* [Estabilizar ensamblaje en línea S390X](https://github.com/rust-lang/rust/pull/131258)
* [añadir nueva característica inestable 'const_eq_ignore_ascii_case'](https://github.com/rust-lang/rust/pull/131721)
* [hacer 'char::is_whitespace' de manera inestable const](https://github.com/rust-lang/rust/pull/132500)
* [en línea 'str::repeat'](https://github.com/rust-lang/rust/pull/132705)
* [core/fmt: Reemplace la indexación de segmentos marcada por no marcada para admitir código sin pánico](https://github.com/rust-lang/rust/pull/132473)
* [agregar API de entrada de conjunto](https://github.com/rust-lang/rust/pull/120077)
* [implementar 'div_ceil' por 'NonZero<unsigned>'](https://github.com/rust-lang/rust/pull/132665)
* [implementar la función 'file_lock'](https://github.com/rust-lang/rust/pull/130999)
* [inicializar el canal 'Block' directamente en el montón](https://github.com/rust-lang/rust/pull/132738)
* [desactivar 'f16' en plataformas que tengan problemas de recursividad](https://github.com/rust-lang/compiler-builtins/pull/730)
* [cargo: advertencias: añadir la opción build.warnings](https://github.com/rust-lang/cargo/pull/14388)
* [cargo: test: Hacer redacciones consistentes con snapbox](https://github.com/rust-lang/cargo/pull/14790)
* [cargo: git: no validar submódulos de nuevos checkouts](https://github.com/rust-lang/cargo/pull/14605)
* [cargo: normalizar las rutas 'objetivo'](https://github.com/rust-lang/cargo/pull/14497)
* [cargo: refactorizar: clonar en escritura cuando sea necesario para InternedString](https://github.com/rust-lang/cargo/pull/14808)
* [Cargo: Rustfix: Reemplaza el manejo de duplicados de casos especiales con error](https://github.com/rust-lang/cargo/pull/14782)
* [rustdoc-search: mostrar firma de tipo en SERP basado en tipos](https://github.com/rust-lang/rust/pull/124544)
* [rustdoc-search: simplificar las reglas para genéricos y parámetros de tipo](https://github.com/rust-lang/rust/pull/127589)
* [bindgen: arreglar 'field_visibility' no llamado para alias de nuevo tipo](https://github.com/rust-lang/rust-bindgen/pull/2967)
* [bindgen: arreglar 'unsafe_op_in_unsafe_fn' al usar bibliotecas dinámicas y 'wrap_unsafe_ops'](https://github.com/rust-lang/rust-bindgen/pull/2961)
* [manejar prefijos separados en las reglas de clippy](https://github.com/rust-lang/rust/pull/132873)
* [clippy: 'no_mangle_with_rust_abi': coloque correctamente el ABI sugerido](https://github.com/rust-lang/rust-clippy/pull/13659)
* [clippy: añadir intento manual basado en coincidencias a 'clippy::question_mark'](https://github.com/rust-lang/rust-clippy/pull/13627)
* [clippy: Recopila intervalos de atributos temprano para macros no permitidas](https://github.com/rust-lang/rust-clippy/pull/13657)
* [clippy: arreglar la pelusa 'large_include_file' que se activa todo el tiempo por los comentarios del documento](https://github.com/rust-lang/rust-clippy/pull/13672)
* [clippy: corrección: Las sugerencias de 'identity_op' usan el paréntesis correcto](https://github.com/rust-lang/rust-clippy/pull/13647)
* [rust-analyzer: editores/código: cambiar el código VS mínimo de 1.78 a 1.83](https://github.com/rust-lang/rust-analyzer/pull/18486)
* [Rust-analyzer: use índices de elementos de finalización en lugar de coincidencia de propiedades al buscar el elemento de finalización a resolver](https://github.com/rust-lang/rust-analyzer/pull/18503)

### Clasificación del rendimiento del compilador de Rust

Regresiones principalmente en compilaciones de documentos. No hay cambios significativos en el ciclo o max-rss
Cuenta.

Triaje realizado por **@simulacrum**.
Rango de revisión: [27e38f8f.. D4822C2D](https://perf.rust-lang.org/?start=27e38f8fc7efc57b75e9a763d7a0ee44822cd5f7&end=d4822c2d84c242cc7403118b50c571464f38ef8f&absolute=false&stat=instructions%3Au)

1 Regresiones, 1 Mejoras, 4 Mixtas; 1 de ellos en rollups
47 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-11-11.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [[RFC] Gancho de generación de hilos (heredando locales de hilos)](https://github.com/rust-lang/rfcs/pull/3642)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para 'const_size_of_val' y 'const_align_of_val'](https://github.com/rust-lang/rust/issues/46571)
* [disposición: fusionar] [marcar is_val_statically_known intrínseco como establemente invocable](https://github.com/rust-lang/rust/pull/132449)
* [disposición: fusionar] [Problema de seguimiento para 'const <*const T>::is_null'](https://github.com/rust-lang/rust/issues/74939)
* [disposición: fusionar] [Problema de seguimiento para los métodos const 'Pin'](https://github.com/rust-lang/rust/issues/76654)
* [disposición: fusionar] [Estabilizar 'const_atomic_from_ptr'](https://github.com/rust-lang/rust/pull/131717)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [feat(resolver): Estabilizar resolver v3](https://github.com/rust-lang/cargo/pull/14754)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Discriminantes de enumeración de conjuntos inseguros](https://github.com/rust-lang/rfcs/pull/3727)

## Próximos eventos

Eventos oxidados entre 2024-11-13 - 2024-12-11 🦀

### Virtual
* 14/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 14/11/2024 | Virtual y presencial (Lehi, UT, EE. UU.) | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Pulgar verde: Construyendo un riego de plantas habilitado para Bluetooth con Rust y microbit**](https://www.meetup.com/utah-rust/events/304206130/)
* 14/11/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Encuentro de noviembre**](https://www.meetup.com/join-srug/events/304166747/)
* 15/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 19/11/2024 | Virtual (Los Ángeles, CA, EE. UU.) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discusión - Tema: Rust para UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 19/11/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346971/)
* 20/11/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Rust para el Club de Lectura de los Rustáceos: Capítulo 12: Rust sin la Biblioteca Estándar**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/304441931/)
* 20/11/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Taller de Rust incrustado**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 21/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633273/)
* 21/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**IoT confiable con Rust... ¡y contraseñas!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 21/11/2024 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 25/11/2024 | Virtual (Bratislava, SK) | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Charla ONLINE, patrocinada por Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 26/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 28/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 28/11/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 03/12/2024 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 04/12/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031652)
* 05/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633275/)
* 2024-12-10 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346988/)
* 11/12/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/304047666/)

### África

* 07/12/2024 | Virtual( Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    
### Asia
* 28/11/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Cumbre RustTechX 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 30/11/2024 | Tokio, JP | [Rust de Tokio](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europa
* 13/11/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 14/11/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 19/11/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 19/11/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #72**](https://www.meetup.com/rust-paris/events/304396616/)
* 21/11/2024 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 21/11/2024 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**Taller de introducción a unit testing en Rust**](https://www.meetup.com/madrust/events/304484962/)
* 21/11/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 23/11/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Taller #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 26/11/2024 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/events/)
    * [**New Rust Warsaw Meetup #3**](https://www.meetup.com/rust-warsaw/events/304379707/)
* 27/11/2024 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 28/11/2024 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 28/11/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 28/11/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421381/)
* 28/11/2024 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 28/11/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn con Mainmatter y Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 28/11/2024 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (noviembre de 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 03/12/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advenimiento del Código**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 04/12/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 05/12/2024 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Encuentro de Rust Moravia (diciembre de 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 06/12/2024 | Moscú, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Rusia**](https://rustcon.ru/)
* 11/12/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)

### América del Norte
* 14/11/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/hackerdojo/events/304211045/)
* 14/11/2024 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**PDXRust noviembre 2024: ¡Conversaciones relámpago!**](https://www.meetup.com/pdxrust/events/304500461/)
* 15/11/2024 | Ciudad de México, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 15/11/2024 | Somerville, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, 15 de noviembre**](https://www.meetup.com/bostonrust/events/303708398/)
* 19/11/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638252/)
* 19/11/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Construyendo tu primera interfaz de línea de comandos - Un taller de código a lo largo**](https://www.meetup.com/spokane-rust/events/304457352/)
* 23/11/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust común de Boston, 23 de noviembre**](https://www.meetup.com/bostonrust/events/303708407/)
* 25/11/2024 | Ferndale, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 26/11/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530470/)
* 27/11/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 28/11/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/304468157/)
* 05/12/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cuerdas de Rust**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)

### Oceanía
* 08/12/2024 | Canberra, Australia | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Fiesta de Navidad CRUG**](https://www.meetup.com/rust-canberra/events/304282046/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1gf5ue1/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Netstack3 abarca 63 cajas y 60 años de desarrollo de código. Contiene más código que las diez cajas principales de [crates.io](https://crates.io/) combinadas. ... Durante los últimos once meses, han estado ejecutando la nueva pila de redes en 60 dispositivos, a tiempo completo. En ese tiempo, dijo Liebow-Feeser, se habría esperado que la mayoría del código mostrara "montañas de errores". Netstack3 solo tenía tres; Atribuyó ese bajo número al enfoque del equipo de codificar tantas invariantes importantes en el sistema de tipos como fuera posible.

– [Joshua Liebow-Feeser en RustConf, según lo informado por Daroc Alden en Linux Weekly News](https://lwn.net/SubscriberLink/995814/17e451bcb3015920/)

¡Gracias a [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1630) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](REDDIT_LINK_HERE)</small>
