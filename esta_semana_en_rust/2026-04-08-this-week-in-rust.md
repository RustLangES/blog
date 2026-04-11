---
title: "Esta semana en Rust #105"
number_of_week: 105
description: El crate de esta semana es aimdb-core, una cadena de datos segura de tipos y independiente de la plataforma, donde el sistema de tipos Rust es el esquema y las implementaciones de rasgos definen su comportamiento.
date: 2026-04-08
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
* [docs.rs: construir menos objetivos por defecto](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)
* [Cambios en los objetivos de WebAssembly y manejo de símbolos indefinidos](https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/)
* [Actualización del Consejo de Liderazgo — marzo de 2026](https://blog.rust-lang.org/inside-rust/2026/04/06/leadership-council-update/)

### Fundación
* [¿Qué sigue para el Laboratorio de Innovación en Rust?](https://rustfoundation.org/media/whats-next-for-the-rust-innovation-lab/)
* [Actualización de la Iniciativa de Interop de la Fundación Rust: De la investigación a la implementación](https://rustfoundation.org/media/rust-foundation-interop-initiative-update-from-research-to-implementation/)

### Boletines
* [Este mes en Rust OSDev: marzo de 2026](https://rust-osdev.com/this-month/2026-03/)

### Actualizaciones de proyectos/herramientas
* [Seguro-seguro](https://notes.brooklynzelenka.com/Blog/Surelock)
* [Actualización de progreso de Rust for CPython abril 2026](https://blog.python.org/2026/04/rust-for-cpython-2026-04/)
* [RustRover 2026.1: Pruebas profesionales con integración nativa de carga y siguiente](https://blog.jetbrains.com/rust/2026/04/03/rustrover-2026-1-professional-testing-with-native-cargo-nextest-integration/)
* [Hoty, un ORM asincrónico para Rust, está ahora en crates.io](https://tokio.rs/blog/2026-04-03-toasty-released)
* [slopc: El maldito macro que los mantenedores de Rust nunca anticiparon](https://dev.to/shorwood/slopc-the-curse-rust-maintainers-never-anticipated-1j67)
* [Procesando 1M de partidas de ajedrez en 15 segundos con Rust](https://dev.to/chrnx/processing-1m-chess-games-in-15-seconds-with-rust-pe3)
* [Dumap v1.1: Visualización del mapa de árbol de uso de disco multiplataforma](https://github.com/jrobhoward/dumap/releases/tag/v1.1.0)
* [Proxelar 0.4.0: Interceptar y modificar el tráfico](https://micheletti.io/proxelar-040/)
* [amoxide: alias de shell componibles y conscientes del contexto](https://d34dl0ck.me/amoxide-composable-context-aware-shell-aliases/index.html)
* [Ply 1.1: Construcción de interfaces pulidas en Rust](https://plyx.iz.rs/blog/ply-1-1/)
* [Myth Engine: Un RenderGraph de estilo compilador para renderizado multiplataforma](https://github.com/panxinmiao/myth/blob/main/CHANGELOG.md)
* [selinux-explain](https://dev.to/mattiabandini1/i-got-tired-of-setenforce-0-so-i-built-a-tool-in-rust-to-actually-understand-selinux-denials-24ie)

### Observaciones/Pensamientos
* [Análisis de callgraph](https://ferrous-systems.com/blog/callgraph-analysis/)
* [Solucionando nuestros propios problemas en el compilador de Rust](https://tweedegolf.nl/en/blog/234/fixing-our-own-problems-in-the-rust-compiler)
* [800 proyectos de terminales de Rust en 3 años](https://blog.orhun.dev/800-rust-projects/)
* [Lo que aprendimos construyendo un runtime de Rust para TypeScript](https://encore.dev/blog/rust-runtime)
* [Pesadilla de la cadena de suministro: Cómo atacarán Rust y qué podemos hacer para mitigar lo inevitable](https://kerkour.com/rust-supply-chain-nightmare)
* [Construcción de un grafo de renderizado declarativo basado en SSA en Rust](https://github.com/panxinmiao/myth/blob/main/docs/RenderGraph.md)
* [audio] [Protocolo Corto: TLS Cliente Cifrado Hola](https://netstack.fm/#episode-33)

### Guías de Rust
* [Aprende lo básico de Rust construyendo un intérprete de Brainfuck](https://blog.sheerluck.dev/posts/learn-rust-basics-by-building-a-brainfuck-interpreter/)
* [Cómo funciona el UV bajo el capó](https://noos.blog/posts/uv-how-it-works-under-the-hood/)
* [Construcción de compatibilidad con Postgres en Rust: pgwire y DataFusion](https://greptime.com/blogs/2026-04-01-greptimedb-postgresql-compatibility)
* [vídeo] [impl Rust: generador de ruido WAV](https://www.youtube.com/watch?v=zOTE4BN59u4)

### Miscelánea
* [vídeo] [Desafíos de interoperabilidad Rust/C++ - Victor Ciura - CppCon 2025](https://www.youtube.com/watch?v=8xqhSy539Pc)

## Crate de la semana

El crate de esta semana es [aimdb-core](https://crates.io/crates/aimdb-core), una cadena de datos segura de tipos y independiente de la plataforma, donde el sistema de tipos Rust es el esquema y las implementaciones de rasgos definen su comportamiento.

¡Gracias a [sounds.like.lx](https://users.rust-lang.org/t/crate-of-the-week/2704/1583) por la autosugerencia!

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

* [**Almendras Silvestres — Biblioteca compartida de reproductor de audio para flutter y tauri**](https://github.com/opeolluwa/almonds/issues/144)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | CFP abierto hasta el 14-04-2026 | Kongsberg, Noruega | 2026-09-09 - 2026-09-12.
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

479 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-31..2026-04-07

#### Compilador
* [calcular el resultado de un tipo de proyección con errores de región](https://github.com/rust-lang/rust/pull/153105)
* [hacer que 'layout_of' consiga errores fatales](https://github.com/rust-lang/rust/pull/153960)
* [generalizar correctamente las consts no evaluadas](https://github.com/rust-lang/rust/pull/154053)

#### Biblioteca
* [métodos de truncamiento entero y de extensión de la suma de enteros](https://github.com/rust-lang/rust/pull/154356)
* [visualizadores depuradores: optimizar el comportamiento de búsqueda](https://github.com/rust-lang/rust/pull/147552)
* [impl 'Paso' para 'NonZero<u*>'](https://github.com/rust-lang/rust/pull/127534)
* [introduce '#[diagnostic::on_move]' en 'Arc'](https://github.com/rust-lang/rust/pull/154669)
* [hacer que 'substr_range' y 'subslice_range' devuelvan el nuevo tipo de 'Rango'](https://github.com/rust-lang/rust/pull/154707)
* [más informativo 'Debug for vec::ExtractIf'](https://github.com/rust-lang/rust/pull/154581)
* [estabilizar nuevo tipo de rango e iterador](https://github.com/rust-lang/rust/pull/154620)

#### Carga
* ['lints': Emite pelusa de 'unused_dependencies'](https://github.com/rust-lang/cargo/pull/16600)
* ['compilar': No ocultes advertencias duras con build.warnings=permite](https://github.com/rust-lang/cargo/pull/16827)
* ['compile': build.warnings=allow no debe ocultar diagnósticos denegados](https://github.com/rust-lang/cargo/pull/16824)
* ['instalar': Ignorar resolver.lockfile-path](https://github.com/rust-lang/cargo/pull/16823)
* [añadir opción de perfil de punteros de fotograma](https://github.com/rust-lang/cargo/pull/16742)
* [establecer var de envío CARGO durante la sonda rustc -vV](https://github.com/rust-lang/cargo/pull/16811)
* [enviar encabezado de Type-Contenido con solicitudes de publicación de carga](https://github.com/rust-lang/cargo/pull/16832)
* [nombres simplificados de bin de script de compilación en un nuevo diseño](https://github.com/rust-lang/cargo/pull/16812)
* [separar 'Cargo-util-terminal'](https://github.com/rust-lang/cargo/pull/16809)
* [advertencia sobre descriptores de archivo de jobserver inválidos](https://github.com/rust-lang/cargo/pull/16843)

#### Clippy
* ['unsafe_removed_from_name'': salta el linting al renombrar a ''_''](https://github.com/rust-lang/rust-clippy/pull/16802)
* [cast de tipos innecesario causando un error de compilación](https://github.com/rust-lang/rust-clippy/pull/16796)
* [múltiples correcciones de falsos negativos de 'question_mark'](https://github.com/rust-lang/rust-clippy/pull/16769)
* [perf: desactiva 'nonminimal_bool' por defecto](https://github.com/rust-lang/rust-clippy/pull/16761)
* [rehacer 'expr_use_ctxt' en un iterador sobre sitios de uso sucesivos](https://github.com/rust-lang/rust-clippy/pull/16784)
* [mejora innecesaria de la carta comodera](https://github.com/rust-lang/rust-clippy/pull/16733)

#### Analizador de Rust
* [añadir soporte para plegar rangos para expresiones encadenadas](https://github.com/rust-lang/rust-analyzer/pull/19659)
* [implementar completación de flecha fina en la posición de retorno de fn](https://github.com/rust-lang/rust-analyzer/pull/21012)
* [oferta en la expansión de cola con else-branch por asistencia de 'if_let_to_guarded'](https://github.com/rust-lang/rust-analyzer/pull/21912)
* [soporte etiquetado bloqueando para 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/21919)
* [soporte de expansión de macros en '#[doc = ...]' atributos](https://github.com/rust-lang/rust-analyzer/pull/21928)
* [fix extract function invalid self param](https://github.com/rust-lang/rust-analyzer/pull/20864)
* [añadir punto y coma para fragmentos de unidad en formato postfijo](https://github.com/rust-lang/rust-analyzer/pull/21955)
* [fija un ciclo en límites bajando](https://github.com/rust-lang/rust-analyzer/pull/21915)
* [fijar variable de extracción en arg con coma](https://github.com/rust-lang/rust-analyzer/pull/21936)
* [corregir sangría para 'convert_let_else_to_match'](https://github.com/rust-lang/rust-analyzer/pull/21938)
* [corregir inlay param Hints en expr y coma vacíos](https://github.com/rust-lang/rust-analyzer/pull/21926)
* [arreglar diagnósticos obsoletos cuando se configura un comando de comprobación personalizado](https://github.com/rust-lang/rust-analyzer/pull/21738)
* [arreglar el upmapping de 'SyntaxEditor' de nodos con ancestro mapeado que no están mapeados ellos mismos](https://github.com/rust-lang/rust-analyzer/pull/21962)
* [mejora el orden insertado para 'trait_impl_redundant_assoc_item'](https://github.com/rust-lang/rust-analyzer/pull/21695)
* [load rust-analyzer.toml para espacios de trabajo virtuales](https://github.com/rust-lang/rust-analyzer/pull/21704)
* [no sugiere nombre en el tipo anidado en variante](https://github.com/rust-lang/rust-analyzer/pull/21927)
* [ofrece ''type_mismatch'' algunas correcciones dentro de macro](https://github.com/rust-lang/rust-analyzer/pull/21952)
* [oferta en bloque de vacío para ''convert_let_else_to_match''](https://github.com/rust-lang/rust-analyzer/pull/21954)
* [informe 'tipo esperado, encontrado {' en analizador](https://github.com/rust-lang/rust-analyzer/pull/21951)
* [diagnóstico de desajuste de tipo silencioso cuando el tipo es desconocido](https://github.com/rust-lang/rust-analyzer/pull/21942)
* [soportan elementos de array CFG](https://github.com/rust-lang/rust-analyzer/pull/21935)
* [soportan sistemas de archivos que no envían eventos de Crear](https://github.com/rust-lang/rust-analyzer/pull/21844)
* [soporte múltiples marcadores de posición de fragmentos en la extensión VS Code](https://github.com/rust-lang/rust-analyzer/pull/21940)
* [pasar incondicionalmente '--incluye-ignorado' para pruebas ejecutables](https://github.com/rust-lang/rust-analyzer/pull/21921)
* [usar la raíz correcta del proyecto cuando hay varios espacios de trabajo](https://github.com/rust-lang/rust-analyzer/pull/21922)
* [Terminar los paréntesis de guardia para 'replace_if_let_with_match'](https://github.com/rust-lang/rust-analyzer/pull/21937)
* [impl Tipo de visualización de la inserción de pista al final de la línea](https://github.com/rust-lang/rust-analyzer/pull/21322)
* [implementar 'funcionalidad(more_qualified_paths)'](https://github.com/rust-lang/rust-analyzer/pull/19956)
* [hacer que el soporte coincidente funcione cuando el cursor no está entre corchetes](https://github.com/rust-lang/rust-analyzer/pull/21792)
* [trasladar la responsabilidad de mutabilidad de la persona que llama a 'edit_algo'](https://github.com/rust-lang/rust-analyzer/pull/21931)
* [Iniciación del editor de movimientos de sintaxis invariante a su constructor](https://github.com/rust-lang/rust-analyzer/pull/21960)
* [publicar no-server en Code Marketplace y OpenVSX](https://github.com/rust-lang/rust-analyzer/pull/21516)
* [Sustituye añadir objetos asociados a rasgos a impl por su variante de fábrica](https://github.com/rust-lang/rust-analyzer/pull/21930)

### Triaje de rendimiento del compilador Rust

Una semana más corta de lo normal (probablemente por un triaje de rendimiento más tardío la semana pasada).
En general, cambios bastante pequeños dispersos entre distintos PR, aunque la red
El efecto fue ligeramente positivo (-0,5% de cambio de media). Al final, todo cambió
Mixtas o mejoras esta semana.

Triaje hecho por **@simulacrum**.
Rango de revisión: [cf7da0b7.. e73c56ab](https://perf.rust-lang.org/?start=cf7da0b7277cad05b79f91b60c290aa08a17a6f0&end=e73c56abd0baf3dbaafbdc3ce6072a416aade867&absolute=false&stat=instructions%3Au)

0 regresiones, 3 mejoras, 8 mixtas; 5 de ellos en rollups
26 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-04-05.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [informa de la pelusa 'varargs_without_pattern' en deps](https://github.com/rust-lang/rust/pull/154599)
* [Estabilizar parcialmente las características objetivo de LoongArch](https://github.com/rust-lang/rust/pull/154510)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* *No hubo problemas de seguimiento de carga ni relaciones públicas que entraron en el Periodo Final de Comentarios esta semana.*

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [limpieza de carga: Añadir validación de directorio objetivo](https://github.com/rust-lang/cargo/pull/16712)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [Propone el concepto de un nombre de usuario crates.io para la identidad](https://github.com/rust-lang/rfcs/pull/3946)
* [RFC: Heredamiento de funciones predeterminadas en Cargo](https://github.com/rust-lang/rfcs/pull/3945)
* [Añadir inicio de sesión OAuth de Bitbucket Cloud para crates.io](https://github.com/rust-lang/rfcs/pull/3944)
* [MOVIMIENTO MIR](https://github.com/rust-lang/rfcs/pull/3943)

## Próximos eventos

Eventos Rusty entre el 8-04-2026 - el 06-05-2026 🦀

### Virtual
* 2026-04-09 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-14 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Desarrollo web usando axum en Rust - parte 3**](https://www.meetup.com/code-mavens/events/314072969/)
* 2026-04-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/jia7wtfv)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/314007434/)
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: De Protobuf a Producción - Guía para gRPC en Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Caminata de Hacker 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjchbjb/)

### Asia
* 2026-04-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Abril 2026/Encuentro previo a la conferencia de los Rustacean**](https://hasgeek.com/rustbangalore/april-2026-pre-conference-rustacean-meetup/)
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Taller de Rust India**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Conferencia Rust India**](https://rustindia.org/schedule)

### Europa
* 2026-04-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2026**](https://luma.com/z8aoscr9)
* 2026-04-09 | Ginebra, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust habla @ AutoStore – "Patrones para sistemas impulsados por eventos" y "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**GUI nativas con Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla y fiesta de cumpleaños en MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-04-24 - 2026-04-26 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Semana del Futuro Augsburgo: Camino al Game Jam – Spielend Bevy und Rust lernen bei Tuxedo Computers**](https://www.tuxedocomputers.com/de/Road-to-Game-Jam-2026-Bevy-Workshop.tuxedo)
* 2026-05-02 | Augsburgo, DE | [Rust Múnich](https://rust-munich.de/) y [Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Augsburger Linux-Infotag 2026: Gemeinschaftsstand Rust Augsburg und Rust München**](https://www.luga.de/static/LIT-2026/)
* 2026-05-04 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo una simulación de cartera de acciones en Rust con Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)

### Norteamérica
* 2026-04-09 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz Oxid**](https://www.meetup.com/chicago-rust-meetup/events/313987321/)
* 2026-04-09 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de abril de Utah Rust**](https://www.meetup.com/utah-rust/events/314156736/)
* 2026-04-09 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Fiesta Mensual de abril**](https://www.meetup.com/rust-montreal/events/313872135/)
* 2026-04-09 | Portland, OR, EE. UU. [PDXRust](https://www.meetup.com/pdxrust)
    * [**¿Punto flotante hexadecimal? ¿Por qué? ¿Cómo?**](https://www.meetup.com/pdxrust/events/314159131/)
* 2026-04-09 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust April Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Brookline Rust, 11 de abril**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, EE. UU. [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Afinando tus habilidades de Rust para entrevistas de trabajo**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-16 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313995065/)
* 2026-04-16 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro Comunitario**](https://www.meetup.com/music-city-rust-developers/events/314090462/)
* 2026-04-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en la Plaza de Harvard, 18 de abril**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-22 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Solucionadores de Rust y SAT Verificados formalmente**](https://www.meetup.com/rust-nyc/events/314167944/)
* 2026-04-23 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Oxida LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Estación Sur, 25 de abril**](https://www.meetup.com/bostonrust/events/313883704/)
* 30-04-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228662/)

### Oceanía
* 2026-04-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Oxidar Brisbane abr 2026**](https://www.meetup.com/rust-brisbane/events/313975190/)

### Sudamérica
* 2026-04-11 | Buenos Aires, AR | [Oxidar Org](https://luma.com/user/oxidar)
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

> Rust *intentó* tener genéricos polimórficos en los primeros días previos a la versión 1.0, y con razón *se rindieron* porque era demasiado trabajo. De verdad, Swift, ¡genial trabajo para que todo esto funcione!

– [Aria Desires en su blog](https://faultlore.com/blah/dsts-are-polymorphic-generics/#polymorphic-compilation-of-generics)

[llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1757) ¡se agradece a sí mismo por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1sgc0kt/this_week_in_rust_646/)</small>
