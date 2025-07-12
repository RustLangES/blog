---
title: "Esta semana en Rust #67"
number_of_week: 67
description: El crate de esta semana es flac-codec.
date: 2025-07-09
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial
* [Funciones desnudas estabilizadoras](https://blog.rust-lang.org/2025/07/03/stabilizing-naked-functions/)

### Actualizaciones de proyectos/herramientas
* [toml v0.9](https://epage.github.io/blog/2025/07/toml-09/)
* [Anunciando dynify: Pin-init trait objects en la pila en Rust estable](https://www.reddit.com/r/rust/s/Xinstp8iSx)
* [¿Qué hay de nuevo en Ratatui 0.30.0?](https://www.youtube.com/live/nucLyibb5NU?si=copCqnEK7X5ZC9ry)
* [Presentamos a Tyr, un nuevo controlador DRM de Rust](https://www.collabora.com/news-and-blog/news-and-events/introducing-tyr-a-new-rust-drm-driver.html)
* [RustFS: almacenamiento de objetos distribuidos y alternativa a MinIO escrito en Rust](https://github.com/rustfs/rustfs)
* [Rama 0.3.0-alpha.1 - Un Salto Adelante en el Protocolo](https://github.com/plabayo/rama/discussions/622)
* [Análisis de datos de streaming, versión 0.18.1 de Fluvio](https://www.fluvio.io/news/this-week-in-fluvio-0077)

### Observaciones/Pensamientos
* [Bootstrapping Rust con GCC](https://fractalfir.github.io/generated_html/cg_gcc_bootstrap.html)
* [El viaje hacia el mejor manejo de errores en los frameworks web de Rust](https://mackow.ski/blog/towards-rust-web-best-errors/)
* [Falta de solidez y características accidentales en el atributo #\[target_feature\](https://predr.ag/blog/unsoundness-and-accidental-features-in-target-feature/)
* [Simultaneidad estructurada en árbol II: Sustitución de tareas en segundo plano por actores](https://blog.yoshuawuyts.com/replacing-tasks-with-actors/)
* [Prototipado rápido de aprendizaje automático en Rust](https://ryuru.com/rapid-machine-learning-prototyping-in-rust/)
* [330× más rápido: Cuatro formas diferentes de acelerar tu código](https://pythonspeed.com/articles/different-ways-speed/)
* [Programación de tipos de datos extensibles en Rust con CGP, Parte 1](https://contextgeneric.dev/blog/extensible-datatypes-part-1/) y [Parte 2](https://contextgeneric.dev/blog/extensible-datatypes-part-2/)
* [Hacer que el Rust inseguro sea un poco más seguro: encuentre errores de memoria en producción con GWP-ASan](https://blog.colinbreck.com/making-unsafe-rust-a-little-safer-find-memory-errors-in-production-with-gwp-asan/)
* [Pruebas de simulación determinista en Rust: un teatro de máquinas de estado](https://www.polarsignals.com/blog/posts/2025/07/08/dst-rust)
* [Avance técnico de los componentes del material de Slint](https://slint.dev/blog/material-comp-tech-preview)
* [Malware seguro para la memoria: Rust desafía a los investigadores de seguridad](https://www.techzine.eu/blogs/security/132626/memory-safe-malware-rust-challenges-security-researchers/)
* [La aterradora y sorprendentemente profunda madriguera del conejo de los temporales de Rust](https://taping-memory.dev/temporaries-rabbit-hole/)
* [Los 4 pilares de la superioridad de Rust](https://kerkour.com/rust-superiority)

### Tutoriales de Rust
* [JavaScript está siendo reescrito en Rust](https://endform.dev/blog/js-is-being-rewritten-in-rust/)
* [Mejores prácticas de seguridad de Rust 2025](https://corgea.com/Learn/rust-security-best-practices-2025)
* [De 'Podría funcionar' a 'Funcionará': estado de tipo en Rust](https://minikin.me/blog/typestate-in-rust)
* [Confianza táctica (1 de 2): Plataforma criptográfica para desarrolladores en Rust](https://tiemoko.com/blog/safer-crypto/)

## Crate de la semana

El crate de esta semana es [flac-codec](https://crates.io/crates/flac-codec), una implementación de (como era de esperar) el formato de archivo Free Lossless Audio Codec (FLAC).

¡Gracias a [tuffy](https://users.rust-lang.org/t/crate-of-the-week/2704/1449) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [rama - Implementar todos los algoritmos JWA](https://github.com/plabayo/rama/issues/621)
* [rama - soporte X-Clacks-Overhead (response) http header](https://github.com/plabayo/rama/issues/620)
* [rama - Solicitud de función: Enlace IPv4/IPv6 local dinámico para TcpConnector](https://github.com/plabayo/rama/issues/595)
* [rama - añadir módulo curl a rama-http-types](https://github.com/plabayo/rama/issues/509)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

588 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-07-01..2025-07-08

#### Compilador
* [reescribir el analizador 'macro_rules!' para que no use el motor MBE en sí](https://github.com/rust-lang/rust/pull/143070)
* [Coincidencia de bucle: arreglar 'No hay terminador en el bloque'](https://github.com/rust-lang/rust/pull/143583)
* [Coincidencia de bucle: Manejar patrones opacos](https://github.com/rust-lang/rust/pull/143276)
* [Alinear correcciones de attr](https://github.com/rust-lang/rust/pull/143206)
* [evite sugerir rasgos de dependencias privadas](https://github.com/rust-lang/rust/pull/143038)
* [detectar más casos de 'unused_parens' alrededor de los tipos](https://github.com/rust-lang/rust/pull/142237)
* [convertir los metadatos en un producto de trabajo y reutilizarlos](https://github.com/rust-lang/rust/pull/114669)
* [eliminar algunos 'inseguros' innecesarios en VecCache](https://github.com/rust-lang/rust/pull/143406)
* [empezar a mover la comprobación de wf fuera de HIR](https://github.com/rust-lang/rust/pull/142030)

#### Biblioteca
* [estabilizar 'mixed_integer_ops_unsigned_sub'](https://github.com/rust-lang/rust/pull/143236)
* [añadir 'Vec::into_chunks'](https://github.com/rust-lang/rust/pull/142138)
* [añadir 'const Rem'](https://github.com/rust-lang/rust/pull/143040)
* [añadir métodos para convertir bool a 'Resultado<(), E>'](https://github.com/rust-lang/rust/pull/142749)
* [cambiar '{Box,Arc,Rc,Weak}::into_raw' para que solo funcione con 'A = Global'](https://github.com/rust-lang/rust/pull/141219)

#### Carga
* [publicar: Estabilizar la publicación de múltiples paquetes](https://github.com/rust-lang/cargo/pull/15636)
* [implementar la unificación de características del paquete](https://github.com/rust-lang/cargo/pull/15684)
* [informe un nombre de archivo válido cuando no podemos encontrar un objetivo de compilación para 'name = "foo.rs"'](https://github.com/rust-lang/cargo/pull/15707)

#### Miri
* [Mejorar los errores de aserción de validez de tipos](https://github.com/rust-lang/rust/pull/143327)
* ['shims::fs' añadiendo más campos a FileMetadata](https://github.com/rust-lang/miri/pull/4444)

#### Rustdoc
* [arreglar que rustdoc no mostraba correctamente los atributos en las reexportaciones](https://github.com/rust-lang/rust/pull/143083)
* [no tratar los métodos bajo const impls o traits como const](https://github.com/rust-lang/rust/pull/143381)

#### Clippy
* ['neg_multiply' debe conservar el paréntesis cuando se llama al método](https://github.com/rust-lang/rust-clippy/pull/15179)
* ['doc_nested_refdefs': no reportar falsamente casillas de verificación como refdefs](https://github.com/rust-lang/rust-clippy/pull/15146)
* ['or_fun_call': también lint 'y' método para 'Opción'/'Resultado'](https://github.com/rust-lang/rust-clippy/pull/15073)
* [no peludar intrínsecos como bucles vacíos](https://github.com/rust-lang/rust-clippy/pull/15201)
* [no eliminar 'as' si cambia el tipo](https://github.com/rust-lang/rust-clippy/pull/15182)
* [No eliminar el puntero de conversión explícita a objeto de rasgo](https://github.com/rust-lang/rust-clippy/pull/15145)
* [Arreglar el falso positivo 'std-instead-of-core' cuando no todos los artículos provienen de la nueva caja](https://github.com/rust-lang/rust-clippy/pull/15165)
* ['redundant_closure_call': pelusa para tapones con bloque](https://github.com/rust-lang/rust-clippy/pull/15144)
* [mejorar la pelusa de punto flotante para manejar el tipo ambiguo](https://github.com/rust-lang/rust-clippy/pull/15133)
* [manejar enlaces potencialmente sombreados en 'manual_let_else'](https://github.com/rust-lang/rust-clippy/pull/15118)
* [propagar 'accept-comment-above-attributes' a las declaraciones](https://github.com/rust-lang/rust-clippy/pull/15213)
* ['return_and_then': evitar falsos positivos en caso de una expresión utilizada parcialmente](https://github.com/rust-lang/rust-clippy/pull/15115)

#### Analizador de Rust
* [añadir 'AsMut', 'Borrow' y 'BorrowMut' a minicore y 'famous_defs'](https://github.com/rust-lang/rust-analyzer/pull/20132)
* [añadir la variante 'fn load_workspace_into_db' para la fn 'load_workspace' de 'ra_ap_load_cargo](https://github.com/rust-lang/rust-analyzer/pull/20144)
* [siempre empareja '--compile-time-deps' con](https://github.com/rust-lang/rust-analyzer/pull/20159)
* [incluir variantes de 'enum' en los símbolos del mundo](https://github.com/rust-lang/rust-analyzer/pull/20185)
* [arreglar un caso donde el tipo de enlace era 'Ninguno'](https://github.com/rust-lang/rust-analyzer/pull/20192)
* [arreglar tareas de desestructuración divergente](https://github.com/rust-lang/rust-analyzer/pull/20179)
* [arreglar algunas cosas con derivaciones incorporadas](https://github.com/rust-lang/rust-analyzer/pull/20167)
* [respeta la opción 'rust-analyzer.cargo.noDeps' al obtener metadatos de sysroot](https://github.com/rust-lang/rust-analyzer/pull/20148)
* [mejorar los rangos de diagnóstico para 'macro_calls!'](https://github.com/rust-lang/rust-analyzer/pull/20160)
* [Corregir la captura de cierre incorrecta para let exprs](https://github.com/rust-lang/rust-analyzer/pull/20161)
* [resuelva los problemas de longitud de la pantalla HIR y mejore la información sobre herramientas de ajuste](https://github.com/rust-lang/rust-analyzer/pull/20031)
* [Resolver elemento en enlace de coincidencia](https://github.com/rust-lang/rust-analyzer/pull/20120)
* [Mejorar los informes de progreso de flycheck y build script](https://github.com/rust-lang/rust-analyzer/pull/20170)
* [volver a habilitar la iteración de punto fijo para el cálculo de varianza](https://github.com/rust-lang/rust-analyzer/pull/20157)
* [reestructurar proc-macro cargando erros, diferenciar la propiedad de error duro en el tipo](https://github.com/rust-lang/rust-analyzer/pull/20156)
* [Solución alternativa que falta soporte para grupos ninguno en macros incorporadas](https://github.com/rust-lang/rust-analyzer/pull/20112)

### Clasificación del rendimiento del compilador de Rust

Semana ajetreada. Los resultados están dominados por cambios que intercambian algunas ganancias por algunas pérdidas en pequeños escenarios incrementales. También tuvimos mucho ruido y pequeños cambios espurios en varios PR. Algunas regresiones provienen del trabajo relacionado con el rendimiento, en el que esperamos recuperar algunas victorias más adelante.

Triaje realizado por **@panstromek**.
Rango de revisión: [ad3b7257.. 0d11be5a](https://perf.rust-lang.org/?start=ad3b7257615c28aaf8212a189ec032b8af75de51&end=0d11be5aabe0cd49609fff5fce57c4691a22fe55&absolute=false&stat=instructions%3Au)

**Resumen**:

Nota: Cambiamos a una nueva máquina de referencia al comienzo del período. Mostramos un resumen basado en un rango ligeramente ajustado [6988a8fe.. 8df4a58a](https://perf.rust-lang.org/?start=6988a8fea774a2a20ebebddb7dbf15dd6ef594f9&end=8df4a58ac47b778b093652d6190a6f9d54638774&absolute=false&stat=instructions%3Au) para evitar comparaciones engañosas de diferentes máquinas.

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 1.1% | [0.2%, 4.3%] | 128 |
| Regresiones ❌ <br /> (secundaria) | 1.0% | [0.2%, 3.9%] | 84 |
| Mejoras ✅ <br /> (primario) | -3,5% | [-7.2%, -0.2%] | 48 |
| Mejoras ✅ <br /> (secundaria) | -5,1% | [-42.6%, -0.2%] | 68 |
| Todos ❌✅ (primarios) | -0,2% | [-7,2%, 4,3%] | 176 |

3 regresiones, 3 mejoras, 11 mixtas; 6 de ellos en rollups
44 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/62972dca0429c46843c2569130670ddea8dfb92b/triage/2025/2025-07-07.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [Macros de derivación declarativas 'macro_rules!](https://github.com/rust-lang/rfcs/pull/3698)
* [Macros de atributo declarativas 'macro_rules!](https://github.com/rust-lang/rfcs/pull/3697)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para 'const_slice_reverse'](https://github.com/rust-lang/rust/issues/135120)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [&str y &[u8] tienen el mismo diseño](https://github.com/rust-lang/reference/pull/1848)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-07-09 - 2025-08-06 🦀

### Virtual
* 13/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust asíncrono**](https://www.meetup.com/dallasrust/events/308298512)
* 15/07/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Puesta al día de la comunidad**](https://www.meetup.com/women-in-rust/events/307560349)
* 15/07/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/306757755)
* 16/07/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307731031)
* 17/07/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 17/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Cajas, Consejos y Trucos Charlas Relámpago - ¡Trae tus ideas!**](https://www.meetup.com/women-in-rust/events/307560304)
* 24/07/2025 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874)
* 27/07/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhckbkc)
* 31/07/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820306)
* 02/08/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763838567)
* 03/08/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/bhctrtyhclbfb)
* 06/08/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhclbjb)

### Asia
* 26/07/2025 | Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de julio de 2025**](https://hasgeek.com/rustbangalore/july-2025-rustacean-meetup/)

### Europa
* 09/07/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 07 2025**](https://lu.ma/hismn492)
* 09/07/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/308705791)
* 10/07/2025 | Berlín, DE | [Rust Berlín](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin en locación 🏳️ 🌈 - Edición 004**](https://www.meetup.com/rust-berlin/events/308987361)
* 15/07/2025 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592246)
* 15/07/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**TUI Power: Simulación y visualización de datos de sensores con Rust**](https://www.meetup.com/london-rust-project-group/events/308434768)
* 23/07/2025 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/)
    * [**Rust Dortmund Meetup - Enseñar y Hackear**](https://www.meetup.com/rust-dortmund/events/308517530/)
* 24/07/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi)
    * [**Charlas de julio: Un cangrejo, un pez globo y una IA de ajedrez de última generación**](https://www.meetup.com/rust-and-friends/events/308687848)
* 24/07/2025 | Núremberg/Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567874/)
* 29/07/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester)
    * [**Lightning Talks julio de 2025**](https://www.meetup.com/rust-manchester/events/308085035)
* 29/07/2025 | Praga, CZ | [Rust República Checa](https://www.meetup.com/rust-czech-republic)
    * [**Nix Meetup en Braiins :)**](https://www.meetup.com/rust-czech-republic/events/308963318)
* 30/07/2025 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/308548455)
* 31/07/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #14: Prof. Dra. Claudia Meitinger - Embajada - Möglichkeiten und Herausforderungen im Modul "Proyecto Interdisciplinario"**](https://rust-augsburg.github.io/meetup/Meetup_14.html)

### América del Norte
* 09/07/2025 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> AI**](https://www.meetup.com/desert-rustaceans/events/308507249)
* 10/07/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/308277549)
* 10/07/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Reunión de julio de 2025 PDX Rust**](https://www.meetup.com/pdxrust/events/309056548)
* 15/07/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307931266)
* 16/07/2025 | Híbrido (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/307731031)
* 17/07/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/308979091)
* 17/07/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Julio, 2025 Panel de Lenguaje de Programación Informática (Evento Especial)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 17/07/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Rust on Bare Metal Series 1 : Introducción al Desarrollo Embebido**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 23/07/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/308791385)
* 24/07/2025 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/xdxtqtyhckbgc)
* 31/07/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675947)

### América del Sur
* 2025-07-12 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 17/07/2025 | Florianópolis, BR | [Rust Brasil + Rust Floripa](https://lu.ma/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://lu.ma/p0umq6vm)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1llcso7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Te estarás preguntando: ¿por qué reescribiste \[...\] en Rust? Y sí, realmente no tengo una buena razón. Es un proyecto de hobby. Como la jardinería, pero con más defectos.

– [Collin Richards en su blog](https://richardscollin.github.io/tmux-rs/)

¡Gracias a [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1701) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1lw4wew/this_week_in_rust_607/)</small>

