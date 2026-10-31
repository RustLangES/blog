---
title: "Esta semana en Rust #131"
number_of_week: 131
description: El crate de esta semana es offline-protocol, una caja que proporciona redes malladas/descubrimiento de pares para diversas redes, bajo licencia AGPL/propietaria. 
date: 2026-10-28
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

* [Resultados de la Microencuesta Genérica Variádica](https://blog.rust-lang.org/inside-rust/2026/10/27/variadic-generics-micro-survey-results/)
* [Cómo financiar a los colaboradores de Rust](https://blog.rust-lang.org/2026/10/26/how-to-fund-rust-contributors/)

### Boletines

* [El Rustacean Incrustado Número #83](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-83)

### Actualizaciones de proyectos/herramientas

* [Resumen del proyecto Ring Buffers - Redox - Tu próximo sistema operativo (https://www.redox-os.org/news/nlnet-rb-summary/)
* [tar-codec: un analizador de alquitrán rápido y seguro para el ecosistema Rust](https://astral.sh/blog/tar-codec)
<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [Oxidar sin atajos: ¿Qué cuesta realmente la seguridad](https://ieeexplore.ieee.org/abstract/document/11723159)
* [En qué establo se sostiene Rust](https://kivooeo.github.io/blog/what-stable-rust-stands-on/)
* [Donde va una inundación](https://shieldify.ee/research/where-a-flood-goes/)
* [Discutiendo una reescritura de XArray](https://lwn.net/SubscriberLink/1100520/2e0f0241deef5804/)
* [Rust aparece en el futuro](https://lwn.net/SubscriberLink/1099606/0fe2b8d731fd167f/)
* [Directrices para pánico en el código del kernel](https://lwn.net/SubscriberLink/1100873/0934a9a322c6df4c/)
* [¿Qué tienen en común los sistemas distribuidos y las aplicaciones cifradas de extremo a extremo? ¡CRDTs y Rust!](https://kerkour.com/rust-crdt-love)
* [vídeo] [P99 CONF 2026 | Creando transmisión de mensajes ultra eficientes por Piotr Gankiewicz](https://www.youtube.com/watch?v=2j6BzKaODUk)

### Guías de Rust

* [La calefacción está encendida: unidades de manejo en Rust](https://bitfieldconsulting.com/posts/heat-is-on)
* [Encontrando pánicos con el Analizador de Grafos de Llamadas](https://tweedegolf.nl/en/blog/242/finding-panics-with-call-graph-analyzer/)
* [vídeo] [Más allá de la codificación Vibe: Verificación formal para software generado por IA en Python y Rust](https://www.youtube.com/watch?v=P9xCUmjkPUM)
* [vídeo] [Rust y la GPU de Native a Web: Introducción a 'wgpu' por Liam Rust](https://www.youtube.com/watch?v=LnxuQKoxvFM)

### Miscelánea

* [uz] [Anunciando Rust Uzbekistan y la traducción oficial al uzbeko de El Libro de Rust](https://doc.ferris.uz/)

## Crate de la semana

El crate de esta semana es [offline-protocol](https://crates.io/crates/offline-protocol), una caja que proporciona redes malladas/descubrimiento de pares para diversas redes, bajo licencia AGPL/propietaria. 

¡Gracias a [Gokul](https://users.rust-lang.org/t/crate-of-the-week/2704/1702) por la autosugerencia! 

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

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
* [sysknife - SetSshdOption informa de éxito mientras que un drop-in anterior de sshd_config.d anula el valor que ha escrito](https://github.com/lacs-project/sysknife/issues/559)
* [sysknife - En Postgres el daemon ancla la cadena de auditoría pero todos los lectores reportan que el ancla no está soportada y sale 2](https://github.com/lacs-project/sysknife/issues/558)
* [sysknife - ci-local.test.sh permanece verde cuando se revierte la corrección de salida #555, la herramienta por defecto o la bandera de exclusión de la opción de excluirse](https://github.com/lacs-project/sysknife/issues/591)
* [Tabularis - [Dote]: Importar conexiones desde pgAdmin 4](https://github.com/TabularisDB/tabularis/issues/1309)
* [Keepword - Indica cuántas redes han observado un nodo que aún no está localizado](https://git.keepword.net/keepword/keepword/issues/49)
* [Octomind - 'octomind workflow docs' falla con "Es un directorio" cuando existe './docs/' (https://github.com/Muvon/octomind/issues/242)
* [Octomind - Bucle de flujo de trabajo con 'max_iterations = 0' pasa la validación y nunca se ejecuta](https://github.com/Muvon/octomind/issues/241)
* [Octomind - ACP: tipo de herramienta de envío, ubicaciones de archivos y diferencias con llamadas a herramienta](https://github.com/Muvon/octomind/issues/166)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

609 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-20..2026-10-27

#### Compilador
* [cuenta la ausencia de turbofish en los errores de resolución](https://github.com/rust-lang/rust/pull/159689)
* [añadir varias anotaciones '#[inline]' para funciones simples en caminos calientes que no se inlinean](https://github.com/rust-lang/rust/pull/163845)
* [deduplicar codificaciones DefId en la caché de consultas](https://github.com/rust-lang/rust/pull/164300)
* ['#[inline(siempre)] + #[target_feature(habilitar = "....")]' #2](https://github.com/rust-lang/rust/pull/162460)
* [MIR eliminación de movimiento \[4/6\]: Eliminación de movimiento](https://github.com/rust-lang/rust/pull/163338)
* ['rustc_builtin_macros' limpieza, parte 8](https://github.com/rust-lang/rust/pull/163485)
* [optimizar las operaciones de Span, especialmente el caliente 'Span::to'](https://github.com/rust-lang/rust/pull/163971)
* [saltar comprobaciones de monomorfización redundantes](https://github.com/rust-lang/rust/pull/164330)
* [usar el espacio de la valla de bloque de código doctest para calcular el número de línea](https://github.com/rust-lang/rust/pull/164849)

#### Biblioteca
* [const-stabilize {'BinaryHeap', 'BTreeMap', 'BTreeSet', 'LinkedList', 'VecDeque')':'{'len', 'is_empty'}](https://github.com/rust-lang/rust/pull/164643)
* [núcleo: implementa 'char::is_xid_'{'start', 'continue'}](https://github.com/rust-lang/rust/pull/164474)
* [no intentes saltarte nada en 'Saltar::advance_by(0)'](https://github.com/rust-lang/rust/pull/164488)
* [implementa 'AsMut<T>' y 'AsRef<T>' para '!'](https://github.com/rust-lang/rust/pull/161253)
* [mejorar el código de permutación de bits](https://github.com/rust-lang/rust/pull/164611)
* [proceso: mejorar la salida de 'Debug' para 'ExitStatus' en Unix](https://github.com/rust-lang/rust/pull/163510)
* [estabilizar ExitCodeExt para Windows](https://github.com/rust-lang/rust/pull/164656)
* [estabilizar asignadores para 'Vec::into_iter'](https://github.com/rust-lang/rust/pull/163499)
* [estabilizar 'Cuerda::into_chars'](https://github.com/rust-lang/rust/pull/163816)
* [estabilizar 'ptr_cast_slice'](https://github.com/rust-lang/rust/pull/162927)
* [estabilizar 'slice_split_once'](https://github.com/rust-lang/rust/pull/164641)
* [estabilizar {Rc, 'Arc}::is_unique'](https://github.com/rust-lang/rust/pull/163458)

#### Carga
* ['tiempos': memoria máxima como columna en la tabla de unidades](https://github.com/rust-lang/cargo/pull/17631)
* [deja de animar los rlibs en cada noche](https://github.com/rust-lang/cargo/pull/17621)
* [excluir abspath de deps de path del hash '-Cmetadata'](https://github.com/rust-lang/cargo/pull/17624)
* [prueba: simplificar afirmaciones de contenido de archivo](https://github.com/rust-lang/cargo/pull/17617)
* [usar build-rs para buildscripts](https://github.com/rust-lang/cargo/pull/17637)

#### Rustdoc
* [incluir atributos 'obsoletos' en la resolución de enlace intra-doc 'rustc_resolve'](https://github.com/rust-lang/rust/pull/164733)
* [manejar correctamente el atributo interno 'obsoleto' en módulos](https://github.com/rust-lang/rust/pull/164594)
* [no te pongas nervioso en un grupo de uso anidado sin prefijo de ruta](https://github.com/rust-lang/rust/pull/164883)

#### Rustfmt
* [evitar más pérdida de comentarios en 'cfg_select!'](https://github.com/rust-lang/rust/pull/164815)
* [no pierdas comentarios ni atributos entre los predicados 'cfg_select!', bloqueos externos/internos, cuerpo del brazo y comas finales](https://github.com/rust-lang/rustfmt/pull/7211)
* [se filtra el recuento de errores 'cfg_if!' (https://github.com/rust-lang/rustfmt/pull/7195)

#### Clippy
* ['wildcard_imports': ya no se produce pelusa y activa automáticamente las expansiones de macros](https://github.com/rust-lang/rust-clippy/pull/17898)
* [añadir pelusa de 'ifs_as_logical_ops'](https://github.com/rust-lang/rust-clippy/pull/14904)
* [añadir pelusas para 'map_or_else' a 'map_or'](https://github.com/rust-lang/rust-clippy/pull/17847)
* [corregir falsos positivos 'missing_const_for_fn' en 'const impl'](https://github.com/rust-lang/rust-clippy/pull/17892)
* [la sugerencia de corregir 'double_comparisons' elimina los paréntesis obligatorios](https://github.com/rust-lang/rust-clippy/pull/17907)
* [PERF: optimizar pelusa de 'unconditional_recursion'](https://github.com/rust-lang/rust-clippy/pull/17835)

#### Analizador de Rust
* [sincronización automatizada de implicaciones de características objetivo](https://github.com/rust-lang/rust-analyzer/pull/23484)
* [arreglar problemas con NamedTempFile en Windows](https://github.com/rust-lang/rust-analyzer/pull/23424)
* [módulo 'principal' 'lib' completo en el submódulo](https://github.com/rust-lang/rust-analyzer/pull/23529)
* [calcular ref-match en el propio sin punto](https://github.com/rust-lang/rust-analyzer/pull/23489)
* [No te asustes cuando se use el param de llamada en línea en macro](https://github.com/rust-lang/rust-analyzer/pull/23231)
* [error de lógica 'fetch_semantic' introducido en #22272](https://github.com/rust-lang/rust-analyzer/pull/23534)
* [mensaje de estado de corrección para clientes que no soportan 'experimental/serverStatusNotification'](https://github.com/rust-lang/rust-analyzer/pull/23552)
* [usar el camino absoluto en 'format_smolstr'](https://github.com/rust-lang/rust-analyzer/pull/23540)
* [buscar antepasados del manifiesto para el archivo de bloqueo al obtener metadatos de carga](https://github.com/rust-lang/rust-analyzer/pull/23296)

### Triaje de rendimiento del compilador Rust

Otra semana de cambios positivos. Algunas mejoras inesperadas de las limpiezas y un puñado de optimizaciones específicas. 
Además de las mejoras en el recuento de instrucciones, también hemos fusionado una gran reducción del tamaño de la caché de consulta en [#164300](https://github.com/rust-lang/rust/pull/164300). 
Los cambios negativos provienen principalmente de una regresión inesperada del 'doc', que actualmente está siendo investigada. 

Triaje hecho por **@panstromek**. 
Rango de revisión: [bd4c6cc4.. 019b59d8](https://perf.rust-lang.org/?start=bd4c6cc4aeef0eb9919c6f73c1e690677e9c6b38&end=019b59d8101bb3e141f1b880684747b4217beeb6&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,8% | [0,2%, 6,8%] | 25 |
| Regresiones ❌ <br /> (secundaria) | 6,8% | [0,0%, 189,1%] | 34 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-12,6%, -0,1%] | 195 |
| Mejoras ✅ <br /> (secundario) | -1,2% | [-10,6%, -0,1%] | 157 |
| Todos ❌✅ (primario) | -0,6% | [-12,6%, 6,8%] | 220 |


3 regresiones, 6 mejoras, 3 mixtas
32 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/5080585bb0dc997cd8aeca17eff036e0c5ddf713/triage/2026/2026-10-26.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/main)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* [2052-epochs.md: error tipográfico](https://github.com/rust-lang/rfcs/pull/4024)
* [3722-explicit-extern-abis.md: error tipográfico](https://github.com/rust-lang/rfcs/pull/4023)
* [Arreglar los libs nominados triagebot.toml](https://github.com/rust-lang/rfcs/pull/4022)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Añadir pelusa de 'let_underscore_drop_guard'.](https://github.com/rust-lang/rust/pull/164414)
* [Desestimar función interna 'vec::from_elem'](https://github.com/rust-lang/rust/pull/164653)
* [núcleo: Añadir 'assert_matches' y 'debug_assert_matches' al siguiente preludio](https://github.com/rust-lang/rust/pull/164328)
* [aclarar cuándo 'Vec::into_boxed_slice()' está en su lugar](https://github.com/rust-lang/rust/pull/164258)
* [Estabilizar parcialmente 'ptr_alignment_type' como 'alignment_type'](https://github.com/rust-lang/rust/pull/153261)
* [estándar: haz que 'Instantáneo' sea un 'Duración' firmado en todas las plataformas](https://github.com/rust-lang/rust/pull/158368)
* [Más abajo #[no_mangle] a #[symbol_name]/#[link_name]](https://github.com/rust-lang/rust/pull/159374)
* [revisión del equipo Lang para un nuevo solucionador de rasgos](https://github.com/rust-lang/rust/issues/164701)
* [Implementar rasgos 'Fn' para funciones seguras de cualquier ABI](https://github.com/rust-lang/rust/pull/160186)
* [CMSE: pelusa en uniones que cruzan la frontera segura](https://github.com/rust-lang/rust/pull/147697)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Añadir 'objetivos-requeridos' para la selección de paquetes en el espacio de trabajo](https://github.com/rust-lang/rfcs/pull/4013)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Añadir soporte para el formato de salida 'no-graphics' de 'annotate-snippets'](https://github.com/rust-lang/compiler-team/issues/1045)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.* 


## Próximos eventos

Eventos Rusty entre el 28-10-2026 - el 25-11-2026 🦀

### Virtual
* 2026-10-28 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Club de Lectura de Sistemas Operativos: Mecanismos de Paginación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creando un editor hexadecimal en Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773682/)
* 04-11-2026 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 05-11-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackeo y Aprendiza**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254770/)
* 2026-11-12 | Virtual (Núremberg, DE) | [Núremberg Rust](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust Maldito**](https://luma.com/omlooqv8)
* 2026-11-15 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Compilación errónea en Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: El arnés llevaba gabardina - Construyendo un servidor MCP de Rust**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcpbwb/)
* 2026-11-18 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Escribiendo un firmware para lector Xteink en Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254769/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Rust en persona noviembre de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**Encuentro de Rustacean de noviembre de 2026](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europa
* 2026-10-28 | Copenhague, DK | [Comunidad Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #72 patrocinado por Google!**](https://www.meetup.com/copenhagen-rust-community/events/316545936/)
* 2026-10-29 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin en localizaciones 🏳️ 🌈 – Edición 019**](https://www.meetup.com/rust-berlin/events/317130457/)
* 2026-10-31 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 01-11-2026 - 3 de 11-2026 | Italia, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - La Conferencia Internacional sobre el Rust en Italia**](https://rustlab.it/schedule)
* 2026-11-02 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/317103436/)
* 2026-11-04 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Empezando con el nuevo controlador Rust para la base de datos de IA de Oracle**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milán, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Trenes Modelo Digitales & Rust: Estación de Mando DCC de Metal Desnudo en ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 05-11-2026 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Epítome de excelencia sináptica**](https://www.meetup.com/oxford-rust-meetup-group/events/317158298/)
* 2026-11-12 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milán, MI, IT | [Milán en idioma oxidado](https://www.meetup.com/rust-language-milano/events/)
    * [**De la curiosidad a la capacidad: Crea tu propio enjambre de dispositivos**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Todo es Git, los protocolos son solo adaptadores**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de Charla en EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)

### Norteamérica
* 2026-10-28 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316655403/)
* 31-10-2026 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en el North End, 31 de octubre**](https://www.meetup.com/bostonrust/events/316924288/)
* 05-11-2026 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Social mensual de noviembre**](https://www.meetup.com/rust-montreal/events/316919004/)
* 05-11-2026 | San Francisco, CA | [Rust LA: NVIDIA](https://luma.com/user/usr-eWgtha6CHSTma7v/)
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://luma.com/iqe7wng1)
* 05-11-2026 | Saint Louis, MO, EE.UU. [Rust STL](https://www.meetup.com/stl-rust)
    * [**HTMX y Rust**](https://www.meetup.com/stl-rust/events/316896797/)
* 2026-11-12 | San Diego, CA, EE.UU. | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, 14 de noviembre**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Ángeles, CA, EE.UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Rust LA November con Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Escribiendo un firmware para lector Xteink en Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-21 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Winter Hill Rust, 21 de noviembre**](https://www.meetup.com/bostonrust/events/316924366/)

### Sudamérica
* 2026-11-07 | Planalto Paulista, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

### Oceanía: 
* 2026-11-24 | Barton, AU | [Grupo de usuarios Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/316907340/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1wzctie/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> llevamos 25 minutos y seguimos perdiendo pestañas frente a espacios

– [Josh Triplett en una reunión del Equipo de Biblioteca](https://hackmd.io/RUHqJ5InQkCBRs5d0WpFBA)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1836) por la sugerencia! 

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1xi7826/this_week_in_rust_675/)</small>