---
title: "Esta semana en Rust #37"
number_of_week: 37
description: El crate de esta semana es postcard, un serializador/deserializador compatible con '#[no_std]' probado en batalla, bien documentado, orientado a su uso en dispositivos integrados.
date: 2024-11-27
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
* [Convocatoria de pruebas de Rust 2024](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html)

### Boletines
* [El Rustáceo Incrustado Edición #33](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-33)

### Actualizaciones de proyectos/herramientas
* [Anunciando Nio: un tiempo de ejecución asíncrono para Rust](https://nurmohammed840.github.io/posts/announcing-nio/)
* [Anunciando la consulta de Rust](https://blog.lucasholten.com/rust-query-announcement/)
* ["pigg" (la GUI GPIO de Raspberry Pi) versión 0.5.0, ahora con soporte para Pi Pico W a través de rust y embassy](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.5.0)
* [¡Hola *florecer*! - Señales intuitivas en Rust](https://github.com/Tamschi/flourish/discussions/8)
* [¡Ferroceno 24.11.0 con calificación médica y QNX ahora disponible!](https://ferrous-systems.com/blog/ferrocene-24-11-0/)

### Observaciones/Pensamientos
* [¿Una idea novedosa sobre 'Functor' en Rust?](https://wolfgirl.dev/blog/2024-11-24-a-novel-idea-about-functor-in-rust/)
* [Optimización de un kernel matmul de GPU Rust](https://rust-gpu.github.io/blog/optimizing-matmul/)
* [Claves de mapa y duraciones](https://blinsay.com/blog/compound-keys/)
* [Rustlantis: Pruebas diferenciales aleatorias del compilador de Rust](https://plf.inf.ethz.ch/research/oopsla24-rustlantis.html)
* [Tipos que no se pueden soltar](https://jack.wrenn.fyi/blog/undroppable/)
* [Un grupo de hackers de 40 años prefiere Rust](https://blog.rust.careers/post/40y_old_hacking_group_rust_veilid/)
* [Tipos lineales malditos en Rust](https://geo-ant.github.io/blog/2024/rust-linear-types-use-once/)
* [Inseguro para el trabajo](https://oida.dev/unsafe-for-work/)
* [Alineándome con el Principio Rector del Rust: mi camino para encontrar mi 'Por qué'](https://flakm.com/posts/rust_guiding_principle/)

### Tutoriales de Rust
* [video] [Construir con Naz - TLS en Rust con tokio, rustls, CFSSL](https://www.youtube.com/watch?v=NeTZGyc9l7E)
* [Aventuras de optimización: hacer una carga de trabajo paralela de Rust 10 veces más rápida con (o sin) Rayon](https://gendignoux.com/blog/2024/11/18/rust-rayon-optimized.html)
* [Construcción de primitivas asíncronas seguras para subprocesos en 150 líneas de Rust](https://amit.prasad.me/blog/async-oneshot)
* [Envío seguro de datos del sensor DHT22 desde una placa ESP32 a PostgreSQL](https://c410-f3r.github.io/thoughts/securely-sending-dht22-sensor-data-from-an-esp32-board-to-postgresql/)
* [Escribir un GC de trazado de marca y barrido en Rust](https://elric.pl/blog/tracing-gc/)

### Miscelánea
* [Haciendo de Rust un ciudadano de primera clase para Xen](https://xcp-ng.org/blog/2024/11/26/making-rust-a-first-class-citizen-for-xen/)

## Crate de la semana

El crate de esta semana es [postcard](https://docs.rs/postcard), un serializador/deserializador compatible con '#[no_std]' probado en batalla, bien documentado, orientado a su uso en dispositivos integrados.

¡Gracias a [Reto Trappitsch](https://users.rust-lang.org/t/crate-of-the-week/2704/1377) por la sugerencia!

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
### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

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

* [Diesel - Agrega soporte para funciones postgres json/jsonb actualmente no compatibles](https://github.com/diesel-rs/diesel/issues/4216)
* [Diesel - Agregar funciones y operadores sqlite json/jsonb que faltan](https://github.com/diesel-rs/diesel/issues/4366)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

* [Rust en París 2025](https://docs.google.com/forms/d/e/1FAIpQLSdamzdbUi3EIGBrmEw0-Na4myXP0088kvxVmVT4YU-1BEiyCg/viewform) | Fecha límite: 30 de noviembre de 2024 | París, FR | 15 marzo 2025

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

* [FOSDEM 2025 Rust devroom](https://rust-fosdem.github.io) | Fecha límite: 1 de diciembre de 2024 | Bruselas, BE | 1 de febrero de 2025

## Actualizaciones del Proyecto Rust

405 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-19..2024-11-26

* ['lints_that_dont_need_to_run': nunca te saltes las pelusas reportadas en el futuro](https://github.com/rust-lang/rust/pull/133108)
* [aarch64 softfloat target: siempre pasa floats en registros int](https://github.com/rust-lang/rust/pull/133102)
* [tenga en cuenta 'wasm32v1-none' al exportar símbolos TLS](https://github.com/rust-lang/rust/pull/133244)
* [añadir métodos de 'visit' a los nodos AST que ya tienen 'walk's en los visitantes AST](https://github.com/rust-lang/rust/pull/133188)
* [agregar diagnóstico específico para usar la macro 'macro_rules' como atributo/derivar](https://github.com/rust-lang/rust/pull/132949)
* [añadir visitas a nodos que ya tienen 'flat_maps' en 'ast::MutVisitor'](https://github.com/rust-lang/rust/pull/133153)
* [permitir deshabilitar la instrumentación de ASan para globales](https://github.com/rust-lang/rust/pull/127483)
* [Efectos de rescate en el solucionador antiguo si el yo es ty var](https://github.com/rust-lang/rust/pull/133323)
* [btree: no fugar valor si el destructor de llaves falla](https://github.com/rust-lang/rust/pull/132597)
* [constifica también los rasgos 'Deref'/'DerefMut'](https://github.com/rust-lang/rust/pull/133260)
* [continuar la transición de 'ParamEnv' a 'TypingEnv'](https://github.com/rust-lang/rust/pull/133212)
* [corregir la lista de niveles de 'wasm32-wasip2'](https://github.com/rust-lang/rust/pull/133213)
* [default-enable 'llvm_tools_enabled' cuando no hay 'config.toml' presente](https://github.com/rust-lang/rust/pull/133207)
* [distinga el desbordamiento y el no implementado en 'Step::steps_between'](https://github.com/rust-lang/rust/pull/130867)
* [no permitir que '-Zunstable-options' tome un valor](https://github.com/rust-lang/rust/pull/133159)
* [elimine la información de depuración en lugar de entrar en pánico si excedemos la capacidad de LLVM para representarlo](https://github.com/rust-lang/rust/pull/133194)
* [emscripten: enlace con '-sWASM_BIGINT'](https://github.com/rust-lang/rust/pull/131736)
* [arreglar el triple objetivo de LLVM para 'x86_64-win7-windows-msvc'](https://github.com/rust-lang/rust/pull/133239)
* [arreglar ASM goto con salidas y moverlo a una puerta de características separada](https://github.com/rust-lang/rust/pull/131523)
* [arreglar la extracción de arg de cierre en 'extract_callable_info', generalizarlo a cierres asíncronos](https://github.com/rust-lang/rust/pull/132489)
* [implementar el objetivo del efecto '~const Destruct' en el nuevo solucionador](https://github.com/rust-lang/rust/pull/132329)
* [implementar el objetivo de rasgo '~const Fn' en el nuevo solucionador](https://github.com/rust-lang/rust/pull/133216)
* [implementar los límites de elementos '~const' en RPIT](https://github.com/rust-lang/rust/pull/133218)
* [implementar el RFC de campos inseguros](https://github.com/rust-lang/rust/pull/132915)
* [hacer que 'PointerLike' sea opcional en lugar de incorporado](https://github.com/rust-lang/rust/pull/133226)
* [Hacer que los bloques de etiquetas ASM sean un contexto seguro](https://github.com/rust-lang/rust/pull/131544)
* [fusionar '-Zhir-stats' en '-Zinput-stats'](https://github.com/rust-lang/rust/pull/133023)
* [señale la definición 'const' cuando se usa en lugar de un enlace en una declaración 'let'](https://github.com/rust-lang/rust/pull/132708)
* [Bonito estampado asíncrono fn azúcar en opacos y límites de rasgos](https://github.com/rust-lang/rust/pull/132911)
* [reducir los falsos positivos de tail-expr-drop-order de los valores consumidos (intento #2)](https://github.com/rust-lang/rust/pull/131326)
* [refactorizar los predicados 'donde' y reservar para el soporte de atributos](https://github.com/rust-lang/rust/pull/132894)
* [eliminar 'is_trivially_const_drop'](https://github.com/rust-lang/rust/pull/133371)
* [eliminar código de bits heredado para iOS](https://github.com/rust-lang/rust/pull/133297)
* [reportar la pelusa 'unexpected_cfgs' en macros externas](https://github.com/rust-lang/rust/pull/132577)
* [rustc: falla rápido al compilar un archivo fuente de más de 4 GiB](https://github.com/rust-lang/rust/pull/132791)
* [mostrar pelusa 'abi_unsupported_vector_types' en futuros informes de rotura](https://github.com/rust-lang/rust/pull/133374)
* [Dejen de ser tan bailables en la asamblea de candidatos](https://github.com/rust-lang/rust/pull/132090)
* [Resolución de almacenamiento para segmentos de módulo propio y de raíz de caja](https://github.com/rust-lang/rust/pull/132207)
* [Métricas de uso de características inestables](https://github.com/rust-lang/rust/pull/130236)
* [use 'ConstArgKind::P ath' para todas las rutas de un solo segmento, no solo los parámetros bajo 'min_generic_const_args'](https://github.com/rust-lang/rust/pull/131081)
* [use 'confstr(_CS_DARWIN_USER_TEMP_DIR, ...)' como un 'TMPDIR' de retroceso a Darwin](https://github.com/rust-lang/rust/pull/131505)
* [use arc4random de libc para el objetivo RTEMS](https://github.com/rust-lang/rust/pull/133313)
* [usar atributos para 'dangling_pointers_from_temporaries' lint](https://github.com/rust-lang/rust/pull/132732)
* [interpretar: no hacer ICE cuando un promocionado falla con OOM](https://github.com/rust-lang/rust/pull/133164)
* [miri: añadido epoll y eventfd para Android](https://github.com/rust-lang/miri/pull/4016)
* [Miri: eventfd: Ajustes de comentarios](https://github.com/rust-lang/miri/pull/4047)
* [Miri: Llenar la tabla de mapeo de errores de E/S de Windows](https://github.com/rust-lang/miri/pull/4046)
* [Miri: seguimiento de #4052, haciendo un contexto de evaluación de Miri fn para 'strerror_r'](https://github.com/rust-lang/miri/pull/4054)
* [Miri: Implementa la raíz cuadrada sin depender de los flotantes del host](https://github.com/rust-lang/miri/pull/4026)
* [miri: refactorizar 'AnonSocket::read/write' para bloquear socketpair](https://github.com/rust-lang/miri/pull/4037)
* [Miri: Simplificar las pruebas de bloqueo de hilos](https://github.com/rust-lang/miri/pull/4059)
* [Miri: sysconf añadiendo algunas constantes más](https://github.com/rust-lang/miri/pull/4053)
* [Miri: Corrección de intercepción de sysconf para sistemas solarish](https://github.com/rust-lang/miri/pull/4052)
* [miri: vitrina de trofeos: añadir 'RwLock::d owngrade' bug](https://github.com/rust-lang/miri/pull/4042)
* [miri: usa las APIs de 'PathBuf' para hacer correctamente alguna manipulación de rutas multiplataforma](https://github.com/rust-lang/miri/pull/4061)
* [Resolver ajustes](https://github.com/rust-lang/rust/pull/132761)
* [finalizar la eliminación de 'Revelar'](https://github.com/rust-lang/rust/pull/133242)
* [estabilizar la edición de 2024](https://github.com/rust-lang/rust/pull/133349)
* [estabilizar 'Ipv6Addr::is_unique_local' e 'Ipv6Addr::is_unicast_link_local'](https://github.com/rust-lang/rust/pull/129238)
* [estabilizar 'const_float_methods'](https://github.com/rust-lang/rust/pull/133389)
* [estabilizar 'const_pin_2'](https://github.com/rust-lang/rust/pull/131904)
* [constificar mínimamente 'Agregar'](https://github.com/rust-lang/rust/pull/133237)
* [marque '<[T; N]>::as_mut_slice' con el especificador 'const'](https://github.com/rust-lang/rust/pull/133332)
* [marque 'get_mut' y 'set_position' en 'std::io::Cursor' como const](https://github.com/rust-lang/rust/pull/130800)
* [reducir el tamaño de implementación de la 'Pantalla' entera](https://github.com/rust-lang/rust/pull/133247)
* [std: permitir el uso posterior a la principal de primitivas de sincronización](https://github.com/rust-lang/rust/pull/132730)
* [implement 'OsString::truncate'](https://github.com/rust-lang/rust/pull/133264)
* [añadir 'AsyncFn*' al preludio en todas las ediciones](https://github.com/rust-lang/rust/pull/132611)
* [añadir los métodos 'BorrowedBuf::into_filled{,_mut}' para permitir el retorno del búfer con la vida útil original](https://github.com/rust-lang/rust/pull/132533)
* [add 'std::thread::add_spawn_hook'](https://github.com/rust-lang/rust/pull/125405)
* [añadir 'vec_deque::Iter::as_slices' y amigos](https://github.com/rust-lang/rust/pull/123947)
* [soporte 'each_ref' y 'each_mut' en '[T; N]' en expresiones constantes](https://github.com/rust-lang/rust/pull/133288)
* [Soporte de entrada/salida en registros vectoriales del ensamblaje en línea S390X (bajo la función 'asm_experimental_reg')](https://github.com/rust-lang/rust/pull/131664)
* [soporte s390x z13 vector ABI](https://github.com/rust-lang/rust/pull/131586)
* [UEFI: proceso: agregar soporte para args](https://github.com/rust-lang/rust/pull/129838)
* [hashbrown: versión v0.15.2](https://github.com/rust-lang/hashbrown/pull/587)
* [cargo: 'test(rustflags)': Verifica -Cmetadata directamente, no a través de -Cextra-filename](https://github.com/rust-lang/cargo/pull/14846)
* [cargo: permitir que los registros omitan campos vacíos/predeterminados en JSON](https://github.com/rust-lang/cargo/pull/14838)
* [cargo: verifique que el objetivo de compilación admita std cuando se compila con -Zbuild-std=std](https://github.com/rust-lang/cargo/pull/14183)
* [cargo: documentos para campos JSON de registro opcionales](https://github.com/rust-lang/cargo/pull/14839)
* [cargo: feat: estabilizar Edición 2024](https://github.com/rust-lang/cargo/pull/14828)
* [cargo: mejorar el manejo de errores cuando PathSource es relativo](https://github.com/rust-lang/cargo/pull/14854)
* [cargo: test: address test output nondeterminism](https://github.com/rust-lang/cargo/pull/14855)
* [Cargo: prueba: cambiar de ''exec_with_output'' a 'correr'](https://github.com/rust-lang/cargo/pull/14848)
* [rustdoc: no llames a 'to_string', ya es impl Display](https://github.com/rust-lang/rust/pull/133398)
* [bindgen: añadir la función 'raw_ref_macros'](https://github.com/rust-lang/rust-bindgen/pull/2988)
* [clippy: añadir nueva pelusa 'doc_include_without_cfg'](https://github.com/rust-lang/rust-clippy/pull/13625)
* [clippy: añadir nota sobre la advertencia para 'cfg(doc)'](https://github.com/rust-lang/rust-clippy/pull/13724)
* [clippy: no considerar los tiempos de vida en los tipos acotados sin usar (arreglar 'extra_unused_lifetimes' FP)](https://github.com/rust-lang/rust-clippy/pull/13583)
* [clippy: sincronización y automatización de lanzamientos](https://github.com/rust-lang/rust-clippy/pull/13694)
* [clippy: usa un mejor mensaje para 'unnecessary_map_or' lint](https://github.com/rust-lang/rust-clippy/pull/13708)
* [rust-analyzer: convertir 'add_braces' a la abstracción de SyntaxFactory SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/18485)
* [Rust-analyzer: use marcadores de posición de fragmentos para los brazos de coincidencia generados](https://github.com/rust-lang/rust-analyzer/pull/18459)
* [Rust-analyzer: Arregla un desbordamiento de pila al calcular el tamaño de una 'estructura' que se incluye a sí misma como el campo de cola](https://github.com/rust-lang/rust-analyzer/pull/18559)
* [Rust-analyzer: Mejorar el manejo de selección para la asistencia 'merge_match_arms'](https://github.com/rust-lang/rust-analyzer/pull/18529)
* [rust-analyzer: migrar 'reorder_impl_items' Assist para usar 'SyntaxFactory'](https://github.com/rust-lang/rust-analyzer/pull/18521)

### Clasificación del rendimiento del compilador de Rust

Esta semana se observaron más regresiones que mejoras, principalmente debido a tres PR que realizaron resultados internos
refactorizaciones que son necesarias para el desarrollo y la modificación posteriores del compilador.

Triaje realizado por **@kobzol**.
Rango de revisión: [7d40450b.. 7db7489f](https://perf.rust-lang.org/?start=7d40450b2df92bdc9dec414b30cf5f7a5979a92e&end=7db7489f9bc274cb60c4956bfa56de0185eb1b9b&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.6% | [0.1%, 3.6%] | 57 |
| Regresiones ❌ <br /> (secundaria) | 0.6% | [0.0%, 2.7%] | 100 |
| Mejoras ✅ <br /> (primario) | -0,5% | [-1.5%, -0.2%] | 11 |
| Mejoras ✅ <br /> (secundaria) | -0,4% | [-0.5%, -0.3%] | 7 |
| Todos ❌✅ (primarios) | 0.4% | [-1,5%, 3,6%] | 68 |

4 regresiones, 2 mejoras, 3 mixtas; 3 de ellos en rollups
40 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/556db980efa8c8553fe92ce64f04db372b0c7d61/triage/2024-11-26.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [[RFC] ABI explícito en extern](https://github.com/rust-lang/rfcs/pull/3722)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para 'sub_ptr' (característica 'ptr_sub_ptr')](https://github.com/rust-lang/rust/issues/95892)
* [disposición: fusionar] [Estabilizar variantes sin signo y flotantes de la función 'num_midpoint'](https://github.com/rust-lang/rust/pull/131784)
* [disposición: fusionar] [Problema de seguimiento para Vec::extract_if y LinkedList::extract_if](https://github.com/rust-lang/rust/issues/43244)
* [disposición: fusionar] [Estabilizar noop_waker](https://github.com/rust-lang/rust/pull/133089)
* [disposición: fusionar] [Estabilizar const_maybe_uninit_write](https://github.com/rust-lang/rust/pull/131713)
* [disposición: fusionar] [Problema de seguimiento para la ruta::file_prefix](https://github.com/rust-lang/rust/issues/86319)
* [disposición: fusionar] [Estabilizar '#[diagnóstico::d o_no_recomendar]'](https://github.com/rust-lang/rust/pull/132056)
* [disposición: fusionar] [Hacer que missing_abi lint advierta por defecto.](https://github.com/rust-lang/rust/pull/132397)
* [disposición: fusionar] [Problema de seguimiento para ptr::fn_addr_eq](https://github.com/rust-lang/rust/issues/129322)
* [disposición: fusionar] [Arreglar y dejar de usar home_dir()](https://github.com/rust-lang/rust/pull/132515)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [new] [crates.io: Eliminar las eliminaciones de versiones del RFC de "eliminaciones de cajas"](https://github.com/rust-lang/rfcs/pull/3731)
* [nuevo] [RFC: Agregar un nivel de lint semánticamente no bloqueante](https://github.com/rust-lang/rfcs/pull/3730)

## Próximos eventos

Eventos oxidados entre 2024-11-27 - 2024-12-25 🦀

### Virtual
* 28/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 28/11/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 29/11/2024 | Virtual (Jersey City, NJ, EE. UU.)| [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304477903/)
* 02/12/2024 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Advenimiento del Código - ¡Kick-Off!**](https://www.meetup.com/women-in-rust/events/304668776/)
* 03/12/2024 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 03/12/2024 | Virtual (San Francisco, CA, EE. UU.) | [Centro Blockchain SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust en Web3: Serie para desarrolladores**](https://www.meetup.com/blockchaincentersf/events/304510595/)
* 04/12/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031652)
* 05/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633275/)
* 05/12/2024 | Virtual (Miami, FL) | [Eventos del grupo de usuarios de Java en Miami](https://www.meetup.com/miami-java-user-group)
    * [**Introducción a Rust para desarrolladores de Java**](https://www.meetup.com/miami-java-user-group/events/304717903/)
* 06/12/2024 | Virtual (Jersey City, NJ, EE. UU.)| [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304369723/)
* 07/12/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 08/12/2024 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Leyendo archivos JSON en Rust - קריאת קבצי ג'ייסון בראסט**](https://www.meetup.com/rust-tlv/events/304685546/)
* 2024-12-10 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346988/)
* 11/12/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 12/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 12/12/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Reunión de Rust en Seattle](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Encuentro de diciembre**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 13/12/2024 | Virtual (Jersey City, NJ, EE. UU.)| [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 17/12/2024 | Virtual (San Francisco, CA, EE. UU.) | [Centro Blockchain SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust en Web3: Serie para desarrolladores**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 17/12/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346972/)
* 19/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633276/)
* 19/12/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 20/12/2024 | Virtual (Jersey City, NJ, EE. UU.)| [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 24/12/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)

### África
* 2024-12-10 | Johannesburgo, ZA | [Reunión de Rust en Johannesburgo](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hola Mundo... otra vez**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)

### Asia
* 28/11/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Cumbre RustTechX 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 30/11/2024 | Tokio, JP | [Rust de Tokio](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)
* 03/12/2024 | Ra'anana, IL | [Reuniones de investigación y desarrollo de Abra](https://www.meetup.com/abra-rnd-solutions/)
    * [**Rust en el kernel de Linux**](https://www.meetup.com/abra-rnd-solutions/events/304302596/)

### Europa
* 27/11/2024 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 28/11/2024 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 28/11/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 28/11/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421381/)
* 28/11/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #53 patrocinado por Microsoft**](https://www.meetup.com/copenhagen-rust-community/events/304608747/)
* 28/11/2024 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 28/11/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn con Mainmatter y Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 28/11/2024 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/304556866/)
* 28/11/2024 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (noviembre de 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 30/11/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #7**](https://www.meetup.com/stockholm-rust/events/304722627/)
* 03/12/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advenimiento del Código**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 04/12/2024 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprenem junts Rust / Aprendan Rust Together**](https://lu.ma/pypwr0m7)
* 04/12/2024 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en diciembre: Advenimiento del código**](https://www.meetup.com/rustcologne/events/304760521/)
* 04/12/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 04/12/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #73**](https://www.meetup.com/rust-paris/events/304685955/)
* 05/12/2024 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #6**](https://www.meetup.com/rust-gdansk/events/304773705/)
* 05/12/2024 | Zlin, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Encuentro de Rust Moravia (diciembre de 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 06/12/2024 | Moscú, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Rusia**](https://rustcon.ru/)
* 11/12/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 12/12/2024 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 17/12/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tipos, rasgos y mejores prácticas**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### América del Norte
* 27/11/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 05/12/2024 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Mensual de Diciembre**](https://www.meetup.com/rust-montreal/events/304778367/)
* 05/12/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cuerdas de Rust**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 12/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/)
    * [**Encuentro de diciembre**](https://www.meetup.com/join-srug/events/304806455/)
* 16/12/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 17/12/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 23/12/2024 | Ferndale, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcqbfc/)

### Oceanía
* 04/12/2024 | Sídney, Australia | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
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

> Nunca dejaré de sorprenderme positivamente por clippy
>
> '''texto
> error: la hipotenusa se puede calcular con mayor precisión:
> --> src/main.rs:835:5
>     |
> 835 |     (ancho * ancho + alto * alto).sqrt() / diag
>     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ayuda: considere usar 'width.hypot(height)'
>     |
> ayuda: Para obtener más información, visite https://rust-lang.github.io/rust-clippy/master/index.html#imprecise_flops
> ```

- [Manos Pitsidianakis (y rust-clippy) en Mastodon](https://chaos.social/@epilys/113538172289599584)

llogiq se aprecia bastante a sí mismo con respecto a [su sugerencia](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1633).

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1h1o513/this_week_in_rust_575/)</small>
