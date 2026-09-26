---
title: "Esta semana en Rust #127"
number_of_week: 127
description: El crate de esta semana es fastlogging-rs, un logger rápido que soporta 8 lenguajes de programación diferentes. 
date: 2026-09-23
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

* [Estad alerta: ataques dirigidos a prominentes Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)
* [Acciones de GitHub filtrando secretos cuando la salida de Miri se almacena en caché](https://blog.rust-lang.org/2026/09/21/github-actions-leaking-secrets-when-miri-output-is-cached/)
* [Destacada mantenedora: Alejandra González (@blyxyas)](https://blog.rust-lang.org/inside-rust/2026/09/21/maintainer-spotlight-alejandra-gonzalez-blyxyas/)
* [Anunciando un mantenedor residente: Scott Schafer para el equipo de carga](https://blog.rust-lang.org/2026/09/22/announcing-a-maintainer-in-residence-scott-schafer-for-the-cargo-team/)

### Fundación

* [Artículo invitado: Rust es lenguaje de nivel 1 en Microsoft](https://rustfoundation.org/media/guest-post-rust-is-tier-1-language-at-microsoft/)

### Actualizaciones de proyectos/herramientas

* [Fearless SIMD v1.0 está aquí](https://linebender.org/blog/fearless-simd-1-0/)
* [Sincronización del backend de Rust GCC o cómo probar la ley de Murphy](https://blog.guillaume-gomez.fr/articles/2026-09-22+Syncing+Rust+GCC+backend+or+how+to+test+Murphy%27s+law)
* [Comparando Salvaje vs Moho ](https://davidlattimore.github.io/posts/2026/09/18/benchmarking-wild-vs-mold.html)

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [Discutiendo sobre discusiones](https://steveklabnik.com/writing/arguing-about-arguments/)
* [Nueve reglas para la validación de vibraciones de algoritmos codificados por vibración (Rust)](https://levelup.gitconnected.com/nine-rules-for-vibe-validation-of-vibe-coded-algorithms-20db019f5583)

### Guías de Rust

* [vídeo] [Entendiendo la propiedad de Rust construyendo un analizador de líneas de registro de copia cero](https://www.youtube.com/watch?v=ZxxUqoUTgnA)
* [Buscando bichos](https://matklad.github.io/2026/09/19/finding-bugs.html)
* [Por qué datadiff empareja los arrays por clave en lugar de calcular la distancia de edición del árbol](https://dev.to/dimanovikov/why-datadiff-matches-arrays-by-key-instead-of-computing-tree-edit-distance-2ehp)
* [vídeo] [lección RustCurious 10: Tres formas de arreglar cualquier error de préstamo](https://www.youtube.com/watch?v=dLx8usb759E)
* [Resolviendo para una detección de colisiones más rápida en SHA-1](https://sam.dev/blog/faster-sha1-collision-detection)
* [Imágenes Docker pequeñas y seguras para Rust: Alpine vs Debian vs Scratch](https://kerkour.com/rust-docker)

## Crate de la semana

El crate de esta semana es [fastlogging-rs](https://github.com/brmmm3/fastlogging-rs), un logger rápido que soporta 8 lenguajes de programación diferentes. 

¡Gracias a [brmmm3](https://users.rust-lang.org/t/crate-of-the-week/2704/1673) por la autosugerencia! 

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

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si eres un implementador de funciones y quieres que tu RFC aparezca en la lista anterior, añade la nueva 'llamada para pruebas'
etiqueta a tu RFC junto con un comentario que ofrezca instrucciones de prueba y/o orientación sobre qué aspecto(s) de la funcionalidad
Necesito pruebas. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [sysknife - Un rollback automático exitoso se renderiza al operador como desconocido](https://github.com/lacs-project/sysknife/issues/482)
* [sysknife - sysknife-setup--desinstalar elimina .mcp.json total, llevándose consigo todos los demás servidores MCP](https://github.com/lacs-project/sysknife/issues/480)
* [sysknife - audit export publica request_hash, un hash sin sal sobre parámetros no redactados, sin declaración de su sensibilidad](https://github.com/lacs-project/sysknife/issues/268)
* [Apache Iggy - SDK Python: exponer el apagado del consumidor y compensar el tiempo de consumo](https://github.com/apache/iggy/issues/4165)
* [Apache Iggy - SDK en Python: métodos de desconexión y apagado del cliente para el ciclo de vida](https://github.com/apache/iggy/issues/4163)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

618 solicitudes de tirada fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-15..2026-09-22

#### Compilador
* [Un par de restricciones de Polonio mejoras en la interpretación](https://github.com/rust-lang/rust/pull/163027)
* [AST bajando limiones](https://github.com/rust-lang/rust/pull/162747)
* [una variedad de ajustes de Polonio](https://github.com/rust-lang/rust/pull/162922)
* [comprobar 'tainted_by_error' en LateLint](https://github.com/rust-lang/rust/pull/147876)
* [aún más limpieza para 'rustc_builtin_macros'](https://github.com/rust-lang/rust/pull/162925)
* [perf: mantén la primera macro de mapeado sintaxis-contexto en línea](https://github.com/rust-lang/rust/pull/162712)
* [sugerir colisión de camino totalmente calificado sobre nombre de método](https://github.com/rust-lang/rust/pull/153662)

#### Biblioteca
* [añadir 'Dir::try_clone'](https://github.com/rust-lang/rust/pull/163007)
* [añadir mapeos rápidos de casos para Latin-1](https://github.com/rust-lang/rust/pull/162750)
* [conjugado complejo, negación y predeterminado](https://github.com/rust-lang/rust/pull/162865)
* [constificar rasgos de comparación en tipos rebanados](https://github.com/rust-lang/rust/pull/147790)
* [implementar un iterador de constrancia para rango](https://github.com/rust-lang/rust/pull/156216)
* [estabilizar 'CommandExt::show_window'](https://github.com/rust-lang/rust/pull/162856)
* [estabilizar 'Feature(trim_prefix_suffix)' ({'str','[T]', 'Path'}'::trim_prefix' y {'str', '[T]'}'::trim_suffix')](https://github.com/rust-lang/rust/pull/160544)
* [estabilizar 'windows_process_extensions_main_thread_handle'](https://github.com/rust-lang/rust/pull/160108)

#### Carga
* ['build-rs': hacer compilación 'inestable'](https://github.com/rust-lang/cargo/pull/17489)
* [tener en cuenta los enlaces duros (uplift) al calcular el tamaño limpio del archivo](https://github.com/rust-lang/cargo/pull/17485)
* [corrección: devolver las especificaciones correctas del paquete al resolver deps de espacio de trabajo](https://github.com/rust-lang/cargo/pull/17469)
* [eliminar -Zasymmetric-token / cargo:paseto](https://github.com/rust-lang/cargo/pull/17486)
* [reporta el número de errores con 'build.warnings=''deny'](https://github.com/rust-lang/cargo/pull/17479)

#### Rustdoc
* [Manejar correctamente los métodos de rasgos 'dyn' que enlazan para la característica de salto a defensa](https://github.com/rust-lang/rust/pull/163036)
* [Gestionar correctamente los enlaces intra-doc en el mismo elemento en línea con diferentes nombres](https://github.com/rust-lang/rust/pull/162669)

#### Clippy
* [añadir pelusa de 'must_use_without_reason'](https://github.com/rust-lang/rust-clippy/pull/16592)
* [corregir el bucle infinito relacionado con 'const_trait_impl' en 'needless_borrows_for_generic_args'](https://github.com/rust-lang/rust-clippy/pull/17731)
* [generaliza 'extend_with_drain' a 'VecDeque' y 'BinaryHeap'](https://github.com/rust-lang/rust-clippy/pull/16778)
* [pelusa 'suboptimal_flops' para 'mul_add', 'custom_abs' y 'radians' en contexto const](https://github.com/rust-lang/rust-clippy/pull/17631)
* [pelusa anidada 'format_args!' para argos sin línea](https://github.com/rust-lang/rust-clippy/pull/16885)

#### Analizador de Rust
* [priorizar los objetos requeridos en el autocompletado de rasgos](https://github.com/rust-lang/rust-analyzer/pull/23407)
* [Completaciones de apoyo dentro de 'CFG! ()'](https://github.com/rust-lang/rust-analyzer/pull/23384)
* [soporte flotando sobre el predicado CFG](https://github.com/rust-lang/rust-analyzer/pull/23390)
* [valor completo de cfg en cadena](https://github.com/rust-lang/rust-analyzer/pull/23395)
* [orden correcto descontinuado const en incorporado ty](https://github.com/rust-lang/rust-analyzer/pull/23405)
* [no completo de atr args cuando antes existe args](https://github.com/rust-lang/rust-analyzer/pull/23399)
* [Watch include roots recursivamente una vez, no todos los directorios](https://github.com/rust-lang/rust-analyzer/pull/23375)

### Triaje de rendimiento del compilador Rust

<!-- resultados de rendimiento aquí -->

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Tipos FCP v2: Estabilización de sombra de objetos de superrasgo](https://github.com/rust-lang/rust/issues/162130)
* [declarar C y C-unwind como mutuamente compatibles con ABI](https://github.com/rust-lang/rust/pull/161904)
* [Distinguir los ZSTs 'repr(C)' de otros en las reglas de compatibilidad ABI](https://github.com/rust-lang/rust/pull/157973)
* [Implementar el predeterminado de NumBuffer](https://github.com/rust-lang/rust/pull/162536)
* [rustc: Estabilizar la función de 'aritmética amplia' de WebAssembly](https://github.com/rust-lang/rust/pull/160877)
* [Permitir vidas elididas ('estáticas) en 'thread_local!'](https://github.com/rust-lang/rust/pull/159564)
* [Estabilizar 'mem::conjure_zst'](https://github.com/rust-lang/rust/pull/161710)
* [Evitar mutar el puntero global de entorno en 'CommandExt::exec' y optar por usar execve y resolver ruta manualmente](https://github.com/rust-lang/rust/pull/157144)
* [Estabilizar 'debug_closure_helpers'](https://github.com/rust-lang/rust/pull/146099)
* [Conversiones adicionales NonZero](https://github.com/rust-lang/rust/pull/129036)
* [Estabilizar 'funnel_shifts' (incluyendo 'const')](https://github.com/rust-lang/rust/pull/161015)
* [Estabilizar 'Resultado::into_{ok,err}'](https://github.com/rust-lang/rust/pull/161712)
* [ventanas: estabilizar inherit_handles](https://github.com/rust-lang/rust/pull/161163)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [OUT_DIR también se activa al ejecutar el programa](https://github.com/rust-lang/cargo/issues/17456)
* [feat(config): Añadir build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* [fix(git)!: Por defecto net.git-fetch-with-cli si hay git presente](https://github.com/rust-lang/cargo/pull/17329)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Crear un nuevo objetivo de nivel 3: 'wasm32-webp2'](https://github.com/rust-lang/compiler-team/issues/1037)
* [Prueba wasm en CI con hilos y desenrollamiento](https://github.com/rust-lang/compiler-team/issues/1039)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Formular un equipo de marcadores de 'contribuyentes de confianza'](https://github.com/rust-lang/leadership-council/issues/325)
* [Participación en Outreachy diciembre 2026 (dedicación de fondos)](https://github.com/rust-lang/leadership-council/issues/320)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.* 

## Próximos eventos

Eventos Rusty entre el 23-09-2026 - el 21-10-2026 🦀

### Virtual
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
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-18 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, EE.UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcnbbc/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)

### Asia
* 2026-09-23 | Maharashtra, IN | [Rust Pune](https://hasgeek.com/rustpune)
    * [**Explorando rasgos estándar en Rust**](https://hasgeek.com/rustpune/exploring-standard-traits-in-rust/)

### Europa
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Charla en SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Ámsterdam, NL | [Grupo Rust Developers Amsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Codificación Agente IA**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-24 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**Rust London, Lloyds Banking Group, con Luca Palmieri & Mainmatter**](https://www.meetup.com/rust-london-user-group/events/316560409/)
* 2026-09-25 | Edimburgo, Reino Unido | [Rust y Amigos](https://www.meetup.com/rust-edi)
    * [**Rust y Amigos (café diurno)**](https://www.meetup.com/rust-and-friends/events/316610395/)
* 2026-09-26 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #30**](https://www.meetup.com/stockholm-rust/events/316570423/)
* 2026-09-28 | Augsburgo, DE | [Encuentro Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #21: Maximilian Grauvogl & Marcel Fink - De bits a bugs: Un generador de Rust para manifiestos de SUIT y fuzzing de analizadores conscientes de la estructura**](https://rust-augsburg.github.io/meetup/Meetup_21.html)
* 2026-09-29 | Manchester, Reino Unido | [Manchester Oxidado](https://www.meetup.com/rust-manchester)
    * [**Noche del Código de Septiembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/316200964/)
* 2026-09-29 | Milán, IT | [Milán en idioma oxidado](https://www.meetup.com/rust-language-milano)
    * [**¿Por qué Rust no es realmente OOP?**](https://www.meetup.com/rust-language-milan/events/316654592/)
* 2026-09-30 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-09-30 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/316661690/)
* 05-10-2026 | Múnich, DE | [Múnich Oxidado](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 08-10-2026 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Hack'n'Learn Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-10 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22ª sesión de bcnrust**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 20-10-2026 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)

### Norteamérica
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316404827/)
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
* 20-10-2026 | San Francisco, CA, EE.UU. | [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)

### Oceanía
* 2026-09-29 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de septiembre**](https://www.meetup.com/rust-canberra/events/316398052/)

### Sudamérica
* 2026-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**WebApp Ergonomics y Secretos Distribuidos.**](https://www.meetup.com/rust-argentina/events/316664266/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1wo5btb/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> pedantismo operativo

– [Clar Fon sobre el zulipán de Rust](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/Looping.20opsem.20into.20libs.20changes/near/626044816)

¡Gracias a [Jules Bertholet](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1803) por la sugerencia! 

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1woppw3/this_week_in_rust_670)</small>