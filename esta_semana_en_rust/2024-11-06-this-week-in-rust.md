---
title: "Esta semana en Rust #34"
number_of_week: 34
description: El crate de esta semana es wtransport, una implementación de la especificación WebTransport, un sucesor de WebSockets con muchas características adicionales.
date: 2024-11-06
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
* [Actualización de los objetivos del proyecto en octubre](https://blog.rust-lang.org/2024/10/31/project-goals-oct-update.html)
* [Próximos pasos en la política de marcas registradas de Rust](https://blog.rust-lang.org/2024/11/06/trademark-update.html)
* [Este ciclo de desarrollo en carga: 1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html)
* [Reorganizar el equipo compilador y reconocer a los miembros de nuestro equipo](https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg.html)
* [Este mes en nuestra infra de pruebas: octubre de 2024](https://blog.rust-lang.org/inside-rust/2024/11/04/test-infra-oct-2024-2.html)
* [Convocatoria de propuestas: Objetivos del proyecto Rust 2025h1](https://blog.rust-lang.org/inside-rust/2024/11/04/project-goals-2025h1-call-for-proposals.html)

### Fundación
* [Resumen del tercer trimestre de 2024 de Rebecca Rumbul](https://foundation.rust-lang.org/news/q3-2024-recap-from-rebecca-rumbul/)
* [Anuncio de los miembros de la Fundación Rust: CodeDay, OpenSource Science (OS-Sci) y PROMOTIC](https://foundation.rust-lang.org/news/rust-foundation-member-announcement-codeday-opensource-science-os-sci-promotic/)

### Boletines
* [El Rustacean Incrustado Edición #31](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-31)

### Actualizaciones de proyectos/herramientas
* [Anunciando Intentrace, un strace alternativo para todos](https://github.com/sectordistrict/intentrace)
* [Inicio rápido de Ractor](https://slawlor.github.io/ractor/quickstart/)
* [Anunciando Sycamore v0.9.0](https://sycamore.dev/post/announcing-v0-9-0)
* [Versión CXX-Qt 0.7](https://www.kdab.com/cxx-qt-0-7/)
* [Un juego de plataformas 'educativo' para que los niños aprendan matemáticas y lectura, y un montón para los desarrolladores](https://github.com/uggla/rock_run)
* [ZH][ES] [Seleccionar componentes HTML en Rust declarativo](https://ideas.reify.ing/en/blog/select-html-declarative-rust/)

### Observaciones/Pensamientos
* [Seguridad en un mundo inseguro](https://lwn.net/SubscriberLink/995814/17e451bcb3015920/)
* [MinPin: otra propuesta de pin](https://smallcultfollowing.com/babysteps/blog/2024/11/05/minpin/)
* [Alcanzado el límite de recursividad... en tiempo de construcción?](https://blog.veeso.dev/blog/en/reached-the-recursion-limit-at-build-time/)
* [Construyendo software confiable: el poder de las pruebas en Rust](https://tylerjw.dev/posts/20241106-building-trustworthy-software/)
* [Async Rust no es seguro con io_uring](https://tonbo.io/blog/async-rust-is-not-safe-with-io-uring)
* [Macros, seguridad y SOA](https://tim-harding.github.io/blog/soa-rs/)
* [¿Qué tan grande es tu futuro?](https://hegdenu.net/posts/how-big-is-your-future/)
* [Una comparación del verificador de préstamos de Rust con el de C#](https://em-tg.github.io/csborrow/)
* [API de transmisión de audio en Rust pt. 3: Decodificación de audio](https://xd009642.github.io/2024/11/04/streaming-audio-APIs-audio-decoding.html)
* [audio] [InfinyOn con Deb Roy Chowdhury](https://corrode.dev/podcast/s03e02-infinyon/)

### Tutoriales de Rust
* [Diferencia entre iter() y into_iter() en Rust](https://crustc.com/difference-iter-and-into_iter/)
* [Punto muerto furtivo de Rust con bloques 'if let'](https://brooksblog.bearblog.dev/rusts-sneaky-deadlock-with-if-let-blocks/)
* [Por qué me encanta Rust para tokenizar y analizar](https://xnacly.me/posts/2024/rust-pldev/)
* [Optimizaciones de "cadena alemana" en Spellbook](https://the-mikedavis.github.io/posts/german-string-optimizations-in-spellbook/)
* [Sintaxis más sutil de Rust](https://zkrising.com/writing/rusts-most-subtle-syntax/)
* [Analizando argumentos en Rust sin dependencias](https://www.ntietz.com/blog/parsing-arguments-rust-no-deps/)
* [Forma sencilla de hacer soporte para i18n en Rust con ejemplos y pruebas](https://www.onlycoiners.com/user/onlycoiners/blog/simple-way-to-make-i18n-support-in-rust-with-with-examples-a)
* [Cómo clonar una vaca a poca profundidad](https://blog.getreu.net/20241005-how_to_shallow_clone_a_cow-blog/)
* [Desarrollo principiante de Rust ESP32 - Snake](https://jamesmcm.github.io/blog/beginner-rust-esp32-lcdsnake/)
* [video] [Colecciones e iteradores de Rust desmitificados 🪄](https://www.youtube.com/watch?v=oiWATcjyUEI)

### Investigación
* [Caronte: Un marco de análisis para el Rust](https://arxiv.org/abs/2410.18042)
* [Crux, un verificador preciso para Rust y otros idiomas](https://arxiv.org/abs/2410.18280)

### Miscelánea
* [Federales: El software crítico debe abandonar C/C++ para 2026 o enfrentar riesgos](https://thenewstack.io/feds-critical-software-must-drop-c-c-by-2026-or-face-risk/)
* [audio] [Hablemos de Rust con John Arundel](https://gopodcast.dev/episodes/046-lets-talk-about-rust-with-john-arundel)
* [audio] [Explorando Rust para Sistemas Embebidos con Philip Markgraf](https://agileembeddedpodcast.com/episodes/philip-markgraf-on-rust)

## Crate de la semana

El crate de esta semana es [wtransport](https://crates.io/crates/wtransport), una implementación de la especificación WebTransport, un sucesor de WebSockets con muchas características adicionales.

¡Gracias a [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/1369) por la sugerencia!

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

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 473 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-29..2024-11-05

* [tenga en cuenta la profundidad de límite tardío al capturar todas las duraciones opacas](https://github.com/rust-lang/rust/pull/132466)
* [añadir '--print host-tuple' para imprimir la tupla de destino del host](https://github.com/rust-lang/rust/pull/125579)
* [añadir 'F16' y 'F128' a 'invalid_nan_comparison'](https://github.com/rust-lang/rust/pull/132439)
* [añadir 'LP64E' RISC-V ABI](https://github.com/rust-lang/rust/pull/132354)
* [también tratar la definición principal 'impl' como transparente con respecto a los módulos](https://github.com/rust-lang/rust/pull/132453)
* [Atributos de limpieza alrededor de turnos no marcados y negación no marcada en const](https://github.com/rust-lang/rust/pull/132445)
* [limpieza op lookup in HIR typeck](https://github.com/rust-lang/rust/pull/132274)
* [recopila los límites de los elementos para los RPITIT a partir de las cláusulas de rasgo donde son similares a los tipos asociados](https://github.com/rust-lang/rust/pull/132194)
* [no forzar los efectos de constancia '~const' en typeck si 'rustc_do_not_const_check'](https://github.com/rust-lang/rust/pull/132366)
* [no peluques 'irrefutable_let_patterns' en los patrones iniciales si 'else if' deja cadenas](https://github.com/rust-lang/rust/pull/129394)
* [doble verificación de la constancia condicional en MIR](https://github.com/rust-lang/rust/pull/132276)
* [asegúrese de que Resume arg sobreviva a la región vinculada a corrutinas](https://github.com/rust-lang/rust/pull/132151)
* [encuentre el contenedor genérico en lugar de simplemente buscar el asociado con const arg](https://github.com/rust-lang/rust/pull/132559)
* [arreglar el pánico del compilador con un gran número de hilos](https://github.com/rust-lang/rust/pull/132355)
* [sugerencia de corrección para el error de diagnóstico E0027](https://github.com/rust-lang/rust/pull/132025)
* [Se corrige la validación al reducir los límites del rasgo '?"](https://github.com/rust-lang/rust/pull/132209)
* [Sugerencia de implementación para nunca escribir lints de reserva](https://github.com/rust-lang/rust/pull/132383)
* [mejorar la pelusa 'missing_abi'](https://github.com/rust-lang/rust/pull/132357)
* [mejorar los diagnósticos de copia/clonación de derivación duplicada](https://github.com/rust-lang/rust/pull/131153)
* [llvm: coincide con la nueva alineación de enteros de 128 bits de LLVM en sparc](https://github.com/rust-lang/rust/pull/132422)
* [Hacer que la salida de ayuda de Codegen sea más consistente](https://github.com/rust-lang/rust/pull/132522)
* [asegúrese de que 'type_param_predicates' se resuelva correctamente para RPITIT](https://github.com/rust-lang/rust/pull/132373)
* [pase 'RUSTC_HOST_FLAGS' de una vez sin el bucle for](https://github.com/rust-lang/rust/pull/132365)
* [portar la mayor parte de '--print=target-cpus' a Rust](https://github.com/rust-lang/rust/pull/132514)
* [registrar '~const' preds para ajustes 'Deref' en HIR typeck](https://github.com/rust-lang/rust/pull/132275)
* [rechazar autotipos genéricos](https://github.com/rust-lang/rust/pull/130098)
* [remapear la vida útil de impl-trait en HIR en lugar de reducir AST](https://github.com/rust-lang/rust/pull/129383)
* [eliminar el caso '""' de la declaración de coincidencia 'llvm_abiname' de RISC-V](https://github.com/rust-lang/rust/pull/132421)
* [eliminar 'do_not_const_check' de los métodos 'Iterator'](https://github.com/rust-lang/rust/pull/132368)
* [Eliminar región de los ajustes](https://github.com/rust-lang/rust/pull/132301)
* [eliminar el soporte para '-Zprofile' (instrumentación de cobertura estilo gcov)](https://github.com/rust-lang/rust/pull/131829)
* [Reemplace las conversiones de tiempo manuales por las estándar, análisis de formato de tiempo Comptime](https://github.com/rust-lang/rust/pull/132521)
* [sugerir la creación de tuplas unarias cuando los tipos no coinciden con un rasgo](https://github.com/rust-lang/rust/pull/132583)
* [soporte 'clobber_abi' y registros vectoriales (solo clobber) en el ensamblaje en línea de PowerPC](https://github.com/rust-lang/rust/pull/131341)
* [Intente señalar cuándo las reglas de captura de por vida de la edición 2024 causan problemas de Borrowck](https://github.com/rust-lang/rust/pull/131186)
* [typingMode: fusionar intercrate, reveal y 'defining_opaque_types'](https://github.com/rust-lang/rust/pull/131856)
* [miri: cambiar 'futex_wait' errno de Scalar a IoError](https://github.com/rust-lang/miri/pull/4000)
* [estabilizar 'const_arguments_as_str'](https://github.com/rust-lang/rust/pull/132511)
* [estabilizar 'if_let_rescope'](https://github.com/rust-lang/rust/pull/131984)
* [marque 'str::is_char_boundary' y 'str::split_at*' de manera inestable 'const'](https://github.com/rust-lang/rust/pull/131520)
* [eliminar el soporte const para 'align_offset' y 'is_aligned'](https://github.com/rust-lang/rust/pull/132423)
* [añadir de manera inestable 'ptr::byte_sub_ptr'](https://github.com/rust-lang/rust/pull/132459)
* [implement 'From<&mut {slice}>' para 'Box/Rc/Arc<{slice}>'](https://github.com/rust-lang/rust/pull/129329)
* [rc/Arc: no filtre la asignación si la caída entra en pánico](https://github.com/rust-lang/rust/pull/132231)
* [agregar implementaciones de LowerExp y UpperExp a NonZero](https://github.com/rust-lang/rust/pull/131377)
* [use la impl de Hacker's Delight en 'i64::midpoint' en lugar de la impl ancha 'i128'](https://github.com/rust-lang/rust/pull/132238)
* [xous: sync: eliminar el atributo 'rustc_const_stable' en Condvar y Mutex 'new()'](https://github.com/rust-lang/rust/pull/132321)
* [añadir macro 'const_panic' para que sea más fácil volver al pánico sin formato en const](https://github.com/rust-lang/rust/pull/132542)
* [Cargo: Downgrade Version-exists Error a warning on dry-run](https://github.com/rust-lang/cargo/pull/14742)
* [cargo: añadir más metadatos a 'rustc_fingerprint'](https://github.com/rust-lang/cargo/pull/14761)
* [cargo: añadir semántica transaccional a 'rustfix'](https://github.com/rust-lang/cargo/pull/14747)
* [cargo: añadir el inestable flag '-Zroot-dir' para configurar la ruta desde la que se debe invocar rustc](https://github.com/rust-lang/cargo/pull/14752)
* [cargo: permitir que los scripts de compilación informen mensajes de error a través de 'cargo::error'](https://github.com/rust-lang/cargo/pull/14743)
* [cargo: cambiar las rutas de configuración para marcar solo 'CARGO_HOME' para cargo-script](https://github.com/rust-lang/cargo/pull/14749)
* [Cargo: Descargar DEP transitivos dirigidos de con la plataforma de destino de DEPS de artefacto](https://github.com/rust-lang/cargo/pull/14723)
* [Corrección de carga: Seguimiento de la versión en los archivos de información de huellas dactilares](https://github.com/rust-lang/cargo/pull/14751)
* [cargo: eliminar el requisito para --target al invocar Cargo con -Zbuild-std](https://github.com/rust-lang/cargo/pull/14317)
* [rustdoc: Arreglar '--show-coverage' cuando se usa el formato de salida JSON](https://github.com/rust-lang/rust/pull/132596)
* [rustdoc: Unificar los márgenes de los campos 'struct' variantes con los campos 'struct'](https://github.com/rust-lang/rust/pull/132258)
* [Rustdoc: Hacer que DocTest Span modifique un cambio en la edición 2024](https://github.com/rust-lang/rust/pull/132210)
* [rustdoc: omitir la herencia de estabilidad para algunos tipos de artículos](https://github.com/rust-lang/rust/pull/132481)
* [mdbook: mejorar el soporte de temas cuando JS está deshabilitado](https://github.com/rust-lang/mdBook/pull/2454)
* [mdbook: cargar el toc de la barra lateral desde un archivo JS compartido o iframe](https://github.com/rust-lang/mdBook/pull/2414)
* [clippy: 'infinite_loops': arreglar sugerencias incorrectas sobre funciones/cierres asíncronos](https://github.com/rust-lang/rust-clippy/pull/13608)
* [clippy: 'needless_continue': verifique la consistencia de las etiquetas antes de la advertencia](https://github.com/rust-lang/rust-clippy/pull/13648)
* [clippy: el atributo 'no_mangle' requiere unsafe in Rust 2024](https://github.com/rust-lang/rust-clippy/pull/13631)
* [clippy: añadir nueva pelusa 'trivial_map_over_range'](https://github.com/rust-lang/rust-clippy/pull/13034)
* [clippy: sugerencia de código de limpieza para 'into_iter_without_iter'](https://github.com/rust-lang/rust-clippy/pull/13634)
* [clippy: no use 'gen' como nombre de variable](https://github.com/rust-lang/rust-clippy/pull/13628)
* [clippy: no elimines consts sin nombre y elementos anidados dentro de las funciones en 'missing_docs_in_private_items'](https://github.com/rust-lang/rust-clippy/pull/13573)
* [clippy: extiende 'large_include_file' lint para trabajar también en los atributos](https://github.com/rust-lang/rust-clippy/pull/13636)
* [clippy: arreglar 'allow_attributes' cuando se expande desde algunas macros](https://github.com/rust-lang/rust-clippy/pull/13599)
* [clippy: mejora la visualización de la página clippy lints cuando JS está deshabilitado](https://github.com/rust-lang/rust-clippy/pull/13585)
* [clippy: nueva pelusa 'map_all_any_identity'](https://github.com/rust-lang/rust-clippy/pull/13499)
* [clippy: nueva pelusa 'needless_as_bytes'](https://github.com/rust-lang/rust-clippy/pull/13437)
* [clippy: nueva pelusa 'source_item_ordering'](https://github.com/rust-lang/rust-clippy/pull/13376)
* [clippy: el iterador de retorno no debe capturar vidas en Rust 2024](https://github.com/rust-lang/rust-clippy/pull/13629)
* [Clippy: use Match Ergonomics compatible con las ediciones 2021 y 2024](https://github.com/rust-lang/rust-clippy/pull/13630)
* [rust-analyzer: permitir la interpretación de consts y estáticas con el comando de función interpreta](https://github.com/rust-lang/rust-analyzer/pull/18470)
* [rust-analyzer: evitar la mutabilidad interior en 'TyLoweringContext'](https://github.com/rust-lang/rust-analyzer/pull/18447)
* [rust-analyzer: no renderizar metainformación al pasar el cursor sobre los usos](https://github.com/rust-lang/rust-analyzer/pull/18436)
* [rust-analyzer: add assist para generar un alias de tipo para una función](https://github.com/rust-lang/rust-analyzer/pull/18385)
* [rust-analyzer: renderizar bloques externos en 'file_structure'](https://github.com/rust-lang/rust-analyzer/pull/18473)
* [rust-analyzer: muestra los valores 'estáticos' al pasar el mouse](https://github.com/rust-lang/rust-analyzer/pull/18469)
* [Rust-analyzer: Importación de autocompletar para la función y el módulo con alias](https://github.com/rust-lang/rust-analyzer/pull/18382)
* [rust-analyzer: arreglar el servidor que no cumple con el soporte de actualización de diagnóstico](https://github.com/rust-lang/rust-analyzer/pull/18432)
* [rust-analyzer: solo analiza 'seguro' como kw contextual en bloques externos](https://github.com/rust-lang/rust-analyzer/pull/18446)
* [Analizador de Rust: Analice correctamente los patrones con la tubería principal en todos los lugares](https://github.com/rust-lang/rust-analyzer/pull/18453)
* [rust-analyzer: soporta el nuevo atributo '#[rustc_intrinsic]' y cuerpos alternativos](https://github.com/rust-lang/rust-analyzer/pull/18475)

### Clasificación del rendimiento del compilador de Rust

Una semana dominada por una gran mejora y una gran regresión donde, afortunadamente, la mejora tuvo un mayor impacto. La regresión parece haber sido causada por una pelusa recién introducida que podría tener problemas de rendimiento. La mejora se produjo en la construcción de rustc con visibilidad protegida, lo que reduce el número de reubicaciones dinámicas necesarias, lo que conduce a algunas buenas ganancias de rendimiento. En una gran parte del conjunto de intérpretes, el compilador es, en promedio, un 1 % más rápido después de esta semana en comparación con la semana pasada.

Triaje realizado por **@rylev**.
Rango de revisión: [c8a8c820.. 27e38f8f](https://perf.rust-lang.org/?start=c8a8c82035439cb2404b8f24ca0bc18209d534ca&end=27e38f8fc7efc57b75e9a763d7a0ee44822cd5f7&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.8% | [0.1%, 2.0%] | 80 |
| Regresiones ❌ <br /> (secundaria) | 1.9% | [0.2%, 3.4%] | 45 |
| Mejoras ✅ <br /> (primario) | -1,9% | [-31.6%, -0.1%] | 148 |
| Mejoras ✅ <br /> (secundaria) | -5,1% | [-27.8%, -0.1%] | 180 |
| Todos ❌✅ (primarios) | -1.0% | [-31.6%, 2.0%] | 228 |

1 regresión, 1 mejora, 5 mixta; 3 de ellos en rollups
46 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/bd155e07def612eff4cb7fec391cf60f22674673/triage/2024-11-05.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [[RFC] Valores de campo predeterminados](https://github.com/rust-lang/rfcs/pull/3681)
* [RFC: Dar a los usuarios el control sobre la unificación de funciones](https://github.com/rust-lang/rfcs/pull/3692)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [Agregar soporte para 'usar Trait::func'](https://github.com/rust-lang/rfcs/pull/3591)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Estabilizar ensamblaje en línea Arm64EC](https://github.com/rust-lang/rust/pull/131781)
* [disposición: fusionar] [Estabilizar ensamblaje en línea s390x](https://github.com/rust-lang/rust/pull/131258)
* [disposición: fusionar] [rustdoc-search: simplificar las reglas para genéricos y parámetros de tipo](https://github.com/rust-lang/rust/pull/127589)
* [disposición: fusionar] [Arreglar ICE al pasar argumentos que crean DefId a legacy_const_generics.](https://github.com/rust-lang/rust/pull/130443)
* [disposición: fusionar] [Problema de seguimiento para 'const_option_ext'](https://github.com/rust-lang/rust/issues/91930)
* [disposición: fusionar] [Problema de seguimiento para const_unicode_case_lookup](https://github.com/rust-lang/rust/issues/101400)
* [disposición: fusionar] [Rechazar el tiempo de vida bruto seguido de ''', como lo hacen los tiempos de vida regulares](https://github.com/rust-lang/rust/pull/132341)
* [disposición: fusionar] [Imponer que las vidas en bruto deben ser identificadores sin procesar válidos](https://github.com/rust-lang/rust/pull/132363)
* [disposición: fusionar] [Estabilizar las características de destino 'multivalor', 'tipos de referencia' y 'llamada de cola' de WebAssembly](https://github.com/rust-lang/rust/pull/131080)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Implementar el marco de actualización para la firma de proyectos](https://github.com/rust-lang/rfcs/pull/3724)
* [new] [[RFC] Desempaquetado de argumentos de función estática](https://github.com/rust-lang/rfcs/pull/3723)
* [nuevo] [[RFC] ABI explícito en extern](https://github.com/rust-lang/rfcs/pull/3722)
* [nuevo] [Añadir RFC de homogeneous_'try_blocks'](https://github.com/rust-lang/rfcs/pull/3721)

## Próximos eventos

Eventos oxidados entre 2024-11-06 - 2024-12-04 🦀

### Virtual
* 06/11/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031651/)
* 07/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633272/)
* 08/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 12/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346985/)
* 14/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 14/11/2024 | Virtual y presencial (Lehi, UT, EE. UU.) | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Pulgar verde: Construyendo un riego de plantas habilitado para Bluetooth con Rust y microbit**](https://www.meetup.com/utah-rust/events/304206130/)
* 14/11/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Encuentro de noviembre**](https://www.meetup.com/join-srug/events/304166747/)
* 15/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 19/11/2024 | Virtual (Los Ángeles, CA, EE. UU.) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discusión - Tema: Rust para UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 19/11/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346971/)
* 20/11/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Taller de Rust incrustado**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 21/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633273/)
* 21/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**IoT confiable con Rust... ¡y contraseñas!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 21/11/2024 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 25/11/2024 | Bratislava, SK | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Charla ONLINE, patrocinada por Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 26/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 28/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 28/11/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 03/12/2024 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)

### Asia
* 28/11/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Cumbre RustTechX 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 30/11/2024 | Tokio, JP | [Rust de Tokio](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europa
* 06/11/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 06/11/2024 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 09/11/2024 - 11/11/2024 | Florencia, IT | [Laboratorio de Rust](https://rustlab.it/)
    * [**Rust Lab 2024: La Conferencia Internacional sobre la Roya en Florencia**](https://rustlab.it/schedule)
* 12/11/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN habla de noviembre de 2024 sobre la adquisición de RustRover con JetBrains**](https://www.meetup.com/rust-london-user-group/events/304378534/)
* 12/11/2024 | Zúrich, CH | [Rust de Zúrich](https://www.meetup.com/rust-zurich/events/)
    * [**Sistemas de archivos cifrados/distribuidos, wasm-bindgen**](https://www.meetup.com/rust-zurich/events/304162840)
* 13/11/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 14/11/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 19/11/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 19/11/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #72**](https://www.meetup.com/rust-paris/events/304396616/)
* 21/11/2024 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 21/11/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 23/11/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Taller #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 26/11/2024 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/events/)
    * [**New Rust Warsaw Meetup #3**](https://www.meetup.com/rust-warsaw/events/304379707/)
* 27/11/2024 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 28/11/2024 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 28/11/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 28/11/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421381/)
* 28/11/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn con Mainmatter y Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 28/11/2024 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (noviembre de 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 04/12/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)

### América del Norte
* 07/11/2024 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/)
    * [**Chicago Rust Meetup**](https://www.meetup.com/chicago-rust-meetup/events/304327595/)
* 07/11/2024 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/)
    * [**Noviembre Social Mensual**](https://www.meetup.com/rust-montreal/events/304248702/)
* 07/11/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Desarrollo de juegos con Rust y el motor Bevy**](https://www.meetup.com/stl-rust/events/302371464/)
* 12/11/2024 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 12/11/2024 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Reunión mensual de Rust NYC**](https://www.meetup.com/rust-nyc/events/304358109/)
* 14/11/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/hackerdojo/events/304211045/)
* 15/11/2024 | Ciudad de México, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 15/11/2024 | Somerville, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, 15 de noviembre**](https://www.meetup.com/bostonrust/events/303708398/)
* 19/11/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638252/)
* 23/11/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust común de Boston, 23 de noviembre**](https://www.meetup.com/bostonrust/events/303708407/)
* 25/11/2024 | Ferndale, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 27/11/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)

### Oceanía
* 12/11/2024 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Rust en Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/304029765/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1gf5ue1/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Cualquier proyecto de C suficientemente complicado contiene una implementación adhoc, informalmente especificada, plagada de errores y lenta de la mitad de la carga.

– [Folkert de Vries en RustNL 2024 (grabación en youtube)](https://www.youtube.com/watch?v=mvzHQdCLkOY&t=912s)

¡Gracias a [Collin Richards](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1629) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1glf0e4/this_week_in_rust_572/)</small>
