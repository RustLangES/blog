---
title: "Esta semana en Rust #100"
number_of_week: 100
description: El crate de esta semana es office2pdf.
date: 2026-03-04
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
* [Resultados de la Encuesta sobre el Estado de la Óxida 2025](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/)

### Boletines
* [El Rustacean Incrustado Número #66](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-66)

### Actualizaciones de proyectos/herramientas
* [Compendio: Añadiendo eBPF para visibilidad a nivel de núcleo](https://pker.xyz/posts/compendium-ebpf)
* [Migración de mensajería del Danubio desde ETCD](https://dev-state.com/posts/migrate_danube_etcd_to_raft/) 
* [Feedr v0.4.0 - Lector de feeds RSS basado en terminal](https://github.com/bahdotsh/feedr/releases/tag/v0.4.0)
* [dag_exec: Ejecutor DAG para pipelines con mucha CPU](https://www.reymom.xyz/blog/rust/2026-03-03-exec_dag-official-release)
* [Supercargar funciones de Rust con argumentos implícitos usando CGP v0.7.0](https://contextgeneric.dev/blog/v0.7.0-release/)
* [vscreen: navegador de agentes de IA](https://dev.to/lowjax/i-built-a-tool-that-lets-ai-agents-browse-the-real-internet-and-you-can-watch-them-do-it-2fff)
* [Ply 1.0: Crear aplicaciones en Rust no debería ser tan difícil](https://plyx.iz.rs/blog/introducing-ply/)

### Observaciones/Pensamientos
* [Usando Rust y Postgres para todo: patrones aprendidos a lo largo de los años](https://kerkour.com/rust-postgres-everything)
* [Kovan: De sistemas MVCC de producción a recuperación de memoria sin espera](https://vertexclique.com/blog/kovan-from-prod-to-mr/)
* [Nunca pospongas un futuro](https://jacko.io/snooze.html)
* [Abstracciones de coste cero de Rust vs. SIMD](https://turbopuffer.com/blog/zero-cost)
* [Nadie fue despedido por usar una struct](https://www.feldera.com/blog/nobody-ever-got-fired-for-using-a-struct)
* [Problemas de depuración de reproducibilidad en software Rust](https://notes.8pit.net/notes/iqfs.html)
* [Diseño de contrapresión en un ejecutor DAG paralelo](https://www.reymom.xyz/blog/rust/2026-02-21-backpressure-in-parallel-executor)
* [Prueba de invariantes de concurrencia en un albacea paralelo](https://www.reymom.xyz/blog/rust/2026-02-24-testing-invariants-atomics)
* [audio] [Netstack.FM episodio 29 — Hyper With Sean McArthur (Ep 2 Remasterizado)](https://netstack.fm/#episode-29)

### Guías de Rust
* [Tutorial: hagamos un Pi Spigot reanudable con SQLite](https://www.sea-ql.org/blog/2026-02-28-sea-orm-sync/)
* [El viaje migratorio de Apache Iggy hacia la arquitectura hilos por núcleo impulsada por io_uring](https://iggy.apache.org/blogs/2026/02/27/thread-per-core-io_uring/)
* [Métodos formales para el lado inseguro de la Fuerza](https://antithesis.com/blog/2026/rust_formal_methods/)
* [Cuantificación del impuesto matrimonial suizo](https://gendx.dev/blog/2026/03/02/swiss-marriage-tax.html)
* [Python rápido con Rust: un enfoque orientado a datos](https://hackeryarn.com/post/fast-python-with-rust/)
* [vídeo] [Rust: compilando a WASM para crear un juego basado en navegador usando canvas](https://artificialworlds.net/blog/2026/02/27/wasm-game/)
* [vídeo] [Entrevista con Daniel Almeida, escribiendo un controlador de kernel para GPU Linux en Rust](https://youtu.be/rgjTPBRae6I)

### Miscelánea
* [Actualización de TokioConf: Qué esperar](https://tokio.rs/blog/2026-03-03-tokioconf-update)

## Crate de la semana

El crate de esta semana es [office2pdf](https://github.com/developer0hye/office2pdf), una biblioteca independiente o binario para generar PDF a partir de archivos OOXML (docx, xlsx, etc.).

¡Gracias a [One](https://users.rust-lang.org/t/crate-of-the-week/2704/1562) por la sugerencia!

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
*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Conferencia Rust India 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP abierto hasta el 14-03-2026 | Bangalore, IN | 2026-04-18
* [**Conferencia Oxid**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

414 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-24..2026-03-03

#### Compilador
* [mejorar las funciones de forzamiento/promoción en 'DepKindVTable'](https://github.com/rust-lang/rust/pull/153122)
* [codegen: Restaurar 'noundef' en 'PassMode::Cast' Args en Rust ABI](https://github.com/rust-lang/rust/pull/152864)

#### Biblioteca
* ['BTreeMap::merge' optimizado](https://github.com/rust-lang/rust/pull/152418)
* [crear primitivas atómicas tipo alias de 'Atómica<T>'](https://github.com/rust-lang/rust/pull/153015)
* [camino rápido neón para 'str::contains'](https://github.com/rust-lang/rust/pull/152176)
* [preparar 'NonNull' para los tipos de patrones](https://github.com/rust-lang/rust/pull/152702)
* [volver a añadir '#[inline]' a 'Eq::assert_fields_are_eq'](https://github.com/rust-lang/rust/pull/153157)
* [estabilizar el nuevo tipo 'RangeToInclusive'](https://github.com/rust-lang/rust/pull/152304)

#### Carga
* [corrección: Inyectar una edición en los guiones](https://github.com/rust-lang/cargo/pull/16678)
* [ayuda: mostrar manpage para comandos anidados](https://github.com/rust-lang/cargo/pull/16432)
* [host-config: corregir el pánico al compilar con host-config](https://github.com/rust-lang/cargo/pull/16674)
* [toml: mostrar versión requerida de Rust en error de edición inestable](https://github.com/rust-lang/cargo/pull/16653)
* [mejorar error de búsqueda en el espacio de trabajo parental msg](https://github.com/rust-lang/cargo/pull/16669)

#### Clippy
* [corrige 'cmp_owned' sugiere erróneamente en 'PathBuf'](https://github.com/rust-lang/rust-clippy/pull/16628)
* [fijar 'explicit_counter_loop' falso positivo cuando el inicializador no es integral](https://github.com/rust-lang/rust-clippy/pull/16647)
* [corregir 'suboptimal_flops' falso negativo al añadir y subasignar](https://github.com/rust-lang/rust-clippy/pull/16625)
* [Asalto de núcleo en todos los formatos de lints](https://github.com/rust-lang/rust-clippy/pull/16597)

#### Analizador de Rust
* [detectar E0804 al lanzar ptr-to-dyn sin editar añade rasgos automáticos](https://github.com/rust-lang/rust-analyzer/pull/21699)
* [no te pongas nervioso por notificaciones LSP inválidas](https://github.com/rust-lang/rust-analyzer/pull/21708)
* [corregir la sangría del examinado por 'replace_if_let_with_match'](https://github.com/rust-lang/rust-analyzer/pull/21698)
* [sin calificativo completo de variante 'enum' en PAT](https://github.com/rust-lang/rust-analyzer/pull/21706)
* [usar 'ExprIsRead::Yes' para la derecha de los operadores binarios](https://github.com/rust-lang/rust-analyzer/pull/21654)
* [implementa 'Span::SpanParent' para proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21669)

### Triaje de rendimiento del compilador Rust

Una semana positiva con algunas mejoras agradables gracias a las limpiezas de los sistemas de consultas.

Triaje hecho por **@panstromek**.
Rango de revisión: [eeb94be7.. ddd36bd5](https://perf.rust-lang.org/?start=eeb94be79adc9df7a09ad0b2421f16e60e6d932c&end=ddd36bd57051f796850345b76c17e9402e28a9e4&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,3%, 0,3%] | 1 |
| Regresiones ❌ <br /> (secundario) | 0,2% | [0,0%, 0,3%] | 3 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-2,1%, -0,1%] | 141 |
| Mejoras ✅ <br /> (secundario) | -1,1% | [-6,6%, -0,1%] | 90 |
| Todos ❌✅ (primario) | -0,8% | [-2,1%, 0,3%] | 142 |

2 regresiones, 5 mejoras, 5 mixtas; 4 de ellos en rollups
30 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/06a788cbc715e02d77e998eefe5ad6d20bf95855/triage/2026/2026-03-02.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Siempre comprueba 'ConstArgHasType' incluso cuando lo ignores](https://github.com/rust-lang/rust/pull/152931)
* [Siempre hacer que los elementos de la tupla sean un sitio de coacción](https://github.com/rust-lang/rust/pull/147834)
* [denegar por defecto & reportar en deps 'uninhabited_static'](https://github.com/rust-lang/rust/pull/152853)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Eliminar soft_unstable](https://github.com/rust-lang/compiler-team/issues/972)
* [Analizar palabras clave inestables para sintaxis experimental](https://github.com/rust-lang/compiler-team/issues/945)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Aplicación de mitigaciones](https://github.com/rust-lang/rfcs/pull/3855)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 04-03-2026 - el 01-04-2026 🦀

### Virtual
* 2026-03-04 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 4: Manejo de módulos en un proyecto**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313526020/)
* 2026-03-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313303094/)
* 05-03-2026 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Presentación: Tock OS Parte #3 - Cápsulas y controladores de hardware de nivel inferior**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 05-03-2026 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763908777)
* 2026-03-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/312799450/)
* 2026-03-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/cgzfpzcp)
* 2026-03-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/313490537/)
* 2026-03-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-18 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**¡Evento híbrido con Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313621933/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/45qqc2eo)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Adopción de Rust, Seguridad y Nube con Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Cajas, consejos y trucos Charlas relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcgbcb/)

### Asia
* 222-03-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**Rust presencial marzo 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)

### Europa
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**Rust en MWC Talent Arena — Talleres + Encuentro Comunitario**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburgo, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg)
    * [**Rust Hack & Learn marzo 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in March: Abstracciones, pero ¿a qué precio?**](https://www.meetup.com/rustcologne/events/313532986/)
* 2026-03-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Discos, Destrozados sobre Hielo: Una introducción al parquet y el iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)
* 2026-03-04 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #83**](https://www.meetup.com/rust-paris/events/313493454/)
* 05-03-2026 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Hack'n'Learn de Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/313464558/)
* 2026-03-11 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/313426708/)
* 2026-03-11 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo un compilador de Python en Rust**](https://www.meetup.com/rust-rhein-main/events/313617138/)
* 2026-03-12 | Berna, CH | [Bern Oxidado](https://www.meetup.com/rust-bern)
    * [**2026 Rust Talks Bern #1 @bespinian**](https://www.meetup.com/rust-bern/events/313443248/)
* 2026-03-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Introducción a Embedded Rust - Marzo**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-2026 | Varsovia, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Conferencia Rustikon**](https://www.rustikon.dev/)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack - Advenimiento del Código**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Marcha de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-27 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París**](https://www.rustinparis.com/)
* 2026-04-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)

### Norteamérica
* 2026-03-04 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: ¡Colector de métricas personalizadas y Rust incrustado!**](https://www.meetup.com/rust-nyc/events/313499010/)
* 05-03-2026 | Chicago, IL, EE. UU. [Encuentro de Chicago Rust](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora Feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/313529755/)
* 05-03-2026 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST ENCUENTRO en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313305800/)
* 05-03-2026 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Noche del Proyecto Rust**](https://www.meetup.com/stl-rust/events/312654992/)
* 2026-03-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, 7 de marzo**](https://www.meetup.com/bostonrust/events/313208584/)
* 2026-03-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Un intérprete para la teoría de la computabilidad, escrito a la manera difícil**](https://www.meetup.com/utah-rust/events/313506767/)
* 2026-03-14 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo North End Rust, 14 de marzo**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313404095/)
* 2026-03-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust aplicado - Aplicaciones de Rust en edificios**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Porter Square Rust, 21 de marzo**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/xvkdgtyjcfbhc/)
* 2026-03-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228658/)

### Oceanía
* 2026-03-12 | Ciudad de Brisbane, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Oxidar Brisbane Mar 2026**](https://www.meetup.com/rust-brisbane/events/313596218/)
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Encuentro de marzo por determinar**](https://www.meetup.com/rust-melbourne/events/313471749/)

### Sudamérica
* 2026-03-21 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP (migrado pro Lumma)**](https://www.meetup.com/rust-sao-paulo-meetup/events/313446400/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Al fin y al cabo, Rust solo se volvió tan bueno tras pasar por una transformación bastante drástica. En su momento tuvo una GC y Green Threads, famosamente. No hay sustituto para que exista y ver cómo se comporta en un problema real.

– [scottmcm sobre Rust-users](https://users.rust-lang.org/t/aliased-xor-mutable-core-for-a-high-level-language/138482/22)

¡Gracias a [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1755) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1rm06mx/this_week_in_rust_641/)</small>
