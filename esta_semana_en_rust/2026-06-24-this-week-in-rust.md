---
title: "Esta semana en Rust #115"
number_of_week: 115
description: El crate de esta semana es cargo-rdme, un comando de carga para crear tu README a partir de la documentación de tu caja.
date: 2026-06-24
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

### Fundación
* [Rust Foundation da la bienvenida a OpenAI como miembro platino](https://rustfoundation.org/media/rust-foundation-welcomes-openai-as-platinum-member-announces-donation-to-rust-project/)
* [Lancement de la red comercial de Rust para unir a usuarios comerciales de Rust](https://rustfoundation.org/media/rust-commercial-network-launches-to-bring-commercial-users-of-rust-language-together/)
* [Lo principal es traer entrenamiento práctico en Rust](https://rustfoundation.org/media/mainmatter-is-bringing-hands-on-rust-training-to-upskilling-week-in-barcelona/)

### Boletines
* [El Rustacean Incrustado Número #74](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-74)

### Actualizaciones de proyectos/herramientas
* [Bevy 0,19](https://bevy.org/news/bevy-0-19)
* [La caja PNG de Rust se vuelve aún más rápida, usada por GNOME y Chromium](https://blog.image-rs.org/2026/06/18/png-adoption.html)
* [kache 0.7.0: almacenamiento en caché de árboles C/C++ del mundo real](https://github.com/kunobi-ninja/kache/releases/tag/v0.7.0)
* [Nueva función en GuardianDB: Introducción de la capa ODM (Object Document Mapper)](https://www.willsearch.com.br/blog/2026/06/23/new-feature-in-guardiandb-introducing-the-odm-object-document-mapper-layer/)
* [SIMD seguro en Rust, incluso por dentro](https://shnatsel.medium.com/safe-simd-in-rust-even-on-the-inside-c6f1ff381828)
* [Se lanza Ratatui 0.30.2 - una biblioteca de Rust para crear interfaces de usuario de terminal](https://ratatui.rs/highlights/v0302/)
* [Añadiendo un handshake híbrido post-cuántico a una VPN Rust](https://dev.to/alexandr_litvinov/adding-a-post-quantum-hybrid-handshake-to-a-rust-vpn-pk8)
* [De Julia a Rust: una pila tensorial diferenciable para la computación científica en la era de la IA agente](https://tensor4all.org/blog/introducing-tenferro-rs/)
* [hotpath-rs 0.18: Perfilado de Rust asíncrono y concurrente - Canales y contención de bloqueo](https://hotpath.rs/blog/profiling-async-rust)

### Observaciones/Pensamientos
* [Cómo encontramos un error en la biblioteca hyper HTTP](https://blog.cloudflare.com/hyper-bug/)
* [ClickHouse con Alexey Milovidov y Austin Bonander](https://corrode.dev/podcast/s06e06-clickhouse/)
* [¿Profundización en iroh: ¿Un sustituto de WireGuard o una capa peer-to-peer para tu aplicación?](https://kerkour.com/iroh-v1-p2p)
* [Optimizando #\[sqlx::test\] tiempo de reconstrucción](https://kobzol.github.io/rust/2026/06/21/optimizing-sqlx-test-rebuild-time.html)
* [Reescribiendo el mundo en Rust](https://bitfieldconsulting.com/posts/rewrite-in-rust)

### Guías de Rust
* [Migrando LiteLLM a Rust - Construcción del Gateway de IA más rápido y ligero](https://docs.litellm.ai/blog/litellm-rust-launch)
* [SIMD seguro en Rust, incluso por dentro](https://medium.com/@shnatsel/safe-simd-in-rust-even-on-the-inside-c6f1ff381828)
* [Aprende Rust Async/Await, Tokio y redes TCP construyendo un servidor HTTP/1.1](https://blog.sheerluck.dev/posts/learn-rust-async-await-by-building-an-http-server/)
* [Ruptura de construcción en Bevy: Paso a paso](https://blog.sheerluck.dev/posts/build-breakout-in-bevy-step-by-step/)
* [Portando 300.000 líneas de C++ y Perl a Rust: un motor de metadatos multimedia dual-oracle](https://medium.com/@vbasky/porting-200-000-lines-of-c-to-rust-building-a-byte-identical-mediainfo-replacement-8e9b587d469a)
* [Una carrera de datos que no se compila](https://corentin-core.github.io/posts/ruxe-type-level-disjointness/)
* [vídeo] [lección 9 de RustCuriose: Los rasgos son Interfaces](https://www.youtube.com/watch?v=RKojTb9IVJc)
* [Vídeo] [BAML: un nuevo lenguaje de programación (creado en Rust)](https://www.youtube.com/watch?v=X8GDc2AtbG8)
* [Vídeo] [El futuro del control de versiones](https://www.youtube.com/watch?v=O3YWQvNqwHc)
* [Vídeo] [Borrowing Beauty: Mi búsqueda de principiante para crear un código Bevy & Rust accesible](https://www.youtube.com/watch?v=1Xz1E_27Uqc)

## Crate de la semana

El crate de esta semana es [cargo-rdme](https://github.com/orium/cargo-rdme), un comando de carga para crear tu README a partir de la documentación de tu caja.

¡Gracias a [Diogo Sousa](https://users.rust-lang.org/t/crate-of-the-week/2704/1616) por la autosugerencia!

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

* [AimDB - try_produce' falible no bloqueante para búferes acotados / no sobrscribibles](https://github.com/aimdb-dev/aimdb/issues/116)
* [AimDB - Añadir ejemplo mínimo: hello-mailbox-async](https://github.com/aimdb-dev/aimdb/issues/99)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

515 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-16..2026-06-23

#### Compilador
* [implementar '#[diagnostic::on_unknown]' para módulos](https://github.com/rust-lang/rust/pull/157926)
* [contornar parte de 'evaluate_goal_raw' en su propia función '#[frío]](https://github.com/rust-lang/rust/pull/158042)
* [preservar 'track_caller' para cuñas Dyn Vtable por valor por subvalor](https://github.com/rust-lang/rust/pull/157967)

#### Biblioteca
* [añadir 'io::Leer::read_le' y 'io::Leer::read_be'](https://github.com/rust-lang/rust/pull/156983)
* [constificar 'TryFrom<Vec>' para el array](https://github.com/rust-lang/rust/pull/155616)
* ['impl [const] Predeterminado para BTreeMap'](https://github.com/rust-lang/rust/pull/157878)
* [estabilizar 'str_from_utf16_endian'](https://github.com/rust-lang/rust/pull/157912)
* [estabilizar 'strip_circumfix'](https://github.com/rust-lang/rust/pull/158012)
* [estabilizar 'substr_range' y 'subslice_range'](https://github.com/rust-lang/rust/pull/141266)

#### Carga
* ['diag': Soporte 'build.warnings' para los lints de carga](https://github.com/rust-lang/cargo/pull/17112)
* ['añadir': listar las versiones nuevas y cómo anular](https://github.com/rust-lang/cargo/pull/17117)
* ['host-config': no aplicar la configuración de destino a los artefactos del anfitrión](https://github.com/rust-lang/cargo/pull/17123)
* ['instalar': Haz funcionar los lints de carga como los rustc lints](https://github.com/rust-lang/cargo/pull/17107)
* ['resolver': pista para resolver versiones demasiado nuevas](https://github.com/rust-lang/cargo/pull/17118)
* ['prueba': Salta la prueba de levantamiento DWP sin debuginfo empaquetada](https://github.com/rust-lang/cargo/pull/17127)
* [añadir bloqueo de archivo Solaris fcntl](https://github.com/rust-lang/cargo/pull/17110)
* ['-Zmin-public-age'](https://github.com/rust-lang/cargo/pull/17012) (RFC [#3923](https://rust-lang.github.io/rfcs/3923-cargo-min-publish-age.html))
* [mejoró los mensajes de error de prueba cuando falla 'rustc -V'](https://github.com/rust-lang/cargo/pull/17108)
* [eliminar dependencias de windows-sys anteriores a 0.61](https://github.com/rust-lang/cargo/pull/17115)

#### Clippy
* [añadir pelusas para sugerir 'as_chunks' sobre 'chunks_exact' con constante](https://github.com/rust-lang/rust-clippy/pull/16931)
* [nuevo 'unnecessary_unwrap_unchecked': pelusa](https://github.com/rust-lang/rust-clippy/pull/16252)
* ['extra_unused_type_parameters': no sugieres una corrección automática](https://github.com/rust-lang/rust-clippy/pull/15907)
* ['let_underscore_future': saltar enlaces con una anotación explícita de tipo](https://github.com/rust-lang/rust-clippy/pull/17001)
* [evitar ICE al evaluar constantes que contienen args de tipo sin tamaño](https://github.com/rust-lang/rust-clippy/pull/16976)
* [evitar la corrección de 'map_unwrap_or' cuando se ajusta el valor predeterminado](https://github.com/rust-lang/rust-clippy/pull/16928)
* [no compruebes las vidas no utilizadas en el código expandido](https://github.com/rust-lang/rust-clippy/pull/17256)
* [no activan 'unnecessary_box_returns' cuando el tamaño depende de los genéricos](https://github.com/rust-lang/rust-clippy/pull/17249)
* [encontrar un contexto compartido para la cadena de formato y la llamada 'format!'](https://github.com/rust-lang/rust-clippy/pull/17243)
* [arreglar el pánico OOM para tipos grandes con la comprobación de uninit](https://github.com/rust-lang/rust-clippy/pull/17205)
* [corregir 'std_instead_of_core': falsos positivos para 'core::io'/MSRV](https://github.com/rust-lang/rust-clippy/pull/16964)
* ['manual_slice_fill' detectar para en bucles sobre '&mut [T; N]' cortes](https://github.com/rust-lang/rust-clippy/pull/16926)
* [Fusionar comentarios y comprobación CFG en 'matches' lint pass](https://github.com/rust-lang/rust-clippy/pull/17239)
* [perf: comprueba el nombre del método primero en 'or_fun_call'](https://github.com/rust-lang/rust-clippy/pull/17266)
* [perf: comparar nombres de métodos antes de consultas de tipo en tres pasadas de lint](https://github.com/rust-lang/rust-clippy/pull/17265)
* [PERF: realiza comprobaciones estructurales antes de consultas de contexto de const en 'question_mark, manual_clamp' y rangos](https://github.com/rust-lang/rust-clippy/pull/17275)
* [PERF: saltarse el trabajo de 'match_same_arms' cuando se permite la pelusa](https://github.com/rust-lang/rust-clippy/pull/17272)
* [perf: salta la tokenización en 'span_contains_cfg' cuando no hay '#' presente](https://github.com/rust-lang/rust-clippy/pull/17226)
* [trata '!' igual que '-' en 'unnecessary_cast'](https://github.com/rust-lang/rust-clippy/pull/17278)

#### Analizador de Rust
* ['asistencia/replace_match_with_if_let': no pongáis paréntesis a los guardias if-let](https://github.com/rust-lang/rust-analyzer/pull/22618)
* ['implements_trait_unique_with_infcx': solo prohíbe que el yo sea tipo error](https://github.com/rust-lang/rust-analyzer/pull/22617)
* [adiós, adiós, Ted](https://github.com/rust-lang/rust-analyzer/pull/22516)
* [no visitar nodos en GC varias veces](https://github.com/rust-lang/rust-analyzer/pull/22627)
* [Tamaños mixtos de bit y byte de evaluación MIR](https://github.com/rust-lang/rust-analyzer/pull/22594)
* [comprueba '#[cfg]s' en macros de expresión de cola](https://github.com/rust-lang/rust-analyzer/pull/22599)
* [fallo en constantes estáticas en posiciones de longitud de array](https://github.com/rust-lang/rust-analyzer/pull/22601)
* [no completar '.await' en receptores de tipo desconocido](https://github.com/rust-lang/rust-analyzer/pull/22486)
* [no te pongas nervioso con los literales enteros fuera de rango en posiciones de const](https://github.com/rust-lang/rust-analyzer/pull/22621)
* [migrar fusión importando al editor](https://github.com/rust-lang/rust-analyzer/pull/22351)

### Triaje de rendimiento del compilador Rust

Esta semana ha tenido muchos cambios importantes, con dos regresiones significativas de rendimiento que se aceptan
Porque desbloquean funciones futuras y mejoras en el rendimiento.
También vimos grandes mejoras en el siguiente solucionador de rasgos debido al trabajo de optimización del rendimiento que se realiza allí.

Triaje realizado por **@JonathanBrouwer** con ayuda de **@Kobzol**.
Rango de revisión: [b5d46ecb.. 8b6558a0](https://perf.rust-lang.org/?start=b5d46ecb51c3e4134b82570cfe718f093daa6390&end=8b6558a02b2774acfb25cf15e199467c37ba7490&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:------:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,9% | [0,2%, 2,7%] | 184 |
| Regresiones ❌ <br /> (secundario) | 1,0% | [0,1%, 4,2%] | 160 |
| Mejoras ✅ <br /> (primaria) | -0,3% | [-0,3%, -0,2%] | 2 |
| Mejoras ✅ <br /> (secundario) | -11,8% | [-69,9%, -0,2%] | 25 |
| Todos ❌✅ (primario) | 0,8% | [-0,3%, 2,7%] | 186 |

5 regresiones, 3 mejoras, 2 mixtas; 4 de ellos en rollups
30 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/660052c17ccde865dff7c7ffd525affa0550c846/triage/2026/2026-06-21.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [rustc_lint: Permitir niveles de pelusa 'non_ascii_idents' con alcance visual](https://github.com/rust-lang/rust/pull/157497)
* [Estabilizar '#[my_macro] mod foo;' (parte de 'proc_macro_hygiene')](https://github.com/rust-lang/rust/pull/157857)
* [Implementa 'IntoIterator' para '[&[mut]] Box<[T; N], A>'](https://github.com/rust-lang/rust/pull/134021)
* [Problema de seguimiento para 'string_from_utf8_lossy_owned'](https://github.com/rust-lang/rust/issues/129436)
* [Inferir todas las vidas anónimas en consts asociadas como ''estática'](https://github.com/rust-lang/rust/pull/156508)
* [considera subtipar al comprobar si un var infer tiene el tamaño](https://github.com/rust-lang/rust/pull/157820)
* [eliminar 'box_patterns'](https://github.com/rust-lang/rust/pull/156749)
* [activar la norma 'param_env' de eager en el nuevo solver](https://github.com/rust-lang/rust/pull/156976)
* [Pelusa contra funciones iteradoras que entran en pánico cuando 'N' es cero](https://github.com/rust-lang/rust/pull/153563)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Iniciar un t-proyecto-estructura/t-comprensibilidad](https://github.com/rust-lang/leadership-council/issues/298)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de compilación](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 24-06-2026 y el 22-07-2026 🦀

### Virtual
* 2026-06-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión Semanal de Rust Girona**](https://luma.com/rust-girona?e=evt-rgneLvX1H85AmjV)
* 2026-07-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Aprendiendo desarrollo de juegos por las malas con Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-04 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-07-05 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Aprendiendo el Rust como primer lenguaje de programación**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/315279653/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de julio de 2026**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### Europa
* 2026-06-24 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-06-24 | Trondheim, NO | [Trondheim Oxidado](https://www.meetup.com/rust-trondheim)
    * [**El Caos del Tiempo y los Intervalos de Tiempo**](https://www.meetup.com/rust-trondheim/events/315298357/)
* 2026-06-25 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-06-25 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #69**](https://www.meetup.com/copenhagen-rust-community/events/315214426/)
* 2026-06-25 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse/)
    * [**Rust Toulouse Meetup - Bevy & ESP32**](https://www.meetup.com/rust-community-toulouse/events/314947457/)
* 2026-06-27 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #27**](https://www.meetup.com/stockholm-rust/events/315371143/)
* 2026-07-02 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Bevy, Bits, & Cats (Charlas de Rust July)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, NL | [Reuniones de Tecnología de Baseflow](https://www.meetup.com/dutch-rust-meetup)
    * [**Cumbre IA**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Únete en directo e INPERSONAL para Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)

### Norteamérica
* 2026-06-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Adiós**](https://www.meetup.com/rust-atx/events/315105633/)
* 2026-06-24 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Solucionadores de restricciones basados en Rust en bocetos 2D con Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-25 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825008/)
* 2026-06-26 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Gran Fiesta de Verano de Rust NYC**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-06-27 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en Union Square en Somerville, 27 de junio**](https://www.meetup.com/bostonrust/events/315225857/)
* 2026-07-02 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**¿Git es fácil?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Rust de la Universidad de Boston, 4 de julio**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de Utah Rust July**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, 11 de julio**](https://www.meetup.com/bostonrust/events/315225865/)
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo North End Rust, 18 de julio**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 222-07-2026 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 222-07-2026 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: ¡Rust en sistemas distribuidos con ciencia del vuelo!**](https://www.meetup.com/rust-los-angeles/events/315376271/)

### Oceanía
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne junio 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)
* 2026-07-21 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de julio**](https://www.meetup.com/rust-canberra/events/315307280/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> creo que esta es la decisión equivocada, y ojalá el equipo de Lang hubiera estabilizado el tipo Late en su lugar.
Más vale tarde que nunca.

– [/u/CouteauBleu en /r/rust](https://www.reddit.com/r/rust/comments/1u1v53c/the_never_type_is_likely_to_stabilize_soon/oqsxf3v/)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1782) por la sugerencia!

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

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1uewqig/this_week_in_rust_657/)</small>
