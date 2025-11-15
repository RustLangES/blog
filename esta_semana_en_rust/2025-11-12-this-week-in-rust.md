---
title: "Esta semana en Rust #84"
number_of_week: 84
description: El crate de esta semana es automesh, una caja para la generación automática de mallas de alto rendimiento en Rust.
date: 2025-11-12
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
* [Anuncio de Rust 1.91.1](https://blog.rust-lang.org/2025/11/10/Rust-1.91.1/)

### Boletines
* [El Problema Rustaceo Incrustado #58](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-58)
* [Este mes en Rust OSDev: octubre de 2025](https://rust-osdev.com/this-month/2025-10/)

### Actualizaciones de proyectos/herramientas
* [channels-console - monitoreo en tiempo real, métricas y registros para canales de Rust](https://github.com/pawurb/channels-console)
* [qstr: Tipos de cadena asignados a la pila y eficientes en caché](https://github.com/tindzk/qstr)
* [Anuncio de Magika 1.0: ahora más rápido, más inteligente y reconstruido en Rust](https://opensource.googleblog.com/2025/11/announcing-magika-10-now-faster-smarter.html?m=1)
* [Tokuin 0.1.2: Prueba de carga de LLM desde el terminal](https://noos.blog/posts/tokuin-token-tooling-for-llm-builders/)
* [semver-query: herramienta de consulta de datos de control de versiones semánticas](https://github.com/zak905/semver-query)
* [SeaORM 2.0: Columna fuertemente tipada](https://www.sea-ql.org/blog/2025-11-11-sea-orm-2.0/)
* [LLM: modelo nanoGPT en Rust - arrowspace v0.22.0 publicado](https://www.tuned.org.uk/posts/009_llms_nanogpt_model_in_rust)
* [InterpN: Interpolación rápida](https://jlogan.dev/blog/2025/11/10/2025-11-10-interpn-fast-interpolation/)
* [Tako 0.5.0 camino a v1.0.0](https://rust-dd.com/post/tako-v-0-5-0-road-to-v-1-0-0)

### Observaciones/Pensamientos
* [Solo llame a clon (o alias)](https://smallcultfollowing.com/babysteps/blog/2025/11/10/just-call-clone/)
* [Cuestionario de optimización de ingeniería de Rust](https://fasterthanli.me/articles/engineering-a-rust-optimization-quiz)
* [Rust vs. Python: Encontrar el equilibrio adecuado entre velocidad y simplicidad](https://blog.jetbrains.com/rust/2025/11/10/rust-vs-python-finding-the-right-balance-between-speed-and-simplicity/)
* [video] [Un comienzo rápido para Rust Lang](https://www.youtube.com/watch?v=fTXtdbt1PFA)
* [video] [Rust & JavaScript - Jakob Meier - Rust Zürisee Noviembre 2024](https://www.youtube.com/watch?v=MEHi7FZjSYw)
* [audio] [Netstack.FM Episodio 13 - Inside Ping Proxies con Joseph Dye](https://netstack.fm/#episode-13)

### Tutoriales de Rust
* [Presentación de reproches Análisis de descenso recursivo resistente](https://thunderseethe.dev/posts/parser-base/)
* [Hoja de trucos de hash de Rust](https://bd103.github.io/blog/2025-11-10-rust-hashing-cheat-sheet/)

### Miscelánea
* [Seguridad de la memoria para escépticos](https://queue.acm.org/detail.cfm?id=3773095)

## Crate de la semana

El crate de esta semana es [automesh](https://docs.rs/automesh), una caja para la generación automática de mallas de alto rendimiento en Rust.

¡Gracias a [Michael R. Buche](https://users.rust-lang.org/t/crate-of-the-week/2704/1485) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing),
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
*No se enviaron convocatorias de participación esta semana.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándote con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust).

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP cierra 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

409 solicitudes de extracción se [fusionaron en la última semana] [fusionaron]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-04..2025-11-11

#### Compilador
* [agregar desinfectante en tiempo real LLVM](https://github.com/rust-lang/rust/pull/147935)
* [no restablecer completamente 'HeadUsages'](https://github.com/rust-lang/rust/pull/148649)
* [use annotate-snippets de forma predeterminada todas las noches](https://github.com/rust-lang/rust/pull/148188)
* [implementar cambios de embudo SIMD en const-eval/Miri](https://github.com/rust-lang/rust/pull/147534)
* [recuperar '[T: N]' como '[T; N]'](https://github.com/rust-lang/rust/pull/148680)

#### Biblioteca
* [agregar impls de proxy de asignador para Box, Rc y Arc](https://github.com/rust-lang/rust/pull/148539)
* [agregue 'extend_front' a VecDeque con especialización como extender](https://github.com/rust-lang/rust/pull/146861)
* [agregar parámetro de alineación a 'simd_masked_{load,store}'](https://github.com/rust-lang/rust/pull/147355)
* [constificar métodos 'ControlFlow' con límites inestables](https://github.com/rust-lang/rust/pull/148285)
* [constificar métodos 'ControlFlow' sin límites inestables](https://github.com/rust-lang/rust/pull/148248)
* [desenvolver el resultado de constify desactivado](https://github.com/rust-lang/rust/pull/148333)
* [optimizar la iteración de los componentes de la ruta en plataformas que no tienen prefijos](https://github.com/rust-lang/rust/pull/148084)
* [estabilizar 'as_array' en '[_]' y '*const [_]'; estabilizar 'as_mut_array' en '[_]' y '*mut [_]'](https://github.com/rust-lang/rust/pull/147540)
* [estabilizar 'vec_deque_pop_if'](https://github.com/rust-lang/rust/pull/145992)
* [Estabilizar la función de objetivo 'vectorial' S390X y la macro '¡is_s390x_feature_detected!'](https://github.com/rust-lang/rust/pull/145656)
* [deja de especializarte en 'Copy'](https://github.com/rust-lang/rust/pull/135634)

#### Carga
* ['cli': Referirse a comandos, no a subcomandos](https://github.com/rust-lang/cargo/pull/16226)
* ['finalizaciones': no envuelva la ayuda del elemento de finalización entre paréntesis](https://github.com/rust-lang/cargo/pull/16215)
* [agregar finalizaciones nativas para '--package' en varios comandos](https://github.com/rust-lang/cargo/pull/16210)

#### Rustdoc
* [buscar: eliminar caso especial de índice roto](https://github.com/rust-lang/rust/pull/148563)
* [resalte correctamente shebang, frontmatter y palabras clave débiles en páginas de código fuente y bloques de código](https://github.com/rust-lang/rust/pull/148230)

#### Clippy
* [perf: 'manual_is_power_of_two': realice primero la comprobación de 'is_integer_literal'](https://github.com/rust-lang/rust-clippy/pull/16050)
* [considere la conversión de tipos que no se desbordará](https://github.com/rust-lang/rust-clippy/pull/15950)
* [no marcar 'cfg(test)' como multiple inherent impl](https://github.com/rust-lang/rust-clippy/pull/16041)
* [corregir 'match_single_binding' sugiriendo erróneamente dentro de la tupla](https://github.com/rust-lang/rust-clippy/pull/15539)
* [arreglar 'missing_asserts_for_indexing' cambiando 'assert_eq' a 'assert'](https://github.com/rust-lang/rust-clippy/pull/16040)
* [corregir 'missing_inline_in_public_items' que no cumple con 'expect' en la compilación '--test'](https://github.com/rust-lang/rust-clippy/pull/15320)
* [corregir el falso positivo 'mod_module_files' para las pruebas en los espacios de trabajo](https://github.com/rust-lang/rust-clippy/pull/16048)
* [arreglar 'nonminimal_bool' términos erróneamente destruidos](https://github.com/rust-lang/rust-clippy/pull/16017)
* [corregir 'useless_let_if_seq' falso negativo cuando 'if' está en la última expresión del bloque](https://github.com/rust-lang/rust-clippy/pull/16063)

#### Analizador de Rust
* [admite cambio de nombre después de agregar etiqueta de bucle](https://github.com/rust-lang/rust-analyzer/pull/20985)
* [agregar bloqueo al completar el sufijo '.const'](https://github.com/rust-lang/rust-analyzer/pull/21003)
* [corregir el pánico al resolver sigs invocables para 'AsyncFnMut'](https://github.com/rust-lang/rust-analyzer/pull/20971)
* [protectores de mango en 'replace_if_let_with_match'](https://github.com/rust-lang/rust-analyzer/pull/20542)
* [manejar llamadas al método en 'apply_demorgan'](https://github.com/rust-lang/rust-analyzer/pull/20973)
* [parse 'impl ! {}'](https://github.com/rust-lang/rust-analyzer/pull/20972)
* [mover el cálculo seguro fuera del bloque inseguro](https://github.com/rust-lang/rust-analyzer/pull/20977)
* [perf: solo rellenar elementos públicos en el índice de símbolos de dependencia](https://github.com/rust-lang/rust-analyzer/pull/20997)
* [perf: reducir el uso de memoria del índice de símbolos](https://github.com/rust-lang/rust-analyzer/pull/20994)

### Triaje de rendimiento del compilador de Rust

Semana mayormente tranquila, con la mayoría de los cambios provenientes del estándar
trabajo de la biblioteca para eliminar la especialización de copia
([#135634](https://github.com/rust-lang/rust/pull/135634)).

Triaje realizado por **@simulacrum**.
Rango de revisión: [35ebdf9b.. 055d0d6a](https://perf.rust-lang.org/?start=35ebdf9ba1414456dfe1cb6a6b13ebae80e99734&end=055d0d6aaf937cc11b3d2a5b5725972723b7f3c6&absolute=false&stat=instructions%3Au)

3 regresiones, 1 mejora, 7 mixtas; 3 de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-11-10.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* [Pasar punteros a 'const' en el ensamblador](https://github.com/rust-lang/rfcs/pull/3848)

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Advertir contra llamadas que mutan un const -item mutable interior](https://github.com/rust-lang/rust/pull/148407)
* [analizador / lexer: pasar a Unicode 17, usar Unicode-ident más rápido](https://github.com/rust-lang/rust/pull/148321)
* [const-eval: corregir y volver a habilitar el soporte de fragmentos de puntero](https://github.com/rust-lang/rust/pull/148259)
* [Reemplace OffsetOf por una suma real de llamadas a intrínseco.](https://github.com/rust-lang/rust/pull/148151)
* [Estabilizar 'asm_cfg'](https://github.com/rust-lang/rust/pull/147736)
* [Estabilizar '-Zremap-path-scope'](https://github.com/rust-lang/rust/pull/147611)
* [error cuando 'repr(align)' excede el límite de COFF](https://github.com/rust-lang/rust/pull/142638)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(Solo MCP)](https://forge.rust-lang.org/compiler/mcp.html)
* [Objetivo Soporte de nivel 3 para hexagon-unknown-qurt](https://github.com/rust-lang/compiler-team/issues/919)
* [Propuesta de un conjunto de pruebas específico para la interfaz paralela](https://github.com/rust-lang/compiler-team/issues/906)
* [Propuesta para adaptar el protector de pila para Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Dar un signo a los literales enteros en lugar de confiar en expresiones de negación](https://github.com/rust-lang/compiler-team/issues/835)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Introducir 'DerefInto' y 'DerefMutInto' para el acceso RAII](https://github.com/rust-lang/rfcs/pull/3880)

## Próximos eventos

Rusty Eventos entre 2025-11-12 - 2025-12-10 🦀

### Virtual
* 2025-11-12 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tock, un sistema operativo basado en Rust Parte #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Sistemas de uso intensivo de datos en Rust: seguridad, velocidad, simultaneidad**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-30 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de Rust Readers: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (Londres, GB) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Advenimiento del código - ¡Inicio!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311886445/)
* 2025-12-04 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046643/)
* 2025-12-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361537/)

### África
* 2025-11-18 | Johannesburgo, ZA | [Reunión de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Más sobre los tipos de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de noviembre de 2025**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europa
* 2025-11-12 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Ginebra, CH | [Rust Ginebra](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Ginebra**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adopción de Rust en el banco Lloyds (capacidad limitada)**](https://www.meetup.com/london-rust-project-group/events/311884244/)
* 2025-11-13 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London en Lloyds Banking Group**](https://www.meetup.com/rust-london-user-group/events/311868858/)
* 2025-11-13 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-14 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Encuentro de Rust @Magello**](https://www.meetup.com/stockholm-rust/events/311844163/)
* 2025-11-18 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Bevy, Compiladores y tipos de datos algebraicos @ Zrch**](https://www.meetup.com/de-DE/bergen-rust-new-technology/events/311630543)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**Círculo de control de calidad**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche social**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Ámsterdam, Países Bajos | [Grupo de Desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Lucerna, CH | [Rust de Lucerna](https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 2 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784127/)
* 2025-11-26 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**19ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Charlas de noviembre: Tipos de tamaño exótico y ...**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Praga @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.github.io)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 2025-12-03 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Reunión de Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcqbfb/)
* 2025-12-10 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 4 - Noche de piratería**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944053/)

### América del Norte
* 2025-11-13 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Creación de aplicaciones de escritorio con Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-13 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Talks: Custom Rust Linting (ast-grep) & Geospatial DataFrame lib (SedonaDB)**](https://www.meetup.com/rust-nyc/events/311904744/)
* 2025-11-13 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Codificación de Rust con AI**](https://www.meetup.com/pdxrust/events/311936952/)
* 2025-11-13 | San Diego, CA, EE. UU. | [Rust de San Diego](https://www.meetup.com/san-diego-rust/events/)
    * [**Encuentro de noviembre de San Diego Rust - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/311876762/)
* 2025-11-16 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Brookline Rust Lunch, 16 de noviembre **](https://www.meetup.com/bostonrust/events/311917229/)
* 2025-11-18 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)
* 2025-11-20 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Noviembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Reunión mensual de Rust: noviembre**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-23 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Inman Rust Lunch & Hackathon, 23 de noviembre**](https://www.meetup.com/bostonrust/events/311917854/)
* 2025-11-26 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-27 | Mountain View, CA, EE. UU. | [Dojo hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Harvard Square Rust, 29 de noviembre **](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 2025-12-04 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Actix Web Unleashed: Dominio del estado, la seguridad y los controladores escalables en Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 2025-12-05 | Nueva York, NY, EE. UU. | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC Unconf 2025: ¡Nuestro evento más grande hasta ahora!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en el centro de Rust, 6 de diciembre **](https://www.meetup.com/bostonrust/events/311917263/)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Hacer que su "inseguro" sea muy pequeño es como poner marcas de precaución *en* el brazo robótico letalmente fuerte sin sensores de proximidad, en lugar de en la puerta de la jaula protectora.

– [Stephan Sokolow en lobste.rs](https://lobste.rs/c/0vkdmo)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1727) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust es editado por:
* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [MarianneGoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1ovpiqv/this_week_in_rust_625/)</small>
