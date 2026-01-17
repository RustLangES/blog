---
title: "Esta semana en Rust #93"
number_of_week: 93
description: El crate de esta semana es diesel-guard, un linter contra peligrosas migraciones de Postgres.
date: 2026-01-14
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

<!--

Estimados colaboradores de la comunidad:
Por favor, lee README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe ser del siguiente tipo:

* [Título de la página enlazada](https://example.com/my_article)

Si añades un enlace a un contenido que no sea textual, por favor prefijadlo con '[vídeo]' o '[audio]':

* [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast)

Si no sabes qué categoría usar, siéntete libre de enviar una marca permanente de todas formas
Y simplemente pide a los editores que seleccionen la categoría.

-->

### Oficial

* [¿Qué es el mantenimiento, de todos modos?](https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/)
* [Resumen del Equipo de Infraestructura del Cuarto trimestre 2025 y Plan del 1º trimestre 2026](https://blog.rust-lang.org/inside-rust/2026/01/13/infrastructure-team-q4-2025-recap-and-q1-2026-plan/)

### Boletines

* [ 🦀 Envuelto en Rust 2025](https://weeklyrust.substack.com/p/rust-wrapped-2025)

### Actualizaciones de proyectos/herramientas

* [Bevy 0,18](https://bevy.org/news/bevy-0-18/)
* [Crecimiento exponencial continuado — cargo-semver-checks Año 2025 Resumen](https://predr.ag/blog/cargo-semver-checks-2025-year-in-review/)
* [Esta semana en Slatron v1.1: El inteligente director de la estación de televisión presenta presentadores DJ con IA con soporte TTS para emisoras de televisión hackeables y DIY, radio y señalización digital.](https://justinwoodring.com/blog/this-week-in-slatron-v1-1/)
* [Guía de migración SeaORM 2.0](https://www.sea-ql.org/blog/2026-01-12-sea-orm-2.0/)
* [BugStalker v0.4.0 - Depurador moderno para Linux x86-64. Escrito en programas Rust para Rust.](https://godzie44.github.io/BugStalker/docs/blog/release_0_4/) ahora con soporte para VSCode.
* [Anunciando cadd: una biblioteca para aritmética y conversiones comprobadas sin dolor](https://users.rust-lang.org/t/announcing-cadd-a-library-for-painless-checked-arithmetic-and-conversions/137423)
* [READ_ONCE(), WRITE_ONCE(), pero no para Rust](https://lwn.net/SubscriberLink/1053142/8ec93e58d5d3cc06/)
* [GuardianDB 0.14.0 - Base de datos descentralizada de alto rendimiento y localmente construida sobre Rust e Iroh](https://www.willsearch.com.br/guardiandb/)
* [Un año de trabajo en el proyecto ALPM](https://devblog.archlinux.page/2026/a-year-of-work-on-the-alpm-project/)
* [GlueSQL v0.19 añade asignación de parámetros y planificadores de consultas personalizables](https://github.com/gluesql/gluesql/releases/tag/v0.19.0)
* [guardia diésel: Tus migraciones de diésel podrían ser bombas de relojería](https://dev.to/ayarotsky/your-diesel-migrations-might-be-ticking-time-bombs-30g7).
* [Reseña del motor físico Rapier 2025 y objetivos 2026](https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge/)
* [Tako v0.5.0 → v0.7.1-2: de "buen router" a "mini plataforma"](https://rust-dd.com/post/tako-v0-5-0-v0-7-1-2-from-nice-router-to-mini-platform)

### Observaciones/Pensamientos

* [El estado de la criptografía de Rust en 2026](https://kerkour.com/rust-cryptography-ecosystem-2026)
* [La recogida de basura es contraria](https://trynova.dev/blog/garbage-collection-is-contrarian)
* [Integración de Lugares Virtuales y Checker de Préstamos](https://bennolossin.github.io/blog/field-projections/virtual-places-and-borrowck.html)
* [vídeo] [39c3 - Xous: Una reinterpretación pura del sistema operativo embebido](https://www.youtube.com/watch?v=BbWWGkyIBGM)
* [vídeo] [Decodificación rápida y segura de imágenes en Rust](https://www.youtube.com/watch?v=8ANzF7UwbZM)
* [vídeo] [ere: Compilación de expresiones regulares en tiempo de compilación](https://www.youtube.com/watch?v=3SFx-emI5r4)
* [vídeo] [Rust en Volvo Cars](https://www.youtube.com/watch?v=vBofCW8j70A)
* [audio] [Radar con Jeff Kao](https://corrode.dev/podcast/s05e08-radar/)

### Guías de Rust
[ES] [Patrón de mando en Rust: Cuando la intención no tiene que ser un objeto](https://codigolinea.com/patron-command-en-rust/) 

* [serie] [Parte 3: Arquitectura de modelos, construyendo un LLM desde cero en Rust](https://www.tag1.com/how-to/part3-model-architecture-building-an-llm-from-scratch/)
* [La guía del programador impaciente para Bevy and Rust: Capítulo 5 - Que haya pastillas](https://aibodh.com/posts/bevy-rust-game-development-chapter-5/)
* [audio] [Netstack.FM episodio 22 — URL de Rust con Simon Sapin](https://netstack.fm/#episode-22)

### Miscelánea
* [IBM Quantum: El Rust es real, pero la ventaja cuántica aún no)](https://filtra.io/rust/interviews/ibm-quantum-jan-26)

## Crate de la semana

El crate de esta semana es [diesel-guard](https://github.com/ayarotsky/diesel-guard), un linter contra peligrosas migraciones de Postgres.

¡Gracias a [Alex Yarotsky](https://users.rust-lang.org/t/crate-of-the-week/2704/1520) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.
Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

* *No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing) o
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [El esquema de impresión diésel - diésel produce un esquema incompilable con columna llamada "table"](https://github.com/diesel-rs/diesel/issues/4928)
* [GuardianDB - Crear Referencias](https://github.com/wmaslonek/guardian-db/issues/7)
* [GuardianDB - Crear ejemplos de uso cohesivos](https://github.com/wmaslonek/guardian-db/issues/5)
* [GuardianDB - Traducir documentación al inglés](https://github.com/wmaslonek/guardian-db/issues/3)
* [peldaño - Añadir completaciones de conchas para golpear/zsh/pez](https://github.com/auswm85/rung/issues/18)
* [ring] - Añadir ---bandera silenciosa para suprimir la salida no esencial](https://github.com/auswm85/rung/issues/19)
* [peldaño - Soporte NO_COLOR variable de entorno](https://github.com/auswm85/rung/issues/20)
* [peldaño - Añadir comandos de navegación de peldaño arriba / peldaño inferior](https://github.com/auswm85/rung/issues/21)
* [rung - Comando Add rung log para mostrar los commits de la pila](https://github.com/auswm85/rung/issues/22)
* [peldaño - Añadir prueba de integración para sincronización con conflictos de fusión](https://github.com/auswm85/rung/issues/23)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 18-01-2026 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-11

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

539 pull requests se [fusionaron en la última semana][fusionados]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-06..2026-01-13

#### Compilador
* [corrige 'Expr::can_have_side_effects' para '[x; Expresión literal y binaria de los arreglos de estilo N](https://github.com/rust-lang/rust/pull/150385)

#### Biblioteca
* [añadir 'AtomicPtr::null'](https://github.com/rust-lang/rust/pull/150736)
* ['Vec::p ush' en consts MVP](https://github.com/rust-lang/rust/pull/147893)
* [añadir parámetro asignador a 'HashMap'](https://github.com/rust-lang/rust/pull/148545)
* [implementa 'partial_sort_unstable' para corte](https://github.com/rust-lang/rust/pull/149318)
* [MVP de la reflexión](https://github.com/rust-lang/rust/pull/146923)
* [estabilizar 'Peekable::next_if_map' ('#![ Feature(peekable_next_if_map)]')](https://github.com/rust-lang/rust/pull/148941)
* [estabilizar 'corte::element_offset'](https://github.com/rust-lang/rust/pull/150777)

#### Carga
* ['docs(inestable)': ampliar la documentación para '-Zbuild-analysis'](https://github.com/rust-lang/cargo/pull/16476)
* ['feat(test)': Hacer que 'CARGO_BIN_EXE_' esté disponible en tiempo de ejecución](https://github.com/rust-lang/cargo/pull/16421)
* ['fix(build-std)': metadatos de enlace estándar se propagan al usuario](https://github.com/rust-lang/cargo/pull/16496)
* ['fix(info)': resolver la discrepancia de guion vs desajuste de guion en la consulta de esquema](https://github.com/rust-lang/cargo/pull/16455)
* ['fix(package)': detectar archivos sucios cuando se ejecutan desde un miembro del espacio de trabajo](https://github.com/rust-lang/cargo/pull/16479)
* [añadir grupos de pelusa tipo clippy](https://github.com/rust-lang/cargo/pull/16464)
* [añadir la bandera '--id' a 'tiempos de reporte de carga' y 'reconstrucciones de informes de carga'](https://github.com/rust-lang/cargo/pull/16490)
* [Reproducir la ruta del archivo de bloqueo en modo muy extenso al bloquear](https://github.com/rust-lang/cargo/pull/16491)
* [dote: solo en memoria 'Manifest'](https://github.com/rust-lang/cargo/pull/16409)
* [corregir(tiempo)!: eliminar '--timings=<FMT>' valores opcionales de formato](https://github.com/rust-lang/cargo/pull/16420)
* [corrección: preservar 'dep_name' para metadatos del script de compilación](https://github.com/rust-lang/cargo/pull/16494)
* [corregió una comparación incorrecta de versiones durante la selección de dependencia del script de compilación](https://github.com/rust-lang/cargo/pull/16486)
* [mejora el mensaje de error por dependencias ausentes](https://github.com/rust-lang/cargo/pull/16500)
* [aislar la proggestión de metadatos del script de compilación entre cajas STD y no STD](https://github.com/rust-lang/cargo/pull/16489)
* [refactorización: nuevo tipo para índice unitario](https://github.com/rust-lang/cargo/pull/16485)
* [prueba: añadir '-Zunstable-options' con objetivos personalizados](https://github.com/rust-lang/cargo/pull/16467)

#### Rustdoc
* ['rustdoc_json': Eliminar una llamada a 'std::mem::take' en 'after_krate'](https://github.com/rust-lang/rust/pull/150867)

#### Clippy
* [Un 'retorno' en un cierre de iterador no debería desencadenar 'never_loop'](https://github.com/rust-lang/rust-clippy/pull/16364)
* ['strlen_on_c_strings': mencionar el tipo específico ('CString o 'CStr')'](https://github.com/rust-lang/rust-clippy/pull/16391)
* ['float_point_arithmetic': respetar la aplicabilidad reducida](https://github.com/rust-lang/rust-clippy/pull/16366)
* ['single_range_in_vec_init': no apliques la sugerencia automáticamente](https://github.com/rust-lang/rust-clippy/pull/16365)
* ['unnecessary_map_or': respetar la aplicabilidad reducida](https://github.com/rust-lang/rust-clippy/pull/16387)
* ['useless_conversion': respetar la aplicabilidad reducida](https://github.com/rust-lang/rust-clippy/pull/16372)
* ['missing_enforced_import_rename': No hacer cumplir los guiones subrayados](https://github.com/rust-lang/rust-clippy/pull/16352)
* ['suspicious_to_owned': mejorar los mensajes de pelusa](https://github.com/rust-lang/rust-clippy/pull/16376)
* ['transmuting_null': Añadir comprobaciones para 'without_provenance' y 'without_provenance_mut'](https://github.com/rust-lang/rust-clippy/pull/16336)
* [añadir nueva pelusa de 'duration_suboptimal_units'](https://github.com/rust-lang/rust-clippy/pull/16250)
* [permite 'expect' en 'impl' para 'derive_ord_xor_partial_ord'](https://github.com/rust-lang/rust-clippy/pull/16303)
* [limpieza 'unnecessary_map_or' y 'manual_is_variant_and'](https://github.com/rust-lang/rust-clippy/pull/16386)
* [no ignorar afirmaciones antes de un 'interrupción' al simplificar el bucle](https://github.com/rust-lang/rust-clippy/pull/16379)
* [no mostrar los tramos de cajas externas en 'missing_trait_methods'](https://github.com/rust-lang/rust-clippy/pull/16380)
* [no advertir sobre grandes matrices de pila sin tener un espacio válido](https://github.com/rust-lang/rust-clippy/pull/16347)
* [no advertir sobre el efecto secundario aritmético de 'String'+'String'](https://github.com/rust-lang/rust-clippy/pull/16358)
* [mejora 'needless_collect' para cubrir a 'push' vec similar a la PUSH](https://github.com/rust-lang/rust-clippy/pull/16305)
* [corregir 'LimitStack::p op_atr' en versiones de lanzamiento](https://github.com/rust-lang/rust-clippy/pull/16371)
* [corregir 'clippy_utils::std_or_core(_)' marcando las cajas de 'no_core' como 'std'](https://github.com/rust-lang/rust-clippy/pull/16374)
* [corregir 'map_unwrap_or' no cubre 'Resultado::unwrap_or'](https://github.com/rust-lang/rust-clippy/pull/15718)
* [corrección 'significant_drop_tightening' sugiere erróneamente para uso no por método](https://github.com/rust-lang/rust-clippy/pull/16355)
* [corregir macros 'str_to_string' mal desordenadas](https://github.com/rust-lang/rust-clippy/pull/16276)
* [corregir macros 'unnecessary_to_owned' mal desordenadas](https://github.com/rust-lang/rust-clippy/pull/16354)
* [arreglar: restringir 'match_bool' a 2 brazos](https://github.com/rust-lang/rust-clippy/pull/16333)
* [mejora la sugerencia 'useless_conversion .into_iter()' para referencias anidadas](https://github.com/rust-lang/rust-clippy/pull/16238)
* [más correcciones para el manejo de macros](https://github.com/rust-lang/rust-clippy/pull/16337)
* [revisión 'int_plus_one'](https://github.com/rust-lang/rust-clippy/pull/16373)

#### Analizador de Rust
* [añadir heredar atributos para 'extract_function' asistencia](https://github.com/rust-lang/rust-analyzer/pull/21442)
* [configurar flycheck usando workspace.discoverConfig](https://github.com/rust-lang/rust-analyzer/pull/18043)
* [permitir rutas de Rust en la búsqueda de símbolos](https://github.com/rust-lang/rust-analyzer/pull/21415)
* [corregir la bandera de ignorar para atributos de prueba con valores](https://github.com/rust-lang/rust-analyzer/pull/21436)
* [arreglo pierde existencia y guarda para 'move_guard'](https://github.com/rust-lang/rust-analyzer/pull/21412)
* [corrección no aplicable en la declaración para 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/20946)
* [corrección no completa 'mut' y 'raw' en '&x.foo()'](https://github.com/rust-lang/rust-analyzer/pull/21451)
* [arreglar no desactivar los resaltos de escape de cadenas](https://github.com/rust-lang/rust-analyzer/pull/21420)
* [desactivar las advertencias de 'unused_variables' y 'unused_mut'](https://github.com/rust-lang/rust-analyzer/pull/21445)
* [corregir la búsqueda raíz de caja en símbolos del mundo que duplican entradas raíz](https://github.com/rust-lang/rust-analyzer/pull/21446)
* [Arreglar los diagnósticos de LEN para punteros FN de tiempos de vida](https://github.com/rust-lang/rust-analyzer/pull/21432)
* [correcciones para expansiones derivadas integradas](https://github.com/rust-lang/rust-analyzer/pull/21421)
* [ocultar importaciones renombradas de macros en el índice de símbolos](https://github.com/rust-lang/rust-analyzer/pull/21459)
* [bajando el bloqueo con predicados de superrasgo](https://github.com/rust-lang/rust-analyzer/pull/21364)
* [¡haz 'naked_asm! ()' siempre devuelven '!'](https://github.com/rust-lang/rust-analyzer/pull/21456)
* [correctamente bajar los predicados 'Solo-Yo'](https://github.com/rust-lang/rust-analyzer/pull/21399)
* [eliminar código hecho redundante por reescritura de resolución de método](https://github.com/rust-lang/rust-analyzer/pull/21434)
* [sugiere rasgos distintos a los de la caja ambiental](https://github.com/rust-lang/rust-analyzer/pull/21414)
* [Sync Cast vuelve a comprobar a RUSTC](https://github.com/rust-lang/rust-analyzer/pull/21462)
* [implementa 'Span::ByteRange' para proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21416)
* [migrar 'generate_mut_trait_impl' asistir para usar SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21443)

### Triaje de rendimiento del compilador Rust

Semana bastante tranquila, la mayoría de los cambios se deben a nuevas características que, naturalmente, incorporan algunas
gastos generales para programas existentes. En general, aunque una pequeña mejora.

Triaje hecho por **@simulacrum**.
Rango de revisión: [7c04f5d2.. 840245e9](https://perf.rust-lang.org/?start=7c04f5d216b5dcfff0a55fc20327a1c519004699&end=840245e91b90f22adf9f26d0a0cd98c2416cdef3&absolute=false&stat=instructions%3Au)

3 regresiones, 1 mejora, 4 mixtas; 2 de ellos en rollups
31 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-01-12.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

<!-- Utiliza * [Título del artículo](URL del artículo) - o * *No se han aprobado RFCs esta semana.* -->

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Eliminar los archivos de Fluent](https://github.com/rust-lang/compiler-team/issues/959)

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar ensamblaje en línea ppc](https://github.com/rust-lang/rust/pull/147996)
* [Evaluación: siempre haz copias de mem a mem si puede haber relleno](https://github.com/rust-lang/rust/pull/148967)
* [Problema de seguimiento para 'Vec::p ush_mut'](https://github.com/rust-lang/rust/issues/135974)
* [Problema de seguimiento para 'error_generic_member_access'](https://github.com/rust-lang/rust/issues/99301)
* [FCW Lint cuando se usa un rasgo importado de globo ambiguo](https://github.com/rust-lang/rust/pull/149058)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [CFGS con tipo de versión](https://github.com/rust-lang/rfcs/pull/3905)
* [Sea 'Opción' derivar '#[must_use]](https://github.com/rust-lang/rfcs/pull/3906)

## Próximos eventos

Eventos Rusty entre el 14-01-2026 y el 11-02-2026 🦀

### Virtual
* 2026-01-15 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-15 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-15 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-16 | Virtual (Tel Aviv-yafo, IL) | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/)
    * [**תרומה לפרויקט קוד פתוח שכתוב בראסט - מפגש ווירטואלי**](https://www.meetup.com/rust-tlv/events/312781560/)
* 2026-01-18 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Lectura de código oxidado y contribución de código abierto (UTC 16:00; Inglés)**](https://www.meetup.com/code-mavens/events/312710291/)
* 2026-01-20 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/e2ea7hxo)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-27 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-27 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Almuerzo y aprendizaje: Manejo de errores en Rust**](https://www.meetup.com/women-in-rust/events/312799344/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/9h9n0dau)
* 2026-01-29 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Encuentro de Bevy #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #2 - Procesos oxidados, límites de memoria y cápsulas básicas**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/5bu9kas1)

### Asia
* 2026-01-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #12**](https://www.meetup.com/rustdelhi/events/312584516/)
* 2026-02-05 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Encuentro de Malasia Rust febrero 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)

### Europa
* 2026-01-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-14 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-16 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café de día)**](https://www.meetup.com/rust-and-friends/events/312662987/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por confirmar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #82**](https://www.meetup.com/rust-paris/events/312364675/)
* 2026-01-21 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 26-01-2026 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #17**: Emily Coaca - Entwicklung des Kernels Update für TockOS](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-28 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - enero 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-01-29 | Ostrava, CZ | [Encuentro con Actualización Ostrava](https://www.meetup.com/meetupdate-ostrava/)
    * [**MeetUpdate Ostrava #28: Rust**](https://www.meetup.com/meetupdate-ostrava/events/312747904)
* 2026-02-04 | Darmstadt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Escribiendo un servicio de suscripción a un boletín con axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Más allá del código: Diseñando servicios que resisten la prueba del tiempo**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Hack Rust & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Hack y Aprendizaje de Karlsruhe Meetup en BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Lectura de Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjcdbpb/)

### Norteamérica
* 2026-01-14 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/312722481/)
* 2026-01-15 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust Común de Boston, 17 de enero**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-17 | Herndon, VA, EE. UU. | [NoVaLUG](https://mobilizon.us/@novalug)
    * [**Reunión mensual - Enfadar a Brian Lunduke, programa en Rust**](https://mobilizon.us/events/140c5c7c-01f3-4aaa-b218-58289c6b4449)
* 2026-01-20 | San Francisco, CA, EE. UU. [Svix](https://luma.com/calendar/cal-Cnmn4RR2n4fRUNZ)
    * [**Encuentro de San Francisco Rust**](https://luma.com/1wle4wl0)
* 2026-01-20 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-22 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Encuentro nocturno de Boston Rust con Jiff, 22 de enero**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Davis Square Rust, 24 de enero**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Ángeles: Construyendo reemplazos de Git-LFS en Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Programación Rust 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust del Noreste, 31 de enero**](https://www.meetup.com/bostonrust/events/312483767/)
* 2026-02-03 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Renderizado y tiempos de construcción de Bevy en Amazon**](https://www.meetup.com/rust-nyc/events/312871242/)
* 2026-02-05 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Renderizando el set de Mandelbrot en Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Allston Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/312483562/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> he escrito en decenas de lenguajes informáticos, incluidos algunos especializados que eran internos de Pixar (incluido uno que diseñé). Pasé décadas escribiendo en C y C++. Escribí microcódigo bit-slice, programado para SIMD antes de que mucha gente fuera de Pixar lo tuviera.
> 
> escribí el primer depurador malloc que detenía tu depurador en la línea del código fuente, ese era el problema. Los fabricantes de estaciones de trabajo Unix tuvieron que hacer un lanzamiento inesperado cuando esto reveló todos los problemas en sus bibliotecas C.
> 
> soy mejor programador en Rust para todo lo de bajo nivel o alto rendimiento. Simplemente me evita cometer toda una serie de errores que eran demasiado fáciles de cometer en cualquier idioma sin necesidad de recoger basura.
> 
> A largo plazo, cualquier cosa que mejore la calidad va a ganar. Hay mucho dolor de estómago por parte de personas que están demasiado enamoradas de lo que llevan usando durante décadas, pero en su mayoría es libre de sustancias. Como que la gente se dé cuenta de que el código marcado como "inseguro" es, sorpresa, inseguro. Y que lo inseguro puede ser abusado.

– [Bruce Perens en LinkedIn](https://www.linkedin.com/posts/bruce-perens_i-have-written-in-dozens-of-computer-languages-activity-7413127858266734592-iMc5)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1746) por la sugerencia!

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

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1qd8zx4/this_week_in_rust_634/)</small>
