---
title: "Esta semana en Rust #99"
number_of_week: 99
description: El crate de esta semana es docstr, una caja de macros que proporciona una macro para crear cadenas de varias líneas a partir de los comentarios de los documentos.
date: 2026-02-25
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

* [Rust participa en Google Summer of Code 2026](https://blog.rust-lang.org/2026/02/19/Rust-participates-in-GSoC-2026/)
* [Encuesta de depuración de Rust 2026](https://blog.rust-lang.org/2026/02/23/rust-debugging-survey-2026/)

### Fundación

* [Blog invitado: FOSDEM 2026 — Análisis de Rust Devroom](https://rustfoundation.org/media/guest-blog-fosdem-2026-rust-devroom-in-review/)

### Actualizaciones de proyectos/herramientas

* [Zed: Split Diffs están aquí](https://zed.dev/blog/split-diffs)
* [CHERIoT Rust: Actualización de estado #0](https://rust.cheriot.org/2026/02/15/status-update.html)
* [SeaORM ahora soporta Arrow & Parquet](https://www.sea-ql.org/blog/2026-02-22-sea-orm-arrow/)
* [Liberando bincode-next v3.0.0-rc.1](https://users.rust-lang.org/t/releasing-bincode-next-v3-0-0-rc-1/138466)
* [Presentando almendras](https://opeolluwa.github.io/almonds/blog/hello-almonds)
* [SafePilot v0.1: asistente de IA autoalojado](https://github.com/3DCF-Labs/safepilot/releases/tag/v0.1)
* [Hitbox 0.2.0: orquestación declarativa de caché](https://www.reddit.com/r/rust/comments/1qp88he/hitbox_020_async_http_caching_framework_for_rust/)

### Observaciones/Pensamientos

* [Qué significa que Ubuntu está usando Rust](https://smallcultfollowing.com/babysteps/blog/2026/02/23/ubuntu-rustnation/)
* [Lee Las cerraduras no son tus amigas](https://eventual-consistency.vercel.app/posts/write-locks-faster)
* [Logrando cero errores: Rust, Especificaciones y Codificación de IA](https://www.borg.org/?p=1472)
* [vídeo] [emisario de dispositivo: Diversión incrustada con Rust—por Carl Kadie](https://www.youtube.com/watch?v=iUu6hvJLVOU)

### Guías de Rust

* [Sobre la presión de memoria, contención de bloqueo y Diseño orientado a datos](https://mnt.io/articles/about-memory-pressure-lock-contention-and-data-oriented-design/)
* [Rompiendo SHA-2: ataques de extensión de longitud en práctica con Rust](https://kerkour.com/sha256-length-extension-attacks)
* [device-envoy: Haciendo diversión incrustada con Rust, Embassy y abstracciones de dispositivos componibles](https://medium.com/@carlmkadie/device-envoy-making-embedded-fun-31534917414b)

### Investigación

* [Auditando eficazmente las cajas de Rust](https://arxiv.org/abs/2602.06466)

### Miscelánea

* [Compresión de prompts hieráticos: Del prototipo a la producción](https://noos.blog/posts/hieratic-prompt-compression-release/)

## Crate de la semana

El crate de esta semana es [docstr](https://github.com/nik-rev/docstr), una caja de macros que proporciona una macro para crear cadenas de varias líneas a partir de los comentarios de los documentos.

¡Gracias a [Nik Revenco](https://users.rust-lang.org/t/crate-of-the-week/2704/1557) por la autosugerencia!

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

* [**Conferencia Rust India 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP abierto hasta el 14-03-2026 | Bangalore, IN | 2026-04-18
* [**Conferencia Oxid**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP abierto hasta 2026-03-23 | Berlín, Alemania | 2026-09-14 - 2026-09-16
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

450 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-17..2026-02-24

#### Compilador
* [traer de vuelta 'enum DepKind'](https://github.com/rust-lang/rust/pull/152747)
* [simplificar las ramas canónicas del clon 'enum' a una instrucción de copia](https://github.com/rust-lang/rust/pull/148034)
* [estabilizar guardias 'si dejamos' ('feature(if_let_guard)')](https://github.com/rust-lang/rust/pull/141295)

#### Biblioteca
* [añadir 'try_shrink_to' y 'try_shrink_to_fit' a Vec](https://github.com/rust-lang/rust/pull/152366)
* [corrigido que ByteStr no rellena dentro de su rasgo de visualización cuando no se menciona un alineamiento específico](https://github.com/rust-lang/rust/pull/152865)
* [reflejo 'TypeId::trait_info_of'](https://github.com/rust-lang/rust/pull/152003)
* [reflejo 'TypeKind::FnPtr'](https://github.com/rust-lang/rust/pull/152173)
* [solo pasa 'Layout' directamente a 'box_new_uninit'](https://github.com/rust-lang/rust/pull/152737)
* [estabilizar 'cfg_select!'](https://github.com/rust-lang/rust/pull/149783)

#### Carga
* ['cli': Eliminar '---lockfile-path'](https://github.com/rust-lang/cargo/pull/16621)
* ['job_queue': Manejar los argumentos de la CLI de Clippy en el mensaje 'arreglar'](https://github.com/rust-lang/cargo/pull/16652)
* [corregir el bloqueo paralelo cuando está activado el bloqueo '-Zfine-grain'](https://github.com/rust-lang/cargo/pull/16659)

#### Clippy
* [añadir pelusa de 'unnecessary_trailing_comma'](https://github.com/rust-lang/rust-clippy/pull/16530)
* [añadir nueva pelusa de 'disallowed_fields'](https://github.com/rust-lang/rust-clippy/pull/16218)
* ['clone_on_ref_ptr': no añadir un '&' al receptor si es una referencia](https://github.com/rust-lang/rust-clippy/pull/15742)
* ['needless_maybe_sized': no pelusas en código generado por macro por activación](https://github.com/rust-lang/rust-clippy/pull/15629)
* ['str_to_string': falsos positivos no de tipos de fuerza](https://github.com/rust-lang/rust-clippy/pull/16571)
* ['useless_conversion': también fuego dentro de los desazúcares del compilador](https://github.com/rust-lang/rust-clippy/pull/16594)
* [añadir la configuración 'permitir-desenrollar-tipos' para 'unwrap_used' y 'expect_used'](https://github.com/rust-lang/rust-clippy/pull/16605)
* [añadir corchetes alrededor del bloque inseguro o etiquetado usado en 'else'](https://github.com/rust-lang/rust-clippy/pull/16603)
* [permitir 'deprecated(since = "CURRENT_RUSTC_VERSION")'](https://github.com/rust-lang/rust-clippy/pull/16557)
* [no sugieres quitar el préstamo de un upvar capturado](https://github.com/rust-lang/rust-clippy/pull/16622)
* [mejora 'collapsible_match' para cubrir si-elses](https://github.com/rust-lang/rust-clippy/pull/16560)
* [mejora 'manual_is_variant_and' para cubrir 'filter' encadenando 'is_some'](https://github.com/rust-lang/rust-clippy/pull/16521)
* [corregir 'explicit_counter_loop' falso negativo cuando el contador de bucle empieza en distinto de cero](https://github.com/rust-lang/rust-clippy/pull/16620)
* [fijar 'join_absolute_paths' para que funcione correctamente según la plataforma](https://github.com/rust-lang/rust-clippy/pull/16610)
* [corregir 'redundant_iter_cloned' falso positivo con cierres de movimiento y corutinas](https://github.com/rust-lang/rust-clippy/pull/16494)
* [corregir 'unnecessary_min_or_max' para usize](https://github.com/rust-lang/rust-clippy/pull/16575)
* [corregir pánico/averiguar detección de mensajes en la edición 2015/2018](https://github.com/rust-lang/rust-clippy/pull/16473)
* [handle 'Result<T, !>' y 'ControlFlow<!, T>' como 'T' respecto a '#[must_use]'](https://github.com/rust-lang/rust-clippy/pull/16353)
* [hacer 'unchecked_time_subtraction' para manejar mejor los literales de 'Duración'](https://github.com/rust-lang/rust-clippy/pull/16528)
* [hacer que 'unnecessary_fold' sea conmutativo](https://github.com/rust-lang/rust-clippy/pull/16604)
* [el camino de un tipo a sí mismo es 'Self'](https://github.com/rust-lang/rust-clippy/pull/16362)

#### Analizador de Rust
* [añadir selección parcial para 'generate_getter_or_setter'](https://github.com/rust-lang/rust-analyzer/pull/20353)
* [bloqueo de oferta Dejar el postfijo de respaldo completado](https://github.com/rust-lang/rust-analyzer/pull/21594)
* [oferta sobre 'is_some_and' por 'replace_is_method_with_if_let_method'](https://github.com/rust-lang/rust-analyzer/pull/21623)
* [arreglar algunas asistencias de referencia de TryEnum](https://github.com/rust-lang/rust-analyzer/pull/21389)
* [añadir manejo para ciclos en 'sizedness_constraint_for_ty()'](https://github.com/rust-lang/rust-analyzer/pull/21664)
* [mejor colocación de importación + fusión](https://github.com/rust-lang/rust-analyzer/pull/21635)
* [completa '.let' en expresión del prefijo de cola de bloque](https://github.com/rust-lang/rust-analyzer/pull/21600)
* [derivar completos ayudantes en nameref vacío](https://github.com/rust-lang/rust-analyzer/pull/21655)
* [correctamente poner entre paréntesis la condición invertida en 'convert_if_to_bool_...'](https://github.com/rust-lang/rust-analyzer/pull/21688)
* [excluir referencias de macro en tests cuando excludeTests está activado](https://github.com/rust-lang/rust-analyzer/pull/21675)
* [arreglar otro caso donde olvidamos poner el param de tipo para 'PartialOrd' y 'PartialEq' en derivados incorporados](https://github.com/rust-lang/rust-analyzer/pull/21692)
* [corrigir predicados de los rasgos derivados incorporados con dos parámetros que por defecto son 'Self'](https://github.com/rust-lang/rust-analyzer/pull/21652)
* [Generar método Assist utiliza el bloque impl encerrado en lugar de First Found](https://github.com/rust-lang/rust-analyzer/pull/21684)
* [sin sugerir param completo en patrón complejo](https://github.com/rust-lang/rust-analyzer/pull/21650)
* [ofrece 'toggle_macro_delimiter' en macro anidado](https://github.com/rust-lang/rust-analyzer/pull/21536)
* [evitar nombres de parámetros calificativos en 'add_missing_impl_members'](https://github.com/rust-lang/rust-analyzer/pull/21665)
* [implementa 'Span::SpanSouce' para proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21657)

### Triaje de rendimiento del compilador Rust

En general, un poco más de ruido de lo habitual esta semana, pero sobre todo una ligera mejora
con varias optimizaciones de bajo nivel en MIR y LLVM IR Building Landing. También
menos commits que aterrizasen de lo habitual, principalmente por problemas de CI en GitHub durante la semana.

Triaje hecho por **@simulacrum**.
Rango de revisión: [3c9faa0d.. eeb94be7](https://perf.rust-lang.org/?start=3c9faa0d037b9eecda4a440cc482ff7f960fb8a5&end=eeb94be79adc9df7a09ad0b2421f16e60e6d932c&absolute=false&stat=instructions%3Au)

3 regresiones, 4 mejoras, 4 mixtas; 3 de ellos en rollups
24 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-02-23.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [¡Puerta #! [reexport_test_harness_main] correctamente](https://github.com/rust-lang/rust/pull/152210)
* [Observar errores 'close(2)' para 'std::fs::{copy, write}'](https://github.com/rust-lang/rust/pull/149834)
* [advertencia con precisión vacía](https://github.com/rust-lang/rust/pull/136638)
* [refactorización 'válido para lectura/escritura': excluir nulo](https://github.com/rust-lang/rust/pull/152615)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Quitar -Csoft-float](https://github.com/rust-lang/compiler-team/issues/971)
* [Intrínseca cg_ssa sin lugar](https://github.com/rust-lang/compiler-team/issues/970)
* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [dote(ayuda): mostrar manpage para comandos anidados](https://github.com/rust-lang/cargo/pull/16432)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [Carga: pistas.min-opt-level](https://github.com/rust-lang/rfcs/pull/3924)
* [RFC de carga para la edad mínima de publicación](https://github.com/rust-lang/rfcs/pull/3923)
* [Rasgos del lugar](https://github.com/rust-lang/rfcs/pull/3921)
* [RFC: Extender las dependencias del manifiesto con used](https://github.com/rust-lang/rfcs/pull/3920)

## Próximos eventos

Eventos Rusty entre el 25-02-2026 - el 25-03-2026 🦀

### Virtual
* 2026-02-25 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Empezando con Rust Parte 3: Patrones y Coincidencia**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313391012/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/fvcjjuv8)
* 2026-02-26 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455923/)
* 2026-03-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/313303094/)
* 05-03-2026 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Presentación: Tock OS Parte #3 - Cápsulas y controladores de hardware de nivel inferior**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 05-03-2026 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763908777)
* 2026-03-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (Londres, Reino Unido)| [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/312799450/)
* 2026-03-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/cgzfpzcp)
* 2026-03-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjcfbwb/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/45qqc2eo)
* 2026-03-18 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Incrustado**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Adopción de Rust, Seguridad y Nube con Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Cajas, consejos y trucos Charlas relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/me4jwgxu)

### Asia
* 222-03-2026 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**Rust presencial marzo 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)

### Europa
* 2026-02-25 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #65 Patrocinado por Factbird**](https://www.meetup.com/copenhagen-rust-community/events/313341944/)
* 2026-02-26 | Praga, CZ | [Rust República Checa](https://www.meetup.com/rust-czech-republic/events/)
    * [**Informační teorie vs. filtry: Proč filtrování bitcoinového mempoolu NEFUNGUJE**](https://www.meetup.com/rust-czech-republic/events/313323947/)
* 28-02-2026 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Fika Forum #24 de Ferris - edición crablings**](https://www.meetup.com/stockholm-rust/events/313367881/)
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**Rust en MWC Talent Arena — Talleres + Encuentro Comunitario**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburgo, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn marzo 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Discos, Destrozados sobre Hielo: Una introducción al parquet y el iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)
* 05-03-2026 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Hack'n'Learn de Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/313464558/)
* 2026-03-11 | Ámsterdam, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/313426708/)
* 2026-03-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup - Introducción a Embedded Rust - Marzo**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-2026 | [Rustikon](https://www.rustikon.dev/)
    * [**Conferencia Rustikon**](https://www.rustikon.dev/)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de Hack - Advenimiento del Código**](https://www.meetup.com/rust-aarhus/events/313284304/)

### Norteamérica
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
* 2026-03-14 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo North End Rust, 14 de marzo**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en Persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcfbwb/)
* 2026-03-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Marzo 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-21 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Porter Square Rust, 21 de marzo**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/xvkdgtyjcfbhc/)

### Oceanía
* 2026-03-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Encuentro de marzo por determinar**](https://www.meetup.com/rust-melbourne/events/313471749/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Esto en realidad es solo que Rust añade soporte para plantillas tipo duck al estilo C++, y la información larga y mayormente irrelevante contenida en el mensaje ICE forma parte de la experiencia.

– [robofinch sobre usuarios de Rust](https://users.rust-lang.org/t/cheat-code-for-bypassing-trait-bounds/138402/3)

¡Gracias a [Kyllingene](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1754) por la sugerencia!

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1rftaij/this_week_in_rust_640/)</small>
