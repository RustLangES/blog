---
title: "Esta semana en Rust #78"
number_of_week: 78
description: El crate de esta semana es blogr, un generador de sitios estáticos rápido y ligero.
date: 2025-10-01
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
* [El próximo Rust All Hands](https://blog.rust-lang.org/inside-rust/2025/09/30/all-hands-2026/)
* [Este ciclo de desarrollo en carga: 1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/)

### Boletines
* [El Rustacean Incrustado Edición #55](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-55)

### Actualizaciones de proyectos/herramientas
* [Bevy 0.17](https://bevy.org/news/bevy-0-17/)
* [Código de Linting Rust en el kernel](https://lwn.net/SubscriberLink/1038750/6aa9769e0b875235/)
* [E/S UDP rápida para Firefox en Rust](https://max-inden.de/post/fast-udp-io-in-firefox/)
* [genedex: Un índice FM pequeño y rápido para Rust](https://github.com/feldroop/genedex)
* [blogr v0.3.0 - Generador de sitios estáticos rápido y liviano con administrador de boletines incorporado](https://github.com/bahdotsh/blogr/releases/tag/v0.3.0)
* [Feedr v0.2.0 - Un lector de feeds RSS basado en terminal rico en funciones con una TUI limpia e intuitiva](https://github.com/bahdotsh/feedr/releases/tag/v0.2.0)
* [Glues v0.8.1 agrega una TUI del navegador, soporte de proxy y almacenamiento redb](https://github.com/gluesql/glues/releases/tag/v0.8.1)
* [Control de acceso basado en roles en SeaORM 2.0](https://www.sea-ql.org/blog/2025-09-30-sea-orm-rbac/)

### Observaciones/Pensamientos
* [Sobre la elección de Rust](https://endler.dev/2025/choosing-rust/)
* [La expresión problema y Rust](https://purplesyringa.moe/blog/the-expression-problem-and-rust/)
* [Estudio de caso: Cómo Proton usa Rust para crear aplicaciones multiplataforma para millones de personas](https://kerkour.com/proton-apps-rust)
* [El motor de juego que no se habría hecho sin Rust](https://blog.vermeilsoft.com/2025-09-rust-game-engine)
* [Introducción de Rust a la pila automotriz: una conversación con Julius Gustavsson de Volvo Cars](https://filtra.io/rust/interviews/volvo-sep-25)
* [Rust in Paris 2025 – Lista de reproducción completa de charlas](https://www.youtube.com/playlist?list=PLiOc9_WF8i8vVy5Qn6I9TxkBjsofzLcaH)
* [video] [Cómo optimizar Rust para la lentitud: inspirado en los nuevos resultados de la máquina de Turing](https://www.youtube.com/watch?v=ec-ucXJ4x-0)

### Tutoriales de Rust
* [Bajo el capó: Vec\<T\>](https://marma.dev/articles/2025/under-the-hood-vec-t)
* [Serie de backend de Axum: Implementar token de acceso JWT](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-jwt-access-token/)
* [Sube de nivel tu coincidencia de patrones de Rust](https://blog.cuongle.dev/p/level-up-your-rust-pattern-matching)
* [video] [Sguaba: Matemáticas espaciales con seguridad de tipos en Rust](https://www.youtube.com/watch?v=kESBAiTYMoQ)

### Miscelánea
* [Cloudflare ahora es más rápido y seguro, impulsado por Rust](https://blog.cloudflare.com/20-percent-internet-upgrade/)
* [Rust: Quién, qué y por qué para el taller de ESA SW PA](https://ferrous-systems.com/blog/rust-who-what-why/)

## Crate de la semana

El crate de esta semana es [blogr](https://github.com/bahdotsh/blogr), un generador de sitios estáticos rápido y ligero.

¡Gracias a [Gokul](https://users.rust-lang.org/t/crate-of-the-week/2704/1472) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

[Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

* [FR: Añadir una opción --fail-fast a libtest](https://github.com/rust-lang/rust/issues/142859)
  * [Instrucciones de prueba](https://github.com/rust-lang/rust/issues/142859#issuecomment-3339090064)

*No se emitieron llamadas para pruebas esta semana por
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háznoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

* [Diésel - Comentarios sobre una guía de All About Select](https://github.com/diesel-rs/diesel/discussions/4786)
* [Diesel - Explicación incompleta de los modos de carga de PgConnection](https://github.com/diesel-rs/diesel/issues/4764)
* [Diesel - La macro de derivación de FromSqlRow falla cuando está en el ámbito donde Ok no es el de la biblioteca estándar](https://github.com/diesel-rs/diesel/issues/4787)
* [Diesel: fk_related_tables no funciona correctamente con dos claves externas en la misma tabla](https://github.com/diesel-rs/diesel/issues/4780)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

473 solicitudes de extracción se [fusionaron en la última semana] [fusionaron]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-23..2025-09-30

#### Compilador
* [agregar un atributo para verificar el número de carriles en un vector SIMD después de la monomorfización](https://github.com/rust-lang/rust/pull/146667)
* [agregar pánico = abortar inmediatamente](https://github.com/rust-lang/rust/pull/146317)
* [omitir controlador de desbordamiento de pila para panic = abortar inmediato](https://github.com/rust-lang/rust/pull/147090)
* [allow '&raw [mut | const]' para el campo de unión en código seguro](https://github.com/rust-lang/rust/pull/141469)
* [debuginfo: agregar un indicador inestable para escribir DWARF dividido en un directorio explícito](https://github.com/rust-lang/rust/pull/146376)
* [detectar estructuras de tupla que no se pueden construir debido a la reexportación](https://github.com/rust-lang/rust/pull/133477)
* [no calcular MIR optimizado si el código no verifica el tipo](https://github.com/rust-lang/rust/pull/147092)
* [no materializar X en '[X; 0]' cuando X está desdimensionando una const](https://github.com/rust-lang/rust/pull/145277)
* [mejorar el diagnóstico de atributos vacíos](https://github.com/rust-lang/rust/pull/146653)
#### Biblioteca
* [BTreeMap: no filtrar asignadores al inicializar nodos](https://github.com/rust-lang/rust/pull/146859)
* [constify {'Mutex', 'RwLock', 'ReentrantLock'}'::d ata_ptr'](https://github.com/rust-lang/rust/pull/146904)
* [constify predeterminado en nanosegundos](https://github.com/rust-lang/rust/pull/146979)
* [constify '{float}::total_cmp()'](https://github.com/rust-lang/rust/pull/146818)
* [métodos de 'mul_add' flotantes de constitución inestable](https://github.com/rust-lang/rust/pull/146735)
* [sin pánico 'Vec::try_remove'](https://github.com/rust-lang/rust/pull/146293)
* [corregir la recursión infinita en 'Path::eq' con String](https://github.com/rust-lang/rust/pull/146958)
* [implementar 'nombre de host'](https://github.com/rust-lang/rust/pull/146937)
* [eliminar la mayoría de '#[track_caller]' de la asignación de métodos Vec](https://github.com/rust-lang/rust/pull/147042)
#### Carga
* [config: combinar el contexto de error clave en uno](https://github.com/rust-lang/cargo/pull/16004)
* [shell: Usar un estilo distinto para el estado transitorio](https://github.com/rust-lang/cargo/pull/16019)
* [agregar reintento para fallas de 'git fetch' en la ruta 'CARGO_NET_GIT_FETCH_WITH_CLI'](https://github.com/rust-lang/cargo/pull/16016)
* [mejor mensaje de error para la incompatibilidad de la versión de Rust](https://github.com/rust-lang/cargo/pull/16021)
* [URL dispersas en 'TomlLockfileSourceId'](https://github.com/rust-lang/cargo/pull/15990)
* [use 'host -tuple' para la sustitución del huésped objetivo](https://github.com/rust-lang/cargo/pull/16003)
#### Rustdoc
* [Agregar soporte para elementos asociados en la función "saltar a def"](https://github.com/rust-lang/rust/pull/135771)
* [agregar características de rustdoc 'doc_cfg'](https://github.com/rust-lang/rust/pull/138907) (RFC [#3631](https://rust-lang.github.io/rfcs/3631-rustdoc-cfgs-handling.html))
* [búsqueda: use el mismo ID para la entrada y la ruta al mismo elemento](https://github.com/rust-lang/rust/pull/147045)
* [ocultar '#[repr]' si no es parte del ABI público](https://github.com/rust-lang/rust/pull/116882)
* [coloque la barra de herramientas en el índice de todos los elementos](https://github.com/rust-lang/rust/pull/147047)
#### Clippy
* ['double_parens': añadir sugerencias estructuradas, corregir error](https://github.com/rust-lang/rust-clippy/pull/15420)
* ['filter_next': comprueba 'filter().next_back()'](https://github.com/rust-lang/rust-clippy/pull/15748)
* ['colapsable'('_else')'_if': respeta '#[esperar]' en el interior 'si'](https://github.com/rust-lang/rust-clippy/pull/15647)
* ['let_unit_value': crear la sugerencia "diferencialmente"](https://github.com/rust-lang/rust-clippy/pull/15788)
* ['new_without_default': si 'new' tiene '#[cfg]', cópielo en 'impl Default'](https://github.com/rust-lang/rust-clippy/pull/15720)
* ['or_fun_call': respete MSRV para la sugerencia 'Resultado::unwrap_or_default'](https://github.com/rust-lang/rust-clippy/pull/15756)
* ['should_implement_trait': solo sugiere rasgos que están en el preludio](https://github.com/rust-lang/rust-clippy/pull/15776)
* ['unnecessary_mut_passed': mantener paréntesis alrededor de los argumentos](https://github.com/rust-lang/rust-clippy/pull/15731)
* [verifique si hay macros de proc dentro de 'explicit_deref_methods' y no pelusa en las expansiones de macros de proc](https://github.com/rust-lang/rust-clippy/pull/15628)
* [corregir el falso positivo 'new_without_default' en el tipo privado con rasgo impl](https://github.com/rust-lang/rust-clippy/pull/15782)
* [revisión 'mut_mut'](https://github.com/rust-lang/rust-clippy/pull/15417)
* [refactorizar 'module_style'](https://github.com/rust-lang/rust-clippy/pull/15469)
* [cambie el nombre de 'unchecked_duration_subtraction' a 'unchecked_time_subtraction' y verifique 'Duración - Duración'](https://github.com/rust-lang/rust-clippy/pull/13800)
#### Analizador de Rust
* [agregue 'todos', 'cualquiera' y 'no' terminaciones en '#[cfg]'](https://github.com/rust-lang/rust-analyzer/pull/20760)
* [agregar 'cfg_attr' finalización del predicado](https://github.com/rust-lang/rust-analyzer/pull/20604)
* [añadir aplicable en bang '!' para 'apply_demorgan'](https://github.com/rust-lang/rust-analyzer/pull/20599)
* [agregar finalización de palabras clave de parámetro const](https://github.com/rust-lang/rust-analyzer/pull/20729)
* [agregar soporte let-chain para 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/20598)
* [permitir '&raw' {'mut', 'const'} para el campo de unión](https://github.com/rust-lang/rust-analyzer/pull/19867)
* [corregir "Reemplazar coincidencia con if let" para que no se active cuando se producen transformaciones no válidas](https://github.com/rust-lang/rust-analyzer/pull/20543)
* [arreglar el pánico de SCIP debido a que la salsa no se adhiere](https://github.com/rust-lang/rust-analyzer/pull/20735)
* [corrección aplicable en if-let-chain para 'invert_if'](https://github.com/rust-lang/rust-analyzer/pull/20736)
* [corregir el patrón de silencio expandido en el patrón de tupla y corte](https://github.com/rust-lang/rust-analyzer/pull/20731)
* [corregir paréntesis de precedencia para 'replace_arith_op'](https://github.com/rust-lang/rust-analyzer/pull/20611)
* [no convertir las variables no utilizadas en un identificador sin procesar](https://github.com/rust-lang/rust-analyzer/pull/20742)
* [implementar la reserva correctamente](https://github.com/rust-lang/rust-analyzer/pull/20721)
* [admite literales enteros negativos en genéricos const en macros declarativas](https://github.com/rust-lang/rust-analyzer/pull/20745)

### Triaje de rendimiento del compilador de Rust

Una semana relativamente tranquila. La mayoría de las mejoras son para compilaciones de documentos, impulsadas por
Empaquetado continuado del índice de búsqueda en rustdoc-search: Actualización de stringdex con
más embalaje [#147002](https://github.com/rust-lang/rust/pull/147002) y
simplificaciones a doc(cfg) en Implementar RFC 3631: agregar funciones de doc_cfg rustdoc
[#138907](https://github.com/rust-lang/rust/pull/138907).

Triaje realizado por **@simulacrum**.
Rango de revisión: [ce4beebe.. 8d72d3e1](https://perf.rust-lang.org/?start=ce4beebecb77821734079cff47d8af08f9f27f11&end=8d72d3e1e96f58ca10059a6bb6e8aecba4a0e9cd&absolute=false&stat=instructions%3Au)

1 regresiones, 6 mejoras, 4 mixtas; 2 de ellos en rollups
29 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-09-28.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Problema de seguimiento para 'const_slice_rotate'](https://github.com/rust-lang/rust/issues/143812)
* [implementar Extend<{Group, Literal, Punct, Ident}> para TokenStream](https://github.com/rust-lang/rust/pull/145722)
* [Estabilizar 'char_max_len'](https://github.com/rust-lang/rust/pull/145610)
* [Agregar 'Desde' impls para tipos de envoltorios](https://github.com/rust-lang/rust/pull/146013)
* [preferir candidatos de alias para objetivos de tamaño + rasgo automático](https://github.com/rust-lang/rust/pull/144064)
* [Problema de seguimiento para 'NonZero<u*>::d iv_ceil'](https://github.com/rust-lang/rust/issues/132968)
* [Problema de seguimiento para las notas de la versión de #146410: Repetición del iterador: no hay bucle infinito para 'último' y 'recuento'](https://github.com/rust-lang/rust/issues/146660)
* [Función de estabilización de la biblioteca 'rwlock_downgrade'](https://github.com/rust-lang/rust/pull/143191)
* [Evitar 'impl DerefMut for Pin<LocalType>'](https://github.com/rust-lang/rust/pull/145608)
* [Prohibir lanzar libremente límites de por vida de tipos dyn](https://github.com/rust-lang/rust/pull/136776)
* [Corregir inferencia de tipo accidental en la coerción de matrices](https://github.com/rust-lang/rust/pull/140283)
* [docs(style): Especifica el estilo del frontmatter](https://github.com/rust-lang/rust/pull/145617)
* [núcleo: simplificar 'Extender' para tuplas](https://github.com/rust-lang/rust/pull/138799)

*Ningún artículo entró en el período de comentarios finales esta semana para
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
  [Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Rusty Eventos entre 2025-10-01 - 2025-10-29 🦀

### Virtual
* 2025-10-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)
* 2025-10-02 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/ekgdex6j)
* 2025-10-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763858627)
* 2025-10-05 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311062530/)
* 2025-10-07 | Virtual (Beijing, CN) | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Reunión mensual de la comunidad de WasmEdge, el tiempo de ejecución de LLM / AGI **](https://www.meetup.com/wasm-rust-meetup/events/310831771/)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-09 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046639/)
* 2025-10-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jotnli2g)
* 2025-10-12 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109160/)
* 2025-10-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361534/)
* 2025-10-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/o8fh3fh7)
* 2025-10-16 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046668/)
* 2025-10-18 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/310935164/)
* 2025-10-19 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109167/)
* 2025-10-21 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068625/)
* 2025-10-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/310002307/)
* 2025-10-23 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046641/)
* 2025-10-23 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/zyc3touy)
* 2025-10-26 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109171/)
* 2025-10-28 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361444/)

### Asia
* 2025-10-02 | Seúl, KR | [Reunión de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310824483/)
* 2025-10-04 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de octubre de 2025**](https://hasgeek.com/rustbangalore/october-2025-rustacean-meetup/)
* 2025-10-08 | Kuala Lumpur, MY | [Rust Malasia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup**](https://docs.google.com/forms/d/e/1FAIpQLScESY4eHc5lzZznAHZmFxI85CYaOKCYTQASRwXxC2y0KpI6zw/viewform)
* 2025-10-09 | Tokio, JP | [Encuentro de Tokyo Rust](https://www.meetup.com/tokyo-rust-meetup)
    * [**Creación de interfaces de usuario de terminal de bolsillo con Rust**](https://www.meetup.com/tokyo-rust-meetup/events/310899137/)
* 2025-10-20 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**En persona Rust octubre de 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/310628902/)

### Europa
* 2025-10-01 | Colonia, DE | [Colonia de Rust](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en octubre: Rust indefinido**](https://www.meetup.com/rustcologne/events/311209846/)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia)
    * [**4. Encuentro de Rust Moravia (¡En la capital!)**](https://www.meetup.com/rust-moravia/events/310743282/)
* 2025-10-01 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Construyendo chatbots de IA con Webassembly, Rust y Leptos**](https://www.meetup.com/oxford-rust-meetup-group/events/311170808/)
* 2025-10-01 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1686673127729)
* 2025-10-02 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin en locación 🏳️ 🌈 - Edición 007**](https://www.meetup.com/rust-berlin/events/311202886/)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134/)
* 2025-10-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 10 2025**](https://luma.com/8u55jo0h)
* 2025-10-08 | París, FR | [Rust París](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #79**](https://www.meetup.com/rust-paris/events/310424476/)
* 2025-10-08 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944041/)
* 2025-10-09 - 2025-10-10 | Híbrido (París, Francia) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2025**](https://eurorust.eu/schedule/)
* 2025-10-14 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #13 @ letsboot**](https://www.meetup.com/rust-basel/events/310827834/)
* 2025-10-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/311035141/)
* 2025-10-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592252/)
* 2025-10-21 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rust in Surgery: Powering the Data Pipelines**](https://www.meetup.com/london-rust-project-group/events/310813952/)
* 2025-10-21 | Bergen, No | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Meetup #01 @ Zrch**](https://www.meetup.com/bergen-rust-new-technology/events/311153821/)
* 2025-10-28 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester October Code Night**](https://www.meetup.com/rust-manchester/events/307919171/)

### América del Norte
* 2025-10-01 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Reunión de desarrolladores de Web3**](https://www.meetup.com/rust-los-angeles/events/311243690/)
* 2025-10-02 | Montreal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Octubre Social Mensual**](https://www.meetup.com/rust-montreal/events/311242811/)
* 2025-10-02 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311004898)
* 2025-10-02 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust)
    * [** 🚁 Rust en vuelo: lecciones del diseño de un cuadricóptero impreso en 3D con incrustación**](https://www.meetup.com/stl-rust/events/310279407/)
* 2025-10-04 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, 4 de octubre **](https://www.meetup.com/bostonrust/events/310983705/)
* 2025-10-09 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust)
    * [**Aya the Beholder: Escribiendo un cortafuegos eBPF con la caja de Aya**](https://www.meetup.com/utah-rust/events/311145663/)
* 2025-10-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731034/)
* 2025-10-16 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311012947/)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Vara & Equipo](https://luma.com/events-by-vara-gear)
    * [**Taller de Rust de Vara Network**](https://luma.com/kbs2os1c)
* 2025-10-21 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284343/)
* 2025-10-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457307/)
* 2025-10-23 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Resumen del año**](https://www.meetup.com/music-city-rust-developers/events/304333267/)
* 2025-10-25 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Porter Square, 25 de octubre **](https://www.meetup.com/bostonrust/events/310983712/)

### Oceanía
* 2025-10-22 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099/)
* 2025-10-28 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Reunión de octubre**](https://www.meetup.com/rust-canberra/events/311234237/)

### América del Sur
* 2025-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Octubre Async - Escribimos un Runtime desde Cero!**](https://www.meetup.com/rust-argentina/events/311276950/)
* 2025-10-25 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na Amazon Web Services**](https://www.meetup.com/rust-sao-paulo-meetup/events/311084440/)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> debo extender personalmente mis condolencias a aquellos que olvidaron que eligieron en el pasado molestar a su yo futuro.

– [@workingjubilee en GitHub](https://github.com/rust-lang/rust/issues/145936#issuecomment-3322104583)

¡Gracias a [Riking](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1719) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1nvpd2x/this_week_in_rust_619/)</small>
