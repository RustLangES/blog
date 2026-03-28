---
title: "Esta semana en Rust #103"
number_of_week: 103
description: El crate de esta semana es noq en Rust puro.
date: 2026-03-25
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
* [Lo que hemos oído sobre los desafíos de Rust](https://blog.rust-lang.org/2026/03/20/rust-challenges/)
* [Aviso de seguridad para carga](https://blog.rust-lang.org/2026/03/21/cve-2026-33056/)

### Fundación
* [Canonical se une a la Fundación Rust como miembro de oro](https://rustfoundation.org/media/canonical-joins-the-rust-foundation-as-a-gold-member/)

### Boletines
* [Scientific Computing in Rust #16 (marzo de 2026)](https://scientificcomputing.rs/monthly/2026-03)

### Actualizaciones de proyectos/herramientas
* [Ferox - Un cliente nativo de PostgreSQL en Rust](https://dev.to/frkdrgt/i-built-a-postgresql-client-in-rust-because-dbeaver-was-eating-my-ram-1eg1)
* [Presentando dial9: Un grabador de vuelo para Tokio](https://tokio.rs/blog/2026-03-18-dial9)
* [Zellij 0.44: soporte nativo para Windows, nuevas APIs de Rust, sesiones remotas](https://zellij.dev/news/remote-sessions-windows-cli/)
* [vigil-rs: Supervisor de Servicio de Rust para Contenedores con manejo PID 1](https://blog.none.at/blog/2026/2026-03-21-virgil/)
* [Fyrox 1.0.0](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/)
* [Edge.js: ejecutando Node.js de forma segura en un sandbox de WebAssembly con Wasmer y WASIX](https://wasmer.io/posts/edgejs-safe-nodejs-using-wasm-sandbox)
* [Bookokrat v0.3.8: Un lector terminal EPUB / PDF Book ahora soporta DJVU](https://github.com/bugzmanov/bookokrat/releases/tag/v0.3.8)
* [FLODL v0.1.5: benchmarking de Rust frente a PyTorch en 7 modelos — hasta un 30% más rápido con variación 3-20 veces más ajustada](https://flodl.dev/blog/benchmarks)
* [Protobuf en copia cero y ConnectRPC para Rust](https://dev.to/iainmcgin/zero-copy-protobuf-and-connectrpc-for-rust-1m3e)
* [mtp-rs: biblioteca MTP pura de Rust, hasta 4 veces más rápida que libmtp](https://www.veszelovszki.com/a/mtp-rs/)
* [vídeo] [Batty: Ejecución supervisada de agentes para equipos de software — Demo](https://www.youtube.com/watch?v=2wmBcUnq0vw)
* [indxr v0.2.0: Un indexador rápido de base de código y servidor MCP para agentes de codificación de IA](https://github.com/bahdotsh/indxr/releases/tag/v0.2.0)
* [halloy 2026.5 - cliente IRC de escritorio con capacidades IRCv3](https://github.com/squidowl/halloy/releases/tag/2026.5)

### Observaciones/Pensamientos
* [Bloqueo de un Tokio Mutex sin sujetar un candado](https://www.e6data.com/blog/deadlocking-tokio-mutex-without-holding-lock)
* [El bueno, el malo y lo chorreante: jemalloc, bumpalo y mimalloc en meilisearch](https://blog.kerollmops.com/the-good-the-bad-and-the-leaky-jemalloc-bumpalo-and-mimalloc-in-meilisearch)
* [Tipos de vista mínimos máximos](https://smallcultfollowing.com/babysteps/blog/2026/03/21/view-types-max-min/)
* [Piezas de puzzle que coinciden y puntos de referencia decepcionantes](https://llogiq.github.io/2026/03/20/case.html)
* [¿Y si los rasgos llevaran valores?(https://nadrieril.github.io/blog/2026/03/22/what-if-traits-carried-values.html)
* [Una notación de efectos basada en cláusulas con y bloques](https://blog.yoshuawuyts.com/a-with-based-effect-notation)
* [Roscas de Rust en la GPU](https://www.vectorware.com/blog/threads-on-gpu)
* [Elaborando rasgos de Rust en el estilo de paso de diccionario](https://nadrieril.github.io/blog/2026/03/20/dictionary-passing-style.html)

### Guías de Rust
* [ZK se burla de Rust Developer parte 2/8](https://rustarians.com/roots-of-unity/)
* [Veamos el analizador CSV SIMD de Paul Allen](https://chunkofcoal.com/posts/simd-csv/)
* [Construir un servidor LSP con Rust es sorprendentemente fácil y divertido](https://codeinput.com/blog/lsp-server)
* [Un Rust incoherente](https://www.boxyuwu.blog/posts/an-incoherent-rust/)
* [Construcción de dispositivos de pentest con microcontroladores Rust y ESP32](https://kerkour.com/rust-esp32-pentest)
* [Rust en el lib-common de Intersec, Parte 1: Integrando Rust en un sistema de compilación en C](https://techtalk.intersec.com/2026/03/rust-in-lib-common-part-1-integrating-rust-in-a-waf-based-c-build-system/)

## Crate de la semana

El crate de esta semana es [noq](https://github.com/n0-computer/noq), una implementación de propósito general del [protocolo de transporte QUIC](https://www.rfc-editor.org/rfc/rfc9000.html) en Rust puro.

¡Gracias a [Brendan O'Brien](https://users.rust-lang.org/t/crate-of-the-week/2704/1569) por la autosugerencia!

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
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [Almendras silvestres — Appimage no se inicia](https://github.com/opeolluwa/almonds/issues/75)
* [Almendras silvestres — implementar espacio de trabajo para user_preference](https://github.com/opeolluwa/almonds/issues/116)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17
* [**NDC Techtown 2026**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-05-03 | Kongsberg, Noruega | 2026-09-21 - 2026-09-24

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

433 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-17..2026-03-24

#### Compilador
* [arreglar algunas sugerencias de la pelusa de 'los bucles por encima de los falibles'](https://github.com/rust-lang/rust/pull/153913)
* [patrones de guardia: bajando a UTHIR](https://github.com/rust-lang/rust/pull/153828)
* [introduce '#[diagnostic::on_move(mensaje)]'](https://github.com/rust-lang/rust/pull/150935)
* [hacer que 'par_slice' sea consistente con la ejecución de un solo hilo](https://github.com/rust-lang/rust/pull/153768)
* [privacidad: corrección de tipos de vulnerabilidades en los RPITITs](https://github.com/rust-lang/rust/pull/152543)

#### Biblioteca
* [añadir APIs para tratar con titlecase](https://github.com/rust-lang/rust/pull/122668)
* [añadir funciones de 'is_disconnected' a los canales mpsc y mpmc](https://github.com/rust-lang/rust/pull/153170)
* [implementa 'BinaryHeap::as_mut_slice'](https://github.com/rust-lang/rust/pull/154011)
* [haz que 'OsString::truncate' sea un no-op cuando 'len > current_len'](https://github.com/rust-lang/rust/pull/152998)
* [optimizar formato entero de 128 bits](https://github.com/rust-lang/rust/pull/154077)
* [optimizar 'BTreeMap::append()' usando CursorMut](https://github.com/rust-lang/rust/pull/153107)
* ['vec::D rain::fill': evitar referencias a memoria no inicializada](https://github.com/rust-lang/rust/pull/154138)
* [impl inestable de 'From<{i64, u64}> for f128'](https://github.com/rust-lang/rust/pull/154012)

#### Carga
* [limpio: Validar que 'target_dir' no es un archivo](https://github.com/rust-lang/cargo/pull/16765)
* ['cli': Añadir soporte para completar valores de argumento '--config' con 'native-completions'](https://github.com/rust-lang/cargo/pull/16249)
* ['cli': complete '--config' y '--color' antes del comando](https://github.com/rust-lang/cargo/pull/16780)
* ['compilar': Hacer build.warnings ignorar deps no locales](https://github.com/rust-lang/cargo/pull/16760)
* [corregir 'symlink_and_directory' al ejecutar un nombre de director objetivo largo](https://github.com/rust-lang/cargo/pull/16775)
* [detectar ciclo circular de dependencia de publicación en la publicación de espacio de trabajo](https://github.com/rust-lang/cargo/pull/16722)
* [corregir la obtención de especificaciones git no estándar en repositorios que no son github](https://github.com/rust-lang/cargo/pull/16768)
* [advertencia al instalar con una cadena de herramientas no predeterminada](https://github.com/rust-lang/cargo/pull/16131)

#### Clippy
* [añadir 'BinaryHeap::p op_if()' a 'manual_pop_if'](https://github.com/rust-lang/rust-clippy/pull/16734)
* [corregir 'collapsible_match' falso positivo cuando la unión del pat se mueve o muta](https://github.com/rust-lang/rust-clippy/pull/16708)
* [perf: 'manual_is_ascii_check', eliminar 822 millones de instrucciones](https://github.com/rust-lang/rust-clippy/pull/16755)

#### Analizador de Rust
* [añadir la implementación 'ops::AddAssign' para IndentLevel](https://github.com/rust-lang/rust-analyzer/pull/20601)
* [añadido aplicable en LetExpr para 'unwrap_tuple'](https://github.com/rust-lang/rust-analyzer/pull/20600)
* [añadido aplicable en la rama let-else para 'unwrap_block'](https://github.com/rust-lang/rust-analyzer/pull/21473)
* [añadir nombre de rasgo automático para 'generate_trait_from_impl'](https://github.com/rust-lang/rust-analyzer/pull/20299)
* [añadir correcciones para el diagnóstico de 'non_exhaustive_let'](https://github.com/rust-lang/rust-analyzer/pull/20762)
* [añadir mapeo a métodos constructores de sintaxis fábrica](https://github.com/rust-lang/rust-analyzer/pull/21832)
* [añadir soporte anidado de por vida para 'add_lifetime_to_type'](https://github.com/rust-lang/rust-analyzer/pull/20628)
* [añadir selección parcial para 'merge_imports'](https://github.com/rust-lang/rust-analyzer/pull/20566)
* [añadir envolver múltiples atracciones para 'wrap_unwrap_cfg_attr'](https://github.com/rust-lang/rust-analyzer/pull/20625)
* [cambiar 'test_name' como marcador de posición por 'executable_arg'](https://github.com/rust-lang/rust-analyzer/pull/21395)
* [bloque completo .let en expresión de clausura](https://github.com/rust-lang/rust-analyzer/pull/21756)
* [oferta ''add_braces'' en asignación de expansión de contenedores](https://github.com/rust-lang/rust-analyzer/pull/21850)
* [oferta en let-expr para 'inline_local_variable'](https://github.com/rust-lang/rust-analyzer/pull/21775)
* [corregir análisis de operandos de sincronía asm para fragmentos expr entre paréntesis](https://github.com/rust-lang/rust-analyzer/pull/21588)
* [corregir sangría para 'convert_closure_to_fn'](https://github.com/rust-lang/rust-analyzer/pull/20594)
* [corregir sangría para 'trait_impl_redundant_assoc_item'](https://github.com/rust-lang/rust-analyzer/pull/20681)
* [corrección no aplicable en 'struct' vacío para 'no_such_field'](https://github.com/rust-lang/rust-analyzer/pull/20614)
* [fijar otro predicado para 'replace_is_method_with_if_let_method'](https://github.com/rust-lang/rust-analyzer/pull/21787)
* [corregir la compensación de sangría de completación postfijo](https://github.com/rust-lang/rust-analyzer/pull/21324)
* [fijar tupla 'struct' pat tipo esperado](https://github.com/rust-lang/rust-analyzer/pull/21333)
* [añadir el calificativo 'ident_pat' al parámetro completamente fn](https://github.com/rust-lang/rust-analyzer/pull/21768)
* [no añadir un segundo punto y coma tras la finalización del posfijo](https://github.com/rust-lang/rust-analyzer/pull/21839)
* [completar la coincidencia de armas en la última coma y expr vacío](https://github.com/rust-lang/rust-analyzer/pull/21822)
* [corregir solapamiento editar en el registro a la tupla asistente usa auto](https://github.com/rust-lang/rust-analyzer/pull/21817)
* [comprobaciones incorrectas con múltiples espacios de trabajo](https://github.com/rust-lang/rust-analyzer/pull/21709)
* [oferta sobre const como path-expr para ''extract_variable''](https://github.com/rust-lang/rust-analyzer/pull/21809)
* [reemplazar los marcadores de TODO en el IrPrint del siguiente solucionador con un formato adecuado](https://github.com/rust-lang/rust-analyzer/pull/21779)
* [implementar inferencia de tipo de firma](https://github.com/rust-lang/rust-analyzer/pull/21823)
* [mejorar el nombre de la variable iteradora tmp para 'convert_for_to_while_let'](https://github.com/rust-lang/rust-analyzer/pull/20979)
* [migrar la asistencia de 'convert_from_to_tryfrom' a la API de SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21843)
* [mejoras de compatibilidad con el proyecto JSON](https://github.com/rust-lang/rust-analyzer/pull/21423)
* [mejoras de compatibilidad con el proyecto JSON](https://github.com/rust-lang/rust-analyzer/pull/21423)
* [eliminar comentarios del documento por 'generate_trait_from_impl'](https://github.com/rust-lang/rust-analyzer/pull/20407)
* [eliminar TAREA desactualizada](https://github.com/rust-lang/rust-analyzer/pull/21845)
* [eliminar el mapeo para 'expr_underscore' del constructor syntax factory](https://github.com/rust-lang/rust-analyzer/pull/21848)
* [reemplazar el uso directo de make por syntax factory y migrar la asistencia a syntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21847)
* [apoyan a WhileExpr y ForExpr para 'add_label_to_loop'](https://github.com/rust-lang/rust-analyzer/pull/20984)
* [soportar más tipos ejecutables en el proyecto JSON](https://github.com/rust-lang/rust-analyzer/pull/21424)

### Triaje de rendimiento del compilador Rust

Muchos resultados mixtos esta semana. Una gran regresión desde [#152931](https://github.com/rust-lang/rust/pull/152931) hace que los resultados parezcan bastante negativos, pero por lo demás la semana fue bastante tranquila.

Triaje hecho por **@panstromek**.
Rango de revisión: [5b61449e.. 6f22f613](https://perf.rust-lang.org/?start=5b61449ed85a670f1dd3fca6a8c759ee0b451b66&end=6f22f61305478df09f9a4523743f85d9f558c3d7&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 1,0% | [0,1%, 4,2%] | 27 |
| Regresiones ❌ <br /> (secundario) | 0,2% | [0,0%, 0,6%] | 36 |
| Mejoras ✅ <br /> (primaria) | -0,1% | [-0,2%, -0,1%] | 3 |
| Mejoras ✅ <br /> (secundario) | -0,3% | [-2,8%, -0,0%] | 14 |
| Todos ❌✅ (primario) | 0,9% | [-0,2%, 4,2%] | 30 |

1 regresión, 1 mejora, 4 mixta; Uno de ellos en rollups
32 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/75edcf889ba56f439f91a3c576388d9969dc5a16/triage/2026/2026-03-24.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [RFC: map_or_default en Opción y Resultado](https://github.com/rust-lang/rfcs/pull/3148)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Problema de seguimiento para fN::BITS](https://github.com/rust-lang/rust/issues/151073)
* [Retroceder '{float}' a 'f32' cuando 'f32: From<{float}>' y añadir 'impl From<f16> for f32'](https://github.com/rust-lang/rust/pull/139087)
* [Problema de seguimiento para tcp_deferaccept](https://github.com/rust-lang/rust/issues/119639)
* [Problema de seguimiento para #138068: Añadir 'Resultado::map_or_default' y 'Opción::map_or_default'](https://github.com/rust-lang/rust/issues/138099)
* [Fusionar 'fabsf16/32/64/128' en 'fabs:<F>:'](https://github.com/rust-lang/rust/pull/153834)
* [1.95 regresión beta: "entrada de atributo de característica malformada"](https://github.com/rust-lang/rust/issues/153764)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [dote: añadir opción de perfil de punteros de fotograma](https://github.com/rust-lang/cargo/pull/16742)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Emitir reetiquetas en el código](https://github.com/rust-lang/compiler-team/issues/958)
* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Garantizar alineación de primitivas enteras de ancho fijo](https://github.com/rust-lang/reference/pull/2205)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [Propone el fondo de mantenimiento de la Fundación Rust](https://github.com/rust-lang/rfcs/pull/3931)
* [¡Evita poner 'unreachable_code' en 'todo!' ()'](https://github.com/rust-lang/rfcs/pull/3928)

## Próximos eventos

Eventos Rusty entre 2026-03-25 - 2026-04-2026 🦀

### Virtual
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455925/)
* 31-03-2026 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 1**](https://www.meetup.com/code-mavens/events/313944077/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313656388/)
* 2026-04-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-04-09 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-15 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/313846195/)

### Asia
* 2026-03-28 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #13**](https://www.meetup.com/rustdelhi/events/313777790/)
* 2026-04-17 | Bangalore, IN, [Rust India](https://rustindia.org/)
    * [**Taller de Rust India**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN, [Rust India](https://rustindia.org/)
    * [**Conferencia Rust India**](https://rustindia.org/schedule)

### Europa
* 2026-03-25 | Dresde, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Primer encuentro**](https://github.com/rust-dresden/rust-dresden/discussions/7)
* 2026-03-26 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #66 patrocinado por Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/313833635/)
* 2026-03-26 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París**](https://www.rustinparis.com/)
* 2026-03-28 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #25**](https://www.meetup.com/stockholm-rust/events/313749232/)
* 2026-04-01 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/313783250/)
* 2026-04-01 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub vespertino)**](https://www.meetup.com/rust-and-friends/events/313898254/)
* 2026-04-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)
* 2026-04-02 | Londres, GB | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN habla de la Exposición Comunitaria de Primavera**](https://www.meetup.com/rust-london-user-group/events/313816694/)
* 2026-04-03 | Edimburgo, GB | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café de día)**](https://www.meetup.com/rust-and-friends/events/313898258/)
* 2026-04-07 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #15 @ letsboot**](https://www.meetup.com/rust-basel/events/313765547/)
* 2026-04-09 | Ginebra, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust habla @ AutoStore – "Patrones para sistemas impulsados por eventos" y "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**GUI nativas con Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)

### Norteamérica
* 2026-03-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Especial de Adopción de Activos Digitales de Rust NYC**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-03-28 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Chinatown, 28 de marzo**](https://www.meetup.com/bostonrust/events/313883686/)
* 2026-04-02 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313543900/)
* 2026-04-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler con LLM**](https://www.meetup.com/stl-rust/events/313482094/)
* 2026-04-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Winter Hill Rust, 4 de abril**](https://www.meetup.com/bostonrust/events/313883689/)
* 2026-04-09 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust April Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Brookline Rust, 11 de abril**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, EE. UU. [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Afinando tus habilidades de Rust para entrevistas de trabajo**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-16 | Seattle, WA, EE. UU. | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en la Plaza de Harvard, 18 de abril**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjcgbdc/)

### Oceanía
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Encuentro de marzo por determinar**](https://www.meetup.com/rust-melbourne/events/313471749/)

### Sudamérica
* 2026-04-11 | Argentina, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)
* 2026-04-17 | Río de Janeiro, BR | [Encuentros con Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Reunión Rust RJ**](https://luma.com/ce46pl7z)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> código no mejora de la nada solo porque lo reescribas en #rustlang. 

– [todos sobre mastodonte](https://mastodon.online/@alip/116275090869947511)

A pesar de que ha pasado una tercera semana sin ninguna sugerencia, llogiq es incansable en su búsqueda de encontrar un presupuesto que merezca la pena.

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1s3uoo0/this_week_in_rust_644/)</small>
