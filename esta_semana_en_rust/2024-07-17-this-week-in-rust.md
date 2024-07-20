---
title: "Esta semana en Rust #27"
number_of_week: 27
description: El crate de esta semana es cargo-wizard, un subcomando de carga que aplica plantillas de perfil y configuración a su proyecto Cargo para configurarlo para obtener el máximo rendimiento, tiempos de compilación rápidos o un tamaño binario mínimo.
date: 2024-07-17
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

### Boletines informativos
* [Este mes en Rust OSDev: junio de 2024](https://rust-osdev.com/this-month/2024-06/)

### Actualizaciones de proyectos/herramientas
* [Zed: ¿Linux cuándo? Linux ahora.](https://zed.dev/blog/zed-on-linux)
* [Publicado r3bl_terminal_async v0.5.4](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#r3bl_terminal_async)
* [Publicado r3bl_test_fixtures v0.0.1](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#r3bl_test_fixtures)
* [ 📦 Cratery, un registro de carga privado liviano con baterías incluidas, construido para organizaciones, ahora es de código abierto](https://github.com/cenotelie/cratery)

### Observaciones/Pensamientos
* [Las piezas que faltan en la carga](https://weihanglo.tw/posts/2024/the-missing-parts-in-cargo/)
* [Cómo funciona HappyLock](https://www.botahamec.dev/blog/how-happylock-works.html)
* [Implementación de un analizador de rango genérico en Rust](https://blog.veeso.dev/blog/en/implementing-a-generic-range-parser-in-rust/)
* [video] [Mi patrón de diseño favorito de Rust](https://www.youtube.com/watch?v=qrf52BVaZM8)
* [audio] [Ingeniería de fusión con Jakub Valtar](https://corrode.dev/podcast/s02e06-fusion-engineering/)
* [audio] [En el camino: RustNL y oxida](https://jamesmunns.com/podcast/017-on-the-road/)

### Tutoriales de Rust
* [Registro Global](https://donsz.nl/blog/global-registration/)
* [Gray-Scott con Rust](https://grayscott-with-rust-grasland-5e6591fc7054976525da4f6c87122ea76c.pages.in2p3.fr/)
* [Cómo hacer la aplicación de escritorio Rust con Egui y ChatGPT](https://www.onlycoiners.com/user/steadylearner/blog/how-to-make-rust-desktop-app-with-egui-and-chatgpt)
* [Escribir el programa eBPF Tracepoint con Rust Aya: consejos y ejemplo](https://yuki-nakamura.com/2024/07/06/writing-ebpf-tracepoint-program-with-rust-aya-tips-and-example/)
* [Sorpresas con el 'as' de Rust (y la división de Python)](https://annahope.me/blog/rust-as/)
* [Construir con Naz: Exploración de cajas y pines en Rust](https://developerlife.com/2024/07/16/pin-box-dynamic-duo)
* [Cómo organizar grandes bases de código de Rust](https://kerkour.com/rust-how-to-organize-large-workspaces)
* [Tocando tablaturas de guitarra en Rust](https://agourlay.github.io/ruxguitar-tablature-player/)

### Investigación
* [Un estudio empírico de Rust-for-Linux: el éxito, la insatisfacción y el compromiso](https://www.usenix.org/conference/atc24/presentation/li-hongyu)
* [Llevando el Rust a los sistemas críticos para la seguridad en el espacio](https://arxiv.org/abs/2405.18135)

### Miscelánea
* [audio] [RustShip: Rust en AWS Lambda con Luciano Mammino](https://ieni.dev/2024/07/%EF%B8%8F-rust-on-aws-lambda-with-luciano-mammino-rustship-8/)

## Crate de la semana

El crate de esta semana es [cargo-wizard](https://github.com/Kobzol/cargo-wizard), un subcomando de carga que aplica plantillas de perfil y configuración a su proyecto Cargo para configurarlo para obtener el máximo rendimiento, tiempos de compilación rápidos o un tamaño binario mínimo.

¡Gracias a [Jakub Beránek](https://users.rust-lang.org/t/crate-of-the-week/2704/1322) por la sugerencia!

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

Se [fusionaron 385 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-09..2024-07-16

* [añadir AMX target-features y bandera de características 'x86_amx_intrinsics'](https://github.com/rust-lang/rust/pull/126639)
* [añadir 'constness' a 'TraitDef'](https://github.com/rust-lang/rust/pull/127200)
* [Agregar clasificación y métodos relacionados para F16 y F128](https://github.com/rust-lang/rust/pull/127020)
* [Agregue lint para etiquetas ASM en línea que parecen binarias](https://github.com/rust-lang/rust/pull/126922)
* [añadir sugerencias para las posibles palabras clave 'fn', struct' o enum' que faltan](https://github.com/rust-lang/rust/pull/127419)
* [añadida la función de destino 'XOP' y la puerta de la función 'xop_target_feature'](https://github.com/rust-lang/rust/pull/127209)
* [permite que '#[diagnostic::d o_not_recommend]' suprima también las implicaciones de rasgos en las sugerencias](https://github.com/rust-lang/rust/pull/127598)
* [se contamina automáticamente al informar errores de ItemCtxt](https://github.com/rust-lang/rust/pull/127358)
* [evite el error "sin campo" e ICE en la variante ADT recuperada](https://github.com/rust-lang/rust/pull/127575)
* [evite errores de seguimiento e ICEs después de errores de vida perdidos en las estructuras de datos](https://github.com/rust-lang/rust/pull/127311)
* [marque 'is_ident' antes de 'parse_ident'](https://github.com/rust-lang/rust/pull/127601)
* [borrar 'inner_attr_ranges' regularmente](https://github.com/rust-lang/rust/pull/127477)
* [consolidar el informe de errores de la región en 'rustc_infer'](https://github.com/rust-lang/rust/pull/127500)
* [cobertura: restringir la simplificación de 'ExpressionUsed' a las asignaciones de 'Código'](https://github.com/rust-lang/rust/pull/127758)
* [asegúrese de que los flotantes sean devueltos sin pérdidas por la ABI de Rust en x86 de 32 bits](https://github.com/rust-lang/rust/pull/123351)
* [dispara pelusa 'unsafe_code' en bloques externos inseguros](https://github.com/rust-lang/rust/pull/127535)
* [arreglar 'DebugParser'](https://github.com/rust-lang/rust/pull/127273)
* [arreglar sugerencia de importación hielo](https://github.com/rust-lang/rust/pull/127310)
* [corregir el manejo incorrecto de NDEBUG en los enlaces LLVM](https://github.com/rust-lang/rust/pull/127654)
* [Arreglar la salida intercalada en el gancho de pánico predeterminado cuando varios hilos entran en pánico simultáneamente](https://github.com/rust-lang/rust/pull/127397)
* [corregida la regresión en la disminución del MIR de los patrones or](https://github.com/rust-lang/rust/pull/127028)
* [puerta la verificación del límite de longitud de tipo detrás de una bandera nocturna](https://github.com/rust-lang/rust/pull/127670)
* [generalizar 'asignador fn' para Rc/Arc](https://github.com/rust-lang/rust/pull/124980)
* [generalizar el gráfico de búsqueda para habilitar el fuzzing](https://github.com/rust-lang/rust/pull/127627)
* [protégete contra llamar a 'libc::exit' varias veces en Linux](https://github.com/rust-lang/rust/pull/126606)
* [Implementa lint simple e inestable para sugerir convertir el cierre del bloque asíncrono en un cierre asíncrono](https://github.com/rust-lang/rust/pull/127097)
* [Vuelva a instanciar los objetivos mejor clasificados en la selección de candidatos](https://github.com/rust-lang/rust/pull/127568)
* [hacer que la impl de 'visit_clobber' sea segura](https://github.com/rust-lang/rust/pull/127560)
* [hacer que las sugerencias de errores de análisis sean detalladas y corregir intervalos](https://github.com/rust-lang/rust/pull/127407)
* [Asegúrese de que las etiquetas se definan después del intervalo principal en los diagnósticos](https://github.com/rust-lang/rust/pull/127591)
* [marque 'builtin_syntax' como interno](https://github.com/rust-lang/rust/pull/127622)
* [lint de migración para 'expr2024' para la edición 2024](https://github.com/rust-lang/rust/pull/125627)
* [más sugerencia para convertir 'Opción<&Vec<T>>' a 'Opción<&[T]>'](https://github.com/rust-lang/rust/pull/127596)
* [Más errores de rasgo reelaborando](https://github.com/rust-lang/rust/pull/127495)
* [solo pista lugares mencionados para el enhebrado de salto](https://github.com/rust-lang/rust/pull/127087)
* [sugerir tomar prestado el argumento fn que es 'impl AsRef'](https://github.com/rust-lang/rust/pull/124599)
* [sugerir usar 'map_or' cuando 'Opción<&T>::unwrap_or donde falla T: Deref'](https://github.com/rust-lang/rust/pull/127629)
* [sugerir el uso de una captura precisa para el tipo oculto que captura la región](https://github.com/rust-lang/rust/pull/127619)
* [use el estilo detallado al sugerir cambiar 'const' por 'let'](https://github.com/rust-lang/rust/pull/127382)
* [miri: TB: reservado + Protegido + IM + perezoso es una combinación horrible que no debería existir](https://github.com/rust-lang/miri/pull/3742)
* [miri: implementar soporte para múltiples destructores TLS en macOS](https://github.com/rust-lang/miri/pull/3739)
* [miri: implementar las funciones 'os_unfair_lock' en macOS](https://github.com/rust-lang/miri/pull/3745)
* [arreglar 'Parser::look_ahead'](https://github.com/rust-lang/rust/pull/127636)
* [estabilizar 'const_cstr_from_ptr (CStr::from_ptr, CStr::count_bytes)'](https://github.com/rust-lang/rust/pull/127433)
* [estabilizar 'io_slice_advance'](https://github.com/rust-lang/rust/pull/127661)
* [Estabilizar la conversión no controlada de Const de U32 a Char](https://github.com/rust-lang/rust/pull/126958)
* [añadir 'f16' y 'f128' como tipos simd en LLVM](https://github.com/rust-lang/rust/pull/127487)
* [añadir caché para 'allocate_str'](https://github.com/rust-lang/rust/pull/127638)
* ['offset_from': permitir siempre que los punteros apunten a la misma dirección](https://github.com/rust-lang/rust/pull/124921)
* [std: '#! [deny(unsafe_op_in_unsafe_fn)]' en código independiente de la plataforma](https://github.com/rust-lang/rust/pull/127744)
* [std: Elimina el manejo de casos extremos de la función de familia logaritmos para Solaris](https://github.com/rust-lang/rust/pull/127719)
* [impl FusedIterator y una sugerencia de tamaño para el iter de fuentes de error](https://github.com/rust-lang/rust/pull/127091)
* [use 'pidfd_spawn' para un desove de proceso más rápido cuando se solicita un PidFd](https://github.com/rust-lang/rust/pull/126827)
* [Haga que OS/Windows y PAL/Windows estén predeterminados en '#! [denegar(unsafe_op_in_unsafe_fn)]'](https://github.com/rust-lang/rust/pull/127750)
* [Windows: Agregue soporte experimental para vincular DLL del sistema requeridos por std usando raw-dylib](https://github.com/rust-lang/rust/pull/127370)
* [Windows: eliminar algunos alias de tipo innecesarios](https://github.com/rust-lang/rust/pull/127712)
* [exponiendo STARTUPINFOW.wShowWindow en el rasgo CommandExt](https://github.com/rust-lang/rust/pull/126690)
* [cargo: 'docs(ref)': Nota MSRV para características en los docs](https://github.com/rust-lang/cargo/pull/14224)
* [cargo: añadir 'cargo_test' al preludio de soporte de prueba](https://github.com/rust-lang/cargo/pull/14243)
* [cargo: anulaciones: No advertir sobre paquetes duplicados por usar '..'](https://github.com/rust-lang/cargo/pull/14234)
* [cargo: fuente: No advierta sobre paquetes duplicados sin referencias](https://github.com/rust-lang/cargo/pull/14239)
* [cargo: prueba: Redactar el tiempo transcurrido en el marco de tiempo de minutos](https://github.com/rust-lang/cargo/pull/14233)
* [cargo: prueba: Reducir la prescripción excesiva a la persona que llama](https://github.com/rust-lang/cargo/pull/14217)
* [cargo: añadir flujo de trabajo para publicar Cargo automáticamente](https://github.com/rust-lang/cargo/pull/14202)
* [cargo: corrección: asegúrese de que dep/feature active la dependencia en 2024](https://github.com/rust-lang/cargo/pull/14221)
* [cargo: fix: renombrar a 'rustdoc::broken_intra_doc_links'](https://github.com/rust-lang/cargo/pull/14215)
* [cargo: refactorizar: mover 'get_source_id' fuera del registro](https://github.com/rust-lang/cargo/pull/14218)
* [clippy: 'unwrap_or_default': saltar advertencia al llamar dentro de la implementación del método sugerido](https://github.com/rust-lang/rust-clippy/pull/13090)
* [clippy: añadir más doc-valid-idents](https://github.com/rust-lang/rust-clippy/pull/13093)
* [clippy: arreglar 'manual_unwrap_or' falso positivo](https://github.com/rust-lang/rust-clippy/pull/13061)
* [clippy: corrige el falso positivo de 'needless_option_as_deref' en los literales 'struct'](https://github.com/rust-lang/rust-clippy/pull/13102)
* [clippy: arreglar y renombrar 'overflow_check_conditional'](https://github.com/rust-lang/rust-clippy/pull/12944)
* [clippy: corrige la guía de 'float_cmp' y 'float_cmp_const' para no recomendar incorrectamente 'f__::EPSILON' como margen de error](https://github.com/rust-lang/rust-clippy/pull/13079)
* [clippy: hacer 'or_fun_call' recursivo](https://github.com/rust-lang/rust-clippy/pull/13085)
* [clippy: arreglar las pelusas 'doc_lazy_continuation'](https://github.com/rust-lang/cc-rs/pull/1153)
* [Rust-analyzer: agregue soporte para 'F16' y 'F128'](https://github.com/rust-lang/rust-analyzer/pull/17572)
* [rust-analyzer: codifica la crudeza ident y el tipo literal por separado en 'tt::Leaf'](https://github.com/rust-lang/rust-analyzer/pull/17559)
* [rust-analyzer: agregue diagnósticos de casos incorrectos para los campos de la variante 'enum' y todas las variables/parámetros](https://github.com/rust-lang/rust-analyzer/pull/17588)
* [rust-analyzer: añadir sugerencias de incrustación para parámetros genéricos](https://github.com/rust-lang/rust-analyzer/pull/17544)
* [rust-analyzer: no agregue una nueva 'enumeración' si ya existe](https://github.com/rust-lang/rust-analyzer/pull/17571)
* [rust-analyzer: Corregir la codificación incorrecta de literales en el proc-macro-api en la versión 4](https://github.com/rust-lang/rust-analyzer/pull/17601)
* [rust-analyzer: implementar la infraestructura de internado de símbolos](https://github.com/rust-lang/rust-analyzer/pull/17584)
* [rust-analyzer: desencadenar VSCode para renombrar después de que se aplique la asistencia de variable de extracción](https://github.com/rust-lang/rust-analyzer/pull/17587)
* [rustfmt: impl 'StyleEditionDefault' trait para todas las configuraciones](https://github.com/rust-lang/rustfmt/pull/5937)
* [rustfmt: devuelve RewriteResult para 'rewrite_block' y 'rewrite_closure'](https://github.com/rust-lang/rustfmt/pull/6235)

### Clasificación del rendimiento del compilador de Rust

Una semana bastante tranquila con las únicas regresiones puras que son pequeñas y provienen de correcciones de corrección. El mayor cambio provino de la desactivación de la verificación '-Zenforce-type-length-limit', que tuvo impactos positivos en muchos puntos de referencia diferentes, ya que el compilador está haciendo estrictamente menos trabajo.

Triaje realizado por **@rylev**.
Rango de revisión: [a2d58197.. 5572759B](https://perf.rust-lang.org/?start=a2d58197a766085856504328948c89a33a6a36e8&end=5572759b8d7012fa34eba47f4885c76fa06d9251&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.3% | [0.2%, 0.7%] | 12 |
| Regresiones ❌ <br /> (secundaria) | 0.4% | [0.2%, 0.9%] | 45 |
| Mejoras ✅ <br /> (primario) | -0,7% | [-1.5%, -0.2%] | 37 |
| Mejoras ✅ <br /> (secundaria) | -3,3% | [-13.5%, -0.4%] | 21 |
| Todos ❌✅ (primarios) | -0,4% | [-1,5%, 0,7%] | 49 |

2 regresiones, 3 mejoras, 2 mixtas; 1 de ellos en rollups
56 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d86903679ac12804e7b15d9007e2539c0b541dc6/triage/2024-07-16.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:
* [Ergonomía del partido 2024](https://github.com/rust-lang/rfcs/pull/3627)
* [Notación de tipo de retorno (RTN)](https://github.com/rust-lang/rfcs/pull/3654)
* [#[derivar(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
*Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Corregir la falta de solidez del tipo asociado al superrasgo](https://github.com/rust-lang/rust/pull/126090)
* [disposición: fusionar] [Manejar amablemente 'Drop' implica introducir parámetros más genéricos que el ADT](https://github.com/rust-lang/rust/pull/127220)
* [disposición: fusionar] [Reordenar los modificadores ligados a rasgos *después de* el aglutinante 'para<...>' en los límites de rasgos](https://github.com/rust-lang/rust/pull/127054)
* [disposición: fusionar] [size_of_val_raw: para la longitud 0 es seguro llamar](https://github.com/rust-lang/rust/pull/126152)
* [disposición: fusionar] [Implementar lint contra literales negativos ambiguos](https://github.com/rust-lang/rust/pull/121364)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hubo problemas de seguimiento de equipos lingüísticos ni relaciones públicas en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el período final de comentarios esta semana.*

##### [Directrices para códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro o PR ingresados al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC para objetivos del proyecto](https://github.com/rust-lang/rfcs/pull/3672)

## Próximos eventos

Eventos de Rusty entre 2024-07-17 - 2024-08-14 🦀

### Virtual
* 17/07/2024| Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/302139632/)
* [**Rust for Rustaceans Book Club: Capítulo 10: Concurrencia (y paralelismo)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 17/07/2024 | Híbrido - Virtual y Presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298488824/)
* 18/07/2024 | Virtual (IL) | [Rust en Israel](https://www.meetup.com/rust-in-israel/)
    * [**Hilos Rust (Virtual) - תהליכונים בראסט - מפגש בזום**](https://www.meetup.com/rust-in-israel/events/302219468/)
* 18/07/2024 | Virtual (Róterdam, Países Bajos)| [Desarrollo de juegos Bevy](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #5**](https://www.meetup.com/bevy-game-development/events/301711262/)
* 23/07/2024 | Híbrido: virtual y presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/301062840/)
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
* 13/08/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346978/)

### África
* 02/08/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)

### Asia
* 20/07/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro 🤝 rustáceo de julio de 2024 C4GT**](https://hasgeek.com/rustbangalore/july-2024-rustacean-meetup-c4gt/)

### Europa
* 17/07/2024 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/302024746/)
* 18/07/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2024**](https://www.meetup.com/rust-bern/events/301952761/)
* 23/07/2024 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/301461206/)
* 23/07/2024 | Híbrido: virtual y presencial (Múnich/Múnich, DE) | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - híbrido**](https://www.meetup.com/rust-munich/events/298507657/)
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

### América del Norte
* 17/07/2024 | Híbrido - Vancouver, Columbia Británica, CA | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 18/07/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: patrón de espera**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 18/07/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/301883176/)
* 21/07/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Back Bay Rust, 21 de julio**](https://www.meetup.com/bostonrust/events/301550076/)
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

> tengo un sueño. Un sueño que Cargo tenga su propia cadencia de lanzamiento, por lo que está libre de la estricta maldición de la estabilidad y luego puede enviar lanzamientos de versiones principales.

– [Weihang Lo en su blog](https://weihanglo.tw/posts/2024/the-missing-parts-in-cargo/)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1591) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1e63esr/this_week_in_rust_556/)</small>
