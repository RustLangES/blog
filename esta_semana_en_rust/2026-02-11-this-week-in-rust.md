---
title: "Esta semana en Rust #97"
number_of_week: 97
description: El crate de esta semana es zedbar, una caja para leer códigos QR y un montón de otros formatos de códigos de barras de imágenes.
date: 2026-02-11
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

### Boletines
* [GCC Front-End For Rust - Informe mensual de enero de 2026](https://rust-gcc.github.io/2026/02/10/2026-01-monthly-report.html)

### Actualizaciones de proyectos/herramientas
* [Fyrox 1.0.0-rc.2](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0-rc-2/)
* [Slint 1.15 Lanzado](https://slint.dev/blog/slint-1.15-released)
* [El futuro de Tyr](https://lwn.net/SubscriberLink/1055590/12d48275b6f81988/)
* [Rustbridge v0.9: Construcción e empaquetado de bibliotecas compartidas de Rust](https://github.com/jrobhoward/rustbridge/blob/v0.9.1/docs/RELEASE_NOTES_0.9.md)
* [Ariel OS v0.3.0: BLE, Sensores, UART y más!](https://ariel-os.org/blog/ariel-os-0.3.0/)
* [Proxy CipherStash 2.1.20 - Cifrado buscable de Postgres en Rust puro](https://github.com/cipherstash/proxy/discussions/361)

### Observaciones/Pensamientos
* [Linux 7.0 concluyendo oficialmente el experimento Rust](https://www.phoronix.com/news/Linux-7.0-Rust)
* [Indexación de tuplas prestada para HashMap](https://traxys.me/tuple_borrow.html)
* [¿Qué tiene de especial Rust?](https://bitfieldconsulting.com/posts/why-rust)
* [Desplegando Rust en la lista de verificación de producción](https://kerkour.com/rust-production-checklist)
* [vídeo] [Seguro, rápido y escalable: Por qué gRPC-Rust debería ser tu próximo framework RPC](https://www.youtube.com/watch?v=l6YTt8ze4lI)
* [vídeo] [Anodizado: Especificaciones más allá de los tipos en Rust](https://www.youtube.com/watch?v=JtYyhXs4t6w)
* [vídeo] [impl Rust: herramienta Avro IDL en Rust vía LLM](https://www.youtube.com/watch?v=vmKvw73V394)
* [audio] [Netstack.FM episodio 26 — Protocolos de correo electrónico con Mauro De Gennaro de Stalwart Labs](https://netstack.fm/#episode-26)

### Guías de Rust
* [Un futuro para los bitflags](https://kodraus.github.io/rust/2026/02/06/bitflags-derive.html)
* [¿Recarga caliente en Rust? ¡Subsecond y Dioxus al rescate!](https://codethoughts.io/posts/2026-02-07-rust-hot-reloading/)
* [Referencia 2 cajas decimales de coma flotante vs punto fijo](https://github.com/WuBingzheng/primitive_fixed_point_decimal/blob/master/benches/README.md)
* [Intentando apoyar FreeBSD y Nix para mi CLI: Lecciones aprendidas](https://ivaniscoding.github.io/posts/rustpackaging3/)
* [vídeo] [Rama @ FOSDEM 2026 — Repensando los servicios de red: Libertad y modularidad con Rama](https://fosdem.org/2026/schedule/event/CKANPK-programmable_networking_with_rama/)
* [vídeo] [Implementando el servidor TCP Echo en Rust](https://www.youtube.com/watch?v=qjOBZ_Xzuio)

## Crate de la semana

El crate de esta semana es [zedbar](https://crates.io/crates/zedbar), una caja para leer códigos QR y un montón de otros formatos de códigos de barras de imágenes.

¡Gracias a [Brian Donovan](https://users.rust-lang.org/t/crate-of-the-week/2704/1536) por la autosugerencia!

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

*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

* [**Conferencia Oxid**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | Última oportunidad: el CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-11

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

569 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-03..2026-02-10

#### Compilador
* [añadir pelusa de 'unreachable_cfg_select_predicates'](https://github.com/rust-lang/rust/pull/149960)
* [implementa MVP para argumentos genéricos opacos de const](https://github.com/rust-lang/rust/pull/150823)
* [proporciona más contexto sobre que los límites de rasgos no se cumplen debido a una derivación imperfecta](https://github.com/rust-lang/rust/pull/151278)

#### Biblioteca
* [añadir 'NonZero:<T>::from_str_radix'](https://github.com/rust-lang/rust/pull/151945)
* [implementa 'int_from_ascii' para 'NonZero<T>'](https://github.com/rust-lang/rust/pull/152267)
* [añadir algo de rasgo de conversión implica](https://github.com/rust-lang/rust/pull/145504)
* [alinear el rasgo 'ArrayWindows' implica con 'Windows'](https://github.com/rust-lang/rust/pull/151613)
* [implementar constantes FD stdio](https://github.com/rust-lang/rust/pull/152071)
* [estabilizar 'núcleo::pista::cold_path'](https://github.com/rust-lang/rust/pull/151576)
* [estabilizar predicados ControlFlow de constancia](https://github.com/rust-lang/rust/pull/152253)
* [estabilizar nuevo tipo de rango inclusivo y tipo iterador](https://github.com/rust-lang/rust/pull/150522)
* [introducir métodos de normalización de caminos en la parte superior de 'std::p ath'](https://github.com/rust-lang/rust/pull/142957)

#### Carga
* ['lints': añadir 'missing_lints_inheritance'](https://github.com/rust-lang/cargo/pull/16588)
* ['lints': añadir pelusa de 'unused_workspace_package_fields'](https://github.com/rust-lang/cargo/pull/16585)
* [''tiempos': activar la selección de texto en los gráficos](https://github.com/rust-lang/cargo/pull/16607)
* [añadir host.runner para envolver ejecuciones de objetivos de build de host](https://github.com/rust-lang/cargo/pull/16599)
* [corregir anfitriones conocidos analizando](https://github.com/rust-lang/cargo/pull/16596)

#### Clippy
* [corregir 'cmp_owned' falso positivo cuando 'to_string' proviene de una entrada macro](https://github.com/rust-lang/rust-clippy/pull/16468)
* [arreglar: manejar falso negativo para 'str_to_string'](https://github.com/rust-lang/rust-clippy/pull/16512)

#### Analizador de Rust
* [añadir 'expression_types()', 'pattern_types()', 'binding_types()' a 'DefWithBody'](https://github.com/rust-lang/rust-analyzer/pull/21584)
* [implementar soporte de cancelación de solicitudes del lado del cliente con grano minucioso](https://github.com/rust-lang/rust-analyzer/pull/21380)
* [al autoimportar un segmento seguido de otros segmentos, solo considera los elementos que se resuelvan con los segmentos posteriores](https://github.com/rust-lang/rust-analyzer/pull/21574)
* [prueba de enlazamiento de postal de corrección](https://github.com/rust-lang/rust-analyzer/pull/21538)
* [cubre más casos donde necesitamos paréntesis en '&(impl Trait1 + Trait2)'](https://github.com/rust-lang/rust-analyzer/pull/21569)
* [corregir 'set_top_subtree_delimiter_span' usando un índice incorrecto para el espacio cerrado](https://github.com/rust-lang/rust-analyzer/pull/21608)
* [fix pierde los límites asociados para 'replace_derive_with_manual_impl'](https://github.com/rust-lang/rust-analyzer/pull/21583)
* [arreglo no completo '.not' en condición](https://github.com/rust-lang/rust-analyzer/pull/21526)
* [¡Infiere el Len esperado en include_bytes! ()', para evitar desajustes](https://github.com/rust-lang/rust-analyzer/pull/21573)
* [bajado de ciclos corre](https://github.com/rust-lang/rust-analyzer/pull/21579)
* [diagnóstico obsoleto con JSON rust-project.json y rustc](https://github.com/rust-lang/rust-analyzer/pull/21571)
* [sincronizar 'allow_normalization' con RUSTC](https://github.com/rust-lang/rust-analyzer/pull/21611)
* [truncar la versión de visualización de comandos de forma consistente](https://github.com/rust-lang/rust-analyzer/pull/21580)
* [usar 'display_source_code()' en 'ReferenceConversion'](https://github.com/rust-lang/rust-analyzer/pull/21578)
* [migrar el manejador de getters y setters a SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21606)

### Triaje de rendimiento del compilador Rust

Esta semana hemos visto bastantes mejoras. La mayor proviene de añadir dos llamadas 'with_capacity' dirigidas en [#151929](https://github.com/rust-lang/rust/pull/151929).
Otra fuente de múltiples mejoras es la migración continua para dejar de usar archivos externos para almacenar mensajes de diagnóstico.

Triaje hecho por **@panstromek**.
Rango de revisión: [a60d12cb.. 39219ceb](https://perf.rust-lang.org/?start=a60d12cbccfbeaf153f3cecb90454aa696ea4b3b&end=39219ceb97d1b37dda72517daa9ebe8364ffe186&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 2,0% | [2,0%, 2,0%] | 1 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,0%, 2,0%] | 22 |
| Mejoras ✅ <br /> (primaria) | -0,8% | [-2,8%, -0,2%] | 179 |
| Mejoras ✅ <br /> (secundario) | -3,1% | [-31,1%, -0,0%] | 211 |
| Todos ❌✅ (primario) | -0,7% | [-2,8%, 2,0%] | 180 |

1 regresión, 6 mejoras, 7 mixtas; 9 de ellos en rollos
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/c3789580abef82e4d72aaeb5e85cfd09f53e8ce8/triage/2026/2026-02-09.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Estabilizar 'str_as_str'](https://github.com/rust-lang/rust/pull/151603)
* [Problema de seguimiento para '#! [feature(control_flow_ok)]'](https://github.com/rust-lang/rust/issues/140266)
* [Soporte para importar palabra clave de segmento de ruta con cambio de nombre](https://github.com/rust-lang/rust/pull/146972)
* ['-Znext-solver' Eliminar el hack de ambigüedad forzada del grafo de búsqueda](https://github.com/rust-lang/rust/pull/149904)
* [Hacer que PinCoerceUnsized requiera Deref](https://github.com/rust-lang/rust/pull/149218)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Eliminar las opciones de traducción '-Z'](https://github.com/rust-lang/compiler-team/issues/967)

#### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Script de estabilizar carga](https://github.com/rust-lang/cargo/pull/16569)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), o 
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Añadir soporte para compiladores para funciones de instrumentación](https://github.com/rust-lang/rfcs/pull/3917)
* [RFC: Añadir 'MaybeDropped<T>'](https://github.com/rust-lang/rfcs/pull/3918) 

## Próximos eventos

Eventos Rusty entre el 11-02-2026 - el 11-03-2026 🦀

### Virtual
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
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254788/)
* 2026-02-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Almuerzo y aprendizaje: Patrón de Rust Coincidiendo Desempacado**](https://www.meetup.com/women-in-rust/events/312799411/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/fvcjjuv8)
* 2026-02-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455923/)
* 2026-03-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcfbgb/)
* 05-03-2026 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Presentación: Tock OS Parte #3 - Cápsulas y controladores de hardware de nivel inferior**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 05-03-2026 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (Londres, Reino Unido)| [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/312799450/)

### Asia
* 2026-02-11 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Encuentro de Malasia Rust febrero 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de febrero de 2026**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**En persona Rust febrero 2026 en Nuvoton en Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europa
* 2026-02-11 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-18 - 2026-02-19 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-19 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en el DOJO HACKER**](https://www.meetup.com/hackerdojo/events/313139277/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Charla de febrero de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313172595/)
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**Rust en MWC Talent Arena — Talleres + Encuentro Comunitario**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburgo, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn marzo 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Discos, Destrozados sobre Hielo: Una introducción al parquet y el iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)

### Norteamérica
* 2026-02-11 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Rust ATX en Cloudflare**](https://www.meetup.com/rust-atx/events/313147803/)
* 2026-02-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Desarrollo web Full Stack en Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-12 | Portland, OR, EE. UU. [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Implementando un Gestor de Ventanas: flujos de trabajo para desarrolladores, vinculaciones en C y herramientas Rust**](https://www.meetup.com/pdxrust/events/313233869/)
* 2026-02-12 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust Febrero Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313211483/)
* 2026-02-14 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Allston Rust, 14 de febrero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro y Saludo Comunitario**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Union Square Somerville, 21 de febrero**](https://www.meetup.com/bostonrust/events/313208518/)
* 2026-02-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust como capa de pegamento- Infraestructura para aplicaciones nativas de IA**](https://www.meetup.com/rust-los-angeles/events/313097225/)
* 2026-02-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228648/)
* 2026-02-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Soluciones en tiempo de compilación**](https://www.meetup.com/rust-nyc/events/313196004/)
* 28-02-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de la Óxida de la Universidad de Boston, 28 de febrero**](https://www.meetup.com/bostonrust/events/313208529/)
* 05-03-2026 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**por determinar**](https://www.meetup.com/stl-rust/events/312654992/)
* 2026-03-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, 7 de marzo**](https://www.meetup.com/bostonrust/events/313208584/)

### Oceanía
* 2026-02-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane febrero 2026**](https://www.meetup.com/rust-brisbane/events/313087789/)
* 2026-02-11 | Sídney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
    * [**Bienvenidos 🦀 a 2026**](https://www.meetup.com/rust-sydney/events/313074935/)
* 2026-02-24 | Canberra, AU | [Canberra Oxidado](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/313199994/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Opinión impopular: el manejo de errores en Rust es realmente fantástico. Una vez que conoces los patrones correctos, que lamentablemente NO siempre son evidentes 😂 

– [Ian Wagner sobre Fosstodon](https://fosstodon.org/@ianthetechie/116012332982905561)

A pesar de otra semana con una lamentable falta de sugerencias, llogiq está satisfecho con lo que ha encontrado.

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1r2p269/this_week_in_rust_638/)</small>
