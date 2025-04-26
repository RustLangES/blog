---
title: "Esta semana en Rust #57"
number_of_week: 57
description: El crate de esta semana es Maycoon, un marco de interfaz de usuario experimental basado en vello/wGPU.
date: 2025-04-23
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

### Fundación
* [Actualización del Director del Proyecto — Abril 2025](https://rustfoundation.org/media/project-director-update-april-2025/)

### Actualizaciones de proyectos/herramientas
* [Gitoxide abril de 2025](https://github.com/GitoxideLabs/gitoxide/discussions/1966)
* [Askama y Rinja se fusionan](https://blog.guillaume-gomez.fr/articles/2025-03-19+Askama+and+Rinja+merge)
* [registro de cambios de rust-analyzer #282](https://rust-analyzer.github.io/thisweek/2025/04/21/changelog-282.html)

### Observaciones/Pensamientos
* [Dos formas de interpretar la visibilidad en Rust](https://kobzol.github.io/rust/2025/04/23/two-ways-of-interpreting-visibility-in-rust.html)
* [¿El uso de Rust realmente hace que tu software sea más seguro? - Blog - Tweede golf](https://tweedegolf.nl/en/blog/152/does-using-rust-really-make-your-software-safer)
* [Cuerpo::p oll_progreso](https://seanmonstar.com/blog/body-poll-progress/)
* [ratatui: ¿ya estamos incrustados?](https://jslazak.com/are-we-embedded-yet-0/)
* [Zig -> asignadores -> Ergonomía Rust](https://www.capturedlambda.dev/blog/zig-allocators-rust_ergo/)
* [Eventos de marcha: ¿Qué tiene que ver iCalendar con la marcha de rayos?](https://pwy.io/posts/marching-events/)
* [audio] [Nushell con WindSoilder](https://rustacean-station.org/episode/windsoilder/)
* [audio] [Microsoft con Víctor Ciura](https://corrode.dev/podcast/s04e01-microsoft/)

### Tutoriales de Rust
* [Implementación de pánicos de Rust en la biblioteca estándar](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_2.html)
* [Cómo funciona un GraphQL DataLoader](https://swatinem.de/blog/graphql-dataloader-part1/)
* [Implementando un GraphQL DataLoader de la manera difícil](https://swatinem.de/blog/graphql-dataloader-part2/)

## Crate de la semana

El crate de esta semana es [Maycoon](https://maycoon-ui.github.io/), un marco de interfaz de usuario experimental basado en vello/wGPU.

¡Gracias a [DraftedDev](https://users.rust-lang.org/t/crate-of-the-week/2704/1431) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de características y desea que su RFC aparezca en esta lista, agregue un
'call-for-testing' a su RFC junto con un comentario que proporcione instrucciones de prueba y/o
orientación sobre qué aspectos de la función deben probarse.

* * Esta semana no se emitieron convocatorias para pruebas por parte de [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háganoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

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

Se [fusionaron 465 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-15..2025-04-22

#### Compilador

* [evitar el desbordamiento al generar información de depuración para expandir tipos recursivos](https://github.com/rust-lang/rust/pull/138599)
* [deref patterns: implementar patrones deref implícitos](https://github.com/rust-lang/rust/pull/138528)
* [Se corrigió la sugerencia incorrecta de "mover palabra clave" para el bloque de generación asíncrona](https://github.com/rust-lang/rust/pull/139871)
* [Mejorar los errores de análisis para la vida útil de los parásitos en la posición del tipo](https://github.com/rust-lang/rust/pull/139854)
* [hacer '#[desnudo]' un atributo inseguro](https://github.com/rust-lang/rust/pull/139753)
* [reescribir el analizador de cadenas de formato 'on_unimplemented'](https://github.com/rust-lang/rust/pull/139091)

#### Miri

* [miri: implementar la eliminación de archivos en Windows](https://github.com/rust-lang/miri/pull/4260)
* [Miri: use intrínsecos declarados por std en lugar de copiar la declaración](https://github.com/rust-lang/miri/pull/4274)

#### Biblioteca

* [añadir 'next_index' a Enumerar](https://github.com/rust-lang/rust/pull/139533)
* [agregar reintentos para eliminar y crear dir todo](https://github.com/rust-lang/rust/pull/139870)
* ['sync::mpsc': evitar doble libre en 'Drop'](https://github.com/rust-lang/rust/pull/139553)
* [implementar 'Default' para punteros sin procesar](https://github.com/rust-lang/rust/pull/139535)
* [Implementa 'pin! ()' usando 'super let'](https://github.com/rust-lang/rust/pull/139114)
* [estabilizar '-Zdwarf-version' como '-Cdwarf-version'](https://github.com/rust-lang/rust/pull/136926)
* [estabilizar 'cfg_boolean_literals'](https://github.com/rust-lang/rust/pull/138632)
* [estabilizar 'naked_functions'](https://github.com/rust-lang/rust/pull/134213)
* [Simd intrínsecos con máscara: acepta máscaras de enteros sin signo y corrige algunos de los errores](https://github.com/rust-lang/rust/pull/137953)
* [añadir 'vec_extract', 'vec_insert', 'vec_promote' y 'vec_insert_and_zero'](https://github.com/rust-lang/stdarch/pull/1772)

#### Carga

* [use 'zlib-rs' para la compresión gzip en el código Rust](https://github.com/rust-lang/cargo/pull/15417)

#### Rustdoc

* [Corregir error cuando un enlace intra doc está intentando resolver un elemento asociado vacío](https://github.com/rust-lang/rust/pull/140052)
* [rustdoc-json: información de la característica de destino de salida](https://github.com/rust-lang/rust/pull/139393)
* [Soporte de alias de rasgos reexportados de cajas cruzadas en línea](https://github.com/rust-lang/rust/pull/139943)

#### Clippy

* [clippy: 'bool_to_int_with_if': maneja correctamente las macros](https://github.com/rust-lang/rust-clippy/pull/14629)
* [clippy: 'empty_enum_variants_with_brackets': No pelar las enumeraciones alcanzables y las variantes de 'enumeración' utilizadas como funciones en la misma caja](https://github.com/rust-lang/rust-clippy/pull/12971)
* [clippy: 'iter_kv_map': reconocer referencias en mapas también](https://github.com/rust-lang/rust-clippy/pull/14596)
* [clippy: 'manual_ok_err': no peluques subpatrones](https://github.com/rust-lang/rust-clippy/pull/14661)
* [clippy: 'match_single_binding': permitir macros en escrutinio y patrones](https://github.com/rust-lang/rust-clippy/pull/14635)
* [clippy: 'missing_asserts_for_indexing': considera 'assert_eq! ()' también](https://github.com/rust-lang/rust-clippy/pull/14258)
* [clippy: 'ptr_cast_constness': mostrar fragmento del contexto correcto](https://github.com/rust-lang/rust-clippy/pull/14622)
* [clippy: compilar un tipo utilizable completo a partir de un prefijo relativo al tipo](https://github.com/rust-lang/rust-clippy/pull/14586)
* [clippy: verifique también los usos de por vida en los cierres](https://github.com/rust-lang/rust-clippy/pull/14608)
* [clippy: no se repita para siempre en 'significant_drop_tightening'](https://github.com/rust-lang/rust-clippy/pull/14641)
* [clippy: corrige 'question_mark' sugiriendo cuando el tipo está detrás de Deref incluir paréntesis](https://github.com/rust-lang/rust-clippy/pull/14655)
* [clippy: corrección: 'unnecessary_lazy_evaluations' sugiere erróneamente el cierre asíncrono](https://github.com/rust-lang/rust-clippy/pull/14644)
* [clippy: hacer que la bandera 'borrow_as_ptr' también se convierta en un signo implícito](https://github.com/rust-lang/rust-clippy/pull/14408)
* [clippy: nueva pelusa: 'redundant_test_prefix'](https://github.com/rust-lang/rust-clippy/pull/13710)
* [clippy: nueva pelusa: 'swap_with_temporary'](https://github.com/rust-lang/rust-clippy/pull/14046)
* [clippy: reemplaza la internación de literales de cadena con símbolos preinternados](https://github.com/rust-lang/rust-clippy/pull/14650)

#### Analizador de Rust

* [Rust-analyzer: agregue punto y coma para usar](https://github.com/rust-lang/rust-analyzer/pull/19604)
* [rust-analyzer: permitir el entrenamiento de PGO en una caja personalizada y habilitarlo Windows en CI](https://github.com/rust-lang/rust-analyzer/pull/19585)
* [rust-analyzer: permitir el uso de 'null' para desestablecer una variable de entorno](https://github.com/rust-lang/rust-analyzer/pull/19629)
* [rust-analyzer: build aarch64 se basa en CI con PGO](https://github.com/rust-lang/rust-analyzer/pull/19597)
* [rust-analyzer: no ignorar los valores de configuración que no se pueden analizar](https://github.com/rust-lang/rust-analyzer/pull/19628)
* [rust-analyzer: añadir la opción 'pub(crate) mod' para archivos desvinculados](https://github.com/rust-lang/rust-analyzer/pull/19590)
* [rust-analyzer: permitir desconfigurar variables de entorno en la configuración de 'server.extraEnv'](https://github.com/rust-lang/rust-analyzer/pull/19634)
* [rust-analyzer: ayuda mejorada de la firma para mostrar parámetros genéricos para invocables y valores predeterminados para args genéricos](https://github.com/rust-lang/rust-analyzer/pull/19596)
* [rust-analyzer: parse 'super let'](https://github.com/rust-lang/rust-analyzer/pull/19653)
* [rust-analyzer: analizar consts genéricas](https://github.com/rust-lang/rust-analyzer/pull/19643)
* [rust-analyzer: La función 'Extraer en' incluye una variable en línea en la macro fmt](https://github.com/rust-lang/rust-analyzer/pull/19588)
* [rust-analyzer: arreglar la configuración 'completion_snippets_custom' siempre fallando](https://github.com/rust-lang/rust-analyzer/pull/19636)
* [rust-analyzer: se corrige un error con la reducción de predicados de elementos asociados](https://github.com/rust-lang/rust-analyzer/pull/19612)
* [rust-analyzer: soluciona un pánico cuando un método de rasgo en un impl declara un parámetro de vida que no está en la declaración de rasgo](https://github.com/rust-lang/rust-analyzer/pull/19613)
* [Rust-analyzer: resaltado para la expansión de la cola en bloques etiquetados](https://github.com/rust-lang/rust-analyzer/pull/19589)
* [Rust-analyzer: ordenar rasgos notables en el desplazamiento](https://github.com/rust-lang/rust-analyzer/pull/19619)
* [rust-analyzer: soporta la inestable 'UnsafePinned struct' en el cálculo de diseño de tipos](https://github.com/rust-lang/rust-analyzer/pull/19650)
* [rust-analyzer: use PGO en compilaciones de Linux x64](https://github.com/rust-lang/rust-analyzer/pull/19595)
* [rust-analyzer: use PGO en compilaciones de macOS x64 y arm64](https://github.com/rust-lang/rust-analyzer/pull/19611)

### Clasificación del rendimiento del compilador de Rust

Semana mayoritariamente positiva. La mayoría de las mejoras provienen de una reversión de una regresión de hace unas semanas, pero también obtenemos buenas ganancias al reutilizar la ruta rápida de Sized, proveniente del trabajo de implementación de la jerarquía de Sized.

Triaje realizado por **@panstromek**.
Rango de revisión: [15f58c46.. 8F2819B0](https://perf.rust-lang.org/?start=15f58c46da79399961a09db0c650a2f90f442e6b&end=8f2819b0e3428d0aee05fa60e91e0211c2aea053&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 1.3% | [0.4%, 2.1%] | 7 |
| Regresiones ❌ <br /> (secundaria) | -     | -               | 0 |
| Mejoras ✅ <br /> (primario) | -1.0% | [-12,9%, -0,1%] | 144 |
| Mejoras ✅ <br /> (secundaria) | -2,2% | [-12.3%, -0.2%] | 111 |
| Todos ❌✅ (primarios) | -0.9% | [-12.9%, 2.1%] | 151 |

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/02138a9d3679be358403ee8906141666870e5346/triage/2024-04-22.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar '<[T; N]>::as_mut_slice' como 'const'](https://github.com/rust-lang/rust/pull/140066)
* [Problema de seguimiento para 'hint::select_unpredictable'](https://github.com/rust-lang/rust/issues/133962)
* [fix(test): Exponer '--no-capture' a favor de '--nocapture'](https://github.com/rust-lang/rust/pull/139224)
* [estabilizar ptr::swap_nonoverlapping en const](https://github.com/rust-lang/rust/pull/137280)
* [Estabilizar las características del objetivo avx512](https://github.com/rust-lang/rust/pull/138940)
* [No permitir format_args aplanadas en const.](https://github.com/rust-lang/rust/pull/139624)
* [lexer: Tratar más flotantes con exponente vacío como tokens válidos](https://github.com/rust-lang/rust/pull/131656)
* [Estabilizar proc_macro::Span::{inicio,fin,línea,columna}.](https://github.com/rust-lang/rust/pull/139865)
* [Comprobar los tipos de valores predeterminados de los parámetros const](https://github.com/rust-lang/rust/pull/139646)
* [Estabilizar banderas para la compilación cruzada doctest](https://github.com/rust-lang/rust/pull/137096)

#### Otras áreas
##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [corrección: por defecto a todos los objetivos cuando se usa '--edition' y '--edition-idioms' en la corrección de carga](https://github.com/rust-lang/cargo/pull/15192)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Aclarar la interacción de asm-goto con IBT](https://github.com/rust-lang/reference/pull/1790)

*No hay artículos ingresados al Período Final de Comentarios esta semana para
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Deshabilitar la optimización de diseño de nicho en discriminantes de enumeración](https://github.com/rust-lang/rfcs/pull/3803)
* [RFC: Asumir límites para funciones genéricas](https://github.com/rust-lang/rfcs/pull/3802)
* [RFC: macros de entrada](https://github.com/rust-lang/rfcs/pull/3799)

## Próximos eventos

Eventos oxidados entre 2025-04-23 - 2025-05-21 🦀

### Virtual
* 23/04/2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Beyond embedded - Desarrollo de sistemas operativos en Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307036053)
* 24/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820299)
* 24/04/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parte 2: ¡Las computadoras cuánticas no pueden proteger esto contra el Rust!" **](https://www.meetup.com/charlottesville-rust-meetup/events/306679733)
* 03/05/2025 | Virtual (Kampala, UG) | [Reunión de Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 05/05/2025 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://www.meetup.com/rust-tlv/events/307178592/)
* 07/05/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031663)
* 07/05/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #10**](https://www.meetup.com/bevy-game-development/events/307354690)
* 08/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 13/05/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/305020415)
* 15/05/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [** 🦀 Celebrando los 10 años de Rust 1.0 🦀 **](https://www.meetup.com/rust-berlin/events/307293317) | [**Transmisión en vivo**](https://berline.rs/)
* 2025-05-20 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/305170826)
* 21/05/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)

### Europa
* 23/04/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Fusionando Python con Rust usando enlaces C sin procesar**](https://www.meetup.com/london-rust-project-group/events/306644439)
* 24/04/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charla y fiesta de cumpleaños en MFT Energy**](https://www.meetup.com/rust-aarhus/events/305809344)
* 24/04/2025 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Encuentro de Rust #57**](https://www.meetup.com/copenhagen-rust-community/events/307340144)
* 24/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/306911347)
* 24/04/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Code Night**](https://www.meetup.com/rust-manchester/events/306899063)
* 25/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/306911357)
* 2025-04-26 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #11**](https://www.meetup.com/stockholm-rust/events/307164617)
* 29/04/2025 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks abril 2025 Community Showcase**](https://www.meetup.com/rust-london-user-group/events/307212039)
* 29/04/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #76**](https://www.meetup.com/rust-paris/events/306952202)
* 30/04/2025 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Operador de Kubernetes en Rust**](https://www.meetup.com/rust-rhein-main/events/306772838)
* 01/05/2025 | Nürnberg, DE | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Hackers Hike 0x0**](https://www.meetup.com/rust-noris/events/305522254)
* 04/05/2025 | Estambul, TR | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**Conexión de Rust**](https://kommunity.com/turkiyerust/events/rust-connect-91f7b3a6)
* 06/05/2025 - 07/05/2025 | París, FR | [WebAssembly y Rust Meetup](https://www.meetup.com/wasm-rust-meetup/)
    * [**GOSIM AI París 2025**](https://www.meetup.com/wasm-rust-meetup/events/306530699/)
* 06/05/2025 | París, FR | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Paris 2025 (Descuento disponible)**](https://www.meetup.com/wasm-rust-meetup/events/306530699)
* 07/05/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 07/05/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 08/05/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 08/05/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adoptando Rust (Alojado por Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 14/05/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 16/05/2025 | Utrecht, NL | [Grupo de Meetup de Rust NL](https://www.meetup.com/rust-nederland/)
    * [**Hackathon de RustWeek**](https://www.meetup.com/rust-nederland/events/307107584/)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)

### América del Norte
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/307089940)
* 23/04/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcgbfc)
* 23/04/2025 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Mostrar y contar a la comunidad en Fuel Coworking**](https://www.meetup.com/spokane-rust/events/307228157)
* 24/04/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**3ª 3ª VEZ ¡DIOS MÍO SÍ!**](https://www.meetup.com/rust-atl/events/307152133)
* 25/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Ball Square Rust, 25 de abril**](https://www.meetup.com/bostonrust/events/306844343)
* 01/05/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Reflexiones del Proyecto Capstone SIUE sobre el Rust**](https://www.meetup.com/stl-rust/events/304026152)
* 03/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Boston Common Rust, 3 de mayo**](https://www.meetup.com/bostonrust/events/306845368)
* 08/05/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Tiempo de compilación vs Tiempo de ejecución. Introducción a uv**](https://www.meetup.com/rust-mx/events/307015601)
* 08/05/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: Un motor de consulta analítica rápido, extensible y modular en Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 11/05/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust de Porter Square, 11 de mayo**](https://www.meetup.com/bostonrust/events/306845728)
* 15/05/2025 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Usando Rust para Web Series 2 : Por qué tú, sí tú. ¡Debería usar Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 15/05/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Mayo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-20 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)

### América del Sur
* 28/05/2025 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Primera meetup de Rust en 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1jttzz4/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> tampoco pienso en el Rust. Ese es el trabajo de un compilador

– [Steve Klabnik en Bluesky](https://bsky.app/profile/steveklabnik.com/post/3lmtavr5ni22l)

¡Gracias a [Matt Wismer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1677) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1k6ijof/this_week_in_rust_596/)</small>
