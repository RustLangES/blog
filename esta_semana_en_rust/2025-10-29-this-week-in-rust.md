---
title: "Esta semana en Rust #82"
number_of_week: 82
description: El crate de esta semana es tower-resilience, una biblioteca que ofrece funciones de resiliencia para la torre.
date: 2025-10-29
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

### Boletines
* [El Rustaceo Incrustado Edición #57](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-57)

### Actualizaciones de proyectos/herramientas
* [Presentación de Apache Fory™](https://fory.apache.org/blog/fory_rust_versatile_serialization_framework/)
* [SeaORM 2.0: Flujo de trabajo de Entity First](https://www.sea-ql.org/blog/2025-10-30-sea-orm-2.0/)
* [Rari v0.3.0: Marco React basado en Rust con SSR: 12 veces más rápido, 10 veces más rápido que Next.js](https://ryanskinner.com/posts/the-rari-ssr-breakthrough-12x-faster-10x-higher-throughput-than-nextjs)
* [Versión Burn 0.19.0: Cuantización, entrenamiento distribuido y backend LLVM](https://burn.dev/blog/release-0.19.0/)
* [Última versión alfa de Yelken e introducción a Yelken Cloud](https://blog.yelken.io/last-alpha-and-yelken-cloud/)
* [Capnproto 0.22 — métodos asíncronos](https://dwrensha.github.io/capnproto-rust/2025/10/27/0.22-release.html)
* [Fyrox 1.0.0-rc.1](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0-rc-1/)
* [Boa release v0.21](https://boajs.dev/blog/2025/10/22/boa-release-21)
* [Typst: Typst 0.14: Ahora accesible](https://typst.app/blog/2025/typst-0.14/)
* [iroh-blobs 0.95](https://www.iroh.computer/blog/iroh-blobs-0-95-new-features)
* [Esta semana en Heave (2025.10.24)](https://www.rustydonkey.dev/blog/2025.10.24_this_week_in_heave/)

### Observaciones/Pensamientos
* [Cláusulas de captura explícitas](https://smallcultfollowing.com/babysteps/blog/2025/10/22/explicit-capture-clauses/)
* [Capturas de cierre](https://andwass.github.io/rust/2025/10/23/closure-captures.html)
* [Cambios recientes de Rust](https://www.ncameron.org/blog/recent-rust-changes/)
* [Cómo Signal usa Rust para asegurar las comunicaciones de millones de personas](https://kerkour.com/signal-app-rust)
* [A hard rain's a-gonna drop: decoding JSON in Rust](https://bitfieldconsulting.com/posts/hard-rain-json-rust)
* [Producto de trabajo GSoC '25: Expansión macro paralela](https://lorrens.me/2025/10/26/GSoC-Parallel-Macro-Expansion.html)
* [Cuando O3 es 2 veces más lento que O2](https://cat-solstice.github.io/test-pqueue/)
* [El concurso de gatos del registro de cambios de Clippy (rust), una breve retrospectiva](https://blog.goose.love/posts/history-of-clippy-changelog-cat/)
* [audio] [Netstack.FM — Episodio 11 – Redes modernas en Firefox con Max Inden](https://netstack.fm/#episode-11)
* [audio] [Novedades de Rust 1.81 a 1.84](https://rustacean-station.org/episode/rust-1.81-1.82-1.83-1.84/)

### Tutoriales de Rust
* [Análisis de datos en Rust](https://ericfecteau.ca/data/rust-data-analysis/index.html)
* [Cómo evitar luchar contra el verificador de préstamos de Rust](https://qouteall.fun/qouteall-blog/2025/How%20to%20Avoid%20Fighting%20Rust%20Borrow%20Checker)
* [Pruebas unitarias de Rust: simulacros y verificación flexible](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_mock/)
* [Construyendo un agente de codificación en Rust: Introducción](https://blog.0xshadow.dev/posts/coding-agent-in-rust/coding-agent-in-rust-introduction/)
* [Compactador de basura Teddy Bear](https://internet.place/content/teddy-bear-trash-compactor/)
* [Rust para ingenieros de JavaScript - Interactividad Connect-4](https://www.afloat.boats/posts/rust-for-javascript-engineers-interactivity)
* [Limpia tus anotaciones de por vida en Rust con Rc y Arc](https://kerkour.com/rust-lifetimes-rc-arc)
* [Validación de Vibe con Lean, ChatGPT-5, & Claude 4.5: Nueve reglas para demostrar que los algoritmos (Rust) son correctos sin conocer los métodos formales (Parte 2)](https://medium.com/@carlmkadie/081e0f06886d)
* [video] [Ingeniería de backend de Rust Axum 0.8 | Hola mundo](https://www.youtube.com/watch?v=Imb6vJkD0Vc)
* [video] [Agente de codificación de construcción en Rust | Configuración del proyecto](https://www.youtube.com/watch?v=tQJTuYkZ4u8&t=1s)

### Investigación
* [Apoyando 'VIEW's en Diesel](https://blog.weiznich.de/blog/diesel-infer-sql-nullablity/)

## Crate de la semana

El crate de esta semana es [tower-resilience](https://github.com/joshrotenberg/tower-resilience), una biblioteca que ofrece funciones de resiliencia para la torre.

¡Gracias a [Josh Rotenberg](https://users.rust-lang.org/t/crate-of-the-week/2704/1483) por la autosugestión!

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
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

* [Diésel - https://github.com/diesel-rs/diesel/issues/4840](https://github.com/diesel-rs/diesel/issues/4840)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándote con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust).

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 463 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-21..2025-10-28

#### Compilador
* ['-Znext-solver' instanciar el enlazador de predicados sin recanonicalizar el objetivo](https://github.com/rust-lang/rust/pull/146725)
* ['hir_analysis': agregar límites de tamaño faltantes](https://github.com/rust-lang/rust/pull/142712)
* [agregar tipos de patrones no nulos](https://github.com/rust-lang/rust/pull/142339)
* [agregar una ruta rápida para reducir las constantes triviales](https://github.com/rust-lang/rust/pull/148040) (¡gran aceleración!)
* [no extender la vida útil de los índices de matriz/segmento](https://github.com/rust-lang/rust/pull/147083)
* [deduce 'captures(none)' para un lugar de retorno y parámetros](https://github.com/rust-lang/rust/pull/147890)
* [privacidad: introducir algo de almacenamiento en caché para escribir la visita en 'DefIdVisitorSkeleton'](https://github.com/rust-lang/rust/pull/147486)

#### Biblioteca
* [agregue 'FromIterator' impls for 'ascii::Char's a 'String's](https://github.com/rust-lang/rust/pull/141445)
* [añadir 'Cadena::replace_first' y 'Cadena::replace_last'](https://github.com/rust-lang/rust/pull/134316)
* [agregar nueva marca 'inherit_handles' al rasgo CommandExt](https://github.com/rust-lang/rust/pull/115501)
* [métodos de celda const](https://github.com/rust-lang/rust/pull/147788)
* [const 'select_unpredictable'](https://github.com/rust-lang/rust/pull/145939)
* [crear la versión UTF-8 de 'OsStr'/'OsString'](https://github.com/rust-lang/rust/pull/147932)

#### Carga
* [git: admite la búsqueda superficial para el backend de la CLI de Git](https://github.com/rust-lang/cargo/pull/16156)
* [hacer que las variables de finalización del shell sean privadas](https://github.com/rust-lang/cargo/pull/16144)

#### Rustdoc
* [Verifique 'doc (cfg ())' incluso de elementos privados / ocultos](https://github.com/rust-lang/rust/pull/147991)
* ['--emit=depinfo' salida a stdout a través de '-'](https://github.com/rust-lang/rust/pull/147762)

#### Clippy
* ['manual_let_else': envolver expresiones que terminan en ''}''](https://github.com/rust-lang/rust-clippy/pull/15919)
* ['match_as_ref': sugerir 'as_ref' cuando sea necesario emitir la referencia](https://github.com/rust-lang/rust-clippy/pull/15934)
* ['needless_if': no expanda las invocaciones de macros en la sugerencia](https://github.com/rust-lang/rust-clippy/pull/15960)
* ['manual_option_as_slice': mejorar el diagnóstico](https://github.com/rust-lang/rust-clippy/pull/15926)
* ['match_as_ref': mejorar el diagnóstico](https://github.com/rust-lang/rust-clippy/pull/15928)
* ['unnecessary_{find,filter}_map': hacer que los intervalos de diagnóstico sean más precisos](https://github.com/rust-lang/rust-clippy/pull/15929)
* ['{option,result}_map_unit_fn': corregir y limpiar pruebas, hacer sugerencias multilínea](https://github.com/rust-lang/rust-clippy/pull/15871)
* [considerar las etiquetas de ASM en línea como ejecutadas condicionalmente](https://github.com/rust-lang/rust-clippy/pull/15676)
* [corregir 'len_zero' falso positivo en métodos inestables](https://github.com/rust-lang/rust-clippy/pull/15894)
* [posible ambigüedad de precedencia de pelusa entre el cierre y la llamada al método](https://github.com/rust-lang/rust-clippy/pull/14421)

#### Analizador de Rust
* [agregar una API de configuración de extensión](https://github.com/rust-lang/rust-analyzer/pull/20837)
* [evite llamar a la consulta 'specializes()' en cajas que no definen '#! [característica(especialización)]'](https://github.com/rust-lang/rust-analyzer/pull/20921)
* [agregar ayuda para "Flip range expression"](https://github.com/rust-lang/rust-analyzer/pull/20705)
* [añadir la ayuda "Eliminar ramas 'else'"](https://github.com/rust-lang/rust-analyzer/pull/19918)
* [proporcionar una opción para no mostrar derivaciones cerca del ADT para "Goto Implementations" o "Implementations" codelens](https://github.com/rust-lang/rust-analyzer/pull/20186)
* [al cambiar el nombre de 'self' a otro nombre, cambie la sintaxis de llamada al método del método de los llamadores a la sintaxis de Assoc FN](https://github.com/rust-lang/rust-analyzer/pull/20369)
* [add '#[doc = include_str!(" ...")]' finalización](https://github.com/rust-lang/rust-analyzer/pull/20755)
* [agregar finalización de campos de registro abreviado](https://github.com/rust-lang/rust-analyzer/pull/20831)
* [agregar tipo de finalización de palabras clave](https://github.com/rust-lang/rust-analyzer/pull/20571)
* [complete 'else' en más expresiones](https://github.com/rust-lang/rust-analyzer/pull/20658)
* [complete 'let' antes de la expresión en 'if'](https://github.com/rust-lang/rust-analyzer/pull/20912)
* [considerar todas las coincidencias para flyimport incluso cuando se busca con un calificador](https://github.com/rust-lang/rust-analyzer/pull/20919)
* [comprobación de habitabilidad de la matriz fija](https://github.com/rust-lang/rust-analyzer/pull/20905)
* [corregir conversiones y usar API de árbol de sintaxis con tipo en 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/20759)
* [manejar 'if'-'let' en 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/20764)
* [manejar patrones de campo abreviados en 'destructure_tuple_binding'](https://github.com/rust-lang/rust-analyzer/pull/20712)
* [implementar 'Interno::impl_specializes()'](https://github.com/rust-lang/rust-analyzer/pull/20893)
* [mejorar la heurística entre paréntesis de finalización de campo](https://github.com/rust-lang/rust-analyzer/pull/20889)
* [mejorar el manejo de nombres faltantes en 'MethodCallExpr'](https://github.com/rust-lang/rust-analyzer/pull/20886)
* [mejorar el manejo de la macro 'env!'](https://github.com/rust-lang/rust-analyzer/pull/20554)
* [mejorar la heurística de la declaración incompleta](https://github.com/rust-lang/rust-analyzer/pull/20670)
* [Reduzca el bloqueo/cierre asíncrono correctamente](https://github.com/rust-lang/rust-analyzer/pull/20895)
* [ofrecer 'add_braces' en las tareas](https://github.com/rust-lang/rust-analyzer/pull/20844)
* [oferta 'invert_if' en 'else'](https://github.com/rust-lang/rust-analyzer/pull/20771)
* [coloque el nuevo módulo fuera del bloque 'impl' en 'extract_module'](https://github.com/rust-lang/rust-analyzer/pull/20589)
* [soporte 'let' cadenas en 'replace_is_method_with_if_let_method'](https://github.com/rust-lang/rust-analyzer/pull/20913)
* [reducir las asignaciones de 'client_commands' en la proto conversión](https://github.com/rust-lang/rust-analyzer/pull/20922)
* [eliminar 'hir-ty/src/next_solver/mapping.rs'](https://github.com/rust-lang/rust-analyzer/pull/20896)
* [tipo semántico para no lógico](https://github.com/rust-lang/rust-analyzer/pull/20891)

### Triaje de rendimiento del compilador de Rust

Semana mayormente negativa, proveniente casi en su totalidad de agregar límites de tamaño en [#142712](https://github.com/rust-lang/rust/pull/142712). Aparte de eso, obtuvimos una buena victoria para el código asíncrono de la optimización de transformación de estado en [#147493](https://github.com/rust-lang/rust/pull/147493) y bastantes mejoras más pequeñas de la optimización de codegen en [#147890](https://github.com/rust-lang/rust/pull/147890).

Triaje realizado por **@panstromek**.
Rango de revisión: [4068bafe.. 23fced0f](https://perf.rust-lang.org/?start=4068bafedd8ba724e332a5221c06a6fa531a30d2&end=23fced0fcc5e0ec260d25f04a8b78b269e5e90f0&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,7% | [0,2%, 3,7%] | 113 |
| Regresiones ❌ <br /> (secundaria) | 0,5% | [0,1%, 1,7%] | 75 |
| Mejoras ✅ <br /> (primaria) | -0,4% | [-0,7%, -0,2%] | 3 |
| Mejoras ✅ <br /> (secundario) | -2,3% | [-20,8%, -0,1%] | 30 |
| Todos ❌✅ (primarios) | 0,7% | [-0,7%, 3,7%] | 116 |

2 regresiones, 2 mejoras, 7 mixtas; 2 de ellos en rollups
42 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/0d28673aa9dacd2fe062baa1cd5336247f373d57/triage/2025/2025-10-27.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [FCW para enumeraciones repr(C) cuyos valores discriminantes no encajan en una c_int](https://github.com/rust-lang/rust/pull/147017)
* [Problema de seguimiento para maybe_uninit_write_slice](https://github.com/rust-lang/rust/issues/79995)
* [Agregar algunas implicaciones de rasgos de conversión](https://github.com/rust-lang/rust/pull/145504)
* [Problema de seguimiento para 'Duración::from_nanos_u128'](https://github.com/rust-lang/rust/issues/139201)
* [Problema de seguimiento para 'core_slice_as_array'.](https://github.com/rust-lang/rust/issues/133508)
* ['TryFrom<integer>' para 'bool'](https://github.com/rust-lang/rust/pull/147400)
* [Problema de seguimiento para el segmento::array_windows](https://github.com/rust-lang/rust/issues/75027)
* [Problema de seguimiento para '#! [característica(maybe_uninit_slice)]'](https://github.com/rust-lang/rust/issues/63569)
* [Problema de seguimiento para 'lazy_get'](https://github.com/rust-lang/rust/issues/129333)
* [agregar iterador::contiene](https://github.com/rust-lang/rust/pull/141994)
* [Problema de seguimiento para métodos enteros no marcados inherentes](https://github.com/rust-lang/rust/issues/85122)
* [Estabilizar la función de objetivo 'vectorial' s390x y la macro 'is_s390x_feature_detected!'](https://github.com/rust-lang/rust/pull/145656)
* [Actualizar el paquete a 1.2.5](https://github.com/rust-lang/rust/pull/142682)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(Solo MCP)](https://forge.rust-lang.org/compiler/mcp.html)
* [Usar la manipulación v0 de forma predeterminada todas las noches](https://github.com/rust-lang/compiler-team/issues/938)
* [Use 'annotate-snippets' como emisor predeterminado todas las noches](https://github.com/rust-lang/compiler-team/issues/937)
* [Creación de un nuevo conjunto de pruebas de información de depuración mantenible](https://github.com/rust-lang/compiler-team/issues/936)
* [Cambiar a v0 para símbolos que superan los caracteres de 64 KB en la información de depuración de PDB generada por destinos](https://github.com/rust-lang/compiler-team/issues/934)
* [Promover 'riscv64a23-unknown-linux-gnu' al Nivel 2 sin herramientas de host](https://github.com/rust-lang/compiler-team/issues/932)
* [Omitir sugerencias cuando los tramos se superponen](https://github.com/rust-lang/compiler-team/issues/929)
* [Reemplace la cadena 'rustc_target::specTarget::arch' con enumeración](https://github.com/rust-lang/compiler-team/issues/926)
* [Ejecutar las principales pruebas del analizador de Rust en rust-lang / rust CI](https://github.com/rust-lang/compiler-team/issues/923)
* [Activar la ABI de desenrollado de emscripten-wasm-eh de forma predeterminada](https://github.com/rust-lang/compiler-team/issues/920)
* [Objetivo Soporte de nivel 3 para hexagon-unknown-qurt](https://github.com/rust-lang/compiler-team/issues/919)
* [Propuesta de un conjunto de pruebas específico para la interfaz paralela](https://github.com/rust-lang/compiler-team/issues/906)
* [Dar un signo a los literales enteros en lugar de confiar en expresiones de negación](https://github.com/rust-lang/compiler-team/issues/835)
* [Habilitar también volcados de archivos ICE en estable](https://github.com/rust-lang/compiler-team/issues/809)
* [Nueva propuesta de objetivo de nivel 3: 'loongarch64-linux-android'](https://github.com/rust-lang/compiler-team/issues/806)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Propuesta: Requerir que todos los miembros del equipo del proyecto tengan identificaciones de Zulip](https://github.com/rust-lang/leadership-council/issues/228)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Adición de una pestaña Seguridad crates.io](https://github.com/rust-lang/rfcs/pull/3872)

## Próximos eventos

Rusty Eventos entre 2025-10-29 - 2025-11-26 🦀

### Virtual
* 2025-10-29 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-29 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**¡Evento híbrido con Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311721465/)
* 2025-10-29 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/311669921/)
* 2025-10-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-04 | Virtual (Beijing, CN) | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Reunión mensual de la comunidad de WasmEdge, el tiempo de ejecución de LLM / AGI **](https://www.meetup.com/wasm-rust-meetup/events/311759399/)
* 2025-11-05 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Dominando el manejo de errores en Rust: De los pánicos a este error y de todos modos**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
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
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)

### África
* 2025-11-11 | Johannesburgo, ZA | [Reunión de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Más sobre los tipos de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de noviembre de 2025**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europa
* 2025-10-29 | Dortmund, DE | [Rust, Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Encuentro de Rust Dortmund Octubre 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-29 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London Code Dojo: Introducción a Rust incrustado asíncrono**](https://www.meetup.com/rust-london-user-group/events/311670686/)
* 2025-10-30 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️ 🌈 - Edición 008**](https://www.meetup.com/rust-berlin/events/311752307/)
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
* 2025-11-05 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Especial de Halloween.**](https://www.meetup.com/oxford-rust-meetup-group/events/311569796/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Encuentro de Rust Gdansk #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-06 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/events/)
    * [**Inicio de la temporada 2 | en metalab 🦀 **](https://www.meetup.com/rust-vienna/events/311679397/)
* 2025-11-11 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London x Zed Meetup**](https://www.meetup.com/rust-london-user-group/events/311737021/)
* 2025-11-12 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Ginebra, CH | [Rust Ginebra](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Ginebra**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche social**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Lucerna, CH | [Rust de Lucerna]((https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)

### América del Norte
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
* 2025-11-20 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Noviembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-26 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310457310/)

### Oceanía
* 2025-10-29 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Reunión de octubre**](https://www.meetup.com/rust-canberra/events/311234237/)
* 2025-11-11 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/311685331/)

### América del Sur
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

> Petición para agregar una palabra clave 'imprudente' en Rust

– [James Logan en hachyderm.io](https://hachyderm.io/@ponderingpothos/115403971956993021)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1724) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1ojr69c/this_week_in_rust_623/)</small>
