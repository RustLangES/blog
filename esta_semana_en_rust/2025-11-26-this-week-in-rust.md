---
title: "Esta semana en Rust #86"
number_of_week: 86
description: El crate de esta semana es grapheme-utils, una biblioteca de funciones para trabajar ergonómicamente con grafemas UTF.
date: 2025-11-26
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
* [Cambiando al propio plan destrozador de Rust cada noche | Rust Blog](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)
* [Entrevista con Jan David Nose | Blog de Rust](https://blog.rust-lang.org/2025/11/25/interview-with-jan-david-nose/)
* [Este ciclo de desarrollo en carga: 1,92 | Inside Rust Blog](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/)

### Fundación
* [vídeo] [Entrevista: Christian Legnitto, Mantenedor: rust-gpu, rust-cuda](https://www.youtube.com/watch?v=monOq_uHHcg)

### Actualizaciones de proyectos/herramientas
* [SeaORM 2.0: Activmodelo anidado y operaciones en cascada](https://www.sea-ql.org/blog/2025-11-25-sea-orm-2.0/)
* [Symbolica 1.0: Matemáticas simbólicas en Rust](https://symbolica.io/posts/stable_release/)
* [Requisito de APT Rust plantea dudas](https://lwn.net/SubscriberLink/1046841/c7ac9fabff6244af/)

### Observaciones/Pensamientos
* [Oxidación en tiempo real](https://tweedegolf.nl/en/blog/198/running-real-time-rust)
* [Una mirada a Rust de 2012](https://purplesyringa.moe/blog/a-look-at-rust-from-2012/)
* [Argumentando que las funciones de carga podrían mejorarse para reducir los tiempos de compilación de Rust](https://saghm.com/cargo-features-rust-compile-times/)
* [Cómo Cloudflare utiliza Rust para servir (y romper) millones de sitios web a 50+ millones de solicitudes por segundo](https://kerkour.com/how-cloudflare-uses-rust)
* [audio] [Netstack.FM episodio 15 — Pingora con Edward y Noah de Cloudflare](https://netstack.fm/#episode-15)
* [vídeo] [Grind: Java merece herramientas modernas*](https://www.youtube.com/watch?v=-mOby4FPRXg)

### Guías de Rust
* [Pruebas Unitarias de Rust: Lectura de archivos](https://jorgeortiz.dev/posts/rust_unit_testing_file_reading/)
* [Lecciones prácticas de rendimiento de Apache DataFusion](https://greptime.com/blogs/2025-11-25-datafusion)
* [Describiendo datos binarios con Deku](https://codeconstruct.com.au/docs/deku-elf-parser/)

### Miscelánea
* [Co-Mantenedor del Kernel de Rust For Linux se retira formalmente](https://www.phoronix.com/news/Alex-Gaynor-Rust-Maintainer)
* [JetBrains apoya los proyectos de código abierto de Rust Ratatui y Biome](https://blog.jetbrains.com/blog/2025/11/18/open-source-in-focus-projects-we-re-proud-to-support/)
* [filtra.io | La "punta de lanza" de Toyota es elegir el Rust](https://filtra.io/rust/interviews/woven-by-toyota-nov-25)

## Crate de la semana

El crate de esta semana es [grapheme-utils](https://github.com/rustkins/grapheme-utils), una biblioteca de funciones para trabajar ergonómicamente con grafemas UTF.

¡Gracias a [rustkins](https://users.rust-lang.org/t/crate-of-the-week/2704/1495) por la autosugerencia!

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
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si eres un implementador de funciones y quieres que tu RFC aparezca en la lista anterior, añade la nueva 'llamada para pruebas'
etiqueta a tu RFC junto con un comentario que ofrezca instrucciones de prueba y/o orientación sobre qué aspecto(s) de la funcionalidad
Necesito pruebas.

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
* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/) | CFP cierra el 24-11-2025 | Varsovia, Polonia | 2025-03-19 - 2025-03-2025 | [Página web del evento](https://www.rustikon.dev/)
* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp) | CFP cierra el 08-12-2025 | Portland, Oregón, EE. UU. | 2026-04-20
* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 31-12-2025 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

456 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-18..2025-11-25

#### Compilador
* [permitir tipos no normalizados en la elaboración de gotas](https://github.com/rust-lang/rust/pull/148719)
* [evitar codificar no constancia o no asincronía en los metadatos](https://github.com/rust-lang/rust/pull/149054)
* [arreglar el código de MaybeUninit usando GVN](https://github.com/rust-lang/rust/pull/147827)
* [sugerencia de corrección para la macro 'cfg!'](https://github.com/rust-lang/rust/pull/148484)
* [manejar ciclos al comprobar candidatos impl para 'doc(hidden)'](https://github.com/rust-lang/rust/pull/149185)
* [const impl inherente](https://github.com/rust-lang/rust/pull/148434)
* [recomienda usar un HashMap si el segundo parámetro genérico de un HashSet no implementa BuildHasher](https://github.com/rust-lang/rust/pull/147171)
* [reducir lints confusos de 'unreachable_code'](https://github.com/rust-lang/rust/pull/149044)
* [reemplazar OffsetOf por una suma real de llamadas a intrinseco](https://github.com/rust-lang/rust/pull/148151)
* [sess: por defecto el símbolo v0 se desforma en cada noche](https://github.com/rust-lang/rust/pull/89917)
* [convertir los movimientos en copias tras la propagación de copias](https://github.com/rust-lang/rust/pull/147804)
* [advertir contra llamadas que muten un elemento 'const' mutable interior](https://github.com/rust-lang/rust/pull/148407)

#### Biblioteca
* [añadir 'bit_width' por 'NonZero' sin <T>signar](https://github.com/rust-lang/rust/pull/148797)
* [alloc: corregir la implementación 'Debug' de 'ExtractIf'](https://github.com/rust-lang/rust/pull/147035)
* [hacer que los intrínsecos SIMD estén disponibles en contextos 'const'](https://github.com/rust-lang/rust/pull/147521)
* [coincide '<OsString' como 'Debug>::fmt' con la de str](https://github.com/rust-lang/rust/pull/148798)
* [mira si este es el momento en que podemos eliminar 'layout::size_align'](https://github.com/rust-lang/rust/pull/149109)
* [desenrollar ret ty de 'iter::ArrayChunks::into_remainder'](https://github.com/rust-lang/rust/pull/149127)
* [v0 deformando para una ETS en Nightly](https://github.com/rust-lang/rust/pull/149148)
* [hashbrown: añadir métodos 'HashTable' relacionados con el índice bruto del bucket](https://github.com/rust-lang/hashbrown/pull/657)
* [hashbrown: permitir proporcionar la clave en el momento de inserción para EntryRef](https://github.com/rust-lang/hashbrown/pull/579)

#### Carga
* ['docs(guía)': Al sugerir un perfil alternativo de desarrollador, enlaza a un problema relacionado](https://github.com/rust-lang/cargo/pull/16275)
* ['feat(generar-bloqueo)': Añadir inestable --bandera de tiempo de publicación](https://github.com/rust-lang/cargo/pull/16265)
* ['feat(tree)': Añadir más completaciones nativas](https://github.com/rust-lang/cargo/pull/16296)
* ['fix(bindeps)': no propagar dependencia de artefactos para activar macros o deps de construcción](https://github.com/rust-lang/cargo/pull/15788)
* ['fix(config-include)': no permitir la sintaxis de glob y plantilla](https://github.com/rust-lang/cargo/pull/16285)
* ['fix(package)': excluir destino/paquete de copias de seguridad](https://github.com/rust-lang/cargo/pull/16272)
* ['refactorización(tiempos)': recogida y presentación de datos separada](https://github.com/rust-lang/cargo/pull/16282)
* ['test(config-include)': incluir siempre en relación con incluir config](https://github.com/rust-lang/cargo/pull/16286)
* [activar 'CARGO_CFG_DEBUG_ASSERTIONS' en scripts de compilación basados en el perfil](https://github.com/rust-lang/cargo/pull/16160)
* [dote: emitir una advertencia cuando se especifican tanto 'package.publish' como '--index'](https://github.com/rust-lang/cargo/pull/16268)
* [prueba: volver a activar la prueba porque ya no es inestable](https://github.com/rust-lang/cargo/pull/16287)

#### Rustdoc
* [rustdoc-json: añadir la ruta rlib a ExternalCrate para habilitar una resolución robusta de la caja](https://github.com/rust-lang/rust/pull/149043)
* [rustdoc: hacer que la información de cajas fusionables sea más útil](https://github.com/rust-lang/rust/pull/148234)

#### Clippy
* ['explicit_deref_methods': no te metas en 'impl Deref(Mut)'](https://github.com/rust-lang/rust-clippy/pull/16113)
* [añadir 'big-error-ignored' config-knob](https://github.com/rust-lang/rust-clippy/pull/15697)
* [corrección 'useless_asref' sugiere erróneamente cuando se usa en ctor](https://github.com/rust-lang/rust-clippy/pull/16115)
* [corregir macros incorrectamente desordenadas para 'transmute_ptr_to_ptr' y 'transmute_bytes_to_str'](https://github.com/rust-lang/rust-clippy/pull/16105)
* [tomar un puntero en bruto en un campo de unión es una operación segura](https://github.com/rust-lang/rust-clippy/pull/16079)

#### Analizador de Rust
* [añadir 'inseguro...'' completación de atributos (https://github.com/rust-lang/rust-analyzer/pull/21047)
* [añade un número bonito para 'add_explicit_enum_discriminant'](https://github.com/rust-lang/rust-analyzer/pull/20559)
* [añadir fichas semánticas para elementos obsoletos](https://github.com/rust-lang/rust-analyzer/pull/21100)
* [añadir token semántico obsoleto para la taquigrafía de exCt](https://github.com/rust-lang/rust-analyzer/pull/21116)
* [añadir asistencia para convertir char literal](https://github.com/rust-lang/rust-analyzer/pull/21093)
* [permitir inferir tamaños de arrays](https://github.com/rust-lang/rust-analyzer/pull/21061)
* [soporte básico para macros declarativas de atributo/derivación](https://github.com/rust-lang/rust-analyzer/pull/21121)
* [completación '= $0' después del predicado keyval cfg](https://github.com/rust-lang/rust-analyzer/pull/21083)
* [derivar ParamEnv de GenericPredicates](https://github.com/rust-lang/rust-analyzer/pull/21059)
* [no sugieres completaciones duplicadas de 'const' 'raw'](https://github.com/rust-lang/rust-analyzer/pull/20937)
* [mejora 'remove_parentheses' ayuda para manejar expresiones de retorno](https://github.com/rust-lang/rust-analyzer/pull/21090)
* [la función de extracción entra en pánico con más de un uso de variable en macro](https://github.com/rust-lang/rust-analyzer/pull/21053)
* [arreglo incorrect_case' en objetos estáticos 'no_mangle'](https://github.com/rust-lang/rust-analyzer/pull/21048)
* [corrección no aplicable en 'y' para 'replace_method_eager_lazy'](https://github.com/rust-lang/rust-analyzer/pull/20967)
* [arreglar no llenar el brazo protegido para 'add_missing_match_arms'](https://github.com/rust-lang/rust-analyzer/pull/21111)
* [corregir la línea de seguimiento en 'tool_path'](https://github.com/rust-lang/rust-analyzer/pull/21088)
* [fijar completación de campos en patrones irrefutables](https://github.com/rust-lang/rust-analyzer/pull/21065)
* [corregir el bloqueo de solicitudes de formato en la consulta 'crate_def_map'](https://github.com/rust-lang/rust-analyzer/pull/21084)
* [información de parámetro de corrección con argumentos faltantes](https://github.com/rust-lang/rust-analyzer/pull/21126)
* [fijar alguna inferencia de patrones](https://github.com/rust-lang/rust-analyzer/pull/21060)
* [incluir todos los tipos de destino con rutas fuera de la raíz del paquete](https://github.com/rust-lang/rust-analyzer/pull/21098)
* [infiere correctamente los patrones de rango](https://github.com/rust-lang/rust-analyzer/pull/21113)
* [hacer configurables las pistas de incrustación dyn](https://github.com/rust-lang/rust-analyzer/pull/21068)
* [hacer que la completación del postfijo gestione correctamente todas las referencias](https://github.com/rust-lang/rust-analyzer/pull/21036)
* [mover diagnósticos de visibilidad para campos para corregir la ubicación](https://github.com/rust-lang/rust-analyzer/pull/21018)
* [nunca eliminar paréntesis de los operadores con prefijo con retorno/interrupción/continuar sin valor](https://github.com/rust-lang/rust-analyzer/pull/21092)
* [analizar archivos de configuración de carga con orígenes](https://github.com/rust-lang/rust-analyzer/pull/21015)
* [eliminar algunas normalizaciones profundas de inferir](https://github.com/rust-lang/rust-analyzer/pull/20980)
* [reescribir la resolución del método para seguir más de cerca a rustc](https://github.com/rust-lang/rust-analyzer/pull/20974)
* [no mostrar error cuando los parámetros coinciden con nombres de macros](https://github.com/rust-lang/rust-analyzer/pull/21074)
* [implementa precedencia para 'print_hir'](https://github.com/rust-lang/rust-analyzer/pull/21057)
* [mejora la asistencia calificada para la primera en el primer segmento](https://github.com/rust-lang/rust-analyzer/pull/21042)
* [infiere el patrón del rango completamente](https://github.com/rust-lang/rust-analyzer/pull/21026)
* [integrar soporte para postales en la CLI del servidor proc-macro](https://github.com/rust-lang/rust-analyzer/pull/20986)
* [optimizar 'SmolStr::clone' 4-5x aceleración en línea, 0,5x heap (ralentizar)](https://github.com/rust-lang/rust-analyzer/pull/21017)
* [PERF: Mejorar el tiempo de arranque](https://github.com/rust-lang/rust-analyzer/pull/21046)
* [perf: rasgo prime implica en el cebado de caché](https://github.com/rust-lang/rust-analyzer/pull/21087)
* [PERF: Produce menos informes de progreso](https://github.com/rust-lang/rust-analyzer/pull/21085)
* [PERF: reducir asignaciones en 'try_evaluate_obligations'](https://github.com/rust-lang/rust-analyzer/pull/21086)
* [imprimir más información macro en volcados de 'DefMap'](https://github.com/rust-lang/rust-analyzer/pull/21094)
* [proc-macro-srv: reimplementar árboles de token mediante árboles inmutables](https://github.com/rust-lang/rust-analyzer/pull/21097)
* [soporte para variantes múltiples para 'generate_from_impl_for_enum'](https://github.com/rust-lang/rust-analyzer/pull/21038)
* [usar tipo inferido en "extraer tipo como tipo alias" asiste y muestra el marcador de posición de tipo inferido '_' pistas de incrustación](https://github.com/rust-lang/rust-analyzer/pull/20125)

### Triaje de rendimiento del compilador Rust

Solo unos pocos cambios relacionados con el rendimiento se lograron esta semana. El mayor fue cambiar el esquema predeterminado de manipulación de nombres en Nightly por la versión v0, que produce nombres de símbolos ligeramente más grandes, por lo que tuvo un pequeño efecto negativo en los tamaños binarios y el tiempo de compilación.

Triaje hecho por **@kobzol**.
Rango de revisión: [6159a440.. b64df9d1](https://perf.rust-lang.org/?start=6159a44067ebce42b38f062cc7df267a1348e092&end=b64df9d1012f2482b54a4d959548cf8fc67e820c&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,9% | [0,3%, 2,7%] | 48 |
| Regresiones ❌ <br /> (secundario) | 0,9% | [0,2%, 2,1%] | 25 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-6,8%, -0,1%] | 33 |
| Mejoras ✅ <br /> (secundario) | -0,5% | [-1,4%, -0,1%] | 53 |
| Todos ❌✅ (primario) | 0,4% | [-6,8%, 2,7%] | 81 |

1 regresión, 2 mejoras, 5 mixtas; Uno de ellos en rollups
28 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/abaa823dbb9569ddf8d5c8a9fa4738106a4eb947/triage/2025/2025-11-25.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

<!-- Utiliza * [Título del artículo](URL del artículo) - o * *No se han aprobado RFCs esta semana.* -->

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Haz que la captura de cierre tenga un comportamiento consistente y correcto alrededor de los patrones](https://github.com/rust-lang/rust/pull/138961)
* [coacción variada limpia y gestiona correctamente la seguridad](https://github.com/rust-lang/rust/pull/148602)
* [Implementa 'TryFrom<char>' para 'usize'.](https://github.com/rust-lang/rust/pull/146792)
* [Contratos: afirmaciones de propiedad primitivas: 'poseído' y 'bloquear'](https://github.com/rust-lang/compiler-team/issues/942)
* [validación de const: eliminar comprobar que referencias mutables en el valor final de const](https://github.com/rust-lang/rust/pull/148746)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o 
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Rasgos exhaustivos. Rasgos que permiten lanzar rasgos cruzados entre objetos de rasgos.](https://github.com/rust-lang/rfcs/pull/3885)
* [convenciones de llamadas CMSE](https://github.com/rust-lang/rfcs/pull/3884)
* ['RUSTC_ALLOW_UNSTABLE_<feature>': una alternativa 'RUSTC_BOOTSTRAP'(https://github.com/rust-lang/rfcs/pull/3882)
* [Etapas Objetivo, una mejora del sistema incremental](https://github.com/rust-lang/rfcs/pull/3881)

## Próximos eventos

Eventos Rusty entre el 26-11-2025 - el 24-12-2025 🦀

### Virtual
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/q5tjirkt)
* 27-11-2025 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Meet de Noviembre - ¡Runtimes asíncronos, parte 2!**](https://www.meetup.com/rust-argentina/events/312061828/)
* 30-11-2025 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de Rust Readers: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Advento del código - ¡Arranca!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 03-12-2025 | Virtual (Buffalo, NY, EE. UU.) [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 03-12-2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311886445/)
* 04-12-2025 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/306046643/)
* 05-12-2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [¡Inicio del Juego de Navidad de Rust & C++!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103307/)
* 2025-12-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763878687)
* 2025-12-07 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Final del Rust & C++ Christmas Game Jam**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103329/)
* 2025-12-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305361537/)
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/li5de4ts)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046672/)
* 2025-12-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/6v2rorp3)
* 2025-12-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361448/)

### Asia
* 2025-12-08 | Tokio, JP | [Rust Global: Tokio](https://rustfoundation.org/event/rust-global-tokyo/)
    * [**Rust Global: Tokyo**](https://rustfoundation.org/event/rust-global-tokyo/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de diciembre 2025**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)

### Europa
* 2025-11-26 | Berna, CH | [Bern Oxidado](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #5 @Source Ingenieros**](https://www.meetup.com/rust-bern/events/311792568/)
* 27-11-2025 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #16: Christian Meusel - Oxidando paso a paso**](https://rust-augsburg.github.io/meetup/Meetup_16.html)
* 27-11-2025 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**19º encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/311787736/)
* 27-11-2025 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Berlín Oxidado en localización 🏳️ 🌈 - Edición 009**](https://www.meetup.com/rust-berlin/events/312169727/)
* 27-11-2025 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #63 patrocinado por Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/312070502/)
* 27-11-2025 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Tipos de tamaño exótico, y Rust en el espacio en la aguja!**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Praga, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 03-12-2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 03-12-2025 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Encuentro Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 04-12-2025 | Viena, AT | [Viena Oxidada](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna S2E2 - Diciembre | en metalab 🦀 **](https://www.meetup.com/rust-vienna/events/311680386/)
* 2025-12-08 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - diciembre de 2025**](https://www.meetup.com/rust-dortmund/events/312165912/)
* 2025-12-08 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)

### Norteamérica
* 2025-11-26 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-26 | Phoenix, AZ, EE. UU. | [Rust del Desierto](https://www.meetup.com/desert-rustaceans)
    * [**Booze.rs**](https://www.meetup.com/desert-rustaceans/events/312000222/)
* 27-11-2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Harvard Square, 29 de noviembre**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Talk diciembre**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 04-12-2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 04-12-2025 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Dominando el estado, la seguridad y los manejadores escalables en Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 05-12-2025 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: ¡Nuestro mayor evento hasta la fecha!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Rust en el centro, 6 de diciembre**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Robótica Competitiva con Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 13 de diciembre**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-20 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 20 de diciembre**](https://www.meetup.com/bostonrust/events/311917280/)
* 2025-12-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312076080/)

### Oceanía
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup dic 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Además: un programa escrito en Rust tenía un error y, aunque causaba tiempo de inactividad, *no hubo ningún problema de seguridad y los datos de nadie fueron comprometidos*.

– [Josh Triplett en /r/rust](https://www.reddit.com/r/rust/comments/1p3dc7y/comment/nq4alwr/)

¡Gracias a [Michael Voelkl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1732) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1p7q9we/this_week_in_rust_627/)</small>
