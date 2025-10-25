---
title: "Esta semana en Rust #81"
number_of_week: 81
description: El crate de esta semana es extend\_mut, una biblioteca para extender de forma segura la vida útil de una referencia exclusiva bajo algunas restricciones. P
date: 2025-10-22
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software confiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíenos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!--

Estimados colaboradores de la comunidad:
Lea README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe tener la siguiente forma:

* [Título de la página enlazada](https://example.com/my_article)

Si agrega un enlace a un contenido que no es de texto, prefije el prefijo '[video]' o '[audio]':

* [video] [Título del video vinculado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio vinculado](https://example.com/my_podcast)

Si no sabe qué categoría usar, no dude en enviar un PR de todos modos
y solo pida a los editores que seleccionen la categoría.

-->

### Oficial
* [Anuncio de los nuevos directores del proyecto Rust](https://blog.rust-lang.org/2025/10/15/announcing-the-new-rust-project-directors-2025/)
* [docs.rs: objetivos predeterminados cambiados](https://blog.rust-lang.org/2025/10/16/docsrs-changed-default-targets/)
* [Resumen del equipo de infraestructura 2025 Q3 y plan Q4](https://blog.rust-lang.org/inside-rust/2025/10/16/infrastructure-team-q3-recap-and-q4-plan/)
* [Cambiar el nombre de la rama predeterminada de rust-lang/rust](https://blog.rust-lang.org/inside-rust/2025/10/16/renaming-the-default-branch-of-rust-langrust/)

### Boletines
* [Computación científica en Rust #11 (octubre de 2025)](https://scientificcomputing.rs/monthly/2025-10)

### Actualizaciones de proyectos/herramientas
* [SeaORM 2.0: nuevo formato de entidad y capacidades relacionales](https://www.sea-ql.org/blog/2025-10-20-sea-orm-2.0/)
* [Lanzamiento de Slint 1.14](https://slint.dev/blog/slint-1.14-released)
* [Mensajería del Danubio - nueva arquitectura de persistencia de temas (Wal + Cloud)](https://danube-docs.dev-state.com/architecture/persistence/)
* [SierraDB: un almacén de eventos distribuido integrado en Rust](https://tqwewe.com/blog/building-sierradb/)
* [Anuncio de C2Rust v0.21](https://immunant.com/blog/2025/10/c2rust_release/)
* [Redacción del proyecto 'Shove'](https://maguire.tech/posts/shove/)
* [ServiceRadar - Gestión y observabilidad de redes de código abierto](https://news.ycombinator.com/item?id=45624186) 
* [¡Se lanza fzf-make v0.65.0! (Una herramienta de línea de comandos que ejecuta comandos usando el buscador difuso)](https://github.com/kyu08/fzf-make/releases/tag/v0.65.0)
* [Informe de auditoría del Código Diesel](https://diesel.rs/assets/NGICore%20Diesel%20penetration%20test%20report%202025%201.0.pdf)

### Observaciones/Pensamientos
* [Git considera SHA-256, Rust, LLM y más](https://lwn.net/SubscriberLink/1042172/c7e1cdef4a518cc3/)
* [DebugFS en Rust](https://lwn.net/SubscriberLink/1041095/2ef0281b0fec4d9d/)
* [Interoperabilidad de Python y Rust](https://medium.com/google-cloud/python-and-rust-interoperability-a-walkthrough-for-building-a-high-performance-mcp-server-56c04e4b651b)
* [Destrucción controlada en Rust: hacia una caída asíncrona y una gestión de recursos más segura](https://smallcultfollowing.com/babysteps/blog/2025/10/21/move-destruct-leak/)
* [¡Todo el mundo es tan creativo!](https://daymare.net/blogs/everbody-so-creative/)
* [Cómo organizamos la congelación de funciones de Rust Clippy](https://blog.goose.love/posts/organizing-a-feature-freeze/)
* [Generalizando sobre la mutabilidad en Rust](https://alexsaveau.dev/blog/tips/generalizing-over-mutability-in-rust)
* [audio] [Netstack.FM Episodio 10 – zerocopy con Joshua Liebow-Feeser](https://netstack.fm/#episode-10)

### Tutoriales de Rust
* [Resultados en tiempo real en un motor de consulta federado](https://blog.vega.io/posts/partial_stream/)
* [Axum: Multi-tenancy (con Hexarch) y Abstracción del Repositorio](https://crustyengineer.com/blog/axum-multi-tenancy-abstract-repository-layer/)
* [Solucionando problemas de rust-lang stdarch en LLVM - Blog - Tweede golf](https://tweedegolf.nl/en/blog/196/fixing-rust-lang-stdarch-issues-in-llvm)
* [Pruebas unitarias de Rust: espías y dobles de prueba ficticios](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_spy/)
* [Serie de backend de Axum: Rotación de tokens de actualización y detección de reutilización](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-refresh-token-rotation/)
* [Validación de vibraciones con Lean, ChatGPT-5, & Claude 4.5: Nueve reglas para demostrar que los algoritmos (Rust) son correctos sin conocer los métodos formales (Parte 1)](https://medium.com/@carlmkadie/vibe-validation-with-lean-chatgpt-5-claude-4-5-part-1-c57b430b3d7a)
* [Un evaluador mecanografiado en Rust](https://rvarago.github.io/typed-evaluator-in-rust/)
* [Composición de costo cero y el poder de los GAT](https://orxfun.github.io/orxfun-notes/#/zero-cost-composition-2025-10-15)
* [Pruebas de integración de cajas binarias de Rust](https://www.unwoundstack.com/blog/integration-testing-rust-binaries.html)
* [video] [Compilación con Naz: Cómo acelerar el compilador de Rust para diferentes flujos de trabajo](https://www.youtube.com/watch?v=hpGDCbO31Rg)

### Miscelánea
* [Informe de empleos de Rust de septiembre de 2025](https://filtra.io/rust/jobs-report/sep-25)

## Crate de la semana

El crate de esta semana es [extend\_mut](https://docs.rs/extend_mut), una biblioteca para extender de forma segura la vida útil de una referencia exclusiva bajo algunas restricciones. P

¡Gracias a [Oleksandr Babak](https://users.rust-lang.org/t/crate-of-the-week/2704/1482) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

[Carga](https://github.com/rust-lang/cargo/labels/call-for-testing)
* [Problema de seguimiento para cargo-script RFC 3424](https://github.com/rust-lang/cargo/issues/12207)
  * [Pasos de prueba](https://github.com/rust-lang/cargo/issues/12207#issuecomment-3412997290)

* * No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háznoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y / o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

* [Diésel - Mejorar la documentación para los modos de carga de Postgres](https://github.com/diesel-rs/diesel/issues/4764)
* [Diesel: agregue soporte para funciones postgres json / jsonb actualmente no compatibles](https://github.com/diesel-rs/diesel/issues/4216)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándote con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust).

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP cierra 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 369 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-14..2025-10-21

#### Compilador
* [agregue una verificación '!=' a 'ChunkedBitSet::union'](https://github.com/rust-lang/rust/pull/147619)
* [limpiezas de conjuntos de bits](https://github.com/rust-lang/rust/pull/147630)
* ['deduced_param_attrs': marque Congelar en tipos monomórficos](https://github.com/rust-lang/rust/pull/147695)
* [denegar por defecto nunca escribir lints](https://github.com/rust-lang/rust/pull/146167)
* [mejorar el mensaje de error para tipos numéricos ambiguos en los parámetros de cierre](https://github.com/rust-lang/rust/pull/147577)
* [eliminar cuadros de los elementos de la lista AST](https://github.com/rust-lang/rust/pull/146221)
* [Mejoras de 'TaskDeps'](https://github.com/rust-lang/rust/pull/147508)
* ['unused_must_use': No advertir sobre 'Result<(), Deshabitado>' o 'ControlFlow<Deshabitado, ()>'](https://github.com/rust-lang/rust/pull/147382)
* [use Vec regular en BitSet](https://github.com/rust-lang/rust/pull/147644)

#### Biblioteca
* [const 'mem::d rop'](https://github.com/rust-lang/rust/pull/147708)
* [constificar impls de clonación básica](https://github.com/rust-lang/rust/pull/146976)
* [iter repeat: pánico al final](https://github.com/rust-lang/rust/pull/147258)
* [estabilizar 'rotate_left' y 'rotate_right' en '[_]' como elementos 'const fn'](https://github.com/rust-lang/rust/pull/146841)
* [función de estabilización de la biblioteca 'rwlock_downgrade'](https://github.com/rust-lang/rust/pull/143191)

#### Carga
* ['check': Corregir el comando sugerido para el paquete bin](https://github.com/rust-lang/cargo/pull/16127)
* ['script': Eliminar la desinfección de nombres fuera de lo estrictamente necesario](https://github.com/rust-lang/cargo/pull/16120)
* ['script': Ajustar el script de carga build-dir / target-dir](https://github.com/rust-lang/cargo/pull/16086)

#### Rustdoc
* [búsqueda: stringdex 0.0.2](https://github.com/rust-lang/rust/pull/147660)
* [arreglar el orden de las pasadas para que los enlaces intra-doc se recopilen después de eliminar las pasadas](https://github.com/rust-lang/rust/pull/147809)

#### Clippy
* ['empty_enum': no pelar si todas las variantes resultan ser 'cfg'-d out](https://github.com/rust-lang/rust-clippy/pull/15911)
* ['option_option': dividir parte del mensaje de diagnóstico en mensaje de ayuda](https://github.com/rust-lang/rust-clippy/pull/15870)
* ['unnecessary_safety_comment' Algunas correcciones con respecto a los comentarios sobre los atributos](https://github.com/rust-lang/rust-clippy/pull/15678)
* [permitir 'explicit_write' en las pruebas](https://github.com/rust-lang/rust-clippy/pull/15862)
* [argumento de desreferencia de 'manual_div_ceil()' si es necesario](https://github.com/rust-lang/rust-clippy/pull/15706)
* ['manual_rotate': también reconocer no consts](https://github.com/rust-lang/rust-clippy/pull/15402)
* [revisión 'mutex_{atomic,integer}'](https://github.com/rust-lang/rust-clippy/pull/15632)

#### Analizador de Rust
* [analizador: No cometer errores en el frontmatter](https://github.com/rust-lang/rust-analyzer/pull/20854)
* [mejorar el soporte de accesorios](https://github.com/rust-lang/rust-analyzer/pull/20855)
* [corregir RestPat no válido para 'convert_tuple_struct_to_named_struct'](https://github.com/rust-lang/rust-analyzer/pull/20880)
* [corregir la falta de RestPat para 'convert_named_struct_to_tuple_struct'](https://github.com/rust-lang/rust-analyzer/pull/20872)
* [no haga que 'convert_to_guarded_return' sea aplicable a 'let-else'](https://github.com/rust-lang/rust-analyzer/pull/20838)
* [corregir 'signature_help' a la proto conversión que crea desplazamientos UTF16 no válidos](https://github.com/rust-lang/rust-analyzer/pull/20876)
* [soporte 'break' con valor en finalizaciones](https://github.com/rust-lang/rust-analyzer/pull/20673)
* [admite bloques 'else' con el tipo de retorno '!' en 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/20758)
* [soporte 'coincidencia' dentro de 'if' en 'pull_assignment_up'](https://github.com/rust-lang/rust-analyzer/pull/20772)
* [migrar más cosas al siguiente solucionador](https://github.com/rust-lang/rust-analyzer/pull/20841)
* [migrar la varianza al siguiente solucionador y eliminar la pelusa permite de su material](https://github.com/rust-lang/rust-analyzer/pull/20867)
* [arrancar tiza de la base 🎉 de código](https://github.com/rust-lang/rust-analyzer/pull/20873)
* [soporte parámetro de sufijo de subrayado hide inlayHints](https://github.com/rust-lang/rust-analyzer/pull/20858)
* [use 'FileId::MAX' para la aserción de id en 'PathInterner::intern'](https://github.com/rust-lang/rust-analyzer/pull/20757)

### Triaje de rendimiento del compilador de Rust

Semana bastante ocupada, con muchos resultados mixtos. Sin embargo, en general terminamos con un
ligera mejora en promedio.

Triaje realizado por **@simulacrum**.
Rango de revisión: [956f47c3.. 4068bafe](https://perf.rust-lang.org/?start=956f47c32f1bd97b22cd702d7ccf78f0f0d42c34&end=4068bafedd8ba724e332a5221c06a6fa531a30d2&absolute=false&stat=instructions%3Au)

2 regresiones, 5 mejoras, 10 mixtas; 5 de ellos en rollups

39 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-10-20.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para los intrínsecos de NEON fp16](https://github.com/rust-lang/rust/issues/136306)
* [Cambiar la vida útil de 'Location<'_>' a ''static' en 'Panic[Hook]Info'](https://github.com/rust-lang/rust/pull/146561)
* [Problema de seguimiento para 'substr_range' y métodos relacionados](https://github.com/rust-lang/rust/issues/126769)
* [repr(transparent]: no considera que los tipos repr(C) sean 1-ZST](https://github.com/rust-lang/rust/pull/147185)
* [No requiere 'T: RefUnwindSafe' para 'vec::IntoIter<T>: UnwindSafe'](https://github.com/rust-lang/rust/pull/145665)
* [Estabilizar -Zno-jump-tables en -Cjump-tables=bool](https://github.com/rust-lang/rust/pull/145974)
* [Problema de seguimiento para alloc_layout_extra](https://github.com/rust-lang/rust/issues/55724)
* [Agregar lint warn-by-default para visibilidad en declaraciones 'const _'](https://github.com/rust-lang/rust/pull/147136)
* [Problema de seguimiento para 'debug_closure_helpers'](https://github.com/rust-lang/rust/issues/117729)
* [desaprobar completamente los módulos integrales heredados](https://github.com/rust-lang/rust/pull/146882)
* [Problema de seguimiento para 'fmt_from_fn'](https://github.com/rust-lang/rust/issues/146705)
* [Hacer que los métodos 'IoSlice' e 'IoSliceMut' sean inestables](https://github.com/rust-lang/rust/pull/144090)
* [Problema de seguimiento para 'VecDeque::p op_front_if' y 'VecDeque::p op_back_if'](https://github.com/rust-lang/rust/issues/135889)
* [disposición: no especificada] [[std][BTree] Corregir el comportamiento de '::append' para que coincida con la documentación, '::insert' y '::extend'](https://github.com/rust-lang/rust/pull/145628)
* [Los elementos impls e impl heredan el nivel de pelusa 'dead_code' de los rasgos y elementos de rasgo correspondientes](https://github.com/rust-lang/rust/pull/144113)
* [Validez de bits del documento MaybeUninit](https://github.com/rust-lang/rust/pull/140463)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(Solo MCP)](https://forge.rust-lang.org/compiler/mcp.html)
* [Mover la pelusa de código inalcanzable de la verificación de tipo HIR a una pelusa adecuada](https://github.com/rust-lang/compiler-team/issues/931)
* [Cambiar el nombre de '//@ add-core-stubs' a '//@ add-minicore'](https://github.com/rust-lang/compiler-team/issues/930)
* [Anotación de movimiento para generar perfiles de movimientos y copias generados por el compilador.](https://github.com/rust-lang/compiler-team/issues/928)
* [Utilice 'llvm-bitcode-linker' como enlazador predeterminado para nvptx64-nvidia-cuda](https://github.com/rust-lang/compiler-team/issues/927)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Delegar el gasto de dinero del GSoC al equipo de tutoría t](https://github.com/rust-lang/leadership-council/issues/232)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Rusty Eventos entre 2025-10-22 - 2025-11-19 🦀

### Virtual
* 2025-10-22 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-22 | Virtual (Buenos Aires, AR) | [[Net-Baires] Comunidad de .NET en Buenos Aires](https://www.meetup.com/es-ES/net-baires/)
    * [**Rust para devs .NET | Standup comunitario #10**](https://www.meetup.com/es-ES/net-baires/events/311365783/)
* 2025-10-23 | Híbrido (Seattle/Bellevue, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2025 Reunión de SRUG (Seattle Rust User Group)**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361444/)
* 2025-10-30 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-05 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Dominando el manejo de errores en Rust: De los pánicos a este error y de todos modos**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación / Weekly coding session**](https://luma.com/xkd84gfz)
* 2025-11-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (Londres, GB) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-13 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de noviembre de 2025**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europa
* 2025-10-23 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/311501254/)
* 2025-10-24 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/311501249/)
* 2025-10-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)
* 2025-10-29 | Dortmund, DE | [Rust, Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Encuentro de Rust Dortmund Octubre 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-30 | Copenhague, Dinamarca | [Comunidad de Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #62 patrocinada por Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga (octubre de 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-01 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #19**](https://www.meetup.com/stockholm-rust/events/311582259/)
* 2025-11-02 - 2025-11-04 | Florencia, IT | [Laboratorio de Rust 2025](https://rustlab.it/)
    * $[**Rustlab 2025**](https://rustlab.it/)
* 2025-11-03 | Berna, CH | [Gremio42](https://www.meetup.com/it-IT/guild42ch/)
    * [**Pasar de la programación de sistemas a Kubernetes: ¿es hora de adoptar Rust?**](https://www.meetup.com/it-IT/guild42ch/events/307260207/)
* 2025-11-04 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Charla de noviembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-04 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/events/)
    * [**Optimización de la multiplicación de matrices y construcción de paquetes de Python con Rust**](https://www.meetup.com/rust-trondheim/events/311595023/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión de Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcpbhb/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Encuentro de Rust Gdansk #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Ginebra, CH | [Rust Ginebra](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Ginebra**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)

### América del Norte
* 2025-10-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Resumen del año**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-23 | Híbrido (Seattle/Bellevue, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2025 Reunión de SRUG (Seattle Rust User Group)**](https://www.meetup.com/seattle-rust-user-group/events/311351020/)
* 2025-10-23 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro de Rust de octubre: ¡Una presentación especial y los encuentros mensuales están de vuelta!**](https://www.meetup.com/spokane-rust/events/311346444/)
* 2025-10-25 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Porter Square, 25 de octubre **](https://www.meetup.com/bostonrust/events/310983712/)
* 2025-10-25 | Dallas, TX, EE. UU. | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**¡Reunión en persona con el Club de Ingeniería de Software de Collin College!**](https://www.meetup.com/dallasrust/events/311562607/)
* 2025-10-28 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/311603282/)
* 2025-10-29 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Análisis estático escalable: enfrentando el problema de la detención**](https://www.meetup.com/rust-nyc/events/311541108/)
* 2025-10-30 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-10-30 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311273832/)
* 2025-11-01 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Chinatown, 1 de noviembre **](https://www.meetup.com/bostonrust/events/311039492/)
* 2025-11-06 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Estudiantes de SIUE sobre animaciones 3D wasm**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, 8 de noviembre **](https://www.meetup.com/bostonrust/events/311039501/)
* 2025-11-13 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Creación de aplicaciones de escritorio con Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-18 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)

### Oceanía
* 2025-10-22 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de octubre**](https://www.meetup.com/rust-canberra/events/311234237/)

### América del Sur
* 2025-10-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Rust Uruguay meetup de Octubre**](https://www.meetup.com/rust-uruguay/events/311475675/)
* 2025-10-25 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)
* 2025-10-30 | Florianópolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Solía haber preguntas recurrentes sobre mod vs uso en el foro de usuarios, hasta que agregué una nota al mensaje de error [...] y creo que resolvió en gran medida el problema

– [Kornel sobre el interior del Rust](https://internals.rust-lang.org/t/curly-brace-support-for-mod/23437/51)

¡Gracias a [Noratrieb](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1722) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1odqrri/this_week_in_rust_622/)</small>
