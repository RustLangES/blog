---
title: "Esta semana en Rust #89"
number_of_week: 89
description: El crate de esta semana es logos, un generador moderno de lexer.
date: 2025-12-17
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
* [Actualización de objetivos del proyecto — noviembre de 2025 | Blog Rust](https://blog.rust-lang.org/2025/12/16/Project-Goals-2025-November-Update.md/)
* [Anunciando Rust 1.92.0 | Rust Blog](https://blog.rust-lang.org/2025/12/11/Rust-1.92.0/)

### Actualizaciones de proyectos/herramientas
* [Cómo hicimos que SeaORM fuera síncrono](https://www.sea-ql.org/blog/2025-12-12-sea-orm-2.0/)
* [Building Slatron: El sistema de programación y automatización de la televisión digital](https://justinwoodring.com/blog/introducing-slatron/)
* [El estado del experimento de Rust del núcleo](https://lwn.net/SubscriberLink/1050174/6b6d55c90ce1100f/)

### Observaciones/Pensamientos
* [Asegura tus proyectos Rust y mejora la experiencia de los desarrolladores con Dev Containers](https://kerkour.com/rust-devcontainers)
* [Miri: Detección práctica de comportamiento indefinido para Rust](https://plf.inf.ethz.ch/research/popl26-miri.html)
* [Encontrar alineación visualizando música con Rust](https://positron.solutions/articles/finding-alignment-by-visualizing-music)
* [Rust GCC backend: Por qué y cómo](https://blog.guillaume-gomez.fr/articles/2025-12-15+Rust+GCC+backend%3A+Why+and+how)
  
### Guías de Rust
* [Pruebas unitarias de Rust: Escritura de archivos](https://jorgeortiz.dev/posts/rust_unit_testing_file_writing/)
* [Dejar de perder la intención: ausente, nulo y valor en Rust](https://minikin.me/blog/presence-rs)
* [Iniciando: Cómo aprender Rust Integrado para principiantes](https://blog.implrust.com/posts/2025/12/how-to-learn-embedded-rust/)
* [Escribiendo un rasgo de sistema de archivos mockable en Rust sin RefCell](https://pyk.sh/blog/2025-12-15-writing-mockable-fs-in-rust-without-refcell)
* [serie] [La Guía del Programador Impaciente para Bevy y Rust: Capítulo 4 - Que haya colisiones](https://aibodh.com/posts/bevy-rust-game-development-chapter-4/)
* [Construcción de actualizaciones OTA seguras para ESP32 sobre BLE con Rust](https://gill.net.in/posts/building-secure-ota-updates-for-esp32-over-ble-with-rust/)
* [Memoización posicional mediante macros de activación en un framework de interfaz Rust](https://tessera-ui.github.io/blog/positional-memoization-via-proc-macros.html)
* [hotpath-rs - tiempo de CPU vs tiempo de reloj de pared: perfilado asíncrono de Rust](https://hotpath.rs/sampling_comparison) 

### Miscelánea
* [Informe de empleo de noviembre de 2025](https://filtra.io/rust/jobs-report/nov-25)
* [Superpotenciando la auditoría de seguridad con Rust: Una entrevista con Caido](https://www.sea-ql.org/interview/2025-12-16-caido/)

## Crate de la semana

El crate de esta semana es [logos](https://github.com/maciejhirsz/logos), un generador moderno de lexer.

¡Gracias a [Sam O'Brien](https://users.rust-lang.org/t/crate-of-the-week/2704/1507) por la (parcial auto)sugerencia!

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
* [**SemanaRust 2026**](https://sessionize.com/rustweek-2026/) | CFP cierra el 31-12-2025 | Utrecht, Países Bajos | 2026-05-19 - 2026-05-20
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-10
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

482 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-09..2025-12-16

#### Compilador
* ['rustc_scalable_vector(N)'](https://github.com/rust-lang/rust/pull/143924)
* [añade experimentalmente bloques de 'intento' *heterogéneos*](https://github.com/rust-lang/rust/pull/149489)
* [elementos implementables externamente](https://github.com/rust-lang/rust/pull/146348)
* [prohíbe lanzar libremente límites de por vida de los tipos dyn](https://github.com/rust-lang/rust/pull/136776)
* [heredar atributos en delegación](https://github.com/rust-lang/rust/pull/149843)
* [solo resolver el principal en cajas de contenedores](https://github.com/rust-lang/rust/pull/149867)
* [revisión del manejo de nombres de archivo para la consistencia entre compiladores](https://github.com/rust-lang/rust/pull/149709)
* [simplifica cómo el asm en línea maneja 'MaybeUninit'](https://github.com/rust-lang/rust/pull/149950)

#### Biblioteca
* [añadir 'Tiempodel Sistema::{MIN,' MAX}](https://github.com/rust-lang/rust/pull/148825)
* [añadir 'core::hint::p refetch_{read, write}_{data, instruction}'](https://github.com/rust-lang/rust/pull/146948)
* [constify 'DropGuard::d ismiss' y implica rasgo](https://github.com/rust-lang/rust/pull/148755)
* [fix vec iter zst alignment](https://github.com/rust-lang/rust/pull/149272)
* [estabilizar 'const_mul_add'](https://github.com/rust-lang/rust/pull/148052)

#### Carga
* ['feat(log)': preparar los mensajes de temporización para la reproducción en HTML](https://github.com/rust-lang/cargo/pull/16382)
* ['feat(report)': tiempos de reporte de carga, reproducción HTML](https://github.com/rust-lang/cargo/pull/16377)
* [nuevo: Mejorar la calidad de los mensajes de error de nombre de paquete](https://github.com/rust-lang/cargo/pull/16398)
* [paquete: No verificar el registro para --list](https://github.com/rust-lang/cargo/pull/16341)
* ['fijar(tiempo)': más datos de tiempo/logaritmía autosuficientes](https://github.com/rust-lang/cargo/pull/16378)
* ['prueba(pelusa)': redactar más por omitir línea](https://github.com/rust-lang/cargo/pull/16391)
* [submódulo de caché en git db](https://github.com/rust-lang/cargo/pull/16246)
* [degradación de curl-sys a 0.4.83](https://github.com/rust-lang/cargo/pull/16379)
* [dote: estabilizar '-Zconfig-include'](https://github.com/rust-lang/cargo/pull/16284)
* [corregido lógica de bloqueo incorrecta cuando artefact-dir == build-dir](https://github.com/rust-lang/cargo/pull/16385)
* [prueba: usar un ancho de término por defecto mayor](https://github.com/rust-lang/cargo/pull/16403)

#### Clippy
* ['format_push_string': dar una sugerencia (posiblemente incompleta)](https://github.com/rust-lang/rust-clippy/pull/16093)
* ['manual_saturating_arithmetic': pelusa 'x.checked_sub(y).unwrap_or_default()'](https://github.com/rust-lang/rust-clippy/pull/15845)
* ['transmute_ptr_to_ref': Empuñadura un puntero envuelto en un 'struct'](https://github.com/rust-lang/rust-clippy/pull/15948)
* ['unnecessary_fold': pelusa en 'fold' con 'Add::add'/'Mul::mul'](https://github.com/rust-lang/rust-clippy/pull/16124)
* ['match_like_matches_macro': corregir falso positivo con guardias que contienen 'si dejamos'](https://github.com/rust-lang/rust-clippy/pull/15876)
* [añadir pelusa de 'needless_type_cast'](https://github.com/rust-lang/rust-clippy/pull/16139)
* [añadir cobertura de reducción del iterador a 'never_loop'](https://github.com/rust-lang/rust-clippy/pull/16222)
* [cuenta operaciones inseguras y macrollamadas una vez hacia el bloque más inseguro en el interior](https://github.com/rust-lang/rust-clippy/pull/16117)
* [no busques métodos prohibidos dentro del código desugared](https://github.com/rust-lang/rust-clippy/pull/16186)
* [fix 'branches-sharing-code' sugiere erróneamente en 'const' y 'static'](https://github.com/rust-lang/rust-clippy/pull/15522)
* [corregir 'clippy::ref_as_ptr' para referencias no temporales en let/const](https://github.com/rust-lang/rust-clippy/pull/16214)
* [corregir 'if_not_else' macros no deformadas](https://github.com/rust-lang/rust-clippy/pull/15931)
* [corregir 'if_then_some_else_none' falso positivo al encontrarse con códigos de 'esperar'](https://github.com/rust-lang/rust-clippy/pull/16178)
* [corrigir 'map_entry' sugiere erróneamente para insertar con código de salida CFG](https://github.com/rust-lang/rust-clippy/pull/15800)
* [corregir macros 'match_like_matches_macro' mal desmanipuladas](https://github.com/rust-lang/rust-clippy/pull/16018)
* [corregir 'set-contains-or-insert' falso positivo cuando set se muta antes de 'insert'](https://github.com/rust-lang/rust-clippy/pull/16009)
* [corregir 'unchecked_time_subtraction' falso negativo en la llamada de método 'Ops::sub'](https://github.com/rust-lang/rust-clippy/pull/16233)

#### Analizador de Rust
* [corregir "Invariant violation: file emited multiple times" al hacer 'scip .'](https://github.com/rust-lang/rust-analyzer/pull/21270)
* [corregir 'bind_unused_param' aplicable al cierre](https://github.com/rust-lang/rust-analyzer/pull/21264)
* [asistencia de reparación 'y' → parámetro 'and_then'](https://github.com/rust-lang/rust-analyzer/pull/21239)
* [corregir referencia completa para '&mut ty' → '& ty'](https://github.com/rust-lang/rust-analyzer/pull/21278)
* [corregir unidad completa retorno punto y coma en la lista de arg](https://github.com/rust-lang/rust-analyzer/pull/21032)
* [fijar tipo esperado sin desmontar la tira](https://github.com/rust-lang/rust-analyzer/pull/21277)
* [corregir sangría para 'toggle_ignore'](https://github.com/rust-lang/rust-analyzer/pull/21175)
* [corregir lógica inválida para 'replace_let_with_if_let'](https://github.com/rust-lang/rust-analyzer/pull/21266)
* [la corrección pierde la etiqueta de 'convert_for_to_while_let'](https://github.com/rust-lang/rust-analyzer/pull/20754)
* [arreglar no aplicable fn en cierre para 'add_return_type'](https://github.com/rust-lang/rust-analyzer/pull/21258)
* ['#[rustc_deprecated_safe_2024]' también puede ser '#[rustc_deprecated_safe_2024(audit_that = "razón")]'](https://github.com/rust-lang/rust-analyzer/pull/21244)
* ['is_transmutable' siempre en pánico](https://github.com/rust-lang/rust-analyzer/pull/21238)
* [arreglar un pánico en 'ast::TypeBound::kind()'](https://github.com/rust-lang/rust-analyzer/pull/21251)
* [corregir resolución de método para impls incoherentes cuando hay dos sysroots en el grafo de caja](https://github.com/rust-lang/rust-analyzer/pull/21273)
* [implementación de 'locals_used' a nivel HIR](https://github.com/rust-lang/rust-analyzer/pull/21262)
* [lsp: gestión del registro dinámico para didSave](https://github.com/rust-lang/rust-analyzer/pull/21253)
* [prefijo json archivo destino con raíz de espacio de trabajo para metadatos sysroot](https://github.com/rust-lang/rust-analyzer/pull/21272)
* [respetar la orden de aplicación del atributo de lint de Rustc](https://github.com/rust-lang/rust-analyzer/pull/21265)
* [indicación de parámetro para argumentos faltantes](https://github.com/rust-lang/rust-analyzer/pull/21240)
* [soporte '#[feature(associated_type_defaults)]'](https://github.com/rust-lang/rust-analyzer/pull/21243)
* [compatibilidad con dyn para cadenas de herramientas antiguas sin 'MetaSized'](https://github.com/rust-lang/rust-analyzer/pull/21241)
* [¡apoyan la nueva bajada de 'format_args! ()'](https://github.com/rust-lang/rust-analyzer/pull/21242)
* [usar 'cmark_with_options' para escribir accesos directos enlaces a la salida](https://github.com/rust-lang/rust-analyzer/pull/21276)
* [incluir ocurrencias de sobrecarga de operadores en el índice SCIP](https://github.com/rust-lang/rust-analyzer/pull/21187)
* [reordenar 'add_return_type' asistencia](https://github.com/rust-lang/rust-analyzer/pull/21256)

### Triaje de rendimiento del compilador Rust

Esta semana vimos varias regresiones, en parte por el compilador haciendo más trabajo. Los restantes
Se están investigando regresiones.

Triaje hecho por **@kobzol**.
Rango de revisión: [55495234..21ff67df](https://perf.rust-lang.org/?start=554952348a7dd13851f25789f6bb1061f45c4b60&end=21ff67df15329dd7548ccba54b6c6ae9a562124f&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,5% | [0,1%, 5,1%] | 40 |
| Regresiones ❌ <br /> (secundario) | 0,8% | [0,0%, 3,0%] | 63 |
| Mejoras ✅ <br /> (primaria) | -0,7% | [-1,5%, -0,1%] | 35 |
| Mejoras ✅ <br /> (secundario) | -1,0% | [-7,4%, -0,0%] | 73 |
| Todos ❌✅ (primario) | -0,1% | [-1,5%, 5,1%] | 75 |

3 regresiones, 2 mejoras, 5 mixtas; 2 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/c003a2f8056b7e6a0783b7c99bb3380ae2c12c5d/triage/2025/2025-12-16.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?
* [Añadiendo una pestaña de seguridad crates.io](https://github.com/rust-lang/rfcs/pull/3872)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [¡No desnudes nada en expr-ctxt 'incluido! (…)'](https://github.com/rust-lang/rust/pull/146377)
* [Política sobre el uso de 'rustc_legacy_const_generics' en stdarch](https://github.com/rust-lang/rust/issues/149654)
* [Problema de seguimiento para 'atomic_try_update'](https://github.com/rust-lang/rust/issues/135894)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [build-std: contexto](https://github.com/rust-lang/rfcs/pull/3873)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat(index): Estabilizar el tiempo de publicación](https://github.com/rust-lang/cargo/pull/16372)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Añadir política de antiguos alumnos](https://github.com/rust-lang/leadership-council/pull/218)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
  [Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
  [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Abre tu enum con una variante sin nombre](https://github.com/rust-lang/rfcs/pull/3894)

## Próximos eventos

Eventos Rusty entre el 17-12-2025 - el 14-01-2026 🦀

### Virtual
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
* 2026-01-08 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡Conocer, intercambiar y aprender!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/310254791/)

### Asia
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de diciembre 2025**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)
* 2026-01-06 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**Rust en persona enero de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europa
* 2025-12-18 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**GCC backend**](https://www.meetup.com/london-rust-project-group/events/312443570/)
* 2025-12-19 | Lyon, FR | [Lyon Oxidado](https://www.meetup.com/rust-lyon)
    * [**Reunión de Rust Lyon #11**](https://www.meetup.com/rust-lyon/events/312180836/)
* 07-01-2026 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-08 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)

### Norteamérica
* 2025-12-17 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312076080/)
* 2025-12-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Spokane, WA, EE. UU. [Rust de Spokane](https://www.meetup.com/spokane-rust)
    * [**Encuentro social de fin de año con grupos de usuarios locales de Python, Rust y otros**](https://www.meetup.com/spokane-rust/events/312292668/)
* 2025-12-20 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 20 de diciembre**](https://www.meetup.com/bostonrust/events/311917280/)
* 2025-12-25 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312323693/)
* 2026-01-08 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1plbecs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> permito que mi código se use para entrenar IA en GitHub. No porque tema que la IA nos quite los puestos, sino porque estoy seguro de que mi código lo ralentizará lo suficiente como para salvarnos a todos.

– [王翼翔 en rust-users](https://users.rust-lang.org/t/whats-going-on-with-bincode/136942/3)

¡Gracias a [Moy2010](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1738) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1pppcsc/this_week_in_rust_630/)</small>
