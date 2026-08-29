---
title: "Esta semana en Rust #124"
number_of_week: 124
description: El crate de esta semana es swift-topomap, una herramienta de observabilidad microarquitectónica. 
date: 2026-08-26
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

### Oficial
* [Anunciando a nuestros primeros Mantenedores en Residencia](https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/)
* [Activando el solucionador de rasgos de nueva generación en cada noche](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/)
* [Ataque a la cadena de suministro en la matriz de fuego](https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/)
* [Función de Rust sobrecargando - Llamada a la experimentación](https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/)

### Actualizaciones de proyectos/herramientas
* [Intención de lanzar: JPEG XL – Mozilla Hacks - el blog de desarrolladores web](https://hacks.mozilla.org/2026/08/intent-to-ship-jpeg-xl/)

### Observaciones/Pensamientos
* [Escalando la seguridad de la memoria: reescrituras asistidas por IA de dependencias C/C++ a oxidarse](https://bughunters.google.com/blog/scaling-memory-safety)
* [Reemplazar un enum oxidado por una palabra de 64 bits hizo que mi intérprete fuera un 17% más rápido](https://pointersgonewild.com/2026-08-25-replacing-a-rust-enum-with-a-64-bit-word/)
* [3 segundos de compilación recortados por análisis de metadatos](https://blog.goose.love/posts/three-seconds-of-compilation-shaved-by-metadata-analysis/)
* [Tu panel de papel electrónico no está roto: Cómo el estado retenido hace que los conductores parezcan defectuosos](https://msj.prose.sh/epaper-retained-state)
* [Asíncrono o no asincrónico: Construcción de un servidor Rust MCP para rust-analyzer](https://developerlife.com/2026/08/22/to-async-or-not-to-async-rust-mcp-server/)
* [Un intento, tres trabajos, cero puntos de referencia ganados](https://akesson.io/wordtree/)
* [Arreglando la seguridad de la cadena de suministro de Rust: lo bueno, lo malo y lo feo](https://kerkour.com/fixing-rust-supply-chain-security)

### Guías de Rust
* [Errores de Rust que cometen todos los principiantes](https://dev.to/yetmike/the-22-rust-errors-every-beginner-hits-in-the-order-they-hit-them-13gi)
* [Construye una calculadora científica en Rust - Comprendiendo variables y tipos](https://blog.sheerluck.dev/posts/understanding-rust-variables-and-types-by-building-a-scientific-calculator/)
* [Demostrando la caché de instrucciones de SQLx con bpftrace](https://flakm.com/posts/sqlx_caches_til/)
* [Más allá de WASI: Rust aplicaciones en el navegador](https://labs.leaningtech.com/blog/browserpod-rust)

### Miscelánea
* [JetBrains colabora con la Rust Foundation para una transmisión en directo de IA](https://rustfoundation.org/media/jetbrains-partners-with-the-rust-foundation-for-an-ai-livestream-series/)

## Crate de la semana

El crate de esta semana es [swift-topomap](https://github.com/swiftlogicsystems/swifttopology), una herramienta de observabilidad microarquitectónica. 

¡Gracias a [Ankur Rathore](https://users.rust-lang.org/t/crate-of-the-week/2704/1658) por la autosugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

##### [Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para '--remap-path-scope' en rustdoc](https://github.com/rust-lang/rust/issues/155451)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para '-zembed-metadata'](https://github.com/rust-lang/cargo/issues/15495)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [sysknife - Separar las acciones exclusivas de Ubuntu de DEBIAN_ONLY_ACTIONS](https://github.com/lacs-project/sysknife/issues/237)
* [sysknife - Haz que Debian sea elegible: un piso de versión de 12, y una razón en is_supported](https://github.com/lacs-project/sysknife/issues/238)
* [sysknife - El cortafuegos predeterminado de Debian es nftables, y el catálogo no tiene vocabulario de nftables](https://github.com/lacs-project/sysknife/issues/239)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [stomatopod - Añadir una revisión de salud Docker Compose en /health](https://github.com/kkir/stomatopod/issues/40)
* [stomatopod - Añadir una imagen de vista previa social personalizada en GitHub](https://github.com/kkir/stomatopod/issues/41)
* [stomatopod - Documentar la etiqueta GHCR v0.1.0 junto a :latest](https://github.com/kkir/stomatopod/issues/42)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

593 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-18..2026-08-25

#### Compilador
* [añadir una caché al visitante 'WfPredicates'](https://github.com/rust-lang/rust/pull/161274)
* [permitir a sí mismo en genéricos de const](https://github.com/rust-lang/rust/pull/157949)
* [eliminar algún buggy 'inalcanzable! ()'s en 'expand_'('option_')'env()'](https://github.com/rust-lang/rust/pull/159940)
* [activar '-Znext-solver' por defecto cada noche](https://github.com/rust-lang/rust/pull/160619)
* [optimizar 'DeepRejectCtxt'](https://github.com/rust-lang/rust/pull/161211)

#### Biblioteca
* [añadir 'Arc/Rc::strong_count_from_raw'](https://github.com/rust-lang/rust/pull/159098)
* [añadir implementación 'Default' para 'std::sync::Once'](https://github.com/rust-lang/rust/pull/160136)
* [añadir impls simétricos de Ecuación Parcial para 'Vec', '&[T]', '&mut [T]' frente a 'Vaca<'_, [T]>'](https://github.com/rust-lang/rust/pull/156160)
* [núcleo: implementar métodos de conversión de flotadores](https://github.com/rust-lang/rust/pull/159954)
* [hacer covariantes 'BorrowedCursor<'a, T>' en ''a' y eliminar una indirecta](https://github.com/rust-lang/rust/pull/160563)
* [rehacer 'div_ceil' para enteros distintos de cero](https://github.com/rust-lang/rust/pull/160819)
* [estabilizar 'bool::toggle'](https://github.com/rust-lang/rust/pull/160299)
* [estabilizar nunca escribe](https://github.com/rust-lang/rust/pull/155499)

#### Carga
* ['config': Añadir build.fingerprint](https://github.com/rust-lang/cargo/pull/17382)
* [corregir 'git gc' con 'safe.bareRepository=explicit'](https://github.com/rust-lang/cargo/pull/17370)
* [instalar herramientas de carga con dependencias bloqueadas](https://github.com/rust-lang/cargo/pull/17377)

#### Rustdoc
* [añadir nueva pelusa de 'invalid_markdown_table' de Rustdoc](https://github.com/rust-lang/rust/pull/159583)
* [solo generar elementos de DOM de búsqueda si la búsqueda es realmente necesaria](https://github.com/rust-lang/rust/pull/160639)
* [habilitar el desplazamiento solo en tabla/código](https://github.com/rust-lang/rust/pull/161340)
* [problema de corrección que impide que se generen enlaces de "leer más"](https://github.com/rust-lang/rust/pull/161553)

#### Rustfmt
* [arreglar ICE en bucles 'for await' con tokens de palabras clave separados](https://github.com/rust-lang/rustfmt/pull/7063)
* [fijar la colocación de refuerzos para flujo de control multilínea](https://github.com/rust-lang/rustfmt/pull/7005)
* [comentarios corregidos demasiado largos](https://github.com/rust-lang/rustfmt/pull/6802)
* [corregir el span usado al reescribir 'ast::TyKind::FnPtr'](https://github.com/rust-lang/rustfmt/pull/7066)
* [Orden correcto de visibilidad y predeterminación en el alias impl asociado](https://github.com/rust-lang/rustfmt/pull/7064)
* [formato inconsistente de los comentarios del documento en macros](https://github.com/rust-lang/rustfmt/pull/7042)

#### Clippy
* [optimizar Clippy con PGO](https://github.com/rust-lang/rust/pull/159642)
* ['unnecessary_fold': pelusa que se plega sobre el iterador de una Opción](https://github.com/rust-lang/rust-clippy/pull/17445)
* ['unused_trait_names': haz la sugerencia más amable](https://github.com/rust-lang/rust-clippy/pull/17589)
* [evitar 'manual_assert_eq' para tipos tipo corte de byte](https://github.com/rust-lang/rust-clippy/pull/17575)
* [no dispares 'manual_contains' cuando ambos bandos usan el elemento de corte](https://github.com/rust-lang/rust-clippy/pull/17564)
* [arreglar 'large_futures' ICE con el siguiente solucionador](https://github.com/rust-lang/rust-clippy/pull/17601)
* [evitar 'double_must_use' en código generado por macros](https://github.com/rust-lang/rust-clippy/pull/17547)
* [hacer que 'needless_bool' sea menos agresivo para los 'si' encadenados(https://github.com/rust-lang/rust-clippy/pull/17598)
* [PERF: comprueba 'first_node_in_macro' antes de la caminata de macro raíz en 'useless_format'](https://github.com/rust-lang/rust-clippy/pull/17584)
* [eliminar sugerencia rota para 'blocks_in_conditions'](https://github.com/rust-lang/rust-clippy/pull/17127)
* [sugiere 'hypot' para 'x.mul_add(x, y * y).sqrt()'](https://github.com/rust-lang/rust-clippy/pull/17600)
* [sugiere 'is_ok/is_err' para mapeos de resultados booleanos](https://github.com/rust-lang/rust-clippy/pull/17537)
* [activación 'integer_division_remainder_used' en 'DivAssign'/'RemAssign'](https://github.com/rust-lang/rust-clippy/pull/16493)

#### Analizador de Rust
* ['hir': Usar el almacén de expresión del cuerpo principal si está disponible](https://github.com/rust-lang/rust-analyzer/pull/23202)
* [suma la flecha para desmapear los rangos cuando fn está dentro de la macro](https://github.com/rust-lang/rust-analyzer/pull/23216)
* [permitir que los bloques de etiqueta 'asm!' diverjan](https://github.com/rust-lang/rust-analyzer/pull/23186)
* [previene el desbordamiento de pila para layouts ADT recursivos](https://github.com/rust-lang/rust-analyzer/pull/23201)
* [optimiza al máximo el almacenamiento de árboles de tokens](https://github.com/rust-lang/rust-analyzer/pull/23079)
* [usar el directorio de construcción de carga para los registros de flycheck](https://github.com/rust-lang/rust-analyzer/pull/23214)

### Triaje de rendimiento del compilador Rust

Una semana ajetreada, con un flujo continuo de mejoras para el siguiente solucionador de rasgos
Y a continuación, implementaciones de cheques de préstamo. Aparte de esos cambios, la semana fue
Bastante silencioso para el rendimiento. 

Triaje hecho por **@simulacrum**. 
Rango de revisión: [8fa1c96c.. 9a4AD59a](https://perf.rust-lang.org/?start=8fa1c96cfd489e4c27654c144ae871ce2c4db6c6&end=9a4ad59ae3073b013cd62f53f8349ddc61a012e8&absolute=false&stat=instructions%3Au)

2 regresiones, 4 mejoras, 2 mixtas; 2 de ellas en rollups. 28 comparaciones de artefactos en total. 

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-08-23.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [hacer que la función objetivo sea ABI comprobar un error grave en ARM](https://github.com/rust-lang/rust/pull/161280)
* [estabilizar funciones inteligentes de mapeo de puntero](https://github.com/rust-lang/rust/pull/160534)
* [volátil: permite que los accesos a la memoria no AM atrapen](https://github.com/rust-lang/rust/pull/160564)
* [Estabilizar la función 'supertrait_item_shadowing'](https://github.com/rust-lang/rust/pull/148605)
* [Añadir intrínsecas para el mínimo y máximo entero](https://github.com/rust-lang/rust/pull/161081)
* [Siempre escape de los extensores de grafema en 'str::escape_debug'](https://github.com/rust-lang/rust/pull/158303)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Cambiar 'i686-pc-windows-msvc' de Tier 1 con herramientas de host => Tier 1 sin herramientas de host](https://github.com/rust-lang/rfcs/pull/3999)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [dote(resolver): Estabilizar min-publicación-edad](https://github.com/rust-lang/cargo/pull/17335)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Añadir 'codeview_annotation' intrínseco](https://github.com/rust-lang/compiler-team/issues/1026)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Actualizar el proceso electoral de PD basado en los comentarios de 2025](https://github.com/rust-lang/leadership-council/pull/286)
* [Crear un equipo de políticas LLM](https://github.com/rust-lang/leadership-council/issues/308)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Añadir 'core::ffi::c_longdouble'](https://github.com/rust-lang/rfcs/pull/4003)
* [RFC: añadir operación de 'congelación'](https://github.com/rust-lang/rfcs/pull/4001)

## Próximos eventos

Eventos Rusty entre 26-08-2026 - 23-09-2026 🦀

### Virtual
* 2026-08-26 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Lotería y Programación Multi-CPU**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316083375/)
* 2026-08-27 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxid**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/arkkrcj5)
* 31-08-2026 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Workshop: Añadir pruebas a un proyecto Rust de código abierto**](https://luma.com/nwfmsdtf)
* 01-09-2026 | Virtual | [Experto del Rust 🦀](https://luma.com/rust-maven)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://luma.com/d9w26vav)
* 2026-09-02 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-02 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/316107210/)
* 2026-09-04 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/sqf4ux01)
* 2026-09-06 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Hechos: Conocimiento Curado para Humanos y Agentes**](https://luma.com/9lte7a58)
* 2026-09-06 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316133872/)
* 2026-09-08 - 2026-09-11 | Híbrido (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-08 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315859305/)
* 2026-09-10 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ibaxicxv)
* 2026-09-2026 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254773/)

### África
* 2026-09-08 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: agosto 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europa
* 2026-08-26 | Copenhague, DK | [Comunidad Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #71 Patrocinado por Factbird**](https://www.meetup.com/copenhagen-rust-community/events/316180984/)
* 2026-08-26 | Dresde, DE | [Dresde Oxidado](https://github.com/rust-dresden)
    * [**Tercer encuentro**](https://pretix.eu/rust-dresden/on-location-3)
* 2026-08-27 | Londres, Reino Unido | [Grupo de Usuarios Rust London](https://www.meetup.com/rust-london-user-group)
    * [**LDN habla de la Exposición Comunitaria de agosto**](https://www.meetup.com/rust-london-user-group/events/316197176/)
* 2026-08-27 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)
* 2026-08-29 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #29**](https://www.meetup.com/stockholm-rust/events/316130996/)
* 2026-09-08 | París, FR | [París Oxidado](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #87**](https://www.meetup.com/rust-paris/events/316169040/)
* 2026-09-14 - 2026-09-16 | Berlín, DE | [Oxidar 2026](https://oxidizeconf.com/)
    * [**Oxidar 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproduciendo artículos científicos - con Rust & "IA"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)
* 22-09-2026 | Praga, CZ | [Praga Oxidada](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)

### Norteamérica
* 26-08-2026 | Austin, TX, EE.UU. [ATX Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/315171660/)
* 26-08-2026 | Los Ángeles, CA, EE.UU. [Los Ángeles Oxidado](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA ¡Agosto! Rust en la computación cuántica**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, EE.UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Mountain View, CA, EE. UU. [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316124372/)
* 2026-09-03 | Saint Louis, MO, EE. UU. | [Rust STL](https://www.meetup.com/stl-rust)
    * [**Criptografía + Ordenadores Cuánticos**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 - 2026-09-11 | Híbrido (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 09-09-2026 | Montreal, CA | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Encuentro de pausa café de RustConf**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Rust Integrado Práctico**](https://www.meetup.com/utah-rust/events/316198046/)
* 10-09-2026 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 15-09-2026 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CALI, EE.UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)

### Oceanía
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne agosto 2026**](https://www.meetup.com/rust-melbourne/events/315039490/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1vtuq1b/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> me importa esta comunidad, incluida su naturaleza humana y social. Quiero que otros aprecien esas cualidades, y no quiero verlas comprometidas y reemplazadas por contenido generado por máquinas. 

– [Quine Dot sobre usuarios de Rust](https://users.rust-lang.org/t/use-of-ai-assitance-to-solve-issues-and-validate-to-reply/142029/11)

¡Gracias a [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1791) por la sugerencia! 

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1vzh4xx/this_week_in_rust_666/)</small>