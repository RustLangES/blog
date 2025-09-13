---
title: "Esta semana en Rust #75"
number_of_week: 75
description: El crate de esta semana es GrimoireCSS, un motor CSS creado en Rust,
date: 2025-09-10
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software confiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[envíenos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!--

Estimados colaboradores de la comunidad:
Lea README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe tener la siguiente forma:

* [Título de la página enlazada](https://example.com/my_article)

Si agrega un enlace a un contenido que no es de texto, prefije el prefijo '[video]' o '[audio]':

* [video] [Título del video vinculado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio vinculado](https://example.com/my_podcast)

Si no sabe qué categoría usar, no dude en enviar un PR de todos modos
y solo pida a los editores que seleccionen la categoría.

-->

### Oficial
* [Cruzando los arroyos: Proyecto + Fundación](https://blog.rust-lang.org/inside-rust/2025/09/04/crossing-the-streams/)
* [Resultados de la encuesta de rendimiento del compilador de Rust 2025](https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/)

### Boletines
* [Este mes en Rust OSDev: agosto de 2025](https://rust-osdev.com/this-month/2025-08/)

### Actualizaciones de proyectos/herramientas
* [Extensión Rust Automod VSCode - Automatiza la creación y administración de archivos 'mod.rs'](https://marketplace.visualstudio.com/items?itemName=drkryz.rustautomod)
* [Ahora disponible: SDK de Rust para Google Cloud](https://cloud.google.com/blog/topics/developers-practitioners/now-available-rust-sdk-for-google-cloud/)

### Observaciones/Pensamientos
* [Protegiendo a Rust contra ataques a la cadena de suministro](https://kerkour.com/rust-supply-chain-attacks)
* [La efectividad irrazonable de los algoritmos de clasificación modernos](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/unreasonable/text.md)
* [Mejora de la generación de código de máquina de estado](https://trifectatech.org/blog/improving-state-machine-code-generation/)
* [video] [Cómo ganó Rust: la búsqueda de un software confiable y de alto rendimiento](https://www.youtube.com/watch?v=k_-6KI3m31M)
* [video] [¡Rust para todos!](https://www.youtube.com/watch?v=R0dP-QR5wQo)

### Tutoriales de Rust
* [Serie Axum Backend - Introducción](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-introduction/)

### Miscelánea
* [🦀 RustConf como ningún otro: un resumen de RustConf 2025](https://weeklyrust.substack.com/p/rustconf-like-no-other)

## Crate de la semana

El crate de esta semana es [GrimoireCSS](https://crates.io/crates/grimoire_css), un motor CSS creado en Rust,
centrándose en una flexibilidad inigualable, un estilo dinámico reutilizable y un rendimiento optimizado para cada entorno.

¡Gracias a [Dmitrii Shatokhin](https://users.rust-lang.org/t/crate-of-the-week/2704/1466) por la autosugestión!

[Por favor, envíe sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.

Si es un implementador de funciones y desea que su RFC aparezca en esta lista, agregue un
'llamada para pruebas' a su RFC junto con un comentario que proporcione instrucciones de prueba y / o
orientación sobre qué aspectos de la función necesitan ser probados.

* * No se emitieron llamadas para pruebas esta semana por
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing) o
  [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Háznoslo saber](https://github.com/rust-lang/this-week-in-rust/issues) si desea que se realice un seguimiento de su función como parte de esta lista.

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y / o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Convocatoria de participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quiso contribuir a proyectos de código abierto pero no sabía por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*No se enviaron convocatorias de participación esta semana.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

*No se enviaron convocatorias de artículos o presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 390 solicitudes de extracción en la última semana]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-09-02..2025-09-09

#### Compilador
* [corregir el alcance de caída para los enlaces 'super let' dentro de 'if let'](https://github.com/rust-lang/rust/pull/145342)
* [Estabilizar varargs de estilo C para sysv64, win64, efiapi, aapcs](https://github.com/rust-lang/rust/pull/144066)

#### Biblioteca
* [agregar cambios de bits exactos](https://github.com/rust-lang/rust/pull/144342)
* [constify impl Try for ControlFlow](https://github.com/rust-lang/rust/pull/146088)
* [corregir ruta str eq](https://github.com/rust-lang/rust/pull/146194)
* [búfer único para FMT de exponente de números enteros](https://github.com/rust-lang/rust/pull/145940)
* [estabilizar 'path_add_extension'](https://github.com/rust-lang/rust/pull/145209)
* [implementar rutinas de stdio específicas de WASIp2](https://github.com/rust-lang/rust/pull/146207)
* [comience a admitir WASIp2 de forma nativa](https://github.com/rust-lang/rust/pull/145944)

#### Carga
* [optimizar la carga con LTO](https://github.com/rust-lang/rust/pull/146253)
* ['fix(manifest)': Informar de errores de manifiesto de script para el número de línea correcto](https://github.com/rust-lang/cargo/pull/15927)
* [corrección: cambiar de --nocapture a --no-capture](https://github.com/rust-lang/cargo/pull/15930)
* [renderizar secciones de compilación individuales en el gráfico de canalización '--timings'](https://github.com/rust-lang/cargo/pull/15923)

#### Rustdoc
* [buscar: omitir la carga innecesaria de fnData](https://github.com/rust-lang/rust/pull/146070)
* [buscar: otro intento de optimización de stringdex](https://github.com/rust-lang/rust/pull/145911)

#### Clippy
* ['let_unit_with_type_underscore': hacer adelanto anticipado](https://github.com/rust-lang/rust-clippy/pull/15458)
* ['ptr_cast_constness': evitar sugerir una llamada a un método irresoluble](https://github.com/rust-lang/rust-clippy/pull/15540)
* [arreglar 'never_loop' olvidar eliminar 'break' en bucle anidado](https://github.com/rust-lang/rust-clippy/pull/15356)
* [arreglar 'read_zero_byte_vec' sugiere erróneamente dentro de 'let' stmt](https://github.com/rust-lang/rust-clippy/pull/15582)
* [preservar bloques 'inseguros' en la sugerencia 'option_map_unit'](https://github.com/rust-lang/rust-clippy/pull/15570)

#### Analizador de Rust
* [admite navegación en primitivas](https://github.com/rust-lang/rust-analyzer/pull/20632)
* [agregar la finalización de la palabra clave 'else' después de las declaraciones 'let'](https://github.com/rust-lang/rust-analyzer/pull/20620)
* [dar sentido al lío que eran (son) diferentes tipos de genéricos en el solucionador](https://github.com/rust-lang/rust-analyzer/pull/20586)
* [mejorar el espacio en blanco 'make::struct_ field_list'](https://github.com/rust-lang/rust-analyzer/pull/20626)
* [eliminar el soporte para 'register_attr'](https://github.com/rust-lang/rust-analyzer/pull/20631)

### Triaje de rendimiento del compilador de Rust

En general, una semana bastante neutral con relativamente pocos cambios que afecten a
aterrizaje de rendimiento.

Triaje realizado por **@simulacrum**.
Rango de revisión: [75ee9ffd.. F13EF0D7](https://perf.rust-lang.org/?start=75ee9ffd5ed3649c0a09493057adaa8feebb2035&end=f13ef0d75d834c826c9479a5d244bcfb9891df45&absolute=false&stat=instructions%3Au)

1 Regresión, 5 Mejoras, 3 Mixto; 4 de ellos en rollups
33 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/HEAD/triage/2025/2025-09-07.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* [proponer objetivos 2025h2](https://github.com/rust-lang/rfcs/pull/3849)

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Problema de seguimiento para AVX512_FP16 intrínsecos](https://github.com/rust-lang/rust/issues/127213)
* [Problema de seguimiento para nombres de archivo terminados en NUL con '#[track_caller]'](https://github.com/rust-lang/rust/issues/141727)
* [Estabilizar 'new_zeroed_alloc'](https://github.com/rust-lang/rust/pull/144091)

*Ningún artículo entró en el período de comentarios finales esta semana para
[RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) o
[Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos o actualizados esta semana.*

## Próximos eventos

Rusty Eventos entre 2025-09-10 - 2025-10-08 🦀

### Virtual
* 2025-09-11 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646019)
* 2025-09-11 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust/events/)
    * [**Reunión en línea de San Diego Rust de septiembre de 2025**](https://www.meetup.com/san-diego-rust/events/310326567)
* 2025-09-14 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/310002480)
* 2025-09-15 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [** Configuración de Tock OS en un entorno virtual (en línea) - preparación para el 17 de septiembre **](https://www.meetup.com/charlottesville-rust-meetup/events/310706165/)
* 2025-09-16 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Oxidado de mediados de mes**](https://www.meetup.com/rustdc/events/306757758)
* 2025-09-17 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731033)
* 2025-09-18 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/305646039/)
* 2025-09-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/305361443)
* 2025-09-25 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046637)
* 2025-10-01 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcnbcb)

### Asia
* 2025-09-13 | Hangzhou, CN | [WebAssembly y Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GOSIM AI Hangzhou 2025 (CFP aún está abierto)**](https://www.meetup.com/wasm-rust-meetup/events/309987624)
* 2025-09-13 - 2025-09-14 | Hangzhou, CN | [GOSIM](https://hangzhou2025.gosim.org/schedule/)
    * [**GOSIM Hangzhou 2025**](https://dev.events/conferences/rust-global-china-and-rust-china-conf-2025-dscrf0e1)
* 2025-09-17 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**En persona Rust septiembre de 2025 en Varonis en Herzeliya**](https://www.meetup.com/rust-tlv/events/310708628)
* 2025-10-02 | Seúl, KR | [Reunión de Seoul Rust (lenguaje de programación)](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Reunión de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/310824483)

### Europa
* 2025-09-10 | Colonia, DE | [Colonia de Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en septiembre: Rust atómico**](https://www.meetup.com/rustcologne/events/310858679)
* 2025-09-10 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944038)
* 2025-09-11 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #4 @Zühlke**](https://www.meetup.com/rust-bern/events/309903540)
* 2025-09-16 - 2025-09-18 | Berlín, DE | [Conferencia Oxidar](https://oxidizeconf.com/)
    * [**Conferencia Oxidize**](https://oxidizeconf.com/)
* 2025-09-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592250)
* 2025-09-17 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 09 2025**](https://lu.ma/ql3u6q5u)
* 2025-09-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de charlas en Mjolner Informatics**](https://www.meetup.com/rust-aarhus/events/310562343)
* 2025-09-23 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #78**](https://www.meetup.com/rust-paris/events/310935603)
* 2025-09-24 | Gotemburgo, SE | [Rust, Göteborg](https://www.meetup.com/rustgbg/events/)
    * [**Rust Gbg — septiembre de 2025**](https://www.meetup.com/rustgbg/events/310866773)
* 2025-09-24 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 3 - híbrido**](https://www.meetup.com/rust-munich/events/307105978)
* 2025-09-25 | Augsburgo, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Reunión de Augsburg Rust #15**](https://rust-augsburg.github.io/meetup/Meetup_15.html)
* 2025-10-01 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**4. Encuentro de Rust Moravia (¡En la capital!)**](https://www.meetup.com/rust-moravia/events/310743282)
* 2025-10-02 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310062134)
* 2025-10-08 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #79**](https://www.meetup.com/rust-paris/events/310424476)
* 2025-10-08 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944041)

### América del Norte
* 2025-09-10 | Phoenix, AZ, EE. UU. | [Rust del desierto](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> JS**](https://www.meetup.com/desert-rustaceans/events/310669989)
* 2025-09-11 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/310840020)
* 2025-09-11 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Laberintos y gráficos en Rust**](https://www.meetup.com/utah-rust/events/310674937)
* 2025-09-11 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Polars para análisis y manipulación de datos**](https://www.meetup.com/rust-mx/events/310408223)
* 2025-09-14 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, 14 de septiembre **](https://www.meetup.com/bostonrust/events/310106317)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308284339)
* 2025-09-16 | San Francisco, CA, EE. UU. | [Red Vara](https://lu.ma/events-by-vara-gear)
    * [**Taller de Rust de Vara Network**](https://luma.com/1bii0kv7)
* 2025-09-18 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Serie 3 : Marcador de posición**](https://www.meetup.com/music-city-rust-developers/events/304333261)
* 2025-09-18 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Septiembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust) **](https://www.meetup.com/seattle-rust-user-group/events/308677324)
* 2025-09-24 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310287849)
* 2025-09-24 | Charlottesville, VA, EE. UU. | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tick, Tock, talk: descubre cómo Rust protege los dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/310603587)
* 2025-09-25 | Atlanta, GA, EE. UU. | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl en la taberna Manuels**](https://www.meetup.com/rust-atl/events/308675983)
* 2025-10-02 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [** 🚁 Rust en vuelo: lecciones del diseño de un cuadricóptero impreso en 3D con incrustación**](https://www.meetup.com/stl-rust/events/310279407)

### Oceanía:
* 2025-10-01 | Perth, WA, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**Reunión de octubre**](https://www.meetup.com/perth-rust-meetup-group/events/310847099)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puede leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1mnpd9p/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Hola,
>
> Lamentamos que no estés contento con el [estado](https://corrode.dev/blog/async/) de la 'asíncrona' en la edición actual de Rust. La intuición de propiedad de la memoria que se suponía que debía desarrollar al trabajar con ejecución de un solo subproceso y/o paralela resultó ser demasiado costosa para portarla a nuestro marco de concurrencia de costo cero, reinventado desde [cero](https://doc.rust-lang.org/std/pin/index.html) para el beneficio final de nadie en particular.
>
> No planeamos hacer nada al respecto.
>
> Soporte asíncrono de Rust - Departamento Internacional

– [00100011 sobre los usuarios de Rust](https://users.rust-lang.org/t/borrow-of-owned-sync-type-in-async-function/133667/2)

¡Gracias a [Aleksander Krauze](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1714) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](REDDIT_LINK_HERE)</small>
