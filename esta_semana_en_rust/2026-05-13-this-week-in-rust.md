---
title: "Esta semana en Rust #109"
number_of_week: 109
description: El crate de esta semana es cloakrs, una biblioteca y herramienta CLI para detectar y enmascarar información personal identificable.
date: 2026-05-13
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

### Fundación
* [Rust Foundation y líderes del registro de paquetes se unen para abordar la crisis de sostenibilidad del código abierto](https://rustfoundation.org/media/rust-foundation-and-package-registry-leaders-unite-to-address-open-source-sustainability-crisis/)

### Boletines
* [El Rustaceano Incrustado Número #71](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-71)

### Actualizaciones de proyectos/herramientas
* [Numax - Un entorno portátil de Rust para aplicaciones distribuidas](https://github.com/GianIac/numax/releases/tag/v0.1.0-alpha.1)
* [Entroly 0.18.0: Motor de contexto de IA impulsado por Rust con aprendizaje por refuerzo PRISM, deduplicación SimHash y caché EGSC](https://github.com/juyterman1000/entroly/discussions/43)
* [uFerris: Una tabla de aprendizaje versátil para Rust incrustado](https://www.theembeddedrustacean.com/uferris)
* [Propiedad del disco: ¿Cuál lado es el correcto?](https://aimdb.dev/blog/record-ownership)
* [iroh 1.0.0-rc.0 - El primer candidato a lanzamiento](https://www.iroh.computer/blog/iroh-1-0-0-rc-0)
* [Burn 0.21.0 Versión: hasta 8× menor sobrecarga del framework, colectivos diferenciables y kernels mejorados](https://burn.dev/blog/release-0.21.0/)
* [Ratty: Un emulador de terminal con gráficos 3D en línea](https://blog.orhun.dev/introducing-ratty/)
* [Anunciando el tiempo de ejecución de Rust para las funciones Appwrite](https://appwrite.io/blog/post/announcing-rust-runtime)
* [Anunciando diésel asincrónico 0.9](https://blog.weiznich.de/blog/diesel-async-0-9/)
* [Fresh 0.3.4: El tema 'terminal' nativo de Ansi coincide con el tema del sistema; UI para Live Grep + proveedores personalizados de grep; división persistente de 'muelle'; soporte para Verilog/VHDL; y mucho más](https://github.com/sinelaw/fresh/releases/tag/v0.3.4)

### Observaciones/Pensamientos
* [Matar una 'vaca' hizo que mi formatador JSON fuera un 42% más rápido](https://jacobasper.com/blog/killing-a-cow-made-my-json-formatter-42-percent-faster/)
* [Empezando con el Rust geoespacial](https://eors-workspace-a6ef35.gitlab.io/posts/001-introduction-geospatial-rust/) — Qué satélites miden: bandas espectrales, índices, detección de nubes.
* [Lecciones aprendidas construyendo un perfilador de Rust de alto rendimiento (https://pawelurbanek.com/rust-performance-profiling)
* [Los límites de Rust, o por qué probablemente no deberías seguir Amazon, Cloudflare y Discord](https://kerkour.com/the-limits-of-rust)
* [El coste oculto de los canales mpsc](https://blog.howardjohn.info/posts/mpsc-cost/)
* [Patch YAML "respetuoso" en Rust](https://verrchu.github.io/blog/2-respectful-yaml-patching-in-rust/)

### Guías de Rust
* [Aprende los genéricos y rasgos de Rust creando un mini juego de blackjack](https://blog.sheerluck.dev/posts/learn-generics-traits-in-rust-by-building-blackjack-card-game-engine/)
* [Crea un editor de texto completo desde cero | 0xKiire](https://0xkiire.com/build-text-editor-from-scratch/)
* [Donde el sol sigue brillando: el patrón proveedor](https://bitfieldconsulting.com/posts/sun-keeps-shinin)
* [Procesamiento Geoespacial de Extremo a Extremo con EORST](https://eors-workspace-a6ef35.gitlab.io/posts/002-end-to-end-workflow/) — Construir una canalización de satélites en Rust: consulta STAC a GeoTIFF.
* [Todas las formas de burlarse de tu código Rust](https://blog.appliedcomputing.io/p/all-the-ways-to-mock-your-rust-code)
* [Rust en el desarrollo de Android: Guía completa](https://chayanmistry.medium.com/rust-in-android-development-complete-guide-5f3313f40e50)

### Miscelánea
* [Anunciando la renovación y CFP de Rust-Edu 2026](https://rust-edu.org/news/call-for-participation/)

## Crate de la semana

El crate de esta semana es [cloakrs](https://github.com/kadir/cloakrs), una biblioteca y herramienta CLI para detectar y enmascarar información personal identificable.

A pesar de no tener ninguna sugerencia con la que trabajar, llogiq está satisfecho con su elección.

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

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
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Computación Científica en Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

502 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-05..2026-05-12

#### Compilador
* [considera 'Resultado<T, Deshabitado>' y 'ControlFlow<Deshabitado, T>' equivalentes a 'T' para must use lint](https://github.com/rust-lang/rust/pull/148214)
* [menos búsquedas globales de 'node_id_to_def_id'](https://github.com/rust-lang/rust/pull/156173)
* [introducir expresiones de movimiento ('move($expr)')](https://github.com/rust-lang/rust/pull/155023)
* [resolver: evaluar con entusiasmo las visibilidades privadas en computación de visualización eff](https://github.com/rust-lang/rust/pull/156185)

#### Biblioteca
* [añadir 'Command::get_resolved_envs'](https://github.com/rust-lang/rust/pull/149362)
* [añadir 'Drop::p in_drop' para las caídas clavadas](https://github.com/rust-lang/rust/pull/144537)
* [añadir 'keepalive', 'set_keepalive' a las implementaciones de 'TcpStream'](https://github.com/rust-lang/rust/pull/154025)
* [colocar ZSTs no mapeados en el array 'map'](https://github.com/rust-lang/rust/pull/152487)
* ['drop_glue' de los arrays simplemente desdimensionan y llaman a la versión slice](https://github.com/rust-lang/rust/pull/155184)
* [implementado 'PathBuf::into_string'](https://github.com/rust-lang/rust/pull/156204)

#### Carga
* ['diag': Advertencia/conteo de error de diagnóstico de Rastreo de Carga como se hace en rustc](https://github.com/rust-lang/cargo/pull/16981)
* [sugiere 'fmt' cuando el usuario escribe 'cargo rustfmt'](https://github.com/rust-lang/cargo/pull/16985)
* [reconstruir cuando cambia la dependencia de -Zpublic](https://github.com/rust-lang/cargo/pull/16965)

#### Clippy
* [añadir nueva pelusa 'inline_trait_bounds'](https://github.com/rust-lang/rust-clippy/pull/16486)
* [nueva pelusa: 'manual_clear'](https://github.com/rust-lang/rust-clippy/pull/16617)
* [corregir 'manual_option_zip' falso positivo cuando se usa el parámetro exterior en el cierre](https://github.com/rust-lang/rust-clippy/pull/16970)
* [incompatibilidad de 'non_canonical_clone_impl' y 'implicit_return'](https://github.com/rust-lang/rust-clippy/pull/16949)

#### Analizador de Rust
* [añadir envolver en la lista de árbol con editor](https://github.com/rust-lang/rust-analyzer/pull/22256)
* [añadir diagnóstico para E0436](https://github.com/rust-lang/rust-analyzer/pull/22309)
* [añadir diagnóstico para E0529](https://github.com/rust-lang/rust-analyzer/pull/22334)
* [completo ':': en módulo def](https://github.com/rust-lang/rust-analyzer/pull/22259)
* [patrones de soporte para deref](https://github.com/rust-lang/rust-analyzer/pull/22292)
* [añadir espacios en blanco sobre la finalización del postfijo en la macro](https://github.com/rust-lang/rust-analyzer/pull/22315)
* [no inferir firmas, en su lugar inferir consts anónimos en ellas](https://github.com/rust-lang/rust-analyzer/pull/22198)
* [no sustituir los tipos de puntos de captura de cierre por errores si no se normalizan](https://github.com/rust-lang/rust-analyzer/pull/22319)
* [arreglar el manejo de 'yo' en 'lower_coroutine_body_with_moved_arguments()'](https://github.com/rust-lang/rust-analyzer/pull/22266)
* [arreglar oferta en no relacionada para 'toggle_macro_delimiter'](https://github.com/rust-lang/rust-analyzer/pull/22304)
* [generalmente fijar la resolución de ayuda en semántica](https://github.com/rust-lang/rust-analyzer/pull/22299)
* [en "Implementar miembros faltantes", no añadir tipos asociados con valores predeterminados](https://github.com/rust-lang/rust-analyzer/pull/22291)
* [no hay espacios añadidos en '.. =' en macro dentro de macro](https://github.com/rust-lang/rust-analyzer/pull/22302)
* [proporcionar un InferCtxt a TyLoweringContext](https://github.com/rust-lang/rust-analyzer/pull/22237)
* [proporcionar el mapa fuente para la vinculación reducida 'let self = self' en async fns](https://github.com/rust-lang/rust-analyzer/pull/22318)
* [coincidencia de referencia usa tipo unificado](https://github.com/rust-lang/rust-analyzer/pull/22285)
* [renombrar las variables mut eliminó 'mut' en los patrones generados por macro](https://github.com/rust-lang/rust-analyzer/pull/22303)
* [respetar atributos de pelusa para diagnósticos que no activan su nodo principal](https://github.com/rust-lang/rust-analyzer/pull/22290)
* [eliminar hacer mutar](https://github.com/rust-lang/rust-analyzer/pull/22310)

### Triaje de rendimiento del compilador Rust

Esta semana hubo un par de PRs que han afectado al nuevo solucionador de rasgos, que avanza de forma constante,
En particular, [#156139](https://github.com/rust-lang/rust/pull/156139) fue una actuación enorme. Ganar.
[#156185](https://github.com/rust-lang/rust/pull/156185) computación optimizada de visibilidad, resultando
con hasta un 8% de victoria en la caja 'typenum'.

Triaje hecho por **@Kobzol**.
Rango de revisión: [1d72d7e8.. aa31d6d8](https://perf.rust-lang.org/?start=1d72d7e8136faaebad3a85eeed432e6ea1b2ffab&end=aa31d6d8020dcb7c6e6635648d1ca2bc18caf059&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:------:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,1%, 0,4%] | 62 |
| Regresiones ❌ <br /> (secundario) | 0,5% | [0,1%, 1,5%] | 77 |
| Mejoras ✅ <br /> (primaria) | -1,7% | [-8,8%, -0,2%] | 18 |
| Mejoras ✅ <br /> (secundario) | -13,6% | [-85,6%, -0,0%] | 34 |
| Todos ❌✅ (primario) | -0,2% | [-8,8%, 0,4%] | 80 |

2 regresiones, 2 mejoras, 5 mixtas; 4 de ellos en rollups
31 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d4003fd3999eabaef2bca2c218d10f7547425a96/triage/2026/2026-05-12.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [Fondo de Mantenimiento de la Fundación Rust](https://github.com/rust-lang/rfcs/pull/3931)
* [RFC: Heredamiento de funciones predeterminadas en Cargo](https://github.com/rust-lang/rfcs/pull/3945)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [pelusa en 'core::ffi::c_void' como tipo de retorno](https://github.com/rust-lang/rust/pull/156379)
* [Problema de seguimiento para las notas de lanzamiento de #154647: cambiar 'c_double' a 'f32' en objetivos 'avr'](https://github.com/rust-lang/rust/issues/156477)
* [Estabilizar '--remap-path-prefix' en rustdoc](https://github.com/rust-lang/rust/pull/155307)
* [Sustituir la tabla de imprimibles por tablas 'unicode_data.rs'](https://github.com/rust-lang/rust/pull/155527)
* [Problema de seguimiento para la RFC 2137: Soporte para definir funciones variádicas compatibles con C en Rust (c_variadic](https://github.com/rust-lang/rust/issues/44930)
* [Problema de seguimiento para 'Path::is_empty'](https://github.com/rust-lang/rust/issues/148494)
* [Problema de seguimiento para el formato entero en un búfer de tamaño fijo](https://github.com/rust-lang/rust/issues/138215)
* [resolver: Convertir parcialmente la pelusa de 'ambiguous_glob_imports' en un error difícil](https://github.com/rust-lang/rust/pull/149195)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Propone el concepto de un nombre de usuario crates.io para la identidad](https://github.com/rust-lang/rfcs/pull/3946)
* [RFC de carga para la edad mínima de publicación](https://github.com/rust-lang/rfcs/pull/3923)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Nueva regla 'layout.repr.c.struct.align-empty'](https://github.com/rust-lang/reference/pull/2264)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Establecer el equipo de financiación](https://github.com/rust-lang/leadership-council/issues/294)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 13-05-2026 - el 10-06-2026 🦀

### Virtual
* 2026-05-17 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/548kbqhl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #4 - ¡Codificación de cápsulas en QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Ver tu código - Una guía práctica para rastrear en Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/9v7hv2g1)
* 2026-06-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcjbfb/)
* 2026-06-04 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-07 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/3bcnx1jb)

### Asia
* 2026-05-13 | Malasia, MI | [Rust Meetup Malasia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup mayo 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
* 2026-05-14 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/314649688/)
* 2026-05-16 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de mayo 2026**](https://hasgeek.com/rustbangalore/may-2026-rustacean-meetup/)
* 2026-06-02 | Pekín, CN | [Voice AI y Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Agentes de IA y LLM de código abierto (Llamada a ponentes)**](https://www.meetup.com/wasm-rust-meetup/events/314750465/)

### Europa
* 2026-05-13 | Girona, ES | [Rust Girona](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 - 2026-05-23 | Utrecht, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-18 | Milano, MI, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**SemanaOxidación 2026**](https://www.meetup.com/rust-language-milan/events/314329200/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hacks**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**anuncio de RustWeek 2026**](https://www.meetup.com/rust-nederland/events/312861992/)
* 2026-05-19 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Construcción y Prueba Cruzada**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | Londres, Reino Unido | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Comida de la Semana del Rust**](https://www.meetup.com/women-in-rust/events/314313054/)
* 2026-05-21 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Tour en bicicleta por Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Programación Agente - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Mayo de Rust Manchester**](https://www.meetup.com/rust-manchester/events/314452972/)
* 2026-05-29 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396588/)
* 2026-06-03 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Únete en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)

### Norteamérica
* 2026-05-14 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Utah Rust May Meetup**](https://www.meetup.com/utah-rust/events/314696639/)
* 2026-05-14 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314469265/)
* 2026-05-14 | Portland, OR, EE. UU. [PDXRust](https://www.meetup.com/pdxrust)
    * [**De ondas de radio a píxeles - Visualizaciones en tiempo real con Rust y WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust May Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 2026-05-16 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Lechmere, 16 de mayo**](https://www.meetup.com/bostonrust/events/314480531/)
* 2026-05-19 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, EE. UU. [Encuentro de Rust en el Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Rust en el Área de la Bahía**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro comunitario**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 23 de mayo**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust en sistemas embebidos y autónomos en sistemas paralelos en DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 30-05-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Central de Cambridge, 30 de mayo**](https://www.meetup.com/bostonrust/events/314480537/)
* 2026-06-04 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Pruebas, Cobertura, Tracey y Mutaciones**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de la Óxida Común de Boston, 6 de junio**](https://www.meetup.com/bostonrust/events/314480539/)

### Oceanía
* 2026-05-14 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne - mayo 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
* 2026-05-26 | Barton, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

### Sudamérica
* 2026-05-13 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Rust Uruguay meetup de Mayo**](https://www.meetup.com/rust-uruguay/events/314532884/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién contrata en r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> De las últimas 150 PRs fusionadas con Bun, **108 son próximas a la seguridad de la memoria** — limpieza fallida en una ruta de error, uso después de liberar, lecturas no inicializadas, acceso fuera de límites, reentrancia. **75 de esos no se compilarían** en un lenguaje con destructores, semántica de movimiento y un verificador de préstamo. Uno de cada tres PRs que enviamos es "olvidó liberar algo en una ruta de error."
> 
> De los 108, ~88 están en Zig. Los ~14 en C++ son principalmente ciclos de referencia y razas de concurrencia GC — la clase residual que sobrevive a cualquier lenguaje. Así que la diferencia de Zig→Rust es real: los bugs de Zig son exactamente del tipo destructor o reparable por la propiedad, y el lado de C++ ya está cerca del suelo.
> 
> Sin garantías de tiempo de compilación más sólidas, esto sigue siendo un juego de gato y ratón. La propuesta es eliminar la mayor clase de error estructuralmente en lugar de corregir instancias indefinidamente de ella.

– [Jarred Sumner en el bun github](https://github.com/oven-sh/bun/blob/eeb4d9fdf6e9a7bdd45388d7f3a03dcf570839ad/docs/rust-rewrite-plan.md#why)
¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1765) por la sugerencia!

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1tcjse1/this_week_in_rust_651/)</small>
