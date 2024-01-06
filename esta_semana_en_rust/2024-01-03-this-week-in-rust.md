---
title: "Esta semana en Rust #13"
number_of_week: 13
description: El crate de esta semana es fast\_pool, un grupo asíncrono rápido basado en la caja del canal de canal.
date: 2024-01-03
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Oficial
* [Anunciando Rust 1.75.0](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

### Actualizaciones de proyectos/herramientas
* [Rustdoc JSON en 2023](https://alona.page/posts/rustdoc-json-2023/)
* [Revisión de 2023: Establecimiento de Rust como lenguaje de Godot 4](https://godot-rust.github.io/dev/godot-rust-2023-review/)
* [Actualización de Rust9x: Rust 1.76.0-beta](https://seri.tools/blog/rust9x-1-76/)
* [Anunciando smol-macros, smol-hyper y smol-axum](https://notgull.net/new-smol-rs-subcrates/)
* [Informe de progreso del equipo de Rust Language Bootstrap 2023](https://onurozkan.dev/read/rust-language-bootstrap-team-progress-report-2023/)
* [gitoxide: El año en retrospectiva, y lo que está por venir](https://github.com/Byron/gitoxide/discussions/1223)

### Observaciones/Pensamientos
* [Algunas soluciones rápidas para Advent of Code 2023](https://jordankaye.dev/posts/aoc-2023/)
* [Una actualización sobre errores de seguridad en la memoria de escritura](https://orodu.net/2023/12/27/rust-abstractions.html)
* [avatar.png](https://tuckersiemens.com/posts/avatar-png/)
* [Arc vs String, ¿es Arc realmente más rápido?](https://blocklisted.github.io/blog/arc_str_vs_string_is_it_really_faster/)
* [Iggy.rs - Construyendo la transmisión de mensajes en Rust](https://blog.iggy.rs/posts/building-message-streaming-in-rust/)
* [Primeros pasos con Loco en Rust: Parte 1](https://www.shuttle.rs/blog/2023/12/28/using-loco-rust-rails)
* [Errores de impresión en Rust](https://heikoseeberger.de/2024-01-01-printing-errors/)
* [video] [Rust 1.75.0: ¡54 momentos destacados en 20 minutos!](https://youtu.be/Z8xig7wEV68)

### Miscelánea
* [Aplicación de línea de comandos de prueba escrita en Rust](https://rust.code-maven.com/test-command-line-application)
* [Aplicaciones de código abierto escritas en Rust](https://rust.code-maven.com/applications)
* [Prompt - entrada de lectura de la entrada estándar (STDIN) en Rust](https://rust.code-maven.com/prompt)
* [Probando tu Rust Incrustado (feat. embedded-hal-mock y explosiones)](https://barretts.club/posts/embedded-tests/)
* [video] [Rust Release Train 1.75](https://www.youtube.com/watch?v=wDzyLFT3AwY)
* [video] [Rust 1.75.0: 54 destacados en 20 minutos](https://www.youtube.com/watch?v=Z8xig7wEV68)

## Crate de la semana

El crate de esta semana es [fast\_pool](https://crates.io/crates/fast_pool), un grupo asíncrono rápido basado en la caja del canal de canal.

¡Gracias a [zhuxiujia](https://users.rust-lang.org/t/crate-of-the-week/2704/1276) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->
* [Hyperswitch - Implementar código cov para el sistema local usando makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - Cobertura de código de configuración para pruebas locales y CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - Agregar tipo de dominio para el secreto de cliente](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - Have get_required_value para usar ValidationError en OptionExt](https://github.com/juspay/hyperswitch/issues/860)
* [Ockam - Usa la API de GitHub para comprobar si la CLI está desactualizada](https://github.com/build-trust/ockam/issues/7312)
* [Ockam - refactorizar para usar interfaces tipadas para implementar comandos para 'servicios de kafka'](https://github.com/build-trust/ockam/issues/6706)
* [Ockam - Validar las estructuras CBOR de acuerdo con el esquema cddl para 'cloud/space' y 'cloud/subscription'](https://github.com/build-trust/ockam/issues/6687)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

194 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-27..2024-01-02

* ['rustc_lint': Forzar 'rustc::p otential_query_instability' lint](https://github.com/rust-lang/rust/pull/119251)
* ['rustc_lint': Evitar la triplicación de varias pelusas](https://github.com/rust-lang/rust/pull/119388)
* ['unused_bindings': también las fijaciones de paseo creadas por los protectores if-let](https://github.com/rust-lang/rust/pull/119402)
* [cambiar la interfaz 'atomic_cmpxchg' de 'rustc_codegen_ssa' para devolver un par de valores](https://github.com/rust-lang/rust/pull/118705)
* [cobertura: evitar un posible peligro de estabilidad de consultas en 'CoverageCounters'](https://github.com/rust-lang/rust/pull/119401)
* [Cobertura: Preparar asignaciones por separado de las instrucciones de inyección](https://github.com/rust-lang/rust/pull/119438)
* [cobertura: desexpandir intervalos con 'find_ancestor_inside_same_ctxt'](https://github.com/rust-lang/rust/pull/119336)
* [No sueltes un nodo hir después de bajar](https://github.com/rust-lang/rust/pull/119284)
* [No sugiero escribir un brazo sin cuerpo si el patrón nunca puede ser un patrón nunca](https://github.com/rust-lang/rust/pull/119380)
* [no validar / lint MIR antes de cada pasada](https://github.com/rust-lang/rust/pull/119377)
* [Habilitar perfilador en dist-powerpc-linux](https://github.com/rust-lang/rust/pull/119404)
* [corregir bucle infinito en '<BoundConstness as Display>'](https://github.com/rust-lang/rust/pull/119447)
* [corregir la ayuda de diagnóstico de la función check-cfg Cargo no válida](https://github.com/rust-lang/rust/pull/119425)
* [Se corrige el paréntesis de los subexprs que contienen el límite de la declaración](https://github.com/rust-lang/rust/pull/119105)
* [corrección: corregir los argumentos para el diagnóstico 'desambiguar la función asociada'](https://github.com/rust-lang/rust/pull/118911)
* [Corrección: diagnóstico para la referencia de conversión a la rebanada](https://github.com/rust-lang/rust/pull/119175)
* [introducir 'const trait' (always-const trait bounds)](https://github.com/rust-lang/rust/pull/119099)
* [simplifica 'Parser::ident_or_error'](https://github.com/rust-lang/rust/pull/119359)
* [simplificar los argumentos de bootstrap '--check-cfg'](https://github.com/rust-lang/rust/pull/119441)
* [Compatibilidad con Solaris en el bloqueo de arranque](https://github.com/rust-lang/rust/pull/119413)
* [sincronización de subárbol para 'rustc_codegen_cranelift'](https://github.com/rust-lang/rust/pull/119470)
* [sugerir '=>' → '>=' en comparaciones](https://github.com/rust-lang/rust/pull/117303)
* [Utilice la opción 'llvm-tools' no utilizada](https://github.com/rust-lang/rust/pull/119378)
* [miri: arreglar los ICE de desbordamiento de enteros de 'round_up_to_next_multiple_of'](https://github.com/rust-lang/miri/pull/3246)
* [miri: No determinismo NaN para funciones intrínsecas y libm](https://github.com/rust-lang/miri/pull/3244)
* [miri: soporte para tempfile crate en hosts UNIX](https://github.com/rust-lang/miri/pull/3240)
* [implementar la propagación constante sobre el análisis MIR SSA](https://github.com/rust-lang/rust/pull/116012)
* [solo almacena StableCrateId una vez en DefPathTable](https://github.com/rust-lang/rust/pull/119259)
* [Reducir aún más la codificación del intervalo](https://github.com/rust-lang/rust/pull/119367)
* [openbsd: 'available_parallelism': usa la API correcta](https://github.com/rust-lang/rust/pull/119436)
* [cargo: 'cargo add' - corrección para agregar características desde el repositorio con múltiples paquetes](https://github.com/rust-lang/cargo/pull/13213)
* [cargo: 'cargo fix': heredar siempre el servidor de trabajo](https://github.com/rust-lang/cargo/pull/13225)
* [cargo: arreglar 'fix::fix_in_dependency' para no depender de rustc](https://github.com/rust-lang/cargo/pull/13220)
* [cargo: rustfix: soporte para insertar nuevas líneas](https://github.com/rust-lang/cargo/pull/13226)
* [rustdoc-search: contar ediciones de ruta con límite de edición separado](https://github.com/rust-lang/rust/pull/119331)
* [rustdoc: tratar la cadena de consulta '+' como espacio](https://github.com/rust-lang/rust/pull/119327)
* [clippy: comprueba si hay 'coincidencias' redundantes con 'Listo', 'Pendiente', 'V4', 'V6'](https://github.com/rust-lang/rust-clippy/pull/12029)
* [clippy: '[doc_markdown]': Añade "WebGL2", "WebGPU" a 'doc_valid_idents'](https://github.com/rust-lang/rust-clippy/pull/12018)
* [clippy: añadir comprobaciones de macros externas a 'iter_without_into_iter' y 'into_iter_without_iter'](https://github.com/rust-lang/rust-clippy/pull/12054)
* [clippy: no pelude 'default_numeric_fallback' en las llamadas de macro asignadas locales y de retorno con el tipo indicado](https://github.com/rust-lang/rust-clippy/pull/11957)
* [clippy: extender 'unconditional_recursion' para comprobar si hay implementaciones de ToString](https://github.com/rust-lang/rust-clippy/pull/11980)
* [clippy: añadir 'manual_is_variant_and' lint](https://github.com/rust-lang/rust-clippy/pull/11865)
* [clippy: añadir nueva pelusa 'pub_underscore_fields'](https://github.com/rust-lang/rust-clippy/pull/10283)
* [clippy: sugerir 'str.lines' cuando se divide en saltos de línea codificados](https://github.com/rust-lang/rust-clippy/pull/11987)
* [clippy: hacer que 'mutex_atomic' sea más consciente de los tipos](https://github.com/rust-lang/rust-clippy/pull/12008)
* [clippy: nueva pelusa: 'empty_enum_variants_with_brackets'](https://github.com/rust-lang/rust-clippy/pull/12047)
* [clippy: nueva pelusa: 'thread_local_initializer_can_be_made_const'](https://github.com/rust-lang/rust-clippy/pull/12026)
* [clippy: nueva pelusa: 'eager_transmute'](https://github.com/rust-lang/rust-clippy/pull/11981)
* [clippy: eliminar mitigaciones para argumentos de nodo incorrectos](https://github.com/rust-lang/rust-clippy/pull/12041)
* [rust-analyzer: corrige SyntaxContextID usando autoidentificadores incorrectos](https://github.com/rust-lang/rust-analyzer/pull/16224)
* [rust-analyzer: se corrige el pánico fuera de límites en algunas macros debido a 'self_ref' no manejado](https://github.com/rust-lang/rust-analyzer/pull/16221)

### Clasificación del rendimiento del compilador de Rust

En general, esta semana ha habido muy pocas regresiones y una cantidad moderada de mejoras. Las dos mejoras más importantes se produjeron en la forma en que se codificaban los metadatos, incluido un cambio para almacenar StableCrateId solo una vez en DefPathTable, lo que produjo una mejora promedio del 0,3 % en 79 puntos de referencia diferentes.

Triaje realizado por **@rylev**.
Rango de revisión: [1ab783112.. 67b6975](https://perf.rust-lang.org/?start=1ab783112ab4e4807304dbd249b39771246013ef&end=67b6975051b83ef2bd28f06e8467470d570aceb3&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.7% | [0,3%, 1,5%] | 8 |
| Regresiones ❌ <br /> (secundaria) | 0.8% | [0,2%, 1,3%] | 23 |
| Mejoras ✅ <br /> (primaria) | -0,6% | [-2,6%, -0,2%] | 121 |
| Mejoras ✅ <br /> (secundaria) | -5,2% | [-62,5%, -0,2%] | 53 |
| Todos ❌✅ (primario) | -0,5% | [-2,6%, 1,5%] | 129 |

2 regresiones, 3 mejoras, 1 mixta; 0 de ellos en rollups
46 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/fef95a1961b31e35d91f1ccde0a9783a1ac1d130/triage/2024-01-02.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [rustdoc: buscar tuplas y unidades por tipo con ()](https://github.com/rust-lang/rust/pull/118194)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Nuevos tipos de gama para la Edición 2024](https://github.com/rust-lang/rfcs/pull/3550)
* [Agregar RFC para discutir el Comité Directivo de RustConf 2024](https://github.com/rust-lang/rfcs/pull/3549)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2024-01-03 - 2024-01-31 🦀

### Virtual

* 03/01/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 06/01/2024 | Virtual (Kampala, UG) | [Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763647997?aff=ebdssbdestsearch)
* 09/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298062049/)
* 11/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 11/01/2024 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679708/)
* 16/01/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/297128172/)
* 17/01/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 18/01/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin)
* 2024-01-21 | Virtual | [Especialista en Rust](https://meet-os.com/group/1)
    * [**Desarrollo web con Rocket - En Inglés**](https://meet-os.com/event/1)
* 23/01/2024 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 24/01/2024 | Virtual (Berlín, DE) | [Comunidad WeAreDevelopers](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 25/01/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 28/01/2024 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 30/01/2024 | Virtual (Búfalo, NY, EE. UU.) | [Grupo de usuarios de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 30/01/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)

### Europa

* 10/01/2024 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/)
    * [**Desarrollo de juegos en Rust**](https://www.meetup.com/rustcologne/events/298303772/)
* 11/01/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 11/01/2024 | Wrocław, PL | [Rust de Breslavia](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 13/01/2024 | Tampere, FI | [Grupo Rust-lang de Finlandia](https://www.meetup.com/finland-rust-meetup/)
    * [**Encuentro de enero**](https://www.meetup.com/finland-rust-meetup/events/297811750/)
* 16/01/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/) 
    * [**Asíncrono en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297376712/)
* 17/01/2024 | Praga / Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup renovado**](https://www.meetup.com/rust-prague/events/298005196/)
* 17/01/2024 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/rust-zurich/)
    * [**Cómo hacer comunidad - Encuentro de enero**](https://www.meetup.com/rust-zurich/events/298066842/)
* 23/01/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hackear y aprender**](https://www.meetup.com/rust-aarhus/events/297463730/)

### América del Norte

* 06/01/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Beacon Hill**](https://www.meetup.com/bostonrust/events/297633937/)
* 08/01/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 09/01/2024 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Noche de Codificación/Hackeo/Aprendizaje Oxidado**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 09/01/2024 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 09/01/2024 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup: A Deep Dive into Tower por Adrien Guillo**](https://www.meetup.com/rust-nyc/events/298169818/)
* 14/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 16/01/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 17/01/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 18/01/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 24/01/2024 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 30/01/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers)
  
# Frase de la semana

> Algunas personas no creen en la vida después de la muerte... Rust no cree en la magia después de la compilación.

– [Stephan Sokolow sobre los usuarios de Rust](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1504)

¡Gracias a [Todd Fleming](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1505) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18y3z3m/this_week_in_rust_528)</small>
