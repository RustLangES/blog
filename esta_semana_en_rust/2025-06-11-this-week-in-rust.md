---
title: "Esta semana en Rust #63"
number_of_week: 63
description: El crate de esta semana es optics, una biblioteca de lentes con todas las funciones y con seguridad de tipos.
date: 2025-06-11
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Un vistazo a las operaciones del equipo del compilador](https://blog.rust-lang.org/inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations)

### Boletines
* [The Embedded Rustacean Issue #47](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-47)
* [Tendencias de Rust Edición #66](https://rust-trends.com/newsletter/rust-rewinded-debug-smarter-build-faster/)

### Actualizaciones de proyectos/herramientas
* [¿Ya estamos integrados? #2](https://jslazak.com/are-we-embedded-yet-2/)
* [Meilisearch 1.15 - nueva configuración de tolerancia de errores tipográficos, operadores de comparación para filtros de cadena y soporte mejorado para chino](https://www.meilisearch.com/blog/meilisearch-1-15)
* [cctx: Un conmutador de contexto para el código Claude](https://syu-m-5151.hatenablog.com/entry/2025/06/05/232126)

### Observaciones/Pensamientos
* [Reescribiendo SymCrypt en Rust para modernizar la biblioteca criptográfica de Microsoft](https://www.microsoft.com/en-us/research/blog/rewriting-symcrypt-in-rust-to-modernize-microsofts-cryptographic-library/)
* [¿Por qué a Rust no le importa más el rendimiento del compilador?](https://kobzol.github.io/rust/rustc/2025/06/09/why-doesn't-rust-care-more-about-compiler-performance.html)
* [Los fondos de cobertura están reemplazando un lenguaje de programación con Rust, pero no es C++](https://www.efinancialcareers.com/news/rust-replacing-c-programming-language-hedge-fund)
* [La trampa de la concurrencia: cómo un contador atómico detuvo un oleoducto](https://www.conviva.com/platform/the-concurrency-trap-how-an-atomic-counter-stalled-a-pipeline/)
* [Rust para software fundacional](https://corrode.dev/blog/foundational-software/)
* [10 años apostando por Rust](https://tably.com/tably/10-years-of-betting-on-rust)
* [Informe sobre las discusiones sobre genéricos variádicos en RustWeek 2025.](https://poignardazur.github.io/2025/06/07/report-on-variadics-rustweek/)
* [Registros funcionales de costo cero en Rust](https://ecency.com/rust-lang/@jonwolski/zero-cost-functional-records-in-rust)
* [Un plan para SIMD](https://linebender.org/blog/a-plan-for-simd/)
* [¿Cuándo una función de Rust es "insegura"?](https://crescentro.se/posts/when-unsafe/)
* [Nueve reglas para las bibliotecas científicas en Rust](https://medium.com/@carlmkadie/nine-rules-for-scientific-libraries-in-rust-6e5e33a6405b)
* [Rust a dieta](https://blog.veeso.dev/blog/en/rust-on-a-diet/)
* [Rust vs Go: Cuál elegir en 2025](https://blog.jetbrains.com/rust/2025/06/12/rust-vs-go/)
* [audio] [Novedades de Rust 1.79 y 1.80](https://rustacean-station.org/episode/rust-1.79-1.80/)
* [audio] [Rust en el trabajo con Ran Reichman, cofundador y CEO de Flarion](https://rustacean-station.org/episode/ran-reichman/)
* [lista de reproducción de videos] [RustWeek 2025](https://www.youtube.com/playlist?list=PL8Q1w7Ff68DCEXiGidlM0DMn8ztjlUlez)

### Tutoriales de Rust
* [Introducción al desarrollo embebido con Rust: Visión general del ecosistema](https://kerkour.com/introduction-to-embedded-development-with-rust)
* [Lograr una latencia de <100 ms para control remoto con WebRTC](https://gethopp.app/blog/latency-exploration)
* [Patrones para modelar datos variantes superpuestas en Rust](https://mcmah309.github.io/posts/patterns-for-modeling-overlapping-variant-data-in-rust/)
* [¿Es Rust más rápido que C?](https://steveklabnik.com/writing/is-rust-faster-than-c/)
* [video] [Faceta introductoria: Reflexión para Rust](https://www.youtube.com/watch?v=0mqFCqw_XvI)
* [video] [Combinando Swift y Rust con UniFFI: Have Your Cake & Eat It Too](https://www.youtube.com/watch?v=DfSBBOlFTeE)

### Investigación
* [Seguridad de la memoria del kernel: misión cumplida](https://asterinas.github.io/2025/06/04/kernel-memory-safety-mission-accomplished.html)

### Miscelánea
* [Leyendo sobre Rust con el entrenador, consultor y autor Herbert Wolverson](https://filtra.io/rust/interviews/ardan-jun-25)
* [video] [Julian Hofer - Pixi: el compañero desaparecido de la carga](https://www.youtube.com/watch?v=Hso3TQx13b0)

## Crate de la semana

El crate de esta semana es [optics](https://crates.io/crates/optics), una biblioteca de lentes con todas las funciones y con seguridad de tipos.

¡Gracias a [Akos Vandra](https://users.rust-lang.org/t/crate-of-the-week/2704/1442) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 516 solicitudes de extracción en la última semana[fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-06-03..2025-06-10

#### Compilador

* [añadir (atrás) 'unsupported_calling_conventions' lint para rechazar más convenciones de llamada no válidas](https://github.com/rust-lang/rust/pull/141435)
* [añadir un nuevo lint 'mismatched-lifetime-syntaxes'](https://github.com/rust-lang/rust/pull/138677)
* [Vía rápida para las obligaciones estancadas en materia de autoservicio](https://github.com/rust-lang/rust/pull/141681)
* [simplificar y optimizar el 'SlotIndex::from_index' de 'VecCache'](https://github.com/rust-lang/rust/pull/142095)
* [reelaborar 'collect_and_apply' para no depender de la sugerencia de tamaño para la optimización](https://github.com/rust-lang/rust/pull/141652)
* [miri: TB: añadir bandera para desactivar el seguimiento más preciso de la mutabilidad interior](https://github.com/rust-lang/miri/pull/4376)
* [miri: native-lib: permitir múltiples bibliotecas y/o directorios](https://github.com/rust-lang/miri/pull/4372)

#### Biblioteca

* [estabilizar 'os_string_pathbuf_leak'](https://github.com/rust-lang/rust/pull/137992)
* [estabilizar 'const_eq_ignore_ascii_case'](https://github.com/rust-lang/rust/pull/142065)
* [estabilizar 'nonnull_provenance'](https://github.com/rust-lang/rust/pull/142238)
* [estabilizar 'sha512', 'sm3' y 'sm4' para x86](https://github.com/rust-lang/rust/pull/140767)
* [estabilizar 'tcp_quickack'](https://github.com/rust-lang/rust/pull/129121)
* [bootstrap: construir punteros de marco de hoja std sans](https://github.com/rust-lang/rust/pull/141800)
* [hacer posible 'NonZero<char>'](https://github.com/rust-lang/rust/pull/141001)
* [optimize 'Seek::stream_len' impl para 'File'](https://github.com/rust-lang/rust/pull/125087)

#### Clippy

* ['doc_suspicious_footnotes': texto de lint que parece una nota al pie de página](https://github.com/rust-lang/rust-clippy/pull/14708)
* ['missing_const_for_fn': considere la constancia de la instancia](https://github.com/rust-lang/rust-clippy/pull/14759)
* ['zombie_processes': no se queje de los rendimientos tempranos](https://github.com/rust-lang/rust-clippy/pull/14912)
* [añadir nueva pelusa: 'ip_constant'](https://github.com/rust-lang/rust-clippy/pull/14878)
* [No pelar los códigos generados por macros](https://github.com/rust-lang/rust-clippy/pull/14976)
* [no repetir indefinidamente mientras se comprueba la mutabilidad interna](https://github.com/rust-lang/rust-clippy/pull/14965)
* [arreglar 'branches_sharing_code' sugiere erróneamente cuando se trata de macros](https://github.com/rust-lang/rust-clippy/pull/14907)
* [arreglar 'create_dir' ignora las rutas en las sugerencias](https://github.com/rust-lang/rust-clippy/pull/15011)
* [arreglar 'match_single_binding' falla los rizos en las firmas de tipo](https://github.com/rust-lang/rust-clippy/pull/15017)
* [arreglar 'std_instead_of_core' FP cuando parte del 'uso' no puede ser reemplazado](https://github.com/rust-lang/rust-clippy/pull/15016)
* [arreglar 'unnecessary_debug_formatting' FP dentro de 'Depurar' impl](https://github.com/rust-lang/rust-clippy/pull/14955)
* [corregir falso positivo para 'unused_unit'](https://github.com/rust-lang/rust-clippy/pull/14962)
* [arreglar sugerencia-causas-error de 'print_literal' y 'write_literal'](https://github.com/rust-lang/rust-clippy/pull/14961)
* [introducir 'coerce_container_to_any'](https://github.com/rust-lang/rust-clippy/pull/14812)
* [Sugerir invertir si se prueba la no nulidad del puntero](https://github.com/rust-lang/rust-clippy/pull/15015)
* [lint orden inverso en orden parcial impl](https://github.com/rust-lang/rust-clippy/pull/14945)
* [use cadenas internadas cuando sea posible, por motivos de eficiencia](https://github.com/rust-lang/rust-clippy/pull/14963)

#### Analizador de Rust

* [Mejor recuperación del analizador para llamadas de macro en posición vinculada a tipo](https://github.com/rust-lang/rust-analyzer/pull/19933)
* [añadir sugerencias de incrustación de palabras clave 'dyn'](https://github.com/rust-lang/rust-analyzer/pull/19922)
* [implementar finalizaciones de atributos para el módulo de diagnóstico](https://github.com/rust-lang/rust-analyzer/pull/19908)
* [Incluya siempre correcciones rápidas para los diagnósticos, incluso cuando los diagnósticos estén deshabilitados](https://github.com/rust-lang/rust-analyzer/pull/19935)
* [no se equivoque en las implicaciones para los tipos sin tamaño que no incluyen elementos 'where Self: Sized'](https://github.com/rust-lang/rust-analyzer/pull/19963)
* [grabar llamadas de macro para campos en 'ChildBySource' impls](https://github.com/rust-lang/rust-analyzer/pull/19937)
* [grabar llamadas de macro en firmas en 'ChildBySource' impls](https://github.com/rust-lang/rust-analyzer/pull/19932)
* [estabilizar el diagnóstico "JSON no es Rust"](https://github.com/rust-lang/rust-analyzer/pull/19949)
* [Estabilizar diagnóstico de archivo no vinculado](https://github.com/rust-lang/rust-analyzer/pull/19936)
* [hir-ty: añadir pruebas incrementales comprobando la invalidación 'inferir'](https://github.com/rust-lang/rust-analyzer/pull/19914)
* [hacer que 'Semantics<'db, DB>' soporte 'Semantics<'db, dyn HirDatabase>', tome dos](https://github.com/rust-lang/rust-analyzer/pull/19930)

### Clasificación del rendimiento del compilador de Rust

Semana mayormente positiva, con muchas mejoras en el sistema de tipos, especialmente en el nuevo solucionador y una gran victoria en el código de almacenamiento en caché. Las regresiones provienen de nuevas advertencias, con un impacto descomunal en un banco de pruebas con una gran cantidad de código generado.

Triaje realizado por **@panstromek**.
Rango de revisión: [2fc3deed.. C31CCCB7](https://perf.rust-lang.org/?start=2fc3deed9fcb8762ad57191e0195f06f7543e4a5&end=c31cccb7b5cc098b1a8c1794ed38d7fdbec0ccb0&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 3.1% | [0.3%, 8.5%] | 22 |
| Regresiones ❌ <br /> (secundaria) | 0.6% | [0.2%, 0.9%] | 3 |
| Mejoras ✅ <br /> (primario) | -1.0% | [-3.4%, -0.2%] | 151 |
| Mejoras ✅ <br /> (secundaria) | -3,5% | [-66.5%, -0.2%] | 146 |
| Todos ❌✅ (primarios) | -0,4% | [-3.4%, 8.5%] | 173 |

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/7a1e00ae0a30c783bdfa8e3c35e3b49ce88b58e9/triage/2025-06-09.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para 'mixed_integer_ops_unsigned_sub'](https://github.com/rust-lang/rust/issues/126043)
* [¡Permitir el almacenamiento de 'format_args! ()' en variable](https://github.com/rust-lang/rust/pull/140748)
* [Problema de seguimiento para la API de bloqueo de archivos](https://github.com/rust-lang/rust/issues/130994)
* [Jerarquía de tamaños: Parte I](https://github.com/rust-lang/rust/pull/137944)
* [Permitir el acceso volátil a la memoria que no es de Rust, incluida la dirección 0](https://github.com/rust-lang/rust/pull/141260)
* [const-eval: permite que las constantes se refieran a la memoria mutable/externa, pero rechaza tales constantes como patrones](https://github.com/rust-lang/rust/pull/140942)
* [Reportar nunca escribir lints en dependencias](https://github.com/rust-lang/rust/pull/141937)
* [builtin dyn impl no guide inference](https://github.com/rust-lang/rust/pull/141352)
* [Cambia la impl predeterminada de 'core::iter::Fuse' para hacer lo que sus documentos dicen que hace](https://github.com/rust-lang/rust/pull/140985)
* [Estabilizar deriva (CoercePointee)](https://github.com/rust-lang/rust/pull/133820)
* [impl 'Default' for 'array::IntoIter'](https://github.com/rust-lang/rust/pull/141574)
* [Añadida la implementación de 'Clone' para 'ChunkBy'](https://github.com/rust-lang/rust/pull/138016)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Literales de cadena dentados](https://github.com/rust-lang/rfcs/pull/3830)

## Próximos eventos

Eventos oxidados entre 2025-06-11 - 2025-07-09 🦀

### Virtual
* 11/06/2025 | Virtual (Tel Aviv, Illinois) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversación con Herbert Wolverson de Ardan Labs y LibreQoS**](https://www.meetup.com/code-mavens/events/308234298/)
* 2025-06-12 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 2025-06-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/jibhz3zs)
* 15/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308074808)
* 17/06/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170853)
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 19/06/2025 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/vna66he1)
* 22/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308246353)
* 24/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361436)
* 24/06/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Construyendo Raspadores Web Eficientes: Rust vs. Python para la Ingesta de Datos**](https://www.meetup.com/women-in-rust/events/306683025)
* 2025-06-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/cgamfls6)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 29/06/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
* 02/07/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031667)
* 03/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820304)
* 05/07/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 06/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308298511)
* 08/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305361452)

### África
* 17/06/2025 | Johannesburgo, ZA | [Reunión de Rust en Johannesburgo](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Cierres de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/308274745)

### Asia
* 14/06/2025 | Kuala Lumpur, MY | [Rust Malasia x APUGDC x ACM SIGGRAPH KL](https://www.eventbrite.sg/cc/rust-lang-malaysia-meetup-2017119)
    * [**Suave introducción a GameDev con Bevy (Rust Meetup junio de 2025)**](https://docs.google.com/forms/d/e/1FAIpQLSeDgCDwORn42mgWoMeMlEJJ2LPsOQzqcgdEPW_L3ipuBFbNbQ/viewform)
* 28/06/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de junio de 2025**](https://hasgeek.com/rustbangalore/june-2025-rustacean-meetup/)
* 02/07/2025 | Seúl, KR | [Encuentro de programación de Rust en Seúl](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Rust en Seúl**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europa
* 11/06/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 2025-06-12 | Berlín, DE | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️ 🌈 - Edition 003**](https://www.meetup.com/rust-berlin/events/308131380)
* 17/06/2025 | Cambridge, Gran Bretaña | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/308294416)
* 17/06/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 18/06/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Magello**](https://www.meetup.com/stockholm-rust/events/308129156)
* 19/06/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Encuentro de Rust Aarhus en Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 19/06/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 24/06/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 25/06/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Lecciones aprendidas de hacer un pequeño juego en nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)
* 2025-06-26 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**18º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/308399403)
* 2025-06-26 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Encuentro de Rust #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 2025-06-26 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentro de Rust #77**](https://www.meetup.com/rust-paris/events/308416060)
* 01/07/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #9**](https://www.meetup.com/rust-gdansk/events/308349712)
* 02/07/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
* 09/07/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 07 2025**](https://lu.ma/hismn492)
* 09/07/2025 | Lectura, GB | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtyhckbmb)

### América del Norte
* 11/06/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 2025-06-12 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/308019627)
* 17/06/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 17/06/2025 | San Francisco, CA, EE. UU. | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Taller de Rust por Vara Network**](https://lu.ma/acwxdnwq)
* 18/06/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307730493)
* 19/06/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 19/06/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx)
    * [**programación asíncrona en Rust usando Tokio**](https://www.meetup.com/rust-mx/events/308248260)
* 19/06/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Uso de Rust para Web Series 3: Presentaciones finales y redes sociales de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 19/06/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Junio de 2025 Encuentro de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-20 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Lechmere Rust, 20 de junio**](https://www.meetup.com/bostonrust/events/307936242)
* 25/06/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust en el grupo de desarrolladores de Web3**](https://www.meetup.com/rust-los-angeles/events/308401269)
* 2025-06-26 | Los Ángeles (Chino Hills), CA, ESTADOS UNIDOS | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Rust en Web3**](https://lu.ma/ek8jx2r3)
* 2025-06-26 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Meetup mensual: ¡Haciendo una API CRUD con Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 28/06/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Back Bay Rust, 28 de junio**](https://www.meetup.com/bostonrust/events/307936269)
* 03/07/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Construcción de Servicios de Rust Resilientes y Observables con steady_state**](https://www.meetup.com/stl-rust/events/306345853)
* 06/07/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Alewife Rust, 6 de julio**](https://www.meetup.com/bostonrust/events/307936287)

### Oceanía
* 11/06/2025 | Sídney, NS, AU | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**Tiempo del cangrejo junio 🦀 **](https://www.meetup.com/rust-sydney/events/308325643)
* 16/06/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Encuentro de Rust en Christchurch**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 24/06/2025 | Barton, Australia | [Grupo de usuarios de Canberra Rust (CRUG)](https://www.meetup.com/rust-canberra)
    * [**Reunión CRUG de junio**](https://www.meetup.com/rust-canberra/events/307520854)

### América del Sur
* 2025-06-12 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Junio de WebAssembly!**](https://www.meetup.com/rust-argentina/events/307990465)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1knkfb6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> No mires al abismo, no vaya a ser que te reconozcan como un ***experto en el dominio del abismo***, y esperen que sigas mirando la maldita cosa.

– [Nick Mathewson en twitter](https://x.com/nickm_tor/status/860234274842324993?lang=en)

¡Gracias a [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1696) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1l9vp64/this_week_in_rust_603/)</small>
