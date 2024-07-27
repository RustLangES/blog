---
title: "Esta semana en Rust #28"
number_of_week: 28
description: El crate de esta semana es diatomic-waker, una biblioteca sin bloqueo por espín para la activación de tareas asíncronas.
date: 2024-07-24
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
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Fundación
* [Programa de Becas de la Fundación Rust 2024](https://foundation.rust-lang.org/grants/fellowships/)

### Boletines informativos
* [thisweekinbevy - Bevy Jam 5, Radiance Cascades, and Calculators in many ui kits](https://thisweekinbevy.com/issue/2024-07-22-bevy-jam-5-radiance-cascades-and-calculators-in-many-ui-kits)

### Actualizaciones de proyectos/herramientas
* [Slint 1.7 lanzado con nuevos widgets, soporte para múltiples ventanas y rediseño de vista previa en vivo](https://slint.dev/blog/slint-1.7-released)
* [Diésel asíncrono 0.5](https://blog.weiznich.de/blog/diesel-async-0-5/)
* [iroh 0.21.0 - Arreglar, limpiar y pulir](https://iroh.computer/blog/iroh-0-21-fix-clean-polish)
* [gitoxide [junio de 2024]](https://github.com/Byron/gitoxide/discussions/1459)
* [Tutorial de Iced v0.12 - Acciones asíncronas con comandos](https://leafheap.com/articles/iced-v0-12-tutorial-asynchronous-actions-with-commands)
* [Query.rs - Un motor de búsqueda para Rust](https://query.rs/)

### Observaciones/Pensamientos
* [sin.botes - Pin](https://without.boats/blog/pin/)
* [sin.barcos - Lugares anclados](https://without.boats/blog/pinned-places/)
* [RocksDB: No es una buena opción para una plataforma de streaming de alto rendimiento](https://www.feldera.com/blog/rocksdb-not-a-good-choice-for-high-performance-streaming/)
* [Un sistema de tipos para RCL: Implementando un corrector de tipos en Rust](https://ruudvanasseldonk.com/2024/implementing-a-typechecker-for-rcl-in-rust)
* [Venciendo al compilador](https://www.mattkeeter.com/blog/2024-07-12-interpreter/)
* [Deconstruyendo el videojuego de rol](https://olano.dev/blog/deconstructing-the-role-playing-videogame/)
* [Implementación de WebSockets](https://www.thespatula.io/rust/rust_websocket/)
* [Argumentos con nombre en Rust, si los quieres](https://rtpg.co/2024/07/22/rust-named-arguments/)
* [WebAssembly en el servidor: Compilando Rust a WASM y ejecutándolo desde Go](https://blog.arcjet.com/webassembly-on-the-server-compiling-rust-to-wasm-and-executing-it-from-go/)
* [Async Rust: ¿El nuevo error de mil millones de dólares?](https://kerkour.com/rust-async-billion-dollar-mistake)
* [Nine Rust Cargo.toml Wats and Wat Nots: Domine las reglas de formato de Cargo.toml y evite la frustración](https://towardsdatascience.com/nine-rust-cargo-toml-wats-and-wat-nots-1e5e02e41648)

### Tutoriales de Rust
* [Plugins con Rust y WASI Preview 2](https://benw.is/posts/plugins-with-rust-and-wasi)
* [Construya su propio SQLite, Parte 1: Listado de tablas](https://blog.sylver.dev/build-your-own-sqlite-part-1-listing-tables)

### Miscelánea
* [Búsqueda de edificios en Rust](https://filtra.io/rust-meilisearch-jul-24)
* [Las charlas de Oxidize 2024 ya están disponibles en YouTube](https://www.youtube.com/playlist?list=PLilpJp3WAOveS7dwg0YNvSTfkRt9hDHrS)

## Crate de la semana

El crate de esta semana es [diatomic-waker](https://crates.io/crates/diatomic-waker), una biblioteca sin bloqueo por espín para la activación de tareas asíncronas.

¡Gracias a [Ddystopia](https://users.rust-lang.org/t/crate-of-the-week/2704/1323) por la sugerencia!

[Por favor, envíe sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la prueba
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (Anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

402 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-16..2024-07-23

* [promover el objetivo 'wasm32-wasip2' al Nivel 2](https://github.com/rust-lang/rust/pull/126967)
* [CFI: métodos proporcionados de apoyo sobre rasgos](https://github.com/rust-lang/rust/pull/127295)
* [Edificio MIR: Deja de usar 'unpack!' para 'BlockAnd<()>'](https://github.com/rust-lang/rust/pull/127472)
* ['C-cmse-nonsecure-call': mensajes de error mejorados](https://github.com/rust-lang/rust/pull/127814)
* ['macro_metavar_expr_concat' Agregar soporte para literales](https://github.com/rust-lang/rust/pull/127542)
* ['macro_metavar_expr_concat' Permitir 'concate' en repeticiones](https://github.com/rust-lang/rust/pull/127720)
* [agregar soporte de proceso para UEFI](https://github.com/rust-lang/rust/pull/123196)
* [añadir los que faltan 'try_new_uninit_slice_in' y 'try_new_zeroed_slice_in'](https://github.com/rust-lang/rust/pull/127415)
* [cambiar 'binary_asm_labels' para disparar solo en x86 y 'x86_64'](https://github.com/rust-lang/rust/pull/127935)
* [Limpiar cálculos de nombre de archivo dll/exe en 'run_make_support'](https://github.com/rust-lang/rust/pull/127960)
* [construir condicionalmente 'wasm-component-ld'](https://github.com/rust-lang/rust/pull/127866)
* [tratar con UTF-8 inválido de 'gai_strerror'](https://github.com/rust-lang/rust/pull/127583)
* [delegación: coerción de apoyo para la expresión objetivo](https://github.com/rust-lang/rust/pull/126699)
* [denegar la vida útil de las palabras clave antes de la expansión](https://github.com/rust-lang/rust/pull/126762)
* [no permitir estáticas inseguras fuera de los bloques externos](https://github.com/rust-lang/rust/pull/127943)
* [no generar artefactos de prueba incrementales en el directorio de trabajo](https://github.com/rust-lang/rust/pull/128038)
* [no use características implícitas en 'Cargo.toml' en 'compiler/'](https://github.com/rust-lang/rust/pull/127769)
* [arreglar ICE en sugerencia causada por '==' siendo recuperado como '=='](https://github.com/rust-lang/rust/pull/127835)
* [arreglar un montón de sitios que estaban caminando en lugar de visitar, haciendo imposible que los implicados de los visitantes miren estos valores](https://github.com/rust-lang/rust/pull/127817)
* [arreglar casos ambiguos de múltiples & en vidas propias elididas](https://github.com/rust-lang/rust/pull/117967)
* [arreglar y reforzar 'unsafe_op_in_unsafe_fn' en el compilador](https://github.com/rust-lang/rust/pull/127730)
* [arreglar sugerencia de eliminación de elemento asociado](https://github.com/rust-lang/rust/pull/127878)
* [se corrige la sugerencia de captura precisa para regiones ocultas cuando tenemos APITs](https://github.com/rust-lang/rust/pull/127664)
* [soluciona el problema de sugerencia inválida para una referencia de iterador](https://github.com/rust-lang/rust/pull/127669)
* [corrige el error de pánico 'índice fuera de límites' en un error conflictivo](https://github.com/rust-lang/rust/pull/127948)
* [prohibir que los préstamos y los tipos sin tamaño se utilicen como el tipo de un genérico const bajo 'adt_const_params'](https://github.com/rust-lang/rust/pull/127722)
* [interpretar: añadir comprobación de cordura en dyn upcast para comprobar lo que hace CodeGen](https://github.com/rust-lang/rust/pull/127856)
* [invertir inferir 'error_reporting' estructura de mod](https://github.com/rust-lang/rust/pull/127501)
* [simplemente negar por completo las consts de enlace tardío](https://github.com/rust-lang/rust/pull/128020)
* [alias de tipo perezoso: diagnóstico: Detecta parámetros bivariantes que solo se usan recursivamente](https://github.com/rust-lang/rust/pull/127976)
* [mantener la orden dada en la ejecución del paso](https://github.com/rust-lang/rust/pull/127602)
* [hacer que ErrorGuaranteed sea detectable fuera de los tipos, consts y tiempos de vida](https://github.com/rust-lang/rust/pull/127808)
* [hacer que 'pub_use_of_private_extern_crate' aparezca en los futuros informes de rotura de la carga](https://github.com/rust-lang/rust/pull/127656)
* [match lowering: Usa un iterador para encontrar 'expand_until'](https://github.com/rust-lang/rust/pull/127707)
* [intervalo preciso de sugerencia de cambio de nombre de 'uso'](https://github.com/rust-lang/rust/pull/127886)
* [intervalo más preciso para sugerencia de argumento anónimo](https://github.com/rust-lang/rust/pull/127889)
* [intervalo más preciso para la sugerencia de parámetro de tipo](https://github.com/rust-lang/rust/pull/127888)
* [use un intervalo más preciso para la sugerencia 'addr_of!](https://github.com/rust-lang/rust/pull/127929)
* [sugerencia más precisa para '-> Box<dyn Trait>' o '-> impl Trait'](https://github.com/rust-lang/rust/pull/127987)
* [analizador: sugiere colocar el tipo de retorno después de los parámetros de la función](https://github.com/rust-lang/rust/pull/127350)
* [Aplicar de forma segura los requisitos de nombre de hilo](https://github.com/rust-lang/rust/pull/127918)
* [resuelve un error de sugerencia '.clone()' al mover una referencia mutable](https://github.com/rust-lang/rust/pull/127579)
* [sugerir un préstamo cuando se usa dbg](https://github.com/rust-lang/rust/pull/120990)
* [Sugerencias de ajustes cuando se usa un tipo incorrecto de literal 'enum'](https://github.com/rust-lang/rust/pull/127891)
* [usar número ordinal en el argumento error](https://github.com/rust-lang/rust/pull/125042)
* [Cuando encuentre un objeto cerrado detrás de una bandera 'CFG', apúntelo](https://github.com/rust-lang/rust/pull/127662)
* [miri: añadir soporte para la bandera 'O_NOFOLLOW'](https://github.com/rust-lang/miri/pull/3744)
* [Miri: añadir cuñas 'pread' y 'pwrite'](https://github.com/rust-lang/miri/pull/3743)
* [eliminar la clasificación de implicaciones innecesarias en consultas y metadatos](https://github.com/rust-lang/rust/pull/120812)
* [algunas mejoras en el analizador](https://github.com/rust-lang/rust/pull/127806)
* [Corregir dígitos menos significativos de las constantes asociadas a F128](https://github.com/rust-lang/rust/pull/127047)
* [std: use 'read_unaligned' para lecturas de DWARF](https://github.com/rust-lang/rust/pull/127792)
* ['impl Send + Sync' y anula 'count' para el iterador 'CStr::bytes'](https://github.com/rust-lang/rust/pull/127444)
* ['ptr::metadata': evitar referencias a tipos extern](https://github.com/rust-lang/rust/pull/127859)
* [añadir 'isqrt' a 'NonZero<uN>'](https://github.com/rust-lang/rust/pull/126199)
* [use ThreadId en lugar de la dirección TLS en 'ReentrantLock'](https://github.com/rust-lang/rust/pull/124881)
* [use el discriminante de la opción como sugerencia de tamaño](https://github.com/rust-lang/rust/pull/127748)
* [usar futex.rs para el estacionamiento de subprocesos de Windows](https://github.com/rust-lang/rust/pull/127807)
* [windows: use la implementación de futex para 'Once'](https://github.com/rust-lang/rust/pull/125942)
* [Windows: Evitar la doble referencia en el futex genérico](https://github.com/rust-lang/rust/pull/127813)
* [Comience a usar '#[diagnostic::d o_not_recommend]' en la biblioteca estándar](https://github.com/rust-lang/rust/pull/128008)
* [Omitir la ruta rápida para dec2flt cuando 'optimize_for_size'](https://github.com/rust-lang/rust/pull/126271)
* [cargo toml: Mejorar el error al faltar el paquete y el espacio de trabajo](https://github.com/rust-lang/cargo/pull/14261)
* [cargo: add 'TomlPackage::new', 'Default' for 'TomlWorkspace'](https://github.com/rust-lang/cargo/pull/14271)
* [cargo: Se corrige el paso de links-overrides con target-applys-to-host y un target implícito](https://github.com/rust-lang/cargo/pull/14205)
* [Rustdoc: Haga clic en el objetivo de los elementos de la barra lateral al ras de la izquierda](https://github.com/rust-lang/rust/pull/127229)
* [rustdoc: arreglar la clase 'actual' en la barra lateral modnav](https://github.com/rust-lang/rust/pull/127932)
* [rustdoc: las descripciones cortas causan saltos de palabras en las tablas](https://github.com/rust-lang/rust/pull/128023)
* [Agregar soporte de captura precisa de caja cruzada a Rustdoc](https://github.com/rust-lang/rust/pull/127658)
* [rustfmt: impl 'rewrite_result' para ArmWrapper](https://github.com/rust-lang/rustfmt/pull/6239)
* [rustfmt: return RewriteResult para 'rewrite_path' y 'rewrite_struct_***'](https://github.com/rust-lang/rustfmt/pull/6236)
* [clippy: 'pathbuf_init_then_push': Comprueba si hay llamadas para 'empujar' inmediatamente a...](https://github.com/rust-lang/rust-clippy/pull/11700)
* [clippy: añadir pelusa para la recreación de una 'estructura' completa](https://github.com/rust-lang/rust-clippy/pull/12772)
* [clippy: crea pases de lint usando 'Conf'](https://github.com/rust-lang/rust-clippy/pull/13088)
* [clippy: arreglar sugerencias de 'excessive_precision' en flotadores escritos en notación científica](https://github.com/rust-lang/rust-clippy/pull/13096)
* [clippy: se corrige la sugerencia incorrecta para 'single_element_loop' donde faltaban paréntesis](https://github.com/rust-lang/rust-clippy/pull/13117)
* [clippy: lint 'zero_repeat_side_effects' solo si la longitud de la matriz es un cero literal](https://github.com/rust-lang/rust-clippy/pull/13116)
* [Rust-analyzer: Añadir los casos especiales de análisis de Dyn que faltan en la edición 2015](https://github.com/rust-lang/rust-analyzer/pull/17646)
* [rust-analyzer: permitir expansiones de macros en 'RestPat' en argumentos de tupla funcionan como puntos suspensivos como 'RestPat' simple](https://github.com/rust-lang/rust-analyzer/pull/17586)
* [rust-analyzer: evite ref cuando use format! en el compilador](https://github.com/rust-lang/rust-analyzer/pull/17641)
* [Rust-analyzer: Agregue soporte de sugerencia de incrustación para Block expr con etiqueta de por vida](https://github.com/rust-lang/rust-analyzer/pull/17635)
* [Rust-Analyzer: Analizador de edición consciente](https://github.com/rust-lang/rust-analyzer/pull/17620)
* [rust-analyzer: go-to-def y find-references en palabras clave de flujo de control](https://github.com/rust-lang/rust-analyzer/pull/17542)
* [Rust-Analyzer: Característica: enseñar a Rust-Analyzer a descubrir 'linked_projects'](https://github.com/rust-lang/rust-analyzer/pull/17246)
* [rust-analyzer: arreglar los valores predeterminados incorrectos de la sugerencia de parámetros genéricos](https://github.com/rust-lang/rust-analyzer/pull/17616)
* [rust-analyzer: arreglar la resolución de la ruta para los mods hijos de los expandidos por 'include!'](https://github.com/rust-lang/rust-analyzer/pull/17650)
* [rust-analyzer: permitir que flyimport importe módulos de sombreado primitivos](https://github.com/rust-lang/rust-analyzer/pull/17656)
* [rust-analyzer: no llames a 'macro_arg' directamente en 'ExpandDatabase::syntax_context'](https://github.com/rust-lang/rust-analyzer/pull/17611)
* [rust-analyzer: arreglar más resolución de ruta para los submódulos incluidos](https://github.com/rust-lang/rust-analyzer/pull/17660)
* [rust-analyzer: maneja las importaciones sinónimas con diferentes cambios de nombre en 'fusionar importaciones'](https://github.com/rust-lang/rust-analyzer/pull/17622)
* [rust-analyzer: pánico en el perfil de depuración para la deconstrucción de tuplas con desajuste de aridad](https://github.com/rust-lang/rust-analyzer/pull/17649)
* [rust-analyzer: eliminar las invocaciones incorrectas de nunca](https://github.com/rust-lang/rust-analyzer/pull/17668)
* [Rust-Analyzer: más mejoras de 'find_path'](https://github.com/rust-lang/rust-analyzer/pull/17655)
* [Rust-analyzer: más uso de símbolos](https://github.com/rust-lang/rust-analyzer/pull/17604)
* [rust-analyzer: analizar correctamente la palabra clave dyn contextual en la edición 2015](https://github.com/rust-lang/rust-analyzer/pull/17640)
* [Rust-analyzer: reduce el uso de memoria de las ranuras Salsa en 8 bytes](https://github.com/rust-lang/rust-analyzer/pull/17638)
* [rust-analyzer: prefiera las rutas de biblioteca estándar sobre las reexportaciones de datos externos más cortas](https://github.com/rust-lang/rust-analyzer/pull/17653)
* [rust-analyzer: establecer 'RUSTC_TOOLCHAIN' para runnables](https://github.com/rust-lang/rust-analyzer/pull/17605)
* [Rust-analyzer: algunas pequeñas mejoras más en la memoria de la salsa](https://github.com/rust-lang/rust-analyzer/pull/17639)
* [rust-analyzer: soporte 'rustc_skip_during_method_dispatch'](https://github.com/rust-lang/rust-analyzer/pull/17618)
* [rust-analyzer: cambiar los árboles de tokens para usar símbolos](https://github.com/rust-lang/rust-analyzer/pull/17603)

### Clasificación del rendimiento del compilador de Rust

Semana de la luz, con pocos cambios en cualquier dimensión (incluida la memoria), eso sí.
En general, una ligera regresión.

Triaje realizado por **@simulacrum**.
Rango de revisión: [5572759b.. 9629b90b](https://perf.rust-lang.org/?start=5572759b8d7012fa34eba47f4885c76fa06d9251&end=9629b90b3f7dd8f5626ec9d3b42556f39f09e214&absolute=false&stat=instructions%3Au)

1 regresión, 1 mejora, 1 mixto; 2 de ellos en rollups
34 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-07-21.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: no especificada] [RFC para objetivos del proyecto](https://github.com/rust-lang/rfcs/pull/3672)
* [disposición: fusionar] [Promover aarch64-apple-darwin al Nivel 1](https://github.com/rust-lang/rfcs/pull/3671)
* [disposición: fusionar] [agregar semántica flotante RFC](https://github.com/rust-lang/rfcs/pull/3514)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para el acceso a campos anidados en offset_of](https://github.com/rust-lang/rust/issues/120140)
* [disposición: fusionar] [[rustdoc] Agregar función de código de copia](https://github.com/rust-lang/rust/pull/125779)
* [disposición: fusionar] [Estabilizar 'const {integer}::from_str_radix' es decir, 'const_int_from_str'](https://github.com/rust-lang/rust/pull/124941)
* [disposición: fusionar] [La unión implícita de subprocesos con ámbito no espera a que se eliminen las ubicaciones de subprocesos](https://github.com/rust-lang/rust/issues/116237)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hubo problemas de seguimiento de equipos lingüísticos ni relaciones públicas en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el período final de comentarios esta semana.*

##### [Directrices para códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro o PR ingresados al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Eventos de Rusty entre 2024-07-24 - 2024-08-21 🦀

### Virtual
* 24/07/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Explorando los casos de uso de la API de Rust**](https://www.meetup.com/women-in-rust/events/301730780/)
* 25/07/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897865/)
* 27/07/2024 | Híbrido - Virtual y Presencial (Kiev, UA) | [Rust de UA](https://uarust.com/)
    * [**Conferencia UARust 2024**](https://uarust.com/)
* 27/07/2024 | Virtual | [Reunión mensual de Leptos](https://lu.ma/user/leptos)
    * [**Encuentro mensual de Leptos: Pavex con Luca Palmieri**](https://lu.ma/3ouqapsr)
* 30/07/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/301585665/)
* 31/07/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Herramientas de línea de comandos: Implementación de wc en Rust (Inglés, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 01/08/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633265/)
* 06/08/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**¡Almuerza y aprende! (Virtual)**](https://www.meetup.com/women-in-rust/events/300994574/)
* 06/08/2024 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191718/)
* 06/08/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Desarrollo web en Rust usando Rocket - parte 2 (Inglés)**](https://www.meetup.com/code-mavens/events/301736709/)
* 07/08/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/300328027/)
* 08/08/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897918/)
* 08/08/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300787793/)
* 08/08/2024 | Virtual (Tel Aviv, Illinois) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Lectura del código fuente de Rust: La caja de los miles (inglés)**](https://www.meetup.com/code-mavens/events/302391142/)
* 13/08/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346978/)
* 15/08/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633266/)
* 20/08/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346968/)
* 21/08/2024 | Híbrido - Virtual y Presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631735/)

### África
* 02/08/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)

### Europa
* 25/07/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)
* 25/07/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288967/)
* 27/07/2024 | Híbrido - Virtual y Presencial (Kiev, UA) | [Rust de UA](https://uarust.com/)
    * [**Conferencia UARust 2024**](https://uarust.com/)
* 30/07/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #9**](https://www.meetup.com/rust-basel/events/301459503/)
* 14/08/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/302153005/)
* 20/08/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de hackeos**](https://www.meetup.com/rust-aarhus/events/301522950/)
* 21/08/2024 | Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Walk'n'Talk por Wöhrder See (+ Burritos)**](https://www.meetup.com/rust-noris/events/302080495/)

### América del Norte
* 24/07/2024 | Austin, TX, EE. UU. | [Oxidar ATC](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)
* 25/07/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302066981/)
* 29/07/2024 | Cambridge, MA, Estados Unidos| [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Alewife Rust, 29 de julio**](https://www.meetup.com/bostonrust/events/301550289/)
* 01/08/2024 | St. Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Vidas**](https://www.meetup.com/stl-rust/events/301697569/)
* 08/08/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302067008/)
* 08/08/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Encuentro de agosto**](https://www.meetup.com/seattle-rust-user-group/events/302330477/)
* 19/08/2024 | Minneapolis, MN EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup: "Estado de la programación de GPU Rust" y hora feliz**](https://www.meetup.com/minneapolis-rust-meetup/events/301428949/)
* 20/08/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/301614081/)
* 21/08/2024 | Híbrido - Vancouver, Columbia Británica, CA | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631735/)

# Oceanía
* 01/08/2024 | Brisbane, QLD, Australia | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**Encuentro de agosto**](https://www.meetup.com/rust-brisbane/events/302244260/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1dvlhl6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Rust no te da errores buenos, te da control sobre los errores.

– [cameronm1024 en r/rust](https://www.reddit.com/r/rust/comments/1e978l7/ive_used_and_loved_rust_for_10_years_here_are_the/lecp79z/)

A pesar de la lamentable escasez de sugerencias, llogiq está satisfecho con su elección.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1ebhawb/this_week_in_rust_557/)</small>
