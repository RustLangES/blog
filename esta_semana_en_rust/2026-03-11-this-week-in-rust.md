---
title: "Esta semana en Rust #101"
number_of_week: 101
description: El crate de esta semana es sentencex, una biblioteca rápida de segmentación de oraciones.
date: 2026-03-11
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
* [Anunciando Rust 1.94.0 | Rust Blog](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0/)

### Boletines
* [ 🦀 Hallazgos del estado del levantamiento de Rust](https://weeklyrust.substack.com/p/state-of-rust-survey-findings)

### Actualizaciones de proyectos/herramientas
* [Versión 0.7.0 · uutils/coreutils](https://github.com/uutils/coreutils/releases/tag/0.7.0)
* [mdterm v1.0.0 - Un navegador Markdown basado en terminal](https://github.com/bahdotsh/mdterm/releases/tag/v1.0.0)
* [La anatomía de un analizador 500ns: Portando libphonenumber a Rust](https://dev.to/vloldik/the-anatomy-of-a-500ns-parser-porting-libphonenumber-to-rust-3daa)
* [mini-agente: Un Marco de Agentes de IA Rust](https://dev.to/rajmandaliya/building-a-rust-ai-agent-framework-from-scratch-what-i-learned-3o23)
* [ClickHouse se encuentra con SeaORM: Pipeline de datos impulsado por flechas](https://www.sea-ql.org/blog/2026-03-08-sea-clickhouse/)
* [Rustaceans.AI](https://rustaceans.ai/)
* [Leptodon 1.0.0: Kit de herramientas de interfaz para el framework Leptos WASM](https://www.openanalytics.eu/blog/2026/03/09/leptodon-1.0.0/)
* [Firmar binarios de Rust no debería requerir scripts de shell](https://d34dl0ck.me/cargo-codesign/index.html)

### Observaciones/Pensamientos
* [derivados simbólicos y la reescritura de Rust de RE# | ian Erik Varatalu](https://iev.ee/blog/symbolic-derivatives-and-the-rust-rewrite-of-resharp/)
* [El estado de los asignadores en 2026](https://cetra3.github.io/blog/state-of-allocators-2026/)
* [serie] [FORTRAN a Rust: parte 1](https://zaynar.co.uk/posts/f2rust-1/)
* [El coste de la indirección en Rust](https://blog.sebastiansastre.co/posts/cost-of-indirection-in-rust/)
* [¿Por qué SeaORM sobre opciones de base de datos cliente JavaScript?](https://opeolluwa.github.io/almonds/blog/why-not-js-database)
* [Rust se está comiendo poco a poco PostgreSQL: Profundiza en Neon, ParadeDB, PgDog y más](https://kerkour.com/rust-eating-postgres)
* [Qué ocurre cuando limitas un sistema impulsado por eventos a tres primitivas](https://www.rodriguez.today/articles/emergent-event-driven-workflows)
* [Mi configuración de desarrollo de Rust en 2026](https://bitfieldconsulting.com/posts/rust-dev-tools)
* [audio] [Netstack.FM episodio 30 — uReq con Martin Algesten](https://netstack.fm/#episode-30)
* [Evaluando Zngur y CXX para interoperabilidad Rust/C++](https://www.kdab.com/weighing-up-zngur-and-cxx-for-rustc-interop/)
### Guías de Rust
* [ZK se burla de Rust Developer parte 1/8](https://rustarians.com/polynomials-in-zk-snarks/)
* [Haz fila (Parte 2) - La Cola de Vyukov y sus especializaciones](https://abhikja.in/blog/2026-03-10-get-in-line-part-2/)
* [Cómo dejar de pelear con la coherencia y empezar a escribir implicaciones de rasgos genéricos de contexto](https://contextgeneric.dev/blog/rustlab-2025-coherence/)
* [Reescribiendo nuestra base de datos en Rust](https://medium.com/airtable-eng/rewriting-our-database-in-rust-f64e37a482ef)
* [OpenTelemetry para desarrolladores de Rust - La guía completa de implementación](https://signoz.io/blog/opentelemetry-rust/)

### Miscelánea
* [Rust brilló sobre Python para mi herramienta CLI - Blog de desarrollo sonriente](https://smiling.dev/blog/rust-shined-over-python-for-my-cli-tool/)
* [Escribe pequeños scripts de Rust](https://llogiq.github.io/2026/03/05/auto.html)
## Crate de la semana

El crate de esta semana es [sentencex](https://github.com/wikimedia/sentencex), una biblioteca rápida de segmentación de oraciones.

¡Gracias a [Santhosh Thottingal](https://users.rust-lang.org/t/crate-of-the-week/2704/1564) por la autosugerencia!

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
* [protector diésel - ACTUALIZAR VISTA MATERIALIZADA sin SIMULTÁNEAMENTE](https://github.com/ayarotsky/diesel-guard/issues/89)
* [protector diésel - AÑADIR COMPROBAR RESTRICCIÓN sin NO VÁLIDO](https://github.com/ayarotsky/diesel-guard/issues/88)
* [protector diésel - AÑADIR CLAVE EXTRANJERA sin NO VÁLIDO](https://github.com/ayarotsky/diesel-guard/issues/87)
* [guardia diésel - sin lock_timeout/statement_timeout antes de DDL](https://github.com/ayarotsky/diesel-guard/issues/97)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

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

483 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-03-03..2026-03-10

#### Compilador
* [activa 'PassMode::Indirect { on_stack: true, .. }'argumentos de llamada de cola](https://github.com/rust-lang/rust/pull/153361)

#### Biblioteca
* [constificar 'Vec::{en, desde}_raw_parts{_in|_alloc}'](https://github.com/rust-lang/rust/pull/153399)
* [implementar soporte para compiladores 'MaybeDangling'](https://github.com/rust-lang/rust/pull/150447)
* [estabilizar 'control_flow_ok'](https://github.com/rust-lang/rust/pull/152911)

#### Carga
* ['compilar': Convierte también los resúmenes de advertencia en errores](https://github.com/rust-lang/cargo/pull/16721)
* ['arreglar': Cambiar de advertencias improvisadas a estructuradas](https://github.com/rust-lang/cargo/pull/16711)
* ['guion': suprimir pelusa de 'unused_features' para incrustada](https://github.com/rust-lang/cargo/pull/16714)
* ['pruebas': permiten 'no pudo' tan bien como no pudo en la salida de prueba](https://github.com/rust-lang/cargo/pull/16698)
* [añadir trunque faltante al escribir archivos '.crate'](https://github.com/rust-lang/cargo/pull/16688)
* [ignorar dependencias implícitas de ETS en la pelusa de 'dependencias de cajas no usadas'](https://github.com/rust-lang/cargo/pull/16677)
* [deja que git decida cuándo ejecutar gc](https://github.com/rust-lang/cargo/pull/16459)
* [Divide la cerradura 'build-dir' en una cerradura dedicada](https://github.com/rust-lang/cargo/pull/16708)

#### Clippy
* [añadir pelusa de 'manual_pop_if'](https://github.com/rust-lang/rust-clippy/pull/16582)
* ['doc_paragraphs_missing_punctuation': Recortar símbolos de la imagen](https://github.com/rust-lang/rust-clippy/pull/16514)
* [no materializar fragmentos cuando no es necesario](https://github.com/rust-lang/rust-clippy/pull/16666)
* [arreglar hielo en 'match_same_arms'](https://github.com/rust-lang/rust-clippy/pull/16685)
* [arreglar hielo en 'swap_binop()'](https://github.com/rust-lang/rust-clippy/pull/16659)
* [arreglar el motor interno al usar la función 'min_generic_const_args' incompleto](https://github.com/rust-lang/rust-clippy/pull/16692)
* [corregir la sugerencia 'infinite_loop' incorrecta dentro de las ramas condicionales](https://github.com/rust-lang/rust-clippy/pull/16619)
* [corrigir 'redundant_closure' sugiere erróneamente cuando local se reduce a llamable](https://github.com/rust-lang/rust-clippy/pull/16648)
* [corregir 'unnecessary_safety_comment' falsos positivos en bloques de código dentro de la documentación interna](https://github.com/rust-lang/rust-clippy/pull/16559)
* [fijar punto y coma dentro del bloque dentro de 'try_blocks'](https://github.com/rust-lang/rust-clippy/pull/16697)
* [optimizar la evaluación de 'allow_unwrap_types' para eliminar la regresión de rendimiento](https://github.com/rust-lang/rust-clippy/pull/16652)

#### Analizador de Rust
* [no volver a consultar raíces de fuente por caja en analysis-stats](https://github.com/rust-lang/rust-analyzer/pull/21788)
* [ofrece 'destructure_struct_binding' en el propio param](https://github.com/rust-lang/rust-analyzer/pull/21687)
* [cuando vas a definir en '?' en 'Resultado' que pasa por 'From', ve a la impl 'From'](https://github.com/rust-lang/rust-analyzer/pull/21752)
* [añadir métodos de 'has_pending' a 'Incoming'/'Outgoing'/'ReqQueue' en 'lsp_server'](https://github.com/rust-lang/rust-analyzer/pull/21755)
* ['cfg_select' soporta tokens que no son de árbol de tokens](https://github.com/rust-lang/rust-analyzer/pull/21705)
* [alinear 'is_rust()' con rustc corrigiendo el constructor ABI en el siguiente solver](https://github.com/rust-lang/rust-analyzer/pull/21726)
* [no usar PostAnalysis TypingMode para la resolución de métodos IDE](https://github.com/rust-lang/rust-analyzer/pull/21750)
* [el observador de archivos debe ver directorios recursivamente](https://github.com/rust-lang/rust-analyzer/pull/21771)
* [corregir rango descendente incorrecto para 'add_missing_match_arms'](https://github.com/rust-lang/rust-analyzer/pull/21728)
* [bloqueo ofrecido '.let' en ref-expr en el brazo del partido](https://github.com/rust-lang/rust-analyzer/pull/21671)

### Triaje de rendimiento del compilador Rust

Casi ninguna regresión esta semana, aunque hubo algunas mejoras en el rendimiento
causada por la refactorización continua del sistema de consultas del compilador. La más grande era de
[#153521](https://github.com/rust-lang/rust/pull/153521).

Triaje hecho por **@kobzol**.
Rango de revisión: [ddd36bd5.. 3945997a](https://perf.rust-lang.org/?start=ddd36bd57051f796850345b76c17e9402e28a9e4&end=3945997aabf6165261ef3419534c1ad59d9dc5c6&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,4%, 0,5%] | 3 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,1%, 1,2%] | 8 |
| Mejoras ✅ <br /> (primaria) | -0,9% | [-2,5%, -0,1%] | 110 |
| Mejoras ✅ <br /> (secundario) | -0,8% | [-2,7%, -0,1%] | 77 |
| Todos ❌✅ (primario) | -0,9% | [-2,5%, 0,5%] | 113 |

0 regresiones, 6 mejoras, 3 mixtas; 5 de ellos en rollups
31 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/4bb67d679eabb4cf31dd545c89498a861e937d75/triage/2026/2026-03-10.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Quitar 'ATTRIBUTE_ORDER'](https://github.com/rust-lang/rust/pull/153041)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Perfiles de pelusa personalizados](https://github.com/rust-lang/rfcs/pull/3926)

## Próximos eventos

Eventos Rusty entre el 11-03-2026 y el 08-04-2026 🦀

### Virtual
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
* 2026-04-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 222-03-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**Rust presencial marzo 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)

### Europa
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
* 2026-03-23 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #18**: Ludwig Weinzierl - Bevy: Spielentwicklung mit Rust](https://rust-augsburg.github.io/meetup/Meetup_18.html)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack - Advenimiento del Código**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Marcha de Rust Manchester**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-24 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim)
    * [**Proyectos de Rust - mostrar y contar en marzo**](https://www.meetup.com/rust-trondheim/events/313537618/)
* 2026-03-26 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París**](https://www.rustinparis.com/)
* 2026-04-01 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Reunión Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)

### Norteamérica
* 2026-03-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Un intérprete para la teoría de la computabilidad, escrito a la manera difícil**](https://www.meetup.com/utah-rust/events/313506767/)
* 2026-03-12 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**Encuentro de San Diego Rust March - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721867/)
* 2026-03-14 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo North End Rust, 14 de marzo**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313404095/)
* 2026-03-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313569258/)
* 2026-03-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust aplicado - Aplicaciones de Rust en edificios**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-19 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Interoperabilidad Social - Rust, C++ y el Bien Mayor**](https://www.meetup.com/rust-nyc/events/313639707/)
* 2026-03-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Porter Square Rust, 21 de marzo**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Especial de Adopción de Activos Digitales de Rust NYC**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-04-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler con LLM**](https://www.meetup.com/stl-rust/events/313482094/)

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

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Feliz día de "Clippy, eres muy útil" para quienes celebran!

– [Manpacket en functional.cafe](https://functional.cafe/@manpacket/116178060408287449)

A pesar de la lamentable falta de sugerencias, llogiq está sumamente satisfecho con su elección.

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1rre8o9/this_week_in_rust_642/)</small>
