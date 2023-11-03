---
title: "Esta semana en Rust #4"
number_of_week: 4
description: Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.
date: 2023-11-01
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

* [Una historia de insignias rotas y 23.000 características](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html)
* [crates.io: Abandono del soporte para descargas no canónicas](https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html)
* [Los generadores están muertos, larga vida a las corrutinas, los generadores están de vuelta](https://blog.rust-lang.org/inside-rust/2023/10/23/coroutines.html)

### Actualizaciones de proyectos/herramientas

* [¡Ratatui 0.24.0 está disponible! (Biblioteca de Rust que se trata de cocinar interfaces de usuario de terminales)](https://ratatui.rs/highlights/v0.24.html)
* [¡Lanzamiento de Git-Cliff 1.4.0! (generador de registro de cambios altamente personalizable)](https://git-cliff.org/blog/1.4.0/)
* [registro de cambios de rust-analyzer #205](https://rust-analyzer.github.io/thisweek/2023/10/30/changelog-205.html)
* [Informe de situación sobre rustc_codegen_cranelift (octubre de 2023)](https://bjorn3.github.io/2023/10/31/progress-report-oct-2023.html)

### Observaciones/Pensamientos

* [Las dependencias del sistema son difíciles (así que las hicimos más fáciles)](https://blog.axo.dev/2023/10/dependencies)
* [¿Valió la pena Rust?](https://jsoverson.medium.com/was-rust-worth-it-f43d171fb1b3)
* [¿Puede Rust evitar errores lógicos?](https://itsallaboutthebit.com/logic-errors-in-rust/)
* [Serialización de Rust más rápida](https://mo8it.com/blog/faster-rust-serialization/)
* [Lidiando con dependencias en Rust](https://tweedegolf.nl/en/blog/104/dealing-with-dependencies-in-rust)
* [Cómo aprendí a dejar de preocuparme y amar el orden de bytes](https://udoprog.github.io/rust/2023-10-28/stop-worrying.html)
* [Cómo moverse rápido con Rust](https://endler.dev/2023/move-fast-rust)
* [Iniciar una reunión virtual de Rust](https://hegdenu.net/posts/virtual-rust-meet-up/)
* [video] [Impl Trait aka Look ma', no generics! por Jon Gjengset](https://www.youtube.com/watch?v=CWiz_RtA1Hw)

### Tutoriales de Rust

* [Lanzamientos totalmente automatizados para proyectos de Rust](https://blog.orhun.dev/automated-rust-releases/)
* [La belleza de un procesador de mensajes Rust](https://worldwithouteng.com/articles/the-beauty-of-a-rust-message-processor/)
* [interrumpe es hilos](https://onevariable.com/blog/interrupts-is-threads/)
* [htmx, Rust & Shuttle: A New Rapid Prototyping Stack](https://www.shuttle.rs/blog/2023/10/25/htmx-with-rust)
* [video] [No se requiere 🦀 asíncrono](https://www.youtube.com/watch?v=QXynWxALJmo)

### Investigación

* [Propiedad funcional a través de la unicidad fraccionaria](https://arxiv.org/abs/2310.18166)
* [Grading on a Curve: How Rust Can Facilitate New Contributors While Reducing Vulnerabilities](https://cypherpunks.ca/~iang/pubs/gradingcurve-secdev23.pdf)

### Miscelánea

* [video] [5 horas a 7.7 segundos: Cómo los trucos de la base de datos aceleraron el revestimiento de Rust más de 2000x](https://www.youtube.com/watch?v=Fqo8r4bInsk)

## Crate de la semana

El crate de esta semana es [silkenweb](https://crates.io/crates/silkenweb), una biblioteca para crear aplicaciones web con una reactividad de grano fino y una separación clara de la lógica y la interfaz de usuario.

¡Gracias a [henrik](https://users.rust-lang.org/t/crate-of-the-week/2704/1255) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [pubgrub - Pasar a la edición 2021](https://github.com/pubgrub-rs/pubgrub/issues/140)
* [pubgrub - Arreglar CI para commits convencionales](https://github.com/pubgrub-rs/pubgrub/issues/139)
* [pubgrub - Cambiar el nombre del rango v0.3 a BoundedRange](https://github.com/pubgrub-rs/pubgrub/issues/123)
* [pubgrub - 'OfflineDependencyProvider' debería tener su propio módulo](https://github.com/pubgrub-rs/pubgrub/issues/114)
* [Ockam - Biblioteca - Adelgazar el 'NodeManagerWorker' para 'node / tcp'](https://github.com/build-trust/ockam/issues/6708)
* [Ockam - Hacer que 'ockam vault delete' (sin args) sea interactivo pidiendo al usuario que elija de una lista de bóvedas para eliminar (tuify)](https://github.com/build-trust/ockam/issues/6462)
* [Ockam - Comando - refactorizar para usar interfaces con tipo para implementar comandos para 'servicios'](https://github.com/build-trust/ockam/issues/6700)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

408 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-23..2023-10-30

* [-Zunpretty help: añadir los valores posibles que faltan](https://github.com/rust-lang/rust/pull/117311)
* [NVPTX: permitir 'PassMode::D irect' para kernels ptx por ahora](https://github.com/rust-lang/rust/pull/117247)
* ['OptWithInfcx' nomenclatura de nits, simplificaciones ligadas a rasgos](https://github.com/rust-lang/rust/pull/117091)
* ['deduce_param_attrs': explica un caso de solo lectura](https://github.com/rust-lang/rust/pull/117385)
* ['rustc_interface' limpiezas](https://github.com/rust-lang/rust/pull/117268)
* ['stack_overflow: get_stackp' usando la bandera 'MAP_STACK' en dragonflybsd también](https://github.com/rust-lang/rust/pull/117102)
* [add bootstrap flag '--skip-stage0-validation'](https://github.com/rust-lang/rust/pull/117043)
* [añadir método para convertir construcciones internas en estables](https://github.com/rust-lang/rust/pull/117010)
* [añadir soporte para i586-unknown-netbsd como destino](https://github.com/rust-lang/rust/pull/117170)
* [añadir soporte para mipsel-unknown-netbsd, mips LE de 32 bits](https://github.com/rust-lang/rust/pull/117356)
* [agregar características de destino para LoongArch](https://github.com/rust-lang/rust/pull/116943)
* [añadir una forma de diferenciar los locales de argumento de otros locales en MIR estable](https://github.com/rust-lang/rust/pull/117095)
* [permitir valores parcialmente movidos en la coincidencia](https://github.com/rust-lang/rust/pull/103208)
* [permitir que las especificaciones de destino usen un tipo de LLD y componentes de enlace autónomos](https://github.com/rust-lang/rust/pull/116035)
* [permite que los atributos '#[diagnostic::on_unimplemented]' tengan múltiples](https://github.com/rust-lang/rust/pull/117205)
* [¡Evite el internamiento repetido de 'env! ("CFG_RELEASE")'](https://github.com/rust-lang/rust/pull/117188)
* [evite 'O(n^2)' ilimitado al analizar args de tipo anidado](https://github.com/rust-lang/rust/pull/117143)
* [evitar construcciones/reconstrucciones innecesarias de 'rust-demangler'](https://github.com/rust-lang/rust/pull/117197)
* [evitar la renumeración innecesaria durante el préstamo](https://github.com/rust-lang/rust/pull/116792)
* [Mejoras en el error c-variádico](https://github.com/rust-lang/rust/pull/117370)
* [considere los límites de alias al calcular la vida en NLL (pero esta vez suena con suerte)](https://github.com/rust-lang/rust/pull/116733)
* [const stabilize 'mem::d iscriminant'](https://github.com/rust-lang/rust/pull/116240)
* [Manejar correctamente los patrones OR anidados en exhaustividad](https://github.com/rust-lang/rust/pull/117398)
* [cobertura: eliminar consistentemente los ID de contador no utilizados de las expresiones/asignaciones](https://github.com/rust-lang/rust/pull/117123)
* [Cobertura: Reemplace las sangrías de depuración manual con intervalos de seguimiento anidados en 'contadores'](https://github.com/rust-lang/rust/pull/117350)
* [crear 'windows/api.rs' para una FFI más segura](https://github.com/rust-lang/rust/pull/116816)
* [crear una nueva variante de ConstantKind (ZeroSized) para StableMIR](https://github.com/rust-lang/rust/pull/117262)
* [declarar la dependencia de 'rustc_target' de objeto/macho](https://github.com/rust-lang/rust/pull/117259)
* [denegar el suministro de parámetros de efecto explícitos](https://github.com/rust-lang/rust/pull/117171)
* [derivar 'Ord', 'PartialOrd' y 'Hash' para 'SocketAddr*'](https://github.com/rust-lang/rust/pull/116714)
* [Detectar cuándo se implementa el rasgo para el tipo y sugerir importarlo](https://github.com/rust-lang/rust/pull/116862)
* [no sugiera ''Trait<Assoc=arg>'' cuando está en trait impl](https://github.com/rust-lang/rust/pull/116553)
* [no normalizar a un opaco no revelado cuando alcanzamos el límite de recursividad](https://github.com/rust-lang/rust/pull/117414)
* [no tratar los tipos de cierres/corrutinas como parte de la API pública](https://github.com/rust-lang/rust/pull/117396)
* [no use símbolos LFS64 en el idioma](https://github.com/rust-lang/rust/pull/115968)
* [coma paren cerrado si 'capture_cfg' para evitar parenes desequilibrados](https://github.com/rust-lang/rust/pull/116889)
* [Habilitar pruebas 'cg_clif' para RISCV64GC](https://github.com/rust-lang/rust/pull/117032)
* [error tipock por ruptura ilegal con valor](https://github.com/rust-lang/rust/pull/117382)
* [arreglar ICE: Restringir sugerencia de restricción de parámetros](https://github.com/rust-lang/rust/pull/117246)
* [Se corrigió la falla al detectar un tipo demasiado grande después de agregar relleno](https://github.com/rust-lang/rust/pull/117277)
* [corregir la comprobación de inseguridad del patrón const en línea en THIR](https://github.com/rust-lang/rust/pull/116482)
* [Se corrige el espacio inicial que falta en la sugerencia](https://github.com/rust-lang/rust/pull/117395)
* [Soluciona el problema de lint de variables no utilizadas para args en macro](https://github.com/rust-lang/rust/pull/117390)
* [Corregir sugerencia de desenvoltura para fn asíncrono](https://github.com/rust-lang/rust/pull/117152)
* [generar constantes agregadas en DataflowConstProp](https://github.com/rust-lang/rust/pull/115796)
* [manejar 'ReErased' en las respuestas en el nuevo solucionador](https://github.com/rust-lang/rust/pull/116435)
* [ignorar las duraciones duplicadas de RPIT en 'opaque_types_defined_by'](https://github.com/rust-lang/rust/pull/117371)
* [implementar la reducción del ITB C para CSKY](https://github.com/rust-lang/rust/pull/117154)
* [implementar bloques 'gen' en la edición de 2024](https://github.com/rust-lang/rust/pull/116447)
* [Mejorar la interfaz de propiedades de Android-NDK](https://github.com/rust-lang/rust/pull/116998)
* [mejorar algunos diagnósticos en torno a '? Límites de rasgos](https://github.com/rust-lang/rust/pull/117411)
* [mejorar los mensajes de advertencia para el '#[diagnostic::on_unimplemented]'](https://github.com/rust-lang/rust/pull/116931)
* [aumentar el alcance de 'panic_immediate_abort'](https://github.com/rust-lang/rust/pull/117332)
* [intern 'LocalDefId' lista de la consulta 'opaque_types_defined_by'](https://github.com/rust-lang/rust/pull/117136)
* [introducir '-C instrument-coverage=branch' a la cobertura de la rama de la puerta](https://github.com/rust-lang/rust/pull/116094)
* [sugerencia '' no válida en 'Ok(T)'](https://github.com/rust-lang/rust/pull/116968)
* [lint superponiendo rangos como una pasada separada](https://github.com/rust-lang/rust/pull/116751)
* [marcar los archivos '.rmeta' como '/SAFESEH' en Windows x86](https://github.com/rust-lang/rust/pull/117115)
* [marcar el constructor de 'BinaryHeap' como const fn](https://github.com/rust-lang/rust/pull/117316)
* [Nunca consideres que los lanzamientos de puntero en bruto sean trivales](https://github.com/rust-lang/rust/pull/113262)
* [Sobre el error de seguridad del objeto, mencione una nueva 'enumeración' como alternativa](https://github.com/rust-lang/rust/pull/117132)
* [en importaciones no resueltas, sugiera una ruta de desambiguación si es necesario para evitar colisiones con elementos locales](https://github.com/rust-lang/rust/pull/117009)
* [solo llame a 'mir_const_qualif' si es absolutamente necesario](https://github.com/rust-lang/rust/pull/117166)
* [solo emite un error por enlace sin tamaño, en lugar de uno por uso](https://github.com/rust-lang/rust/pull/113183)
* [envenenar 'check_well_formed' si los receptores del método no son válidos para evitar que typeck se ejecute en él](https://github.com/rust-lang/rust/pull/117403)
* [imprimir patrón de argumento variádico en impresora HIR bonita](https://github.com/rust-lang/rust/pull/117147)
* [restaurar correctamente la instantánea cuando no se puede recuperar el análisis ternario](https://github.com/rust-lang/rust/pull/117212)
* [falla silenciosamente si ya se ha producido un error](https://github.com/rust-lang/rust/pull/117214)
* [rand use getrandom para freeBSD (disponible desde 12.x)](https://github.com/rust-lang/rust/pull/107159)
* [Refactorizar el tipo de visitante caminando](https://github.com/rust-lang/rust/pull/117076)
* [reasignar las dependencias de carga a /rust/deps](https://github.com/rust-lang/rust/pull/115872)
* [remove -Zdep-tasks](https://github.com/rust-lang/rust/pull/116534)
* [eliminar los respaldos RNG de Apple y simplificar la implementación](https://github.com/rust-lang/rust/pull/116319)
* [eliminar el código de plegado y agregar 'Const::internal()' a StableMIR](https://github.com/rust-lang/rust/pull/117113)
* [eliminar el soporte para el alias '-Z instrument-coverage'](https://github.com/rust-lang/rust/pull/117111)
* [Requerir que las entidades de destino coincidan exactamente durante la inserción](https://github.com/rust-lang/rust/pull/117141)
* [devuelve múltiples errores de violación de la seguridad de los objetos y mejoras de código en la comprobación de la seguridad de los objetos](https://github.com/rust-lang/rust/pull/116401)
* [devuelve LEN no fijado si Pat ha informado de un error](https://github.com/rust-lang/rust/pull/117046)
* [reelaborar la coherencia negativa para considerar adecuadamente los implicamientos que solo se superponen parcialmente](https://github.com/rust-lang/rust/pull/112875)
* [rustdoc: elide cross-crate default arguments generive](https://github.com/rust-lang/rust/pull/112463)
* [rustdoc: use 'ThinVec' en 'GenericParamDefKind'](https://github.com/rust-lang/rust/pull/117337)
* [ver a través de agregados en GVN](https://github.com/rust-lang/rust/pull/116270)
* [Seguimiento de la ruta de movimiento separada entre Borrowck y Drop Elaboration](https://github.com/rust-lang/rust/pull/116300)
* [comparte algo de lógica 'track_caller' entre interpret y codegen](https://github.com/rust-lang/rust/pull/117317)
* [pequeñas limpiezas 'ty::p rint'](https://github.com/rust-lang/rust/pull/117325)
* [algunas mejoras de diagnóstico de los bloques 'gen'](https://github.com/rust-lang/rust/pull/117389)
* [Ocultar y cancelar errores de ciclo para la fuga de rasgos automáticos en opacos](https://github.com/rust-lang/rust/pull/117241)
* [deja de decirle a la gente que envíe errores para los ICEs de características internas](https://github.com/rust-lang/rust/pull/116818)
* [almacenar el valor 'desde' del atributo '#[estable]' en forma estructurada](https://github.com/rust-lang/rust/pull/117148)
* [sugerir assoc fn 'new' cuando se intenta construir la tupla 'struct' con campos privados](https://github.com/rust-lang/rust/pull/116858)
* [sugerir unwrap/expect for let binding type mismatch](https://github.com/rust-lang/rust/pull/116841)
* [Soporte de simulador de tvOS en Apple Silicon para rustc](https://github.com/rust-lang/rust/pull/115773)
* [Ajustar el intervalo de sugerencia para el atributo externo y apuntar al elemento que sigue al atributo interno no válido](https://github.com/rust-lang/rust/pull/116868)
* [eleva 'Canónico' a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/117008)
* [eleva 'ClauseKind' y 'PredicateKind' a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/116993)
* [use ImageDataType para el tipo de asignación](https://github.com/rust-lang/rust/pull/117177)
* [valida los valores 'feature' y 'since' dentro de '#[stable(...)] «](https://github.com/rust-lang/rust/pull/116773)
* [cuando se encuentran rasgos sellados, tipos de puntos que lo implementan](https://github.com/rust-lang/rust/pull/116945)
* [cuando se espera el argumento de cierre pero se encuentra el bloqueo proporcionar una sugerencia](https://github.com/rust-lang/rust/pull/117106)
* [solucione el hecho de que 'check_mod_type_wf' puede devolver falsamente 'ErrorGuaranteed'](https://github.com/rust-lang/rust/pull/117159)
* [Hora: use 'clock_gettime' en macOS](https://github.com/rust-lang/rust/pull/116238)
* [Windows: admite suspensión de menos de milisegundos](https://github.com/rust-lang/rust/pull/116461)
* [refactorizar algunas funciones ASCII 'char', 'u8' para que no tengan ramas](https://github.com/rust-lang/rust/pull/117260)
* [añadir '#[inline]' a algunos métodos recalcitrantes 'ops::range'](https://github.com/rust-lang/rust/pull/117038)
* [estabilizar las características del objetivo RISC-V ratificadas](https://github.com/rust-lang/rust/pull/116485)
* [estabilizar '[const_]pointer_byte_offsets'](https://github.com/rust-lang/rust/pull/116205)
* [Estabilizar el uso de ASM en línea con 'rustc_codegen_cranelift'](https://github.com/rust-lang/rust/pull/117365)
* [futuros: añadir adaptadores 'TryAll' y 'TryAny'](https://github.com/rust-lang/futures-rs/pull/2783)
* [futuros: arreglar la implicación de 'Sincronizar' de 'FuturosDesordenados'](https://github.com/rust-lang/futures-rs/pull/2788)
* [futuros: proporcionar AtomicWaker si la función de atómico portátil está habilitada, incluso si el CAS atómico no está disponible](https://github.com/rust-lang/futures-rs/pull/2790)
* [codegen\_gcc: agregar conceptos básicos para el comando 'test' en el sistema de compilación](https://github.com/rust-lang/rustc_codegen_gcc/pull/363)
* [codegen\_gcc: arreglar 'volatile_load'](https://github.com/rust-lang/rustc_codegen_gcc/pull/365)
* [cargo toml: Permitir manifiestos sin versión](https://github.com/rust-lang/cargo/pull/12786)
* [cargo toml: Desacoplar el análisis sintáctico del sistema de internado](https://github.com/rust-lang/cargo/pull/12881)
* [cargo: shell: Escribe de una vez en lugar de en fragmentos](https://github.com/rust-lang/cargo/pull/12880)
* [cargo: añadir nuevos paquetes a '[workspace.members]' automáticamente](https://github.com/rust-lang/cargo/pull/12779)
* [cargo: subir mirando el resumen del índice 'enum'](https://github.com/rust-lang/cargo/pull/12749)
* [cargo: eliminar binarios duplicados durante la instalación](https://github.com/rust-lang/cargo/pull/12868)
* [cargo: eliminar la opción obsoleta de las advertencias '-Zcheck-cfg'](https://github.com/rust-lang/cargo/pull/12884)
* [rustfmt: error corregido causado por la combinación de 'match_arm_blocks' y 'control_brace_style'](https://github.com/rust-lang/rustfmt/pull/5923)
* [clippy: 'ignored_unit_patterns': comprobar &(), &&(),](https://github.com/rust-lang/rust-clippy/pull/11670)
* [clippy: 'iter_without_into_iter': corrige los recortes de papel en la sugerencia y restringe el linting a los tipos exportados](https://github.com/rust-lang/rust-clippy/pull/11696)
* [clippy: 'let_and_return': Envolver con paréntesis si es necesario](https://github.com/rust-lang/rust-clippy/pull/11584)
* [clippy: agregue la pelusa 'waker_clone_and_wake' para verificar clones innecesarios de 'Waker'](https://github.com/rust-lang/rust-clippy/pull/11698)
* [clippy: se corrige el paréntesis que falta en la ayuda de coma flotante subóptima](https://github.com/rust-lang/rust-clippy/pull/11724)
* [clippy: ignora las palabras en minúsculas en 'doc_markdown'](https://github.com/rust-lang/rust-clippy/pull/11735)
* [clippy: mover 'read_zero_byte_vec' a la guardería](https://github.com/rust-lang/rust-clippy/pull/11727)
* [clippy: eliminar la función interna de 'clippy_utils'](https://github.com/rust-lang/rust-clippy/pull/11723)
* [clippy: eliminar la categoría de pelusa 'internal_warn'](https://github.com/rust-lang/rust-clippy/pull/11712)
* [Rust-Analyzer: hacer que 'extract_variable' ayude en su lugar](https://github.com/rust-lang/rust-analyzer/pull/15809)

### Clasificación del rendimiento del compilador de Rust

Esta semana tenemos dos conjuntos de resultados, ya que los de la semana pasada llegaron más tarde de la fecha de publicación:

Triaje realizado por **@rylev** y **@simulacrum**.

Rango de revisión: [b9832e72.. 650991d](https://perf.rust-lang.org/?start=b9832e72c9223f4e96049aa5911effd258b92591&end=650991d62c3a2c80ba27009d06839adbb038bf5e&absolute=false&stat=instructions%3Au)

En ambos informes:

9 Regresiones, 7 Mejoras, 5 Mixtas
127 comparaciones de artefactos realizadas en total

* [Informe completo #1](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-26.md)
* [Informe completo #2](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-10-31.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* [Edición 2024](https://github.com/rust-lang/rfcs/pull/3501)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: fusionar] [Reglas de captura de por vida 2024](https://github.com/rust-lang/rfcs/pull/3498)
* [disposición: fusionar] [Agregar RFC "Actualización de política crates.io"](https://github.com/rust-lang/rfcs/pull/3463)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposición: fusionar] [Anular 'Waker::clone'_from para evitar clonar Wakers innecesariamente](https://github.com/rust-lang/rust/pull/96979)
* [Disposición: Fusionar] [dropck_outlives comprobar si el testigo generador needs_drop](https://github.com/rust-lang/rust/pull/117134)
* [disposición: fusionar] [tipo oculto más estricto wf-check](https://github.com/rust-lang/rust/pull/115008)
* [disposición: fusionar] [Problema de seguimiento de los iteradores GroupBy y GroupByMut](https://github.com/rust-lang/rust/issues/80552)
* [disposition: merge] [No entres en pánico en '<BorrowedCursor as io::Write>::write'](https://github.com/rust-lang/rust/pull/115460)
* [disposición: fusionar] [Garantizar que 'char' tenga el mismo tamaño y alineación que 'u32'](https://github.com/rust-lang/rust/pull/116894)
* [disposición: fusionar] [Estabilizar 'const_maybe_uninit_zeroed' y 'const_mem_zeroed'](https://github.com/rust-lang/rust/pull/116218)
* [disposición: fusionar] [Aclarar UB en 'get_unchecked(_mut)'](https://github.com/rust-lang/rust/pull/117039)
* [disposición: fusionar] [documento que el puntero nulo tiene la dirección 0](https://github.com/rust-lang/rust/pull/116988)
* [disposición: cerrar] [regresión: el tipo de parámetro puede no vivir lo suficiente](https://github.com/rust-lang/rust/issues/117055)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposición: fusionar] [Garantizar que las conversiones de puntero sin procesar conserven el recuento de elementos de corte](https://github.com/rust-lang/reference/pull/1417)

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos o actualizados esta semana.*

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-11-01 - 2023-11-29 🦀

### Virtual

* 01/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS con Bevy Game Engine**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583207/)
* 01/11/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 02/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 07/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679790/) | [**Espejo**](https://berline.rs/)
* 07/11/2023 | Virtual (Búfalo, NY, EE. UU.) | [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/296827010/)
* 09/11/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 14/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 15/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Construyendo Nuestras Propias Cerraduras (Atómicas y Cerraduras Capítulo 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 15/11/2023 | Virtual (Richmond, VA, EE. UU.) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 15/11/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 16/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 21/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 21/11/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/296807537/)
* 28/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)

### Europa

* 01/11/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Aplicaciones web con axum: ¡Hola CRUD!**](https://www.meetup.com/rustcologne/events/296540949/)
* 07/11/2023 | Bratislava, SK | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/296809100/)
* 07/11/2023 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Edición para principiantes de Rust and Talk**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 07/11/2023 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #7**](https://www.meetup.com/rust-lyon/events/296853019/)
* 09/11/2023 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/296567395)
* 09/11/2023 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547/)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-732823744547)
* 09/11/2023 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 21/11/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Procesamiento de GPU en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 23/11/2023 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### América del Norte

* 01/11/2023 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de la roya común de Boston**](https://www.meetup.com/bostonrust/events/296223910/)
* 02/11/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062678/)
* 08/11/2023 | Boulder, CO, EE. UU. | [Reunión de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**¡Hagamos un bot de Discord!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 14/11/2023 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀 **](https://www.meetup.com/rust-nyc/events/296895126/)
* 14/11/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Noche de Codificación/Hackeo/Aprendizaje Oxidado**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 15/11/2023 | Richmond, VA, EE. UU. + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 16/11/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 16/11/2023 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**¡A Python le encanta Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 16/11/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 21/11/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 22/11/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)

### Oceanía

* 06/11/2023 | Perth, WA, AU | [Grupo de Meetup de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Abrazando las vidas: Un viaje hacia un código seguro y eficiente**](https://www.meetup.com/perth-rust-meetup-group/events/296963595)
* 21/11/2023 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Reunión de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 28/11/2023 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/296391733/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Después de hacer un mejor ajuste, descubrimos que los proyectos de Rust tenían menos probabilidades de introducir vulnerabilidades que sus proyectos equivalentes de C++ en todos los niveles de experiencia relevantes, pero lo que es más importante, descubrimos que el efecto era más significativo para los contribuyentes primerizos, que tenían casi dos órdenes de magnitud menos probabilidades de contribuir con vulnerabilidades. Es decir, a pesar de que Rust puede tener la reputación de ser un idioma más difícil de aprender, hay un efecto muy medible que lo hace mejor para los novatos. Los revisores no deberían tener que esforzarse tanto en revisar el código para estar seguros de que alguien que hace su primera incursión en su proyecto está agregando accidentalmente una vulnerabilidad.

– [Justin Tracey en crysp.org](https://ftp.crysp.org/@j3tracey/111315653313272566)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1473) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/17lqhp2/this_week_in_rust_519/)</small>

