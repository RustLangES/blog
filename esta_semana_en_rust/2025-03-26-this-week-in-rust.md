---
title: "Esta semana en Rust #53"
number_of_week: 53
description: El crate de esta semana es jiff, una biblioteca de fecha y hora para Rust.
date: 2025-03-26
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos un PR](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Fundación
* [Ferrous Systems dona la especificación del lenguaje del ferroceno al proyecto Rust](https://rustfoundation.org/media/ferrous-systems-donates-ferrocene-language-specification-to-rust-project/)

### Actualizaciones de proyectos/herramientas
* [Fastrace: Un enfoque moderno para el rastreo distribuido en Rust](https://fast.github.io/blog/fastrace-a-modern-approach-to-distributed-tracing-in-rust/)
* [registro de cambios de rust-analyzer #278](https://rust-analyzer.github.io/thisweek/2025/03/24/changelog-278.html)
* [Presentamos Cot v0.2: Una nueva versión del framework web Rust para desarrolladores perezosos](https://mackow.ski/blog/introducing-cot-v02/)

### Observaciones/Pensamientos
* [¿Lo inseguro socava las garantías de Rust?](https://steveklabnik.com/writing/does-unsafe-undermine-rusts-guarantees/)
* [Notas sobre coreutils en Rust](https://alexgaynor.net/2025/mar/22/coreutils-in-rust/)
* [Rust en 2025: Interoperabilidad del lenguaje y el compilador extensible](https://smallcultfollowing.com/babysteps/blog/2025/03/18/lang-interop-extensibility/)
* [Rasgos asíncronos de Dyn, parte 10: Caja caja caja](https://smallcultfollowing.com/babysteps/blog/2025/03/24/box-box-box/)
* [¿Tienes una idea para 'dyn'?](https://smallcultfollowing.com/babysteps/blog/2025/03/25/dyn-you-have-idea-for-dyn/)
* [Inicialización retrasada segura para extensión de por vida](https://paper.wf/binarycat/safe-delayed-initialization-for-lifetime-extension)
* [Solo escribe una prueba para ello](https://kobzol.github.io/rust/2025/03/25/just-write-a-test-for-it.html)
* [audio] [ExpressVPN con Pete Membrey](https://rustacean-station.org/episode/pete-membrey/)
* [Construyendo un sitio web rápido con la pila MASH](https://emschwartz.me/building-a-fast-website-with-the-mash-stack-in-rust/)

### Tutoriales de Rust
* [Un truco tonto de proc-macro: Cómo emitir código parcial + errores](https://schneems.com/2025/03/26/a-daft-procmacro-trick-how-to-emit-partialcode-errors/)
* [Proveedor de dependencias de C/C++ en Rust](https://blog.veeso.dev/blog/en/vendoring-c-cpp-dependencies-in-rust/)
* [La actualización más rápida de Vec en mi PC](https://jtjlehi.github.io/2024/03/13/fastest-vec-update-on-my-computer.html)
* [Un trabajo por lotes 10 veces más rápido mediante el procesamiento por lotes de inserciones/actualizaciones de PostgreSQL con Rust y SQLx](https://kerkour.com/postgresql-batching)
* [Cerrando la brecha de eficiencia entre FromStr y string](https://lucumr.pocoo.org/2025/3/23/from-string/)
* [video] [Build with Naz : rasgos, subtipado, polimorfismo en Rust](https://www.youtube.com/watch?v=K5SY-lc8nTE)
* [video] [Rust y programación embebida con Leon Vak](https://rust.code-maven.com/rust-and-embedded-programming-with-leon-vak)

## Crate de la semana

El crate de esta semana es [jiff](https://crates.io/crates/jiff), una biblioteca de fecha y hora para Rust.

¡Gracias a [Filip T](https://users.rust-lang.org/t/crate-of-the-week/2704/1420) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características
y desea que su RFC aparezca en esta lista, agregue una etiqueta de 'llamada para pruebas' a su RFC junto con
con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 496 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-18..2025-03-25

#### Compilador

* [inferior a un 'memset(undef)' cuando 'Rvalue::Repeat' repite uninit](https://github.com/rust-lang/rust/pull/138634)

#### Biblioteca

* ['MaybeUninit' métodos de corte inherentes parte 2](https://github.com/rust-lang/rust/pull/135394)
* [core/slice: marcar algunas variantes 'split_off' de manera inestable const](https://github.com/rust-lang/rust/pull/138540)
* [core: optimize 'RepeatN'](https://github.com/rust-lang/rust/pull/138833)
* [implementar métodos predeterminados para 'io::Empty' y 'io::Sink'](https://github.com/rust-lang/rust/pull/137051)
* [optimizar 'io::Write::write_fmt' para cadenas constantes](https://github.com/rust-lang/rust/pull/138650)
* [simplificar 'PartialOrd' en tuplas que contienen primitivas](https://github.com/rust-lang/rust/pull/138135)
* [reducir 'FormattingOptions' a 64 bits](https://github.com/rust-lang/rust/pull/136974)

#### Carga

* [Agregar completador personalizado para <TAB>carga '' para completar los alias definidos en config.toml](https://github.com/rust-lang/cargo/pull/15319)

#### Rustdoc

* [ser más estricto acerca de los "Métodos de Deref"](https://github.com/rust-lang/rust/pull/138574)
* [predicados de puerta inestable 'doc(cfg())'](https://github.com/rust-lang/rust/pull/138293)
* [use su propia lógica para imprimir '#[repr(..)]' en la salida JSON](https://github.com/rust-lang/rust/pull/138018)

#### Clippy

* ['wildcard_imports': pelusa en 'uso de pub' si se te pide](https://github.com/rust-lang/rust-clippy/pull/14182)
* [agregue la verificación de MSRV para 'question_mark'](https://github.com/rust-lang/rust-clippy/pull/14436)
* [añadir pelusa 'ignore_without_reason'](https://github.com/rust-lang/rust-clippy/pull/13931)
* [emitir 'collapsible_match' en el nodo derecho](https://github.com/rust-lang/rust-clippy/pull/14311)
* [expanda 'neg_multiply' a los números flotantes de lint también](https://github.com/rust-lang/rust-clippy/pull/14447)
* [Sugerencia de corrección para las tareas que tienen paréntesis encerrados debajo de 'needless_late_init'](https://github.com/rust-lang/rust-clippy/pull/14169)
* [corrección: 'borrow_deref_ref' sugiere erróneamente cuando se coacciona para silenciar](https://github.com/rust-lang/rust-clippy/pull/14403)
* [corrección: 'filter_map_bool_then' sugieren erróneamente cuando el cierre no se puede descomponer directamente](https://github.com/rust-lang/rust-clippy/pull/14370)
* [corrección: 'manual_find' sugiere erróneamente cuándo se regresa temprano](https://github.com/rust-lang/rust-clippy/pull/14405)
* [corrección: 'missing_const_for_fn' falso positivo en rasgos const inestables](https://github.com/rust-lang/rust-clippy/pull/14294)
* [corrección: 'nonminimal_bool' mostró erróneamente la definición de macro](https://github.com/rust-lang/rust-clippy/pull/14424)
* [corrección: falso positivo 'option_if_let_else' cuando el valor se movió parcialmente](https://github.com/rust-lang/rust-clippy/pull/14209)
* [corrección: falso positivo de 'redundant_clone' en el lanzamiento de 'enum'](https://github.com/rust-lang/rust-clippy/pull/14395)
* [mejorar la pelusa 'string_to_string' en caso de que esté en una llamada de mapa](https://github.com/rust-lang/rust-clippy/pull/14396)
* [lint más casos en 'collapsible_if'](https://github.com/rust-lang/rust-clippy/pull/14231)
* [flexibilizar la aplicabilidad de 'never_loop'](https://github.com/rust-lang/rust-clippy/pull/14203)
* [mover 'uninlined_format_args' de nuevo a 'estilo'](https://github.com/rust-lang/rust-clippy/pull/14160)
* [restablecer las líneas 'single_match'/'single_match_else' con comentarios](https://github.com/rust-lang/rust-clippy/pull/14420)
* [sugerir 'is_some_and' en lugar de 'map_or' en 'case_sensitive_file_extension_comparions'](https://github.com/rust-lang/rust-clippy/pull/14358)
* [unificar el código 'manual_unwrap_or' y 'manual_unwrap_or_default'](https://github.com/rust-lang/rust-clippy/pull/14332)
* [use 'código' para referencias a otras lints en los documentos 'as_conversions'](https://github.com/rust-lang/rust-clippy/pull/14283)

#### Analizador de Rust

* [arreglar ide-assist 'let else' a 'if let else'](https://github.com/rust-lang/rust-analyzer/pull/19433)
* [Agregar diagnóstico para el error de ambigüedad faltante para el rasgo IMPL](https://github.com/rust-lang/rust-analyzer/pull/19347)
* [agregar finalización de sufijo para el bloque const](https://github.com/rust-lang/rust-analyzer/pull/19397)
* [Agregar soporte de edición de texto para sugerencias de tipo de retorno en cierres de cuerpo que no son de bloque](https://github.com/rust-lang/rust-analyzer/pull/19348)
* [analysis-stats: emitir líneas de código y recuentos de árboles de elementos para el espacio de trabajo; dependencias](https://github.com/rust-lang/rust-analyzer/pull/19359)
* [analizar campos de registro 'inseguros'](https://github.com/rust-lang/rust-analyzer/pull/19388)
* [Se corrige el resaltado de sintaxis faltante para '&raw const' / '&raw mut' en todos los archivos](https://github.com/rust-lang/rust-analyzer/pull/19400)
* [arreglar el cierre de retorno de inlayhints usando rangos de macros](https://github.com/rust-lang/rust-analyzer/pull/19435)
* [manejar múltiples '#[repr(..)]' attrs correctamente](https://github.com/rust-lang/rust-analyzer/pull/19416)
* [calcular correctamente los diseños de las tuplas ptrs cuyos últimos campos son DST](https://github.com/rust-lang/rust-analyzer/pull/19413)
* [diseño de renderizado y otra información adicional al pasar el cursor sobre 'Self'](https://github.com/rust-lang/rust-analyzer/pull/19419)
* [acelerar la resolución de una asistencia de "Generar método de delegado"](https://github.com/rust-lang/rust-analyzer/pull/19362)

### Clasificación del rendimiento del compilador de Rust

Una semana casi sin ruido, lo cual es emocionante, con una serie de
mejoras aterrizando para una velocidad media acumulada de hasta un 0,5%, posiblemente mayor si
Ignoramos las regresiones que probablemente se corrijan o reviertan de
[#138674](https://github.com/rust-lang/rust/pull/138674).

Triaje realizado por **@simulacrum**.
Rango de revisión: [493c38ba.. 4510E86A](https://perf.rust-lang.org/?start=493c38ba371929579fe136df26eccd9516347c7a&end=4510e86a41388733675465a8647d4235f3bf2023&absolute=false&stat=instructions%3Au)

3 regresiones, 4 mejoras, 2 mixtas; 3 de ellos en rollups
35 comparaciones de artefactos realizadas en total

Lea el [informe completo](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-03-24.md) para obtener más detalles.

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [¡Deja de usar el inestable 'concat_idents!'](https://github.com/rust-lang/rust/pull/137653)
* [Estabilizar '#! [característica(precise_capturing_in_traits)]'](https://github.com/rust-lang/rust/pull/138128)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Problema de seguimiento para slice::array_chunks](https://github.com/rust-lang/rust/issues/74985)
* [Estabilizar const_cell](https://github.com/rust-lang/rust/pull/137928)
* [Eliminar los acentos graves de 'ShouldPanic::YesWithMessage' 'TrFailedMsg'](https://github.com/rust-lang/rust/pull/136160)
* [Use 'binOp::cmp' para 'iNN::signum'](https://github.com/rust-lang/rust/pull/137835)
* [Prefiera impls de tamaño incorporado (y solo impls de tamaño) para tipos rígidos siempre](https://github.com/rust-lang/rust/pull/138176)

#### Otras áreas
* *No hay artículos ingresados al Período Final de Comentarios esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2025-03-26 - 2025-04-23 🦀

### Virtual
* 27/03/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820297)
* 01/04/2025 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304228)
* 02/04/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031661)
* 03/04/2025 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820282/)
* 05/04/2025 | Virtual | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Comunicarse con canales en Rust**](https://www.eventbrite.com/e/communicate-with-channels-in-rust-tickets-1278267335009)
* 05/04/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 08/04/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/303522530)
* 10/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820298)
* 2025-04-15 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170698)
* 16/04/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/306231500)
* 17/04/2025 | Virtual y presencial (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-22 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361432)

### Asia
* 2025-03-28 | Kowloon Tong, HK | [Rust de Asia](https://www.rustasiaconf.com/)
    * [**Rust Asia 2025**](https://www.rustasiaconf.com/)
* 05/04/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de abril de 2025**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)
* 2025-04-22 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust abril de 2025 en Braavos en Tel Aviv en colaboración con StarkWare**](https://www.meetup.com/rust-tlv/events/306530984)

### Europa
* 2025-03-26 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**"¡Más que increíblemente rápido!" Performance Optimierungen in Rust**](https://www.meetup.com/rust-rhein-main/events/306659893)
* 2025-03-26 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester habla de marcha**](https://www.meetup.com/rust-manchester/events/305897029)
* 2025-03-26 | Varsovia, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon**](https://www.rustikon.dev/)
* 27/03/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #12: Probando en Rust**](https://rust-augsburg.github.io/meetup/Meetup_12.html)
* 27/03/2025 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Encuentro de Rust #56**](https://www.meetup.com/copenhagen-rust-community/events/306659637)
* 29/03/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #10**](https://www.meetup.com/stockholm-rust/events/306770525)
* 02/04/2025 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/306553077)
* 02/04/2025 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 1 - híbrido**](https://www.meetup.com/rust-munich/events/306097261)
* 02/04/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541535)
* 02/04/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/stockholm-rust/events/306627608)
* 03/04/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809680)
* 08/04/2025 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**3. Encuentro de Rust Moravia (Real Embedded Rust)**](https://www.meetup.com/rust-moravia/events/306377283)
* 09/04/2025 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2025**](https://lu.ma/dlvfol30)
* 09/04/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045446)
* 10/04/2025 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/306682264)
* 2025-04-15 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741632)
* 2025-04-15 | Londres, Reino Unido | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**WIR x WCC: Encontrando tu voz en la tecnología**](https://www.meetup.com/women-in-rust/events/306774769)
* 2025-04-19 | Estambul, TR | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**Rust Konf Türkiye**](https://kommunity.com/turkiye-rust-community/events/rust-konf-turkiye-91f7b3a6)
* 23/04/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Fusionando Python con Rust usando enlaces C sin procesar**](https://www.meetup.com/london-rust-project-group/events/306644439)

### América del Norte
* 2025-03-26 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcfbjc)
* 2025-03-26 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Cena de Allston Rust, 26 de marzo!**](https://www.meetup.com/bostonrust/events/306847057)
* 2025-03-26 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: No puedo creer que sea legal Rust con Michael Gattozzi (NUEVA UBICACIÓN)**](https://www.meetup.com/rust-nyc/events/306777565)
* 27/03/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/306470345)
* 29/03/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de North End Rust, 29 de marzo**](https://www.meetup.com/bostonrust/events/306844321)
* 31/03/2025 | Boulder, CO, EE. UU. | [Depósito de estado sólido](https://www.meetup.com/solidstatedepot/events/)
    * [**Boulder Rust: Bryan presenta Rusted Hardware**](https://www.meetup.com/solidstatedepot/events/306573447)
* 03/04/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306728493)
* 03/04/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Abril Mensual Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 03/04/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - Internacionalización con restricciones de recursos (i18N)**](https://www.meetup.com/stl-rust/events/304890140)
* 06/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Kendall Rust Lunch, 6 de abril**](https://www.meetup.com/bostonrust/events/306844327)
* 10/04/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**TetaNES: Una vacuna contra la roya: sin aguja, solo el verificador de préstamos**](https://www.meetup.com/pdxrust/events/306714209)
* 14/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Coolidge Corner Brookline, 14 de abril**](https://www.meetup.com/bostonrust/events/306844334)
* 17/04/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 1 : Por qué HTMX es malo**](https://www.meetup.com/music-city-rust-developers/events/304333092)
* 17/04/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Reunión de abril de 2025 SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcgbfc)

### Oceanía
* 14/04/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306841248)
* 2025-04-22 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de abril**](https://www.meetup.com/rust-canberra/events/306425557)

### América del Sur
* 27/03/2025 | Medellín, CO | [Rust Medellín](https://www.meetup.com/rust-medellin/events/)
    * [**Multithreading y Terminal User Interfaces con Rust**](https://www.meetup.com/rust-medellin/events/306836484)
* 03/04/2025 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Abril - Lambdas y más!**](https://www.meetup.com/rust-argentina/events/306671000)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ivrkhs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> ¿Funcionó? Es Rust, ¡así que funcionó en el primer intento!

– [James Calligeros sobre el informe de progreso de Asahi](https://asahilinux.org/2025/03/progress-report-6-14/)

¡Gracias a [yerke](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1663) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1jkrugx/this_week_in_rust_592/)</small>
