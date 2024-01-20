---
title: "Esta semana en Rust #13"
number_of_week: 13
description: El crate de esta semana es named-sem.
date: 2024-01-10
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---

¡Hola y bienvenido a otra edición de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que capacita a todos para construir software confiable y eficiente.
Esto es un resumen semanal de su progreso y comunidad.
¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla de manera abierta [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentras algún error en la edición de esta semana, [por favor envía una PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!--

Estimados contribuyentes de la comunidad:
Por favor, lean README.md para obtener orientación sobre las contribuciones.
Cada enlace enviado debe tener la forma:

* [Título de la Página Enlazada](https://ejemplo.com/mi_articulo)

Si no sabes qué categoría usar, siéntete libre de enviar una PR de todos modos
y simplemente pide a los editores que seleccionen la categoría.

-->

### Oficial
* [Este Ciclo de Desarrollo en Cargo: 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html)

### Fundación
* [Anuncio de Nuevo Miembro de la Fundación Rust: xFusion, Lynx y SpruceID](https://foundation.rust-lang.org/news/rust-foundation-new-member-announcement-xfusion-lynx-spruceid/)

### Boletines
* [Este Mes en Rust OSDev: Diciembre de 2023](https://rust-osdev.com/this-month/2023-12/)

### Actualizaciones de Proyectos/Herramientas
* [Maestro - Introducción](https://blog.lenot.re/a/introduction)
* [Polars](https://pola.rs/posts/polars_in_aggregrate-0.20/)
* [Registro de cambios de rust-analyzer #215](https://rust-analyzer.github.io/thisweek/2024/01/08/changelog-215.html)
* [argmin 0.9.0 - Una crate Rust para optimización numérica](https://argmin-rs.org/blog/version-v0-9-0/)
* [Benchmarks continuos para rustls](https://ochagavia.nl/blog/continuous-benchmarking-for-rustls/)
* [¡embedded-hal v1.0 ya lanzado!](https://blog.rust-embedded.org/embedded-hal-v1/)

### Observaciones/Pensamientos
* [Arrays: ¿Error de índice fuera de límites? ¡No siempre!](https://www.greyblake.com/blog/index-out-of-bounds-not-always-a-rust-surprise/)
* [Lo que me gustaría ver para Async Rust en 2024](https://smallcultfollowing.com/babysteps/blog/2024/01/03/async-rust-2024/)
* [Asegurando la Web: Rustls en camino de superar a OpenSSL](https://www.memorysafety.org/blog/rustls-performance/)
* [Construcción al estilo Inception con dependencias privadas de GitHub](https://heikoseeberger.de/2024-01-06-inception-style-build/)
* [Verificando Rust Zeroize con Assembly...incluyendo SIMD portátil](https://cipherstash.com/blog/verifying-rust-zeroize-with-assembly-including-portable-simd)
* [¿Por qué stdout es más rápido que stderr?](https://blog.orhun.dev/stdout-vs-stderr/)
* [audio] [Programación de Audio en Rust con Ian Hobson](https://thewolfsound.com/talk016/)
* [audio] [Polars con Ritchie Vink](https://rustacean-station.org/episode/ritchie-vink/)

### Tutoriales Rust
* [Comenzando con Tracing en Rust](https://www.shuttle.rs/blog/2024/01/09/getting-started-tracing-rust)
* [Haciendo Matemáticas de Primer Grado en el Sistema de Tipos de Rust](https://fprasx.github.io/articles/type-system-arithmetic/)
* [Creemos una pantalla de información en Rust](https://blog.stillinbeta.com/2024-01-01-overengineered-household-display.html)

### Miscelánea
* [3 formas de manejar el desbordamiento de números en Rust](https://rust.code-maven.com/how-to-handle-overflow)
* [Rocket - multicuenta usando cookies](https://rust.code-maven.com/rocket-multi-counter-using-cookies)
* [unwrap, una forma de manejar errores en Rust](https://rust.code-maven.com/unwrap)
* [video] [Top 10 Juegos de Bevy Jam 4](https://www.youtube.com/watch?v=FVhOkpIytJc)
* [video] [¡Necesitas construir una aplicación de escritorio RUST!!](https://m.youtube.com/watch?v=7aFgeUG9TK4)

## Crate de la semana

El crate de esta semana es [named-sem](https://crates.io/crates/named-sem), una biblioteca de envoltura para semáforos con nombre en Linux y Windows.

¡Gracias a [EvianZhang](https://users.rust-lang.org/t/crate-of-the-week/2704/1277) por la auto-sugerencia!

[Por favor, envía tus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la Participación; proyectos y oradores

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad Rust para que elijas y te pongas en marcha!

Algunas de estas tareas también pueden contar con mentores, visita la página de la tarea para obtener más información.

* [Ockam - refactorizar para usar interfaces tipadas para implementar comandos para `kafka services`](https://github.com/build-trust/ockam/issues/6706)
---
* [Ockam - refactor to use typed interfaces to implement commands for `workers`](https://github.com/build-trust/ockam/issues/6702)
* [Ockam - Validate CBOR structs according to the cddl schema for `nodes/models/transport` and `nodes/models/workers`](https://github.com/build-trust/ockam/issues/6694)
* [Hyperswitch - Make cache configuration configurable at runtime](https://github.com/juspay/hyperswitch/issues/3276)
* [Hyperswitch - Implement Code cov for local system using makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - Setup code coverage for local tests & CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - Add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - Have get_required_value to use ValidationError in OptionExt](https://github.com/juspay/hyperswitch/issues/860)

Si eres propietario de un proyecto Rust y estás buscando colaboradores, por favor presenta tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un ponente nuevo o experimentado buscando un lugar para compartir algo interesante? Esta sección destaca eventos que se están planeando y que aceptan presentaciones para unirse como ponente.

* *No se enviaron convocatorias esta semana.*

Si eres un organizador de eventos que desea ampliar el alcance de su evento, por favor presenta un enlace al sitio de presentación a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

Se fusionaron 446 solicitudes de extracción [la semana pasada][fusionadas].

[fusionadas]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-02..2024-01-09

* [promover objetivos `riscv32{im|imafc}` a nivel 2](https://github.com/rust-lang/rust/pull/118704)
* [agregar objetivo de nivel 3 `riscv32imafc-esp-espidf` para ESP32-P4](https://github.com/rust-lang/rust/pull/119738)
* [soportar clase de registro `reg_addr` en ensamblaje en línea s390x](https://github.com/rust-lang/rust/pull/119431)
* [agregar -Zuse-sync-unwind](https://github.com/rust-lang/rust/pull/117744)
* [`macro_rules`: Agregar una caché local de expansión al marcador de spans](https://github.com/rust-lang/rust/pull/119693)
* [`macro_rules`: Heurística menos engañosa para usar los spans de la metavariable `tt`](https://github.com/rust-lang/rust/pull/119204)
* [`rustc_mir_transform`: Aplicar lint `rustc::potential_query_instability`](https://github.com/rust-lang/rust/pull/119252)
* [`rustc_mir_transform`: Hacer estable `DestinationPropagation` para consultas](https://github.com/rust-lang/rust/pull/119591)
* [`rustc_span`: Operaciones de combinación de spans más consistentes](https://github.com/rust-lang/rust/pull/119624)
* [`rustc_span`: Optimizar comparaciones de contextos de sintaxis](https://github.com/rust-lang/rust/pull/119531)
* [permitir que las pruebas de cobertura ignoren los modos de prueba y habiliten el color en los informes de cobertura](https://github.com/rust-lang/rust/pull/119034)
* [evitar la especialización en el código de serialización de metadatos](https://github.com/rust-lang/rust/pull/119478)
* [verificar el tipo de reanudación del terminador de yield en borrowck](https://github.com/rust-lang/rust/pull/119563)
* [cobertura: `llvm-cov` espera que los números de columna sean bytes, no puntos de código](https://github.com/rust-lang/rust/pull/119033)
* [cobertura: anonimizar números de línea en vistas de ramas](https://github.com/rust-lang/rust/pull/119681)
* [cobertura: evitar un riesgo de estabilidad de consulta en `function_coverage_map`](https://github.com/rust-lang/rust/pull/119514)
* [cobertura: elevar parte del código complejo fuera del bucle principal de refinamiento de spans](https://github.com/rust-lang/rust/pull/119208)
* [denegar valores predeterminados para parámetros genéricos de rango superior](https://github.com/rust-lang/rust/pull/119494)
* [no sintetizar argumentos de efecto host dentro de tipos de objetos de trait](https://github.com/rust-lang/rust/pull/119540)
* [no sintetizar parámetros de efecto host para funciones asociadas de trait marcadas como const](https://github.com/rust-lang/rust/pull/119505)
* [habilitar el sanitizer de direcciones para objetivos MSVC usando la bandera del enlazador INFERASANLIBS](https://github.com/rust-lang/rust/pull/118521)
* [exhaustividad: hacer cumplir estáticamente la revelación de opacos](https://github.com/rust-lang/rust/pull/119329)
* [corregir el alcance para cadenas let en guardias de coincidencia](https://github.com/rust-lang/rust/pull/119554)
* [manejar ForeignItem como alcance TAIT](https://github.com/rust-lang/rust/pull/119420)
* [ocultar rutas externas `#[doc(hidden)]` en sugerencias de importación](https://github.com/rust-lang/rust/pull/119151)
* [ocultar rutas extranjeras `#[doc(hidden)]` en sugerencias de importación](https://github.com/rust-lang/rust/pull/119151)
* [ajustes en el diagnóstico de `impl trait`](https://github.com/rust-lang/rust/pull/119703)
* [implicar límites de supervivencia en alias de tipo diferido](https://github.com/rust-lang/rust/pull/119350)
* [mejorar el soporte del atributo `collapse_debuginfo` para macros](https://github.com/rust-lang/rust/pull/118903)
* [incorporar algunas funciones de utilidad alrededor de MIR](https://github.com/rust-lang/rust/pull/119459)
* [llvm: permitir `noundef` en pruebas de generación de código](https://github.com/rust-lang/rust/pull/119523)
* [hacer la sugerencia de `derive(Trait)` más precisa](https://github.com/rust-lang/rust/pull/119362)
* [hacer que la lint `named_asm_labels` no se active en Unicode y se active en los argumentos de formato](https://github.com/rust-lang/rust/pull/119195)
* [hacer siempre ambiguos los ciclos inductivos en coherencia](https://github.com/rust-lang/rust/pull/118649)
* [marcarme como de vuelta de la licencia](https://github.com/rust-lang/rust/pull/119512)
* [migrar la verificación de superposición de memoria de validador a lint](https://github.com/rust-lang/rust/pull/119577)
* [llenar los tipos `yield` y `resume` en el cuerpo de MIR mientras se inicializa el cuerpo](https://github.com/rust-lang/rust/pull/119666)
* [imprimir correctamente los predicados de trait siempre-const](https://github.com/rust-lang/rust/pull/119476)
* [consultar `panic!()` para obtener un diagnóstico útil](https://github.com/rust-lang/rust/pull/119086)
* [recuperar paréntesis en patrones de rango](https://github.com/rust-lang/rust/pull/119397)
* [restablecer la configuración `optimized-compiler-builtins`](https://github.com/rust-lang/rust/pull/119556)
* [reordenar los diagnósticos de `check_item_type` para que ocurran junto a los diagnósticos correspondientes de `check_well_formed`](https://github.com/rust-lang/rust/pull/117213)
* [reemplazar varios FxHashMaps/Sets con alternativas de orden de iteración estable](https://github.com/rust-lang/rust/pull/119192)
* [separar la representación de ScalarPair inmediata y en memoria](https://github.com/rust-lang/rust/pull/118991)
* [establecer la característica `in-rust-tree` para todos los pasos de rust-analyzer{-proc-macro-srv}](https://github.com/rust-lang/rust/pull/118861)
* [omitir el enroscado sobre SetDiscriminant sin operación](https://github.com/rust-lang/rust/pull/119675)
* [estabilizar THIR unsafeck](https://github.com/rust-lang/rust/pull/117673)
* [dejar de alimentar vis cuando no se puede acceder a un elemento de trait](https://github.com/rust-lang/rust/pull/119553)
* [soportar `~const` en funciones asociadas en implementaciones de trait](https://github.com/rust-lang/rust/pull/119705)
* [suprimir las advertencias del change-tracker en los contenedores de CI](https://github.com/rust-lang/rust/pull/119298)
* [cambiar de usar anotaciones `//~ERROR` con `--error-format` a `error-pattern`](https://github.com/rust-lang/rust/pull/119184)
* [desactivar temporalmente los ejecutores M1 en las acciones de GitHub](https://github.com/rust-lang/rust/pull/119546)
* [ajustar las sugerencias para un trait desnudo utilizado como tipo](https://github.com/rust-lang/rust/pull/119148)
* [usar `resolutions(()).effective_visibilities` para evitar errores cíclicos en `report_object_error`](https://github.com/rust-lang/rust/pull/119506)
* [mir personalizado: dejar claro cuál es el bloque de retorno](https://github.com/rust-lang/rust/pull/119325)
* [miri: implementar las instrucciones de redondeo utilizando el redondeo apfloat](https://github.com/rust-lang/miri/pull/3256)
* [miri: usar jemalloc como asignador global](https://github.com/rust-lang/miri/pull/3259)
* [miri: solo usar jemalloc en Linux y macOS](https://github.com/rust-lang/miri/pull/3261)
* [eliminar binarios lld-wrapper](https://github.com/rust-lang/rust/pull/119661)
* [dos pequeñas optimizaciones de bitset](https://github.com/rust-lang/rust/pull/119499)
* [codegen-cranelift: reestructurar las instrucciones de empaquetado con signo x86](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1443)
* [hacer constante estable `intrinsics::assume`](https://github.com/rust-lang/rust/pull/119583)
* [reescribir la implementación predeterminada de `Iterator::position`](https://github.com/rust-lang/rust/pull/119599)
* [hacer que el análisis de campos de `offset_of` use una metavariante que maneje cualquier espaciado](https://github.com/rust-lang/rust/pull/119532)
* [marcar los punteros de `vec::IntoIter` como `!nonnull`](https://github.com/rust-lang/rust/pull/114205)
* [cargo fix: Llamar a rustc menos veces](https://github.com/rust-lang/cargo/pull/13243)
* [cargo fix: establecer `OUT_DIR` para todas las unidades con scripts de construcción](https://github.com/rust-lang/cargo/pull/13204)
* [cargo cli: agregar colores a la salida de consola de `-Zhelp`](https://github.com/rust-lang/cargo/pull/13269)
* [cargo embedded: agregar varias sintaxis experimentales de manifiesto](https://github.com/rust-lang/cargo/pull/13241)
* [cargo embedded: agregar soporte para sintaxis de frontmatter con prefijo de caracteres](https://github.com/rust-lang/cargo
---
* [rust-analyzer: `extract_struct_from_enum_variant` assist should resolve Self generic arg](https://github.com/rust-lang/rust-analyzer/pull/16199)
* [rust-analyzer: assists panic when trying to edit usage inside macro](https://github.com/rust-lang/rust-analyzer/pull/15810)
* [rust-analyzer: correct references from `rust-analyzer.cargo.check` to `rust-analyzer.check`](https://github.com/rust-lang/rust-analyzer/pull/16062)
* [rust-analyzer: fix focus range being discarded in attributes/derives when upmapping](https://github.com/rust-lang/rust-analyzer/pull/16234)
* [rust-analyzer: fix panic on unaligned packed attribute](https://github.com/rust-lang/rust-analyzer/pull/16285)
* [rust-analyzer: fix type inference with `IndexMut` returning references](https://github.com/rust-lang/rust-analyzer/pull/16085)
* [rust-analyzer: give a userful error when rustc cannot be found in explicit sysroot](https://github.com/rust-lang/rust-analyzer/pull/16241)
* [rust-analyzer: make callable fields not complete in method access no parens case](https://github.com/rust-lang/rust-analyzer/pull/16049)
* [rust-analyzer: make functions in impl have a container name](https://github.com/rust-lang/rust-analyzer/pull/16139)
* [rust-analyzer: no code action `'introduce_named_generic'` for impl inside types](https://github.com/rust-lang/rust-analyzer/pull/16067)
* [rust-analyzer: notify user that linkedProjects is set when failing to discover projects](https://github.com/rust-lang/rust-analyzer/pull/16153)
* [rust-analyzer: pick up new names when the name conflicts in `'introduce_named_generic'`](https://github.com/rust-lang/rust-analyzer/pull/16068)
* [rust-analyzer: remove completion limit for trait importing method completions](https://github.com/rust-lang/rust-analyzer/pull/16268)
* [rust-analyzer: rewrite `code_action generate_delegate_trait`](https://github.com/rust-lang/rust-analyzer/pull/16112)
* [rust-analyzer: self type replacement in inline-function](https://github.com/rust-lang/rust-analyzer/pull/16114)

### Rust Compiler Performance Triage

Not a particularly notable week. Large swings aren't spurious but also are
driven by changes in high-level behavior (diagnostics going from zero to one
emission primarily), which causes a lot more work to happen. This isn't really
representative of the underlying rustc performance changing though.

Triage done by **@simulacrum**.
Revision range: [67b6975..76101ee](https://perf.rust-lang.org/?start=67b6975051b83ef2bd28f06e8467470d570aceb3&end=76101eecbe9aa80753664bbe637ad06d1925f315&absolute=false&stat=instructions%3Au)

4 Regresiones, 4 Mejoras, 6 Mixtas; 1 de ellas en consolidados
33 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-01-08.md)

### [RFCs Aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFCs que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFCs esta semana.*

### Período de Comentario Final
Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período de comentario final' para RFCs y PRs clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: constants in patterns](https://github.com/rust-lang/rfcs/pull/3535)
* [disposition: merge] [Add RFC combining Infra and Release teams](https://github.com/rust-lang/rfcs/pull/3533)
* [disposition: merge] [RFC: Precise Pre-release `cargo update`](https://github.com/rust-lang/rfcs/pull/3493)
* [disposition: postpone] [[Draft] RFC: Patch dependencies using unidiff patchfiles](https://github.com/rust-lang/rfcs/pull/3177)
---

#### [Seguimiento de Problemas y Pull Requests](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusión] [error en límites implícitos incorrectos en wfcheck excepto para dependientes de Bevy](https://github.com/rust-lang/rust/pull/118553)
* [disposición: fusión] [Estabilizar `slice_first_last_chunk`](https://github.com/rust-lang/rust/pull/117561)
* [disposición: fusión] [Advertir sobre referencias que se convierten en un diseño de memoria más grande](https://github.com/rust-lang/rust/pull/118983)
* [disposición: fusión] [Intercambio const-eval: eliminar la travesía impulsada por el tipo](https://github.com/rust-lang/rust/pull/119044)
* [disposición: fusión] [Issue de Seguimiento para `round_ties_even`](https://github.com/rust-lang/rust/issues/96710)
* [disposición: fusión] [Estabilizar offset_of de un solo campo](https://github.com/rust-lang/rust/pull/118799)
* [disposición: fusión] [revertir la estabilización de const_intrinsic_copy](https://github.com/rust-lang/rust/pull/117905)
* [disposición: fusión] [[rustdoc] Permite enlaces en encabezados](https://github.com/rust-lang/rust/pull/117662)
* [disposición: fusión] [Usar ordenación por versión para todas las ordenaciones](https://github.com/rust-lang/rust/pull/115046)
* [disposición: fusión] [Denegar invocaciones de macro con llaves en let-else](https://github.com/rust-lang/rust/pull/119062)

### [Referencia de Lenguaje](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Referencia de Lenguaje entró en el Período Final de Comentarios esta semana.*

### [Directrices de Código No Seguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de las Directrices de Código No Seguro entró en el Período Final de Comentarios esta semana.*

### [RFCs Nuevas y Actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Permitir la reexportación de símbolos en una crate cdylib desde un staticlib enlazado](https://github.com/rust-lang/rfcs/pull/3556)
* [RFC: cargo-sbom](https://github.com/rust-lang/rfcs/pull/3553)

### [Llamado a Pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con la
implementación y den retroalimentación, especialmente antes de la estabilización.  Los siguientes
RFC se beneficiarían de pruebas de usuario antes de avanzar:

* *Ningún RFC emitió un llamado a pruebas esta semana.*

Si eres un implementador de funciones y deseas que tu RFC aparezca en la lista anterior, agrega la nueva etiqueta `call-for-testing`
a tu RFC junto con un comentario proporcionando instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Próximos Eventos

Eventos de Rust entre el 2024-01-10 y el 2024-02-07 🦀

### Virtual

* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Encuentro Rust de Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Desarrollo colaborativo de intérpretes en Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-11 | Virtual (Núremberg, DE) | [Rust Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Núremberg en línea**](https://www.meetup.com/rust-noris/events/295679708/)
* 2024-01-11 | Virtual (San Diego, CA, US) | [Rust San Diego](https://www.meetup.com/san-diego-rust/)
    * [**Teleencuentro de Rust de enero de 2024 en San Diego**](https://www.meetup.com/san-diego-rust/events/298441403/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/297128172/)
* 2024-01-17 | Virtual (Vancouver, BC, CA) | [Rust Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio/Hackeo/Reunión de Rust**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 2024-01-21 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Desarrollo web con Rocket - En inglés**](https://meet-os.com/event/1)
* 2024-01-23 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 2024-01-24 | Virtual (Berlín, DE) | [WeAreDevelopers Community](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Día de Rust**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 2024-01-25 | Virtual (Charlottesville, NC, US) | [Encuentro Rust de Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Desarrollo colaborativo de intérpretes en Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
---
* 2024-01-25 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 2024-01-28 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 2024-01-30 | Virtual | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 2024-01-30 | Virtual (Buffalo, NY, US) | [Buffalo Rust User Group](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 2024-01-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
* 2024-02-01 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin)
* 2024-02-03 | Virtual + Presencial (Bruselas, BE) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**Conferencia FOSDEM: Sala Rust - charlas**](https://fosdem.org/2024/schedule/track/rust/)
* 2024-02-03 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 2024-02-07 | Virtual (Indianápolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con Distanciamiento Social**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creando Intérpretes en Rust Colaborativamente**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)

### Europa

* 2024-01-10 | Colonia, DE | [Rust Colonia](https://www.meetup.com/rustcologne/)
    * [**Desarrollo de juegos en Rust**](https://www.meetup.com/rustcologne/events/298303772/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro Reading Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Encuentro Rust #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Tampere, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**Encuentro de enero**](https://www.meetup.com/finland-rust-meetup/events/297811750/)
* 2024-01-16 | Leipzig, DE | [Rust - Programación Moderna de Sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Async en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297376712/)
* 2024-01-17 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Qué es Rust y sus ventajas / What's Rust and its advantages**](https://www.meetup.com/rust-girona/events/294080437/)
* 2024-01-17 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Encuentro Rust Reloaded 2024**](https://www.meetup.com/rust-prague/events/298005196/) 
* 2024-01-17 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/rust-zurich/)
    * [**Cómo Comunidad - Encuentro de enero**](https://www.meetup.com/rust-zurich/events/298066842/)
* 2024-01-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack and Learn**](https://www.meetup.com/rust-aarhus/events/297463730/)
* 2024-01-23 | París, FR | [Rust París](https://mobilizon.fr/@rust_paris)
    * [**Encuentro Rust París #64**](https://mobilizon.fr/events/0fce31cd-3578-43f2-abf4-ffecd8d16da2)
* 2024-02-01 | Barcelona, ES | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)
---

### América del Norte

* 2024-01-11 | Lehi, UT, EE. UU. | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Asignación de Arena: Otro enfoque para gestionar lifetimes con Taylor Allred**](https://www.meetup.com/utah-rust/events/298448713/)
* 2024-01-14 | Cambridge, MA, EE. UU. | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, EE. UU. | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, EE. UU. | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 2024-01-18 | Seattle, WA, EE. UU. | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Reunión del grupo de usuarios de Rust de Seattle**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, EE. UU. | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 2024-01-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Hackatón para aprovechar Rust en problemas del mundo real: Día 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Hackatón para aprovechar Rust en problemas del mundo real: Día 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 2024-01-30 | Cambridge, MA, EE. UU. | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 2024-02-07 | Brookline, MA, EE. UU. | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, 7 de febrero**](https://www.meetup.com/bostonrust/events/297635028/)

### Oceanía

* 2024-01-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Reunión del meetup de Rust de Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/298382221/)
* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Reunión de Rust de febrero de 2024**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

Si estás organizando un evento de Rust, agrégalo al [calendario] para que se mencione aquí. Recuerda agregar también un enlace al evento. Envía un correo electrónico al [Equipo de la Comunidad Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Trabajos
<!--

Trabajos de Rust:

TWiR ha dejado de destacar anuncios de trabajo individuales. Puedes obtener más información sobre este cambio aquí:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Consulta el último [hilo de "¿Quién está contratando?" en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> * Modular
> * Construcción de muy alta calidad en comparación con sus competidores
> * Si lo dejas por ahí, olvídate de él, entrar en un proyecto es doloroso?

– [Leonardo Giovanni Scur en mastodon](https://floss.social/@kroltan@functional.cafe/111687927473117112) explicando cómo [bevy](https://bevyengine.org) es como Lego™

Gracias a [Jan Riemer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1506) por la sugerencia!

[¡Envía citas y vota para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*Esta Semana en Rust es editada por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/199ctmk/this_week_in_rust_529/)</small>
