---
title: "Esta semana en Rust #54"
number_of_week: 54
description: El crate de esta semana es candystore, un almacén de clave-valor rápido y persistente que no requiere LSM ni WALs.
date: 2025-04-02
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

### Boletines
* [El Rustáceo Incrustado Edición #42](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-42)
* [Esta semana en Bevy - 2025-03-31](https://thisweekinbevy.com/issue/2025-03-31-0160-rc2-breakout-on-game-boy-advance-and-bevyecs-for-static-sites)

### Actualizaciones de proyectos/herramientas
* [Fjall 2.8](https://fjall-rs.github.io/post/fjall-2-8/)
* [EtherCrab, el EtherCAT MainDevice puro de Rust, versión 0.6 lanzada](https://wapl.es/ethercrab-0-6/)
* [Un proceso para manejar el código Rust en el núcleo central](https://lwn.net/SubscriberLink/1015409/be9d004a43a7102d/)
* [api-version: Axum middleware para la selección de versiones basada en encabezados](https://heikoseeberger.de/2025-03-20-api-version/)
* [SALT: una extensión de VS Code, buscando participantes en un estudio sobre la utilidad de Rust](https://marketplace.visualstudio.com/items?itemName=kale-lab.salt)

### Observaciones/Pensamientos
* [Presentación de Stringleton](https://simonask.github.io/introducing-stringleton/)
* [Rust Any Part 3: Finalmente tenemos Upcasts](https://lucumr.pocoo.org/2025/3/27/any-upcast/)
* [Hacia un SIMD intrépido, 7 años después](https://linebender.org/blog/towards-fearless-simd/)
* [LLDB's TypeSystems: Una interfaz inacabada](https://walnut356.github.io/posts/lldbs-typesystems-an-unfinished-interface/)
* [Pruebas de mutación en Rust](https://blog.frankel.ch/mutation-testing-rust/)
* [Incrustación de objetos compartidos en Rust](https://blog.veeso.dev/blog/en/embedding-shared-objects-in-rust/)

### Tutoriales de Rust
* [Arquitectura y construcción de servicios web de tamaño medio en Rust con Axum, SQLx y PostgreSQL](https://kerkour.com/rust-web-services-axum-sqlx-postgresql)
* [Resolviendo el problema de ABA en Rust con punteros de peligro](https://minikin.me/blog/solving-the-aba-problem-in-rust-hazard-pointers)
* [Creación de una aplicación CoAP en Ariel OS](https://christian.amsuess.com/blog/website/2025-03-27_ariel_coap/)
* [Cómo optimizar tu programa de Rust para la lentitud: escribe un programa corto que termine después de que el universo muera](https://medium.com/@carlmkadie/how-to-optimize-your-rust-program-for-slowness-eb2c1a64d184)
* [Dentro del controlador Rust 1.0 de ScyllaDB: Un controlador CQL totalmente asíncrono con reconocimiento de fragmentos usando Tokio](https://www.scylladb.com/2025/03/31/inside-scylladb-rust-driver-1-0/)
* [Construyendo un motor de búsqueda desde cero, en Rust: parte 2](https://jdrouet.github.io/posts/202503191700-search-engine-part-2/)
* [Introducción a Monoio: Un tiempo de ejecución de Rust de alto rendimiento](https://chesedo.me/blog/monoio-introduction/)
* [Introducción a Rust en Google Cloud](https://medium.com/google-cloud/getting-started-with-rust-on-google-cloud-ced48447ec91)

### Miscelánea
* [El ROM de una AlphaStation](https://www.thejpster.org.uk/blog/blog-2025-03-30/)
* [Verificación en el mundo real de software para aplicaciones criptográficas](https://cryptographycaffe.sandboxaq.com/posts/real-world-verification-of-software-for-cryptographic-applications/)
* [Libros mdPúblicos ](https://mdbooks.code-maven.com/)
* [video] [Networking en Bevy con replicación ECS - Hennadii](https://www.youtube.com/watch?v=aDsVFmXD2cc)
* [video] [Representaciones Intermedias para Estructuras Reactivas - Pete](https://www.youtube.com/watch?v=JeXOajFv8Dk)

## Crate de la semana

El crate de esta semana es [candystore](https://docs.rs/candystore/latest/candystore/), un almacén de clave-valor rápido y persistente que no requiere LSM ni WALs.

¡Gracias a [Tomer Filiba](https://users.rust-lang.org/t/crate-of-the-week/2704/1424) por la autosugerencia!

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
* [**Convocatoria de oradores de Rust Conf 2025**](https://rustfoundation.org/media/rustconf-2025-call-for-talk-proposals-open/) | Cierra 2025-04-29 11:59 PM PDT | Seattle, WA, EE. UU. | 2025-09-02 - 2025-09-05

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 438 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-25..2025-04-01

#### Compilador

* [permitir definir opacos en estáticas y consts](https://github.com/rust-lang/rust/pull/138911)
* [Evite ajustar asignaciones constantes en estructuras empaquetadas cuando no sea necesario](https://github.com/rust-lang/rust/pull/138503)
* [realizar menos decodificación si tiene el mismo contexto sintáctico](https://github.com/rust-lang/rust/pull/129827)
* [estabilizar 'precise_capturing_in_traits'](https://github.com/rust-lang/rust/pull/138128)
* [levanta la pelusa 'clippy::invalid_null_ptr_usage' como 'invalid_null_arguments'](https://github.com/rust-lang/rust/pull/119220)

#### Biblioteca

* [permitir hilos de generación después de la destrucción de TLS](https://github.com/rust-lang/rust/pull/138702)
* [anular métodos PartialOrd para bool](https://github.com/rust-lang/rust/pull/138945)
* [¡Simplifique la expansión para 'format_args! ()](https://github.com/rust-lang/rust/pull/139131)
* [estabilizar 'const_cell'](https://github.com/rust-lang/rust/pull/137928)

#### Rustdoc

* [simplificar en gran medida el análisis sintáctico de DocTest y la extracción de información](https://github.com/rust-lang/rust/pull/138104)
* [reorganizar 'Item'/'ItemInner'](https://github.com/rust-lang/rust/pull/138927)

#### Clippy

* [nueva pelusa: 'char_indices_as_byte_indices'](https://github.com/rust-lang/rust-clippy/pull/13435)
* [añadir pelusa 'manual_dangling_ptr'](https://github.com/rust-lang/rust-clippy/pull/14107)
* [respeta '#[esperar]' y '#[permitir]' dentro de los cuerpos de las funciones para 'missing_panics_doc'](https://github.com/rust-lang/rust-clippy/pull/14407)
* [no hacer sugerencias incompletas o inválidas](https://github.com/rust-lang/rust-clippy/pull/14487)
* [no advierta sobre el sombreado en una asignación desestructurante](https://github.com/rust-lang/rust-clippy/pull/14381)
* [expanda 'obfuscated_if_else' para admitir '{then(), then_some()}.unwrap_or_default()'](https://github.com/rust-lang/rust-clippy/pull/14431)
* [arreglar el intervalo principal de 'redundant_pub_crate' al marcar elementos sin nombre](https://github.com/rust-lang/rust-clippy/pull/14516)
* [Arreglar la sugerencia de 'option_if_let_else' cuando la coerción requiere un lanzamiento explícito](https://github.com/rust-lang/rust-clippy/pull/14389)
* [corregir la sugerencia de 'unnested_or_patterns' en 'let'](https://github.com/rust-lang/rust-clippy/pull/14401)
* [hacer que 'collapsible_if' reconozca la característica 'let_chains'](https://github.com/rust-lang/rust-clippy/pull/14481)
* [hacer que 'missing_const_for_fn' opere en MIR no optimizado](https://github.com/rust-lang/rust-clippy/pull/14003)
* [sugerencias más naturales para 'cmp_owned'](https://github.com/rust-lang/rust-clippy/pull/14247)
* ['collapsible_if': evita incluir espacios en blanco anteriores si la línea contiene espacios que no están en blanco](https://github.com/rust-lang/rust-clippy/pull/14480)
* [manejar correctamente la expansión en 'single_match'](https://github.com/rust-lang/rust-clippy/pull/14495)
* [validar rutas en configuraciones 'disallowed_*'](https://github.com/rust-lang/rust-clippy/pull/14397)

#### Analizador de Rust

* [permitir que los autores de cajas controlen la finalización de sus cosas](https://github.com/rust-lang/rust-analyzer/pull/19375)
* [evite confiar en 'block_def_map()' innecesariamente](https://github.com/rust-lang/rust-analyzer/pull/19492)
* [corregir debug sourceFileMap cuando se usa cppvsdbg](https://github.com/rust-lang/rust-analyzer/pull/19475)
* [arreglar la disminución de 'format_args' usando un sufijo entero incorrecto](https://github.com/rust-lang/rust-analyzer/pull/19460)
* [Soluciona un error en el cálculo de reglas huérfanas](https://github.com/rust-lang/rust-analyzer/pull/19466)
* [Arreglar el pánico en curso debido a la división incorrecta de Unicode](https://github.com/rust-lang/rust-analyzer/pull/19490)
* [use durabilidad media para los cambios en el gráfico de cajas, alta para los archivos fuente de la biblioteca](https://github.com/rust-lang/rust-analyzer/pull/19451)

### Clasificación del rendimiento del compilador de Rust

Semana positiva, con muchas mejoras primarias y solo algunas regresiones secundarias. Se revirtió una sola gran regresión.

Triaje realizado por **@panstromek**.
Rango de revisión: [4510e86a.. 2ea33b59](https://perf.rust-lang.org/?start=4510e86a41388733675465a8647d4235f3bf2023&end=2ea33b591050c4ca1a3752830b29112638faecf6&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | -     | -              | 0 |
| Regresiones ❌ <br /> (secundaria) | 0.9% | [0.2%, 1.5%] | 17 |
| Mejoras ✅ <br /> (primario) | -0,4% | [-4.5%, -0.1%] | 136 |
| Mejoras ✅ <br /> (secundaria) | -0,6% | [-3.2%, -0.1%] | 59 |
| Todos ❌✅ (primarios) | -0,4% | [-4.5%, -0.1%] | 136 |

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/9bd6fc2f4594023b82acd8d876dcf659aee9a931/triage/2025-03-31.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para slice::array_chunks](https://github.com/rust-lang/rust/issues/74985)
* [Estabilizar 'cfg_boolean_literals'](https://github.com/rust-lang/rust/pull/138632)
* [La promesa 'array::from_fn se genera en orden de índices crecientes'](https://github.com/rust-lang/rust/pull/139099)
* [Estabilizar 'repr128'](https://github.com/rust-lang/rust/pull/138285)
* [Estabilizar 'naked_functions'](https://github.com/rust-lang/rust/pull/134213)
* [Arreglar la const que faltaba para los métodos de 'reemplazo' de puntero inherente](https://github.com/rust-lang/rust/pull/136877)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [core::marker::NoCell in bounds (anteriormente conocido como [sic] 'Freeze')](https://github.com/rust-lang/rfcs/pull/3633)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
* [Estabilizar la recolección automática de basura.](https://github.com/rust-lang/cargo/pull/14287)

#### Otras áreas
* No hay artículos ingresados al Período Final de Comentarios esta semana para
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [Permitir '&&', '||', y '!' en 'cfg'](https://github.com/rust-lang/rfcs/pull/3796)

## Próximos eventos

Eventos oxidados entre 2025-04-02 - 2025-04-30 🦀

### Virtual
* 02/04/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031661)
* 03/04/2025 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820282/)
* 03/04/2025 | Virtual | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
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
* 23/04/2025 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Beyond embedded - Desarrollo de sistemas operativos en Rust **](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307036053)
* 24/04/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820299)
* 24/04/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parte 2: ¡Las computadoras cuánticas no pueden proteger esto contra el Rust!" **](https://www.meetup.com/charlottesville-rust-meetup/events/306679733)

### Asia
* 05/04/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de abril de 2025**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)
* 2025-04-22 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust abril de 2025 en Braavos en Tel Aviv en colaboración con StarkWare**](https://www.meetup.com/rust-tlv/events/306530984)

### Europa
* 02/04/2025 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/306553077)
* 02/04/2025 | Köln, DE | [Colonia Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in April: Rust Embedded, Show and Tell**](https://www.meetup.com/rustcologne/events/306940549)
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
* 24/04/2025 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charla en MFT Energy**](https://www.meetup.com/rust-aarhus/events/305809344)
* 24/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub nocturno)**](https://www.meetup.com/rust-and-friends/events/306911347)
* 24/04/2025 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Code Night**](https://www.meetup.com/rust-manchester/events/306899063)
* 25/04/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/306911357)
* 29/04/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #76**](https://www.meetup.com/rust-paris/events/306952202)

### América del Norte
* 03/04/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306728493)
* 03/04/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Abril Mensual Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 03/04/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - Internacionalización con restricciones de recursos (i18N)**](https://www.meetup.com/stl-rust/events/304890140)
* 06/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Kendall Rust Lunch, 6 de abril**](https://www.meetup.com/bostonrust/events/306844327)
* 08/04/2025 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Construyendo una extensión de Postgres de búsqueda de texto completo en Rust**](https://www.meetup.com/rust-nyc/events/306983122)
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
* 25/04/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Ball Square Rust, 25 de abril**](https://www.meetup.com/bostonrust/events/306844343)

### Oceanía
* 09/04/2025 | Sídney, NS, AU | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**Cangrejo 🦀 X 🕳️🐇 **](https://www.meetup.com/rust-sydney/events/306978026)
* 14/04/2025 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306841248)
* 2025-04-22 | Barton, AC, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de abril**](https://www.meetup.com/rust-canberra/events/306425557)

### América del Sur
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

> Si escribes un error en tu programa Rust, Rust no te culpa. Rust se pregunta "¿cómo pudo el compilador haber detectado ese error?". 

– [Ian Jackson blogueando sobre Rust](https://diziet.dreamwidth.org/19480.html)

A pesar de la falta de sugerencias, llogiq está bastante satisfecho con su elección.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1jqlycx/this_week_in_rust_593/)</small>
