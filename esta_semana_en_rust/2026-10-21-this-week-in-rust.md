---
title: "Esta semana en Rust #130"
number_of_week: 130
description: El crate de esta semana es eilmeldung, un lector TUI RSS. 
date: 2026-10-21
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

* [Anunciando Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)
* [Anunciando el grupo de notificaciones de la GPU y el Área de Trabajo del Linker](https://blog.rust-lang.org/inside-rust/2026/10/20/gpu-target-notification-group/)

### Boletines

* [Rust Trends Número 85 - Si se compila, el agente lo hizo bien](https://rust-trends.com/newsletter/if-it-compiles-the-agent-got-it-right/)

### Actualizaciones de proyectos/herramientas

* [Presentando tsrs: un port de TypeScript para Rust](https://blog.cloudflare.com/typescript-rust-port/)

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [Cloudflare está reescribiendo todo en Rust](https://kerkour.com/rust-at-cloudflare)
* [Qué significan los agentes para Rust (y viceversa)](https://smallcultfollowing.com/babysteps/blog/2026/10/14/agentic-dev-and-rust/)
* [HashDoS de caja negra: atacar tablas hash sin conocer la función hash](https://orlp.net/blog/black-box-hashdos/)
* [2026 es un año emocionante para el compilador de Rust (y para mí)](https://yoyo-code.com/2026-is-exciting-year-for-rust-compiler-and-me/)
* [Lo que realmente mide el recuento de descargas crates.io: 90 días de datos diarios de 69 cajas](https://tensorust.com/articles/crates-io-download-count/)
* [Estructuras de datos autorreferenciales seguras en kernel Rust](https://lwn.net/SubscriberLink/1099224/54c55e87a0828522/)
* [Declaraciones de avance cruzadas en Rust](https://lwn.net/SubscriberLink/1099596/7bfb3ee846a5f102/)
* [Por qué los manejadores de mensajes de tellus son síncronos](https://heikoseeberger.de/2026-10-18-why-sync-handlers/)
* [Upstream Tyr: Rust, DRM y el núcleo de Linux](https://www.collabora.com/news-and-blog/blog/2026/10/21/upstreaming-tyr-rust-drm-and-the-linux-kernel/)

### Guías de Rust

* [He forzado un modelo de voz en la GPU durante meses. La CPU era más rápida.](https://altunenes.github.io/posts/reuse/)
* [Superoptimizando un emulador de FPU](https://purplesyringa.moe/blog/superoptimizing-an-fpu-emulator/)
* [Dibujar y animar la spinadora de carga con GPUI](https://hlcfan.github.io/gpui-canvas-spinner.html)
* [serie] [Codificación TCP en el sistema de tipos de Rust, Parte 1: Un número de secuencia no es un 'u32'](https://noahkawaguchi.com/blog/typenet-post-1/)
* [serie] [Ejecutando 10 millones de CLIs de códice en un servidor - procesos LFI](https://kapteko.com/running-10-million-codex-clis-on-one-server-lfi-processes/)

### Investigación

* [Un enfoque incremental para la construcción del JIT](https://doi.org/10.1145/3830440.3832304)
* [vídeo] [[Scheme'26] Un enfoque incremental para la construcción del JIT](https://www.youtube.com/watch?v=8qwXgappTWQ)

### Miscelánea

## Crate de la semana

El crate de esta semana es [eilmeldung](https://crates.io/crates/eilmeldung), un lector TUI RSS. 

Llogiq está bastante satisfecho con su elección, a pesar de la lamentable falta de sugerencias. 

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

ScreenCue

* [ScreenCue - Añadir un formulario reproducible de informe de error en el escritorio](https://github.com/CrioNIK/ScreenCue/issues/1)
<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->

sysknife

* [sysknife - README dice que la puerta de aprobación de la CLI es sysknife aprender; la CLI aprueba en proceso tras su propio prompt](https://github.com/lacs-project/sysknife/issues/560)
* [sysknife - --run-run exit 0 para un plan que rompe --max-risk o que esta distribución no pueda ejecutar](https://github.com/lacs-project/sysknife/issues/561)
* [sysknife - Reejecutar sysknife-setup mientras el daemon se ejecuta falla con ETXTBSY y puede dejar binarios desajustados](https://github.com/lacs-project/sysknife/issues/563)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Manguebyte

* [Manguebyte - Poner ParamSmoother bajo la protección de asignación](https://github.com/alessonqueirozdev-hub/manguebyte/issues/22)
* [Manguebyte - Trata un retraso NaN como el retraso más corto en DelayLine::read y Delay::set_time](https://github.com/alessonqueirozdev-hub/manguebyte/issues/23)
* [Manguebyte - Revisa avisos de seguridad en el trabajo de denegación de carga](https://github.com/alessonqueirozdev-hub/manguebyte/issues/24)
* [Manguebyte - Permite que un plugin informe su cola a los hosts](https://github.com/alessonqueirozdev-hub/manguebyte/issues/25)
* [Manguebyte - Añadir widgets de audio, empezando por un teclado en pantalla](https://github.com/alessonqueirozdev-hub/manguebyte/issues/7)

Krate

* [krate - Corregir dos advertencias en los objetivos de wasm32 y Windows (fácil)](https://github.com/incyashraj/krate/issues/32)
* [krate - Linux: sigue el modo oscuro y el tamaño del texto (medio)](https://github.com/incyashraj/krate/issues/28)
* [krate - krate pack: añadir una bandera --keep-debug (media)](https://github.com/incyashraj/krate/issues/29)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

601 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-13..2026-10-20

#### Compilador
* [mejorar el diagnóstico de literales en atributos de pelusa](https://github.com/rust-lang/rust/pull/164486)
* [mejorar el diagnóstico para elementos asociados no bien formados](https://github.com/rust-lang/rust/pull/162432)
* [fijar el marcador de posición 'diagnostic::on_unknown' para rutas en expresiones y rutas de tipos](https://github.com/rust-lang/rust/pull/164178)
* [mover 'fully_perform' regreso anticipado antes](https://github.com/rust-lang/rust/pull/164236)
* [revisión de la compresión del tramo](https://github.com/rust-lang/rust/pull/163857)
* [perf: error de caída en 'unwrap_failed' en vez de 'unwrap'](https://github.com/rust-lang/rust/pull/164059)
* [atributos de la pelusa del puerto](https://github.com/rust-lang/rust/pull/162811)
* [proporciona contexto apuntando a rasgo/objeto al hablar de objetos asociados](https://github.com/rust-lang/rust/pull/163383)
* [restaurar el comportamiento antiguo de 'derive(Default)' en tipos sin campo](https://github.com/rust-lang/rust/pull/164114)
* [saltar movimientos cuando el límite de tamaño es cero](https://github.com/rust-lang/rust/pull/164332)
* [estabilizar '-Zembed-metadata'](https://github.com/rust-lang/rust/pull/163436)
* [función de soporte args const generics](https://github.com/rust-lang/rust/pull/162727)

#### Biblioteca
* [añadir 'codeview_annotation' intrínseco](https://github.com/rust-lang/rust/pull/160285)
* [añadir 'núcleo::p rocess::resumable_breakpoint'](https://github.com/rust-lang/rust/pull/163611)
* [haz que 'std::fs::{File, ReadDir, DirEntry}' siempre 'needs_drop' incluso cuando no está soportado](https://github.com/rust-lang/rust/pull/162444)
* [facilita el encadenamiento de tupla 'ParcialOrd' en mir-opt](https://github.com/rust-lang/rust/pull/164369)
* [ventanas: estabilizar 'inherit_handles'](https://github.com/rust-lang/rust/pull/161163)

#### Carga
* ['build-std': Añadir build.build-std.when y crates config keys](https://github.com/rust-lang/cargo/pull/17602)
* ['prerelease': soporte '[parche]' con '-Zprerelease'](https://github.com/rust-lang/cargo/pull/17613)
* ['config': Aplicar soporte de tuplas con puntos para rustflags](https://github.com/rust-lang/cargo/pull/17594)
* ['config': No cargues la configuración del registro desde credentials.toml](https://github.com/rust-lang/cargo/pull/17599)
* ['huella digital': elige un camino estable cuando la fuente se relaciona](https://github.com/rust-lang/cargo/pull/17521)
* ['util-schemas': compila la pista de ruta de archivo donde la URL no puede compilarla](https://github.com/rust-lang/cargo/pull/17600)
* [completación de shell muestra todos los paquetes que tienen esta característica](https://github.com/rust-lang/cargo/pull/17574)
* [comprobar el tipo de caja al podar los caminos '-L'](https://github.com/rust-lang/cargo/pull/17589)

#### Rustfmt
* [config: corregir la búsqueda de archivos de configuración globales](https://github.com/rust-lang/rustfmt/pull/7179)
* [no eliminar args genéricos vacíos al formatear en un contexto macro](https://github.com/rust-lang/rustfmt/pull/7181)
* [hacer cumplir el ancho de comentario en las listas de ítems](https://github.com/rust-lang/rustfmt/pull/7174)
* [arreglar la recuperación del analizador 'cfg_select!' manejo y fuga de error de la sesión del analizador](https://github.com/rust-lang/rustfmt/pull/7171)
* [declaración de corrección terminando el comentario](https://github.com/rust-lang/rustfmt/pull/7170)
* [impide que Rustfmt elimine Attraction de expresión al eliminar paréntesis anidados](https://github.com/rust-lang/rustfmt/pull/7192)
* [impiden que Rustfmt retire los bloques necesarios en los brazos de 'cfg_select!'](https://github.com/rust-lang/rustfmt/pull/7183)

#### Rustdoc
* [añadir desambiguadores para método con nombre idéntico en la barra lateral](https://github.com/rust-lang/rust/pull/162778)
* [mejoras para 'rustc_must_implement_one_of'](https://github.com/rust-lang/rust/pull/164266)

#### Clippy
* [permitir 'allow_attributes' desde dentro de macros](https://github.com/rust-lang/rust-clippy/pull/17863)
* [cancelar todos los lints de doc cuando los atributos provienen de una expansión macro externa](https://github.com/rust-lang/rust-clippy/pull/17866)
* [reasignación de recogida clara](https://github.com/rust-lang/rust-clippy/pull/16549)
* [extiende 'from_str_radix_10' a tipos NonZero](https://github.com/rust-lang/rust-clippy/pull/17715)
* [la sugerencia de 'try_from_instead_of_from_str' de fijar la pelusa causa error cuando el parámetro 'TryFrom::try_from' no se llama 'value'](https://github.com/rust-lang/rust-clippy/pull/17881)
* [corregir el bucle infinito 'wildcard_imports' en las importaciones de globos generadas por macro](https://github.com/rust-lang/rust-clippy/pull/17828)
* [corregir punto y coma final con 'nonstandard_macro_braces'](https://github.com/rust-lang/rust-clippy/pull/17879)
* [nueva pelusa: 'bufreader_stdin'](https://github.com/rust-lang/rust-clippy/pull/17029)
* ['useless_attribute': ya no pelusa 'clippy::allow_attributes'](https://github.com/rust-lang/rust-clippy/pull/17865)

#### Analizador de Rust
* ['viewMir' entrando en pánico en código que contiene proyecciones de campo](https://github.com/rust-lang/rust-analyzer/pull/23513)
* [salen de 'struct_tail_erasing_lifetimes' en estructuras infinitas](https://github.com/rust-lang/rust-analyzer/pull/23504)
* [construye nombres no ambiguos en cajas con múltiples objetivos](https://github.com/rust-lang/rust-analyzer/pull/23173)
* [emitir E0029 para patrones de rango con tipos inválidos inferidos](https://github.com/rust-lang/rust-analyzer/pull/23533)
* [completaciones de nombres de módulos de palabras clave escape](https://github.com/rust-lang/rust-analyzer/pull/23518)
* [mejora la completación de atributos sin fragmentos](https://github.com/rust-lang/rust-analyzer/pull/23523)
* [OPS: Resolver 'ref-version-mismatch', 'stale-action-refs' y 'unpinned-uses'](https://github.com/rust-lang/rust-analyzer/pull/23488)

### Triaje de rendimiento del compilador Rust

Una semana dominada por cambios de pequeña magnitud. Sin embargo, varios
Las mejoras logradas resultan en una mejora media general del 1,4%. 
Lo cual es bastante agradable de ver. 

Triaje hecho por **@simulacrum**. 
Rango de revisión: [7211EA59.. BD4C6cc4](https://perf.rust-lang.org/?start=7211ea594cbf137ef6118b1752b959dea7cb9088&end=bd4c6cc4aeef0eb9919c6f73c1e690677e9c6b38&absolute=false&stat=instructions%3Au)

6 regresiones, 9 mejoras, 8 mixtas; 0 de ellas en rollups
43 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-10-19.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/main)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* [Arreglar los libs nominados triagebot.toml](https://github.com/rust-lang/rfcs/pull/4022)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [núcleo: Añadir 'assert_matches' y 'debug_assert_matches' al siguiente preludio](https://github.com/rust-lang/rust/pull/164328)
* [Más abajo #[no_mangle] a #[symbol_name]/#[link_name]](https://github.com/rust-lang/rust/pull/159374)
* [estabilizar cuerda::into_chars](https://github.com/rust-lang/rust/pull/163816)
* [estabilizar {Rc, Arc}::is_unique](https://github.com/rust-lang/rust/pull/163458)
* [VEC: estabilizar asignadores para into_iter](https://github.com/rust-lang/rust/pull/163499)
* [No requieren 'inseguro' para patrones de estructuras y arrays contra campos de unión](https://github.com/rust-lang/rust/pull/161771)
* [Añadir una advertencia futura de incompat para #[no_link]](https://github.com/rust-lang/rust/pull/164028)
* [error de ambigüedad de elevación en la macro de atributo que podría ser el atributo de la herramienta](https://github.com/rust-lang/rust/pull/162597)
* [estabilizar '-Znext-solver=global'](https://github.com/rust-lang/rust/pull/164209)
* [Estabilizar '--remap-path-scope' en rustdoc y el alcance de 'documentación'](https://github.com/rust-lang/rust/pull/163895)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Añadir soporte para el formato de salida 'no-graphics' de 'annotate-snippets'](https://github.com/rust-lang/compiler-team/issues/1045)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Desestimar 'std::env::{vars, args}'](https://github.com/rust-lang/rfcs/pull/4021)
* [RFC: documentar un contrato 'Future' estricto que prohíbe posponerse](https://github.com/rust-lang/rfcs/pull/4019)
* [RFC: externo "Rust_manyregs" ABI](https://github.com/rust-lang/rfcs/pull/4018)
* [RFC: añadir 'poll_progress' a 'AsyncIterator'](https://github.com/rust-lang/rfcs/pull/4020)

## Próximos eventos

Eventos Rusty entre 2026-10-21 - 2026-11-18 🦀

### Virtual
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Mecanismos de Paginación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-222 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust y la GPU de Nativo a Web: Introducción a 'wgpu'](https://luma.com/k1978ath)
* 2026-10-222 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-26 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Ningún Python resultó dañado: enseñando a un pequeño MCU a aprender sobre la marcha**](https://luma.com/1byfc495)
* 2026-10-27 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Razonamiento con Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creando un editor hexadecimal en Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773682/)
* 04-11-2026 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 05-11-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackeo y Aprendiza**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254770/)
* 2026-11-12 | Virtual (Núremberg, DE) | [Núremberg Rust](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust Maldito**](https://luma.com/omlooqv8)
* 2026-11-15 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Compilación errónea en Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: El arnés llevaba gabardina - Construyendo un servidor MCP de Rust**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcpbwb/)
* 2026-11-18 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Escribiendo un firmware para lector Xteink en Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Rust en persona noviembre de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**Encuentro de Rustacean de noviembre de 2026](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europa
* 2026-10-22 | Karlsruhe, DE | [Rust Hack & Aprende Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Hack y Aprendizaje de Karlsruhe Hack and Learn bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/316799366/)
* 2026-10-22 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Interoperabilidad entre Rust y Python**](https://www.meetup.com/rust-community-toulouse/events/316880185/)
* 2026-10-27 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Rust se encuentra con IA**](https://www.meetup.com/rust-aarhus/events/316796329/)
* 2026-10-27 | París, FR | [París Oxidado](https://www.meetup.com/rust-paris)
    * [**Rust meetup #89**](https://www.meetup.com/rust-paris/events/316865643/)
* 2026-10-28 | Copenhague, DK | [Comunidad Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #72 patrocinado por Google!**](https://www.meetup.com/copenhagen-rust-community/events/316545936/)
* 2026-10-31 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 01-11-2026 - 3 de 11-2026 | Italia, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - La Conferencia Internacional sobre el Rust en Italia**](https://rustlab.it/schedule)
* 2026-11-04 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Empezando con el nuevo controlador Rust para la base de datos de IA de Oracle**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milán, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Trenes Modelo Digitales & Rust: Estación de Mando DCC de Metal Desnudo en ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 2026-11-17 | Leipzig, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Todo es Git, los protocolos son solo adaptadores**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)

### Norteamérica
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Encuentro Incrustado**](https://luma.com/ur4pm34i)
* 2026-10-22 | Nueva York, NY, EE.UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC con Bryan Cantrill - Code Matters: Reflexiones sobre Rust en la era de los LLM**](https://www.meetup.com/rust-nyc/events/316901518/)
* 2026-10-24 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Davis Square Rust, 24 de octubre**](https://www.meetup.com/bostonrust/events/316924143/)
* 2026-10-28 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316655403/)
* 31-10-2026 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en el North End, 31 de octubre**](https://www.meetup.com/bostonrust/events/316924288/)
* 05-11-2026 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Social mensual de noviembre**](https://www.meetup.com/rust-montreal/events/316919004/)
* 05-11-2026 | San Francisco, CA | Rust LA: NVIDIA! 
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://luma.com/iqe7wng1)
* 05-11-2026 | Saint Louis, MO, EE.UU. [Rust STL](https://www.meetup.com/stl-rust)
    * [**HTMX y Rust**](https://www.meetup.com/stl-rust/events/316896797/)
* 2026-11-12 | San Diego, CA, EE.UU. | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, 14 de noviembre**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Ángeles, CA, EE.UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Rust LA November con Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Escribiendo un firmware para lector Xteink en Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)

### Sudamérica
* 2026-11-07 | Planalto Paulista, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1wzctie/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Para mí, parece que las mismas cosas que hacen que Rust sea un objetivo deseable para código generado por LLM son las mismas cosas que lo hacen deseable para mí como humano escribiendo código, que es el fantástico feedback del compilador y las herramientas de primera clase. 

– [Christian Haynes sobre rust-users](https://users.rust-lang.org/t/recent-explosion-in-rust-rewrites-and-reimplementations-with-llms-thoughts/142939/33)

¡Gracias a [Moy2010](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1829) por la sugerencia! 

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1xc54av/this_week_in_rust_674/)</small>