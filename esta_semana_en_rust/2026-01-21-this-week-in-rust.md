---
title: "Esta semana en Rust #94"
number_of_week: 94
description: El crate de esta semana es throttled-tracing, una caja de macros de logging periódicas y con throttled.
date: 2026-01-21
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
* [¿Qué se necesita para enviar a Rust en modo crítico de seguridad?](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/)

### Boletines
* [El Rustacean Incrustado Número #63](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-63)
* [Scientific Computing in Rust #14 (enero 2026)](https://scientificcomputing.rs/monthly/2026-01)

### Actualizaciones de proyectos/herramientas
* [Creusot 0.9.0](https://creusot-rs.github.io/devlog/2026-01-19/)
* [Diesel-Guard v0.5.0 lanzado](https://github.com/ayarotsky/diesel-guard/releases/tag/v0.5.0)
* [skim v1.0.0: fuzzy-finder TUI ahora usa Ratatui](https://github.com/skim-rs/skim/releases/tag/v1.0.0)
* [d-engine - Un motor ligero de coordinación distribuida para el Rust](https://dev.to/joshua_c/d-engine-a-lightweight-distributed-coordination-engine-for-rust-210j)
* [govctl: Herramienta CLI con opiniones para aplicar la codificación de IA impulsada por RFC](https://dev.to/lucifer1004/govctl-opinionated-cli-tool-to-enforce-rfc-driven-ai-coding-2ngi)
* [Burn 0.20.0 Release: Unificación de núcleos de CPU y GPU con CubeCL](https://burn.dev/blog/release-0.20.0/)
* [¡se ha lanzado git-cliff 2.12.0! (un generador de registros de cambios altamente personalizable)](https://git-cliff.org/blog/2.12.0)
* [Cot v0.5 Lanzado: Nuevas funciones para desarrolladores web perezosos](https://mackow.ski/blog/cot-v05-new-features-for-lazy-web-developers/)

### Observaciones/Pensamientos
* [Dejar de asignar por etiqueta: Una tabla de símbolos de Rust basada en datos para OTLP/TSDB](https://baarse.substack.com/p/stop-allocating-per-label-a-datadriven)
* [Perfilar una implementación de analizador en Rust](https://blog.wybxc.cc/blog/profile-cgrammar/)
* [La cultura de precisión semántica de Rust](https://www.alilleybrinker.com/mini/rusts-culture-of-semantic-precision/)
* [vídeo] [Rust no trata sobre la seguridad de la memoria](https://www.youtube.com/watch?v=ngTZN09poqk)

### Guías de Rust
* [Estructurando una aplicación Rust Gtk4](https://w-graj.net/posts/rust-gtk4-mvpvm/)
* [Biblioteca estándar de Rust en la GPU](https://www.vectorware.com/blog/rust-std-on-gpu/)
* [Concurrencia elegante y segura en Rust con combinadores asincrónicos](https://kerkour.com/rust-async-combinators-concurrency)
* [AWS Lambda desde cero](https://forgestream.idverse.com/blog/20260119-lambda-from-scratch/)
* [Usando Oracle db26ai de Rust con la caja oráculo - Consultas](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_oracle_crate_1/)
* [Usando Oracle db26ai de Rust con la caja de sibilas - Consultas](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_sibyl_crate_1/)

### Miscelánea
* [Informe de Empleos de Oxidación de diciembre de 2025](https://filtra.io/rust/jobs-report/dec-25)

## Crate de la semana

El crate de esta semana es [throttled-tracing](https://crates.io/crates/throttled-tracing), una caja de macros de logging periódicas y con throttled.

¡Gracias a [Paperinik](https://users.rust-lang.org/t/crate-of-the-week/2704/1522) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

##### [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing)
* [Problema de seguimiento para el backend de renderizado SVG del informe de temporización](https://github.com/rust-lang/cargo/issues/16440)

* *No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing) o
  [RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

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

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-11

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

464 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-13..2026-01-20

#### Compilador
* ['rustc_errors': Añadir (heurística) Resaltado de sintaxis para 'rustc --explain'](https://github.com/rust-lang/rust/pull/150895)
* [derivación de caché proc expansión de macro con consulta incremental](https://github.com/rust-lang/rust/pull/145354)
* [dote: Referencia de apoyo en información de tipo reflexión](https://github.com/rust-lang/rust/pull/151222)

#### Biblioteca
* [hacer que 'Type::of' soporte tipos no dimensionados](https://github.com/rust-lang/rust/pull/151019)

#### Carga
* ['git': evitar un oid parcial se ha rellenado cero](https://github.com/rust-lang/cargo/pull/16511)
* ['lockfile': cambiar a 'resolver.lockfile-path' config](https://github.com/rust-lang/cargo/pull/16510)
* [invalida toda la caché de compilación cuando cambia '-Zno-embed-metadata'](https://github.com/rust-lang/cargo/pull/16513)
* [movió los bins de build-script al directorio 'deps'](https://github.com/rust-lang/cargo/pull/16515)
* [optimizar 'cargo localizar-proyecto --espacio de trabajo'](https://github.com/rust-lang/cargo/pull/16423)
* [almacenan diferencias de artefactos en la dirección de construcción de la unidad de construcción](https://github.com/rust-lang/cargo/pull/16519)

#### Rustdoc
* [corregir errores de enlace intra-doc relacionados con alias de tipo y elementos asociados](https://github.com/rust-lang/rust/pull/150586)
* [dejar de evaluar incondicionalmente el inicializador de las consts asociadas](https://github.com/rust-lang/rust/pull/151232)

#### Clippy
* ['double_comparisons': comprueba expresiones como 'x != y & x >= y'](https://github.com/rust-lang/rust-clippy/pull/16033)
* ['collapsible_span_lint_calls': usa 'snippet_with_context' para los tramos que probablemente contengan macro expns](https://github.com/rust-lang/rust-clippy/pull/15881)
* ['unnecessary_sort_by': reducir diferencias de sugerencias](https://github.com/rust-lang/rust-clippy/pull/16417)
* [añadir pelusa de 'manual_checked_ops'](https://github.com/rust-lang/rust-clippy/pull/16149)
* [añadir pelusa de 'manual_take'](https://github.com/rust-lang/rust-clippy/pull/16368)
* [no consideran los operadores binarios como conmutativos por defecto](https://github.com/rust-lang/rust-clippy/pull/16420)
* [no emitir un error si la salida estándar está completamente activada --ayuda/--versión](https://github.com/rust-lang/rust-clippy/pull/16412)
* [corregir 'unnecessary_sort_by' falso negativo al acceso al campo](https://github.com/rust-lang/rust-clippy/pull/16406)
* [diagnóstico de 'needless_continue' posterior en el nodo derecho](https://github.com/rust-lang/rust-clippy/pull/16265)
* [salta la pelusa 'elidable_lifetime_names' para el código generado por macro proc](https://github.com/rust-lang/rust-clippy/pull/16402)
* [sugiere 'Cstr::count_bytes' en 'strlen_on_c_strings'](https://github.com/rust-lang/rust-clippy/pull/16323)

#### Analizador de Rust
* [activa la comprobación de vuelo si se modifican archivos fuera del espacio de trabajo](https://github.com/rust-lang/rust-analyzer/pull/21483)
* [corregir la precedencia de falsos positivos en '(2 como i32) < 3'](https://github.com/rust-lang/rust-analyzer/pull/21465)
* [no mostrar dependencias de sysroot en la búsqueda de símbolos](https://github.com/rust-lang/rust-analyzer/pull/21484)
* [no producir bloque redundante en 'move_guard'](https://github.com/rust-lang/rust-analyzer/pull/21485)
* [asegurar la captura correcta de parámetros asíncronos de fn incluso cuando usan patrones extraños](https://github.com/rust-lang/rust-analyzer/pull/21492)
* [búsqueda de símbolo de ruta de corrección no respeta las reexportaciones](https://github.com/rust-lang/rust-analyzer/pull/21464)
* [insertar variaciones de tipo y normalizar para el tipo de 'estática' usada](https://github.com/rust-lang/rust-analyzer/pull/21491)
* [buscar flycheck por ID en lugar de índice vectorial](https://github.com/rust-lang/rust-analyzer/pull/21475)
* [migrar 'unwrap_block' asistencia para usar SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21458)
* [eliminar postal del legado](https://github.com/rust-lang/rust-analyzer/pull/21466)

### Triaje de rendimiento del compilador Rust

Varios cambios en ambas direcciones, pero en general no ha cambiado mucho.

Triaje hecho por **@panstromek**.
Rango de revisión: [840245e9.. 3d087e60](https://perf.rust-lang.org/?start=840245e91b90f22adf9f26d0a0cd98c2416cdef3&end=3d087e6044bddc65723bf42c76fe4cc33a0076b0&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,6% | [0,1%, 1,6%] | 21 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,0%, 2,6%] | 113 |
| Mejoras ✅ <br /> (primaria) | -0,3% | [-2,1%, -0,2%] | 37 |
| Mejoras ✅ <br /> (secundario) | -1,2% | [-29,6%, -0,1%] | 37 |
| Todos ❌✅ (primario) | 0,0% | [-2,1%, 1,6%] | 58 |

3 regresiones, 4 mejoras, 7 mixtas; 6 de ellos en rollups
40 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/98f432f4bae9972f8f320bb0df52c80546cae724/triage/2026/2026-01-19.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [RFC: '#[export_visibility = ...]' atributo](https://github.com/rust-lang/rfcs/pull/3834)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para AArch64 FEAT_JSCVT](https://github.com/rust-lang/rust/issues/147555)
* [thread::scope: documenta cómo interactúa join con los destructores TLS](https://github.com/rust-lang/rust/pull/149482)A
* [No intentes evaluar bloques const durante la promoción constante](https://github.com/rust-lang/rust/pull/150557)
* [implementa Eq<Parcial<U\>\> para [T; N] y &[T; N]](https://github.com/rust-lang/rust/pull/149045)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [docs(informe): mejorar las páginas de usuario para 'informe de carga *'](https://github.com/rust-lang/cargo/pull/16430)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Directores de proyecto: responsabilidades](https://github.com/rust-lang/leadership-council/pull/250)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
  [RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Autenticación del registro mTLS de carga](https://github.com/rust-lang/rfcs/pull/3907)
* [RFC: desambiguación del método de estilo obj-action](https://github.com/rust-lang/rfcs/pull/3908)

## Próximos eventos

Eventos Rusty entre el 21-01-2026 - el 18-02-2026 🦀

### Virtual
* 2026-01-21 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/e2ea7hxo)
* 26-01-2026 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Lectura de código oxidado y contribución de código abierto (UTC 18:00; Inglés)**](https://www.meetup.com/code-mavens/events/312782592/)
* 2026-01-27 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-27 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Almuerzo y aprendizaje: Manejo de errores en Rust**](https://www.meetup.com/women-in-rust/events/312799344/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/9h9n0dau)
* 2026-01-28 | Virtual (Lima-Perú, PE) | [Perú Oxidado](https://rust.pe)
    * [**Meetup: Oxidación del hardware a la web (Embebido + Backend desde cero)**](https://calendar.app.google/jc9DziLWVTUn1qNVA)    
* 2026-01-29 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Encuentro de Bevy #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #2 - Procesos oxidados, límites de memoria y cápsulas básicas**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 1: Conceptos Comunes de Programación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Lectura de código oxidado y contribución de código abierto (UTC 18:00; Inglés)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 2: Propiedad y Estructuras**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ir8s81ec)

### Asia
* 2026-02-05 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Encuentro de Malasia Rust febrero 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)

### Europa
* 2026-01-21 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 2026-01-22 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin en localización 🏳️ 🌈 - Edición 010**](https://www.meetup.com/rust-berlin/events/312962219/)
* 26-01-2026 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #17: Emily Coaca - Entwicklung des Kernels Update für TockOS**](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-27 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #4 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312851079)
* 2026-01-27 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Enero de Rust Manchester**](https://www.meetup.com/rust-manchester/events/312848708/)
* 2026-01-28 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - enero 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-01-28 | Praga, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga (enero de 2026)**](https://www.meetup.com/rust-prague/events/312895761/)
* 2026-01-28 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - WASM & Elegant CLI**](https://www.meetup.com/rust-community-toulouse/events/312782796/)
* 2026-01-29 | Ostrava, CZ | [Encuentro con Actualización Ostrava](https://www.meetup.com/meetupdate-ostrava)
    * [**MeetUpdate Ostrava #28: Rust**](https://www.meetup.com/meetupdate-ostrava/events/312747904/)
* 2026-01-31 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #23**](https://www.meetup.com/stockholm-rust/events/312919934/)
* 2026-02-04 | Darmstadt, HE, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo un servicio de suscripción a un boletín con axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Más allá del código: Diseñando servicios que resisten la prueba del tiempo**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Hack Rust & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Hack y Aprendizaje de Karlsruhe Meetup en BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-18 - 2026-02-19 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)

### Norteamérica
* 2026-01-21 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-21 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Seguro de la pila**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-22 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Encuentro nocturno de Boston Rust con Jiff, 22 de enero**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Davis Square Rust, 24 de enero**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Ángeles: Construyendo reemplazos de Git-LFS en Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Programación Rust 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust del Noreste, 31 de enero**](https://www.meetup.com/bostonrust/events/312483767/)
* 2026-02-03 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Renderizado y tiempos de construcción de Bevy en Amazon**](https://www.meetup.com/rust-nyc/events/312871242/)
* 2026-02-05 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Renderizando el set de Mandelbrot en Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Desarrollo web Full Stack en Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> podría sospechar que si agrupas todos los lenguajes tipados estáticamente en un solo cubo sin hacer distinción particular entre ellos, puede que no hayas interiorizado completamente las implicaciones de las estructuras de datos tipadas union (es decir, enum de Rust, o sum) combinadas con una coincidencia exhaustiva de patrones.
>
> me gusta llamarlo "pillado por sindicatos" y es muy difícil aceptar lenguajes estáticamente tipados una vez que te familiarizas.

– [Arwhatever en Hacker News](https://news.ycombinator.com/item?id=45043148)

¡Gracias a [Colin Bennett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1748) por la sugerencia!

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1qjkhiv/this_week_in_rust_635/)</small>
