---
title: "Esta semana en Rust #132"
number_of_week: 132
description: El crate de esta semana es tellus, un framework de actores basado en tokio que se puede usar sin necesidad de escribir código asincrónico. 
date: 2026-11-04
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

* [Destacado en mantenedores: Anthony Bolden (@Walnut356)](https://blog.rust-lang.org/inside-rust/2026/11/02/maintainer-spotlight-anthony-bolden-walnut356/)
* [Anunciando rustup 1.29.3](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)

### Fundación

* [Actualización sobre el personal de la Fundación Rust](https://rustfoundation.org/media/update-on-rust-foundation-staffing/)

### Boletines

* [Rust Trends Número 86 - systemd abre la puerta al Rust](https://rust-trends.com/newsletter/systemd-opens-the-door-to-rust/)

### Actualizaciones de proyectos/herramientas

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

* [sistemad ❤️ Rust](https://amutable.com/blog/systemd-loves-rust)
* [sysinfo: corrección de rendimiento para recuperar el uso de GPU de procesos](https://blog.guillaume-gomez.fr/articles/2026-10-30+sysinfo%3A+fixing+performance+to+retrieve+process+GPU+usage)
* [Hacia la verificación formal: mbrotli y mzstd](https://m-compressors.github.io/2026/formal-verification-mbrotli-mzstd/)
* [Arrugue: Versiones estables, API, texto... ¡y todo lo demás!](https://ruffle.rs/blog/2026/11/04/stable-releases-and-everything-else)
* [Informe de progreso #44: ¡rustc_codegen_gcc ahora puedo arrancar rustc en Aarch64!](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-44)

### Observaciones/Pensamientos

* [Pensando en Rasgos](https://corrode.dev/blog/thinking-in-traits/)
* [Cómo acelerar el compilador de Rust en noviembre de 2026](https://nnethercote.github.io/2026/11/04/how-to-speed-up-the-rust-compiler-in-november-2026.html)
* [Es la latencia de memoria, idiota · Blog Pivot](https://pivotlake.io/blog/its-the-memory-latency-stupid/)
* [El estado futuro desaparecido](https://blog.yoshuawuyts.com/the-future-trait-has-a-missing-state)
* [Todo lo que necesitas saber para proteger tus proyectos Rust contra la oleada de ciberataques](https://kerkour.com/secure-rust-projects)
* [audio] [Cajas pequeñas detrás de grandes sistemas de Rust](https://netstack.fm/episodes/small-crates-behind-big-rust-systems)
* [audio] [Migrando C a Rust con Jonas Kruckenberg](https://rustacean-station.org/episode/jonas-kruckenberg/)
* [Construcción de control de versiones de grano fino para agentes con Rust y Objetos Duraderos](https://delta.dev/blog/fine-grained-version-control-for-agents)

### Guías de Rust

* [serie] [Programador de Linux de Userland en Rust - más rápido que el tiempo](https://kapteko.com/userland-linux-scheduler-in-rust/)
* [¿Quién eres? versus ¿Quién te posee? — Qué tienen en común la seguridad de la memoria Rust y la direccionamiento inalámbrica](https://medium.com/@potto_94870/who-are-you-versus-who-owns-you-what-rust-memory-safety-and-wireless-addressing-have-in-common-b876a81468d5)
* [vídeo] [Rust Atomics desmitificado](https://www.youtube.com/watch?v=R8IeTvTHlxU)
* [vídeo] [Arregla tus funciones de Rust](https://www.youtube.com/watch?v=X5Bf_Udenys)

## Crate de la semana

El crate de esta semana es [tellus](https://crates.io/crates/tellus), un framework de actores basado en tokio que se puede usar sin necesidad de escribir código asincrónico. 

¡Gracias a [Sean Kwak](https://users.rust-lang.org/t/crate-of-the-week/2704/1708) por la autosugerencia parcial! 

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
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->
* [INVAR - Convertir las sondas de entrada hostil HELLTEST en pruebas de regresión comprometidas](https://github.com/Ramesh-35/invar/issues/2

* [Plastor - Portar el paso de trazado de Python a Rust](https://github.com/endlessGold/Plastor/issues/6)
* [Plastor - Lectura de profundidad de bandas tonales y contornos transversales (fijar cabezas planas en dibujos reales)](https://github.com/endlessGold/Plastor/issues/2)
* [Plastor - CI: compila y ejecuta las comprobaciones sin Blender en Linux, macOS y Windows](https://github.com/endlessGold/Plastor/issues/8)
* [Plastor - 'partitura mh': informe hermético y característica de Euler](https://github.com/endlessGold/Plastor/issues/10)
* [Quill - Mostrar el mensaje de servicio para chats iniciados a través de un enlace de mensajes gratuito](https://github.com/shinycake/quill/issues/1351)
* [Quill - Aplica el token de mensajes gratuitos al abrir un enlace que lo lleve](https://github.com/shinycake/quill/issues/1352)
* [Quill - Prueba Quill en máquinas Windows y Linux reales e informa de lo que falla](https://github.com/shinycake/quill/issues/1353)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->
* [**FOSDEM'27 Rust Devroom](https://rust-fosdem.github.io/) | 2026-11-23:59 CET | Bruselas, Bélgica | 2027-01-31

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

597 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-27..2026-11-03

#### Compilador
* [GCI: comprobar las cláusulas donde de constantes asociadas para comprobar la bien-formación](https://github.com/rust-lang/rust/pull/165184)
* [Eliminación de movimientos MIR (1.5/6): Asegurarse de que los ZSTs siempre estén inicializados](https://github.com/rust-lang/rust/pull/163359)
* [Eliminación de movimientos MIR (5/6): Reconstrucción de almacenamiento](https://github.com/rust-lang/rust/pull/163339)
* [comprueba los tipos asociados de vinculación de consts](https://github.com/rust-lang/rust/pull/161131)
* [desastringify infraestructura de pelusa](https://github.com/rust-lang/rust/pull/164607)
* [ampliar un caso de devolución anticipado en 'coerce_unsized'](https://github.com/rust-lang/rust/pull/164341)
* [nuevo solucionador opta por 'diésel' (parte 2)](https://github.com/rust-lang/rust/pull/164234)
* [nuevo solucionador opta por diésel (parte 3)](https://github.com/rust-lang/rust/pull/165160)
* [perf: emite nombres de campos como una cadena concatenada en 'derive(Debug)' para structs](https://github.com/rust-lang/rust/pull/164726)

#### Biblioteca
* [añadir 'MaybeUninit::from_ref'](https://github.com/rust-lang/rust/pull/165214)
* [añadir {'Receptor', 'Sendero', 'SyncSender'}'::'{'is_empty,len'}](https://github.com/rust-lang/rust/pull/165248)
* [añadir métodos de 'strict_' a 'NonZero'](https://github.com/rust-lang/rust/pull/164848)
* [añadir 'TrySendError::into_inner'](https://github.com/rust-lang/rust/pull/165103)
* [núcleo: constify 'resumable_breakpoint' y reexportarlo en 'std'](https://github.com/rust-lang/rust/pull/164608)
* [asegúrate de que la impl de pantalla de uXX sea pequeña con 'optimize_for_size'](https://github.com/rust-lang/rust/pull/165284)
* [implementa 'Duración::d segundos_de_juego'](https://github.com/rust-lang/rust/pull/165181)
* [implementar rasgos 'Fn' para funciones seguras de cualquier ABI](https://github.com/rust-lang/rust/pull/160186)
* [implementar 'Vec::retain_unstable' ACP](https://github.com/rust-lang/rust/pull/164155)
* [implementa 'advance_back_by' en 'Fusible'](https://github.com/rust-lang/rust/pull/165132)
* [hacen que los métodos 'IoSlice' e 'IoSliceMut' consteñen de forma inestable](https://github.com/rust-lang/rust/pull/144090)
* [trasladar la recogida de objetos al resolver](https://github.com/rust-lang/rust/pull/163525)
* [especializar vec 'extend_with'](https://github.com/rust-lang/rust/pull/163757)
* [optimizar elementos grandes para la rotación de cortes](https://github.com/rust-lang/rust/pull/162754)

#### Carga
* [windows: DLLs del sistema con retardo y carga que la mayoría de los comandos no usan](https://github.com/rust-lang/cargo/pull/17623)
* [corregir generación de esquema de manifiestos: 'TomlOptLevel' y 'StringOrVec'](https://github.com/rust-lang/cargo/pull/17660)
* [advertencia sobre ajustes de manifestación de proc-macro en conflicto](https://github.com/rust-lang/cargo/pull/17641)

#### Rustdoc
* [añadir la opción inestable '--caja-lista-encabezado' para personalizar la lista de cajas en la barra lateral](https://github.com/rust-lang/rust/pull/138143)
* [mejora 'ItemKind'](https://github.com/rust-lang/rust/pull/162916)
* [mejoras notables de insignia de rasgo y popover](https://github.com/rust-lang/rust/pull/160370)

#### Rustfmt
* [tienen 'cfg_select!' doble refuerzo conciso de formato respetando 'control_brace_style'](https://github.com/rust-lang/rustfmt/pull/7214)

#### Clippy
* [no reportar la falta de documento de seguridad por funciones atribuidas '#[target_feature(enable)]'](https://github.com/rust-lang/rust-clippy/pull/17051)
* [arreglar el hielo en 'arbitrary_source_item_ordering'](https://github.com/rust-lang/rust-clippy/pull/17941)
* [corregir 'assert_is_empty' falso positivo con contexto constante](https://github.com/rust-lang/rust-clippy/pull/17950)
* [sugerencia de corregir 'borrowed_box' para que los objetos de rasgo mencionen '.as_ref()' en los sitios de llamada](https://github.com/rust-lang/rust-clippy/pull/17794)
* [sugerencias de 'manual_inspect' para mutar cierres](https://github.com/rust-lang/rust-clippy/pull/17741)
* [corrección: error de redondeo de ABS firmado en la comprobación de identidad del resto](https://github.com/rust-lang/rust-clippy/pull/17963)

#### Analizador de Rust
* [corregir el pánico LSP en función no ligada cuando hay genéricos presentes](https://github.com/rust-lang/rust-analyzer/pull/23602)
* [corregir la reducción de 'yield' en 'async gen'](https://github.com/rust-lang/rust-analyzer/pull/23586)
* [arreglar: sugerir referencia tras las desreferencias](https://github.com/rust-lang/rust-analyzer/pull/23562)
* [pánico 'Expr no está en búsqueda' debido a tipos de proyección](https://github.com/rust-lang/rust-analyzer/pull/23495)
* [perf: proporciona 'ThinMap', un mapa que usa 'ThinVec' cuando la longitud es pequeña](https://github.com/rust-lang/rust-analyzer/pull/23228)

### Triaje de rendimiento del compilador Rust

Esta semana se negocia un aumento del 0,4% en las instrucciones por una reducción del 33% en el tamaño del 'rmeta' en algunos benchmarks reales. 
Incluso con esa regresión, la media de esta semana sigue siendo muy positiva gracias a una refactorización en torno a 'AttrTokenStream'. 
usando 'Símbolos' en lugar de 'String' en la infraestructura de pelusa, 
y evitando una caminata extra de AST para recoger objetos de lang, entre otras cosas. 
Mucho ruido esta semana, que se debe principalmente a dos grandes rollups cuyo récord base era ruido. 

Triaje hecho por **@JonathanBrouwer**. 
Rango de revisión: [019b59d8.. 4c46afa6](https://perf.rust-lang.org/?start=019b59d8101bb3e141f1b880684747b4217beeb6&end=4c46afa63688fb473618f9b3553d70b0c15a88eb&absolute=false&stat=instructions%3Au)

3 regresiones, 11 mejoras, 27 mixtas; 2 de ellas en rollups
195 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/87c69dbbcdc5f571ebe50dc0107c846b6a102422/triage/2026/2026-11-02.md)

<!-- resultados de rendimiento aquí -->

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/main)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* [RFC: Añadir 'objetivos-requeridos' para la selección de paquetes en el espacio de trabajo](https://github.com/rust-lang/rfcs/pull/4013)
* [Actualizar acciones/acción de páginas de despliegue a la versión 5.0.1](https://github.com/rust-lang/rfcs/pull/4026)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Presentar correctamente el documento de la puerta (rust_logo)](https://github.com/rust-lang/rust/pull/164582)
* [núcleo: Estabilizar 'abort_immediate' como 'crash'](https://github.com/rust-lang/rust/pull/164731)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Añadir herramienta bootstrap para analizar el compilador prueba la cobertura de código](https://github.com/rust-lang/compiler-team/issues/1047)
* [Añadir pruebas para maquinaria de pelusa en tiempo de ejecución](https://github.com/rust-lang/compiler-team/issues/1004)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), 
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Encabezado de lista de cajas RFC](https://github.com/rust-lang/rfcs/pull/4027)

## Próximos eventos

Eventos Rusty entre el 4 de septiembre de 2026 y el 2 🦀 de septiembre de 2026

### Virtual
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
* 2026-12-01 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Reto de Programación Navidad - ¡Inicio!**](https://www.meetup.com/women-in-rust/events/316993134/)
* 2026-12-02 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcqbdb/)

### Asia
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**Encuentro de Rustacean de noviembre de 2026](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europa
* 2026-11-04 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Empezando con el nuevo controlador Rust para la base de datos de IA de Oracle**](https://www.meetup.com/rust-dublin/events/316898325/)
* 04-11-2026 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en noviembre: tour de descubrimiento de los estándares estándar**](https://www.meetup.com/rustcologne/events/317183850/)
* 2026-11-05 | Milán, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Trenes Modelo Digitales & Rust: Estación de Mando DCC de Metal Desnudo en ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 05-11-2026 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Epítome de excelencia sináptica**](https://www.meetup.com/oxford-rust-meetup-group/events/317158298/)
* 2026-11-12 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Depuración en Rust**](https://www.meetup.com/rust-rhein-main/events/317175844/)
* 2026-11-12 | Ginebra, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milán, MI, IT | [Milán en idioma oxidado](https://www.meetup.com/rust-language-milano/events/)
    * [**De la curiosidad a la capacidad: Crea tu propio enjambre de dispositivos**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Programación moderna de sistemas en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Todo es Git, los protocolos son solo adaptadores**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de Charla en EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)
* 2026-11-19 | Bergen, NO | [Rugido Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust: FFI con C++ e Introducción a Async**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/317045463/)
* 20-11-2026 y 2026-11-21 | Augsburgo, DE | [Encuentro Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Incrustado Rust mit Embassy - Workshop und Projekttag**](https://my.tuxedocomputers.com/event/embedded-rust-mit-embassy-workshop-und-projekttag-28/register)
* 2026-11-26 | Copenhague, DK | [Comunidad Copenhagen Rust](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #73 patrocinado por Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/317181792/)
* 2026-11-26 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London x Zed**](https://www.meetup.com/rust-london-user-group/events/317253958/)
* 2026-11-26 | Manchester, Reino Unido | [Manchester Oxidado](https://www.meetup.com/rust-manchester/events/)
    * [**Noche de Código de Noviembre de Rust Manchester**](https://www.meetup.com/rust-manchester/events/317222338/)
* 2026-11-26 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Lovable**](https://www.meetup.com/stockholm-rust/events/317161063/)
* 01-12-2026 | París, FR | [París Oxidado](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #90**](https://www.meetup.com/rust-paris/events/316865676/)

### Norteamérica
* 05-11-2026 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Social mensual de noviembre**](https://www.meetup.com/rust-montreal/events/316919004/)
* 05-11-2026 | San Francisco, CA | [Rust LA: NVIDIA](https://luma.com/user/usr-eWgtha6CHSTma7v/)
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://luma.com/iqe7wng1)
* 05-11-2026 | Saint Louis, MO, EE.UU. [Rust STL](https://www.meetup.com/stl-rust)
    * [**HTMX y Rust**](https://www.meetup.com/stl-rust/events/316896797/)
* 2026-11-12 | Lehi, UT, EE. UU. | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Rust Game Jam**](https://www.meetup.com/utah-rust/events/317178211/)
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
* 2026-11-18 | San Francisco, CA, EE. UU.) | [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Noche Social**](https://luma.com/v0mxbgs8)
* 2026-11-21 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Winter Hill Rust, 21 de noviembre**](https://www.meetup.com/bostonrust/events/316924366/)
* 2026-11-28 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de la Estación Sur, 28 de noviembre**](https://www.meetup.com/bostonrust/events/316924412/)

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

> Piensa en cómo pueden recibir tus contribuciones antes de enviarlas. Este tipo de pensamiento, pensamiento verdadero, con emoción y razonamiento reales, es lo único que tú, como humano, tienes garantizado de poder aportar. 

– [Jubilee sobre los usuarios de Rust](https://users.rust-lang.org/t/tips-for-your-first-pull-request/143356/3)

¡Gracias a [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1838) por la sugerencia! 

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1xo7mbb/this_week_in_rust_676/)</small>