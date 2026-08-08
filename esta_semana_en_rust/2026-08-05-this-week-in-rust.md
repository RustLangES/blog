---
title: "Esta semana en Rust #121"
number_of_week: 121
description: El crate de esta semana es index_type, una caja para proporcionar índices fuertemente tipados para colecciones.
date: 2026-08-05
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
* [Activando la siguiente versión del comprobador de préstamos en cada noche](https://blog.rust-lang.org/2026/08/04/enabling-polonius-alpha-on-nightly/)
* [Actualización del progreso del equipo de financiación](https://blog.rust-lang.org/inside-rust/2026/08/04/funding-team-progress-update-july-2026/)
* [rust-lang/rust está adoptando una política LLM](https://blog.rust-lang.org/inside-rust/2026/08/05/rust-langrust-is-adopting-an-llm-policy/)
* [Retrospectiva de All Hands 2026](https://blog.rust-lang.org/inside-rust/2026/07/31/all-hands-2026-retrospective/)

### Boletines
* [El Rustacean Incrustado Número #77](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-77)

### Actualizaciones de proyectos/herramientas
* [Kevat 0.4.0 — copia rápida y reanudábrica y traslado a discos externos, ahora con interfaz gráfica en las tres plataformas](https://kevat.app/)
* [kache 0.13.0: claves de los vars env proc-macros read](https://github.com/kunobi-ninja/kache/releases/tag/v0.13.0)
* [kobe 101: alquila un clúster Kubernetes, no crees uno](https://kunobi.ninja/blog/kobe-101-leasing-kubernetes-clusters)
* [Reescribiendo FalkorDB en Rust: Haz que funcione, haz que sea estable](https://www.falkordb.com/blog/rewriting-falkordb-in-rust/)
* [Anunciando 'webrtc' v0.20.0: WebRTC asíncrono, agnóstico en tiempo de ejecución en Sans-I/O Core 'rtc'](https://webrtc.rs/blog/2026/07/31/announcing-webrtc-v0.20.0.html)
* [Proxelar 0.5.0: sesiones, reglas y más formas de capturar tráfico](https://micheletti.io/proxelar-050/)
* [Mirador 1.0.0: Un Panel Personal de Terminal](https://github.com/jchultarsky/mirador/releases/tag/v1.0.0)
* [BitFun 0.2.15: un agente de IA de escritorio de código abierto construido sobre un runtime de Rust](https://github.com/GCWing/BitFun/releases/tag/v0.2.15)
* [mvis v0.5.0: Perfilado CI/CD e Histogramas de Asignación](https://dev.to/sicklefire/mvis-v050-new-release-5997)
* [Multicalc 0.9.0: Computación científica para sistemas embebidos y robóticos](https://github.com/kmolan/multicalc-rust/releases/tag/v0.9.0)
* [Corregir automáticamente una maquetada de diseño en Wayland es casi imposible. Lo hicimos de todas formas](https://poltertype.com/blog/wrong-layout-typing-on-wayland/)
* [wimux 0.1.0: multiplexor nativo de terminales de Windows](https://github.com/fabperso/wimux/releases/tag/v0.1.0)
* [AMTR: un monitor de ventana contextual estilo BTOP para sesiones de Claude Code y la autopsia forense de su propia construcción de 153 horas](https://github.com/arian-shamaei/anthropometer/tree/main/docs/autopsy)
* [versión de RSigma v0.20.0](https://github.com/timescale/rsigma/releases/tag/v0.20.0)
* [El Estado de RSigma](https://mostafa.dev/the-state-of-rsigma-7ba0a99020d9), y [Parte Dos: El Bucle](https://mostafa.dev/the-state-of-rsigma-part-two-the-loop-c114f379dd78)

### Observaciones/Pensamientos
* [Cómo funcionan las microVMs de Firecracker en el interior para poner en sandbox código no confiable y agentes de IA](https://kerkour.com/firecracker-sandboxing-rust)
* [Matemáticas de coma flotante más rápidas con la nueva API de Rust](https://pythonspeed.com/articles/faster-float-math-rust/)
* [Controladores Rust MEMS: 3 razones para intentar adoptar nuestro nuevo controlador de sensores](https://blog.st.com/rust-mems-drivers/)
* [La base del diseño de software | Alex Fedoseev](https://alex.draftist.io/blog/the-bedrock-of-software-design-ycqvcedsj)
* [Intérpretes de Llamadas de Cola en Rust](https://lordgoati.us/blog/tail-call/)
* [Cómo acelerar el compilador de Rust en julio de 2026](https://nnethercote.github.io/2026/07/31/how-to-speed-up-the-rust-compiler-in-july-2026.html)
* [Beca Sovereign Tech para mantenimiento de Rust (informe junio-julio de 2026)](https://kobzol.github.io/rust/2026/08/03/stf-june-july-2026.html)
* [Una visión antigua-nueva sobre el análisis de argumentos en Rust](https://jmmv.dev/2026/07/hello-getoptsargs.html)
* [Servidores sin estado, cargas útiles con estado: Sesiones vs Continuaciones, medidas en Rust](https://dmitrii.app/stateless-servers-stateful-payloads-sessions-vs-continuations-measured-in-rust/)
* [vídeo] [Rust en la era de la IA generativa con Niko, Allen y Zeeshan](https://www.youtube.com/watch?v=2937MGszrak)
* [audio] [Rust in Production S06 E09: JetBrains con Orhun Parmaksız](https://corrode.dev/podcast/s06e09-jetbrains/)
* [Robo de trabajo vs. Ejecutor por hilo: Evaluación de diferentes cargas de trabajo de servidores HTTP con Tokio, Smol y Glommio](https://c410-f3r.github.io/thoughts/work-stealing-vs-executor-per-thread-evaluating-different-http-server-workloads-with-tokio-smol-and-glommio/)
* [Tu '#[target_feature(enable = "avx2")]' no hace nada en 'x86_64-unknown-uefi'](https://github.com/Aefinity-AI/alice-aegis/blob/main/docs/posts/2026-08-05_uefi-soft-float-deletes-your-avx2.md)
* [Tres fallos que mis agentes de IA no pudieron arreglar](https://dev.to/fabperso/three-bugs-my-ai-agents-couldn't-fix-13bn)

### Guías de Rust
* [Parpadeando un LED en la Píldora Azul STM32 (STM32F103C8T6) con Rust incrustado](https://blog.implrust.com/posts/2026/08/blinky-with-stm32f103c8t6-embedded-rust/)
* [Rust sin ramas: Hacer un filtro 4 veces más rápido eliminando un 'si'](https://www.greyblake.com/blog/branchless-rust/)
* [Por qué las métricas modernas de fuentes no pueden reproducir la paginación de Word](https://oxi-dd65f4.gitlab.io/articles/word-pagination-gdi-rounding.html)
* [Construiendo un hooklog sobre una estructura de seis días](https://github.com/JuanMarchetto/hooklog/blob/main/ARTICLE.md)

## Crate de la semana

El crate de esta semana es [index_type](https://crates.io/crates/index_type), una caja para proporcionar índices fuertemente tipados para colecciones.

¡Gracias a [Roee Shoshani](https://users.rust-lang.org/t/crate-of-the-week/2704/1638) por la autosugerencia!

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
* [Cordial - Unificar las dos implementaciones del bloqueo de perfil](https://github.com/luohoa97/cordial/issues/6)
* [Cordial - Clips a pantalla completa y buzones de letras hasta que el espacio de trabajo se cambia y vuelve a aparecer](https://github.com/luohoa97/cordial/issues/7)
* [Dofigen - Extender archivos Docker](https://github.com/lenra-io/dofigen/issues/481)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

630 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-28..2026-08-04

#### Compilador
* [mejora el desplazamiento del CFG](https://github.com/rust-lang/rust/pull/160193)
* [perf: evitar una asignación de heap por bloque básico en los mapas de ubicación de MoveData](https://github.com/rust-lang/rust/pull/160245)
* [estabilizar enteros de 128 bits pasando mediante registros vectoriales con 'asm!' en x86](https://github.com/rust-lang/rust/pull/159525)

#### Biblioteca
* [a bit optimizar bloques de cuatro dígitos en formato entero](https://github.com/rust-lang/rust/pull/159130)
* [añadir soporte NEON para 'is_ascii' y 'eq_ignore_ascii_case'](https://github.com/rust-lang/rust/pull/160143)
* [add semver check test command for check check for check compatibility API of stdlib](https://github.com/rust-lang/rust/pull/159671)
* [permite solo implementar 'Leer::read_buf'](https://github.com/rust-lang/rust/pull/106643)
* [núcleo: implementar muestreo aleatorio acotado](https://github.com/rust-lang/rust/pull/159592)
* [iter: especializa 'Take::count' usando 'advance_by'](https://github.com/rust-lang/rust/pull/160139)
* [iter: especializar método 'advance_by' de 'Fuse'](https://github.com/rust-lang/rust/pull/160342)
* [hacer que las operaciones atómicas constan](https://github.com/rust-lang/rust/pull/160079)
* [mover 'std::io::copy' a 'alloc::io'](https://github.com/rust-lang/rust/pull/158548)
* [estabilizar 'size_of_val_raw, align_of_val_raw, Layout::for_value_raw'](https://github.com/rust-lang/rust/pull/157572)

#### Carga
* [añadir una sugerencia al añadir '[lints]' a un espacio de trabajo para usar '[workspace.lints]' en su lugar](https://github.com/rust-lang/cargo/pull/17300)
* [evitar analizar archivos de bloqueo sin cambiar](https://github.com/rust-lang/cargo/pull/17301)
* [completaciones: rutas completas para argumentos de carrera de carga](https://github.com/rust-lang/cargo/pull/17284)
* [corregir la pelusa 'manual_readme' para archivos README de menor prioridad](https://github.com/rust-lang/cargo/pull/17208)
* [git: hacer que los nombres de salida sean independientes de git config](https://github.com/rust-lang/cargo/pull/17289)
* [hacer que '__CARGO_TEST_FORCE_ARGFILE' esté disponible en compilaciones distribuidas](https://github.com/rust-lang/cargo/pull/17293)
* [pasan las banderas de rustdoc al paso final de la fusión CCI](https://github.com/rust-lang/cargo/pull/17269)
* [evitar el pánico cuando 'package.build' esté vacío](https://github.com/rust-lang/cargo/pull/17268)
* [reestructurado cómo activamos el nuevo diseño de directores de construcción en Nightly](https://github.com/rust-lang/cargo/pull/17272)
* [trim-paths: reglas de remapeo inequívocas y reversibles](https://github.com/rust-lang/cargo/pull/17302)

#### Rustdoc
* [insignia de etiqueta para rasgos notables](https://github.com/rust-lang/rust/pull/157058)
* [rustdoc-json: hacer que 'Stability' sea compatible con formatos serde que no se describen a sí mismos](https://github.com/rust-lang/rust/pull/160032)
* [fijar ICE cuando un clúster de grafemas une un carácter de clase Prepend a '_' o ':'](https://github.com/rust-lang/rust/pull/160232)
* [corregir el fallo al intentar listar atributos en un tipo opaco](https://github.com/rust-lang/rust/pull/160208)
* [solo analizar el tipo de cabeza del propio al decidir impl inlining](https://github.com/rust-lang/rust/pull/159854)

#### Rustfmt
* [formato 'cfg_select!'](https://github.com/rust-lang/rust/pull/154202)

#### Clippy
* ['manual_div_ceil': evitar sugerencias que cambien el recuento de evaluaciones](https://github.com/rust-lang/rust-clippy/pull/17468)
* [corregir 'no_effect_underscore_binding' falso positivo en código generado por macro proc](https://github.com/rust-lang/rust-clippy/pull/17473)
* [añadir comprobación de imagen con enlace incrustado a 'doc_paragraphs_missing_punctuation'](https://github.com/rust-lang/rust-clippy/pull/16773)
* [pelusa para UFCS en 'clone_on_copy'](https://github.com/rust-lang/rust-clippy/pull/16972)
* [disparo 'float_cmp_const' para 'assert_eq!' con flotadores de const](https://github.com/rust-lang/rust-clippy/pull/17024)

#### Analizador de Rust
* [permitir el 'yo' como el último segmento de un camino](https://github.com/rust-lang/rust-analyzer/pull/23014)
* [manejar correctamente los casos límite de módulos no vinculados](https://github.com/rust-lang/rust-analyzer/pull/22977)
* [soporte 'CovariantUnsafeCell'](https://github.com/rust-lang/rust-analyzer/pull/22959)
* [añadir '-Zjson-target-spec' en las llamadas de carga donde sea necesario](https://github.com/rust-lang/rust-analyzer/pull/21846)
* [añadir referencia para el mismo nombre Param Coerce Matches](https://github.com/rust-lang/rust-analyzer/pull/23003)
* [permitir derechos divergentes en asignaciones de desestructuración](https://github.com/rust-lang/rust-analyzer/pull/23017)
* [evitar el pánico al marcar 'Copiar' para argumentos de cierre de hrtb](https://github.com/rust-lang/rust-analyzer/pull/22938)
* [detectar el componente analizador de Rust en un arreglo de componentes multilínea](https://github.com/rust-lang/rust-analyzer/pull/22996)
* [no alloc anon consts para caminos desnudos en bloques](https://github.com/rust-lang/rust-analyzer/pull/22965)
* [no te pongas nervioso con una función autorreferencial de 'Rasgo impl'](https://github.com/rust-lang/rust-analyzer/pull/22992)
* [doble tamaño de pila para hilos hasta 16MiB](https://github.com/rust-lang/rust-analyzer/pull/22956)
* [excluir tipos desconocidos de la búsqueda de términos](https://github.com/rust-lang/rust-analyzer/pull/23015)
* [búsqueda de corrección 'MACRO_CALL@...' en esta Semántica debe incluir!](https://github.com/rust-lang/rust-analyzer/pull/22933)
* [corregir la gestión de 'ExprScopes' de exprs dentro de patrones](https://github.com/rust-lang/rust-analyzer/pull/23008)
* [corregir error de shadowing de importación globo](https://github.com/rust-lang/rust-analyzer/pull/22886)
* [hacer que la ejecución de depuración de MIR funcione para los elementos de Bitflags](https://github.com/rust-lang/rust-analyzer/pull/22948)
* [marcar rasgos automáticos como coincidentes](https://github.com/rust-lang/rust-analyzer/pull/22943)
* [sin pista con nombre similar raw-ident arg](https://github.com/rust-lang/rust-analyzer/pull/22957)
* [analizar el rango de postfijo dentro del cierre en el acceso](https://github.com/rust-lang/rust-analyzer/pull/23004)
* [reconocer argumentos de formato tras una barra diagonal en cadenas sin procesar](https://github.com/rust-lang/rust-analyzer/pull/22993)
* [resolver asignación LHS en su ámbito de expresión](https://github.com/rust-lang/rust-analyzer/pull/23016)
* [mostrar rutas calificadas cuando los nombres de tipo chocan en E0308](https://github.com/rust-lang/rust-analyzer/pull/22964)
* [hir-ty, diagnósticos de ide: usar E0057/E0061 para la discrepancia de arg-count (antes E0107)](https://github.com/rust-lang/rust-analyzer/pull/22947)
* [perf: evita tener una consulta separada para los opacos definidos](https://github.com/rust-lang/rust-analyzer/pull/22966)
* [PERF: Guardar una asignación en el manejo de por vida](https://github.com/rust-lang/rust-analyzer/pull/23001)
* [reportar un error de configuración para fragmentos de postfijo con el alcance del ítem](https://github.com/rust-lang/rust-analyzer/pull/22937)
* ['VFS': usar coincidencia de prefijos de camino basada en componentes para rutas virtuales](https://github.com/rust-lang/rust-analyzer/pull/22940)

### Triaje de rendimiento del compilador Rust
Esta semana han llegado muchas optimizaciones. Algunas grandes mejoras en rustdoc en [#159854](https://github.com/rust-lang/rust/pull/159854), una gran mejora en el recorrido de grafos de flujo de control para 'cranelift-codegen', algunas mejoras más en benchmarks next-solver y varias otras microoptimizaciones, llevando el total a un buen número de 10 mejoras esta semana.

Triaje hecho por **@panstromek**.
Rango de revisión: [ad0c9dce.. 65dd30fb](https://perf.rust-lang.org/?start=ad0c9dce27a22416b65946bc0010edaf22ac6c83&end=65dd30fb9e882a7e8f0be10caca62936db2a98b8&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,2%, 0,5%] | 18 |
| Regresiones ❌ <br /> (secundario) | 2,1% | [0,1%, 16,8%] | 64 |
| Mejoras ✅ <br /> (primaria) | -3,3% | [-39,8%, -0,2%] | 97 |
| Mejoras ✅ <br /> (secundario) | -6,1% | [-39,6%, -0,1%] | 111 |
| Todos ❌✅ (primario) | -2,7% | [-39,8%, 0,5%] | 115 |

1 regresión, 5 mejoras, 11 mixtas; 6 de ellos en rollups
32 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/c41ca2a96f74761503b333d9f416eb7012eef858/triage/2026/2026-08-03.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Problema de seguimiento para 'core_io_borrowed_buf'](https://github.com/rust-lang/rust/issues/117693)
* [Problema de seguimiento para 'derive_macro_global_path'](https://github.com/rust-lang/rust/issues/154645)
* [estabilizar 'c_variadic_naked_functions'](https://github.com/rust-lang/rust/pull/159746)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Implementar una convención de nombres para 'rustc_' solo de lint/diagnóstico 'attrs](https://github.com/rust-lang/compiler-team/issues/1021)
* [Codificar OpenBSD versión '-actual' en la versión 'target_env'](https://github.com/rust-lang/compiler-team/issues/1018)
* [Añadir 'target_feature_available_at_call_site'](https://github.com/rust-lang/compiler-team/issues/1010)
* [Promocionar 'wasm32-wasip3' a Nivel 2](https://github.com/rust-lang/compiler-team/issues/1001)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 05-08-2026 y el 02-09-2026 🦀

### Virtual
* 2026-08-05 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Ejecución y Programación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315880365/)
* 2026-08-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-07 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/ii2jrwva)
* 2026-08-10 | Híbrido (Kuala Lumpur, Malasia) | [Reunión de Rust Malaysia](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup agosto 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfwGMGqDit9jn9INA1EROWTbvnjTAZAO1oUQaEwqmao7AYy1A/viewform)
* 2026-08-11 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-13 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-08-18 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/315604176/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #5 — Comunicación inalámbrica con el protocolo IEEE 802.15.4**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254775/)
* 27-08-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxid**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-21 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/arkkrcj5)
* 2026-09-02 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)

### África
* 2026-08-11 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-10 | Híbrido (Kuala Lumpur, MY) | [Reunión de Rust Malaysia](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup agosto 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfwGMGqDit9jn9INA1EROWTbvnjTAZAO1oUQaEwqmao7AYy1A/viewform)
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de agosto 2026**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Encuentro de Rust Delhi X SciPy India**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Computación científica en Rust y pitón**](https://scipy.in/sci-py-rs/)
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: agosto 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europa
* 2026-08-05 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in August: ¡No te asustes! ... or_else?**](https://www.meetup.com/rustcologne/events/315910506/)
* 2026-08-06 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin en localización 🏳️ 🌈 - Edición 016**](https://www.meetup.com/rust-berlin/events/315966137/)
* 2026-08-06 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Social ACCU/Rust Summer**](https://www.meetup.com/oxford-rust-meetup-group/events/315863373/)
* 2026-08-13 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Confía pero verifica el LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por definir**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Construcción de una cámara acústica con egui y embajada**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 27-08-2026 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)

### Norteamérica
* 2026-08-06 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315590399/)
* 2026-08-06 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Envío temporal: Cómo un ecosistema global de Rust construyó la última API web de Chrome**](https://www.meetup.com/stl-rust/events/314701905/)
* 2026-08-11 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: 'Una introducción a wgpu' y '¡Hablemos de genéricos!' **](https://www.meetup.com/rust-nyc/events/315963710/)
* 2026-08-13 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de agosto de Utah Rust**](https://www.meetup.com/utah-rust/events/314696652/)
* 2026-08-13 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust August Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601099/)
* 2026-08-15 | San Francisco, CA, EE. UU. [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 3**](https://partiful.com/e/juWAwRs3XMWP7s9wLNWK)
* 2026-08-18 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997215/)
* 2026-08-19 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-19 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Agosto de Rust en el Área de la Bahía**](https://luma.com/00f2s7q9)
* 2026-08-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-26 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust LA August! Rust en la computación cuántica**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 27-08-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539331/)

### Oceanía
* 27-08-2026 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne agosto 2026**](https://www.meetup.com/rust-melbourne/events/315039490/)

### Sudamérica
* 2026-08-08 | São Paulo, SP | [Rust-SP](https://luma.com/calendar/cal-bif2oHITU1aVvsr)
    * [**Rust SP - Ago/2026**](https://luma.com/41oiyhtk)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> … pero abandoné la idea porque las reglas de macros se estaban convirtiendo en un analizador sintáctico completo de Turing Rust

– [Koosha sobre usuarios de Rust](https://users.rust-lang.org/t/crate-of-the-week/2704/1637)

¡Gracias a [miro](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1787) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1vgv7sn/this_week_in_rust_663)</small>
