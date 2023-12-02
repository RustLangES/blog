---
title: "Esta semana en Rust #8"
number_of_week: 8
description: El crate de esta semana es tokio-graceful, una biblioteca para el apagado correcto de servidores asíncronos basados en tokio.
date: 2023-11-29
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

### Actualizaciones de proyectos/herramientas
* [Anunciando axum 0.7.0](https://tokio.rs/blog/2023-11-27-announcing-axum-0-7-0)
* [Anuncio de la disponibilidad general del SDK de AWS para Rust](https://aws.amazon.com/blogs/developer/announcing-general-availability-of-the-aws-sdk-for-rust/)
* [ripgrep 14.0.0](https://github.com/BurntSushi/ripgrep/releases/tag/14.0.0)
* [Multithreading mejorado en wgpu - La arcanización aterriza en el tronco](https://gfx-rs.github.io/2023/11/24/arcanization.html)
* [Presentamos SALT: ¡Un tutor de errores de Rust, buscando participantes en un estudio de errores!](https://marketplace.visualstudio.com/items?itemName=kale-lab.salt)
* [Bionic GPT - Reemplazo de Chat GPT integrado en Rust](https://github.com/bionic-gpt/bionic-gpt)
* [cargo-run-bin: ¿Por qué todo el mundo instala cajas en todo el mundo?](https://dustinblackman.com/posts/why-does-everyone-install-crates-globally/)
* [CXX-Qt: En el camino hacia la estabilidad, soporte de señales mejorado y más en la versión 0.6](https://www.kdab.com/cxx-qt-0-6/)
* [Oatmeal: Terminal UI para chatear con grandes modelos de lenguaje (LLM) utilizando diferentes backends de modelos, e integraciones con tus editores favoritos!](https://dustinblackman.com/posts/oatmeal/)

### Observaciones/Pensamientos
* [Objetivos del proyecto](https://smallcultfollowing.com/babysteps/blog/2023/11/28/project-goals/)
* [poll_next](https://without.boats/blog/poll-next/)
* [video] [Pero, ¿qué es 'toda una vida?'](https://www.youtube.com/watch?v=gRAVZv7V91Q)
* [audio] [Reclutamiento en Rust con Cedric Sellmann](https://rustacean-station.org/episode/cedric-sellmann/)

### Tutoriales de Rust
* [Diseño de un algoritmo SIMD desde cero](https://mcyoung.xyz/2023/11/27/simd-base64/)
* [Por qué las enumeraciones en Rust se sienten mucho mejor](https://www.shuttle.rs/blog/2023/11/23/enums-in-rust)
* [Cómo uso macros declarativas en Rust](https://flinect.com/blog/quick-tips-rust-declarative-macros)
* [Embajada en ESP: Primeros pasos](https://apollolabsblog.hashnode.dev/embassy-on-esp-getting-started)
* [Introducción a LLVM y MLIR con Rust y Melior](https://edgarluque.com/blog/mlir-with-rust/)
* [Investigando tiempos de compilación locos](https://blog.adamchalmers.com/crazy-compile-time/)
* [video] [Curso de Rust (Parte 2)](https://www.youtube.com/watch?v=Yj2aANykEgM) (Bahasa Indonesia).

### Investigación
* [Pruebas de refinamiento en Rust usando cerraduras fantasmas](https://arxiv.org/abs/2311.14452)
* [Fuzzing semántico del compilador e intérprete de Rust](https://ethz.ch/content/dam/ethz/special-interest/infk/inst-pls/plf-dam/documents/StudentProjects/MasterTheses/2023-Andy-Thesis.pdf)

### Miscelánea
* [Leer archivos YAML arbitrarios en Rust](https://rust.code-maven.com/read-arbitrary-yaml)
* [Crear una Lambda en Rust usando Terraform](https://maahl.net/blog/rust-aws-lambda/)

## Crate de la semana

El crate de esta semana es [tokio-graceful](https://docs.rs/tokio-graceful), una biblioteca para el apagado correcto de servidores asíncronos basados en tokio.

¡Gracias a [Glen De Cauwsemaecker](https://users.rust-lang.org/t/crate-of-the-week/2704/1266) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->
* [Hyperswitch - [REFACTOR]: [Nuvei] Validación de metadatos MCA](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [Características]: [Mediodía] Sincronización con Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [Características]: [Payme] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2906)
* [Hyperswitch - [BUG]: Los errores de deserialización de metadatos MCA deben ser 4xx](https://github.com/juspay/hyperswitch/issues/2899)
* [Hyperswitch - [REFACTOR]: Mensaje de error [Stripe] para la implementación del conector](https://github.com/juspay/hyperswitch/issues/2910)
* [Ockam - Biblioteca - Adelgazar el 'NodeManagerWorker' para 'nodo / estado del nodo'](https://github.com/build-trust/ockam/issues/6707)
* [Ockam - Comando - refactorizar para usar interfaces con tipo para implementar comandos para 'servicios de kafka'](https://github.com/build-trust/ockam/issues/6706)
* [Ockam - Biblioteca - Validar estructuras CBOR de acuerdo con el esquema cddl para 'nodos/models/transport' y 'nodes/models/workers'](https://github.com/build-trust/ockam/issues/6694)
* [r3bl-open-core - [tuify] API cambia el tipo de retorno de 'select_from_list()' 3](https://github.com/r3bl-org/r3bl-open-core/issues/200)
* [r3bl-open-core - Mejorar la plantilla de problema de "Informe de errores"](https://github.com/r3bl-org/r3bl-open-core/issues/248)
  
Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

405 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-20..2023-11-27

* [estabilizar la coerción de conversión de dyn](https://github.com/rust-lang/rust/pull/118133) (RFC [#3324](https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html))
* [añadir una puerta de características experimental para la delegación de funciones](https://github.com/rust-lang/rust/pull/117978)
* [habilitar Rust para usar la función de seguridad EHCont de Windows](https://github.com/rust-lang/rust/pull/118013)
* [Habilitar perfilador en dist-powerpc64-linux](https://github.com/rust-lang/rust/pull/118100)
* [habilitar la mitigación de erratas de Arm Cortex-A53 en aarch64-unknown-none](https://github.com/rust-lang/rust/pull/118095)
* [eliminar el objetivo ahora obsoleto 'x86_64-sun-solaris'](https://github.com/rust-lang/rust/pull/118091)
* ['EvalCtxt::commit_if_ok' no hereda objetivos anidados](https://github.com/rust-lang/rust/pull/118243)
* ['intercrate_ambiguity_causes': handle self ty infer + reservation impls](https://github.com/rust-lang/rust/pull/118089)
* [añadir 'Span' a 'TraitBoundModifier'](https://github.com/rust-lang/rust/pull/118245)
* [añadir 'debug_assert_nounwind' y convertir 'assert_unsafe_precondition'](https://github.com/rust-lang/rust/pull/110303)
* [agregar lint de permiso por defecto para los enlaces de unidades](https://github.com/rust-lang/rust/pull/112380)
* [permitir definir opacos en 'check_coroutine_obligations'](https://github.com/rust-lang/rust/pull/118161)
* [llamar a 'FileEncoder::finish' en la codificación rmeta](https://github.com/rust-lang/rust/pull/117301)
* [cobertura: simplificar las expresiones de cobertura de edificios basadas en sumas](https://github.com/rust-lang/rust/pull/117651)
* [No borrar las regiones enlazadas en tiempo de ejecución al seleccionar tipos asociados inherentes](https://github.com/rust-lang/rust/pull/118118)
* [no ice cuando se encuentre ambigüedad al seleccionar la implementación de 'Índice' en typeck](https://github.com/rust-lang/rust/pull/118112)
* [no ice ICE cuando encuentre marcadores de posición en el cálculo de límites implícitos](https://github.com/rust-lang/rust/pull/118290)
* [no consideres los argumentos genéricos de supertrait en 'deref_into_dyn_supertrait' lint](https://github.com/rust-lang/rust/pull/118026)
* [no requiere el modo de caja para la coherencia negativa](https://github.com/rust-lang/rust/pull/117992)
* [Cómputo ansiosamente 'output_filenames'](https://github.com/rust-lang/rust/pull/117584)
* [arreglar la vida útil de los parámetros tempranos en 'generic_const_exprs'](https://github.com/rust-lang/rust/pull/118035)
* [corrige que la visualización del recuento de errores es diferente cuando solo queda un error](https://github.com/rust-lang/rust/pull/118138)
* [Mejorar la ayuda de solo herramientas para múltiples variantes '#[predeterminado]']](https://github.com/rust-lang/rust/pull/118131)
* [hacer que PlaceholderReplacer 'shallow_resolver' y se repita cuando se infieran vars](https://github.com/rust-lang/rust/pull/118261)
* [Nota sobre los valores predeterminados de duración del objeto en el error no vive lo suficiente](https://github.com/rust-lang/rust/pull/117835)
* [Imprimir mapa de consulta para el interbloqueo cuando se usa el front-end paralelo](https://github.com/rust-lang/rust/pull/118169)
* [relacionar los tipos asociados inherentes usando la eq](https://github.com/rust-lang/rust/pull/118262)
* [eliminar '--check-cfg' comprobación de la línea de comandos '--cfg' args](https://github.com/rust-lang/rust/pull/117522)
* [eliminar 'HirId' de 'QPath::LangItem'](https://github.com/rust-lang/rust/pull/118199)
* [separar los flotantes 'NaN'/'Inf' con '_'](https://github.com/rust-lang/rust/pull/118271)
* [sugerir cambiar el orden de 'ref' y 'box'](https://github.com/rust-lang/rust/pull/118359)
* [el ITB no ajustado necesita pasar agregados por valor](https://github.com/rust-lang/rust/pull/118127)
* [Ajustar los atributos de enlace para los enlaces LLVM-wrapper](https://github.com/rust-lang/rust/pull/118142)
* [typeck break expr incluso si break es ilegal](https://github.com/rust-lang/rust/pull/118010)
* [use una ruta absoluta al dispositivo NUL](https://github.com/rust-lang/rust/pull/118060)
* [cuando no se puede importar 'core', sugiera 'std'](https://github.com/rust-lang/rust/pull/118065)
* [añadir 'VarDebugInfo' a la MIR estable](https://github.com/rust-lang/rust/pull/117972)
* [Agregar soporte para la asignación global en SMIR](https://github.com/rust-lang/rust/pull/118012)
* [arreglar la bonita impresión de 'Ty::Ref' de smir](https://github.com/rust-lang/rust/pull/118274)
* [expandir el GC BorTag de Miri a un GC de procedencia](https://github.com/rust-lang/rust/pull/118029)
* [validar que no hay bordes de llamada críticos en MIR optimizado](https://github.com/rust-lang/rust/pull/118075)
* [miri: GC el 'dead_alloc_map' también](https://github.com/rust-lang/rust/pull/118073)
* [miri: compruebe que las características de destino requeridas por los intrínsecos de LLVM están habilitadas](https://github.com/rust-lang/miri/pull/3180)
* [Miri: refactorizar 'float_to_int_checked' para eliminar su parámetro genérico y reducir un poco la duplicación de código](https://github.com/rust-lang/miri/pull/3185)
* [caché para 'ty::Const'](https://github.com/rust-lang/rust/pull/118189)
* [indica que la multiplicación en 'Layout::array' no puede desbordarse](https://github.com/rust-lang/rust/pull/118228)
* [reescribir la exhaustividad en una sola pasada](https://github.com/rust-lang/rust/pull/117611)
* ['AmbiguityCause' no debería formatear ansiosamente las cadenas](https://github.com/rust-lang/rust/pull/118267)
* [especialice 'SpecFromElem' para '()'](https://github.com/rust-lang/rust/pull/118094)
* [refactorizar 'binary_search_by' para usar movimientos condicionales](https://github.com/rust-lang/rust/pull/117722)
* [estabilizar 'ptr::addr_eq'](https://github.com/rust-lang/rust/pull/117968)
* [add 'BufRead::skip_until'](https://github.com/rust-lang/rust/pull/98943)
* [kmc-solid: seguridad de E/S](https://github.com/rust-lang/rust/pull/115159)
* [añadir 'Duración::abs_diff'](https://github.com/rust-lang/rust/pull/117619)
* [operaciones de conveniencia no nulas](https://github.com/rust-lang/rust/pull/117697)
* [hashbrown: Especializarse en la implementación de 'plegado' de iteradores](https://github.com/rust-lang/hashbrown/pull/480)
* [Elevador de grúa: implemente otro lote de intrínsecos del proveedor](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1431)
* [cargo: 'refactor(toml)': Mejores detalles de herencia abstracta](https://github.com/rust-lang/cargo/pull/13021)
* [cargo: salió con un error grave cuando el archivo de compilación personalizado no existe o no está en el paquete](https://github.com/rust-lang/cargo/pull/12995)
* [carga: añadir 'CARGO_RUSTC_CURRENT_DIR' (inestable)](https://github.com/rust-lang/cargo/pull/12996)
* [cargo: use custom error en lugar de anyhow](https://github.com/rust-lang/cargo/pull/13050)
* [Cargo: Revisar y eliminar las pruebas ignoradas en RustFix](https://github.com/rust-lang/cargo/pull/13047)
* [cargo: intente ejecutarlo en Windows](https://github.com/rust-lang/cargo/pull/13042)
* [rustdoc-search: añadir soporte para rasgos y tipos asociados](https://github.com/rust-lang/rust/pull/116085)
* [rustdoc-search: evitar la unboxing de la cláusula where infinita](https://github.com/rust-lang/rust/pull/118251)
* [rustdoc-search: simplifica 'checkPath' y 'sortResults'](https://github.com/rust-lang/rust/pull/118109)
* [rustdoc: ordenar los elementos inestables al final en lugar del primero](https://github.com/rust-lang/rust/pull/118224)
* [rustfix: añadir aviso de que rustfix se ha movido](https://github.com/rust-lang/rustfix/pull/225)
* [clippy: 'TypeckResults::node_type()' se puede usar dentro de los cuerpos](https://github.com/rust-lang/rust-clippy/pull/11877)
* [clippy: 'deprecated_semver': Permitir '#[deprecated(since = "TBD")]'](https://github.com/rust-lang/rust-clippy/pull/11850)
* [clippy: 'manual_try_fold': comprueba que 'fold' es realmente 'Iterator::fold'](https://github.com/rust-lang/rust-clippy/pull/11879)
* [clippy: 'missing_asserts_for_indexing': trabaja con cuerpos en lugar de bloques por separado](https://github.com/rust-lang/rust-clippy/pull/11859)
* [clippy: 'needless_return_with_question_mark': no peluar si nunca se usa el tipo para coerción](https://github.com/rust-lang/rust-clippy/pull/11627)
* [clippy: 'ptr_arg': reconoce métodos que también existen en los sectores](https://github.com/rust-lang/rust-clippy/pull/11817)
* [clippy: añadir nueva configuración 'check_private_items'](https://github.com/rust-lang/rust-clippy/pull/11842)
* [clippy: crear nueva pelusa 'option_map_or_err_ok'](https://github.com/rust-lang/rust-clippy/pull/11864)
* [clippy: no sugiera 'a.mul_add(b, c)' si los parámetros no son float](https://github.com/rust-lang/rust-clippy/pull/11836)
* [clippy: extiende 'result_map_or_into_option' lint para manejar 'Result::map_or_else(|_| Ninguno, algunos)'](https://github.com/rust-lang/rust-clippy/pull/11845)
* [clippy: se corrige el comportamiento de 'box_default' con 'vec! []' proveniente de macro arg](https://github.com/rust-lang/rust-clippy/pull/11875)
* [clippy: corrige la sugerencia de 'iter_kv_map' falsos positivos 'into_keys' y 'into_values'](https://github.com/rust-lang/rust-clippy/pull/11757)
* [clippy: mejorar el formato de los mensajes de error](https://github.com/rust-lang/rust-clippy/pull/11860)
* [clippy: eliminar la comprobación de guiones bajos para 'manual_non_exhaustive' lint](https://github.com/rust-lang/rust-clippy/pull/11844)
* [clippy: sugerir alternativas para iterar una matriz de rangos](https://github.com/rust-lang/rust-clippy/pull/11862)
* [clippy: usa la ruta absoluta para 'declare_tool_lint' en 'declare_clippy_lint'](https://github.com/rust-lang/rust-clippy/pull/11870)
* [rust-analyzer: inicialización cancelable](https://github.com/rust-lang/rust-analyzer/pull/15894)
* [Rust-analyzer: editor/código: añadir opción para suprimir las notificaciones de error internas](https://github.com/rust-lang/rust-analyzer/pull/15846)
* [rust-analyzer: asegúrese de que los cambios de nombre ocurran después de la edición](https://github.com/rust-lang/rust-analyzer/pull/15940)
* [Rust-analyzer: Arreglar la resolución de variantes para el alias de tipo](https://github.com/rust-lang/rust-analyzer/pull/15970)
* [rust-analyzer: corrección: agregar respaldo para los detalles de la etiqueta de finalización](https://github.com/rust-lang/rust-analyzer/pull/15962)
* [Rust-analyzer: corrección: resolver mejor el elemento de asociación con el tipo vinculado](https://github.com/rust-lang/rust-analyzer/pull/15825)
* [rust-analyzer: corrección: cajas duplicadas desduplicadas con diferentes orígenes en la construcción de CrateGraph](https://github.com/rust-lang/rust-analyzer/pull/15754)
* [Rust-analyzer: corrección: eliminar el paréntesis debe garantizar el espacio](https://github.com/rust-lang/rust-analyzer/pull/15857)
* [Rust-Analyzer: Mejorar la visualización de los detalles de la etiqueta de finalización](https://github.com/rust-lang/rust-analyzer/pull/15956)
* [rust-analyzer: reemplace 'option.map(cond) == Some(true)' por 'option.is_some_and(cond)'](https://github.com/rust-lang/rust-analyzer/pull/15960)

### Clasificación del rendimiento del compilador de Rust

Una buena semana, a pesar de algunos PR que pnkfelix optó por no marcar como clasificados. En
en particular, un amplio conjunto de parámetros de referencia primarios mejoró, debido a las mejoras en la
resolución (PR #118188) y una reescritura de una sola pasada de exhaustividad (PR #117611).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [4f3da903.. DF0295F0](https://perf.rust-lang.org/?start=4f3da903a43f22ea33d2ca4435a24b42fc1f842a&end=df0295f07175acc7325ce3ca4152eb05752af1f2&absolute=false&stat=instructions%3Au)

1 regresiones, 5 mejoras, 5 mixtas; 2 de ellos en rollups
84 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/20911b7f28a4b88d36ecd5b13414f26feac49d4d/triage/2023-11-28.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: fusionar] [Política de edición de especificadores de fragmentos de macros](https://github.com/rust-lang/rfcs/pull/3531)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [[rustdoc] Añadir resaltado para comentarios en la declaración de elementos](https://github.com/rust-lang/rust/pull/117869)
* [Disposición: Fusionar] [Generalizar: el identificador produce un error de comprobación en los alias](https://github.com/rust-lang/rust/pull/117088)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: constantes en patrones](https://github.com/rust-lang/rfcs/pull/3535)
* [Agregar RFC que combina los equipos de Infraestructura y Lanzamiento](https://github.com/rust-lang/rfcs/pull/3533)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-11-29 - 2023-12-27 🦀

### Virtual

* 29/11/2023 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [** ¡Capítulo final del Club de Lectura de Atomics & Locks! (Capítulo 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 30/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 30/11/2023 | Virtual (Dublín, IE) | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Automatización de la experiencia con comprobaciones de carga de carga**](https://www.meetup.com/rust-dublin/events/296346693/)
* 01/12/2023 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 02/12/2023 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 05/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Espejo**](https://berline.rs/)
* 05/12/2023 | Virtual (Búfalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)
* 06/12/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/297172140)
* 10/12/2023 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583652/)
* 12/12/2023 | Virtual | [Materia principal](https://mainmatter.com)
    * [**Taller: Telemetría para aplicaciones de Rust**](https://rust-telemetry-workshop.mainmatter.com)
* 12/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/297532862/)
* 2023-12-14| Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 14/12/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 17/12/2023 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**¡Que no cunda el pánico! - Nuestro viaje hacia el manejo de errores en Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 18/12/2023 | Virtual (Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Espejo**](https://berline.rs/)
* 20/12/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Aventuras en el desarrollo de aplicaciones egui**](https://www.meetup.com/vancouver-rust/events/292763506/)

### Asia

* 16/12/2023 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europa

* 30/11/2023 | Bruselas, BE | [Lambda Bruselas](https://lambda-brussels.glitch.me/)
    * [**Lambda Bruselas**](https://lambda-brussels.glitch.me/)
* 30/11/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #42 patrocinado por Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/297405705/)
* 30/11/2023 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - Noviembre**](https://www.meetup.com/rust-vienna/events/297382145/)
* 30/11/2023 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-zurich/events/297312190/)
* 06/12/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Encuentro de diciembre**](https://www.meetup.com/rustcologne/events/297100007/)
* 07/12/2023 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 07/12/2023 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 14/12/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 18/12/2023 | Múnich, DE + Virtual | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Heidelberg, DE | [Elimina tus insectos y oxida tus motores](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 19/12/2023 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, una alternativa al electrón**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### América del Norte

* 29/11/2023 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296657831/)
* 30/11/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628043/)
* 07/12/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/297533440/)
* 12/12/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 12/12/2023 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)  
    * [**Rust NYC Monthly Mixer: ¡Comparte, muestra y cuenta! 🦀 **](https://www.meetup.com/rust-nyc/events/297659937/)
* 19/12/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)

### Oceanía

* 28/11/2023 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/296391733/)
* 05/12/2023 | Aukland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Trucos asíncronos avanzados + software interrumpible**](https://www.meetup.com/rust-akl/events/297271684/)
* 11/12/2023 | Perth, WA, AU | [Grupo de Meetup de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Evento de fin de año de Rust**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/182f6dv/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Me gustaría informar que los tiempos de compilación de Rust estuvieron bien hoy, ayer y anteayer.
>
> los mantendré informados.

– [ZiCog sobre los lentos tiempos de compilación de Rust en los usuarios de Rust](https://users.rust-lang.org/t/is-rust-compile-time-really-that-slow/102863/15)

¡Gracias a [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1491) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18749of/this_week_in_rust_523/)</small>
