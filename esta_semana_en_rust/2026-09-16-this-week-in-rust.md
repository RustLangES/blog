---
title: "Esta semana en Rust #126"
number_of_week: 126
description: El crate de esta semana es zenjpeg, un codificador y decodificador JPEG puro de Rust. 
date: 2026-09-16
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

### Boletines

* [Rust Trends Número 82 - Incluso el Linker se está reescribiendo en Rust](https://rust-trends.com/newsletter/even-the-linker-is-getting-rewritten-in-rust/)
* [El Rustacean Incrustado Número #80](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-80)

### Actualizaciones de proyectos/herramientas

* [Slint 1.18 Lanzado](https://slint.dev/blog/slint-1.18-released)

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [¿Dónde pertenece el Rust en Arduino? Si pertenece.](https://talmondrlm.medium.com/where-does-rust-belong-on-arduino-if-it-belongs-325cbee63c1a)
* [CO3: Hacia el FFI óptimo](https://mversic.github.io/co3/)
* [Por qué es difícil construir un LSP Rust · Rust Glancer](https://rust-glancer.github.io/blog/why-lsp-is-hard/)
* [Desarrollando código Rust demostrablemente correcto con Verus](https://www.amazon.science/blog/developing-provably-correct-rust-code-with-verus)
* [Principios para aplicaciones rápidas de Tokio](https://dial9-rs.github.io/blog/principles-for-fast-tokio-applications/)

### Guías de Rust

* [intentando hacer que un bucle se auto-vectorice](https://jsgroth.dev/blog/posts/trying-to-make-a-loop-auto-vectorize/)
* [¿Rust soporta la herencia? Sí, no, y quizá, todo en el mismo archivo](https://msj.prose.sh/does-rust-support-inheritance)
* [Bibliotecas estáticas de Rust de envío sin colisiones de símbolos](https://ai-coustics.com/blog/libpatcher)

### Guías de Rust

* [Guía visual de Rust asíncrona](https://akesson.io/a-visual-guide-to-rust-async/)
* [Proyectos Rust - Escribe un clon de Redis - Versión 3.0.0](https://rust-projects-write-a-redis-clone.github.io/#3.0.0)
* [¿Se puede usar ESP32 como programador SWD para STM32 con Rust?](https://blog.implrust.com/posts/2026/09/swd-protocol-programmer-embedded-rust/)
* [Trampas de tiempo y pánico en WebAssembly: compila, pero se cierra en el navegador](https://rust-blog.github.io/post/wasm-time-panic-traps)
* [Un candado para gobernarlos a todos](https://flakm.com/posts/sqlx_migration_wrapper_til/)
* [Operadores de la muerte: aritmética comprobada en Rust](https://bitfieldconsulting.com/posts/operators-of-death)
* [Genéricos de Rust: de despacho estático a dinámico](https://kerkour.com/rust-generics)
* [vídeo] [Tu primera app de GPUI - Creando una interfaz de escritorio en Rust](https://www.youtube.com/watch?v=NT2XPvtof-Y)

### Investigación

* [Optimizando una sola pelusa Clippy por 3133X](https://blog.goose.love/posts/making-a-clippy-lint-faster-by-3133x/)

## Crate de la semana

El crate de esta semana es [zenjpeg](https://lib.rs/crates/zenjpeg), un codificador y decodificador JPEG puro de Rust. 

¡Gracias a [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1669) por la sugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

##### [Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para trim-paths RFC 3127](https://github.com/rust-lang/cargo/issues/12137)
  * [Instrucciones de prueba](https://github.com/rust-lang/cargo/issues/12137#issuecomment-5607218160)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
- *No se presentaron convocatorias para participar esta semana.* 

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->
- *No se presentaron convocatorias ni presentaciones esta semana.* 

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

523 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-08..2026-09-15

#### Compilador
* [recopilación de viejas sesiones incrementales de recopilación de basura](https://github.com/rust-lang/rust/pull/162240)
* [menos clones y más limpieza para 'rustc_builtin_macros'](https://github.com/rust-lang/rust/pull/162234)
* [perf: leer datos de higiene una vez al hacer hash de contextos sintáctics](https://github.com/rust-lang/rust/pull/162571)
* [usa 'DenseBit' para 'drop_live_at' en el trazado de liveness](https://github.com/rust-lang/rust/pull/162488)
* [usar 'Box<[Word]>' para almacenar palabras en 'DenseBitSet'](https://github.com/rust-lang/rust/pull/161957)

#### Biblioteca
* [generaliza (la mayoría) impls en 'Box'](https://github.com/rust-lang/rust/pull/161946)
* [implementa 'Thread::os_id'](https://github.com/rust-lang/rust/pull/160219)
* [reservar elementos en implementaciones 'Extender'](https://github.com/rust-lang/rust/pull/162495)
* [estabilizar 'Vec::from_fn'](https://github.com/rust-lang/rust/pull/162685)
* [estabilizar 'núcleo::mem::D ropGuard'](https://github.com/rust-lang/rust/pull/161520)
* [estabilizar 'unsafe_cell_access'](https://github.com/rust-lang/rust/pull/162504)

#### Carga
* ['fix(git)': Para git cli, indica a los usuarios qué configuración no estamos reenviando en caso de error](https://github.com/rust-lang/cargo/pull/17477)
* ['fix(install)': usar el archivo de bloqueo empaquetado por defecto](https://github.com/rust-lang/cargo/pull/17388)
* [arreglo(trim-paths)!: desmapear archivo en un documento JSON](https://github.com/rust-lang/cargo/pull/17476)
* [baja el nivel de pelusa de 'manual_readme' y 'non_kebab_case_bins' a 'permitir'](https://github.com/rust-lang/cargo/pull/17478)
* [especificar '--edición' en las pruebas de 'mensajes'](https://github.com/rust-lang/cargo/pull/17470)
* [prueba: añadir pruebas más completas de unificación de características de espacios de trabajo](https://github.com/rust-lang/cargo/pull/17466)

#### Rustfmt
* [no trates un identificador en bruto como un prefijo de cadena en bruto](https://github.com/rust-lang/rustfmt/pull/7110)
* [corregir el ajuste de 'max_width' dentro de macros](https://github.com/rust-lang/rustfmt/pull/6651)
* [Reserva el ancho para 'const' al formatear bloques const en línea](https://github.com/rust-lang/rustfmt/pull/7065)

#### Clippy
* ['map_clone': evitar sugerencias tras coerciones de cambio de tipo](https://github.com/rust-lang/rust-clippy/pull/17671)
* [corregir 'collapsible_match' sugiriendo erróneamente para código compilado condicional](https://github.com/rust-lang/rust-clippy/pull/16942)
* [activar 'manual_swap' en contextos const](https://github.com/rust-lang/rust-clippy/pull/17703)

#### Analizador de Rust
* [caché macro-expandidas raíces al trepar ancestros](https://github.com/rust-lang/rust-analyzer/pull/23359)
* [no rellenar métodos inestables en "Implementar miembros predeterminados"](https://github.com/rust-lang/rust-analyzer/pull/23318)
* [no te pongas nervioso en JSON con nombre de campo inválido](https://github.com/rust-lang/rust-analyzer/pull/23330)
* [no te pongas nervioso en los comentarios del documental adjuntos a expresiones literales](https://github.com/rust-lang/rust-analyzer/pull/23295)
* [corrigir 'hir::Type' desajustes de propietario entre consts anónimos](https://github.com/rust-lang/rust-analyzer/pull/23315)
* [arreglar el pánico cuando el solucionador de rasgos vuelve a entrar](https://github.com/rust-lang/rust-analyzer/pull/23367)
* [arreglar el pánico cuando llamamos a 'impls_trait' para el tipo self de derivación incorporada para tipos genéricos](https://github.com/rust-lang/rust-analyzer/pull/23352)
* [para en el desbordamiento de recursións de macros entusiastas](https://github.com/rust-lang/rust-analyzer/pull/23323)
* [ide: cálculo de offset de comentario en el documento de corrección](https://github.com/rust-lang/rust-analyzer/pull/23300)

### Triaje de rendimiento del compilador Rust

Casi no hubo regresiones esta semana, ¡y varias mejoras en el rendimiento! Aunque algunas de ellas
eran reversos de regresiones respecto a una semana anterior. [#162422](https://github.com/rust-lang/rust/pull/162422) mejoró el rendimiento de Polonio, cuyo
el rendimiento se está acercando al anterior comprobador de préstamos NLL. 

Triaje hecho por **@Kobzol**. 
Rango de revisión: [656a9da1.. 20d35a3a](https://perf.rust-lang.org/?start=656a9da186dacaf3bf8f7f7296a825d256cb4ae3&end=20d35a3ae8f310f2a002e5f6e0bc583830010cd4&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | -     | -              | 0 |
| Regresiones ❌ <br /> (secundario) | 0,4% | [0,1%, 0,9%] | 3 |
| Mejoras ✅ <br /> (primaria) | -0,7% | [-4,4%, -0,1%] | 199 |
| Mejoras ✅ <br /> (secundario) | -0,9% | [-2,7%, -0,1%] | 222 |
| Todos ❌✅ (primario) | -0,7% | [-4,4%, -0,1%] | 199 |

0 regresiones, 5 mejoras, 5 mixtas; 2 de ellas en rollups
40 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/85651591cba06f7122ef35d459ab76b7f583bc48/triage/2026/2026-09-14.md). 

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Estabilizar 'debug_closure_helpers'](https://github.com/rust-lang/rust/pull/146099)
* [Conversiones adicionales NonZero](https://github.com/rust-lang/rust/pull/129036)
* [Implementar el predeterminado de NumBuffer](https://github.com/rust-lang/rust/pull/162536)
* [Permitir vidas elididas ('estáticas) en 'thread_local!'](https://github.com/rust-lang/rust/pull/159564)
* [Estabilizar 'funnel_shifts' (incluyendo 'const')](https://github.com/rust-lang/rust/pull/161015)
* [Estabilizar 'mem::conjure_zst'](https://github.com/rust-lang/rust/pull/161710)
* [Estabilizar 'Resultado::into_{ok,err}'](https://github.com/rust-lang/rust/pull/161712)
* [ventanas: estabilizar inherit_handles](https://github.com/rust-lang/rust/pull/161163)
* [rustc: Estabilizar la función de 'aritmética amplia' de WebAssembly](https://github.com/rust-lang/rust/pull/160877)
* [Evitar mutar el puntero global de entorno en 'CommandExt::exec' y optar por usar execve y resolver ruta manualmente](https://github.com/rust-lang/rust/pull/157144)
* [alloc: estabilizar 'Allocator'](https://github.com/rust-lang/rust/pull/156882)
* [No permitir accesos a través de una proyección Index cuando una proyección hermana de ConstantIndex ha sido movida de](https://github.com/rust-lang/rust/pull/160780)
* [Permitir que los tipos de operandos unarios se infieran más adelante](https://github.com/rust-lang/rust/pull/159744)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(config): Añadir build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* [OUT_DIR también se activa al ejecutar el programa](https://github.com/rust-lang/cargo/issues/17456)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Permitir la participación de observadores durante las reuniones](https://github.com/rust-lang/leadership-council/pull/110)
* [Sugiero discutir candidatos a LC con moderadores](https://github.com/rust-lang/leadership-council/pull/331)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo Compilador](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Corregir enlace Zig roto en el RFC 3308 y errores tipográficos en los textos del RFC](https://github.com/rust-lang/rfcs/pull/4006)

## Próximos eventos

Eventos Rusty entre el 16-09-2026 y el 14-10-2026 🦀

### Virtual
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Construcción de un controlador de GPU Rust en el kernel de Linux**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ibaxicxv)
* 2026-09-20 | Virtual (Bengaluru, IN) | [Discord de Rust incrustado](https://discord.com/invite/pvYY69PvyS)
    * [**Silicon Sundays 3**](https://discord.gg/t9Cb2gjjq7?event=1546754977374932993)
* 2026-09-2026 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254773/)
* 2026-09-24 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn de Oxid**](https://www.meetup.com/rust-berlin/events/315907979/)
* 2026-09-24 | Virtual (Charlottesville, VA, EE.UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Celdas de Rust — Mutabilidad Interior de Núcleo de Rust a Sistemas Operativo Tock**](https://www.meetup.com/charlottesville-rust-meetup/events/316460694/)
* 2026-09-29 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Cajas, consejos y trucos Charlas relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)
* 2026-09-30 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Segmentación e Introducción al Paginado**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/yqxvguts)
* 04-10-2026 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316134009/)
* 06-10-2026 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773044/)
* 07-10-2026 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-13 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254772/)

### Europa
* 2026-09-14 - 2026-09-16 | Berlín, DE | [Oxidar 2026](https://oxidizeconf.com/)
    * [**Oxidar 2026**](https://oxidizeconf.com/)
* 2026-09-17 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - De Segfault a Seguridad @DiWoDo**](https://www.meetup.com/rust-dortmund/events/316428507/)
* 22-09-2026 | Praga, CZ | [Praga Oxidada](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Charla en SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Ámsterdam, NL | [Grupo Rust Developers Amsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Codificación Agente IA**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-26 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #30**](https://www.meetup.com/stockholm-rust/events/316570423/)
* 2026-09-28 | Augsburgo, DE | [Encuentro Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #21: Maximilian Grauvogl & Marcel Fink - De bits a bugs: Un generador de Rust para manifiestos de SUIT y fuzzing de analizadores conscientes de la estructura**](https://rust-augsburg.github.io/meetup/Meetup_21.html)
* 2026-09-29 | Manchester, Reino Unido | [Manchester Oxidado](https://www.meetup.com/rust-manchester)
    * [**Noche del Código de Septiembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/316200964/)
* 2026-09-30 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 05-10-2026 | Múnich, DE | [Múnich Oxidado](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 08-10-2026 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Hack'n'Learn Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-10 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22ª sesión de bcnrust**](https://www.meetup.com/bcnrust/events/316316234/)

### Norteamérica
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Construcción de un controlador de GPU Rust en el kernel de Linux**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-16 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Encuentro de gráficos**](https://luma.com/9oiujuyw)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CALI, EE.UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-19 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Común de Boston, 19 de septiembre**](https://www.meetup.com/bostonrust/events/316378813/)
* 2026-09-22 | Chicago, IL, EE. UU. | [Encuentro Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz Oxid**](https://www.meetup.com/chicago-rust-meetup/events/316572988/)
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316404827/)
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-24 | Atlanta, GA, EE.UU. | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Harvard Rust, 26 de septiembre**](https://www.meetup.com/bostonrust/events/316378817/)
* 2026-10-01 | Saint Louis, MO, EE. UU. | [STL Oxidación](https://www.meetup.com/stl-rust)
    * [**construyendo un contenedor mínimo y sin raíces en Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 03-10-2026 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 3 de octubre**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 10 de octubre**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Ángeles, CA, EE.UU. [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust con Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)

### Oceanía
* 2026-09-29 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de septiembre**](https://www.meetup.com/rust-canberra/events/316398052/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1vtuq1b/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> De vez en cuando me cuentan algún dato matemático que solo puedo suponer que alguien fue a prisión por descubrir

– [Simon Buchan](https://users.rust-lang.org/t/as-str-for-integers/142364/17)

¡Gracias a [Chayim Refael Friedman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1799) por la sugerencia! 

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1wikjqo/this_week_in_rust_669/)</small>