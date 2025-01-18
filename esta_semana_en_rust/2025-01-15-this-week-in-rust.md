---
title: "Esta semana en Rust #44"
number_of_week: 44
description: Actualizaciones del compilador, RFCs destacados, eventos comunitarios y recursos para empleos en Rust, junto con la "Frase de la Semana"
date: 2025-01-15
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---

Hola y bienvenidos a otra edición de *Esta Semana en Rust*!  
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos construir software confiable y eficiente.  
Este es un resumen semanal de su progreso y comunidad.  

¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (anteriormente Twitter), en [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos un pull request](https://github.com/rust-lang/this-week-in-rust).  

¿Quieres involucrarte? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).  

*Esta Semana en Rust* se desarrolla de manera abierta en [GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos están disponibles en [this-week-in-rust.org](https://this-week-in-rust.org/).  
Si encuentras algún error en esta edición, [envía un PR](https://github.com/rust-lang/this-week-in-rust/pulls).  

¿Quieres recibir *Esta Semana en Rust* en tu correo? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).  

## Actualizaciones de la comunidad de Rust  

<!--

Queridos contribuidores de la comunidad:  
Lean README.md para obtener orientación sobre cómo enviar contenidos.  
Cada enlace enviado debe tener la siguiente forma:  

* [Título de la página enlazada](https://example.com/mi_artículo)  

Si no sabes en qué categoría colocar el enlace, siéntete libre de enviar un PR y pedir a los editores que seleccionen la categoría.  

-->

### Oficial  
* [Anuncio de Rust 1.84.0](https://blog.rust-lang.org/2025/01/09/Rust-1.84.0.html)  
* [Este Mes en Nuestra Infra de Pruebas: Diciembre 2024](https://blog.rust-lang.org/inside-rust/2025/01/10/test-infra-dec-2024.html)  

### Fundación  
* [Anuncio de Rust Global 2025: Londres](https://foundation.rust-lang.org/news/announcing-rust-global-2025-london/)  

### Boletines  
* [Este Mes en Rust OSDev: Diciembre 2024](https://rust-osdev.com/this-month/2024-12/)  
* [Tendencias de Rust Edición #57](https://rust-trends.com/newsletter/start-2025-with-rust-honeypots-free-cloud-deployments-and-software-reliability/)  

### Actualizaciones de proyectos/herramientas  
* [cargo.nvim - Un plugin de Neovim para comandos de Cargo en Rust](https://github.com/nwiizo/cargo.nvim)  
* [Actualizaciones en Programación Genérica por Contexto: Lanzamiento de la versión v0.3.0 y nuevos capítulos](https://contextgeneric.dev/blog/v0-3-0-release/)  
* [El runtime de aprendizaje automático RTen - Una retrospectiva del 2024](https://robertknight.me.uk/posts/rten-2024/)  

### Observaciones/Reflexiones  
* [El problema de los auto-traits en gen](https://blog.yoshuawuyts.com/gen-auto-trait-problem/)  
* [Rust Async trata sobre concurrencia, no (solo) rendimiento](https://kobzol.github.io/rust/2025/01/15/async-rust-is-about-concurrency.html)  
* [El atractivo emocional de Rust](https://www.shuttle.dev/blog/2025/01/14/the-appeal-of-rust)  
* [audio] [Brave con Anton Lazarev](https://corrode.dev/podcast/s03e07-brave/)  
* [audio] [Lychee con Matthias Endler](https://rustacean-station.org/episode/matthias-endler/)  

### Tutoriales de Rust  
* [Creando un controlador de dispositivos embebidos en Rust](https://blog.mjolner.tech/device-driver-rust/)  
* [Evaluación constante en Rust para validación de cadenas hexadecimales](https://asyncmove.com/blog/2025/01/compile-time-hex-string-validation-in-rust-using-const-evaluation/)  
* [Ejecución concurrente y paralela de futuros en Rust](https://heikoseeberger.de/2025-01-15-concurrent-parallel-futures/)  
* [video] [Introducción a Embassy: desarrollo embebido con Rust async](https://www.youtube.com/watch?v=pDd5mXBF4tY)  
* [video] [Comprendiendo Macros Procedurales](https://www.youtube.com/watch?v=SMCRQj9Hbx8)  
* [video] [CppCon - Interoperabilidad C++/Rust: Uso práctico de puentes](https://tylerjw.dev/posts/20240920-cppcon-cpp-rust-interop/)  

### Misceláneo  
* [Informe de empleos de Rust - Diciembre 2024](https://filtra.io/rust/jobs-report/dec-24)  
* [Trazando grandes asignaciones de memoria en Rust con BPFtrace](https://readyset.io/blog/tracing-large-memory-allocations-in-rust-with-bpftrace)  
* [Sobre los LLMs y la optimización de código](https://wiredream.com/llm-optimizing-digit-diff/)  
* [Nand2Tetris - Proyecto 7 (Traductor de Máquina Virtual, Parte 1)](https://gurudas.dev/blog/2025/01/01/nand-to-tetris-2024-project-7/)  

## Crate de la Semana  

El crate de esta semana es [vidyut](https://github.com/ambuda-org/vidyut), un kit de herramientas en sánscrito con funcionalidades sobre métrica, segmentación, inflexiones, etc.  

Gracias a [Arun Prasad](https://users.rust-lang.org/t/crate-of-the-week/2704/1387) por la auto-sugerencia.  

[¡Envía tus sugerencias y votos para la próxima semana aquí!][submit_crate]  

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704  

### Convocatorias de Participación (CFPs)

<!-- Las convocatorias van aquí, usa este formato: * [nombre del proyecto - título del issue](URL al issue) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna - *No se enviaron convocatorias de participación esta semana.* -->

* [rama - revisar si se pueden/deben realizar mejoras en el soporte de la capa HTTP Open Telemetry de rama](https://github.com/plabayo/rama/issues/383)
* [rama – agregar rama al FrameworkBenchmark de TechEmpower](https://github.com/plabayo/rama/issues/389)
* [rama – agregar el benchmark del servidor rama a sharkbench](https://github.com/plabayo/rama/issues/390)

Si eres el propietario de un proyecto en Rust y estás buscando colaboradores, envía tus tareas [aquí][guidelines], a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust), o contactándonos en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]: https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### Convocatorias de Ponencias - Eventos

¿Eres un ponente nuevo o experimentado buscando un lugar para compartir algo interesante? Esta sección resalta eventos que están siendo planeados y que están aceptando propuestas para participar como orador.

<!-- Las convocatorias van aquí, usa este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguna - *No se enviaron convocatorias de ponencias o presentaciones esta semana.* -->

* [**Rust Week (Rust NL)**](https://www.papercall.io/rust-week) | Cierra el 2025-01-19 | Utrecht, NL | Evento los días 2025-05-13 y 2025-05-14
* [**Rust Summit**](https://docs.google.com/forms/d/e/1FAIpQLSf_O5ISyLQm_DL5_pEcMkc6AWMp7t4YufO-7kwUQTGQ5MW4dw/viewform) | Fecha límite continua | Belgrado, RS | Evento el 2025-06-07

Si eres un organizador de eventos y deseas ampliar el alcance de tu evento, envía un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o contáctanos en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

---

### Actualizaciones del Proyecto Rust

469 solicitudes de extracción (pull requests) fueron [fusionadas la semana pasada][merged].

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-07..2025-01-14

* [agregar nuevos objetivos `{x86_64,i686}-win7-windows-gnu`](https://github.com/rust-lang/rust/pull/134609)
* [arm: añadir característica de objetivo de punto flotante suave inestable](https://github.com/rust-lang/rust/pull/135203)
* [`-Zrandomize-layout` más estricto. `Foo<T> != Foo<U>`](https://github.com/rust-lang/rust/pull/133088)
* [`best_blame_constraint`: Mejorar la atribución de mejores restricciones cuando el gráfico de regiones tiene ciclos por invariancia o `'static`](https://github.com/rust-lang/rust/pull/133858)
* [`mir_transform`: implementar `#[rustc_force_inline]`](https://github.com/rust-lang/rust/pull/134082)
* [`run_make_support`: añadir `#![warn(unreachable_pub)]`](https://github.com/rust-lang/rust/pull/135411)
* [tener en cuenta elementos sustituidos por identidad en el mangling de símbolos](https://github.com/rust-lang/rust/pull/135261)
* [añadir `-Zmin-function-alignment`](https://github.com/rust-lang/rust/pull/134030)
* [agregar entrada `default_field_values` al libro inestable](https://github.com/rust-lang/rust/pull/134855)
* [añadir una lista de símbolos para crates de biblioteca estándar estables](https://github.com/rust-lang/rust/pull/135247)
* [implementar `const Destruct` en el antiguo solucionador](https://github.com/rust-lang/rust/pull/134875)
* [reducir el alcance del error de ciclo solo con `-Zdump-mir`](https://github.com/rust-lang/rust/pull/134498)
* [reservar el registro x18 para el objetivo aarch64 wrs vxworks](https://github.com/rust-lang/rust/pull/135184)
* [sugerir reemplazo de coma por punto y coma en expresiones repetidas incorrectas](https://github.com/rust-lang/rust/pull/128110)
* [soporte para target específico en `optimized-compiler-builtins`](https://github.com/rust-lang/rust/pull/135326)
* [unificar los errores de `conditional-const` con los errores de no constantes](https://github.com/rust-lang/rust/pull/134732)
* [usar un entorno de tipado post-monomorfización al generar nombres para componentes provenientes de `impls`](https://github.com/rust-lang/rust/pull/135149)
* [usar `llvm.memset.p0i8.*` para inicializar arreglos de bytes iguales](https://github.com/rust-lang/rust/pull/135258)
* [usado el nombre de funciones de `pthread` que devuelven un resultado para FreeBSD y DragonFly](https://github.com/rust-lang/rust/pull/132607)
* [advertir sobre SIMD roto no solo en structs sino también en enums y unions si no se optó por ello](https://github.com/rust-lang/rust/pull/135219)
* [implementar subida de traits](https://github.com/rust-lang/chalk/pull/821)
* [optimización MIR: más casos de `transmute` en GVN](https://github.com/rust-lang/rust/pull/133324)
* [Miri: agregar mantenedor para FreeBSD; probar todo en sistemas Solarish](https://github.com/rust-lang/miri/pull/4135)
* [Miri: añadido Android a los objetivos de prueba para `epoll` y `eventfd`](https://github.com/rust-lang/miri/pull/4137)
* [Miri: ajustar cómo se compila `miri-script` en RA para arreglar proc-macros](https://github.com/rust-lang/miri/pull/4134)
* [Miri: añadido soporte para `epoll` y `eventfd` en illumos](https://github.com/rust-lang/miri/pull/4136)
* [Miri: soportado `fioclex` para `ioctl` en macOS](https://github.com/rust-lang/miri/pull/4133)
* [Miri: cambiado FreeBSD a usar `pthread_setname_np`](https://github.com/rust-lang/miri/pull/4132)
* [Miri: usar `deref_pointer_as` en lugar de `deref_pointer`](https://github.com/rust-lang/miri/pull/4138)
* [`proc_macro`: usar el trait `ToTokens` en la macro `quote`](https://github.com/rust-lang/rust/pull/134693)
* [añadir `#[inline]` a `copy_from_slice`](https://github.com/rust-lang/rust/pull/135384)
* [implementar `String::into_chars`](https://github.com/rust-lang/rust/pull/133057)
* [módulo inicial de sistema de archivos para UEFI](https://github.com/rust-lang/rust/pull/135324)
* [HashBrown: añadido argumento de plantilla Allocator para `rustc_iter`](https://github.com/rust-lang/hashbrown/pull/605)
* [considerar niveles de optimización diferentes de números](https://github.com/rust-lang/compiler-builtins/pull/743)
* [Cargo: arreglar esquema JSON de 'metadata'](https://github.com/rust-lang/cargo/pull/15033)
* [Cargo: corregir esquema JSON de `[lints]`](https://github.com/rust-lang/cargo/pull/15035)
* [Cargo: rendimiento, `cargo-package`: coincidir ciertos prefijos de ruta con `pathspec`](https://github.com/rust-lang/cargo/pull/14997)
* [Cargo: corregir emisión de advertencias al aprender información sobre objetivos de Rust](https://github.com/rust-lang/cargo/pull/15036)
* [Cargo: hacer explícito `"C"` en `extern "C"`](https://github.com/rust-lang/cargo/pull/15034)
* [Cargo: configurar entorno de cargo para `cargo rustc --print`](https://github.com/rust-lang/cargo/pull/15026)
* [Cargo: simplificar `SourceID` Ord/Eq](https://github.com/rust-lang/cargo/pull/14980)
* [Rustdoc JSON: incluir elementos en módulos marcados como eliminados en `Crate::paths`](https://github.com/rust-lang/rust/pull/135348)
* [Rustdoc: usar el marcador de estabilidad de importación en la visualización](https://github.com/rust-lang/rust/pull/135352)
* [Rustdoc: usar rutas estables como rutas canónicas preferidas](https://github.com/rust-lang/rust/pull/135171)
* [Rustfmt: eliminar el bloqueo nocturno de la bandera `--style-edition`](https://github.com/rust-lang/rust/pull/135200)
* [rust-analyzer: agregar vista de árbol de sintaxis nueva y mejorada](https://github.com/rust-lang/rust-analyzer/pull/18813)  
* [rust-analyzer: agregar configuración para incluir rutas adicionales al VFS](https://github.com/rust-lang/rust-analyzer/pull/18880)  
* [rust-analyzer: reimplementar el resaltado de cadenas de Rust con atributos de herramientas](https://github.com/rust-lang/rust-analyzer/pull/18906)  
* [rust-analyzer: corregir inclusión de archivos de construcción en `PackageRoot` del proyecto JSON](https://github.com/rust-lang/rust-analyzer/pull/18866)  
* [rust-analyzer: no calcular `prettify_macro_expansion()` a menos que se invoque la asistencia "Expandir macro en línea"](https://github.com/rust-lang/rust-analyzer/pull/18900)  
* [rust-analyzer: no ofrecer completados dentro de cadenas en macros](https://github.com/rust-lang/rust-analyzer/pull/18832)  
* [rust-analyzer: corregir chequeo de macros `env`/`option_env` que ignoraba definiciones de `macro_rules`](https://github.com/rust-lang/rust-analyzer/pull/18884)  
* [rust-analyzer: corregir edición de texto para pistas de modo de enlace `ref`](https://github.com/rust-lang/rust-analyzer/pull/18920)  
* [rust-analyzer: corregir un error con enlaces faltantes en MBE](https://github.com/rust-lang/rust-analyzer/pull/18877)  
* [rust-analyzer: corregir búsqueda de tokens reales en la expansión de completado](https://github.com/rust-lang/rust-analyzer/pull/18889)  
* [rust-analyzer: solucionar otro problema con reversión de ajustes](https://github.com/rust-lang/rust-analyzer/pull/18899)  
* [rust-analyzer: corregir diagnósticos que no se limpiaban entre verificaciones incrementales](https://github.com/rust-lang/rust-analyzer/pull/18864)  
* [rust-analyzer: hacer que la edición sea por token, no por archivo](https://github.com/rust-lang/rust-analyzer/pull/18861)  
* [rust-analyzer: implementar `#[rust_analyzer::skip]` para cuerpos](https://github.com/rust-lang/rust-analyzer/pull/18907)  
* [rust-analyzer: implementar sugerencias en línea para límites implícitos de tamaño](https://github.com/rust-lang/rust-analyzer/pull/18903)  
* [rust-analyzer: mejorar el renderizado del hover para rutas de módulos](https://github.com/rust-lang/rust-analyzer/pull/18904)  

### Evaluación de Rendimiento del Compilador de Rust

Una semana tranquila con pocos cambios en el rendimiento del compilador. La mayor regresión fue rápidamente identificada y revertida.

Evaluación realizada por **@rylev**.  
Rango de revisiones: [0f1e965f..1ab85fbd](https://perf.rust-lang.org/?start=0f1e965fec3bc2f97b932e9dd8e85fca6d7faadc&end=1ab85fbd7474e8ce84d5283548f21472860de3e2&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instructions:u)                   | Media   | Rango            | Cantidad |
|:----------------------------------:|:-------:|:----------------:|:--------:|
| Regresiones ❌ <br /> (primarias)  | 0.4%    | [0.1%, 1.8%]     | 21       |
| Regresiones ❌ <br /> (secundarias)| 0.5%    | [0.0%, 2.0%]     | 35       |
| Mejoras ✅ <br /> (primarias)      | -0.8%   | [-2.7%, -0.3%]   | 6        |
| Mejoras ✅ <br /> (secundarias)    | -10.2%  | [-27.8%, -0.1%]  | 13       |
| Total ❌✅ (primarias)             | 0.2%    | [-2.7%, 1.8%]    | 27       |

4 regresiones, 3 mejoras, 3 resultados mixtos; 3 de ellos en *rollups*.  
Se realizaron 44 comparaciones de artefactos en total.

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/194b9b1c83f638475d35b5bc27c9617ec3725941/triage/2025-01-14.md)

---

### [RFCs Aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de [RFC (Request for Comments)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos son los RFCs aprobados para implementación esta semana:

* *No se aprobaron RFCs esta semana.*

---

### Periodo de Comentarios Finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'Periodo de Comentarios Finales' para RFCs y PRs clave que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Superposición de elementos de supertrazas v2](https://github.com/rust-lang/rfcs/pull/3624)

#### Issues y PRs en Seguimiento
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Eliminar soporte para el atributo (inestable) `#[start]`](https://github.com/rust-lang/rust/pull/134299)
* [Desestabilizar completamente todos los atributos internos personalizados](https://github.com/rust-lang/rust/pull/134276)
* [Elevar el lint `clippy::double_neg` como `double_negations`](https://github.com/rust-lang/rust/pull/126604)
* [Optimizar implementación de `Seek::stream_len` para File](https://github.com/rust-lang/rust/pull/125087)
* [[rustdoc] Agregar configuración de fuente sans-serif](https://github.com/rust-lang/rust/pull/133636)
* [Issue de seguimiento para `PathBuf::add_extension` y `Path::with_added_extension`](https://github.com/rust-lang/rust/issues/127292)
* [Convertir la advertencia futura de compatibilidad `wasm_c_abi` en un error crítico](https://github.com/rust-lang/rust/pull/133951)
* [const-eval: detectar más punteros como definitivamente no nulos](https://github.com/rust-lang/rust/pull/133700)
* [Considerar campos como habitables si son inestables](https://github.com/rust-lang/rust/pull/133889)
* [Prohibir `repr()` en elementos inválidos](https://github.com/rust-lang/rust/pull/133925)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay Issues o PRs de Cargo en el Periodo de Comentarios Finales esta semana.*

##### [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hay propuestas del Equipo de Lenguaje en el Periodo de Comentarios Finales esta semana.*

##### [Referencia del Lenguaje](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Los elementos estáticos 'static' distintos nunca se superponen](https://github.com/rust-lang/reference/pull/1657)

##### [Pautas de Código Inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay Issues o PRs en el Periodo de Comentarios Finales esta semana.*

#### [Nuevos y Actualizados RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Permitir métodos de traza en contextos constantes](https://github.com/rust-lang/rfcs/pull/3762)
* [RFC: Permitir a los paquetes especificar un conjunto de objetivos compatibles](https://github.com/rust-lang/rfcs/pull/3759)

### Próximos Eventos

Eventos relacionados con Rust entre el **15 de enero de 2025** y el **12 de febrero de 2025** 🦀

### Virtual
* 2025-01-15 | Virtual (London, UK) | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/)
    * [**Meet and greet with project allocations**](https://www.meetup.com/london-rust-project-group/events/305211634/)
* 2025-01-15 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**An introduction to WASM in Rust with Márk Tolmács (Virtual, English)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)
* 2025-01-16 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633278/)
* 2025-01-16 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust January 2025 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/305613752)
* 2025-01-16 | Virtual and In-Person (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**January Meetup**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-17 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305207687/)
* 2025-01-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/305646783/)
* 2025-01-21 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Exploring Rust Enums with Yoni Peleg (Virtual, Hebrew)**](https://www.meetup.com/rust-tlv/events/305110744)
* 2025-01-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyhccbcc)
* 2025-01-22 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)
* 2025-01-23 & 2025-01-24 | Virtual | [Mainmatter Rust Workshop](https://ti.to/mainmatter/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-jan-2025)
* 2025-01-24 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305425309/)
* 2025-01-26 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Rust and embedded programming with Leon Vak (online in Hebrew)**](https://www.meetup.com/rust-tlv/events/304971264)
* 2025-01-27 | Virtual (London, UK) | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/)
    * [**using traits in Rust for flexibility, mocking/ unit testing, and more**](https://www.meetup.com/london-rust-project-group/events/305211672/)
* 2025-01-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/305361243)
* 2025-01-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/299468340)
* 2025-01-30 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Quantum Computers Can’t Rust-Proof This!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 2025-01-30 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/code-mavens/events/305382647)
* 2025-01-31 | Virtual (Delhi, IN) | [Hackathon Raptors Association](https://www.meetup.com/hackathon-raptors-association/)
    * [**Blazingly Fast Rust Hackathon**](https://www.meetup.com/hackathon-raptors-association/events/305435372/)
* 2025-01-31 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305560416/)
* 2025-02-01 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-02-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
* 2025-02-04 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Rust Nation UK Talks**](https://www.meetup.com/women-in-rust/events/305647334/)
* 2025-02-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031658)
* 2025-02-07 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbkb/)
* 2025-02-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815036)
* 2025-02-11 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Meet Elusion: New DataFrame Library powered by Rust 🦀 with Borivoj Grujicic**](https://www.meetup.com/code-mavens/events/305513416)

### Europe
* 2025-01-16 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/305339712)
* 2025-01-16 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/305144321)
* 2025-01-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #8**](https://www.meetup.com/stockholm-rust/events/305475761)
* 2025-01-21 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/304110936)
* 2025-01-21 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Tech Talks & Dinner: Insights on Systems Programming Side Projects (in Rust) - Leptos (full-stack Rust with webassembly), Karyon (distributed p2p software in Rust), FunDSP (audio synthesis in Rust)**](https://www.meetup.com/systems-programming-ghent/events/305201540/?slug=systems-programming-ghent&eventId=305201540)
* 2025-01-21 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Self-Organized Peer-to-Peer Networks using Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303604074)
* 2025-01-22 | London, GB | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London's New Years Party & Community Swag Drop**](https://www.meetup.com/rust-london-user-group/events/305588703)
* 2025-01-22 | Oberursel, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust 2024 Edition and Beyond**](https://www.meetup.com/rust-rhein-main/events/305330873)
* 2025-01-23 | Barcelona, ES | [Barcelona Free Software](https://www.meetup.com/barcelona-free-software/events/)
    * [**Why Build a New Browser Engine in Rust?**](https://www.meetup.com/barcelona-free-software/events/305179554)
* 2025-01-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #74**](https://www.meetup.com/rust-paris/events/305455221)
* 2025-01-24 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/305204279)
* 2025-01-27 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Prague (January 2025)**](https://www.meetup.com/rust-prague/events/305455153)
* 2025-01-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/304487851)
* 2025-01-28 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester January Code Night**](https://www.meetup.com/rust-manchester/events/305496243)
* 2025-01-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Hypermedia-driven development in Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 2025-01-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421383)
* 2025-02-01 | Brussels, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 2025-02-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 2025-02-05 | Oxford, GB | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)
* 2025-02-12 | Reading, GB | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045444)

### North America
* 2025-01-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 1: Community Introductions**](https://www.meetup.com/music-city-rust-developers/events/304333017)
* 2025-01-16 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**January Meetup**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-16 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Spokane Rust Monthly Meetup: Traits and Generics**](https://www.meetup.com/spokane-rust/events/305501106)
* 2025-01-17 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Multithreading y Async en Rust 101 - HolaMundo - Parte 3**](https://www.meetup.com/rust-mx/events/305464827)
* 2025-01-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Back Bay Rust Lunch, Jan 18**](https://www.meetup.com/bostonrust/events/304951470)
* 2025-01-21 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/305600833)
* 2025-01-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638258)
* 2025-01-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/305325657)
* 2025-01-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305414182) | [**Rust Meetup at Hacker Dojo - Mountain View Rust Meetup Page**](https://www.meetup.com/mv-rust-meetup/events/305564600)
* 2025-01-28 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/events/)
    * [**From Basics to Advanced: Testing**](https://www.meetup.com/boulder-rust-meetup/events/305597961)
* 2025-02-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Async, the Future of Futures**](https://www.meetup.com/stl-rust/events/304959018)

### Oceania:
* 2025-02-04 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust AKL: How We Learn Rust**](https://www.meetup.com/rust-akl/events/305583693)

Si estás organizando un evento relacionado con Rust, agrégalo al [calendario] para que sea mencionado aquí. Recuerda también incluir un enlace al evento. Puedes escribir al [Equipo de la Comunidad de Rust][community] para solicitar acceso.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com  
[community]: mailto:community-team@rust-lang.org  

## Empleos  
<!--  
Empleos en Rust:

TWiR ha dejado de incluir ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí:  

https://github.com/rust-lang/this-week-in-rust/issues/3412  
-->  

Consulta el último [hilo de "Who's Hiring" en r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)  

## Frase de la Semana  

> Esta es una maravillosa falta de solidez y estoy increíblemente emocionado por ella :3  

– [lcnr en GitHub](https://github.com/rust-lang/rust/issues/135011#issuecomment-2573248261)  

Gracias a [Christoph Grenz](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1650) por la sugerencia.  

[¡Envía tus frases y vota por la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)  

*Esta Semana en Rust fue editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*  

*El hospedaje de la lista de correos es patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*  

<small>[Discute en r/rust](https://www.reddit.com/r/rust/comments/1i2e9mn/this_week_in_rust_582/)</small>  