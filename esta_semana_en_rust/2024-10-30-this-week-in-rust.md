---
title: "Esta semana en Rust #33"
number_of_week: 33
description: El crate de esta semana es tower-http-client, una biblioteca de middlewares y varias utilidades para clientes HTTP.
date: 2024-10-30
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

### Actualizaciones de proyectos/herramientas
* [Una actualización sobre los controladores de GPU Apple M1/M2](https://lwn.net/SubscriberLink/995383/34dc5950cab5e739/)
* [Anunciando Toasty, un ORM asíncrono para Rust](https://tokio.rs/blog/2024-10-23-announcing-toasty)
* [gitoxide - octubre de 2024](https://github.com/GitoxideLabs/gitoxide/discussions/1641)
* [Glues v0.4 - Soporte para MongoDB y funciones de edición de Vim](https://github.com/gluesql/glues/releases/tag/v0.4.0)
* [Meilisearch 1.11 - Mejoras en la búsqueda impulsada por IA y la búsqueda federada](https://www.meilisearch.com/blog/meilisearch-1-11)

### Observaciones/Pensamientos
* [Hacia una transmutación segura en Rust](https://lwn.net/SubscriberLink/994334/d43e27786ad96a50/)
* [El rendimiento del compilador de Rust](https://lwn.net/SubscriberLink/995125/01e2791629e8f6bd/)
* [Un nuevo enfoque para validar conjuntos de pruebas](https://lwn.net/SubscriberLink/995276/298d2f5b0be5ac34/)
* [Por qué no deberías arquear un HashMap en Rust](https://packetandpine.com/blog/arc-mutex-hashmap-rust/)
* [Implementación del rasgo de servicio de la torre](https://omarabid.com/tower-service)
* [Mejores prácticas para derivar atributos de macro en Rust](https://w-graj.net/posts/rust-derive-attribute-macros/)
* [Recortando un binario de Rust a la mitad](https://tech.dreamleaves.org/trimming-down-a-rust-binary-in-half/)
* [Una mirada profunda a nuestra nueva arquitectura masiva de múltiples inquilinos](https://turso.tech/blog/a-deep-look-into-our-new-massive-multitenant-architecture)
* [El Rust inseguro es más duro que C](https://chadaustin.me/2024/10/intrusive-linked-list-in-rust/)
* [Generadores con UnpinCell](https://without.boats/blog/generators-with-unpin-cell/)
* [¿Qué modelo de LLM es mejor para generar código Rust?](https://blog.rust.careers/post/which_llm_is_best_at_rust/)
* [Aprendizajes de la contribución al Proyecto Rust](https://blog.shrirambalaji.com/posts/oss/rust/learnings-from-contributing-to-the-rust-project)
* [Dyn Box Vs. Genéricos](https://blog.veeso.dev/blog/en/dyn-box-vs-generics-in-rust/): ¿Cuál es el mejor enfoque para lograr genéricos condicionales en Rust?

### Tutoriales de Rust
* [Compresión básica de enteros](https://blog.maguire.tech/posts/explorations/integercmp/)

### Miscelánea
* [Prisma de Rust](https://registerspill.thorstenball.com/p/rust-prism)
* [audio] [Rust vs. C++ con Steve Klabnik y Herb Sutter](https://softwareengineeringdaily.com/2024/10/23/rust-vs-c-with-steve-klabnik-herb-sutter/)
* [audio] [Novedades de Rust 1.76, 1.77 y 1.78](https://rustacean-station.org/episode/rust-1.76-1.77-1.78)
* [video] [Charla sobre la nueva pila de fuentes Rust de Chrome, fuentes](https://youtu.be/2HuxUN-mEIY?si=kj5SCc8PB5eP5K9f)
* [video] [Arquitectura de un motor de juego Rust (con Alice Cecile)](https://pod.link/developer-voices/episode/c17465dd71701f9bea9a4a4acf52423e)
* [video] [Gitoxide: Lo que es y lo que no es - Sebastian Thiel](https://www.youtube.com/watch?v=r1LwDYtghPM)

## Crate de la semana

El crate de esta semana es [tower-http-client](https://github.com/alekseysidorov/tower-http-client), una biblioteca de middlewares y varias utilidades para clientes HTTP.

¡Gracias a [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1366) por la autosugestión!

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

* [wtx - [HTTP/2] Investigar la latencia de las solicitudes](https://github.com/c410-f3r/wtx/issues/234)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 447 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-22..2024-10-29

* [añadir objetivo wasm32v1-none](https://github.com/rust-lang/rust/pull/131487)
* [AIX: use /dev/urandom para la implementación aleatoria](https://github.com/rust-lang/rust/pull/132048)
* ['rustc_target': Añadir la característica de destino pauth-lr aarch64](https://github.com/rust-lang/rust/pull/131900)
* [añadir una nota para '?' en un 'impl Future<Output = Result<..>>' en la función de sincronización](https://github.com/rust-lang/rust/pull/131549)
* [agregar soporte para los límites de elementos '~const'](https://github.com/rust-lang/rust/pull/132118)
* [considerar a los candidatos param-env incluso si tienen errores](https://github.com/rust-lang/rust/pull/132084)
* [Comprobaciones de estabilidad const v2](https://github.com/rust-lang/rust/pull/131349)
* [Cobertura: Consolidar la creación de registros de Covmap/Covfun](https://github.com/rust-lang/rust/pull/132124)
* [Cobertura: no confíe en el recorrido personalizado para encontrar bucles envolventes](https://github.com/rust-lang/rust/pull/132091)
* [cobertura: emitir intrínsecos LLVM usando el método auxiliar normal](https://github.com/rust-lang/rust/pull/132125)
* [coverage: pasar las asignaciones de cobertura a LLVM como estructuras separadas](https://github.com/rust-lang/rust/pull/131956)
* [normalizar profundamente 'TypeTrace' al informar de un error de tipo en el nuevo solucionador](https://github.com/rust-lang/rust/pull/131756)
* [denegar llamadas a métodos que no sean '#[const_trait]' en MIR constck](https://github.com/rust-lang/rust/pull/132169)
* [no eliminar el directorio '.cargo'](https://github.com/rust-lang/rust/pull/132054)
* [no se desconecte al compilador anterior cuando CI rustc esté disponible](https://github.com/rust-lang/rust/pull/132006)
* [emitir lint de incompatibilidad futura al llamar/declarar funciones con vectores que requieren la característica de destino faltante](https://github.com/rust-lang/rust/pull/127731)
* [habilitar la función LSX para objetivos LoongArch Linux](https://github.com/rust-lang/rust/pull/132140)
* [error en alineaciones mayores que 'isize::MAX'](https://github.com/rust-lang/rust/pull/131633)
* [expandir: dejar de usar artificial 'ast::Item' para macros cargadas desde metadatos](https://github.com/rust-lang/rust/pull/132192)
* [arreglar las rutas textuales de Windows cuando se usa con la macro 'include!'](https://github.com/rust-lang/rust/pull/125205)
* [hashStable for 'rustc_feature::Features': detener el hash de la constante en tiempo de compilación](https://github.com/rust-lang/rust/pull/132076)
* [lint contra obtener punteros de temporarios eliminados inmediatamente](https://github.com/rust-lang/rust/pull/128985)
* [mover 'cmp_in_dominator_order' fuera del cálculo del dominador de grafos](https://github.com/rust-lang/rust/pull/132022)
* [pasar constness con span en 'lower_poly_trait_ref'](https://github.com/rust-lang/rust/pull/132227)
* [evitar el desbordamiento de la caída de 'enumeración' del hielo](https://github.com/rust-lang/rust/pull/131909)
* [Refactorizar la detección de cambios para rustdoc y download-rustc](https://github.com/rust-lang/rust/pull/131043)
* [reemplace un enlace FTP en los comentarios con un enlace HTTPS equivalente](https://github.com/rust-lang/rust/pull/132096)
* [reemplace algunos envoltorios de LLVMRust con llamadas a la API de LLVM C](https://github.com/rust-lang/rust/pull/132167)
* [representan 'hir::TraitBoundModifiers' como partes distintas en HIR](https://github.com/rust-lang/rust/pull/131982)
* [representar la constancia del rasgo como un predicado distinto](https://github.com/rust-lang/rust/pull/131985)
* [redondea el número entero con signo negativo hacia cero en 'iN::midpoint'](https://github.com/rust-lang/rust/pull/132191)
* [simplificar la lógica de recompilación forzada para la "biblioteca"](https://github.com/rust-lang/rust/pull/132215)
* [simplificar el manejo de parámetros en 'resolve_bound_vars'](https://github.com/rust-lang/rust/pull/132043)
* [tomar una referencia sin procesar ('&raw (const|mut)') de una referencia deref de puntero ('*ptr') siempre es seguro](https://github.com/rust-lang/rust/pull/129248)
* [use 'Enabled{Lang,Lib}Feature' en lugar de n-tuplas](https://github.com/rust-lang/rust/pull/132114)
* [validar que los argumentos son correctos para 'UnevaluatedConst', 'ExistentialTraitRef'/'ExistentialProjection'](https://github.com/rust-lang/rust/pull/131049)
* [Características del objetivo x86: hacer que pclmulqdq implique SSE2](https://github.com/rust-lang/rust/pull/132174)
* [Retorno de flotador x86-32 para ABI 'Rust': trate todos los tipos de flotador de manera consistente](https://github.com/rust-lang/rust/pull/131871)
* [Miri: Añadir opción para generar informes de cobertura](https://github.com/rust-lang/miri/pull/3954)
* [Miri: Android: Añadido soporte para llamadas al sistema](https://github.com/rust-lang/miri/pull/3992)
* [Miri: Borra los errores 'eval_libc' de las cuñas de Unix](https://github.com/rust-lang/miri/pull/3984)
* [Miri: usar consistentemente manejadores de errores de E/S](https://github.com/rust-lang/miri/pull/3990)
* [Miri: Corrige el error devuelto por 'readdir_r' cuando el aislamiento está habilitado, y los usos de 'raw_os_error'](https://github.com/rust-lang/miri/pull/3995)
* [miri: implementar LLVM x86 vpclmulqdq intrínsecos](https://github.com/rust-lang/miri/pull/3987)
* [miri: indicar más explícitamente dónde cerramos los manejadores de archivos de host/dir](https://github.com/rust-lang/miri/pull/3993)
* [(Gran cambio en el rendimiento) No ejecutar lints que no puedan emitir](https://github.com/rust-lang/rust/pull/125116)
* [optimizar 'Rc<T>::d efault'](https://github.com/rust-lang/rust/pull/132031)
* [especialice 'read_exact' y 'read_buf_exact' para 'VecDeque'](https://github.com/rust-lang/rust/pull/132039)
* [Estabilizar la función 'isqrt'](https://github.com/rust-lang/rust/pull/131391)
* [estabilizar la vida útil de la cola más corta](https://github.com/rust-lang/rust/pull/131983)
* [soporte 'char::is_digit' en contextos const](https://github.com/rust-lang/rust/pull/132242)
* [eliminar la asignación 'Arc rt::init' para la información del hilo](https://github.com/rust-lang/rust/pull/123550)
* [proporciona una impl predeterminada para 'Pattern::as_utf8_pattern'](https://github.com/rust-lang/rust/pull/132113)
* [vectorizado 'SliceContains'](https://github.com/rust-lang/rust/pull/130991)
* [evite usar importaciones en 'thread_local_inner!' en estático](https://github.com/rust-lang/rust/pull/132101)
* [mejor capacidad predeterminada para 'str::replace'](https://github.com/rust-lang/rust/pull/131929)
* [musl: use 'posix_spawn' si se solicitó un cambio de directorio](https://github.com/rust-lang/rust/pull/131851)
* [resolución de carga: Hacer espacio para la resolución v3](https://github.com/rust-lang/cargo/pull/14725)
* [cargo completo: Incluir descripciones en zsh](https://github.com/rust-lang/cargo/pull/14726)
* [cargo env: eliminar clones innecesarios](https://github.com/rust-lang/cargo/pull/14730)
* [Cargo: huella dactilar: evite llamadas innecesarias](https://github.com/rust-lang/cargo/pull/14728)
* [cargo: añadida generación de esquemas inestables para Cargo.toml](https://github.com/rust-lang/cargo/pull/14683)
* [cargo: deprecate 'cargo verify-project'](https://github.com/rust-lang/cargo/pull/14736)
* [Corrección de carga: agregar información de reemplazo de fuente cuando no se encuentra ningún paquete que coincida](https://github.com/rust-lang/cargo/pull/14715)
* [Cargo fix: Trace 'config [env]' tabla en dep-info](https://github.com/rust-lang/cargo/pull/14701)
* [Prueba de carga: agregar correcciones en el resolución de satélites](https://github.com/rust-lang/cargo/pull/14707)
* [rustdoc: No considerar las funciones anidadas como función principal incluso si se llaman 'main' en doctests](https://github.com/rust-lang/rust/pull/132105)
* [rustdoc: extender 'fake_variadic' a tuplas "envueltas"](https://github.com/rust-lang/rust/pull/132115)
* [rustdoc: activos hash en tiempo de compilación de rustdoc](https://github.com/rust-lang/rust/pull/131951)
* [permitir búsqueda basada en tipos en funciones extranjeras](https://github.com/rust-lang/rust/pull/132123)
* [clippy: 'borrow_deref_ref': no activar en referencias '&raw'](https://github.com/rust-lang/rust-clippy/pull/13600)
* [clippy: no activar 'const_is_empty' para aserciones const en línea](https://github.com/rust-lang/rust-clippy/pull/13558)
* [clippy: dispara 'large_const_arrays' para longitudes de matriz calculadas](https://github.com/rust-lang/rust-clippy/pull/13620)
* [clippy: arreglar sugerencia incorrecta para '! (a >= b) como i32 == c'](https://github.com/rust-lang/rust-clippy/pull/13338)
* [clippy: arreglar el anclaje de pelusa que no funciona (generación y filtrado)](https://github.com/rust-lang/rust-clippy/pull/13588)
* [clippy: eliminar los usos innecesarios de 'filter_map'](https://github.com/rust-lang/rust-clippy/pull/13548)
* [clippy: dejar de linting 'unused_io_amount' en los rasgos de IO](https://github.com/rust-lang/rust-clippy/pull/13605)
* [Rust-analyzer: añadir ediciones de texto a más sugerencias de incrustación](https://github.com/rust-lang/rust-analyzer/pull/18376)
* [Rust-Analyzer: Implementar modelo de extracción de diagnóstico](https://github.com/rust-lang/rust-analyzer/pull/18404)
* [rust-analyzer: renderizar documentos de tipo aliasado cuando el tipo no tiene documentos](https://github.com/rust-lang/rust-analyzer/pull/18349)
* [Rust-analyzer: resuelven patrones de rango a sus estructuras](https://github.com/rust-lang/rust-analyzer/pull/18370)
* [Rust-analyzer: Divida el diagnóstico de 'macro-error' para que los usuarios puedan ignorar solo partes de él](https://github.com/rust-lang/rust-analyzer/pull/18418)
* [rust-analyzer: soporte 'cfg(true)' y 'cfg(false)'](https://github.com/rust-lang/rust-analyzer/pull/18420)
* [rust-analyzer: arreglar la configuración de habilitación de diagnóstico que se ignora](https://github.com/rust-lang/rust-analyzer/pull/18399)
* [rust-analyzer: arreglar el mensaje de sugerencia incompatible con DYN](https://github.com/rust-lang/rust-analyzer/pull/18379)
* [rust-analyzer: arreglar el formato en la página de bienvenida, ejemplo de configuración de rutas de solo lectura](https://github.com/rust-lang/rust-analyzer/pull/18407)
* [rust-analyzer: añadir las banderas CFG que faltan para la caja 'core'](https://github.com/rust-lang/rust-analyzer/pull/18395)
* [Rust-analyzer: permitir la reexportación pública de la importación de 'cajas externas'](https://github.com/rust-lang/rust-analyzer/pull/18413)
* [rust-analyzer: maneje correctamente '#""' en la edición '<2024'](https://github.com/rust-lang/rust-analyzer/pull/18417)
* [Rust-analyzer: No compute diagnósticos para archivos no locales](https://github.com/rust-lang/rust-analyzer/pull/18408)
* [rust-analyzer: arreglar la comprobación del valor 'false labelDetailsSupport'](https://github.com/rust-lang/rust-analyzer/pull/18388)
* [rust-analyzer: arreglar el análisis incorrecto de los límites de uso](https://github.com/rust-lang/rust-analyzer/pull/18371)
* [rust-analyzer: maneja las compensaciones de tiempo faltantes con gracia](https://github.com/rust-lang/rust-analyzer/pull/18386)
* [Analizador de Rust: Implemente la higiene mixta del sitio](https://github.com/rust-lang/rust-analyzer/pull/18264)
* [Rust-Analyzer: Tarea de desestructuración de clavos de una vez por todas](https://github.com/rust-lang/rust-analyzer/pull/18254)
* [Analizador de Rust: Evitar la reexportación pública de un artículo privado](https://github.com/rust-lang/rust-analyzer/pull/18390)
* [Rust-analyzer: resuelva correctamente las rutas de preludio dentro de los módulos dentro de los bloques](https://github.com/rust-lang/rust-analyzer/pull/18422)
* [rust-analyzer: pon '|' inicial en patrones debajo de 'OrPat'](https://github.com/rust-lang/rust-analyzer/pull/18419)
* [rust-analyzer: convierte "Eliminar 'dbg!'" en una solución rápida para una mejor priorización](https://github.com/rust-lang/rust-analyzer/pull/18415)
* [Rust-analyzer: mover text-edit a ide-db](https://github.com/rust-lang/rust-analyzer/pull/18421)
* [Rust-analyzer: solo construya un resolver en macro descensión cuando sea necesario](https://github.com/rust-lang/rust-analyzer/pull/18409)
* [rust-analyzer: orden de llamada de consulta de intercambio en 'file_item_tree_query'](https://github.com/rust-lang/rust-analyzer/pull/18392)

### Clasificación del rendimiento del compilador de Rust

Esta semana ha habido mucha actividad, tanto en el lado de las regresiones como en el de las mejoras. Había una gran
regresión, que se revirtió de inmediato. En general, la semana terminó siendo positiva, gracias a
un PR acumulativo que provocó una pequeña mejora en casi todos los puntos de referencia.

Triaje realizado por **@kobzol**.
Rango de revisión: [3e33bda0.. c8a8c820](https://perf.rust-lang.org/?start=3e33bda0326586a6e1e34d0f5c060ca6d116e6a4&end=c8a8c82035439cb2404b8f24ca0bc18209d534ca&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.7% | [0.2%, 2.7%] | 15 |
| Regresiones ❌ <br /> (secundaria) | 0.8% | [0.1%, 1.6%] | 22 |
| Mejoras ✅ <br /> (primario) | -0,6% | [-1.5%, -0.2%] | 153 |
| Mejoras ✅ <br /> (secundaria) | -0,7% | [-1.9%, -0.1%] | 80 |
| Todos ❌✅ (primarios) | -0,5% | [-1.5%, 2.7%] | 168 |

6 regresiones, 6 mejoras, 4 mixtas; 6 de ellos en rollups
58 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/1d21b2ec1d00198e99c8e75edf0a303955b8054c/triage/2024-10-29.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Decidir si los bloques dentro de 'asm' goto deben ser seguros por defecto](https://github.com/rust-lang/rust/issues/132078)
* [disposición: fusionar] [#[inline(never)] no funciona para funciones asíncronas](https://github.com/rust-lang/rust/issues/129347)
* [disposición: no especificada] [Agregar implementaciones de LowerExp y UpperExp a NonZero](https://github.com/rust-lang/rust/pull/131377)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Coincidencia etiquetada](https://github.com/rust-lang/rfcs/pull/3720)
* [nuevo] [RFC: Nunca patrones](https://github.com/rust-lang/rfcs/pull/3719)
* [nuevo] [[RFC] Permitir que los tipos empaquetados contengan transitivamente tipos alineados](https://github.com/rust-lang/rfcs/pull/3718)
* [nuevo] [[RFC] Modificadores de objetivo](https://github.com/rust-lang/rfcs/pull/3716)

## Próximos eventos

Eventos oxidados entre 2024-10-30 - 2024-11-27 🦀

### Virtual
* 31/10/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Elaborando intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 31/10/2024 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 01/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbcb/)
* 02/11/2024 | Virtual( Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
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
* 21/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**IoT confiable con Rust... ¡y contraseñas!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 21/11/2024 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 26/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)

### Europa
* 30/10/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Octubre 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 31/10/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/300820289/)
* 31/10/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #52 patrocinado por Trifork y OpenZeppelin**](https://www.meetup.com/copenhagen-rust-community/events/303041084/)
* 05/11/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #10: Rust <3 Nix**](https://www.meetup.com/copenhagen-rust-community/events/304237226/)
* 06/11/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 06/11/2024 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 12/11/2024 | Zúrich, CH | [Rust de Zúrich](https://www.meetup.com/rust-zurich/events/)
    * [**Sistemas de archivos cifrados/distribuidos, wasm-bindgen**](https://www.meetup.com/rust-zurich/events/304162840)
* 13/11/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 14/11/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 19/11/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 21/11/2024 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 21/11/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 23/11/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Taller #3**](https://www.meetup.com/rust-basel/events/303714372/)

### América del Norte
* 30/10/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Workshop: desplegando tu código**](https://www.meetup.com/deep-dish-rust/events/304071348/)
* 31/10/2024 | Mountain View, CA, EE. UU. | [Encuentro de Rust en Mountain View](https://www.meetup.com/rust-study-group/events/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/304272695/)
* 04/11/2024 | Brookline, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust en Coolidge Corner Brookline, 4 de noviembre**](https://www.meetup.com/bostonrust/events/303708387/)
* 07/11/2024 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/)
    * [**Noviembre Social Mensual**](https://www.meetup.com/rust-montreal/events/304248702/)
* 07/11/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Desarrollo de juegos con Rust y el motor Bevy**](https://www.meetup.com/stl-rust/events/302371464/)
* 12/11/2024 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
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
* 31/10/2024 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Rust on AWS: Sustainability + Peace: Zero Stress Automation**](https://www.meetup.com/rust-akl/events/303824919/)
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

> Un esfuerzo serio para perseguir \[[P1179R1](https://wg21.link/p1179r1)\] como un TS de por vida\[[P3465R0](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2024/p3465r0.pdf)\] comprometerá los principios básicos obsoletos e inviables de C++ y adoptará mecanismos más parecidos a los de Rust. En el negocio de los compiladores, esto se llama "carcinización": una tendencia de los organismos que no son cangrejos a desarrollar características similares a las de los cangrejos.
– [Sean Baxter en circle-lang.org](https://www.circle-lang.org/draft-profiles.html#carcinization)

¡Gracias a [Collin Richards](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1627) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1gg2v76/this_week_in_rust_571/)</small>
