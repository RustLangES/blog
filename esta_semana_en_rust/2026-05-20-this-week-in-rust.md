---
title: "Esta semana en Rust #110"
number_of_week: 110
description: El crate de esta semana es cargo-crap, un subcomando de carga para calcular la métrica Change Risk Anti-Patterns para una caja.
date: 2026-05-20
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

* [Actualización de objetivos del proyecto — abril de 2026 (finales de 2025H2)](https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/)
* [Actualización de gestión de programas — abril de 2026](https://blog.rust-lang.org/inside-rust/2026/05/13/program-management-update--april-2026/)

### Boletines

* [Este mes en Rust OSDev: abril 2026](https://rust-osdev.com/this-month/2026-04/)

### Actualizaciones de proyectos/herramientas

* [Tonic se une al proyecto gRPC](https://luciofranco.com/blog/tonic-joins-grpc/)
* [Warmy 0.6.0 - ¿Qué hay de nuevo?](https://tokio.rs/blog/2026-05-15-announcing-toasty-0-6-0)
* [ex_ratatui: Encuadernaciones de elixir para ratatui mediante NIFs Rustler](https://hexdocs.pm/ex_ratatui)
* [OmniScope: Un analizador estático LLVM IR multilenguaje que apunta a límites inseguros/FFI](https://medium.com/@jinhopers/in-depth-llvm-ir-how-omniscope-tracks-ownership-across-languages-2919e418ca61): 
* [citum: un nuevo procesador de citas de Rust y herramientas asociadas.](https://citum.org/)
* [cargo-clab: Encontrando complejidad no probada en código de Rust generado por IA](https://minikin.me/blog/cargo-crap)
* [Lo que debe el gráfico: conectores que impulsan la salida](https://aimdb.dev/blog/graph-owes)
* [swpui: un TUI para búsqueda y reemplazo consciente del caso](https://beeb.li/blog/introducing-swpui)
* [kache 0.3.0: compilación eficiente de árboles de trabajo sin copias](https://kunobi.ninja/blog/kache-update)
* [ghr: una TUI de Rust para gestionar pull requests, incidencias, notificaciones y revisiones de GitHub](https://catcoding.me/ghr/)

### Observaciones/Pensamientos

* [Escalando bases de código de Rust: Lecciones aprendidas organizando grandes proyectos y gestionando errores](https://kerkour.com/rust-organize-large-projects-code-error-handling)
* [Migrando de Go a Rust](https://corrode.dev/learn/migration-guides/go-to-rust/)
* [Por qué construí wrkflw](https://blog.gokuls.in/posts/why-i-built-wrkflw.html)
* [vídeo] [Modo Dios de Rust](https://www.youtube.com/watch?v=VIsKIzFz_zA)
* [vídeo] [Cómo Rust diseñó el tiempo de ejecución asíncrono perfecto](https://www.youtube.com/watch?v=FUg1y-yv6cs)

### Guías de Rust

* [5× fast_blur más rápido en imagen-rs](https://apas.tel/blog/optimizing-image-rs-blur)
* [Encontrando el tiempo Parte 2 - Rust Async y el temporizador genérico del brazo](https://thejpster.org.uk/blog/blog-2026-05-17/)
* [Analizando archivos .tres de Godot y recorriendo el grafo de recursos](https://assethoard.com/blog/parsing-godot-tres-files)
* [Rust x GBA: Configuración y Pixeles](https://jonahnestrick.com/blog/rust-gba-tutorial-1/)
* [Aprende vidas útiles de Rust construyendo una caché LRU genérica](https://blog.sheerluck.dev/posts/learn-rust-lifetimes-by-building-a-lru-cache/)
* [Cómo comparar código de Rust con Gungraun](https://bencher.dev/learn/benchmarking/rust/gungraun/)
* [Libro: Introducción a la programación, usando ECS y EBP en Rust](https://root-11.github.io/intro-book/)

## Crate de la semana

El crate de esta semana es [cargo-crap](https://github.com/minikin/cargo-crap), un subcomando de carga para calcular la métrica Change Risk Anti-Patterns para una caja.

A pesar de la lamentable falta de sugerencias, llogiq está satisfecho con su elección.

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

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Computación Científica en Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

369 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-12..2026-05-19

#### Compilador
* [añadir llamada de función Swift ABI](https://github.com/rust-lang/rust/pull/155815)
* [implementa azúcar con gota clavada](https://github.com/rust-lang/rust/pull/156452)

#### Biblioteca
* ['map_try_insert' cambia](https://github.com/rust-lang/rust/pull/155360)
* [implementa 'OsStr::split_at'](https://github.com/rust-lang/rust/pull/156444)
* [implementa 'into_array' para 'Vec<T>'](https://github.com/rust-lang/rust/pull/156234)
* [mover 'std::io::Cursor' a 'core::io'](https://github.com/rust-lang/rust/pull/156428)
* [mover 'std:io::util' a 'core::io'](https://github.com/rust-lang/rust/pull/156431)
* [ensancha el resultado de 'widening_mul'](https://github.com/rust-lang/rust/pull/156644)

#### Carga
* ['clean': respeta la configuración 'build.target' para 'clean -p'](https://github.com/rust-lang/cargo/pull/16988)
* ['diag': Consolidar, verificar/ejecutar diagnósticos pasa](https://github.com/rust-lang/cargo/pull/16989)
* ['diag': Reportar diagnósticos diferidos como otros diagnósticos](https://github.com/rust-lang/cargo/pull/16994)
* ['diag': Atrae el pase de analización](https://github.com/rust-lang/cargo/pull/17008)
* ['lints': Evita compilar cuando sea posible](https://github.com/rust-lang/cargo/pull/17007)
* [elimina '-Zunstable-options' por 'rustdoc --emitir'](https://github.com/rust-lang/cargo/pull/17002)

#### Rustdoc
* [estabilizar bandera '--emitir'](https://github.com/rust-lang/rust/pull/146220)
* [manejar correctamente los elementos asociados en la expansión de macros de rustdoc](https://github.com/rust-lang/rust/pull/156587)
* [mejoras de corrección y rendimiento en el enlace a la definición](https://github.com/rust-lang/rust/pull/156413)
* [soportar correctamente macros con varios tipos](https://github.com/rust-lang/rust/pull/152449)

#### Clippy
* [corregir 'duration_suboptimal_units' para literales pequeños](https://github.com/rust-lang/rust-clippy/pull/16922)
* [corrección aritmética de efectos secundarios falsos positivo](https://github.com/rust-lang/rust-clippy/pull/17011)

#### Analizador de Rust
* [añadir diagnóstico para E0029](https://github.com/rust-lang/rust-analyzer/pull/22347)
* [añadir diagnóstico para E0614](https://github.com/rust-lang/rust-analyzer/pull/22380)
* [añadir diagnóstico para E0638](https://github.com/rust-lang/rust-analyzer/pull/22355)
* [añadir manejador para E0040](https://github.com/rust-lang/rust-analyzer/pull/22378)
* [codificar el nombre en lugar de índice en 'EnumVariantId'](https://github.com/rust-lang/rust-analyzer/pull/22329)
* [fix assist 'qualify_path' pierde el segmento de camino](https://github.com/rust-lang/rust-analyzer/pull/22354)
* [añadir parámetro en métodos de resultado para 'replace_method_eager_lazy'](https://github.com/rust-lang/rust-analyzer/pull/22335)
* ['ref_match' completo en macro](https://github.com/rust-lang/rust-analyzer/pull/22399)
* [soporte total para tipos de patrones](https://github.com/rust-lang/rust-analyzer/pull/22368)
* [usos de handle en la macro para 'extract_function'](https://github.com/rust-lang/rust-analyzer/pull/22344)
* [no hay dos puntos completos antes de que existan dos puntos](https://github.com/rust-lang/rust-analyzer/pull/22386)
* [sin pelusas ADT sin tamaño 'self_ty' faltante, asociado acotado](https://github.com/rust-lang/rust-analyzer/pull/22363)
* [métodos de deref inherentes no completos al mismo nombre](https://github.com/rust-lang/rust-analyzer/pull/22376)
* [solo la referencia coincide con objetos de valor no desconocido](https://github.com/rust-lang/rust-analyzer/pull/22367)
* [mostrar Ejecutar objetivo para el principal de FN en blancos de banca](https://github.com/rust-lang/rust-analyzer/pull/22357)
* [usuario 'TyKind::{Pat,UnsafeBinder}' en 'has_drop_glue'](https://github.com/rust-lang/rust-analyzer/pull/22384)
* [implementar la macro 'pattern_type'](https://github.com/rust-lang/rust-analyzer/pull/22082)
* [resolución de método: emitir error para llamadas a métodos con límite de tamaño ilegal](https://github.com/rust-lang/rust-analyzer/pull/22372)
* [migrar 'inline_call' asistencia a SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/22352)
* [perf: proporcionar acceso al 'LineIndex' de 'RootDatabase' para el protocolo de macro de proceso](https://github.com/rust-lang/rust-analyzer/pull/22191)
* [mostrar 'const' en la ayuda de firma si procede](https://github.com/rust-lang/rust-analyzer/pull/22358)
* [mostrar 'inseguro' en la ayuda de firma si procede](https://github.com/rust-lang/rust-analyzer/pull/22381)

### Triaje de rendimiento del compilador Rust

Se combinaron menos récords personales de lo habitual, principalmente por una semana más corta de lo normal y algunos
Problemas con el informe. En general, una semana ligeramente positiva en cuanto a rendimiento.

Triaje hecho por **@simulacrum**.
Rango de revisión: [29b75901.. 281c97c3](https://perf.rust-lang.org/?start=29b7590130c83542a095cdf1323ed0f78eec2bb8&end=281c97c3240a9abd984ca0c6a2cd7389115e80d5&absolute=false&stat=instructions%3Au)

0 regresiones, 0 mejoras, 4 mixtas; Uno de ellos en rollups
17 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-05-17.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [RFC de carga para la edad mínima de publicación](https://github.com/rust-lang/rfcs/pull/3923)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Eliminando el sabor inestable del enlazador ptx](https://github.com/rust-lang/compiler-team/issues/990)
* [Crear un nuevo objetivo de Nivel 3: 'powerpc64le-desconocido-ninguno'](https://github.com/rust-lang/compiler-team/issues/988)
* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Interpolación de documentación](https://github.com/rust-lang/rfcs/pull/3962)

## Próximos eventos

Eventos Rusty entre el 20-05-2026 - el 17-06-2026 🦀

### Virtual
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/548kbqhl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #4 - ¡Codificación de cápsulas en QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Cardiff, GB) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**¡Evento híbrido con Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314820642/)
* 2026-05-26 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Ver tu código - Una guía práctica para rastrear en Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/9v7hv2g1)
* 2026-06-02 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/ukfh0mcf)
* 2026-06-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-07 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-02 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/pegz5x4h)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ekws5nr4)
* 2026-06-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/314000478/)

### Asia
* 2026-06-02 | Pekín, CN | [Voice AI y Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Agentes de IA y LLM de código abierto (Llamada a ponentes)**](https://www.meetup.com/wasm-rust-meetup/events/314750465/)

### Europa
* 2026-05-18 - 2026-05-23 | Utrecht, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-21 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Ruta a pie por Utrecht**](https://www.meetup.com/rust-nederland/events/314770275/)
* 2026-05-22 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Tour en bicicleta por Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Programación Agente - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Mayo de Rust Manchester**](https://www.meetup.com/rust-manchester/events/314452972/)
* 2026-05-26 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim/events/)
    * [**Persianas motorizadas, y reemplazando a Docker, en Rust!**](https://www.meetup.com/rust-trondheim/events/314711434/)
* 2026-05-28 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN habla de la muestra comunitaria de mayo**](https://www.meetup.com/rust-london-user-group/events/314846861/)
* 2026-05-29 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396588/)
* 2026-06-03 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Únete en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-11 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-16 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Interactivo: Todo es de código abierto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)

### Norteamérica
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, EE. UU. [Encuentro de Rust en el Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Rust en el Área de la Bahía**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: "Almacenamiento de archivos aburrido" y "Optimización del feed de noticias indie"**](https://www.meetup.com/rust-nyc/events/314783868/)
* 2026-05-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro comunitario**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 23 de mayo**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust en sistemas embebidos y autónomos en sistemas paralelos en DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 2026-05-28 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314716463/)
* 30-05-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Central de Cambridge, 30 de mayo**](https://www.meetup.com/bostonrust/events/314480537/)
* 2026-06-04 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Pruebas, Cobertura, Tracey y Mutaciones**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de la Óxida Común de Boston, 6 de junio**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Encuentro de junio de Utah Rust**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust June Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)

### Oceanía
* 2026-05-26 | Barton, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién contrata en r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Publicaciones como esta son útiles para quienes nos gusta ayudar y que trabajamos en rustc para hacerlo más útil, permitiéndonos aprender qué tipo de errores comete la gente.

– [Kevin Reid sobre usuarios de Rust](https://users.rust-lang.org/t/slightly-surprising-behavior-of-a-while-loop/140117/5)

¡Gracias a [firebits.io](https://users.rust-lang.org/t/crate-of-the-week/2704/1605) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1tj8ja6/this_week_in_rust_652/)</small>
