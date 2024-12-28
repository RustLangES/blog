---
title: "Esta semana en Rust #41"
number_of_week: 41
description: El crate de esta semana es OmniLED.
date: 2024-12-25
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

### Boletines

* [El Rustáceo Incrustado Edición #35](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-35)
* [Tendencias de Rust Edición #56](https://rust-trends.com/newsletter/unlocking-new-insights-and-opportunities-in-rust/)

### Actualizaciones de proyectos/herramientas

* [Lanzamiento de Musi Lili Retro Game Engine 0.1](https://crates.io/crates/musi_lili)
* [Nutype 0.5.1: mejor soporte no_std y corrección de errores](https://github.com/greyblake/nutype/releases/tag/v0.5.1)
* [Ibis 0.2.0 - Wiki federado con rediseño brillante, basado en Diesel, Actix y Leptos](https://ibis.wiki/article/Ibis_release_0.2.0_-_Federated_Wiki_with_Shiny_Redesign@ibis.wiki)
* [dagrs 0.4.2 - Programación basada en flujo en Rust](https://crates.io/crates/dagrs)
* [Física Aviar 0.2](https://joonaa.dev/blog/07/avian-0-2)
* [Slint 1.9](https://slint.dev/blog/slint-1.9-released)
* [Gitoxide - Diciembre 2024](https://github.com/GitoxideLabs/gitoxide/discussions/1738)

### Observaciones/Pensamientos

* [Caída de Hiper](https://daniel.haxx.se/blog/2024/12/21/dropping-hyper/)
* [Incrustación de Lua en sqleibniz con Rust](https://xnacly.me/posts/2024/embed-lua-in-rust/)
* [¿Qué se necesitaría para agregar tipos de refinamiento a Rust?](https://yoric.github.io/post/rust-refinement-types/)
* [Programación genérica contextual](https://contextgeneric.dev/blog/early-preview-announcement/)
* [Construcción de un sistema seguro de derivación de claves jerárquicas en Rust](https://medium.com/@evadawnley/building-a-secure-hierarchical-key-derivation-system-in-rust-b5a0ecee18d7)
* [Código de análisis de flujo de datos simplificado en rustc](https://nnethercote.github.io/2024/12/19/streamlined-dataflow-analysis-code-in-rustc.html)
* [Cuatro limitaciones del verificador de préstamos de Rust](https://blog.polybdenum.com/2024/12/21/four-limitations-of-rust-s-borrow-checker.html)
* [Una revisión de Rust en 2024: ¿qué sigue?](https://barretts.club/posts/rust_review_2024/)
* [Mi acción actions-rust-cross ahora tiene almacenamiento en caché incorporado](https://blog.urth.org/2024/12/21/my-actions-rust-cross-action-now-has-built-in-caching/)
* [Transformando el desarrollo de Rust: Cómo la IA del windsurf triplicó la productividad](https://neoexogenesis.com/posts/rust-windsurf-transformation/)

### Tutoriales de Rust

* [Cómo construir y publicar binarios de Rust multiplataforma a través de acciones de Github](https://rakhim.exotext.com/how-to-build-and-publish-multi-platform-rust-binaries)
* [Compilando C para Safe Rust, formalizado](https://arxiv.org/abs/2412.15042)
* [Optimización innecesaria en Rust: distancias de Hamming, SIMD y autovectorización](https://emschwartz.me/unnecessary-optimization-in-rust-hamming-distances-simd-and-auto-vectorization/)

### Miscelánea

* [Ejercicios del curso universitario de Rust](https://kobzol.github.io/teaching/2024/12/18/rust-exercises.html)
* [video] [La reescritura de SQLite en Rust](https://www.youtube.com/watch?v=PPjXM8G8ax0)

## Crate de la semana

El crate de esta semana es [OmniLED](https://github.com/llMBQll/OmniLED), un ayudante para mostrar cosas como la hora o el volumen de audio en una matriz de LED que tienen algunos periféricos (como los teclados de juegos).

¡Gracias a [llogiq](https://users.rust-lang.org/t/crate-of-the-week/2704/1383) por la sugerencia!

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
* [CfT: Pruebe el backend 'reqwest' de Rustup con 'rustls'](https://github.com/rust-lang/rustup/issues/3806)
  - [Pasos de prueba](https://github.com/rust-lang/rustup/issues/3806#issue-2278962476)

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

* [Rama — proporciona constantes para hosts comunes (de red) al host de rama-net](https://github.com/plabayo/rama/issues/363)
* [Rama — soporte vec/array impl para DnsResolver](https://github.com/plabayo/rama/issues/332)
* [Rama — soporta la capa del exportador HAR (http) en rama](https://github.com/plabayo/rama/issues/357)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 398 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-17..2024-12-24

* [-Znext-solver: modificar las reglas de preferencia del candidato](https://github.com/rust-lang/rust/pull/133643)
* ['Variants::Single': no use VariantIdx no válido para enumeraciones deshabitadas](https://github.com/rust-lang/rust/pull/133702)
* ['rustc_borrowck': Sugerir cambiar '&raw const' por '&raw mut' si corresponde](https://github.com/rust-lang/rust/pull/134397)
* [abstraer 'ProcThreadAttributeList' en su propia 'struct'](https://github.com/rust-lang/rust/pull/123604)
* [añadir la opción '--doctest-compilation-args' para añadir banderas de compilación a la compilación doctest](https://github.com/rust-lang/rust/pull/128780)
* [add 'ignore-rustc-debug-assertions' a 'tests/ui/associated-consts/issue-93775.rs'](https://github.com/rust-lang/rust/pull/134608)
* [alinear '{i686,x86_64}-win7-windows-msvc' a sus objetivos principales](https://github.com/rust-lang/rust/pull/134611)
* [también lint en la opción de comparaciones de punteros de función](https://github.com/rust-lang/rust/pull/134586)
* [siempre ejecute 'tail_expr_drop_order' lint en la consulta MIR promocionada](https://github.com/rust-lang/rust/pull/134493)
* [comenzar a implementar la capa del sistema de tipos de aglutinantes inseguros](https://github.com/rust-lang/rust/pull/134625)
* [limpieza 'TypeVerifier'](https://github.com/rust-lang/rust/pull/134465)
* [manejo de regiones de limpieza: agregue 'LateParamRegionKind'](https://github.com/rust-lang/rust/pull/133961)
* [anotar correctamente el tipo de elemento en el mensaje de error 'NonConstFunctionCall'](https://github.com/rust-lang/rust/pull/134701)
* [Cobertura: desmantelar 'map_data.rs' trasladando sus responsabilidades a otro lugar](https://github.com/rust-lang/rust/pull/134323)
* [coverage: almacena las regiones de origen de cobertura como 'Span' hasta codegen (toma 2)](https://github.com/rust-lang/rust/pull/134497)
* [Detectar exprs no válidos en el analizador utilizado por las pruebas de pretty-printer](https://github.com/rust-lang/rust/pull/134599)
* [detectar '.' faltante en la cadena de métodos en enlaces y sentencias 'let'](https://github.com/rust-lang/rust/pull/133087)
* [no hagas hielo en lances ilegales de 'dyn*'](https://github.com/rust-lang/rust/pull/134635)
* [explique por qué un tipo no es elegible para 'impl PointerLike'](https://github.com/rust-lang/rust/pull/134603)
* [arreglar el orden de '-Z input-stats'](https://github.com/rust-lang/rust/pull/134406)
* [arreglar las condiciones de const para RPITITs](https://github.com/rust-lang/rust/pull/133926)
* [arreglar predicados de efecto de los límites de los elementos en el solucionador antiguo](https://github.com/rust-lang/rust/pull/134638)
* [Corregir error lógico con qué texto se considera espacio en blanco](https://github.com/rust-lang/rust/pull/134366)
* [corregido el paréntesis de las comparaciones encadenadas por pretty-printer](https://github.com/rust-lang/rust/pull/134600)
* [prohibir sobrescribir tipos en typeck](https://github.com/rust-lang/rust/pull/134474)
* [Fundamentos de Polonio sensible a la ubicación](https://github.com/rust-lang/rust/pull/134268)
* [maneja 'DropKind::ForLint' en corrutinas correctamente](https://github.com/rust-lang/rust/pull/134575)
* [manejar el renderizado fndef junto con el renderizado de firma](https://github.com/rust-lang/rust/pull/134354)
* [manejar las restricciones de los miembros directamente en el verificador de tipos mir](https://github.com/rust-lang/rust/pull/134501)
* [ocultar '= _' como valor constante asociado dentro de los bloques impl](https://github.com/rust-lang/rust/pull/134321)
* [asegúrese de que no perdemos el valor predeterminado de 'struct' al formatear 'struct'](https://github.com/rust-lang/rust/pull/134668)
* [asegúrese de que manejamos las gotas 'backwards_incompatible_lint' apropiadamente en la elaboración de gotas](https://github.com/rust-lang/rust/pull/134486)
* [asegúrese de anotar las causas de ambigüedad en los conflictos de implicaciones positivas/negativas](https://github.com/rust-lang/rust/pull/134639)
* [mover 'lint_unused_mut' a sub-fn](https://github.com/rust-lang/rust/pull/134477)
* [next-solver: deshabilitar hack innecesario](https://github.com/rust-lang/rust/pull/134574)
* [pasa FnAbi a 'find_mir_or_eval_fn'](https://github.com/rust-lang/rust/pull/133103)
* [señale el nombre de la pelusa en lugar de la letra entera para las pelusas cerradas](https://github.com/rust-lang/rust/pull/134481)
* [Mejoras de precedencia: cierres y saltos](https://github.com/rust-lang/rust/pull/133782)
* [Promover powerpc64le-unknown-linux-musl al nivel 2 con herramientas de host](https://github.com/rust-lang/rust/pull/133801)
* [reexportar más cosas de 'rustc_span::symbol' de 'rustc_span'](https://github.com/rust-lang/rust/pull/134243)
* [reducir la cantidad de explícito 'FatalError.raise()'](https://github.com/rust-lang/rust/pull/134561)
* [restringir '#[non_exaustive]' en estructuras con valores de campo predeterminados](https://github.com/rust-lang/rust/pull/134539)
* [simplificar el manejo de 'SwitchInt'](https://github.com/rust-lang/rust/pull/133328)
* [Admite objetos de rasgo 'dyn*' de impresión bonita](https://github.com/rust-lang/rust/pull/134601)
* [use 'PtrMetadata' en lugar de 'Len' en las cuñas de caída de corte](https://github.com/rust-lang/rust/pull/134326)
* [usar enlaces a la guía de edición para migraciones de edición](https://github.com/rust-lang/rust/pull/134368)
* [win: use la semántica de cambio de nombre POSIX para 'std::fs::rename' si está disponible](https://github.com/rust-lang/rust/pull/131072)
* [mir-opt: un sub-BB de un BB de limpieza también debe ser un BB de limpieza en 'EarlyOtherwiseBranch'](https://github.com/rust-lang/rust/pull/130786)
* [Miri: añade 'track_caller' a los métodos de desove de hilos para mejorar los backtraces](https://github.com/rust-lang/rust/pull/134560)
* [Miri: añadir advertencia explicando las limitaciones del modo de código nativo](https://github.com/rust-lang/miri/pull/4098)
* [miri: implementar el bloqueo 'unnamed_socket'](https://github.com/rust-lang/miri/pull/4072)
* [Miri: implementa el modo de muchas semillas directamente en el controlador](https://github.com/rust-lang/miri/pull/4105)
* [Miri: proporcionar una forma de comparar los resultados de referencia con la línea de base](https://github.com/rust-lang/miri/pull/4104)
* [acelerar 'Parser::expected_tokens'](https://github.com/rust-lang/rust/pull/133793)
* [mejorar un poco 'dependency_format'](https://github.com/rust-lang/rust/pull/134514)
* [Revisar cursores de token](https://github.com/rust-lang/rust/pull/134161)
* [Arreglar la regresión de rendimiento en Rustdoc después de los atributos hir](https://github.com/rust-lang/rust/pull/134376)
* [estabilizar '#[diagnóstico::d o_no_recomendar]'](https://github.com/rust-lang/rust/pull/132056)
* [core: fix const 'ptr::swap_nonoverlapping' cuando hay punteros en desplazamientos impares](https://github.com/rust-lang/rust/pull/134689)
* [añadir un argumento de rango a 'vec.extract_if'](https://github.com/rust-lang/rust/pull/133265)
* [optimizar 'is_ascii' para 'str' y '[u8]' aún más](https://github.com/rust-lang/rust/pull/130733)
* [implemente 'PointerLike' para 'isize', 'NonNull', 'Cell', 'UnsafeCell' y 'SyncUnsafeCell'](https://github.com/rust-lang/rust/pull/134642)
* [desimplementar 'PointerLike' para objetos de rasgo](https://github.com/rust-lang/rust/pull/134573)
* [hashbrown: agregar implementación de SIMD de 128 bits para LoongArch](https://github.com/rust-lang/hashbrown/pull/592)
* [compiler-builtins: arregla un error en 'abs_diff'](https://github.com/rust-lang/compiler-builtins/pull/736)
* [cargo: build-std: hacer que Resolve se alinee con lo que se va a construir](https://github.com/rust-lang/cargo/pull/14938)
* [cargo: cargo-package: añadir más rastros](https://github.com/rust-lang/cargo/pull/14960)
* [cargo: cargo-rustc: estabilizar las banderas de arrastre de mayor precedencia](https://github.com/rust-lang/cargo/pull/14900)
* [cargo: paquete: mostrar rutas de archivo sucias relativas a git workdir](https://github.com/rust-lang/cargo/pull/14968)
* [Cargo: Paquete: use RelPath para CWD para el informe de suciedad de VCS](https://github.com/rust-lang/cargo/pull/14970)
* [cargo: añadir el cfg 'test' como un cfg bien conocido antes del cambio de compilador](https://github.com/rust-lang/cargo/pull/14963)
* [cargo: no hash de la ruta absoluta del sysroot en los metadatos de las cajas stdlib](https://github.com/rust-lang/cargo/pull/14951)
* [Cargo: Se corrigió el mensaje de error para que un usuario abriera la caja](https://github.com/rust-lang/cargo/pull/14969)
* [rustfmt: arreglar el prefijo r# de tira incorrecto de las etiquetas](https://github.com/rust-lang/rustfmt/pull/6425)
* [clippy: add allow-indexing-slicing-in-testsoption](https://github.com/rust-lang/rust-clippy/pull/13854)
* [clippy: no marcar atributos con espacio de nombres desconocido como inútiles](https://github.com/rust-lang/rust-clippy/pull/13766)
* [clippy: no dispares 'filter_map_identity' con un iterador de un array vacío](https://github.com/rust-lang/rust-clippy/pull/13826)
* [clippy: usa la sugerencia de varias partes en 'unnecessary_to_owned'](https://github.com/rust-lang/rust-clippy/pull/13847)
* [Rust-Analyzer: Borrar los diagnósticos de flycheck de forma más granular](https://github.com/rust-lang/rust-analyzer/pull/18729)
* [rust-analyzer: arreglar la regla AsmOption en rust.ungram](https://github.com/rust-lang/rust-analyzer/pull/18715)
* [rust-analyzer: considere ambos campos de detalle de finalización en 'to_proto'](https://github.com/rust-lang/rust-analyzer/pull/18716)
* [rust-analyzer: retrasar la comprobación inicial hasta después de los scripts de compilación](https://github.com/rust-lang/rust-analyzer/pull/18741)
* [Rust-analyzer: no le pida al cliente que resuelva por detalles de etiqueta no existentes](https://github.com/rust-lang/rust-analyzer/pull/18714)
* [rust-analyzer: no activar el controlador de escritura de envoltura de paréntesis después de las identificaciones](https://github.com/rust-lang/rust-analyzer/pull/18739)
* [rust-analyzer: arreglar un caso en el que la finalización no podía expandir una macro](https://github.com/rust-lang/rust-analyzer/pull/18723)
* [Rust-analyzer: arreglar los diagnósticos de verificación vacíos que no marcan los archivos como cambiados](https://github.com/rust-lang/rust-analyzer/pull/18740)
* [Rust-analyzer: arreglar el espacio de trabajo de flycheck cuando se solicitó pero se encontró el paquete](https://github.com/rust-lang/rust-analyzer/pull/18742)
* [Rust-analyzer: arreglar la impresión bonita de los patrones '@'](https://github.com/rust-lang/rust-analyzer/pull/18708)
* [Rust-analyzer: Compruebe correctamente si se permite el flychecking del espacio de trabajo](https://github.com/rust-lang/rust-analyzer/pull/18738)
* [Rust-Analyzer: reducir la aplicabilidad de la asistencia 'unnecessary_async'](https://github.com/rust-lang/rust-analyzer/pull/18726)
* [Rust-analyzer: eliminar la comprobación de 'siempre!' por 'file_id' en 'Runnables'](https://github.com/rust-lang/rust-analyzer/pull/18727)
* [rust-analyzer: eliminar salsa del árbol dep del servidor proc-macro](https://github.com/rust-lang/rust-analyzer/pull/18710)
* [Rust-analyzer: tomar una referencia en bruto de un deref siempre es seguro](https://github.com/rust-lang/rust-analyzer/pull/18711)

### Clasificación del rendimiento del compilador de Rust

La semana pasada nos perdimos la clasificación debido a algunos problemas del proceso, por lo que esta clasificación incluye dos semanas de datos. El resultado general es positivo, debido a las optimizaciones del analizador ([#133793](https://github.com/rust-lang/rust/pull/133793)), las optimizaciones de resolución de rasgos ([#134501](https://github.com/rust-lang/rust/pull/134501), [#132325](https://github.com/rust-lang/rust/pull/132325)) y el aumento de la caja cc ([#134505](https://github.com/rust-lang/rust/pull/134505)), que [mejoró el rendimiento](https://github.com/rust-lang/cc-rs/pull/1279) de las dependencias de C/C++ del compilador.

Triaje realizado por **@kobzol**.
Rango de revisión: [1b3fb316.. 0eca4dd3](https://perf.rust-lang.org/?start=1b3fb316751227d30b1523ed0e3f00d83956d4d0&end=0eca4dd3205a01dba4bd7b7c140ec370aff03440&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.3%, 0.8%] | 3 |
| Regresiones ❌ <br /> (secundaria) | 1.0% | [1.0%, 1.0%] | 1 |
| Mejoras ✅ <br /> (primario) | -1,8% | [-7.5%, -0.3%] | 254 |
| Mejoras ✅ <br /> (secundaria) | -1,3% | [-5,4%, -0,3%] | 224 |
| Todos ❌✅ (primarios) | -1,8% | [-7,5%, 0,8%] | 257 |

4 regresiones, 10 mejoras, 12 mixtas; 9 de ellos en rollups
90 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/457d83dc231ed684e9f09e96fdf41f45bed0fe67/triage/2024-12-23.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [Añadido soporte para 'use Trait::func'](https://github.com/rust-lang/rfcs/pull/3591)
* [crates.io: Trusted Publishing Support](https://github.com/rust-lang/rfcs/pull/3691)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Especificar el comportamiento de 'archivo!'](https://github.com/rust-lang/rust/pull/134442)
* [disposición: fusionar] [Estabilizar 'característica(trait_upcasting)'](https://github.com/rust-lang/rust/pull/134367)
* [disposición: fusionar] [Estabilizar 'derivar(CoercePointee)'](https://github.com/rust-lang/rust/pull/133820)
* [disposición: fusionar] [Estabilizar 'asm_goto puerta de características'](https://github.com/rust-lang/rust/pull/133870)
* [disposición: fusionar] [Problema de seguimiento para get_many_mut](https://github.com/rust-lang/rust/issues/104642)
* [disposition: merge] ['--nocapture' no sigue las convenciones comunes de CLI, lo que lo convierte en un obstáculo para las personas que depuran fallas](https://github.com/rust-lang/rust/issues/133073)
* [disposición: fusionar] [Problema de seguimiento para 'sub_ptr' (característica 'ptr_sub_ptr')](https://github.com/rust-lang/rust/issues/95892)
* [disposición: fusionar] [Del iterador para más tuplas](https://github.com/rust-lang/rust/pull/132431)
* [disposición: fusionar] [Problema de seguimiento para const_swap](https://github.com/rust-lang/rust/issues/83163)
* [disposición: fusionar] [Problema de seguimiento para const 'alloc::Layout'](https://github.com/rust-lang/rust/issues/67521)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Establecer CARGO_CHECK variable de entorno al verificar el tipo](https://github.com/rust-lang/rfcs/pull/3748)
* [nuevo] [parámetros sin tamaño en rasgos](https://github.com/rust-lang/rfcs/pull/3745)
* [nuevo] [Convertir "reasignación de local inmutable" y "préstamo mutable de local inmutable" de un error grave a un lint denegado por defecto](https://github.com/rust-lang/rfcs/pull/3742)

## Próximos eventos

Eventos oxidados entre 2024-12-25 - 2025-01-22 🦀

### Virtual
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
* 09/01/2025 | Miami, FL, EE. UU. | [Rust Miami](https://www.meetup.com/rust-miami/)
    * [**Rust / Wasm en Serverless y Frontend**](https://www.meetup.com/rust-miami/events/305122950)
* 09/01/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 14/01/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/302815031)
* 14/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Una introducción a WASM en Rust con Márk Tolmács (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/305064546)
* 15/01/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)
* 16/01/2025 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633278/)
* 21/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Explorando Rust Enums con Yoni Peleg (Virtual, Hebreo)**](https://www.meetup.com/rust-tlv/events/305110744)
* 21/01/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/rdhhptyhccbcc)
* 2025-01-22 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)

### Asia
* 2025-01-12 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust enero 2025 en Abra en Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europa
* 08/01/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 09/01/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)
* 16/01/2025 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/305144321)
* 21/01/2025 | Gante, BE | [Programación de Sistemas Gante](https://sysghent.be)
    * [**Tech Talks & Dinner: Insights on Systems Programming Side Projects (en Rust) - Leptos (Rust full-stack con webassembly), Karyon (software p2p distribuido en Rust), FunDSP (síntesis de audio en Rust)**](https://www.meetup.com/systems-programming-ghent/events/305201540/?slug=systems-programming-ghent&eventId=305201540)
* 21/01/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Redes Peer-to-Peer Auto-Organizadas usando Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303604074)

### América del Norte
* 26/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)
* 2025-01-10 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo de Rust de Lechmere, 10 de enero**](https://www.meetup.com/bostonrust/events/304951467)
* 16/01/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 1: Introducciones de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333017)
* 18/01/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo de Rust en Back Bay, 18 de enero**](https://www.meetup.com/bostonrust/events/304951470)
* 21/01/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638258)
* 2025-01-22 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhccbdc)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Solo es una transmutación si es de la región transmutada de std; de lo contrario, es solo una inseguridad brillante.

– [Josh Triplett en github](https://github.com/rust-lang/rust/pull/128351#issuecomment-2552304484)

¡Gracias a [Josh](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1645) por la autosugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1hn7y8z/this_week_in_rust_579/)</small>
