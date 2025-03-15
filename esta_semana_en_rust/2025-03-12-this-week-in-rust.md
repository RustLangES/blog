---
title: "Esta semana en Rust #51"
number_of_week: 51
description: El crate de esta semana es eval-macro, una caja que permite evaluar macros en tiempo de compilación, dando una sensación similar al comptime de Zig.
date: 2025-03-12
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

### Oficial
* [Anunciando rustup 1.28.1](https://blog.rust-lang.org/2025/03/04/Rustup-1.28.1.html)
* [Argumentos genéricos const inferidos: ¡Llamada a prueba!](https://blog.rust-lang.org/inside-rust/2025/03/05/inferred-const-generic-arguments.html)
* [Este mes en nuestra infra de pruebas: enero y febrero de 2025](https://blog.rust-lang.org/inside-rust/2025/03/11/test-infra-jan-feb-2025.html)

### Actualizaciones de proyectos/herramientas
* [Soporte nativo de Git en Zed - Zed Blog](https://zed.dev/blog/git)
* [tfmcp 🦀 : Una herramienta implementada por Rust para operar Terraform desde LLM](https://syu-m-5151.hatenablog.com/entry/2025/03/10/091144)
* [Novedades de SeaORM 1.1](https://www.sea-ql.org/blog/2025-03-08-whats-new-in-sea-orm-1.1/)

### Observaciones/Pensamientos
* [Rust en 2025: Apuntando al software fundamental](https://smallcultfollowing.com/babysteps/blog/2025/03/10/rust-2025-intro/)
* [Un día feliz para Rust](https://steveklabnik.com/writing/a-happy-day-for-rust/)
* [Recursos de aprendizaje de Rust 2025](https://corrode.dev/blog/rust-learning-resources-2025/)
* [Domar a un voraz proxy de Rust](https://fly.io/blog/taming-rust-proxy/)
* [Estructuras de datos sucintas](https://blog.startifact.com/posts/succinct/)
* [¿Cuándo es "este rasgo se puede implementar" parte de la API pública del rasgo?](https://predr.ag/blog/when-is-trait-can-be-implemented-public-api/)
* [¿Cuándo se ejecutan las const fns de Rust?](https://felixwrt.dev/posts/const-fn/)
* [Diseño de objetos de rasgo de Rust](https://neugierig.org/software/blog/2025/03/trait-object-layout.html)
* [El arte de formatear el código](https://mcyoung.xyz/2025/03/11/formatters/)
* [video] [Rust is the New C](https://www.youtube.com/watch?v=3e-nauaCkgo)
* [audio] [Rust con Guillaume Gomez](https://rustacean-station.org/episode/guillaume-gomez/)

### Tutoriales de Rust
* [Escribir en búferes no inicializados en Rust](https://blog.sunfishcode.online/writingintouninitializedbuffersinrust/)
* [Traduciendo bzip2 con c2rust](https://trifectatech.org/blog/translating-bzip2-with-c2rust/)
* [Nine Pico PIO Wats with Rust: Raspberry Pi programable IO pitfalls ilustrado con un ejemplo musical (Parte 1)](https://towardsdatascience.com/nine-pico-pio-wats-with-rust-part-1-9d062067dc25/)
* [Rust asíncrono para dummies](https://blog.veeso.dev/blog/en/async-rust-for-dummies/)
* [Cómo construimos nuestras demostraciones de Embedded World 2025](https://ferrous-systems.com/blog/embedded-world-2025-demos/)
* [video] [Ratatui - interfaces de usuario de terminal en Rust con Orhun Parmaksız - construir ratatop en programación de pares](https://www.youtube.com/watch?v=OkmYsa25pIw)
* [video] [Derivar macros: O, cómo aprendí a dejar de preocuparme y amar el proc_macro2::TokenStream](https://www.youtube.com/watch?v=ALZr9BwWHQU&t=1769s)
* [video] [Portando el dispositivo de trama guff a Rust](https://www.youtube.com/watch?v=bbWcGAOsbIE)

### Miscelánea
* [Mapa del Mundo de Comunidades de Rust/Grupos de Usuarios](https://mamaicode.github.io/rust-communities-map/)

## Crate de la semana

El crate de esta semana es [eval-macro](https://crates.io/crates/eval-macro), una caja que permite evaluar macros en tiempo de compilación, dando una sensación similar al comptime de Zig.

¡Gracias a [Aleksander Krauze](https://users.rust-lang.org/t/crate-of-the-week/2704/1419) por la sugerencia!

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

Háganos saber si desea que se realice un seguimiento de su función como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->
*Esta semana no se han presentado convocatorias para participar.* 

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->
* [**GOSIM Rust Spotlight - ¡Nomina y apoya tus proyectos favoritos!**](https://spotlight.gosim.org/rust2025#deadline-extended) | Cierra el 15/03/2025 a las 7:59 a. m. UTC | Utrecht, NL | 2025-05-13 - 2025-05-17

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 555 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-04..2025-03-11

#### Compilador

* [conteo de referencias ergonómico](https://github.com/rust-lang/rust/pull/134797)
* [en tramos largos, recorte la mitad de ellos para que quepan en el ancho del terminal](https://github.com/rust-lang/rust/pull/137757)
* [dividir el iterador 'Edges'](https://github.com/rust-lang/rust/pull/137655)
* [perf: cambiar TaskDeps para iniciar preasignado con 128 de capacidad](https://github.com/rust-lang/rust/pull/137563)
* [perf: acelerar el cálculo de la característica objetivo](https://github.com/rust-lang/rust/pull/137586)

#### Biblioteca

* [estabilizar '[T]::split_off... ' métodos](https://github.com/rust-lang/rust/pull/137829)
* [estabilizar 'box_uninit_write'](https://github.com/rust-lang/rust/pull/137850)
* [estabilizar 'const_char_classify, const_sockaddr_setters'](https://github.com/rust-lang/rust/pull/138129)
* [estabilizar 'const_vec_string_slice'](https://github.com/rust-lang/rust/pull/137319)
* [estabilizar 'string_extend_from_within'](https://github.com/rust-lang/rust/pull/137569)
* [estabilizar la función 'const_copy_from_slice'](https://github.com/rust-lang/rust/pull/138098)
* [anular los métodos predeterminados de 'Escritura' para tipos similares al cursor](https://github.com/rust-lang/rust/pull/137107)
* [especialice 'OsString::p ush' y 'OsString as From' para UTF-8](https://github.com/rust-lang/rust/pull/137777)
* [perf: mejorar el MIR genérico en el predeterminado 'PartialOrd::le' y amigos](https://github.com/rust-lang/rust/pull/137904)
* [cuente el ancho de los caracteres como máximo una vez en 'Formatter::p ad'](https://github.com/rust-lang/rust/pull/136662)
* [se corrige el conteo de caracteres en 'Display' para 'ByteStr'](https://github.com/rust-lang/rust/pull/137772)
* [arreglar bloqueo en 'BufReader::p eek()'](https://github.com/rust-lang/rust/pull/137832)

#### Carga

* [árbol de carga: Añade '--depth public' detrás de '-Zunstable-options'](https://github.com/rust-lang/cargo/pull/15243)
* [cargo: agregar integración de terminal a través de ANSI OSC 9; 4 secuencias](https://github.com/rust-lang/cargo/pull/14615)
* [cargo: no uses '$CARGO_BUILD_TARGET' en 'metadatos de carga'](https://github.com/rust-lang/cargo/pull/15271)
* [cargo: añadir terminaciones para add --path](https://github.com/rust-lang/cargo/pull/15288)
* [cargo: añadir finalizaciones para install --path](https://github.com/rust-lang/cargo/pull/15266)
* [Cargo: se acepta el respeto --congelado en todas partes --fuera de línea o --bloqueado](https://github.com/rust-lang/cargo/pull/15263)

#### Rustdoc

* [corregir el uso de la pila 'O(tests)' en la edición 2024 de doctests fusionables](https://github.com/rust-lang/rust/pull/138281)
* [buscar: aumentar el rigor de la comprobación de tipos](https://github.com/rust-lang/rust/pull/137981)* [rustdoc: añadir pruebas relacionadas con atributos para rustdoc JSON](https://github.com/rust-lang/rust/pull/138033)
* [ocultar elemento que no está marcado como 'doc(inline)' y cuyo src es 'doc(hidden)'](https://github.com/rust-lang/rust/pull/137534)

#### Clippy

* [clippy: 'arbitrary_source_item_ordering': Hacer que el orden alfabético en los grupos de elementos del módulo sea opcional](https://github.com/rust-lang/rust-clippy/pull/13718)
* [clippy: 'unnecessary_to_owned': no llames a 'iter()' en un objeto temporal](https://github.com/rust-lang/rust-clippy/pull/14243)
* [clippy: agregar anotaciones de pruebas faltantes para 'ui-internal'](https://github.com/rust-lang/rust-clippy/pull/14388)
* [clippy: no active 'blocks_in_conditions' cuando la condición contiene un 'return'](https://github.com/rust-lang/rust-clippy/pull/14338)
* [clippy: no activar 'unnecessary_debug_formatting' en las pruebas](https://github.com/rust-lang/rust-clippy/pull/14347)
* [clippy: arreglar el modo de enlace faltante de 'manual_let_else'](https://github.com/rust-lang/rust-clippy/pull/14204)
* [clippy: mejor ayuda para 'mixed_case_hex_literals'](https://github.com/rust-lang/rust-clippy/pull/14235)
* [clippy: mejorar la sugerencia de 'needless_pass_by_value'](https://github.com/rust-lang/rust-clippy/pull/13880)
* [clippy: hacer que 'struct_field_names' verifique los campos privados de las estructuras públicas](https://github.com/rust-lang/rust-clippy/pull/14076)
* [clippy: función de refactorización después de agregar un nuevo elemento de diagnóstico](https://github.com/rust-lang/rust-clippy/pull/14306)
* [clippy: eliminar la sección de problemas conocidos para 'vec_box'](https://github.com/rust-lang/rust-clippy/pull/14252)
* [clippy: renombra el alias de MSRV 'MANUAL_DIV_CEIL' a 'DIV_CEIL'](https://github.com/rust-lang/rust-clippy/pull/14329)
* [clippy: usa 'size_of' del preludio en lugar de importado](https://github.com/rust-lang/rust-clippy/pull/14355)
* [clippy: 'io_error_other': regresa al contexto raíz para calcular el intervalo](https://github.com/rust-lang/rust-clippy/pull/14349)

#### Analizador de Rust

* [rust-analyzer: 'fix(hir): VariantDef' is 'impl HasSource'](https://github.com/rust-lang/rust-analyzer/pull/19314)
* [Rust-analyzer: Agregar los padres name-ref que faltan para resaltar sintáctico](https://github.com/rust-lang/rust-analyzer/pull/19326)
* [rust-analyzer: agregar información de advertencia y depuración cuando fallan los 'metadatos de carga'](https://github.com/rust-lang/rust-analyzer/pull/19290)
* [Rust-Analyzer: Ajuste el umbral de puntuación de relevancia para que sea coherente con la implementación existente...](https://github.com/rust-lang/rust-analyzer/pull/19297)
* [Rust-analyzer: agregue diagnóstico para Dyn, Impl y Dhangling Dyn](https://github.com/rust-lang/rust-analyzer/pull/19265)
* [rust-analyzer: avisa al usuario cuando un cambio de nombre cambiará el significado del programa](https://github.com/rust-lang/rust-analyzer/pull/19079)
* [Rust-Analyzer: Macrohigiene de 'ruta'](https://github.com/rust-lang/rust-analyzer/pull/19327)
* [rust-analyzer: resaltado de sintaxis, filtrado de puntos ignorando mods](https://github.com/rust-lang/rust-analyzer/pull/19292)
* [rust-analyzer: arreglar los diagnósticos que se borran justo después de ser recibidos](https://github.com/rust-lang/rust-analyzer/pull/19333)
* [Rust-Analyzer: Normalizar las proyecciones en la visualización de const evaluada y el cálculo del diseño](https://github.com/rust-lang/rust-analyzer/pull/19330)
* [rust-analyzer: evitar invocaciones incorrectas de 'needs_parens_in' con "padres" no ancestrales](https://github.com/rust-lang/rust-analyzer/pull/19324)
* [Rust-analyzer: resalte las operaciones inseguras como inseguras, no definiciones](https://github.com/rust-lang/rust-analyzer/pull/19274)
* [rust-analyzer: mejorar la finalización de palabras clave para 'let' y 'let mut'](https://github.com/rust-lang/rust-analyzer/pull/19279)
* [rust-analyzer: Salida de error del script de compilación de registros en 'load_cargo::load_workspace_at'](https://github.com/rust-lang/rust-analyzer/pull/19311)
* [rust-analyzer: make 'GenericParamsCollector::type_or_consts' no innecesariamente 'pub(crate)'](https://github.com/rust-lang/rust-analyzer/pull/19343)
* [rust-analyzer: hacer anotaciones de cambio por edición de texto](https://github.com/rust-lang/rust-analyzer/pull/19332)
* [rust-analyzer: mover el MSRV del proyecto cargado de nuevo a 1.78, mostrar notificación para la advertencia](https://github.com/rust-lang/rust-analyzer/pull/19308)
* [rust-analyzer: clasificar los constructores de ADT como constructores para la puntuación de finalización](https://github.com/rust-lang/rust-analyzer/pull/19325)

### Clasificación del rendimiento del compilador de Rust

Esta semana tuvimos que fusionar muchos rollups grandes debido a muchos problemas con nuestra infraestructura de CI,
lo que dificultó el análisis. A pesar de que las estadísticas agregadas parecen haber muchas regresiones,
Está sesgado por dos grandes regresiones que ocurren en una compilación incremental optimizada poco común y un
Construcción de documentación de una sola caja. Se está realizando un seguimiento de la regresión de la documentación y se corrigen las correcciones
Algunas otras regresiones ya están en curso.

Triaje realizado por **@kobzol**.
Rango de revisión: [daf59857.. 9fb94b32](https://perf.rust-lang.org/?start=daf59857d6d2b87af4b846316bf1561a6083ed51&end=9fb94b32df38073bf63d009df77ed10cb1c989d0&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 1.2% | [0.2%, 58.8%] | 149 |
| Regresiones ❌ <br /> (secundaria) | 4.2% | [0.2%, 165.8%] | 127 |
| Mejoras ✅ <br /> (primario) | -1,1% | [-14.0%, -0.3%] | 31 |
| Mejoras ✅ <br /> (secundaria) | -2,9% | [-38.4%, -0.1%] | 43 |
| Todos ❌✅ (primarios) | 0.8% | [-14.0%, 58.8%] | 180 |

2 regresiones, 2 mejoras, 5 mixtas; 4 de ellos en rollups
37 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/38617ae5d7a849d2f7fc7a712c737768b6ee4a90/triage/2025-03-11.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC: Deseche el campo 'edition' por destino de compilación en 'Cargo.toml'](https://github.com/rust-lang/rfcs/pull/3772)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Levanta la pelusa 'clippy::invalid_null_ptr_usage'](https://github.com/rust-lang/rust/pull/119220)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC para doc_cfg, doc_cfg_auto, doc_cfg_hide y doc_cfg_show características](https://github.com/rust-lang/rfcs/pull/3631)

#### Otras áreas
* No hay artículos ingresados al Período Final de Comentarios esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: ergonomía constante para NonZero\<T\>](https://github.com/rust-lang/rfcs/pull/3786)

## Próximos eventos

Eventos oxidados entre 2025-03-12 - 2025-04-09 🦀

### Virtual
* 13/03/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820296)
* 18/03/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**crum: Números Complejos y Matrices Complejas en Rust con Frans Slabber**](https://www.meetup.com/code-mavens/events/305823397/)
* 18/03/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**A mediados de mes Rustful—Rust GameDev Basics with Raylib por Tony Bradley**](https://www.meetup.com/rustdc/events/305170694)
* 19/03/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Tocino y Benchmarking de Rendimiento**](https://www.meetup.com/vancouver-rust/events/305470139)
* 2025-03-20 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Rust y programación integrada con Leon Vak (en línea en inglés)**](https://www.meetup.com/code-mavens/events/306357728)
* 25/03/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361431)
* 25/03/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: SKIing into Rust - elaborando un intérprete sencillo**](https://www.meetup.com/women-in-rust/events/305988711)
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
* 08/04/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/303522530)

### Asia
* 15/03/2025 | Pekín, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**KCD Beijing 2025**](https://www.meetup.com/wasm-rust-meetup/events/303112196)
* 19/03/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust March 2025 en Jit en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)
* 2025-03-28 | Kowloon Tong, HK | [Rust de Asia](https://www.rustasiaconf.com/)
    * [**Rust Asia 2025**](https://www.rustasiaconf.com/)
* 05/04/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de abril de 2025**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)

### Europa
* 12/03/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 13/03/2025 | Biel, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #2 @ BFH Biel**](https://www.meetup.com/rust-bern/events/306169573)
* 14/03/2025 | París, FR | [Rust en París](https://www.rustinparis.com/)
    * [**Rust en París 2025**](https://www.rustinparis.com/schedule)
* 18/03/2025 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #10 @ MDPI Basel**](https://www.meetup.com/rust-basel/events/306121044)
* 18/03/2025 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**(1) Mago; (2) Iggy; (2) Tamaños binarios de Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729673)
* 2025-03-20 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**¡Conversaciones de marzo! (Dos)**](https://www.meetup.com/rust-and-friends/events/306524042)
* 2025-03-20 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (marzo de 2025)**](https://www.meetup.com/rust-prague/events/306512157)
* 25/03/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Edición Robot**](https://www.meetup.com/rust-aarhus/events/306478352)
* 25/03/2025 | Eindhoven, Países Bajos | [Rust](https://www.meetup.com/rust-amsterdam/events/)
    * [**Rust x Julia Meetup Eindhoven**](https://www.meetup.com/rust-nederland/events/306434865)
* 25/03/2025 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group/events/)
    * [**Sumérgete en el Rust asíncrono**](https://www.meetup.com/london-rust-project-group/events/306643055)
* 2025-03-26 | Fráncfort, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**"¡Más que increíblemente rápido!" Performance Optimierungen in Rust**](https://www.meetup.com/rust-rhein-main/events/306659893)
* 2025-03-26 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester habla de marcha**](https://www.meetup.com/rust-manchester/events/305897029)
* 2025-03-26 | Varsovia, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon**](https://www.rustikon.dev/)
* 27/03/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #12: Probando en Rust**](https://rust-augsburg.github.io/meetup/Meetup_12.html)
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
* 09/04/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045446)

### América del Norte
* 13/03/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306484710)
* 13/03/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**PDXRust Meetup: Encontrando una salida**](https://www.meetup.com/pdxrust/events/306658850)
* 18/03/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638264)
* 18/03/2025 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Encuentro mensual: Introducción a Rust y Python; ¡Usando Rustup, Cargo y Rust!**](https://www.meetup.com/spokane-rust/events/306368210)
* 2025-03-20 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/306473234)
* 2025-03-20 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 3: Presentaciones de la comunidad**](https://www.meetup.com/music-city-rust-developers/events/304333074/)
* 2025-03-20 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Marzo de 2025 SRUG (Grupo de usuarios de Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658448)
* 21/03/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y AWS Lambda. Preparando el camino para desplegar ML/AI**](https://www.meetup.com/rust-mx/events/306406018)
* 2025-03-26 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcfbjc)
* 27/03/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/306470345)
* 31/03/2025 | Boulder, CO, EE. UU. | [Depósito de estado sólido](https://www.meetup.com/solidstatedepot/events/)
    * [**Boulder Rust: Bryan presenta Rusted Hardware**](https://www.meetup.com/solidstatedepot/events/306573447)
* 03/04/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Abril Mensual Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 03/04/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - Internacionalización con restricciones de recursos (i18N)**](https://www.meetup.com/stl-rust/events/304890140)

### América del Sur
* 15/03/2025 | São Paulo, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na CloudWalk**](https://www.meetup.com/rust-sao-paulo-meetup/events/306034427)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1ivrkhs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Trata *cualquier cosa* que comience con 'carga' como si fuera 'carga de carrera'. Esto se aplica incluso a los comandos que no construyen nada, como 'cargo clean', y a los plugins de terceros, como 'cargo audit'.

– [Sergey "Shnatsel" Davidoff en /r/rust](https://old.reddit.com/r/rust/comments/1j2i3s0/psa_do_not_run_any_cargo_commands_on_untrusted)

¡Gracias a [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1661) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ja2n2d/this_week_in_rust_590/)</small>
