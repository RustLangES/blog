---
title: "Esta semana en Rust #125"
number_of_week: 125
description: El crate de esta semana es tokio-rcu, una implementación de RCU en espacio de usuario construida específicamente en torno a la semántica de rust asíncrona y tokio. 
date: 2026-09-09
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

* [Resultados de la encuesta de depuración de Rust 2026](https://blog.rust-lang.org/2026/09/07/rust-debugging-survey-2026-results/)

### Fundación

* [Iniciativa de interoperabilidad Rust/C++: Actualización de progreso, septiembre de 2026](https://rustfoundation.org/media/rust-cpp-interop-initiative-progress-update/)

### Boletines

* [Rust Trends Número 81 - Rust 1.98 se lanza mientras se pone a prueba la cadena de suministro](https://rust-trends.com/newsletter/rust-1-98-ships-as-the-supply-chain-gets-tested/)

### Actualizaciones de proyectos/herramientas

* [Una década de ladrones](https://rustls.dev/blog/2026-09-08-a-decade-of-rustls/)

### Observaciones/Pensamientos

* [Microcontroladores con buen soporte para Rust](https://kerkour.com/rust-microcontrollers)
* [¿Cuánto cuesta un tiempo de ejecución de datos gobernados? TeaQL vs Diésel y SeaORM en MusicBrainz](https://teaql.io/blog/musicbrainz-rust-orm-benchmark/)
* [Tipo nunca de Estabilizating Rust](https://lwn.net/SubscriberLink/1091015/5009546caa744c57/)
* [Buscando a través de 150 GiB de texto por segundo con SIMD](https://pid7.com/blog/searching-150gb-text-per-second/)
* [Nueve reglas para trabajo en tiempo de compilación con Rust 'const fn': Análisis de archivos, tablas de compilación y detección de errores ... sin un script de compilación (Parte 2)](https://levelup.gitconnected.com/nine-rules-for-compile-time-work-with-rust-const-fn-part-2-76ccd0e8a965)
* [Una exploración espacial de diseño de Async/Await](https://cel.cs.brown.edu/blog/design-space-async-await/)
* [Rust: Cuando el vacío no es fondo](https://ettolrach.com/blog/rust_when_empty_isnt_bottom.html)

### Guías de Rust

* [Qué cambió realmente el +simd128 de Rust en mi WebAssembly](https://www.debugdiary.dev/log/rust-simd128-what-changed-in-webassembly)
* [Flujo de Control de Rust en la práctica - Crear un juego de adivinanzas de números](https://blog.sheerluck.dev/posts/understanding-rust-control-flow-by-building-a-number-guessing-game/)
* [Desdimensionando valores no dimensionados](https://hackmd.io/@WorldSEnder/Hkyqni6Ofl)
* [Arquitectura del juego](https://quietism.art/posts/game-architecture/)
* [Presentando CUDA Rust: Dos pistas para escribir kernels de GPU](https://developer.nvidia.com/blog/introducing-cuda-rust-two-tracks-for-writing-gpu-kernels/)
* [El estado de los asignadores en 2026 - 6 meses después](https://cetra3.github.io/blog/state-of-allocators-2026-part-2/)
* [Visualizando los Vtables de Rust: Cómo funciona el rasgo dyn en la memoria](https://sofiabelen.github.io/projects/visualizing-rusts-vtables-how-dyn-trait-works-in-memory/)
* [Generación segura de movimientos legales de ajedrez a 475.000.000 de nodos/s](https://bamburac.com/blog/chess-engine/)
* [Acelerando el hash de marcha en ARM64 (2× más rápido)](https://sam.dev/blog/gearhash-on-arm64)
* [Construyamos un compresor desde cero](https://ochagavia.nl/blog/lets-build-a-compressor-from-scratch/)
* [Ingeniería inversa de mi patinete eléctrico y reescritura del firmware con Rust](https://bensimms.moe/reverse-engineering-scooter/)
* [Gloo + Tejo para estado persistente de la webapp](https://hemomorphic.alexblood.net/posts/gloo-yew-for-persistent-webapp-state/)

### Miscelánea

* [Migraciones de Rust Impresionantes](https://github.com/kevincouton/awesome-rust-migrations)

## Crate de la semana

El crate de esta semana es [tokio-rcu](https://github.com/roeeshoshani/tokio_rcu), una implementación de RCU en espacio de usuario construida específicamente en torno a la semántica de rust asíncrona y tokio. 

¡Gracias a [Roee Shoshani](https://users.rust-lang.org/t/crate-of-the-week/2704/1662) por la autosugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrezca instrucciones de prueba y/o orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

##### [Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para la frescura de la suma de comprobación](https://github.com/rust-lang/cargo/issues/14136)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
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
* [sysknife - action_reference_doc_is_current imprime dos documentos de 44 KB en lugar de la línea que difiere](https://github.com/lacs-project/sysknife/issues/345)
* [sysknife - packages/setup afirma soporte para el Nodo 18, y el Nodo 18 está en fin de vida desde el 30-04-2025](https://github.com/lacs-project/sysknife/issues/327)
* [sysknife - Carga prueba falla intermitentemente en main: a test establece una variable ambiental global de proceso](https://github.com/lacs-project/sysknife/issues/356)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->
- *No se presentaron convocatorias ni presentaciones esta semana.* 

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

613 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-01..2026-09-08

#### Compilador
* [siempre se repite si normalizamos los opacos locales](https://github.com/rust-lang/rust/pull/161795)
* [optimizar flujos de tokens vacíos](https://github.com/rust-lang/rust/pull/162047)
* [tienda LiveLoans más densamente poblada](https://github.com/rust-lang/rust/pull/161850)
* [usar IndexVec en lugar de BTreeMap para varianzas de polonio](https://github.com/rust-lang/rust/pull/162422)

#### Biblioteca
* [añadir soporte para asignadores personalizados en '(try_)map' en 'UniqueArc' y 'UniqueRc'](https://github.com/rust-lang/rust/pull/161893)
* [Box: Arreglar 'Mapa/try_map' Llamadas de Desalocar](https://github.com/rust-lang/rust/pull/162285)
* [estabilizar funciones inteligentes de mapeo de puntero](https://github.com/rust-lang/rust/pull/160534)

#### Carga
* ['docs(lints)': cómo configurar lints de carga](https://github.com/rust-lang/cargo/pull/17441)
* ['docs(trim-paths)': añadir limitaciones y pulir](https://github.com/rust-lang/cargo/pull/17425)
* ['docs(trim-paths)': el remapeo del espacio de trabajo comienza con '.'](https://github.com/rust-lang/cargo/pull/17433)
* ['fix(git)': Aplicar pr hint a git-fetch-with-cli](https://github.com/rust-lang/cargo/pull/17437)
* ['fix(git)': Haz que la descripción de PR se cierre en nuestra guía de estilo](https://github.com/rust-lang/cargo/pull/17436)
* ['fix(git)': Simplificar mensaje de error](https://github.com/rust-lang/cargo/pull/17429)
* ['fix(git)': Usa el 429 de git's retry, cuando esté disponible](https://github.com/rust-lang/cargo/pull/17422)
* ['arreglar (analizador)': Resolver uso teórico después de liberar](https://github.com/rust-lang/cargo/pull/17428)
* [evitar pasar los args de ruta de búsqueda (-L) cuando se pasan como --extern](https://github.com/rust-lang/cargo/pull/17410)
* [documentación: cambiar de "target triple" a "target tuple"](https://github.com/rust-lang/cargo/pull/17430)
* [corregir el manejo relativo del enlace simbólico en 'write_atomic'](https://github.com/rust-lang/cargo/pull/17362)
* [corregir(trim-paths)!: limitar las opciones a 'ninguno|objeto|todos'](https://github.com/rust-lang/cargo/pull/17432)
* [corrección(trim-paths)!: eliminar el alcance predeterminado del perfil de lanzamiento](https://github.com/rust-lang/cargo/pull/17424)
* [comentario ranciado sobre el método de revisión de huellas dactilares](https://github.com/rust-lang/cargo/pull/17450)

#### Rustdoc
* [añadir '--opción de imprimir'](https://github.com/rust-lang/rust/pull/151618)

#### Rustfmt
* [corregir el documento de bloqueo no idempotente comentario más cercano reescribir](https://github.com/rust-lang/rustfmt/pull/7017)
* [evitar bucles infinitos al analizar objetos de los brazos 'cfg_select!'](https://github.com/rust-lang/rustfmt/pull/7089)

#### Clippy
* ['unnecessary_self_imports': importaciones anidadas de pelusa](https://github.com/rust-lang/rust-clippy/pull/17653)
* ['legacy_numeric_constants': hacer correcciones aplicables a la máquina](https://github.com/rust-lang/rust-clippy/pull/17490)
* ['std_instead_of_core': No sugieres un camino que no se resuelve](https://github.com/rust-lang/rust-clippy/pull/17648)
* ['useless_conversion': ignora 'From::from' en el código generado](https://github.com/rust-lang/rust-clippy/pull/17583)
* ['useless_format': mejora la sugerencia](https://github.com/rust-lang/rust-clippy/pull/16595)
* ['regex_creation_in_loops': comprobar estructura del bucle MIR](https://github.com/rust-lang/rust-clippy/pull/17681)
* [comprueba que los enlaces intra-doc no están rotos](https://github.com/rust-lang/rust-clippy/pull/17504)
* [detectar pruebas de integración en 'is_in_test'](https://github.com/rust-lang/rust-clippy/pull/16786)
* [no activar 'integer_division_remainder_used' en macros](https://github.com/rust-lang/rust-clippy/pull/17049)
* [mejora la pelusa 'map_unwrap_or' para soportar 'map(f).unwrap_or_default()'](https://github.com/rust-lang/rust-clippy/pull/17644)
* [trasladar la integración de 'clippy_ci_panic_test' a una prueba normal](https://github.com/rust-lang/rust-clippy/pull/17502)
* [el respeto en línea permite en 'needless_pass_by_value'](https://github.com/rust-lang/rust-clippy/pull/17665)
* [renombrar suavemente 'clippy::all' a 'clippy::d efault'](https://github.com/rust-lang/rust-clippy/pull/14689)

#### Analizador de Rust
* [añadir diagnósticos de cuerpos desaparecidos de forma gratuita y objetos asociados](https://github.com/rust-lang/rust-analyzer/pull/23262)
* [corregir constructores 'NamedTempFile'](https://github.com/rust-lang/rust-analyzer/pull/23292)
* [aceptar Self como segmento de camino no principal en los caminos de atributos](https://github.com/rust-lang/rust-analyzer/pull/23249)
* [permitir atributos internos en bloques en expresiones de tuplas](https://github.com/rust-lang/rust-analyzer/pull/23246)
* [evitar errores de unificación de tipos en la búsqueda de términos](https://github.com/rust-lang/rust-analyzer/pull/22662)
* [arreglar el manejo de '#[unsafe()]' atrae sin meta interna](https://github.com/rust-lang/rust-analyzer/pull/23270)
* [corregir análisis de 'self': en la lista de params de fn](https://github.com/rust-lang/rust-analyzer/pull/23163)
* [pasa el cursor '1f64' usa float en lugar de entero](https://github.com/rust-lang/rust-analyzer/pull/23279)
* [seguir los enlaces simbólicos al escanear la sysroot en busca de proc-macro dylibs](https://github.com/rust-lang/rust-analyzer/pull/23297)
* [instalar herramientas de carga con dependencias bloqueadas](https://github.com/rust-lang/rust-analyzer/pull/23248)
* [fusionar 'hir_def::hir::Expr::Unsafe' en 'Expr::Block'](https://github.com/rust-lang/rust-analyzer/pull/23271)
* [render const value en los detalles de la etiqueta de completions](https://github.com/rust-lang/rust-analyzer/pull/23266)

### Triaje de rendimiento del compilador Rust

Esta semana hemos experimentado bastantes regresiones, tanto esperadas como inesperadas. 
Uno de ellos ya ha sido arreglado, y se están discutiendo soluciones para algunas otras. 
Una gran mejora viene de almacenar en caché el set de desinfectante en 'Session', lo que corrige una gran regresión respecto a la semana pasada. 
Algunas mejoras menores lograron, incluyendo una reducción del 75% en el uso de memoria al compilar 'bevy_render' con el siguiente solucionador de rasgos. 

Triaje hecho por **@JonathanBrouwer**. 
Rango de revisión: [5321a4f4.. 656a9da1](https://perf.rust-lang.org/?start=5321a4f40c957cf3587c055e77461febc2ebc865&end=656a9da186dacaf3bf8f7f7296a825d256cb4ae3&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,1%, 1,3%] | 121 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,1%, 10,3%] | 106 |
| Mejoras ✅ <br /> (primaria) | -0,6% | [-1,9%, -0,1%] | 63 |
| Mejoras ✅ <br /> (secundario) | -0,6% | [-2,4%, -0,1%] | 65 |
| Todos ❌✅ (primario) | 0,1% | [-1,9%, 1,3%] | 184 |


3 regresiones, 2 mejoras, 8 mixtas; 6 de ellas en rollups
En total se realizaron 33 comparaciones de artefactos

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/a953e9c59b18feffa9dc06bee4ab30ad5a5700e8/triage/2026/2026-09-07.md)

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrezca instrucciones de prueba y/o orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

##### [Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para la frescura de la suma de comprobación](https://github.com/rust-lang/cargo/issues/14136)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

--- 

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* [Matemáticas de Rustdoc LaTeX](https://github.com/rust-lang/rfcs/pull/3958)
* [RFC: Descripciones de características de carga](https://github.com/rust-lang/rfcs/pull/3485)
* [Cambiar 'i686-pc-windows-msvc' de Tier 1 con herramientas de host => Tier 1 sin herramientas de host](https://github.com/rust-lang/rfcs/pull/3999)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [RISCV: estabilizar características objetivo 'd' y 'f'](https://github.com/rust-lang/rust/pull/161385)
* [x86: en objetivos que requieren SSE, usa esos registros para ABI](https://github.com/rust-lang/rust/pull/161583)
* [Reexportar 'core::fmt::NumBuffer' en 'alloc' (y 'std')](https://github.com/rust-lang/rust/pull/161430)
* [corrección: cod muerto anidado sin cumplir](https://github.com/rust-lang/rust/pull/161005)
* [convertir error alineado-in-empaquetado en pelusa](https://github.com/rust-lang/rust/pull/162160)
* [Garantía de 8 bytes de alineación de RawWakerVTable](https://github.com/rust-lang/rust/pull/158186)
* [libtest: Permitir pasar --test-threads y --color varias veces, con argumentos posteriores sobrescribiendo antes](https://github.com/rust-lang/rust/pull/161312)
* [Estabilizar 'núcleo::mem::D ropGuard'](https://github.com/rust-lang/rust/pull/161520)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [corrección(instalar): usar archivo de bloqueo empaquetado por defecto](https://github.com/rust-lang/cargo/pull/17388)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [UB no viaja en el tiempo](https://github.com/rust-lang/reference/pull/2320)

##### [Directrices del Código de Peligros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [¿Pueden las referencias dentro de un enum establecer transitoriamente un discriminante de enum inválido?](https://github.com/rust-lang/unsafe-code-guidelines/issues/621)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo Compilador](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.* 

## Próximos eventos

Eventos Rusty entre el 09-09-2026 y el 07-10-2026 🦀

### Virtual
* 2026-09-09 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Club de Libros de Sistemas Operativos: Espacios de direcciones y API de memoria**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316283651/)
* 2026-09-10 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Núremberg, DE) | [Núremberg Oxidado](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Construcción de un controlador de GPU Rust en el kernel de Linux**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-18 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ibaxicxv)
* 2026-09-2026 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316133974/)
* 2026-09-22 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254773/)
* 2026-09-24 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn de Oxid**](https://www.meetup.com/rust-berlin/events/315907979/)
* 2026-09-24 | Virtual (Charlottesville, VA, EE.UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Celdas de Rust — Mutabilidad Interior de Núcleo de Rust a Sistemas Operativo Tock**](https://www.meetup.com/charlottesville-rust-meetup/events/316460694/)
* 2026-09-29 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Cajas, consejos y trucos Charlas relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/yqxvguts)
* 2026-10-04 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316134009/)
* 2026-10-06 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773044/)
* 07-10-2026 | Virtual (Indianápolis, IN, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)

### Europa
* 2026-09-10 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-09-14 - 2026-09-16 | Berlín, DE | [Oxidar 2026](https://oxidizeconf.com/)
    * [**Oxidar 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproduciendo artículos científicos - con Rust & "IA"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)
* 2026-09-15 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**Tras la Máscara de Async Rust**](https://www.meetup.com/madrust/events/316361267/)
* 2026-09-17 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup - De Segfault a Seguridad @DiWoDo**](https://www.meetup.com/rust-dortmund/events/316428507/)
* 22-09-2026 | Praga, CZ | [Praga Oxidada](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga @ Rockwell Automation**](https://www.meetup.com/rust-prague/events/316070376/)
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de Charla en SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Ámsterdam, Países Bajos | [Grupo Oxidados Ámsterdam de Rust](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Codificación Agente IA**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-28 | Augsburgo, DE | [Encuentro Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #21: Maximilian Grauvogl & Marcel Fink - De bits a bugs: Un generador de Rust para manifiestos de SUIT y fuzzing de analizadores conscientes de la estructura**](https://rust-augsburg.github.io/meetup/Meetup_21.html)
* 2026-09-29 | Manchester, Reino Unido | [Manchester Oxidado](https://www.meetup.com/rust-manchester/events/)
    * [**Noche del Código de Septiembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/316200964/)
* 30-09-2026 | Basilea, CH | [Basilea Oxidada](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-10-05 | Múnich, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)

### Norteamérica
* 2026-09-08 - 2026-09-11 | Híbrido (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 09-09-2026 | Montreal, CA | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Encuentro de pausa café de RustConf**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Rust Integrado Práctico**](https://www.meetup.com/utah-rust/events/316198046/)
* 10-09-2026 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 2026-09-12 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Union Square en Somerville, 12 de septiembre**](https://www.meetup.com/bostonrust/events/310983699/)
* 15-09-2026 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Encuentro de gráficos**](https://luma.com/9oiujuyw)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Construcción de un controlador de GPU Rust en el kernel de Linux**](https://www.meetup.com/vancouver-rust/events/314233757/)
* 2026-09-17 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de septiembre de 2026**](https://www.meetup.com/seattle-rust-user-group/events/315635881/)
* 2026-09-17 | Mountain View, CALI, EE.UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316176445/)
* 2026-09-19 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de la Óxida Común de Boston, 19 de septiembre**](https://www.meetup.com/bostonrust/events/316378813/)
* 2026-09-23 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-24 | Atlanta, GA, EE.UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Harvard Rust, 26 de septiembre**](https://www.meetup.com/bostonrust/events/316378817/)
* 2026-10-01 | Saint Louis, MO, EE. UU. | [Rust STL](https://www.meetup.com/stl-rust/events/)
    * [**construyendo un contenedor mínimo y sin raíces en Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 03-10-2026 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Alewife Rust, 3 de octubre**](https://www.meetup.com/bostonrust/events/316378820/)

### Oceanía
* 2026-09-29 | Barton, AU | [Grupo de usuarios Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de septiembre**](https://www.meetup.com/rust-canberra/events/316398052/)


Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1vtuq1b/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> El Demonio tiene acceso a una Máquina de Súper Turing. 

– [Connor Horman sobre zulipán de Rust](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/.E2.9C.94.20Can.20IO.20provide.20angelic.20choice/near/621828055)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1798) por la sugerencia! 

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1wc8sbm/this_week_in_rust_668/)</small>