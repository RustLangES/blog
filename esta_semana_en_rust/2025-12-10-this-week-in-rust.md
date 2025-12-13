---
title: "Esta semana en Rust #88"
number_of_week: 88
description: El crate de esta semana es mdbook-lint, un linter de descuento pensado para mdbook, pero útil con cualquier descuento.
date: 2025-12-10
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
* [Lecciones aprendidas del proceso de Rust Vision Doc](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/)
* [Actualizando los objetivos de Rust para Linux a la versión 1.2.5](https://blog.rust-lang.org/2025/12/05/Updating-musl-1.2.5/)
* [Facilitando el patrocinio de colaboradores de Rust](https://blog.rust-lang.org/2025/12/08/making-it-easier-to-sponsor-rust-contributors/)

### Boletines
* [El Rustaceano Incrustado Número #60](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-60)

### Actualizaciones de proyectos/herramientas
* [Tejo 0,22 - Esta vez de verdad](https://yew.rs/blog/2025/11/29/release-0-22)
* [¡Ferroceno 25.11.0 ya disponible!](https://ferrous-systems.com/blog/ferrocene-25-11-0/)
* [YM2149-rs 0.8.0 — Ecosistema chiptune YM2149/AY-3-8910 con precisión en el ciclo](https://ym2149-rs.org/)
* [¡Los nuevos superpoderes de Duper!](https://duper.dev.br/blog/duper-s-new-superpowers.html)
* [Anunciando el lanzamiento de redis-rs 1.0.0](https://github.com/redis-rs/redis-rs/blob/main/version1.md)
* [Anunciando diesel-guard 0.2.0: Detectar migraciones inseguras de PostgreSQL antes de que entren en producción](https://www.reddit.com/r/rust/comments/1phx68w/dieselguard_catch_unsafe_postgresql_migrations/)
* [vídeo] [Grabación del encuentro de Rust Seúl: Zia, un lenguaje de programación que se define a sí mismo (escrito en Rust)](https://www.youtube.com/watch?v=LbFTP3pITWU)

### Observaciones/Pensamientos
* [Explorando deboa-macros: Macros ergonómicas de cliente HTTP para Rust](https://medium.com/@ararog/exploring-deboa-macros-ergonomic-http-client-macros-for-rust-d0c3df22e0a7)
* [Luchando contra el Monstruo Espagueti Cliente con Rasgos de Rust](http://gnunicorn.org/writings/spaghetti-monster-clients-rust-traits-final-boss/)
* [Pruebas unitarias de Rust: lectura de archivos en búfer](https://jorgeortiz.dev/posts/rust_unit_testing_file_buf_reading/)
* [Inmersión profunda de Firecracker: Cómo el Rust y las microVMs están revolucionando la infraestructura en la nube](https://kerkour.com/firecracker-deep-dive-rust)
* [iksemel oxidado](https://thinkerf.blogspot.com/2025/12/iksemel-rusted.html)
* [Macros de postfijo y 'let place'](https://nadrieril.github.io/blog/2025/12/09/postfix-macros-and-let-place.html)
* [¿Deberíamos deshacernos de clippy::manual_try_fold?](https://blog.veeso.dev/blog/en/should-we-get-rid-of-clippy-manual-try-fold/)

### Guías de Rust
* [Haz cola - cola SPSC superrápida](https://abhikja.in/blog/2025-12-07-get-in-line/)
* [Emulando los intrínsecos del avx-512 en Miri](https://trifectatech.org/blog/emulating-avx-512-intrinsics-in-miri/)
* [Cómo acelerar el compilador de Rust en diciembre de 2025](https://nnethercote.github.io/2025/12/05/how-to-speed-up-the-rust-compiler-in-december-2025.html)
* [De árboles a gráficos: acelerando la búsqueda vectorial 10x con Hannoy](https://blog.kerollmops.com/from-trees-to-graphs-speeding-up-vector-search-10x-with-hannoy)
* [serie] [Parte 1: Tokenización, Construcción de un LLM desde cero en Rust](https://www.tag1.com/white-paper/part1-tokenization-building-an-llm-from-scratch-in-rust/)

### Miscelánea
* [Pydantic: La pitón que ama el Rust](https://filtra.io/rust/interviews/pydantic-dec-25)
* [vídeo] [AWS re:Invent 2025 - Desata el potencial de Rust en AWS (DEV307)](https://www.youtube.com/watch?v=buBBQ5mXAi8)

## Crate de la semana

El crate de esta semana es [mdbook-lint](https://github.com/joshrotenberg/mdbook-lint), un linter de descuento pensado para mdbook, pero útil con cualquier descuento.

¡Gracias a [josh rotenberg](https://users.rust-lang.org/t/crate-of-the-week/2704/1502) por la autosugerencia!

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

* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 31-12-2025 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

494 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-02..2025-12-09

#### Compilador
* [retorno anticipado en descensos duplicados de vano](https://github.com/rust-lang/rust/pull/149060)
* [MIRI: soporte 'fstat' en Linux](https://github.com/rust-lang/miri/pull/4714)

#### Biblioteca
* ['c_variadic': hacer que 'VaList' sea compatible con C](https://github.com/rust-lang/rust/pull/141980)
* [añadir '#[inline]' a 'Layout::is_size_align_valid'](https://github.com/rust-lang/rust/pull/149690)
* [añadir 'Opción::into_flat_iter'](https://github.com/rust-lang/rust/pull/148487)
* [también introduce 'Peekable::next_if_map_mut' junto a 'next_if_map'](https://github.com/rust-lang/rust/pull/149520)
* [asumir el valor devuelto en '.filter(...). contar()'](https://github.com/rust-lang/rust/pull/149495)
* [implementa 'Allocator' para '&mut A' donde 'A: Allocador + ? Sized'](https://github.com/rust-lang/rust/pull/146826)
* [implementa 'Vec::from_fn'](https://github.com/rust-lang/rust/pull/149699)
* [eliminar el seguimiento de bytes inicializados de 'BorrowedBuf' y 'BorrowedCursor'](https://github.com/rust-lang/rust/pull/148937)
* [estabilizar 'array_windows'](https://github.com/rust-lang/rust/pull/148814)

#### Carga
* ['pelusa': nueva pelusa de 'implicit_minimum_version_req'](https://github.com/rust-lang/cargo/pull/16321)
* ['tiempos': derivar datos de concurrencia a partir de datos unitarios](https://github.com/rust-lang/cargo/pull/16350)
* ['lints': manejar lints por separado a nivel de WS pkg](https://github.com/rust-lang/cargo/pull/16367)
* ['limpia': Optimizar (legacy) clean con múltiples especificadores -p](https://github.com/rust-lang/cargo/pull/16264)
* [no leas el archivo de configuración dos veces cuando '$CARGO_HOME' es un enlace simbólico](https://github.com/rust-lang/cargo/pull/16325)
* [soporte para información cross-crate fusionable de rustdoc](https://github.com/rust-lang/cargo/pull/16309)

#### Clippy
* ['len_without_is_empty': permite 'is_empty(y yo)' con 'len(& mut self)'](https://github.com/rust-lang/rust-clippy/pull/16194)
* [corregir 'map_entry' falso positivo cuando haría que 'MutexGuard' se mantuviera en un lugar](https://github.com/rust-lang/rust-clippy/pull/16199)
* [corregir 'nonstandard_macro_braces' falso negativo en macros con args vacíos](https://github.com/rust-lang/rust-clippy/pull/15601)
* [corregir 'panicking_unwrap' falso positivo en el acceso al campo con deref implícito](https://github.com/rust-lang/rust-clippy/pull/16196)
* [corregir 'tuple_array_conversions' falso positivo cuando se usan vars vinculadas antes de la conversión](https://github.com/rust-lang/rust-clippy/pull/16197)
* [corregir macros 'useless_conversion' mal desordenados](https://github.com/rust-lang/rust-clippy/pull/16171)
* [sugerencia de arreglar 'while_let_on_iterator' roto para tipos no dimensionados](https://github.com/rust-lang/rust-clippy/pull/16100)

#### Analizador de Rust
* [añadir configuración, ocultar marcadores de posición, tipis hints](https://github.com/rust-lang/rust-analyzer/pull/21203)
* [corregir 'hacer::unnamed_param' resultado un 'untyped_param'](https://github.com/rust-lang/rust-analyzer/pull/21044)
* [corregir expr anidado que falta punto y coma en incompleto-let](https://github.com/rust-lang/rust-analyzer/pull/21198)
* [corregir publicación en el campo variante 'enum' para 'no_such_field'](https://github.com/rust-lang/rust-analyzer/pull/21221)
* [permitir múltiples operaciones de descubrimiento](https://github.com/rust-lang/rust-analyzer/pull/21164)
* [no implementar comprobación de tamaño mediante 'all_field_tys()'](https://github.com/rust-lang/rust-analyzer/pull/21215)
* [corregir la completación en cadenas de formato](https://github.com/rust-lang/rust-analyzer/pull/21210)
* [Actualizado la visualización impl para mostrar args genéricos de rasgos](https://github.com/rust-lang/rust-analyzer/pull/21226)
* [más proto proto arreglos](https://github.com/rust-lang/rust-analyzer/pull/21195)
* [macro de atributo incorporado 'define_opaque', registrado](https://github.com/rust-lang/rust-analyzer/pull/21183)
* [resolver const generic param-env panic in type projection](https://github.com/rust-lang/rust-analyzer/pull/21235)
* [Saltar atributos CFG en el despojo de atributos de entrada de macro](https://github.com/rust-lang/rust-analyzer/pull/21205)
* [ningún RetType completo de unidad en el objeto asociado asíncrono de reazúcar](https://github.com/rust-lang/rust-analyzer/pull/21222)

### Triaje de rendimiento del compilador Rust

El resultado general es negativo esta semana, pero ambas regresiones principales están en camino de ser abordadas. No hay cambios pendientes por lo demás.

Triaje hecho por **@panstromek**.
Rango de revisión: [eca9d93f.. 55495234](https://perf.rust-lang.org/?start=eca9d93f9057f9a48ff691bd65e7daf2f94c1b67&end=554952348a7dd13851f25789f6bb1061f45c4b60&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,1%, 4,3%] | 111 |
| Regresiones ❌ <br /> (secundario) | 0,4% | [0,1%, 2,2%] | 97 |
| Mejoras ✅ <br /> (primaria) | -1,0% | [-1,3%, -0,7%] | 2 |
| Mejoras ✅ <br /> (secundario) | -0,2% | [-0,3%, -0,0%] | 9 |
| Todos ❌✅ (primario) | 0,4% | [-1,3%, 4,3%] | 113 |

3 regresiones, 2 mejoras, 3 mixtas; 3 de ellos en rollups
30 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/1e31d44e9db6e283552733052331af16e14e58e2/triage/2025/2025-12-08.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No hubo RFC entrando en el Periodo Final de Comentarios esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Advertencia sobre atributos de código en métodos de rasgos requeridos](https://github.com/rust-lang/rust/pull/148756)
* [NFC normaliza identificadores de por vida](https://github.com/rust-lang/rust/pull/149192)
* [no normalizan las cláusulas de donde al comprobar la buena formación](https://github.com/rust-lang/rust/pull/148477)

[Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Desestabilizar objetivo-especificación](https://github.com/rust-lang/compiler-team/issues/944)

[RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: '#[export_visibility = ...]' atributo](https://github.com/rust-lang/rfcs/pull/3834)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 10-12-2025 - el 07-01-2026 🦀

### Virtual
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/li5de4ts)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310728572/)
* 2025-12-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/6v2rorp3)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361448/)
* 2025-12-25 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046673/)
* 2026-01-01 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/306046646/)
* 2026-01-03 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 07-01-2026 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312102790/)

### Asia
* 2025-12-13 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**COMPLETO: Hack de fin de año con la llegada del código**](https://forms.gle/97jqPeGvpUjMXA8x9)
* 2025-12-14 | Pekín, CN | [Voice AI y Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GDPS语音AI agent WorkShop+Meetup**](https://www.meetup.com/wasm-rust-meetup/events/312264659/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de diciembre 2025**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)
* 2026-01-06 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**Rust en persona enero de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europa
* 2025-12-10 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**Rust LDN Habla: Fiesta de Navidad con los London Gophers y Red Badger**](https://www.meetup.com/rust-london-user-group/events/312264843/)
* 2025-12-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-11 | Ginebra, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-12-15 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim)
    * [**Oxidación del Hackathon**](https://www.meetup.com/rust-trondheim/events/312278650/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Noche de Hackeo de Rust #12: Advenimiento del Código**](https://www.meetup.com/copenhagen-rust-community/events/312295930/)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)
* 2025-12-19 | Lyon, FR | [Lyon Oxidado](https://www.meetup.com/rust-lyon)
    * [**Reunión de Rust Lyon #11**](https://www.meetup.com/rust-lyon/events/312180836/)
* 07-01-2026 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)

### Norteamérica
* 2025-12-10 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/312289655/)
* 2025-12-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Robótica Competitiva con Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-11 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de diciembre de 2025**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-13 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Alewife Rust, 13 de diciembre**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312076080/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Spokane, WA, EE. UU. [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro social de fin de año con grupos de usuarios locales de Python, Rust y otros**](https://www.meetup.com/spokane-rust/events/312292668/)
* 2025-12-20 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 20 de diciembre**](https://www.meetup.com/bostonrust/events/311917280/)
* 2026-01-01 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cancelado**](https://www.meetup.com/stl-rust/events/311396047/)

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

> \[.. \] Si va a ocurrir un cambio brusco, es mucho mejor hacer que el bloqueo entre en pánico automáticamente que que se desbloquee en silencio.

– [Rain en su blog](https://sunshowers.io/posts/on-poisoning)

¡Gracias a [hkBst](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1735) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1pjqjgs/this_week_in_rust_629/)</small>
