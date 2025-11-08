---
title: "Esta semana en Rust #83"
number_of_week: 83
description: El crate de esta semana es dioxus, un marco para crear aplicaciones multiplataforma.
date: 2025-11-05
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
* [Objetivos del proyecto para 2025H2 | Blog de Rust](https://blog.rust-lang.org/2025/10/28/project-goals-2025h2/)
* [Anunciando Rust 1.91.0 | Blog de Rust](https://blog.rust-lang.org/2025/10/30/Rust-1.91.0/)

### Fundación
* [Anuncio del Fondo de Mantenedores de la Fundación Rust - La Fundación Rust](https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/)

### Boletines
* [Rust Trends Issue #71: Producción de Rust a escala de Internet](https://rust-trends.com/newsletter/production-rust-internet-scale)
* [Este mes en Redox - Octubre 2025 - Redox - Su próximo sistema operativo (Gen)](https://www.redox-os.org/news/this-month-251031/)

### Actualizaciones de proyectos/herramientas
* [Desarrollo de UEFI en Rust with Patina](https://opendevicepartnership.github.io/patina/introduction.html)
* [Anuncio de cgp-serde: una biblioteca de serialización modular para Serde impulsada por CGP](https://contextgeneric.dev/blog/cgp-serde-release/)
* [Lanzamiento de CGP v0.6.0 - Importantes mejoras ergonómicas para implementaciones de proveedores y contextos](https://contextgeneric.dev/blog/v0-6-0-release/)
* [anuncio de lanzamiento de 'esp-hal' 1.0.0](https://developer.espressif.com/blog/2025/10/esp-hal-1/)

### Observaciones/Pensamientos
* [Fantasmas en la compilación](https://predr.ag/blog/ghosts-in-the-compilation/)
* [Patrones para la programación defensiva en Rust | corrode Rust Consulting](https://corrode.dev/blog/defensive-programming/)
* [Cloudflare con Edward Wang y Kevin Guthrie - Rust en producción](https://corrode.dev/podcast/s05e03-cloudflare/)
* [Redes neuronales con vela](https://pranitha.dev/posts/neural-networks-with-candle/)
* [Rust asíncrono - Parte 18 de Rust idiomático en pasos simples](https://www.youtube.com/watch?v=_x61dSP4ZKM)
* [El estado de SIMD en Rust en 2025](https://shnatsel.medium.com/the-state-of-simd-in-rust-in-2025-32c263e5f53d)
* [Rust se está comiendo el mundo: desde firmware integrado hasta aplicaciones multiplataforma, bases de datos y grandes servidores](https://kerkour.com/rust-is-eating-the-world)
* [video] [Construyendo sistemas ferroviarios de próxima generación con Rust: Tom Praderio de Parallel](https://www.youtube.com/watch?v=Ga2YDNDHQsM)
* [video] [¿Ya estamos en el escritorio? - Victoria Brekenfeld | EuroRust 2025](https://www.youtube.com/watch?v=0ZrD2oYabn4)
* [audio] [Netstack.FM Episodio 12 - Oxide Networking con Ryan Goodfellow](https://netstack.fm/#episode-12)

### Tutoriales de Rust
* [Dobles de prueba de la unidad de Rust: falsificaciones](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_fake/)
* [Creación de un agente de codificación en Rust: implementación de la función de chat](https://blog.0xshadow.dev/posts/coding-agent-in-rust/coding-agent-in-rust-chat/)
* [Clasificación de imágenes en Rust con Tch-rs (encuadernaciones de antorcha)](https://www.djamware.com/post/690864cde87a290bcfebeebe/image-classification-in-rust-with-tchrs-torch-bindings)
* [Dentro de las exclusiones estáticas y parking_lot de Rust, ¿quién gana?](https://blog.cuongle.dev/p/inside-rusts-std-and-parking-lot-mutexes-who-win)
* [Positrón - Solo el futuro es seguro](https://positron.solutions/articles/building-nicely-with-rust-and-nix)
* [Introducción a Rust y ClickHouse](https://www.svix.com/blog/getting-started-with-rust-and-clickhouse/)
* ['SocketAddrV6' no es serializable de ida y vuelta · lluvias solares](https://sunshowers.io/posts/socketaddrv6-not-roundtrip/)
* [Construyendo sistemas ferroviarios de próxima generación con Rust: Tom Praderio de Parallel](https://filtra.io/rust/interviews/parallel-nov-25)
* [Diapositivas del taller de diésel de RustWeek2025](https://blog.weiznich.de/rustweek_2025.html#/title-slide)
* [video] [Agente de codificación de construcción en Rust | Implementar CLI de chat | Parte 2](https://www.youtube.com/watch?v=N21aCBICHLU)

### Miscelánea
* [No puedo parar hasta que tengas suficiente](https://cant.bearblog.dev/can-t-stop-till-you-get-enough/)

## Crate de la semana

El crate de esta semana es [dioxus](https://docs.rs/dioxus), un marco para crear aplicaciones multiplataforma.

¡Gracias a [llogiq](https://users.rust-lang.org/t/crate-of-the-week/2704/1484) por la sugerencia!

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
  [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [RFC del lenguaje Rust](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) o
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

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL del problema) -->
* [Motor OS - Mejorar la prisa (el caparazón en Motor OS)](https://github.com/moturus/motor-os/issues/33)
* [Motor OS - Hacer que el generador de imágenes sea configurable](https://github.com/moturus/motor-os/issues/24)
* [Motor OS - Puerto libc/llvm/rustc](https://github.com/moturus/motor-os/issues/26)
* [Diésel - Mejorar la documentación para los modos de carga de Postgres](https://github.com/diesel-rs/diesel/issues/4764)
<!-- o si no hay ninguna, *No se enviaron convocatorias de participación esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándote con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust).

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre de CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno, *No se enviaron convocatorias de artículos o presentaciones esta semana.* -->

* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP cierra 2025-12-08 | Portland, Oregón, Estados Unidos | 2026-04-20

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

480 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-28..2025-11-04

#### Compilador
* ['rustc_codegen': se corrigen los rendimientos de musttail para ABI de reparto/indirectos](https://github.com/rust-lang/rust/pull/148240)
* [aceptar consts triviales basados en consts triviales](https://github.com/rust-lang/rust/pull/148182)
* [agregar atributos de rango LLVM a los parámetros de longitud de corte](https://github.com/rust-lang/rust/pull/148350)
* [ajustar iteradores sucesores](https://github.com/rust-lang/rust/pull/148157)
* [permitir compilaciones de verificación con binarios para el backend ficticio de codegen](https://github.com/rust-lang/rust/pull/148299)
* [permitir que los backends de codegen indiquen qué tipos de cajas admiten](https://github.com/rust-lang/rust/pull/148177)
* [mejor mensaje de advertencia para el tipo de caja no compatible con el backend de codegen](https://github.com/rust-lang/rust/pull/148400)
* [declaraciones de variables de contrato](https://github.com/rust-lang/rust/pull/144444)
* [corregir comprobaciones de fundición diferidas que usan el cuerpo incorrecto para determinar la constecencia](https://github.com/rust-lang/rust/pull/148287)
* [corregir los tipos que se marcan como muertos cuando se infieren argumentos genéricos](https://github.com/rust-lang/rust/pull/148262)
* [implementar pin-project en la coincidencia de patrones para '&pin mut|const T'](https://github.com/rust-lang/rust/pull/139751)
* [correcciones relacionadas con genéricos comunes](https://github.com/rust-lang/rust/pull/147642)
* [eliminar 'QPath::LangItem'](https://github.com/rust-lang/rust/pull/148193)
* [estabilizar -Zno-jump-tables en -Cjump-tables=bool](https://github.com/rust-lang/rust/pull/145974)
* [cuando no se implementa un rasgo, pero se encuentra otro impacto similar, apúntalo](https://github.com/rust-lang/rust/pull/145640)

#### Biblioteca
* [agregue 'from_fn_ptr' a 'Waker' y 'LocalWaker'](https://github.com/rust-lang/rust/pull/146057)
* [agregar los tipos de envoltura SliceIndex Last y 'Clamp<Idx>'](https://github.com/rust-lang/rust/pull/146260)
* [constificar funciones de rango](https://github.com/rust-lang/rust/pull/146573)
* [constificar alias de rasgos](https://github.com/rust-lang/rust/pull/144291)
* [implementar VecDeque 'extend_from_within' y 'prepend_from_within'](https://github.com/rust-lang/rust/pull/147161)
* [implementar 'VecDeque::extract_if'](https://github.com/rust-lang/rust/pull/147780)
* [implementar la función de biblioteca 'strip_circumfix'](https://github.com/rust-lang/rust/pull/147947)
* [puntero inteligente '(try_)map'](https://github.com/rust-lang/rust/pull/144420)
* [estabilizar 'fmt::from_fn'](https://github.com/rust-lang/rust/pull/145915)

#### Carga
* ['build-analysis': infraestructura de registro basada en JSONL](https://github.com/rust-lang/cargo/pull/16150)
* ['build-analysis': emit timing-info log](https://github.com/rust-lang/cargo/pull/16179)
* ['config-include': agregar soporte de campo opcional](https://github.com/rust-lang/cargo/pull/16180)
* ['config-include': soporte en línea y matriz de tablas](https://github.com/rust-lang/cargo/pull/16174)
* [admite matriz de cualquier tipo en la configuración de Cargo](https://github.com/rust-lang/cargo/pull/16103)

#### Rustdoc
* [buscar: Incluir cajas externas al filtrar por 'importar'](https://github.com/rust-lang/rust/pull/148301)
* [Incluir atributos y derivar macros al filtrar por "macros"](https://github.com/rust-lang/rust/pull/148176)
* [use modificadores de destino configurados al recopilar doctests](https://github.com/rust-lang/rust/pull/148068)

#### Clippy
* ['search_is_some': Se corrige cuando el cierre abarca varias líneas](https://github.com/rust-lang/rust-clippy/pull/15902)
* ['double_parens': no pelar en proc-macros](https://github.com/rust-lang/rust-clippy/pull/15939)
* ['let_and_return': no permitir '_cualquier_' texto entre let y return](https://github.com/rust-lang/rust-clippy/pull/16006)
* ['use_debug': no se confunda con las implicaciones anidadas de 'Depuración'](https://github.com/rust-lang/rust-clippy/pull/15946)
* ['incompatible_msrv': No compruebe el const MSRV en busca de funciones no llamadas](https://github.com/rust-lang/rust-clippy/pull/15795)
* ['manual_unwrap_or(_default)': no pelusa si no es seguro mover el escrutador](https://github.com/rust-lang/rust-clippy/pull/15817)
* [extender 'needless_collect'](https://github.com/rust-lang/rust-clippy/pull/14361)
* [corregir 'replace_box' falso positivo cuando se mueve la caja](https://github.com/rust-lang/rust-clippy/pull/15984)
* [mejorar el análisis de etiquetas de idioma del código de comentario del documento, no use un analizador completo](https://github.com/rust-lang/rust-clippy/pull/15967)

#### Analizador de Rust
* [añadir ide-assist: 'convert_range_for_to_while'](https://github.com/rust-lang/rust-analyzer/pull/20565)
* [admite perfiles de memoria con dhat](https://github.com/rust-lang/rust-analyzer/pull/20927)
* [corregir otros elementos asociados faltantes para 'generate_blanket_trait_impl'](https://github.com/rust-lang/rust-analyzer/pull/20957)
* [la solución no se aplica en while para 'replace_is_method_with_if_let_method'](https://github.com/rust-lang/rust-analyzer/pull/20915)
* [canonicalizar las rutas 'custom-target.json' al obtener metadatos de sysroot](https://github.com/rust-lang/rust-analyzer/pull/20964)
* [considere más tipos de expresión como 'in_value'](https://github.com/rust-lang/rust-analyzer/pull/20961)
* [expandir literales con sufijos incorrectos en 'LitKind::Err'](https://github.com/rust-lang/rust-analyzer/pull/20963)
* [errores de sintaxis falsos positivos en frontmatter](https://github.com/rust-lang/rust-analyzer/pull/20942)
* [arreglar el manejo de módulos de bloques que no son el módulo raíz](https://github.com/rust-lang/rust-analyzer/pull/20930)
* [mejorar la recuperación de errores al analizar tipos de retorno de funciones mal formados](https://github.com/rust-lang/rust-analyzer/pull/20934)
* [soporte adecuado opacos](https://github.com/rust-lang/rust-analyzer/pull/20906)
* [resolver 'target-dir' con mayor precisión](https://github.com/rust-lang/rust-analyzer/pull/20920)
* [mostrar las firmas de función asíncronas adecuadas en la ayuda de firmas](https://github.com/rust-lang/rust-analyzer/pull/20931)

### Triaje de rendimiento del compilador de Rust

Semana mayormente positiva. Vimos una gran ganancia de rendimiento implementada por [#148040](https://github.com/rust-lang/rust/pull/148040) y [#148182](https://github.com/rust-lang/rust/pull/148182), que optimiza las cajas con muchas constantes triviales.

Triaje realizado por **@kobzol**.

Rango de revisión: [23fced0f.. 35ebdf9b](https://perf.rust-lang.org/?start=23fced0fcc5e0ec260d25f04a8b78b269e5e90f0&end=35ebdf9ba1414456dfe1cb6a6b13ebae80e99734&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:U) | media | Gama | recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,8% | [0,1%, 2,9%] | 22 |
| Regresiones ❌ <br /> (secundaria) | 0,5% | [0,1%, 1,7%] | 48 |
| Mejoras ✅ <br /> (primaria) | -2,8% | [-16,4%, -0,1%] | 102 |
| Mejoras ✅ <br /> (secundario) | -1,9% | [-8,0%, -0,1%] | 51 |
| Todos ❌✅ (primarios) | -2,1% | [-16,4%, 2,9%] | 124 |

4 regresiones, 6 mejoras, 7 mixtas; 7 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/057eaab3021d6bc301bba06b69e7e1cfdb4f9c3d/triage/2025/2025-11-03.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que fueron aprobados para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período de comentarios finales

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "período de comentarios finales" para RFC y PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Error de emisión al usar la palabra clave path-segment como cfg pred](https://github.com/rust-lang/rust/pull/146978)
* [estabilizar extern_system_varargs](https://github.com/rust-lang/rust/pull/145954)
* [Problema de seguimiento para 'vec_into_raw_parts'](https://github.com/rust-lang/rust/issues/65816)
* [rustdoc: Borrar '#! [doc(document_private_items)]'](https://github.com/rust-lang/rust/pull/146495)
* [Agregar nueva pelusa 'function_casts_as_integer'](https://github.com/rust-lang/rust/pull/141470)
* [resolver: Preservar las reexportaciones ambiguas de globos en los metadatos de la caja](https://github.com/rust-lang/rust/pull/147984)
* [Hacer que deref_nullptr deniegue por defecto en lugar de advertir](https://github.com/rust-lang/rust/pull/148122)
* [Problema de seguimiento para 'const_mul_add'](https://github.com/rust-lang/rust/issues/146724)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(Solo MCP)](https://forge.rust-lang.org/compiler/mcp.html)

* [eliminar el soporte para 'typeof'](https://github.com/rust-lang/compiler-team/issues/940)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Garantizar la representación binaria de 'isize' explícitamente](https://github.com/rust-lang/reference/pull/2064)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)

* [Propuesta: Requerir que todos los miembros del equipo del proyecto tengan identificaciones de Zulip](https://github.com/rust-lang/leadership-council/issues/228)

*Ningún artículo entró en el período de comentarios finales esta semana para
  [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [RFC de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Equipo de idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
  [Pautas de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háganos saber si desea que se realice un seguimiento de sus PR, problemas de seguimiento o RFC como parte de esta lista.

#### [RFC nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [build-std: dependencias explícitas](https://github.com/rust-lang/rfcs/pull/3875)
* [build-std: siempre](https://github.com/rust-lang/rfcs/pull/3874)
* [build-std: contexto](https://github.com/rust-lang/rfcs/pull/3873)

## Próximos eventos

Rusty Eventos entre 2025-11-05 - 2025-12-03 🦀

### Virtual
* 2025-11-05 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Dominando el manejo de errores en Rust: De los pánicos a este error y de todos modos**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-09 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Recuperación de la comunidad**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-12 | Virtual (Boulder, CO, EE. UU.) | [Elixir de roca](https://www.meetup.com/boulder-elixir/events/)
    * [**Integración de Elixir y Apache DataFusion con Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Núremberg, DE) | [Rust de Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Rustful de mediados de mes**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Estudio de Rust/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Virtual (Berlín, Alemania) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tock, un sistema operativo basado en Rust Parte #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de los lectores de Rust: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Sistemas de uso intensivo de datos en Rust: seguridad, velocidad, simultaneidad**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-30 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Discusión de Discord de Rust Readers: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (Londres, GB) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Advenimiento del código - ¡Inicio!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Búfalo, Nueva York, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de roya de búfalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Sesión de codificación semanal**](https://luma.com/yf2t878c)
* 2025-12-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyhcqbfb/)

### África
* 2025-11-11 | Johannesburgo, ZA | [Reunión de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Más sobre los tipos de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Reunión de Rustacean de noviembre de 2025**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europa
* 2025-11-05 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 1 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784113/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicio Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Colonia, DE | [Colonia de Rust](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in November: Small Crates Cult**](https://www.meetup.com/rustcologne/events/311767026/)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Especial de Halloween.**](https://www.meetup.com/oxford-rust-meetup-group/events/311569796/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Encuentro de Rust Gdansk #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-06 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/events/)
    * [**Inicio de la temporada 2 | en metalab 🦀 **](https://www.meetup.com/rust-vienna/events/311679397/)
* 2025-11-07 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**Conferencia TechMeetup Ostrava**](https://www.meetup.com/techmeetupostrava/events/306776025/)
* 2025-11-11 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London x Zed Meetup**](https://www.meetup.com/rust-london-user-group/events/311737021/)
* 2025-11-11 | Estocolmo, SE | [Func Prog Suecia](https://www.meetup.com/func-prog-sweden/events/)
    * [**Func Prog Sweden 2025 en Kivra**](https://www.meetup.com/func-prog-sweden/events/308526518/)
* 2025-11-12 | Cambridge, Reino Unido | [Reunión de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Reunión mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de Reading Rust**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Ginebra, CH | [Rust Ginebra](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Ginebra**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Reunión de Rust #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-14 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Encuentro de Rust @Magello**](https://www.meetup.com/stockholm-rust/events/311844163/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-19 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/events/)
    * [**Círculo de control de calidad**](https://www.meetup.com/techmeetupostrava/events/311581090/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche social**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Ámsterdam, Países Bajos | [Grupo de Desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ Monumental X Zed**](https://www.meetup.com/rust-amsterdam-group/events/311829267/)
* 2025-11-20 | Lucerna, CH | [Rust de Lucerna]((https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)
* 2025-11-26 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/events/)
    * [**Workshop c# - nr 2 av 2 - grunnleggende nivå**](https://www.meetup.com/bergen-html-css-meetup-group/events/311784127/)
* 2025-11-26 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**19ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Charlas de noviembre: Tipos de tamaño exótico y ...**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Praga @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Oxford, Reino Unido | [Encuentro de Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Reunión de Rust/ACCU.**](https://www.meetup.com/oxford-rust-meetup-group/events/nnrkttyhcqbfb/)

### América del Norte
* 2025-11-06 | Montreal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**Noviembre Mensual Social**](https://www.meetup.com/rust-montreal/events/311762040/)
* 2025-11-06 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Sorpresa**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, 8 de noviembre **](https://www.meetup.com/bostonrust/events/311039501/)
* 2025-11-13 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Creación de aplicaciones de escritorio con Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-18 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)
* 2025-11-20 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Noviembre de 2025 Reunión de SRUG (Grupo de usuarios de Seattle Rust)**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-20 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/events/)
    * [**Reunión mensual de Rust: noviembre**](https://www.meetup.com/spokane-rust/events/311863560/)
* 2025-11-26 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo de Rust - Terreno de destino**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-12-02 | Chicago, IL, EE. UU. | [Reunión de Chicago Rust](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)

### Oceanía
* 2025-11-11 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Encuentro de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/311685331/)

Si está organizando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, vea el último hilo [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Si alguien abre un PR que introduce C++ en su proyecto de Rust, ese código es libre como en "usar después"

– [Predrag Gruevski sobre Mastodon](https://hachyderm.io/@predrag/115475147692306391)

¡Gracias a [Brett Witty](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1726) por la sugerencia!

[¡Por favor, envíe cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust es editado por:

* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [MarianneGoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

* El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir sobre r/rust](https://www.reddit.com/r/rust/comments/1optr5g/this_week_in_rust_624/)</small>
