---
title: "Esta semana en Rust #52"
number_of_week: 52
description: El crate de esta semana es dom\_smoothie, una caja para extraer contenido legible de páginas web.
date: 2025-03-19
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

* [Anunciando Rust 1.85.1](https://blog.rust-lang.org/2025/03/18/Rust-1.85.1.html)
* [Contratación para la gestión del programa Rust](https://blog.rust-lang.org/inside-rust/2025/03/18/hiring-for-program-management.html)
* [Actualización del Consejo de Liderazgo de marzo de 2025](https://blog.rust-lang.org/inside-rust/2025/03/17/leadership-council-update.html)

### Actualizaciones de proyectos/herramientas

* [analizador de Rust #277](https://rust-analyzer.github.io/thisweek/2025/03/17/changelog-277.html)
* [Git 2.49 lanzado con empaquetado más rápido, interfaz de idioma extranjero de Rust](https://www.phoronix.com/news/Git-2.49-Released)
* [Actualización del proyecto Rust CUDA](https://rust-gpu.github.io/blog/2025/03/18/rust-cuda-update/)
* [Gran actualización de Rust fusionada para GCC 15 - Tierras El verificador de préstamos de Polonio](https://www.phoronix.com/news/GCC-15-Big-GCCRS-Update)
* [Oxidando Ubuntu: adoptando las utilidades de Rust por defecto](https://lwn.net/SubscriberLink/1014002/c1fa692b30a202c9/)
* [Hoja de ruta de Apache OpenDAL 2025: Perfeccionando la adopción de la producción](https://opendal.apache.org/blog/2025/03/01/2025-roadmap/)
* ['bevy_lint' v0.2.0: lint tus proyectos de Bevy](https://bd103.github.io/blog/2025-03-19-bevy-lint-v0.2.0)
* [¿Por qué Yōzefu? Una interfaz de usuario de terminal para buscar datos en un clúster de Kafka](https://mcdostone.github.io/articles/why-yozefu/)
* [Anunciando AIScript y cómo lo construí](https://aiscript.dev/blog/announcing-aiscript)
* [Anunciando mocktail: Simulacro de servidor HTTP y gRPC para Rust](https://danclark.io/blog/announcing-mocktail/)

### Observaciones/Pensamientos

* [Cómo acelerar el compilador de Rust en marzo de 2025](https://nnethercote.github.io/2025/03/19/how-to-speed-up-the-rust-compiler-in-march-2025.html)
* [Oxidando Ubuntu cuidadosa pero deliberadamente](https://jnsgr.uk/2025/03/carefully-but-purposefully-oxidising-ubuntu/)
* [abajo: Directorio mundial de escritura en /var/log/below permite la escalada de privilegios locales](https://security.opensuse.org/2025/03/12/below-world-writable-log-dir.html)
* [Extendiendo el futuro en Rust](https://blog.veeso.dev/blog/en/extending-future-in-rust/)
* [Escribir un código terrible](https://bitfieldconsulting.com/posts/writing-terrible-code)
* [ALP-RS es más rápido que la referencia de C++](https://blog.spiraldb.com/alp-rust-is-faster-than-c/)
* [video] [No es tan simple como "Usar un lenguaje seguro para la memoria"](https://www.youtube.com/watch?v=iQ-eTaW6-cM)
* [video] [Ubuntu reemplazará las utilidades principales de GNU con Rust](https://www.youtube.com/watch?v=N2dbyFddcIs)
* [video] [¿Qué le pasa a Rust?](https://www.youtube.com/watch?v=pppU--kHLP0)

### Tutoriales de Rust

* [Crear una aplicación web + de escritorio con Rust](https://medium.com/gitconnected/build-a-web-desktop-application-with-rust-i-8eb12cf160b6) 
* [Sistemas de transición en Rust](https://minikin.me/blog/transition-systems-in-rust)
* [Nine Pico PIO Wats with Rust: Raspberry Pi programmable IO Pitfalls Ilustrado con un Ejemplo Musical (Parte 2)](https://towardsdatascience.com/nine-pico-pio-wats-with-rust-part-2/)
* [serie] [Construyendo un motor de búsqueda desde cero, en Rust](https://jdrouet.github.io/posts/202503161800-search-engine-intro/)
* [video] [Build with Naz : patrón de diseño newtype, e impl Into T para APIs ergonómicas](https://www.youtube.com/watch?v=3-Ika3mAOGQ)

### Miscelánea
* [Informe de empleo de Rust de febrero de 2025](https://filtra.io/rust/jobs-report/feb-25)

## Crate de la semana

El crate de esta semana es [dom\_smoothie](https://github.com/niklak/dom_smoothie), una caja para extraer contenido legible de páginas web.

A pesar de la falta de sugerencias esta semana, llogiq está satisfecho con su elección.

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

Se [fusionaron 468 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-11..2025-03-18

#### Compilador

* [perf:allow bounds checks when enumerating 'IndexSlice' to be elided](https://github.com/rust-lang/rust/pull/137795)
* [Estabilizar puerta de función 'asm_goto'](https://github.com/rust-lang/rust/pull/133870)

#### Miri

* ['native_calls': aseguramos que realmente exponemos la procedencia *mutable* a la memoria a la que FFI puede acceder](https://github.com/rust-lang/rust/pull/138352)
* ['alloc_addresses': use MemoryKind en lugar de la consulta tcx para determinar las asignaciones globales](https://github.com/rust-lang/miri/pull/4225)

#### Bibliotecas

* [añadir 'From<{integer}>' para 'f16'/'f128' impls](https://github.com/rust-lang/rust/pull/138363)
* [denote 'ControlFlow' como '#[must_use]'](https://github.com/rust-lang/rust/pull/137449)
* [optimizar patrones de cadenas de varios caracteres](https://github.com/rust-lang/rust/pull/138537)
* [estabilizar 'std::io::ErrorKind::InvalidFilename'](https://github.com/rust-lang/rust/pull/134076)
* [Estabilizar tubería anónima](https://github.com/rust-lang/rust/pull/137793)

#### Carga

* [Agregar completador personalizado para carga '+<TAB>' para completar el nombre de la cadena de herramientas](https://github.com/rust-lang/cargo/pull/15301)
* [Comando de deduplicación de tipos de cajas en Cargo rustc](https://github.com/rust-lang/cargo/pull/15314)

#### Rustdoc

* [añadir soporte RTN a rustdoc](https://github.com/rust-lang/rust/pull/137956)
* [rustdoc-json: no incluyas también '#[deprecated]' en 'Item::attrs'](https://github.com/rust-lang/rust/pull/138577)

#### Rustfmt

* [rustfmt: permitir también permitir literales como primer elemento de la cadena let de una sola línea](https://github.com/rust-lang/rustfmt/pull/6492)

#### Clippy

* [nueva pelusa: 'doc_comment_double_space_linebreaks'](https://github.com/rust-lang/rust-clippy/pull/12876)
* ['incompatible_msrv': llamadas a la función lint con cualquier recuento de argumentos](https://github.com/rust-lang/rust-clippy/pull/14216)
* ['needless_pass_by_value': referencia al contenido de 'Opción' más interno](https://github.com/rust-lang/rust-clippy/pull/14392)
* ['question_mark': evitar sugerencias incorrectas cuando se usa el enlace 'ref'](https://github.com/rust-lang/rust-clippy/pull/14158)
* [arreglar la pelusa 'from_over_into' que sugiere un código no válido](https://github.com/rust-lang/rust-clippy/pull/14409)
* [corregir sugerencias incorrectas relacionadas con paréntesis en 'needless_return'](https://github.com/rust-lang/rust-clippy/pull/14094)
* [arreglar 'unnecessary_safety_comment' falso positivo en la asignación de azúcar](https://github.com/rust-lang/rust-clippy/pull/14371)

#### Analizador de Rust

* [añadir iconos a las vistas](https://github.com/rust-lang/rust-analyzer/pull/19344)
* [analysis-stats: ejecuta el LRU de Salsa al final del análisis](https://github.com/rust-lang/rust-analyzer/pull/19378)
* [mostrar varargs en detalle de finalización](https://github.com/rust-lang/rust-analyzer/pull/19363)
* [no cometer errores para acciones sin datos para resolver](https://github.com/rust-lang/rust-analyzer/pull/19369)
* [para bucle a mientras deja asistir](https://github.com/rust-lang/rust-analyzer/pull/19271)
* [arreglar paquetes de prueba con múltiples objetivos](https://github.com/rust-lang/rust-analyzer/pull/19005)
* [evitar depurar recursivamente las cajas de impresión](https://github.com/rust-lang/rust-analyzer/pull/19356)
* [arreglar el informe obsoleto de 'Building CrateGraph'](https://github.com/rust-lang/rust-analyzer/pull/19384)
* [observe la inseguridad al generar implicaciones manuales de derivadas anteriores](https://github.com/rust-lang/rust-analyzer/pull/19320)
* [preparación para la notación de tipo de retorno (RTN)](https://github.com/rust-lang/rust-analyzer/pull/19354)
* [Analizar el Rust del puerto a la nueva salsa](https://github.com/rust-lang/rust-analyzer/pull/18964)
* [Gráfico de salsifíe la caja](https://github.com/rust-lang/rust-analyzer/pull/19337)

### Clasificación del rendimiento del compilador de Rust

Una semana relativamente ocupada con una gran cantidad de regresiones en los rollups, lo que complicó las investigaciones. Afortunadamente, en general, la semana fue una mejora debido a algunas mejoras de tamaño medio a través de la mejora del cálculo de características objetivo y una corrección de los componentes internos de los sistemas de tipos.

Triaje realizado por **@rylev**.
Rango de revisión: [9fb94b32.. 493C38Ba](https://perf.rust-lang.org/?start=9fb94b32df38073bf63d009df77ed10cb1c989d0&end=493c38ba371929579fe136df26eccd9516347c7a&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 1.7% | [0.2%, 3.0%] | 18 |
| Regresiones ❌ <br /> (secundaria) | 0.8% | [0.2%, 2.7%] | 37 |
| Mejoras ✅ <br /> (primario) | -1.0% | [-10.3%, -0.2%] | 157 |
| Mejoras ✅ <br /> (secundaria) | -1,7% | [-8.8%, -0.2%] | 158 |
| Todos ❌✅ (primarios) | -0.8% | [-10.3%, 3.0%] | 175 |

5 regresiones, 5 mejoras, 3 mixtas; 5 de ellos en rollups
44 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/8148c03b441e9a23b93dab2f2c7124eaf9ff925b/triage/2025-03-18.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC para doc_cfg, doc_cfg_auto, doc_cfg_hide y doc_cfg_show características](https://github.com/rust-lang/rfcs/pull/3631)
* [RFC: Degradar i686-pc-windows-gnu a Tier 2](https://github.com/rust-lang/rfcs/pull/3771)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [¡Deja de usar el inestable 'concat_idents!'](https://github.com/rust-lang/rust/pull/137653)
* [Estabilizar '#! [característica(precise_capturing_in_traits)]'](https://github.com/rust-lang/rust/pull/138128)

##### [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: cerrar] [RFC: Agregar nombres descriptivos a las pruebas documentales](https://github.com/rust-lang/rfcs/pull/3311)

#### Otras áreas
* *No hay artículos ingresados al Período Final de Comentarios esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) o
  [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus solicitudes de incorporación de cambios, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: '--crate-attr'](https://github.com/rust-lang/rfcs/pull/3791)

## Próximos eventos

Eventos oxidados entre 2025-03-19 - 2025-04-16 🦀

### Virtual
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

### Asia
* 19/03/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust March 2025 en Jit en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)
* 2025-03-28 | Kowloon Tong, HK | [Rust de Asia](https://www.rustasiaconf.com/)
    * [**Rust Asia 2025**](https://www.rustasiaconf.com/)
* 05/04/2025 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de abril de 2025**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)

### Europa
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

### América del Norte
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
* 2025-03-26 | Nueva York, NY, EE. UU. | [Rust Nueva York](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: No puedo creer que sea legal Rust con Michael Gattozzi (NUEVA UBICACIÓN)**](https://www.meetup.com/rust-nyc/events/306777565)
* 27/03/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**¡Vamos de nuevo!**](https://www.meetup.com/rust-atl/events/306470345)
* 31/03/2025 | Boulder, CO, EE. UU. | [Depósito de estado sólido](https://www.meetup.com/solidstatedepot/events/)
    * [**Boulder Rust: Bryan presenta Rusted Hardware**](https://www.meetup.com/solidstatedepot/events/306573447)
* 03/04/2025 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306728493)
* 03/04/2025 | Montreal, QC, CA | [Rust Montreal](https://www.meetup.com/rust-montreal/events/)
    * [**Abril Mensual Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 03/04/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - Internacionalización con restricciones de recursos (i18N)**](https://www.meetup.com/stl-rust/events/304890140)
* 10/04/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**TetaNES: Una vacuna contra la roya: sin aguja, solo el verificador de préstamos**](https://www.meetup.com/pdxrust/events/306714209)

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

> Probablemente sea una idea terrible, pero disfruto lanzando ideas a la pared y viendo lo afilados que están sus fragmentos rotos.

– [Katt en la discusión RFC #3762](https://github.com/rust-lang/rfcs/pull/3762#discussion_r1990901450)

¡Gracias a [Jacob Lifshay](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1662) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1jfitub/this_week_in_rust_591/)</small>
