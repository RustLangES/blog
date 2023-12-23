---
title: "Esta semana en Rust #11"
number_of_week: 11
description: El crate de esta semana es constcat, un reemplazo de 'std::concat!', con soporte para variables y expresiones const.
date: 2023-12-20
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
* [Blog: Lanzamiento de la Encuesta sobre el estado de la roya 2023](https://blog.rust-lang.org/2023/12/18/survey-launch.html) | [Enlace directo a la encuesta](https://www.surveyhero.com/c/4vxempzc)
* [Convocatoria de propuestas para la edición de Rust 2024](https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html)

### Actualizaciones de proyectos/herramientas
* [ratatui: una librería de Rust para cocinar interfaces de usuario de terminal - v0.25.0](https://ratatui.rs/highlights/v025/)
* [Presentamos a Gooey: Mi opinión sobre un marco de interfaz gráfica de usuario oxidado](https://ecton.dev/introducing-gooey/)
* [Dos nuevas cajas de Rust de código abierto facilitan la gestión de políticas de cedro](https://aws.amazon.com/blogs/opensource/easier-cedar-policy-management/)
* [Presentación de FireDBG - un depurador visual de viajes en el tiempo para Rust](https://firedbg.sea-ql.org/blog/2023-12-12-introducing-firedbg/)
* [Fornjot 0.48.0 - kernel CAD b-rep de código abierto escrito en Rust](https://www.fornjot.app/blog/release/0.48.0/)
* [Comprometerse con Rust para el código del kernel](https://lwn.net/Articles/952029/)
* [Una implementación de Rust de Binder de Android](https://lwn.net/Articles/953116/)
* [Prevención de violaciones del contexto atómico en el código de Rust con klint](https://lwn.net/Articles/951550/)
* [Rust para Linux — en el espacio](https://lwn.net/Articles/954974/)

### Observaciones/Pensamientos
* [La roya está creciendo](https://flawless.dev/essays/rust-is-growing/)
* [Un problema de por vida curiosamente recurrente](https://blog.dureuill.net/articles/recurring-lifetime/)
* [La madriguera del conejo de los inseguros insectos de Rust](https://notgull.net/cautionary-unsafe-tale/)
* [Cadenas de herramientas de Rust más rápidas para Android](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)
* [Los errores más comunes del compilador de Rust que se encuentran en RustRover: Parte 1](https://blog.jetbrains.com/rust/2023/12/14/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-1/)
* [Nueve reglas para la aceleración SIMD de su código Rust (Parte 2): Lecciones generales de aumentar la ingesta de datos en la caja range-set-fire en 7x](https://medium.com/towards-data-science/nine-rules-for-simd-acceleration-of-your-rust-code-part-2-6a104b3be6f3)
* [Lo que aprendí haciendo un controlador de hal incrustado en Rust (para el digitalizador de termopar MAX6675)](https://barretts.club/posts/max6675-hal/)

### Tutoriales de Rust
* [Rust: Rasgos](https://priver.dev/blog/rust/traits/)
* [Escribir una VPN de juguete en Rust](https://write.yiransheng.com/vpn)
* [Primeros pasos con Actix Web en Rust](https://www.shuttle.rs/blog/2023/12/15/using-actix-rust)
* [Primeros pasos con Rocket en Rust](https://www.shuttle.rs/blog/2023/12/13/using-rocket-rust)
* [Tipos genéricos para parámetros de función en Rust 🦀 ](https://rust.code-maven.com/generic-types-for-simple-function)
* [Benchmarking Rust Compiler Settings with Criterion: Controlling Criterion with Scripts and Environment Variables](https://medium.com/towards-data-science/benchmarking-rust-compiler-settings-with-criterion-62db50cd62fb)
* [series] [Multithreading and Memory-Mapping: Refining ANN Performance with Arroy](https://blog.kerollmops.com/multithreading-and-memory-mapping-refining-ann-performance-with-arroy)  
* [series] [Introducción a la creación de una aplicación web en Rust por parte de Tiny HTTP](https://rust.code-maven.com/tiny-http)

### Miscelánea
* [Educación sobre el Rust incrustado: reflexiones y visiones de 2023](https://apollolabsblog.hashnode.dev/embedded-rust-education-2023-reflections-2024-visions)
* [Argumentos predeterminados para funciones en Rust usando macros](https://rust.code-maven.com/default-arguments-for-functions)
* [audio] [Rust in Production Ep 1 - InfluxData's Paul Dix](https://www.youtube.com/watch?v=DutVRGs9oZc)
* [audio] [Episodio 160: Rust y seguridad en Adobe con Sean Parent](https://adspthepodcast.com/2023/12/15/Episode-160.html)

## Crate de la semana

El crate de esta semana es [constcat](https://crates.io/crates/constcat), un reemplazo de 'std::concat!', con soporte para variables y expresiones const.

¡Gracias a [Ross MacArthur](https://users.rust-lang.org/t/crate-of-the-week/2704/1272) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Ockam - Corregir advertencias de documentación](https://github.com/build-trust/ockam/issues/7145)
* [Ockam - Biblioteca - Validar estructuras CBOR de acuerdo con el esquema cddl para 'nodos/models/secure_channel'](https://github.com/build-trust/ockam/issues/6692)
* [Ockam - Implementar eventos en 'SqlxDatabase'](https://github.com/build-trust/ockam/issues/7116)
* [Hyperswitch - [REFACTOR]: [Nuvei] Validación de metadatos MCA](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [CARACTERÍSTICA] : [Mediodía] Sincronización con Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [CARACTERÍSTICA]: [Zen] Sincronización con Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2908)
* [Hyperswitch - [REFACTOR] : [Authorizedotnet] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2909)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

386 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-13..2023-12-20

* [habilitar sondas de pila en aarch64 para LLVM 18](https://github.com/rust-lang/rust/pull/118491)
* [Se ha añadido un nuevo objetivo aarch64-apple-watchos de nivel 3](https://github.com/rust-lang/rust/pull/119074)
* [añadir soporte hexagonal](https://github.com/rust-lang/compiler-builtins/pull/556)
* [añadir el intervalo del cuerpo de la función a StableMIR](https://github.com/rust-lang/rust/pull/119100)
* [permitir rasgos 'async_fn_in_trait' con la variante de envío](https://github.com/rust-lang/impl-trait-utils/pull/6)
* [cherry-pick "M68k: Fix ODR violation in GISel code (#72797)"](https://github.com/rust-lang/llvm-project/pull/159)
* [AIX: corregir metadatos XCOFF](https://github.com/rust-lang/rust/pull/118905)
* ['-Ztrait-solver=next' a '-Znext-solver'](https://github.com/rust-lang/rust/pull/118937)
* [analizar correctamente los bloques de generación asíncronos](https://github.com/rust-lang/rust/pull/118891)
* [añadir un método a StableMIR para comprobar si un tipo es un CStr](https://github.com/rust-lang/rust/pull/119000)
* [añadir más sugerencias a nombres y valores inesperados de cfg](https://github.com/rust-lang/rust/pull/118213)
* [añadir soporte para '--env' en 'tracked_env::var'](https://github.com/rust-lang/rust/pull/118830)
* [add unstable '-Zdefault-hidden-visibility' cmdline flag for 'rustc'](https://github.com/rust-lang/rust/pull/118417)
* [anotar razones de pánico durante el diseño de enumeración](https://github.com/rust-lang/rust/pull/118974)
* [intento de resolver problemas de bloqueo](https://github.com/rust-lang/rust/pull/117050) (RFC [#3086](https://rust-lang.github.io/rfcs/3086-macro-metavar-expr.html))
* [evitar el desbordamiento en la indexación de constantes GVN](https://github.com/rust-lang/rust/pull/119052)
* [cache param env canonicalization](https://github.com/rust-lang/rust/pull/117749)
* [marque correctamente los rasgos fn incorporados 'FnPtr'/'FnDef' con efectos](https://github.com/rust-lang/rust/pull/119023)
* [comprobar los parámetros genéricos después de la Sigatura para main-fn-ty](https://github.com/rust-lang/rust/pull/119047)
* [recolectar elementos de idioma de AST, deshacerse de 'GenericBound::LangItemTrait'](https://github.com/rust-lang/rust/pull/118396)
* [los campos de variantes de corrutina se pueden desinicializar](https://github.com/rust-lang/rust/pull/118871)
* [cobertura: omitir la instrumentación de una función si no se extrajeron intervalos de MIR](https://github.com/rust-lang/rust/pull/118852)
* [denegar los límites del rasgo '~const' en los encabezados impl inherentes](https://github.com/rust-lang/rust/pull/119059)
* [desazucarar 'yield' en 'async gen' correctamente, asegurarse de que 'gen' siempre devuelva la unidad](https://github.com/rust-lang/rust/pull/119061)
* [No fusionar los atributos cfg y doc(cfg) para reexportaciones](https://github.com/rust-lang/rust/pull/113091)
* [borrar las regiones enlazadas en tiempo de ejecución de 'Instance::fn_sig()' y añadir algunos detalles más a las API de StableMIR](https://github.com/rust-lang/rust/pull/118927)
* [arreglar ICE 'ProjectionTypes Deref y Field no coincidían'](https://github.com/rust-lang/rust/pull/118584)
* [arreglar las banderas de subprocesos LLD en el arranque en Windows](https://github.com/rust-lang/rust/pull/118906)
* [Corregir el número de problema de seguimiento 'waker_getters'](https://github.com/rust-lang/rust/pull/118873)
* [arreglar la alineación pasada a LLVM para 'simd_masked_load'](https://github.com/rust-lang/rust/pull/118864)
* [Se corrigió el tamaño dinámico/alinear la lógica de cálculo para tipos empaquetados con cola de rasgo Dyn](https://github.com/rust-lang/rust/pull/118538)
* [Se corrigen los intervalos superpuestos en las meta-vars delimitadas](https://github.com/rust-lang/rust/pull/118928)
* [ICE 110453: corregido con errores](https://github.com/rust-lang/glacier/pull/1702)
* [llvm-wrapper: adaptarse a los cambios de la API de LLVM](https://github.com/rust-lang/rust/pull/118941)
* [convertir 'IMPLIED_BOUNDS_ENTAILMENT' en un error grave de una pelusa](https://github.com/rust-lang/rust/pull/117984)
* [hacer que la exhaustividad sea utilizable fuera de rustc](https://github.com/rust-lang/rust/pull/118842)
* [Reducción de partidos: Eliminar el truco 'make_target_blocks'](https://github.com/rust-lang/rust/pull/119112)
* [Más expresiones correctamente están marcadas para terminar con llaves](https://github.com/rust-lang/rust/pull/118880)
* [empujar al usuario a matar programas usando CPU excesiva](https://github.com/rust-lang/rust-playground/pull/1020)
* [resolver de forma oportunista la variable de la región en canonicalizer (en lugar de resolver la variable raíz)](https://github.com/rust-lang/rust/pull/118964)
* [rechazar correctamente 'default' en elementos const libres](https://github.com/rust-lang/rust/pull/117818)
* [eliminar la constancia innecesaria de ProjectionCandidate](https://github.com/rust-lang/rust/pull/119022)
* [reemplace algunas instancias de 'FxHashMap'/'FxHashSet' con alternativas estables (principalmente en 'rustc_hir' y 'rustc_ast_lowering')](https://github.com/rust-lang/rust/pull/119093)
* [Resolver: reemplazar la tabla de visibilidad en las salidas de resolución con alimentación de consultas](https://github.com/rust-lang/rust/pull/118657)
* [omitir el comprobador de restricciones de RPIT si se produce un error de tipo de retorno de Borrowck](https://github.com/rust-lang/rust/pull/117884)
* [Alguna limpieza y mejora para la impl de conversión de referencias no válidas](https://github.com/rust-lang/rust/pull/118909)
* [Ajustar 'short_ty_string' para reducir el número de archivos](https://github.com/rust-lang/rust/pull/118389)
* [registrar incondicionalmente alias-relate en el objetivo de la proyección](https://github.com/rust-lang/rust/pull/118914)
* [actualizar la imagen CI de FreeBSD](https://github.com/rust-lang/stdarch/pull/1507)
* [aumentar 'TypeAndMut' y 'ClosureKind' a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/118888)
* [use 'if cfg!' en lugar de '#[cfg]'](https://github.com/rust-lang/rust/pull/118993)
* [use la opción LLVM NoTrapAfterNoreturn](https://github.com/rust-lang/rust/pull/110494)
* [miri: visita los AllocIds y BorTags en estado de préstamo FrameExtra](https://github.com/rust-lang/miri/pull/3229)
* [Miri Run: por defecto a la edición 2021](https://github.com/rust-lang/miri/pull/3221)
* [Miri: Hacer que mmap no use exponer semántica](https://github.com/rust-lang/miri/pull/3220)
* [ruta rápida para 'declared_generic_bounds_from_env'](https://github.com/rust-lang/rust/pull/119084)
* [estabilizar 'type_name_of_val'](https://github.com/rust-lang/rust/pull/118234)
* [estabilizar 'ptr::{from_ref, from_mut}'](https://github.com/rust-lang/rust/pull/117824)
* [añadir 'core::intrinsics::simd'](https://github.com/rust-lang/rust/pull/118853)
* [Agregue un número de columna a 'dbg! ()»(https://github.com/rust-lang/rust/pull/114962)
* [añadir más nichos a 'rawvec'](https://github.com/rust-lang/rust/pull/106790)
* [añadir funciones de recorte de espacios en blanco ASCII a '&str'](https://github.com/rust-lang/rust/pull/118523)
* [Se corrigieron los casos en los que STD se basaba accidentalmente en Inline(Never)](https://github.com/rust-lang/rust/pull/118770)
* [Windows: permitir que 'File::create' funcione en archivos ocultos](https://github.com/rust-lang/rust/pull/116438)
* [std: añadir xcoff en la lista de características del objeto](https://github.com/rust-lang/rust/pull/118851)
* [codegen: pánico al intentar calcular el tamaño/alineación del tipo externo](https://github.com/rust-lang/rust/pull/118534)
* [codegen\_gcc: simd: implementa los intrínsecos faltantes de simd/generic-arithmetic-pass.rs](https://github.com/rust-lang/rustc_codegen_gcc/pull/382)
* [codegen\_llvm: establece 'DW_AT_accessibility'](https://github.com/rust-lang/rust/pull/115165)
* [Cargo: Limpiar los metadatos del paquete](https://github.com/rust-lang/cargo/pull/13184)
* [cargo: no permitir nombre vacío en la especificación de identificación del paquete](https://github.com/rust-lang/cargo/pull/13152)
* [cargo: rellena más huecos de nombre vacíos](https://github.com/rust-lang/cargo/pull/13164)
* [Carga: mantén el bloqueo exclusivo de mutar al vender](https://github.com/rust-lang/cargo/pull/12509)
* [rustdoc: use Map en lugar de Object para los archivos fuente y el índice de búsqueda](https://github.com/rust-lang/rust/pull/118910)
* [rustdoc: permite cambiar el tamaño de la barra lateral / ocultar la barra superior](https://github.com/rust-lang/rust/pull/115660)
* [rustdoc-search: se corrige una condición de carrera en la carga del índice de búsqueda](https://github.com/rust-lang/rust/pull/118961)
* [rustdoc-search: use set ops para clasificar y filtrar](https://github.com/rust-lang/rust/pull/118402)
* [bindgen: use '\r\n' en Windows](https://github.com/rust-lang/rust-bindgen/pull/2698)
* [bindgen: destructores que funcionan mejor en Windows](https://github.com/rust-lang/rust-bindgen/pull/2663)
* [clippy: añadir nueva pelusa 'unconditional_recursion'](https://github.com/rust-lang/rust-clippy/pull/11938)
* [clippy: new Lint: 'result_filter_map' / Mirror of 'option_filter_map'](https://github.com/rust-lang/rust-clippy/pull/11869)
* [clippy: no visites cuerpos anidados en 'is_const_evaluatable'](https://github.com/rust-lang/rust-clippy/pull/11977)
* [clippy: 'redundant_pattern_matching': lint 'if let true', 'while let true', 'matches! (.., verdadero)'](https://github.com/rust-lang/rust-clippy/pull/11974)
* [clippy: no peluar 'assertions_on_constants' por 'const _: () = assert! (expr)»](https://github.com/rust-lang/rust-clippy/pull/11966)
* [clippy: 'doc_markdown' Reconoce palabras seguidas de paréntesis vacíos '()' para citar](https://github.com/rust-lang/rust-clippy/pull/11956)
* [clippy: arreglar el manejo de la carpeta en 'unnecessary_to_owned'](https://github.com/rust-lang/rust-clippy/pull/11953)
* [rust-analyzer: deduplicar anotaciones](https://github.com/rust-lang/rust-analyzer/pull/16163)
* [rust-analyzer: optimizando el rendimiento con 'Promise.all' 🏎 ](https://github.com/rust-lang/rust-analyzer/pull/16162)
* [Rust-Analyzer: Desugar Doc correctamente para MBE](https://github.com/rust-lang/rust-analyzer/pull/16158)
* [rust-analyzer: no asumas ASCII en 'remove_markdown'](https://github.com/rust-lang/rust-analyzer/pull/16155)
* [rust-analyzer: resolver alias antes de resolver la variante de enumeración](https://github.com/rust-lang/rust-analyzer/pull/16152)
* [Rust-Analyzer: Agregue soporte mínimo para la edición 2024](https://github.com/rust-lang/rust-analyzer/pull/16151)
* [rust-analyzer: mover 'WithFixture' a la caja solo dev-dep](https://github.com/rust-lang/rust-analyzer/pull/16150)
* [Rust-analyzer: Corregir la discordancia de tipo falso positivo en los patrones de referencia const](https://github.com/rust-lang/rust-analyzer/pull/16131)
* [rust-analyzer: la corrección de sintaxis ahora elimina los subárboles con intervalos falsos](https://github.com/rust-lang/rust-analyzer/pull/16130)
* [Rust-analyzer: Actualizar los attrs incorporados de rustc](https://github.com/rust-lang/rust-analyzer/pull/16115)
* [rust-analyzer: arreglar el analizador de fragmentos que reemplaza las coincidencias con dummies en análisis incompletos](https://github.com/rust-lang/rust-analyzer/pull/16061)
* [rust-analyzer: se corrige el reemplazo incorrecto de referencias en la invocación de macros en la asistencia "Convertir a estructura con nombre"](https://github.com/rust-lang/rust-analyzer/pull/15887)

### Clasificación del rendimiento del compilador de Rust

Mucho ruido en los resultados de esta semana; Hubo una pausa en el ruido
recientemente, por lo que nuestro umbral de ruido autoinferido bajó y, por lo tanto, cinco PR
fueron marcados artificialmente esta semana (y tres supuestas mejoras fueron
volver a la media). Más allá de eso, tuvimos tres buenas mejoras: la primera
para depurar compilaciones en #117962 (dejando de emitir costosos + no utilizados
'.debug_pubnames' y '.debug_pubtypes'), un segundo después de diesel y serde
en #119048 (evitando algún trabajo innecesario), y un tercero a varios puntos de referencia
en #117749 (agregando algo de almacenamiento en caché de una estructura interna del compilador).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [57010939..bf9229a2](https://perf.rust-lang.org/?start=57010939ed1d00076b4af0ed06a81ec69ea5e4a8&end=bf9229a2e366b4c311f059014a4aa08af16de5d8&absolute=false&stat=instructions%3Au)

6 regresiones, 9 mejoras, 3 mixtas; 5 de ellos en rollups
67 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/b9ecf1aba002cd6b33d06f784e088839636d7e92/triage/2023-12-18.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: posponer] [RFC: Precise Pre-release Deps](https://github.com/rust-lang/rfcs/pull/3263)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Admite llamadas recursivas asíncronas (siempre que tengan direccionamiento indirecto)](https://github.com/rust-lang/rust/pull/117703)
* [Disposición: Fusionar] [Hacer que soft_unstable aparezcan en futuros informes de roturas](https://github.com/rust-lang/rust/pull/116274)
* [disposición: fusionar] [Problema de seguimiento para ip_in_core](https://github.com/rust-lang/rust/issues/108443)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: patchable-function-entry](https://github.com/rust-lang/rfcs/pull/3543)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-12-20 - 2024-01-17 🦀

### Virtual

* 20/12/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Aventuras en el desarrollo de aplicaciones egui**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 26/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 28/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 03/01/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 09/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fvdtgtygccbmb/)
* 11/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 16/01/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/297128172/)

### Europa

* 27/12/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs y peluches**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)
* 28/12/2023 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 3: Edición Navideña**](https://www.meetup.com/rust-vienna/events/297826979/)
* 11/01/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 11/01/2024 | Wrocław, PL | [Rust de Breslavia](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 13/01/2024 | Helsinki, FI | [Grupo Rust-lang de Finlandia](https://www.meetup.com/finland-rust-meetup/)
    * [**Encuentro de enero**](https://www.meetup.com/finland-rust-meetup/events/297811750/)

### América del Norte

* 20/12/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297818036/)
* 27/12/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 06/01/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Beacon Hill**](https://www.meetup.com/bostonrust/events/297633937/)
* 08/01/2024 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 09/01/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Noche de Codificación/Hackeo/Aprendizaje Oxidado**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 09/01/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 14/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 16/01/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 17/01/2024 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/298003233/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/182f6dv/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> El satélite Tianyi-33 es un satélite experimental de ciencia espacial de 50 kg equipado con un sistema operativo desarrollado de forma independiente por la Universidad de Correos y Telecomunicaciones de Pekín: el sistema operativo en tiempo real de doble kernel basado en Rust **RROS**. RROS llevará a cabo tareas generales representadas por tensorflow/k8s y tareas en tiempo real representadas por sistemas de archivos en tiempo real y transmisión de red en tiempo real en el satélite. Garantizará la ejecución normal de las aplicaciones de capa superior y las tareas de investigación científica, como la medición del retardo de tiempo entre el satélite y la tierra, la transmisión de vídeo en directo, los servicios de chat web a bordo, los experimentos pseudo-SSH, etc. Esto marca la primera aplicación oficial del mundo de un sistema operativo de doble kernel escrito por Rust en un escenario satelital.

– [Qichen en la página web de RROS](https://bupt-os.github.io/website/news/2023_12_9/satellite_launch/)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1496) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18ndmbb/this_week_in_rust_526/)</small>
