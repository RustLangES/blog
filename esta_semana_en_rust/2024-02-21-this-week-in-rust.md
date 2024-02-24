---
title: "Esta semana en Rust #19"
number_of_week: 19
description: El crate de esta semana es kind, una caja auxiliar para UUID con tipo.
date: 2024-02-21
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

### Oficial

* [Resultados de la Encuesta Anual de Roya 2023](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html)
* [Rust participa en Google Summer of Code 2024](https://blog.rust-lang.org/2024/02/21/Rust-participates-in-GSoC-2024.html)
* [Selecciones de Representantes de marzo del Consejo de Liderazgo](https://blog.rust-lang.org/inside-rust/2024/02/19/leadership-council-repr-selection.html)

### Fundación

* [Reserve la fecha: RustConf 2024 – 10-13 de septiembre](https://foundation.rust-lang.org/news/save-the-date-rustconf-2024-september-10-13/)
* [El segundo informe de la iniciativa de seguridad detalla los avances de seguridad de Rust](https://foundation.rust-lang.org/news/second-security-initiative-report-details-rust-security-advancements/)

### Actualizaciones de proyectos/herramientas

* [Bevy 0.13](https://bevyengine.org/news/bevy-0-13/)
* [Bevy XPBD 0.4: Agnosticismo del colisionador, reelaboración de capas y Bevy 0.13](https://joonaa.dev/blog/05/bevy-xpbd-0-4-0)
* [uv: Empaquetado de Python en Rust](https://astral.sh/blog/uv)
* [git-cliff: ¿Qué hay de nuevo en la versión 2.0.0? (generador de registro de cambios altamente personalizable)](https://git-cliff.org/blog/2.0.0/)
* [rustc_codegen_gcc: Informe de Progreso #30](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-30)
* [Retrospectiva de actualizaciones de características de RustRover Q4 2023](https://blog.jetbrains.com/rust/2024/02/14/q4-2023-features-retrospective/)
* [registro de cambios de rust-analyzer #221](https://rust-analyzer.github.io/thisweek/2024/02/19/changelog-221.html)
* [Anunciando Stabby 3.0](https://www.reddit.com/r/rust/comments/1amjknw/anouncing_stabby_300_and_rustconf_video_available/)
* [argmin_testfunctions 0.2.0: funciones de prueba para problemas de optimización en Rust y Python](https://argmin-rs.org/blog/argmin-testfunctions-v0-2-0/)
* [vscode-rustup liberado: interfaz con rustup como extensión VSCode](https://github.com/emberian/vscode-rustup)
* [Publicado OpenPGP-card-tools 0.10.0. El proyecto proporciona la herramienta de línea de comandos 'oct' para inspeccionar, configurar y usar dispositivos de tarjeta OpenPGP como Nitrokey o Yubikey.](https://codeberg.org/openpgp-card/openpgp-card-tools)
* [Cliente de Rust para Timeplus Proton SQL Streaming](https://www.timeplus.com/post/rust-client-for-proton)

### Observaciones/Pensamientos

* [FuturosDesordenados y el orden de los futuros](https://without.boats/blog/futures-unordered/)
* [Herramientas de Rust: 8 herramientas que aumentarán su productividad](https://www.shuttle.rs/blog/2024/02/15/best-rust-tooling)
* [Escribiendo mi modelo mental de inseguridad](https://gist.github.com/ia0/820ab50d4c5f0f5e3aeb841cef8e6792)
* [¿Cómo puede Rust ser tan rápido en los puntos de referencia de TechEmpower Web Framework?](https://kerkour.com/rust-fast-techempower-web-framework-benchmarks)

### Tutoriales de Rust

* [De 1 s a 4 ms](https://registerspill.thorstenball.com/p/from-1s-to-4ms)
* [Traducción de datos de OpenStreetMap a HTML5 Canvas con Rust y WebAssembly](https://mary.codes/blog/programming/translating_openstreetmaps_to_HTML5_canvas_rust_wasm/)
* [¡macros_rule!](https://auroranssolis.github.io/rust/2024/02/14/macros-rule.html)
* [Implementación de la autenticación JWT en Rust](https://www.shuttle.rs/blog/2024/02/21/using-jwt-auth-rust)
* [Implementación de Axum en Lambda y ECS, mediante el adaptador web de Lambda](https://medium.com/@sam.van.overmeire/deploying-axum-to-lambda-and-ecs-using-lambda-web-adapter-2273bd56bb81)
* [Interoperabilidad de Rust/C++ Parte 3 - Cxx](https://tylerjw.dev/posts/rust-cmake-interop-part-3-cxx/)
* [FR] [Les closures en Rust](https://lafor.ge/closure/)
* [video] [Safe Rust NO ES SEGURO!? (CVE-RS)](https://www.youtube.com/watch?v=vfMpIsJwpjU)

### Miscelánea

* [video] [Release-please: liberando cajas como si fuera 2023 (RustLab 2023)](https://www.youtube.com/watch?v=kXPBVGDkQSs)

## Crate de la semana

El crate de esta semana es [kind](https://github.com/wingbackapp/kind/), una caja auxiliar para UUID con tipo.

¡Gracias a [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1290) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

#### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* [RFC: Comprobación de la compilación condicional en tiempo de compilación](https://github.com/rust-lang/rfcs/pull/3013)
    * [Pasos de prueba](https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479)

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Ockam - Resaltado de sintaxis para bloques de código cercados, en la salida de ayuda de comandos, en Linux funciona](https://github.com/build-trust/ockam/issues/7471)
* [Ockam - Apagar el Trabajador/Procesador si falla la inicialización](https://github.com/build-trust/ockam/issues/7575)
* [Ockam - Se ha mejorado la salida del ticket del proyecto ockam y la información no es opaca](https://github.com/build-trust/ockam/issues/7478)
* [Hyperswitch - [FEATURE] : añadir el campo 'offset' a la lista de disputas](https://github.com/juspay/hyperswitch/issues/3749)
* [Hyperswitch - [FEATURE]: añadir el campo 'offset' a la lista de mandatos](https://github.com/juspay/hyperswitch/issues/3748)
* [Hyperswitch - [CARACTERÍSTICA]: agregar soporte de paginación para la lista de clientes](https://github.com/juspay/hyperswitch/issues/3746)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

508 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-13..2024-02-20

* [Habilitar implícitamente EVEX512 si AVX512 está habilitado](https://github.com/rust-lang/rust/pull/121088)
* ['const_mut_refs': permitir punteros mutables a estáticas](https://github.com/rust-lang/rust/pull/120932)
* ['macro_rules': Conserva todos los intervalos de metavariables en una tabla lateral global](https://github.com/rust-lang/rust/pull/119673)
* [agregar APIs para obtener elementos externos](https://github.com/rust-lang/rust/pull/120982)
* [añadir la opción de configuración 'rust.frame-pointers'](https://github.com/rust-lang/rust/pull/121203)
* [añadir más comprobaciones de 'unnamed_fields' durante el análisis HIR](https://github.com/rust-lang/rust/pull/121198)
* [permitir que los visitantes AST y HIR devuelvan 'ControlFlow'](https://github.com/rust-lang/rust/pull/121256)
* [permitir referencias mutables en valores const cuando apuntan a que no hay memoria](https://github.com/rust-lang/rust/pull/121179)
* [Evalúe siempre las constantes libres y las estáticas, incluso si ocurrieron errores anteriores](https://github.com/rust-lang/rust/pull/121087)
* [evitar un ICE en el diagnóstico](https://github.com/rust-lang/rust/pull/121020)
* [tener menos confianza cuando no se comprueba la seguridad del objeto de la sugerencia 'dyn'](https://github.com/rust-lang/rust/pull/120530)
* [comprobar la firma de llamada normalizada para WF en mir typeck](https://github.com/rust-lang/rust/pull/118882)
* [Considere los superrasgos auto-rasgos de la referencia de rasgo principal en dyn upcasting](https://github.com/rust-lang/rust/pull/119338)
* [continuar compilando después de errores 'check_mod_type_wf'](https://github.com/rust-lang/rust/pull/120847)
* [continuar la compilación incluso si fallan las comprobaciones impl inherentes](https://github.com/rust-lang/rust/pull/121113)
* [continuar informando de los errores restantes en lugar de descartarlos silenciosamente](https://github.com/rust-lang/rust/pull/121032)
* [detectar cuándo se puede eliminar la llamada al método en el argumento para cumplir con el enlace de rasgo fallido](https://github.com/rust-lang/rust/pull/121100)
* [arreglar un ICE en la pelusa de recursividad](https://github.com/rust-lang/rust/pull/121181)
* [ignorar los tipos sin tamaño al intentar determinar el tamaño del tipo original](https://github.com/rust-lang/rust/pull/121104)
* [hacer que 'ConstPropLint' lint se ejecute en promocionados](https://github.com/rust-lang/rust/pull/119432)
* [mejorar los errores de tipo de rasgo 'asíncrono Fn'](https://github.com/rust-lang/rust/pull/121119)
* [tratar adecuadamente los tipos de alias débiles como tipos propios de impls](https://github.com/rust-lang/rust/pull/120780)
* [cambiar el nombre de 'ConstPropLint' a 'KnownPanicsLint'](https://github.com/rust-lang/rust/pull/121286)
* [almacenar inicializadores estáticos en metadatos en lugar del MIR de estática](https://github.com/rust-lang/rust/pull/116564)
* [sugerir 'into_iter()' cuando se llama al método 'Iterator' en 'impl IntoIterator'](https://github.com/rust-lang/rust/pull/119928)
* [desencadenar 'unsafe_code' lint en las invocaciones de 'global_asm'](https://github.com/rust-lang/rust/pull/121318)
* [Usar cumplimiento en la siguiente coherencia del solucionador de rasgos](https://github.com/rust-lang/rust/pull/121193)
* [miri: implementar intrínsecos x86 AVX](https://github.com/rust-lang/miri/pull/3192)
* [optimizar el manejo de 'delayed_bug'](https://github.com/rust-lang/rust/pull/121015)
* [Optimizar los protectores contra venenos cuando se construye STD con panic=abort](https://github.com/rust-lang/rust/pull/100603)
* [revisión 'Diagnostic' y 'DiagnosticBuilder'](https://github.com/rust-lang/rust/pull/120576)
* [implementar 'Instant' para UEFI](https://github.com/rust-lang/rust/pull/120889)
* [implementar 'Default' para 'AsciiChar'](https://github.com/rust-lang/rust/pull/121024)
* [almacenar 'core::str::CharSearcher::utf8_size' como u8](https://github.com/rust-lang/rust/pull/119808)
* [hacer que 'Archivo::read_to_end' sea menos especial](https://github.com/rust-lang/rust/pull/120538)
* [implementar los rasgos 'NonZero' genéricamente](https://github.com/rust-lang/rust/pull/121241)
* [hacer genérico 'NonZero::get'](https://github.com/rust-lang/rust/pull/120563)
* [hacer que 'io::BorrowedCursor::advance' sea seguro](https://github.com/rust-lang/rust/pull/120741)
* [make 'is_nonoverlapping #[inline]'](https://github.com/rust-lang/rust/pull/121311)
* [especializarse en aplanar iteradores con un solo elemento interno](https://github.com/rust-lang/rust/pull/121204)
* [especializarse en algunos métodos de 'io::Chain'](https://github.com/rust-lang/rust/pull/105917)
* [renombrar 'MaybeUninit::write_slice'](https://github.com/rust-lang/rust/pull/116385)
* [no use 'mem::zeroed' en 'vec::IntoIter'](https://github.com/rust-lang/rust/pull/120952)
* [optimizar 'VecDeque::d rain' para rangos (semi)abiertos](https://github.com/rust-lang/rust/pull/118264)
* [arreglar el 'Cursor::remove_{next,prev}'](https://github.com/rust-lang/rust/pull/120505) de BTreeMap
* [añadir 'Future' y 'IntoFuture' al preludio de 2024](https://github.com/rust-lang/rust/pull/121041)
* [hashbrown: ajustes en línea a 'HashTable'](https://github.com/rust-lang/hashbrown/pull/505)
* [hashbrown: make 'HashSet::insert' return OccupiedEntry](https://github.com/rust-lang/hashbrown/pull/495)
* [codegen\_gcc: maneja correctamente '--use-system-gcc'](https://github.com/rust-lang/rustc_codegen_gcc/pull/429)
* [codegen\_gcc: implementar dummy emit=llvm-ir](https://github.com/rust-lang/rustc_codegen_gcc/pull/437)
* [CodeGen\_gcc: usa la destrucción de Rust predeterminada](https://github.com/rust-lang/rustc_codegen_gcc/pull/440)
* [codegen\_cranelift: arreglar 'simd_select_bitmask' en sistemas big-endian](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1457)
* [CodeGen\_cranelift: Se corrigió la verificación de hash de descarga en sistemas big-endian](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1458)
* [cargo add: Asegúrese de que los usuarios sepan que se está creando una función](https://github.com/rust-lang/cargo/pull/13434)
* [cargo add: Eliminar período inconsistente](https://github.com/rust-lang/cargo/pull/13446)
* [prueba de carga: Sugerir '--' para los argumentos libtest](https://github.com/rust-lang/cargo/pull/13448)
* [cargo: respete 'rust-version' al generar lockfile](https://github.com/rust-lang/cargo/pull/12861)
* [Rustdoc: Maneja correctamente los nombres largos de las cajas en el móvil](https://github.com/rust-lang/rust/pull/120526)
* [Rustdoc: Reexportaciones entre cajas: renderizar correctamente los parámetros enlazados en tiempo de ejecución en orden de origen, incluso si los parámetros enlazados en tiempo de ejecución están presentes](https://github.com/rust-lang/rust/pull/121022)
* [Rustdoc: Se corrigió el manejo de la función 'doc_auto_cfg' para los atributos CFG en la reexportación de globos](https://github.com/rust-lang/rust/pull/120548)
* [rustfmt: corregir error al intentar formatear UTF8 no normalizado](https://github.com/rust-lang/rustfmt/pull/6073)
* [formatear los límites de rasgos 'asíncronos' en rustfmt](https://github.com/rust-lang/rust/pull/121035)
* [añadir clippy a la lista conocida 'cfg'](https://github.com/rust-lang/rust/pull/121137)
* [clippy: añadir nueva pelusa 'deprecated_clippy_cfg_attr'](https://github.com/rust-lang/rust-clippy/pull/12292)
* [clippy: 'case_sensitive_file_extension_comparisons': No se activa en extensiones de solo dígitos](https://github.com/rust-lang/rust-clippy/pull/12293)
* [clippy: 'implied_bounds_in_impls': evitar la pelusa en tys asociados superpuestos](https://github.com/rust-lang/rust-clippy/pull/11881)
* [clippy: 'incompatible_msrv': permite expresiones que provienen de la eliminación de azúcar](https://github.com/rust-lang/rust-clippy/pull/12275)
* [clippy: 'new_without_default': Ahora emite en const fns](https://github.com/rust-lang/rust-clippy/pull/10903)
* [clippy: permitir literales negativos en 'redundant_guards'](https://github.com/rust-lang/rust-clippy/pull/11641)
* [clippy: comprobar los elementos de rasgo en 'min_ident_chars'](https://github.com/rust-lang/rust-clippy/pull/12294)
* [clippy: manejo de conflictos de salida de prueba predeterminado a error](https://github.com/rust-lang/rust-clippy/pull/12314)
* [clippy: asegúrese de que la sintaxis de ASM detecte 'global_asm!' y 'asm!' solo en arquitecturas x86](https://github.com/rust-lang/rust-clippy/pull/12305)
* [clippy: añadir comprobación de protecciones idénticas en lint 'match_same_arms'](https://github.com/rust-lang/rust-clippy/pull/12059)
* [clippy: hacer que '#[permitir]' funcione en el campo para 'pub_underscore_fields'](https://github.com/rust-lang/rust-clippy/pull/12309)
* [clippy: ignorar elementos importados en 'min_ident_chars'](https://github.com/rust-lang/rust-clippy/pull/12285)
* [rust-analyzer: activar en el nivel superior los archivos 'Cargo.toml' y 'rust-project.json'](https://github.com/rust-lang/rust-analyzer/pull/16550)
* [rust-analyzer: deduplicar referencias cuando algunas de ellas están en expansiones de macros](https://github.com/rust-lang/rust-analyzer/pull/16358)
* [rust-analyzer: crear alias al cambiar el nombre de una importación](https://github.com/rust-lang/rust-analyzer/pull/16489)
* [Rust-analyzer: agregue un diagnóstico de permiso no exhaustivo](https://github.com/rust-lang/rust-analyzer/pull/16303)
* [Rust-Analyzer: agregar diagnóstico de identidad no resuelta](https://github.com/rust-lang/rust-analyzer/pull/16589)
* [rust-analyzer: admite múltiples tabulaciones para finalizaciones en VSCode](https://github.com/rust-lang/rust-analyzer/pull/16475)
* [rust-analyzer: se ha añadido soporte básico para las llamadas 'become' expr/tail](https://github.com/rust-lang/rust-analyzer/pull/15003)
* [rust-analyzer: no añadir '\' antes de '{'](https://github.com/rust-lang/rust-analyzer/pull/16618)
* [rust-analyzer: no mostrar discrepancias de tipo para discrepancias '{unknown}' a discrepancias que no son '{desconocidas}'](https://github.com/rust-lang/rust-analyzer/pull/16583)
* [rust-analyzer: corrige el diagnóstico de "devolución innecesaria" para las declaraciones de elementos finales](https://github.com/rust-lang/rust-analyzer/pull/16574)
* [rust-analyzer: arreglar los scripts de compilación que no se reconstruyen en algunas ocasiones](https://github.com/rust-lang/rust-analyzer/pull/16247)
* [rust-analyzer: corrige falsos positivos para el diagnóstico de "otra cosa innecesaria"](https://github.com/rust-lang/rust-analyzer/pull/16590)
* [rust-analyzer: arreglar los fragmentos que se colocan hacia la izquierda de donde deberían estar](https://github.com/rust-lang/rust-analyzer/pull/16579)
* [rust-analyzer: mejora la recuperación en '=' para el inicializador y el patrón del campo de registro](https://github.com/rust-lang/rust-analyzer/pull/16553)
* [rust-analyzer: solo rasgos completos en 'impl .. for'](https://github.com/rust-lang/rust-analyzer/pull/16544)
* [Rust-Analyzer: Colocar fragmentos correctamente en las ayudas de edición múltiple](https://github.com/rust-lang/rust-analyzer/pull/16569)
* [rust-analyzer: servidor que cuelga en la tarea de script de compilación](https://github.com/rust-lang/rust-analyzer/pull/16616)

### Clasificación del rendimiento del compilador de Rust

Relativamente pocos PR que afecten al rendimiento, pero mejoras masivas gracias a la
actualización a LLVM 18 (PR #12005), así como la fusión de dos compiladores relacionados
consultas (PR #120919) y otras pequeñas mejoras de un paquete acumulativo (PR #121055).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [74c3f5a1.. 5af21304](https://perf.rust-lang.org/?start=74c3f5a146860c94ff4d179fc3bfa34f879adf41&end=5af2130440c198afefbe5b8099342057cf272ef4&absolute=false&stat=instructions%3Au)

3 regresiones, 1 mejoras, 6 mixtas; 1 de ellos en rollups
65 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d18e18944c4ab14988ca5219b17530454d133474/triage/2024-02-20.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [eRFC: Iterar y estabilizar la salida programática de libtest](https://github.com/rust-lang/rfcs/pull/3558)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Hacer que Cargo respete la versión mínima admitida de Rust (MSRV) al seleccionar dependencias](https://github.com/rust-lang/rfcs/pull/3537)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Disposición: Fusionar] [Cambiar la equidad para que los aglutinantes no dependan de la subtipificación](https://github.com/rust-lang/rust/pull/118247)
* [disposición: fusionar] [Implementar RFC 3373: Evitar definiciones no locales en funciones](https://github.com/rust-lang/rust/pull/120393)
* [disposición: fusionar] [Problema de seguimiento para 'waker_getters'](https://github.com/rust-lang/rust/issues/96992)
* [disposición: fusionar] [Estabilizar el espacio de nombres '#[diagnóstico]' y el atributo '#[diagnóstico::on_unimplemented]']](https://github.com/rust-lang/rust/pull/119888)
* [disposición: fusionar] [Problema de seguimiento para cfg-target-abi](https://github.com/rust-lang/rust/issues/80970)
* [disposición: fusionar] [hacer que las consts no tipificadas por PartialEq como patrones sean un error grave](https://github.com/rust-lang/rust/pull/120805)
* [disposición: cerrar] [Permitir la conversión de ?-desde 'Resultado<T, E>' en funciones que devuelven 'Opción<Resultado<T, E>>'](https://github.com/rust-lang/rust/pull/99333)
* [disposición: fusionar] [Añadir impl de lectura para &stdin](https://github.com/rust-lang/rust/pull/99153)
* [disposición: fusionar] [Crear 'barrera::nuevo()' const ](https://github.com/rust-lang/rust/pull/119536)
* [disposición: cerrar] [Implementar 'Futuro' para 'Opción<F>'](https://github.com/rust-lang/rust/pull/109691)

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [operador 'is' para la coincidencia de patrones y el enlace](https://github.com/rust-lang/rfcs/pull/3573)

## Próximos eventos

Eventos oxidados entre 2024-02-21 - 2024-03-20 🦀

### Virtual

* 2024-02-21 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 2 - Tipos**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 22/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)
* 27/02/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/299068302/)
* 29/02/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Espejo: Berline.rs página**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 29/02/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfeando las olas inalámbricas oxidadas con la tabla ESP32-C3**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 06/03/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/299047891/)
* 07/03/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 12/03/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341582/)
* 12/03/2024 | Híbrido (Virtual + Presencial) Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
* 14/03/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-21 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 26/03/2024 | Virtual + Presencial (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Transmisión](https://www.youtube.com/@bcnrust)

### Europa

* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 22/02/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)
* 29/02/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Inicio de temporada 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 12/03/2024 | Múnich, DE + Virtual | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
* 19/03/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 20/03/2024 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introducción a la programación de microcontroladores con Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 26/03/2024 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26, 2024-03-28 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation 2024**](https://www.rustnationuk.com/)

### América del Norte

* 2024-02-21 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Reunión nocturna de Boston Rust en Microsoft, 21 de febrero**](https://www.meetup.com/bostonrust/events/299054786/)
* 22/02/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 28/02/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 07/03/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)

### Oceanía

* 27/02/2024 | Canberra, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/297650401/)
* 27/02/2024 | Sídney, Nueva Gales del Sur, Australia | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [** 🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)
* 29/02/2024 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-brisbane/events/299304438/)
* 05/03/2024 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Introducción a Embedded Rust + The State of Rust UI**](https://www.meetup.com/rust-akl/events/299158887/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1arr8xi/official_rrust_whos_hiring_thread_for_jobseekers)

# Frase de la semana

> El estado mutable compartido es malvado, y puedes resolverlo prohibiendo la mutación o prohibiendo compartir. Rust es compatible con ambos.

– [Kornel en Lobste.rs](https://lobste.rs/s/fud3fk/from_1s_4ms#c_relksr)

¡Gracias a [Aleksey Kladov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1535) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1awuo07/this_week_in_rust_535/)</small>
