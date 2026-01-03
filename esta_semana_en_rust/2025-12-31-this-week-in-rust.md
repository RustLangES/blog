---
title: "Esta semana en Rust #91"
number_of_week: 91
description: El crate de esta semana es wgsl-bindgen.
date: 2025-12-31
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

### Actualizaciones de proyectos/herramientas
* [Reqwest v0.13 - Suena por defecto](https://seanmonstar.com/blog/reqwest-v013-rustls-default/)
* [SE PUBLICA RAMA 0.3.0-Alpha.4 — Marco de Servicios Modulares para mover y transformar paquetes de red](https://github.com/plabayo/rama/releases/tag/rama-0.3.0-alpha.4)
* [¡Se lanza Ratatui 0.30.0! - una biblioteca de Rust para crear interfaces de usuario de terminales](https://ratatui.rs/highlights/v030/)

### Observaciones/Pensamientos
* [Cuatro años de Rust: Una odisea de fracasos, logros y duras lecciones](https://medium.com/@adefemiadeoye/four-years-of-rust-an-odyssey-of-failures-achievements-and-hard-lessons-0da41298a152)
* [Inferencia de tipo bidireccional simple](https://ettolrach.com/blog/bidirectional_inference.html)
* [El préstamo de Serde puede ser traicionero](https://yossarian.net/til/post/serde-s-borrowing-can-be-treacherous/)
* [La recogida de basura en Rust mejoró un poco](https://claytonwramsey.com/blog/dumpster2/)
* [audio] [Netstack.FM episodio 20 — Especial de Año Nuevo Netstack.FM, Resumen 2025](https://netstack.fm/#episode-20)

### Guías de Rust
* [¿Por qué llamar a mi función asm desde Rust es más lento que llamarla desde C?](https://ohadravid.github.io/posts/2025-12-rav1d-faster-asm/)
* [Errores de Rust Sin Dependencias](https://vincents.dev/blog/rust-errors-without-dependencies/)
* [vídeo] [¡Creando tu primera APP usando el nuevo Hotaru Web Framework!](https://www.youtube.com/watch?v=8pV-o04GuKk)

### Miscelánea
* [audio] [Especial navideño 2025 - Pódcast Rust in Production](https://corrode.dev/podcast/s05e07-holiday/)
* [Investigando y arreglando un desagradable error de clon](https://kobzol.github.io/rust/2025/12/30/investigating-and-fixing-a-nasty-clone-bug.html)

## Crate de la semana

El crate de esta semana es [wgsl-bindgen](https://github.com/Swoorup/wgsl-bindgen), un generador de binding para WGSL, el lenguaje de sombreado WebGPU, que se usa con [wgpu](https://github.com/gfx-rs/wgpu).

¡Gracias a [Artem Borisovskiy](https://users.rust-lang.org/t/crate-of-the-week/2704/1511) por la sugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

##### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* [Rustup 1.29.0 beta: ¡Llamada para pruebas!](https://blog.rust-lang.org/inside-rust/2025/12/20/rustup-1.29.0-beta-cft/)
  - Pasos de prueba: Consulta la sección "Cómo probar" en el enlace anterior.

* *No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [Spindalis - Crear un analizador AST](https://github.com/lignum-vitae/spindalis/issues/27)
* [Spindalis - Añadir macro procedimental para integral definida](https://github.com/lignum-vitae/spindalis/issues/39)
* [Spindalis - Añadir una función y una macro que puedan expandir polinomios](https://github.com/lignum-vitae/spindalis/issues/36)
* [Spindalis - Añadir rasgo de visualización a funciones en el núcleo spindalis](https://github.com/lignum-vitae/spindalis/issues/43)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 18-01-2026 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se han fusionado 297 solicitudes de tirada [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-23..2025-12-30

#### Compilador
* [mejoras en la delegación recursiva](https://github.com/rust-lang/rust/pull/150347)
* [miri: arreglar el ICE para situaciones concretas de carrera de datos](https://github.com/rust-lang/miri/pull/4796)
* [MIRI: Mostrar una advertencia al combinar el modo native-lib y muchas semillas](https://github.com/rust-lang/miri/pull/4790)
* [miri: árbol Toma prestado: mejora protector y acceso al niño salto](https://github.com/rust-lang/miri/pull/4766)

#### Biblioteca
* [añadir 'MaybeDangling' a 'núcleo'](https://github.com/rust-lang/rust/pull/149775)
* [alloc: especializar 'String::extend' para cortes de fuerza](https://github.com/rust-lang/rust/pull/149694)
* [implementa 'Duración::d iv_duración_{floor,ceil}'](https://github.com/rust-lang/rust/pull/149582)
* [implementa aplanar para 'Option<&<T>Option>' y 'Option<&mut <T>Option>'](https://github.com/rust-lang/rust/pull/108671)
* [implementación optimizada para 'uN::{gather,scatter}_bits'](https://github.com/rust-lang/rust/pull/149663)
* [reescritura de 'String::replace_range'](https://github.com/rust-lang/rust/pull/149447)
* [estabilizar 'lazy_get'](https://github.com/rust-lang/rust/pull/150016)

#### Carga
* ['índice': Estabilizar el tiempo de publicación](https://github.com/rust-lang/cargo/pull/16372)
* ['informe': nuevo mando 'sesiones de informe de carga'](https://github.com/rust-lang/cargo/pull/16428)
* ['informe': soporte --ruta de manifiesto en 'tiempos de informes de carga'](https://github.com/rust-lang/cargo/pull/16441)
* ['resolver': Lista de características cuando no hay coincidencia cercana](https://github.com/rust-lang/cargo/pull/16445)
* ['toml': soporte de análisis sintáctico TOML 1.1](https://github.com/rust-lang/cargo/pull/16415)
* ['vendor': filtra recursivamente archivos git en subdirectorios](https://github.com/rust-lang/cargo/pull/16439)
* ['vendor': desempaquetar desde la ruta de caché del registro local](https://github.com/rust-lang/cargo/pull/16435)
* ['build-rs': Reducir de 'build' a 'check' cuando sea posible](https://github.com/rust-lang/cargo/pull/16444)
* [experimento: pipeline de renderizado de temporización en SVG](https://github.com/rust-lang/cargo/pull/15091)
* [parche: Mostrar dónde se definió el parche en mensajes de error relacionados con el parche](https://github.com/rust-lang/cargo/pull/16407)

#### Rustdoc
* [si se desactiva la configuración del número de línea, no hagas que los números de línea ocupen espacio](https://github.com/rust-lang/rust/pull/150396)
* [ejemplo de código de copia con números de línea](https://github.com/rust-lang/rust/pull/150395)
* [corregir secciones duplicadas de reexportación](https://github.com/rust-lang/rust/pull/150362)
* [corregir el tipo incorrecto del nombre del filtro en la ventana emergente de ayuda](https://github.com/rust-lang/rust/pull/150360)

#### Clippy
* [fijar 'assertions_on_constants' falso positivo cuando hay un valor no constante en la condición expr](https://github.com/rust-lang/rust-clippy/pull/16297)
* [corregir 'double_parens' falso positivo en patrones de macro repetición](https://github.com/rust-lang/rust-clippy/pull/16301)
* [corregir macros 'obfuscated_if_else' mal desordenadas](https://github.com/rust-lang/rust-clippy/pull/16289)
* [corregir 'result_large_err' falso negativo en cierres](https://github.com/rust-lang/rust-clippy/pull/16277)
* [preservar información explícita de por vida al eliminar 'mut'](https://github.com/rust-lang/rust-clippy/pull/16273)
* [diversas correcciones para el manejo de macros](https://github.com/rust-lang/rust-clippy/pull/16296)

#### Analizador de Rust
* [añadir prototipo de mensajería bidireccional proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21249)
* [añadir completación de segmento macro](https://github.com/rust-lang/rust-analyzer/pull/20741)
* [implementa configuración para cambiar subcomando por test, bench y doctest](https://github.com/rust-lang/rust-analyzer/pull/21308)
* [proporcionar una configuración para desactivar la visualización de conflictos de cambio de nombre](https://github.com/rust-lang/rust-analyzer/pull/20193)
* [diagnóstico 🎉 de desajuste de tipo de estabilización ](https://github.com/rust-lang/rust-analyzer/pull/21337)
* [sangría para 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/21330)
* [arreglar gestión de solicitudes de configuración LSP](https://github.com/rust-lang/rust-analyzer/pull/21297)
* [arreglar el análisis de 'format_args! ("...", palabra clave=...)'](https://github.com/rust-lang/rust-analyzer/pull/21351)
* [inferencia de tipo de corrección al pasar el cursor por '_'](https://github.com/rust-lang/rust-analyzer/pull/21358)
* [volver a activar variación de punto fijo](https://github.com/rust-lang/rust-analyzer/pull/21348)
* [realmente no expandir los derivados incorporados, sino tratarlos específicamente](https://github.com/rust-lang/rust-analyzer/pull/21200)
* [preasignar algunos buffers en el análisis sintáctico](https://github.com/rust-lang/rust-analyzer/pull/21353)
* [reduce la contención de bloqueo de canal para drop-threads](https://github.com/rust-lang/rust-analyzer/pull/21355)
* [pide al usuario en VSCode que añada el componente rust-anaylzer al archivo de la cadena de herramientas](https://github.com/rust-lang/rust-analyzer/pull/21359)

### Triaje de rendimiento del compilador Rust

No ha habido muchos cambios esta semana. El resultado global es positivo, en gran parte gracias a https://github.com/rust-lang/rust/pull/142881, que hace que la computación de una estructura de datos costosa para la optimización de MIR sea perezosa.

Triaje hecho por **@panstromek**.
Rango de revisión: [e1212ea7.. 112a2742](https://perf.rust-lang.org/?start=e1212ea79b38d51954625291c04d2797c4bb8ec5&end=112a274275d77ebc2b892f056a1e2fad141f4f08&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,1%, 1,7%] | 11 |
| Regresiones ❌ <br /> (secundario) | 0,2% | [0,1%, 0,5%] | 6 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-1,3%, -0,1%] | 74 |
| Mejoras ✅ <br /> (secundario) | -0,6% | [-1,8%, -0,2%] | 71 |
| Todos ❌✅ (primario) | -0,4% | [-1,3%, 1,7%] | 85 |

2 regresiones, 0 mejoras, 3 mixtas; Uno de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/19d2f05e6e3c86fe2496deb4d8ed585375602d78/triage/2025/2025-12-29.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?
* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Dar un signo a los literales enteros en lugar de depender de expresiones de negación](https://github.com/rust-lang/compiler-team/issues/835)
* [También activar volcados de archivos ICE en stable](https://github.com/rust-lang/compiler-team/issues/809)
* [Nueva propuesta objetivo de nivel 3: 'loongarch64-linux-android'](https://github.com/rust-lang/compiler-team/issues/806)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Punteros delgados con metadatos en línea](https://github.com/rust-lang/rfcs/pull/3898)

#### Problemas de seguimiento y marcas personales
<!-- O bien elimina al grupo de la sección "No se han introducido elementos en el periodo final de comentarios esta semana para"
     y añadir el/los ítem(s) que entraron en el periodo final de comentarios:
##### [Grupo](URL del Grupo)
* [Título del ítem](URL del ítem)
  - para elementos de 'fusión de disposición' 'periodo de comentario final', o
* [disposición: posponer]
  - para elementos de 'disposición-posponer' 'periodo de comentarios finales', o
* [disposición: cercana]
  - para ítems de 'cierre de disposición' y 'periodo de comentarios final',
* [disposición: no especificada]
  - cuando la 'disposición' no está especificada o asegurarse de que el grupo forma parte del
     Sección "No se han inscrito en el periodo final de comentarios esta semana para" sección "No se han inscrito en el Último Periodo de Comentarios esta semana"
*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.
-->

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
<!-- Utilizar * [Título del elemento](URL del artículo) - para nuevos elementos, o * [actualizado] [Título del artículo](URL del artículo) - para elementos actualizados, o * * No se han creado RFCs nuevos o actualizados esta semana.* -->

<!-- Ejemplo de mensajes de commit Actualizaciones de las secciones CFT, FCP, MCP y RFC para TWiR-xxx -->

## Próximos eventos

Eventos Rusty entre el 31-12-2025 - el 28-01-2026 🦀

### Virtual
* 2026-01-03 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 07-01-2026 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/xov10pef)
* 2026-01-13 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Contribución al proyecto Live Open Source Rust**](https://www.meetup.com/code-mavens/events/312641560/)
* 2026-01-14 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/4p6rxjc5)
* 2026-01-15 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-20 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/e2ea7hxo)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-27 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/9h9n0dau)

### Asia
* 07-01-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**Rust en persona enero de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)
* 2026-01-08 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312645994/)
* 2026-01-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #12**](https://www.meetup.com/rustdelhi/events/312584516/)

### Europa
* 07-01-2026 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 07-01-2026 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-08 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por confirmar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | París, FR | [París Rust](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #82**](https://www.meetup.com/rust-paris/events/312364675/)

### Norteamérica
* 2026-01-01 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cancelado**](https://www.meetup.com/stl-rust/events/311396047/)
* 2026-01-03 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Allston Rust, 3 de enero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-01-08 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Trayendo el data lake al navegador con WASM**](https://www.meetup.com/utah-rust/events/312565472/)
* 2026-01-08 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-10 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Central Cambridge Rust, 10 de enero**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-13 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Panel de Rust NYC: Agentes de codificación de IA en Rust: Qué funciona, qué se rompe**](https://www.meetup.com/rust-nyc/events/312632598/)
* 2026-01-13 | Spokane, WA, EE. UU. [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**.NET Conference 2025 Spokane**](https://www.meetup.com/spokane-rust/events/312372555/)
* 2026-01-15 | Seattle, WA, EE. UU. | [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Enero, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust Común de Boston, 17 de enero**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-17 | Herndon, VA, EE. UU. | [NoVaLUG](https://mobilizon.us/@novalug)
    * [**Reunión mensual - Enfadar a Brian Lunduke, programa en Rust**](https://mobilizon.us/events/140c5c7c-01f3-4aaa-b218-58289c6b4449)
* 2026-01-20 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-22 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Encuentro nocturno de Boston Rust con Jiff, 22 de enero**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-24 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Davis Square Rust, 24 de enero**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust Los Ángeles: Construyendo reemplazos de Git-LFS en Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> qué es el tiempo?!?

– [Ralf Jung en su blog](https://www.ralfj.de/blog/2025/12/22/miri.html)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1743) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1q0yh7l/this_week_in_rust_632/)</small>
