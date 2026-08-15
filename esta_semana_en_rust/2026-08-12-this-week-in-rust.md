---
title: "Esta semana en Rust #122"
number_of_week: 122
description: El crate de esta semana es literator, una caja para mostrar eficientemente los objetos de un iterador sin asignaciones temporales. 
date: 2026-08-12
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *Esta Semana en Rust*! 
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software fiable y eficiente. 
Este es un resumen semanal de su progreso y comunidad. 
¿Quieres que se mencione algo? Etiquetanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[mándanos una solicitud de retirada](https://github.com/rust-lang/this-week-in-rust). 
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md). 

*This Week in Rust* está desarrollado abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos pueden consultarse en [this-week-in-rust.org](https://this-week-in-rust.org/). 
Si encuentras algún error en el número de esta semana, [por favor presenta un RP](https://github.com/rust-lang/this-week-in-rust/pulls). 

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485). 

## Actualizaciones de la comunidad Rust

<!-- Queridos colaboradores de la comunidad: Por favor, leed README.md para orientarse sobre las aportaciones. Cada enlace enviado debe ser de la forma: * [Título de la página enlazada](https://example.com/my_article) Si añades un enlace a un contenido no textual, por favor prefijadlo con '[vídeo]' o '[audio]': * [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article) * [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todas formas y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial

* [Llamada para pruebas: Restringiendo la implementabilidad de rasgos y mutabilidad de campo](https://blog.rust-lang.org/inside-rust/2026/08/10/call-for-testing-impl-and-mut-restrictions/)

### Fundación

* [Cumbre de Salud del Equipo Rust: 8 de septiembre en Montreal](https://rustfoundation.org/media/rust-teams-health-summit-september-8-in-montreal/)
* [Poniéndose al día con el equipo de contenido de Rust: entrevistas de RustWeek y una nueva serie de registros](https://rustfoundation.org/media/catching-up-with-the-rust-content-team-rustweek-interviews-and-a-new-changelog-series/)

### Boletines

* [Este mes en Rust OSDev: julio 2026](https://rust-osdev.com/this-month/2026-07/)

### Actualizaciones de proyectos/herramientas

Estamos realizando cambios en la sección de Actualizaciones de Proyectos/Herramientas - véase [aquí](https://github.com/rust-lang/this-week-in-rust/issues/8575) para más detalles

* [Sexto cumpleaños de Bevy](https://bevy.org/news/bevys-sixth-birthday/)
* [fearless_simd v0.7: enteros de 64 bits, genéricos mejorados, SSE2 y próximas versiones 1.0](https://linebender.org/blog/fearless-simd-0-7/)
* [vairedb 0.1.0 - Base de datos analítica distribuida nativa en la nube](https://github.com/matteobovetti/vairedb/releases/tag/v0.1.0)
* [OXVG 0.0.7: introducción de un transformador SVG a JSX en la cadena de herramientas OXVG](https://github.com/noahbald/oxvg/releases/tag/v0.0.7)
* [HTML, JavaScript, CSS deberían haber muerto hace tiempo](https://dev.to/zionsati/html-javascript-css-should-have-died-long-ago-1mm3)
* [Verificación aún más formal para BPF](https://lwn.net/SubscriberLink/1087069/d25c9e5027849a8a/)
* [Kache 0.14.0: restauraciones depurables y convergencia entre clones](https://github.com/kunobi-ninja/kache/releases/tag/v0.14.0)
* [kobe 0.39.0: endurecimiento del ciclo de vida del arrendamiento de clúster](https://github.com/kunobi-ninja/kobe/releases/tag/v0.39.0)
* [renovar 0.1.1: un motor de juego determinista y basado en el código](https://github.com/renew-engine/renew/releases/tag/v0.1.1)
* [GRIT 1.1: comprueba tus tensores cuantizados](https://singhpratech.github.io/grit-datatype/)
* [floDl: Introducción a soporte para GPU AMD](https://flodl.dev/blog/making-room)
* [git-cache-proxy: caché de solo lectura para git](https://rolandsdev.blog/posts/caching-git-clones-across-a-slow-network/)

### Observaciones/Pensamientos

* [Una visión para la carga](https://epage.github.io/blog/2026/08/cargo-vision/)
* [Implementaciones de rasgos cílicos: motivación](https://smallcultfollowing.com/babysteps/blog/2026/08/10/cyclic-trait-solving/)
* [SIMD de Rust en la GPU](https://www.vectorware.com/blog/simd-on-gpu/)
* [Reescritura en Rust: Rendimiento, Fracasos, Reality Check 2026](https://blog.jetbrains.com/rust/2026/08/10/rewriting-in-rust/)
* [RangeFrom, Parte 1..: Historia y antecedentes](https://erk.dev/2026/08/12/rangefrom-part-1)
* [Conversaciones mecanografiadas: Hacen que los diálogos de agentes ilegales sean irrepresentables](https://dmitrii.app/typed-conversations-make-illegal-agent-dialogues-unrepresentable/)
* [ECQV: certificados implícitos, y por qué los dejé fuera del proyecto que los motivó](https://Abdk4Moura.github.io/post.html?post=2026-08-09-ecqv.md)
* [Apretones de manos TLS: Medición del rendimiento de 4 bibliotecas de criptografía](https://c410-f3r.github.io/thoughts/tls-handshakes-measuring-the-performance-of-4-cryptography-libraries/)
* [Construcción de servicios backend escalables con Rust y PostgreSQL](https://kerkour.com/rust-scalable-backend-services)
* [PoC para la suite de pruebas HAL de hardware en el bucle universal — Tweede golf](https://tweedegolf.nl/en/blog/240/PoC-for-universal-hardware-in-the-loop-HAL-test-suite/)
* [vídeo] [FLOSS 878 - Una herramienta con opiniones](https://www.youtube.com/watch?v=ah11nzclXag)

### Guías de Rust

* [Arcos descendentes en Rust](https://ashdnazg.github.io/articles/26/Downcasting-Arcs-in-Rust)
* [Perfilando Rust con hotpath-rs: La guía completa - De consultas SQL a muestreo de CPU](https://hotpath.rs/blog/profiling-rust-guide)
* [Una arquitectura independiente de chip para Rust incrustado en bare-metal](https://aaronqian.com/log/2026-08-01-chip-agnostic-architecture-bare-metal-rust/)

### Investigación

* [Rust Coreutils: Reconstruyendo los Fundamentos de Unix en un Lenguaje Moderno](https://arxiv.org/abs/2608.07135)

### Miscelánea

## Crate de la semana

El crate de esta semana es [literator](https://crates.io/crates/literator), una caja para mostrar eficientemente los objetos de un iterador sin asignaciones temporales. 

¡Gracias a [Nora](https://users.rust-lang.org/t/crate-of-the-week/2704/1644) por la sugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [Diésel - Mejorar la documentación de nuestros derivados](https://github.com/diesel-rs/diesel/issues/4840)
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

698 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-04..2026-08-11

#### Compilador
* [consulta el mapa de reserva antes de poner en cola el hijo en la búsqueda 'visible_parent_map' en amplitud](https://github.com/rust-lang/rust/pull/160464)
* [activar Polonius Alpha cada noche](https://github.com/rust-lang/rust/pull/159343)
* [mejora 'canonical_param_env_cache'](https://github.com/rust-lang/rust/pull/160673)
* [interpretar: saltarse comprobaciones de validez de la proyección de ref cuando no se necesitan](https://github.com/rust-lang/rust/pull/160399)
* [optimizar la resolución de la caja para un espacio de trabajo grande](https://github.com/rust-lang/rust/pull/159763)
* [optimizar la gestión de errores del solucionador](https://github.com/rust-lang/rust/pull/160160)
* [optimizar 'try_evaluate_obligations'](https://github.com/rust-lang/rust/pull/160479)
* [perf: caché ya comprobada en el visitante de privacidad](https://github.com/rust-lang/rust/pull/160317)
* [PERF: Root Fast Paths sin bloqueo para consultas de higiene](https://github.com/rust-lang/rust/pull/160494)
* [PERF: Saltarse impls extranjeros irrelevantes al construir el grafo de especialización](https://github.com/rust-lang/rust/pull/157281)
* [perf: guardar el motor de cumplimiento en línea en ObligationCtxt](https://github.com/rust-lang/rust/pull/160268)
* [resolución poco profunda de variaciones ty y const a sus vars raíz, intento 2](https://github.com/rust-lang/rust/pull/158447)
* [acelerar 'EverInitializedPlaces'](https://github.com/rust-lang/rust/pull/160033)
* [split 'apply_primary_terminator_effect'](https://github.com/rust-lang/rust/pull/160555)
* [estabilizar 'c_variadic_naked_functions'](https://github.com/rust-lang/rust/pull/159746)

#### Biblioteca
* [añadir camino rápido a 'escape_string_symbol'](https://github.com/rust-lang/rust/pull/160453)
* [núcleo: generaliza 'BorrowedCursor::ensure_init'](https://github.com/rust-lang/rust/pull/160432)
* [pista de que memchr devuelve un índice de entrada](https://github.com/rust-lang/rust/pull/159784)
* [implementa '<OnceCell,OnceLock>::new_init'](https://github.com/rust-lang/rust/pull/160881)
* [implementa 'to_string()' en 'ByteStr' y 'ByteString'](https://github.com/rust-lang/rust/pull/159300)
* [introducir un rasgo 'PinSafePointer' que generaliza 'PinCoerceUnsized'](https://github.com/rust-lang/rust/pull/156935)
* [marcar los métodos const ptr y funciones libres como 'inline(always)' para que coincidan con *mut](https://github.com/rust-lang/rust/pull/160816)
* [optimizar 'slice::contains' para tipos de BytewiseEq de un byte](https://github.com/rust-lang/rust/pull/160732)
* [conversión ASCII de paso único](https://github.com/rust-lang/rust/pull/160480)
* [estabilizar 'fs_set_times'](https://github.com/rust-lang/rust/pull/160820)

#### Carga
* ['docs(changelog)': Retira el --ítem verboso](https://github.com/rust-lang/cargo/pull/17315)
* ['docs(ref)': Añadir variables de configuración env que faltan](https://github.com/rust-lang/cargo/pull/17345)
* ['feat(log)': emitir mensaje JSON de inicio de compilación con 'run_id'](https://github.com/rust-lang/cargo/pull/16632)
* ['feat(profile)': Añadir depuración de perfil incorporada](https://github.com/rust-lang/cargo/pull/17214)
* ['feat(resolver)': Reporta la edad mínima de publicación en el mensaje de bloqueo](https://github.com/rust-lang/cargo/pull/17328)
* ['feat(toml)': permitir anular características por defecto heredadas en 2024](https://github.com/rust-lang/cargo/pull/17126)
* ['feat(trim-paths)': emitir archivos de desmapeado para artefactos finales](https://github.com/rust-lang/cargo/pull/17303)
* ['arreglar (limpiar)': respetar objetivo con --doc](https://github.com/rust-lang/cargo/pull/17322)
* ['fix(diag)': Asegurarse de que los títulos diagnósticos funcionen sin fragmentos](https://github.com/rust-lang/cargo/pull/17304)
* ['arreglar(diag)': Puerta 'blanket_hint_mostly_unused' con -Zprofile-hint-majorly-unused](https://github.com/rust-lang/cargo/pull/17313)
* ['fix(diag)': Eliminar grupos de complejidad, perfección y pelusas de vivero](https://github.com/rust-lang/cargo/pull/17307)
* ['fix(git)': Evitar usar git's core.fsmonitor](https://github.com/rust-lang/cargo/pull/17306)
* ['fijar(cerradura)': Usa más preciso 'más alto, en lugar de 'más reciente'](https://github.com/rust-lang/cargo/pull/17317)
* ['fix(resolver)': Haz la edad mínima de publicación relativa a ---tiempo de publicación](https://github.com/rust-lang/cargo/pull/17327)
* ['fijar(trim-paths): /carga/deps' fuentes de respaldo](https://github.com/rust-lang/cargo/pull/17338)
* ['fix(trim-paths)': reasignación del espacio de trabajo bajo -Zroot-dir](https://github.com/rust-lang/cargo/pull/17337)
* ['refactor(resolver): resolve()' no necesita un gctx opcional](https://github.com/rust-lang/cargo/pull/17331)
* ['revert(compilador)': bandera verbosa de reenvío a rustc para cajas locales](https://github.com/rust-lang/cargo/pull/17314)
* ['test(trim-paths)': ejercicio de desmapear archivos con depuradores](https://github.com/rust-lang/cargo/pull/17326)
* [doc: no usar la información fusionable y JSON juntos](https://github.com/rust-lang/cargo/pull/17336)
* [enlace de reparación de financiación](https://github.com/rust-lang/cargo/pull/17344)
* [refactorización: eliminar mut innecesario en las fuentes](https://github.com/rust-lang/cargo/pull/17305)
* [prueba: manejo del orden de compilación no determinista](https://github.com/rust-lang/cargo/pull/17347)

#### Rustdoc
* [Crear archivo de salida después de comprobar que el archivo markdown independiente es válido](https://github.com/rust-lang/rust/pull/160576)
* [No tener en cuenta 'doc(cfg())' al filtrar doctests](https://github.com/rust-lang/rust/pull/159014)

#### Clippy
* ['cast_possible_truncation': corrigir 'try_from' sugerencia expansión macros](https://github.com/rust-lang/rust-clippy/pull/17530)
* [no pongas pelusa 'semicolon_if_nothing_returned' en '#[automatically_derived]' ...](https://github.com/rust-lang/rust-clippy/pull/17229)
* [corrigir 'needless_range_loop' sugiere erróneamente para el índice anidado](https://github.com/rust-lang/rust-clippy/pull/16634)
* ['needless_bool': lint, la forma de guardia de regreso anticipado](https://github.com/rust-lang/rust-clippy/pull/17185)
* [nuevo LINT: operadores y métodos no nulos](https://github.com/rust-lang/rust-clippy/pull/17499)
* ['redundant_pattern_matching': sugerencia de '¡cerillas!' con paréntesis y guardados](https://github.com/rust-lang/rust-clippy/pull/17287)
* ['unwrap_or_default': respetar MSRV para el puntero crudo Default impls](https://github.com/rust-lang/rust-clippy/pull/17452)

#### Analizador de Rust
* [cuenta las continuaciones de la línea final en cadenas de bytes](https://github.com/rust-lang/rust-analyzer/pull/23032)
* [apoyo '#[rustc_must_implement_one_of]' en las asistencias](https://github.com/rust-lang/rust-analyzer/pull/23042)
* [añadir 'replace_arith_with_strict' asistencia](https://github.com/rust-lang/rust-analyzer/pull/23082)
* ['term_search' excluir tipo de objetivo inútil](https://github.com/rust-lang/rust-analyzer/pull/22994)
* [añadir paréntesis en algunos casos comunes de 'type_mismatch'](https://github.com/rust-lang/rust-analyzer/pull/23117)
* [permitir literales de 'struct' en los guardas de partido dentro de los excres 'let'](https://github.com/rust-lang/rust-analyzer/pull/23055)
* [siempre asigna consts anónimos para literales de cadenas de c/cadenas de bytes](https://github.com/rust-lang/rust-analyzer/pull/23021)
* [evitar la descoordinación de tipo de Len de la matriz con el pánico de cuerdas](https://github.com/rust-lang/rust-analyzer/pull/23019)
* [evitar escapar de vars atados producidos por 'skip_binder's infer_method_call'](https://github.com/rust-lang/rust-analyzer/pull/23059)
* [profundidad de expansión macro limitada a través de los límites del cuerpo y del bloque](https://github.com/rust-lang/rust-analyzer/pull/22974)
* [no consideres los locales de 'async fn' como upvars de la corutina devuelta](https://github.com/rust-lang/rust-analyzer/pull/23103)
* [no declarar el constructor de valor NS para variantes structs/enum si no existe](https://github.com/rust-lang/rust-analyzer/pull/23096)
* [no te pongas nervioso cuando una vida entera se transmite a una metavariable 'ident'](https://github.com/rust-lang/rust-analyzer/pull/22922)
* [arreglar el pánico de 'no se ha encontrado entrada para la llave' en VFS](https://github.com/rust-lang/rust-analyzer/pull/23120)
* [consulta de fix upvars del bloque const dentro del cierre](https://github.com/rust-lang/rust-analyzer/pull/23040)
* [diagnóstico positivo corregido para código válido](https://github.com/rust-lang/rust-analyzer/pull/23020)
* [inicializar 'macro_depth' a la profundidad macro del archivo en docs.rs y assoc.rs](https://github.com/rust-lang/rust-analyzer/pull/23034)
* [dejemos que 'Param::p arent_fn' retorne la función para los métodos BuiltinDeriveImplMethod](https://github.com/rust-lang/rust-analyzer/pull/23071)
* [normalizar tipos asociados en las comprobaciones de huérfanos](https://github.com/rust-lang/rust-analyzer/pull/23078)
* [desajustados uno por uno en los binders de Lifetime al bajar 'Dyn Trait<'a>'](https://github.com/rust-lang/rust-analyzer/pull/23107)
* [optimizar el uso de memoria del árbol de objetos](https://github.com/rust-lang/rust-analyzer/pull/23056)
* [analizar ASM en línea con palabra clave como nombre del operando](https://github.com/rust-lang/rust-analyzer/pull/23054)
* [análisis o patrón tras patrón de rango](https://github.com/rust-lang/rust-analyzer/pull/23077)
* [preservar el texto final cuando 'InsertReplaceEdit' no esté soportado](https://github.com/rust-lang/rust-analyzer/pull/23028)
* [eliminar espacios extra en firmas de función completa](https://github.com/rust-lang/rust-analyzer/pull/23070)
* [soportar macros en atributos '#[doc]' en características del IDE](https://github.com/rust-lang/rust-analyzer/pull/22899)
* [las variables de tipo no resueltas no deberían escapar a la selección impl](https://github.com/rust-lang/rust-analyzer/pull/23072)
* [al buscar un 'pub macro', considéralo disponible para revertir dependencias](https://github.com/rust-lang/rust-analyzer/pull/23036)

### Triaje de rendimiento del compilador Rust

Esta semana han llegado tantas mejoras de rendimiento nuevas que tuvimos que juntar 10 para mantener la cola de Bors manejable, ¡gran trabajo! 
También es nueva la actualización LLVM 23, que provocó enormes mejoras en tiempo de compilación, ejecución, tiempo de arranque y tamaño de artefactos. 
Estas mejoras se ven reducidas por la fusión de Polonio Alfa cada noche, provocando una regresión del 3,0%. Aún parece haber cierto potencial para mitigar esta regresión. 

Triaje hecho por **@JonathanBrouwer**. 
Rango de revisión: [65dd30fb.. 771916f9](https://perf.rust-lang.org/?start=65dd30fb9e882a7e8f0be10caca62936db2a98b8&end=771916f9028e7fe56d2685f2c4f698de5d7d6a45&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 4,6% | [0,2%, 12,8%] | 24 |
| Regresiones ❌ <br /> (secundario) | 4,3% | [0,2%, 14,3%] | 30 |
| Mejoras ✅ <br /> (primaria) | -3,3% | [-16,4%, -0,2%] | 251 |
| Mejoras ✅ <br /> (secundaria) | -5,2% | [-34,8%, -0,2%] | 308 |
| Todos ❌✅ (primario) | -2,6% | [-16,4%, 12,8%] | 275 |


1 regresión, 4 mejoras, 7 mixtas; 5 de ellas en rollups
25 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/e34d7594ad4dfdd6541038f505ec37d4602171f7/triage/2026/2026-08-09.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* [Carga: 'hints.min-opt-level'](https://github.com/rust-lang/rfcs/pull/3924)
* [Añadir 'externo "personalizado"'](https://github.com/rust-lang/rfcs/pull/3980)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [std: map ENOTSUP to ErrorKind::Unsupported- #158580](https://github.com/rust-lang/rust/pull/158580)
* [dote: añadir impls simétricos de Ecuaciones Parciales para 'Vec', '&[T]', '&mut [T]' frente a 'Vaca<'_, [T]>'](https://github.com/rust-lang/rust/pull/156160)
* [Añadir implementación 'por defecto' para 'std::sync::Once'](https://github.com/rust-lang/rust/pull/160136)
* [Extiende 'dropping_{references,copy_types}' lints a 'drop_in_place'](https://github.com/rust-lang/rust/pull/160229)
* [pelusa en usos más incorrectos de 'core::ffi::c_void'](https://github.com/rust-lang/rust/pull/159986)
* [target_features: sse (o al menos avx2) es incompatible con soft-float ABI](https://github.com/rust-lang/rust/pull/160302)
* [Hacer que let-else respete macro_rules agrupación de metavariables expr](https://github.com/rust-lang/rust/pull/158515)
* [ASM en línea PowerPC: Corregir que los floats escalares estén en el carril vectorial incorrecto en el little endian](https://github.com/rust-lang/rust/pull/160441)
* [estabilizar 'Box::take'](https://github.com/rust-lang/rust/pull/160436)
* [hacen que los cierres actúen como Quizá colgando](https://github.com/rust-lang/rust/pull/160745)
* [activar el siguiente solucionador por defecto en orphanck](https://github.com/rust-lang/rust/pull/160668)
* [Error en la proyección del tipo de no compat dyn en el antiguo solucionador de rasgos](https://github.com/rust-lang/rust/pull/154992)
* [Estabilizar '-Zprofile-sample-use'](https://github.com/rust-lang/rust/pull/155942)

<!-- Este artículo lleva varias semanas rondando. Vale borrarlo cuando desaparece online * [Nunca se rompa entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761) -->

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Dejar de usar dlltool para generar bibliotecas de importación en MinGW](https://github.com/rust-lang/compiler-team/issues/1029)
* [Promover riscv64a23-unknown-linux-gnu a Nivel 2 con herramientas de host](https://github.com/rust-lang/compiler-team/issues/1022)
* [Eliminar herramientas anfitrionas de nivel 2 'i686-pc-windows-gnu'](https://github.com/rust-lang/compiler-team/issues/1020)

<!-- Estos elementos llevan varias semanas rondando. Vale eliminarlo cuando desaparecen en línea * [Añadir 'target_feature_available_at_call_site'](https://github.com/rust-lang/compiler-team/issues/1010) * [Optimizar los enums de repr(Rust) omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922) * [Propuesta para Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841) -->

<!-- Estos elementos llevan varias semanas por ahí. Vale borrarlo cuando desaparezcan online. ##### Rust RFCs * [RFC: Refactorizar el equipo liberal](https://github.com/rust-lang/rfcs/pull/3984) * [Cargo: 'hints.min-opt-level'](https://github.com/rust-lang/rfcs/pull/3924) -->

<!-- Estos elementos llevan varias semanas rondando. Vale para eliminar cuando desaparecen online ##### Cargo * [feat(profile): Añadir depuración de perfil incorporada](https://github.com/rust-lang/cargo/pull/17214) * [feat(toml): permitir anular funciones predeterminadas heredadas en 2024](https://github.com/rust-lang/cargo/pull/17126) -->

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Asignar más fondos al equipo de financiación en 2026](https://github.com/rust-lang/leadership-council/issues/318)
* [Destina más fondos al presupuesto de viaje de 2026](https://github.com/rust-lang/leadership-council/issues/316)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Soporte a enlaces externos](https://github.com/rust-lang/rfcs/pull/3993)

## Próximos eventos

Eventos Rusty entre el 12-08-2026 - el 09-09-2026 🦀

### Virtual
* 2026-08-13 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-08-16 | Virtual (Bangalore, IN) | [Rust incrustado](https://discord.com/invite/pvYY69PvyS)
    * [**Domingos de Silicio #2**](https://discord.gg/tpsNpDHC?event=1536322186829242389)
* 2026-08-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/315604176/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, EE.UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #5 — Comunicación inalámbrica con el protocolo IEEE 802.15.4**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254775/)
* 2026-08-27 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxid**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/arkkrcj5)
* 2026-08-31 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Workshop: Añadir pruebas a un proyecto Rust de código abierto**](https://luma.com/nwfmsdtf)
* 2026-09-01 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://luma.com/d9w26vav)
* 2026-09-02 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-04 | Virtual (Girona, ES) | [Girona Oxidada](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/sqf4ux01)
* 2026-09-08 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315859305/)

### África
* 2026-09-08 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de agosto 2026**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Delhi Oxidado](https://www.meetup.com/rustdelhi)
    * [**Encuentro de Rust Delhi X SciPy India**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Computación científica en Rust y pitón**](https://scipy.in/sci-py-rs/)
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: agosto 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europa
* 2026-08-13 | Suiza, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Confía pero verifica el LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rhein-Meno Oxidado](https://www.meetup.com/rust-rhein-main)
    * [**Construcción de una cámara acústica con egui y embajada**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 27-08-2026 | Manchester, Reino Unido | [Manchester Oxidado](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)

### Norteamérica
* 2026-08-13 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de agosto de Utah Rust**](https://www.meetup.com/utah-rust/events/314696652/)
* 2026-08-13 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust August Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601099/)
* 2026-08-15 | San Francisco, CA, EE. UU. [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 3**](https://partiful.com/e/juWAwRs3XMWP7s9wLNWK)
* 18-08-2026 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997215/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-19 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Agosto de Rust en el Área de la Bahía**](https://luma.com/00f2s7q9)
* 20-08-2026 | Mountain View, CA, EE.UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315929355/)
* 2026-08-2026 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: 'Los datos moldean tu memoria' y 'Oxida en paz'](https://www.meetup.com/rust-nyc/events/316056830/)
* 2026-08-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 26-08-2026 | Austin, TX, EE.UU. [ATX Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/315171660/)
* 26-08-2026 | Los Ángeles, CA, EE.UU. [Los Ángeles Oxidado](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust LA ¡Agosto! Rust en la computación cuántica**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, EE.UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Saint Louis, MO, EE. UU. | [Rust STL](https://www.meetup.com/stl-rust/events/)
    * [**Criptografía + Ordenadores Cuánticos**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 | Montreal, QC, CA | [Fundación de Rust](https://rustfoundation.org/)
    * [**Cumbre de Salud de los Equipos Rust**](https://rustfoundation.org/event/rust-teams-health-summit/)
* 2026-09-08 - 2026-09-11 | Montreal, QC, CA | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026, organizado por la Fundación Rust**](https://rustconf.com/schedule/)
* 2026-09-09 | Montreal, QC, CA | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Encuentro de pausa café de RustConf**](https://www.meetup.com/women-in-rust/events/315773005/)

### Oceanía
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://luma.com/rustmelbourne)
    * [**Rust Melbourne Meetup**](https://luma.com/d0rndgyv)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> La discusión sobre la IA ya se ha cerrado por estar fuera de tema y descarrilando. No invoques a los moderadores tan descuidadamente, porque están enfadados y llenos de cruel venganza. 

– [Simon Buchan sobre usuarios de Rust](https://users.rust-lang.org/t/rust-being-non-standard-affects-compilers/141600/38)

¡Gracias a [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1788) por la sugerencia! 

[¡Por favor, enviad citas y votad para la semana que viene!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

Esta semana en el Rust está editado por: 

* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [Marianne Goldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

*El alojamiento de la lista de correo está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)* 

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1vn1ttk/this_week_in_rust_664/)</small>