---
title: "Esta semana en Rust #24"
number_of_week: 24
description: El crate de esta semana es hydra, un marco de actores inspirado en Erlang/Elixir.
date: 2024-06-12
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

### Boletines informativos
* [ThisWeekInBevy: Bevy0.14-rc.2, Guante de poder y sopa](https://thisweekinbevy.com/issue/2024-06-10-bevy-014-rc2-powerglove-and-soup)
* [Este mes en Rust OSDev: mayo de 2024](https://rust-osdev.com/this-month/2024-05/)

### Actualizaciones de proyectos/herramientas
* [El compilador de Rust a .NET (backend) ahora puede compilar correctamente el "juego de adivinanzas" del libro de Rust](https://www.reddit.com/r/rust/comments/1db8vmg/media_the_rust_to_net_compiler_backend_can_now/)
* [Cattaca 1.0.0](https://github.com/sdball/cattaca/releases/tag/v1.0.0)
* [BugStalker v0.2.0 - depurador de Rust](https://www.reddit.com/r/rust/comments/1d6d3be/bugstalker_020_debugger_for_rust_programs/)

### Observaciones/Pensamientos
* [Cómo pasé 2 años construyendo mi propio motor de juego (Rust, WASM, WebGPU)](https://legendofworlds.com/blog/4)
* [Los tipos inconcebibles de Rust: cómo hacer que los autopréstamos sean seguros](https://blog.polybdenum.com/2024/06/07/the-inconceivable-types-of-rust-how-to-make-self-borrows-safe.html)
* [Hacer que los robots planifiquen más rápido con SIMD y Rust](https://www.claytonwramsey.com/blog/captree)
* [Aprendiendo Rust: Enhebrado desnudo](https://levelup.gitconnected.com/learning-rust-bare-threading-1defb65038c9)
* [999 cajas de Rust en la pared](https://lawngno.me/blog/2024/06/10/divine-provenance.html)
* [Cómo compilar Rust más rápido](https://blog.rust.careers/post/compile_rust_faster/)
* [Tamaño binario de Tock](https://tweedegolf.nl/en/blog/126/tock-binary-size)
* [Geometría virtual en Bevy 0.14](https://jms55.github.io/posts/2024-06-09-virtual-geometry-bevy-0-14/)
* [Construcción de datos antiguos desde cero](https://onevariable.com/blog/pods-from-scratch/)
* [Latencia en el borde con Rust/WebAssembly y Postgres: Parte 1](https://exograph.dev/blog/wasm-pg-explorations-1), [Parte 2](https://exograph.dev/blog/wasm-pg-explorations-2)
* [video] [Desarrollo full-stack de una infraestructura de pagos B2B con Rust - con Florent Bécart](https://www.youtube.com/watch?v=RA-r4F4ZmXM)

### Tutoriales de Rust
* [serie] [Master Hexagonal Architecture in Rust: Anatomy of a Bad Rust Application](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust)
* [Cómo construir un arnés de evaluación comparativa personalizado en Rust](https://bencher.dev/learn/benchmarking/rust/custom-harness/)
* [Del remitente al receptor: el enfoque de Rust para las transferencias de archivos locales](https://medium.com/@otukof/from-sender-to-receiver-rusts-approach-to-local-file-transfers-e6a778020d90)
* [Build with Naz - Manejo de errores de Rust con miette](https://developerlife.com/2024/06/10/rust-miette-error-handling/)

### Miscelánea
* [Informe de empleos de Rust de mayo de 2024](https://filtra.io/rust-may-24)
* [Eventos virtuales de Rust](https://events.code-maven.com/)

## Crate de la semana

El crate de esta semana es [hydra](https://github.com/dtzxporter/hydra), un marco de actores inspirado en Erlang/Elixir.

¡Gracias a [DTZxPorter](https://users.rust-lang.org/t/crate-of-the-week/2704/1313) por la autosugerencia!

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

*Esta semana no se han presentado convocatorias de participación.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí] [directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [Computación científica en Rust 2024](https://scientificcomputing.rs/) | Cierra 14/06/2024 | En línea | Fecha del evento: 2024-07-17 - 2024-07-19
* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Cierra el 06/07/2024 | Online + Ucrania, Kiev | Fecha del evento: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se presentaron 409 solicitudes de incorporación de cambios [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-04..2024-06-11

* [corrección: construir sobre haiku](https://github.com/rust-lang/rust/pull/126212)
* [bloques externos inseguros](https://github.com/rust-lang/rust/pull/124482)
* [permitir definiciones de mut estático con '#[linkage]'](https://github.com/rust-lang/rust/pull/125920)
* [los cierres son recursivamente accesibles](https://github.com/rust-lang/rust/pull/125996)
* ['rustc_codegen_ssa': corrige el pánico 'get_rpath_relative_to_output' cuando lib solo contiene el nombre del archivo](https://github.com/rust-lang/rust/pull/125861)
* [evitar errores de seguimiento si el número de parámetros genéricos ya no coincide](https://github.com/rust-lang/rust/pull/125608)
* [convertir 'proc_macro_back_compat' lint en un error incondicional](https://github.com/rust-lang/rust/pull/125596)
* [detectar estructuras de publicación nunca construidas y constantes asociadas no utilizadas](https://github.com/rust-lang/rust/pull/125572)
* [detectar cuando el usuario está intentando crear un 'Iterador' de préstamos y dar una explicación personalizada](https://github.com/rust-lang/rust/pull/125407)
* [añadir directamente la extensión en lugar de usar 'Path::with_extension'](https://github.com/rust-lang/rust/pull/125406)
* [no sueltes el candidato 'Destamaño' en el modo de intercadisco](https://github.com/rust-lang/rust/pull/125792)
* [no activar 'unsafe_op_in_unsafe_fn' para FNS seguros obsoletos](https://github.com/rust-lang/rust/pull/125925)
* [no camines por los cuerpos de las constantes libres para la accesibilidad](https://github.com/rust-lang/rust/pull/122505)
* [no advertir sobre los campos en la pelusa 'unreachable_pub'](https://github.com/rust-lang/rust/pull/126040)
* [enable GVN for 'AggregateKind::RawPtr'](https://github.com/rust-lang/rust/pull/125041)
* [arreglar ICE causado por ignorar EffectVars en la inferencia de tipos](https://github.com/rust-lang/rust/pull/125865)
* [arreglar ICE debido a 'desenvolver' en 'probe_for_name_many'](https://github.com/rust-lang/rust/pull/126115)
* [Mejorar la sugerencia de cambio de nombre para los nombres con guiones bajos iniciales](https://github.com/rust-lang/rust/pull/125795)
* [interpretar: no hacer ICE en vectores SIMD rellenados que no sean pow2](https://github.com/rust-lang/rust/pull/126184)
* [hacer cierres de descriptores de acceso TLS que devuelvan punteros](https://github.com/rust-lang/rust/pull/125525)
* [hacer que la eliminación en LinkedList sea consciente del asignador](https://github.com/rust-lang/rust/pull/125982)
* [hacer que html renderizado por rustdoc permita buscar identificador / alias que no esté en inglés](https://github.com/rust-lang/rust/pull/126057)
* [marque el enlace indeterminado si el nombre de destino existe y no se obtiene](https://github.com/rust-lang/rust/pull/126065)
* [Match ergonomics 2024: alinear la implementación con RFC](https://github.com/rust-lang/rust/pull/125168)
* [orphanck (solucionador antiguo): Considere los tipos opacos para nunca cubrir los parámetros de tipo](https://github.com/rust-lang/rust/pull/125871)
* [analizar atributos inseguros](https://github.com/rust-lang/rust/pull/124214)
* [aumentar 'DEFAULT_MIN_STACK_SIZE' a al menos 64 KiB](https://github.com/rust-lang/rust/pull/126059)
* [Resolver: Marcarlo como indeterminado si la importación única no tiene enlaces](https://github.com/rust-lang/rust/pull/124840)
* [scalarInt: las discrepancias de tamaño son un error, no retrases el pánico](https://github.com/rust-lang/rust/pull/126159)
* [establece 'has_unconstrained_ty_var' al generalizar alias en contextos bivariantes](https://github.com/rust-lang/rust/pull/126022)
* [silenciar los errores de seguimiento directamente en función de los tipos de error y las regiones](https://github.com/rust-lang/rust/pull/125667)
* [dividir smir 'Const' en 'TyConst' y 'MirConst'](https://github.com/rust-lang/rust/pull/125967)
* [almacenar los tipos de argumentos 'ty::Expr' en 'ty::Expr'](https://github.com/rust-lang/rust/pull/125968)
* [cuando 'deriva', tenga en cuenta HRTB en los campos 'BareFn'](https://github.com/rust-lang/rust/pull/125987)
* [aventar candidatos de método privado en lugar de asumir que se aplicará cualquier candidato con el nombre correcto](https://github.com/rust-lang/rust/pull/125622)
* [add 'SingleUseConsts' mir-opt pass](https://github.com/rust-lang/rust/pull/125910)
* [Miri: 'simd_bitmask': error más agradable cuando la máscara es demasiado grande](https://github.com/rust-lang/miri/pull/3659)
* [miri: 'simd_bitmask': funciona correctamente para tallas como 24](https://github.com/rust-lang/miri/pull/3662)
* [miri: 'simd_select_bitmask': corregir el nombre intrínseco en error](https://github.com/rust-lang/miri/pull/3660)
* [Miri: añadir soporte para 'pclmulqdq' intrínseco](https://github.com/rust-lang/miri/pull/3640)
* [Miri: Que no cunda el pánico si el cómputo del tiempo se desborda](https://github.com/rust-lang/miri/pull/3663)
* [miri: arreglar futex con gran tiempo de espera ICE](https://github.com/rust-lang/miri/pull/3653)
* [Miri: Arreglar la etapa en la contribución](https://github.com/rust-lang/miri/pull/3654)
* [estabilizar el orden de los MonoItems en las CGU y no permitir 'query_instability' lint por 'rustc_monomorphize'](https://github.com/rust-lang/rust/pull/125928)
* [estabilizar 'Opción::take_if'](https://github.com/rust-lang/rust/pull/126089)
* [estabilizar 'binary_heap_as_slice'](https://github.com/rust-lang/rust/pull/124012)
* [estabilizar 'error_in_core'](https://github.com/rust-lang/rust/pull/125951)
* [Permitir 'core_intrinsics' cuando está activado](https://github.com/rust-lang/rust/pull/126096) (RFC [#2011](https://rust-lang.github.io/rfcs/2011-generic-assert.html))
* [añadir función 'core::iter::chain'](https://github.com/rust-lang/rust/pull/106186)
* ['offset_of': permite (de forma inestable) tomar el desplazamiento de los campos de cola de corte](https://github.com/rust-lang/rust/pull/126150)
* [añadir constante 'FRAC_1_SQRT_2PI' a F16/F32/F64/F128](https://github.com/rust-lang/rust/pull/125253)
* [añadir 'size_of' y 'size_of_val' y 'align_of' y 'align_of_val' al preludio](https://github.com/rust-lang/rust/pull/123168)
* [hashbrown: feat: borsh serde](https://github.com/rust-lang/hashbrown/pull/525)
* [SIMD portátil: implementa swizzles especiales para máscaras y elimina {'to', 'from'}'_bitmask_vector'](https://github.com/rust-lang/portable-simd/pull/423)
* [regex: escape de bytes UTF-8 no válidos en la salida de depuración para 'Match'](https://github.com/rust-lang/regex/pull/1203)
* [pelusas de carga: Añadir 'unknown_lints' a la lista de pelusas](https://github.com/rust-lang/cargo/pull/14024)
* [cargo toml: Convierte en errores las advertencias de que los archivos 'license' y 'readme' no existen](https://github.com/rust-lang/cargo/pull/13921)
* [cargo toml: eliminar el soporte de la clave 'lib.plugin' y convertirlo en advertencia](https://github.com/rust-lang/cargo/pull/13902)
* [cargo: el ejemplo proc-macro de DEP ya no afecta a la resolución de características](https://github.com/rust-lang/cargo/pull/13892)
* [cargo: eliminar '__CARGO_GITOXIDE_DISABLE_LIST_FILES' env var](https://github.com/rust-lang/cargo/pull/14036)
* [cargo: el uso de '--release/debug' y '--profile' juntos se convierte en un error](https://github.com/rust-lang/cargo/pull/13971)
* [cargo: rename --out-dir a --artifact-dir](https://github.com/rust-lang/cargo/pull/13809)
* [rustdoc-search: use un nombre en minúsculas, no normalizado para la búsqueda de tipos](https://github.com/rust-lang/rust/pull/126176)
* [rustdoc: añadir soporte para --remap-path-prefix](https://github.com/rust-lang/rust/pull/107099)
* [rustdoc: incluir comas finales en las declaraciones de funciones envueltas](https://github.com/rust-lang/rust/pull/125946)
* [clippy: 'lint_groups_priority': ignora las pelusas y los grupos en el mismo nivel](https://github.com/rust-lang/rust-clippy/pull/12827)
* [clippy: 'match_same_arms': añadir un caso de prueba con tiempos de vida](https://github.com/rust-lang/rust-clippy/pull/12901)
* [clippy: 'overly_complex_bool_expr': Corregir falso positivo en el tipo nunca](https://github.com/rust-lang/rust-clippy/pull/12700)
* [clippy: añadir pelusa 'needless_maybe_sized'](https://github.com/rust-lang/rust-clippy/pull/10632)
* [clippy: añadir paréntesis obligatorios alrededor del receptor del método](https://github.com/rust-lang/rust-clippy/pull/12851)
* [clippy: desduplicado 'nonminimal_bool_methods' diagnostica](https://github.com/rust-lang/rust-clippy/pull/12845)
* [clippy: no peluques los bloques en los cierres para 'blocks_in_conditions'](https://github.com/rust-lang/rust-clippy/pull/12805)
* [clippy: arreglar 'to_string_in_format_args' con el receptor de llamadas macro](https://github.com/rust-lang/rust-clippy/pull/12844)
* [clippy: corregir falso positivo para 'needless_character_iteration' lint](https://github.com/rust-lang/rust-clippy/pull/12886)
* [clippy: maneja correctamente los efectos const heredados del padre en 'type_certainty'](https://github.com/rust-lang/rust-clippy/pull/12877)
* [clippy: lint 'manual_unwrap_or_default' para 'Resultado' también](https://github.com/rust-lang/rust-clippy/pull/12897)
* [clippy: hacer cierres de visitas 'for_each_expr' por defecto, cambiar el nombre de la versión anterior a 'for_each_expr_without_closures'](https://github.com/rust-lang/rust-clippy/pull/12822)
* [clippy: solo ejecuta 'suboptimal_flops' en llamadas a métodos inherentes](https://github.com/rust-lang/rust-clippy/pull/12884)
* [Rust-Analyzer: Agrega un modificador de preferencias para cajas locales del espacio de trabajo cuando se usa la importación automática](https://github.com/rust-lang/rust-analyzer/pull/17308)
* [Rust-analyzer: Agregar información de versión al elemento de la barra de estado](https://github.com/rust-lang/rust-analyzer/pull/17359)
* [rust-analyzer: se cambió 'package.json' para que la configuración de la extensión vscode tenga submenús](https://github.com/rust-lang/rust-analyzer/pull/17346)
* [rust-analyzer: configuración basada en TOML para rust-analyzer](https://github.com/rust-lang/rust-analyzer/pull/17058)
* [Rust-Analyzer: Calcular diagnósticos nativos en paralelo](https://github.com/rust-lang/rust-analyzer/pull/17372)
* [Rust-analyzer: Ocultar símbolos de doble subrayado de la búsqueda de símbolos](https://github.com/rust-lang/rust-analyzer/pull/17282)
* [Rust-analyzer: No resuelve el preludio dentro de los módulos de bloques](https://github.com/rust-lang/rust-analyzer/pull/17352)
* [rust-analyzer: asegúrese de que el padre de un 'SourceRoot' no puede ser él mismo](https://github.com/rust-lang/rust-analyzer/pull/17381)
* [rust-analyzer: arreglar los marcadores generados que no se pueden parchear en 'package.json'](https://github.com/rust-lang/rust-analyzer/pull/17368)
* [Rust-analyzer: Se corrige el cambio de nombre de las importaciones de artículos extranjeros que tocan fuentes extranjeras](https://github.com/rust-lang/rust-analyzer/pull/17360)
* [Rust-analyzer: resalte los archivos desvinculados de forma coherente con los archivos inactivos](https://github.com/rust-lang/rust-analyzer/pull/17350)
* [Rust-analyzer: formato incorrecto de las acciones de desplazamiento](https://github.com/rust-lang/rust-analyzer/pull/17353)
* [rust-analyzer: eliminar la caché de análisis adicional de Semantics nuevamente](https://github.com/rust-lang/rust-analyzer/pull/17380)
* [rust-analyzer: intente almacenar en caché las llamadas de macros de forma más agresiva en Semantics](https://github.com/rust-lang/rust-analyzer/pull/17004)

### Clasificación del rendimiento del compilador de Rust

Esta semana se han producido más regresiones que victorias, causadas principalmente por la reorganización del código dentro de la
compilador y una nueva característica que se está implementando. También se han producido algunas mejoras agradables
optimizando mejor los intervalos.

Triaje realizado por **@kobzol**.
Rango de revisión: [1d52972d.. b5b13568](https://perf.rust-lang.org/?start=1d52972dd8592edf4026aa577c8ce69acc0ac2d1&end=b5b13568fb5da4ac988bde370008d6134d3dfe6c&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.6% | [0,2%, 2,7%] | 105 |
| Regresiones ❌ <br /> (secundaria) | 1.0% | [0,1%, 6,9%] | 74 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-1,0%, -0,2%] | 20 |
| Mejoras ✅ <br /> (secundaria) | -1,4% | [-8,8%, -0,2%] | 32 |
| Todos ❌✅ (primario) | 0.5% | [-1,0%, 2,7%] | 125 |

5 regresiones, 3 mejoras, 4 mixtas; 5 de ellos en rollups
59 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/aba4c3895edeee39e7454f600a85c9dd3f8867cf/triage/2024-06-11.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [reorganizar el equipo compilador](https://github.com/rust-lang/rfcs/pull/3599)
* [Captura precisa](https://github.com/rust-lang/rfcs/pull/3617)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [UnsafePinned: permitir alias de referencias mutables ancladas](https://github.com/rust-lang/rfcs/pull/3467)
* [disposición: posponer] [RFC: hacer que Cargo incruste versiones de dependencia en el binario compilado](https://github.com/rust-lang/rfcs/pull/2801)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: cerrar] [¿Deberíamos permitir StorageLive en una configuración local activa?](https://github.com/rust-lang/rust/issues/99160)
* [disposición: fusionar] [Problema de seguimiento para 'hint::assert_unchecked'](https://github.com/rust-lang/rust/issues/119131)
* [disposición: fusionar] [Recopilar los límites de elementos relevantes de las cláusulas de rasgos para proyecciones rígidas anidadas](https://github.com/rust-lang/rust/pull/120752)
* [disposición: cerrar] [impl conflictivo desde Nightly-2024-05-01](https://github.com/rust-lang/rust/issues/125763)
* [disposición: fusionar] [Comportamiento del documento de create_dir_all ruta vacía escrita](https://github.com/rust-lang/rust/pull/125112)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Incluir vcs_info incluso si el espacio de trabajo está sucio](https://github.com/rust-lang/cargo/pull/13960)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna RFC de equipo lingüístico entró en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

##### [Directrices sobre códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [actualizado] [corregir enlaces de RFC de seguridad de E/S](https://github.com/rust-lang/rfcs/pull/3655)
* [nuevo] [RFC: Notación de tipo de retorno](https://github.com/rust-lang/rfcs/pull/3654)

## Próximos eventos

Eventos oxidados entre 2024-06-12 - 2024-07-10 🦀

### Virtual

* 12/06/2024 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 8 - Programación asíncrona**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 13/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 13/06/2024 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 16/06/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Taller: Desarrollo web en Rust usando Rocket (Inglés)**](https://www.meetup.com/code-mavens/events/301294669/)
* 18/06/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346963/)
* 19/06/2024 | Híbrido - Virtual y Presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 20/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477705/)
* 25/06/2024 | Virtual (Dallas, TX, EE. UU.)| [Grupo de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 25/06/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Usando el sistema de plantillas Liquid en Rust (inglés)**](https://www.meetup.com/code-mavens/events/301487547/)
* 27/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 02/07/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 03/07/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Crear aplicaciones web con Rust y Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 03/07/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/300328025/)
* 04/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488820/)
* 06/07/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 09/07/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/299346976/)
* 10/07/2024 | Virtual | [Centro de Investigación Electrónica](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Investigación informática con el lenguaje de programación Rust**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)

### Asia
* 22/06/2024 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de junio de 2024**](https://hasgeek.com/rustbangalore/june-2024-rustacean-meetup/)
* 30/06/2024 | Kioto, JP | [Rust de Kioto](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Aplicaciones multiplataforma**](https://www.meetup.com/kyoto-rust/events/301499550/)
    
### Europa

* 12/06/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Leyendo Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/301012491/)
* 18/06/2024 | Fráncfort del Meno, DE | [Reunión de Rust Frankfurt](https://www.meetup.com/rust-frankfurt)
    * [**¡Rust Frankfurt ha vuelto!**](https://www.meetup.com/rust-frankfurt/events/301441434/)
* 19/06/2024 - 24/06/2024 | Zúrich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 20/06/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 25/06/2024 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 27/06/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288965/)
* 27/06/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 patrocinado por Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)

### América del Norte

* 12/06/2024 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Detroit Rust Meet - Ann Arbor**](https://www.meetup.com/detroitrust/events/301387848/)
* 13/06/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/)
    * [**Reunión mensual: Creando un intérprete en Rust, parte 1**](https://www.meetup.com/spokane-rust/events/300020010/)
* 14/06/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/)
    * [**¡Barbacoa de verano para los grupos de usuarios de tecnología local de Spokane en la azotea del pub Saranac!**](https://www.meetup.com/spokane-rust/events/301569401/)
* 17/06/2024 | Minneapolis, MN Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301411625/)
* 18/06/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 19/06/2024 | Híbrido - Vancouver, Columbia Británica, CA | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 20/06/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 24/06/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Harvard Square Rust, 24 de junio**](https://www.meetup.com/bostonrust/events/301549722/)
* 26/06/2024 | Austin, TX, EE. UU. | [ATC de Rust](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 27/06/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 05/07/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de la Universidad de Boston, 5 de julio**](https://www.meetup.com/bostonrust/events/301549737/)

### Oceanía

* 14/06/2024 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**Reunión de junio de 2024 de Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/301311680/)
* 20/06/2024 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Escribir un compilador por diversión y (no) beneficio**](https://www.meetup.com/rust-akl/events/301193761/)
* 25/06/2024 | Canberra, ACt, AU | [Grupo de usuarios de Canberra Rust (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/300749371/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> me gusta lo explícito pero odio el ruido...

– [dlevac discutiendo el manejo de errores en /r/golang](https://www.reddit.com/r/golang/comments/1d7tswh/comment/l71of8o/)

¡Gracias a [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1571) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1deqkgg/this_week_in_rust_551/)</small>
