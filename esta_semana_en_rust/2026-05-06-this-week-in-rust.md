---
title: "Esta semana en Rust #108"
number_of_week: 108
description: El crate de esta semana es burn, una biblioteca de tensor y aprendizaje profundo.
date: 2026-05-06
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

### Oficial
* [Anunciando proyectos seleccionados de Google Summer of Code 2026](https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/)

### Boletines
* [Rust Trends Número 77 - El Rust afila el oficio](https://rust-trends.com/newsletter/rust-sharpens-the-craft/)

### Actualizaciones de proyectos/herramientas
* [Imgclip: Una CLI multiplataforma para conversión de archivos de imagen en portapapeles ↔](https://dev.to/alex_yan_6135f8195a1a3b01/imgclip-a-cross-platform-cli-for-clipboard-image-file-conversion-2i1l)
* [Conectores: Donde AimDB se encuentra con el mundo real](https://aimdb.dev/blog/connectors-where-aimdb-meets-the-real-world)
* [rkik-nts 1.0.0: una biblioteca cliente de alto nivel de Rust Network Time Security (RFC 8915)](https://github.com/aguacero7/rkik-nts/releases/tag/v1.0.0)
* [unix-ancillary 0.2.2 — seguro SCM_RIGHTS fd-passing para Rust](https://github.com/MohibShaikh/unix-ancillary/releases/tag/v0.2.2)
* [kache 0.2.0: caché de construcción de Rust con copia cero y dirección de contenido (RUSTC_WRAPPER)](https://github.com/kunobi-ninja/kache/releases/tag/v0.2.0)
* [Fileman](https://kvark.github.io/application/2026/03/14/fileman.html) - un gestor de archivos multiplataforma de 2 paneles

### Observaciones/Pensamientos
* [Una semana de view_types](https://scrabsha.dev/articles/one-week-of-view-types.html)
* [Async Rust nunca salió del estado MVP](https://tweedegolf.nl/en/blog/237/async-rust-never-left-the-mvp-state)
* [especialización estable en Rust](https://goldstein.lol/posts/stable-specialization/)
* [Tu configuración de Clippy debería ser más estricta](https://emschwartz.me/your-clippy-config-should-be-stricter/)
* [tu configuración de clippy debería ser más estricta](https://billylevin.dev/posts/clippy-config/)
* [El encadenado 'Sync' que nadie pidió](https://verrchu.github.io/blog/1-the-sync-bound-nobody-asked-for/)
* [Cross-platform Rust: Analizando cómo WhatsApp, Signal y más están lanzando Rust a miles de millones de dispositivos](https://kerkour.com/rust-cross-platform-apps)
* [audio] [Netstack.FM episodio 37 — Dial9: De la caja negra a la visión en Tokio](https://netstack.fm/#episode-37)

### Guías de Rust
* [¡ups, macro cúbico!](https://bal-e.org/blog/2026/oops-cubic-macro/)
* [vídeo] [RustCurious lección 7: Arreglos y Cortes](https://www.youtube.com/watch?v=JWfVqDEkQQw)
* [Escribiendo medios para funciones Rust Lambda](https://loige.co/writing-middlewares-for-rust-lambda-functions/)
* [Aprende a manejar errores en Rust construyendo un analizador de configuración TOML](https://blog.sheerluck.dev/posts/learn-error-hanlding-in-rust/)

### Miscelánea
* [Increíbles recursos SQLx](https://github.com/szabgab/awesome-sqlx)

## Crate de la semana

El crate de esta semana es [burn](https://github.com/tracel-ai/burn), una biblioteca de tensor y aprendizaje profundo.

¡Gracias a [Jonas](https://users.rust-lang.org/t/crate-of-the-week/2704/1604) por la sugerencia!

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
*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**Computación Científica en Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

504 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-28..2026-05-05

#### Compilador
* [canonizar regiones libres de entradas como marcadores de posición en la raíz univ](https://github.com/rust-lang/rust/pull/155487)

#### Biblioteca
* [no recargue la duración en 'String::p ush'](https://github.com/rust-lang/rust/pull/155847)

#### Carga
* ['feat(lints)': Añadir 'text_direction_codepoint' lints por defecto](https://github.com/rust-lang/cargo/pull/16950)
* ['fix(compile)': Siempre que sea posible, insinúa sobre deps fuera de lugar](https://github.com/rust-lang/cargo/pull/16940)
* [''fix(config): [env]' definición de rutas relativas](https://github.com/rust-lang/cargo/pull/16957)
* ['fix(config)': normalizar rutas de configuración incluidas](https://github.com/rust-lang/cargo/pull/16964)
* [eliminar la dependencia de curl de crates-io crate](https://github.com/rust-lang/cargo/pull/16936)

#### Rustdoc
* [función de corregir 'doc_cfg' en reexportaciones](https://github.com/rust-lang/rust/pull/156073)
* [preservar el cfg del documento padre para macros 'macro_export'](https://github.com/rust-lang/rust/pull/155954)

#### Clippy
* [añadir una verificación para algunos seguida de filtro](https://github.com/rust-lang/rust-clippy/pull/15745)
* [arreglar 'bad_bit_mask' ICE para operaciones de bits sobrecargadas](https://github.com/rust-lang/rust-clippy/pull/16937)
* ['needless_return_with_question_mark' activación en funciones asincrónicas](https://github.com/rust-lang/rust-clippy/pull/16952)

#### Analizador de Rust
* ['diagnósticos': añadir manejador para E0130](https://github.com/rust-lang/rust-analyzer/pull/22197)
* [añadir variante del editor 'add_item' de AssocItemList](https://github.com/rust-lang/rust-analyzer/pull/22245)
* [importar globo expandido al fallar la importación cíclica](https://github.com/rust-lang/rust-analyzer/pull/22244)
* [añadir diagnóstico para E0784](https://github.com/rust-lang/rust-analyzer/pull/22202)
* [permitir cambiar de nombre de vidas elididas](https://github.com/rust-lang/rust-analyzer/pull/22178)
* [diagnosticar errores 🎉 de rasgo](https://github.com/rust-lang/rust-analyzer/pull/22186)
* [emitir un diagnóstico para 'non_exhaustive struct' cuando se construye](https://github.com/rust-lang/rust-analyzer/pull/22193)
* [oferta en si-expr con else-if para 'convert_to_guarded_return'](https://github.com/rust-lang/rust-analyzer/pull/22199)
* [soporte if-else en valor en completaciones de posfijo](https://github.com/rust-lang/rust-analyzer/pull/22222)
* [añadir exprs faltantes a visitantes](https://github.com/rust-lang/rust-analyzer/pull/22214)
* [añadir elementos de idioma del solver faltantes](https://github.com/rust-lang/rust-analyzer/pull/22274)
* [añade punto y coma tras expr en stmt para 'unwrap_branch'](https://github.com/rust-lang/rust-analyzer/pull/22217)
* [atrapa '#[rustc_reservation_impl = "razón"]'](https://github.com/rust-lang/rust-analyzer/pull/22282)
* [no buscar diagnósticos hasta que se carguen los proc-macros](https://github.com/rust-lang/rust-analyzer/pull/22272)
* [no te pongas nervioso en 'impl ? Talla para 'introduce_named_type_parameter'](https://github.com/rust-lang/rust-analyzer/pull/22265)
* [corregir 'unwrap_branch' en 'match_arm'](https://github.com/rust-lang/rust-analyzer/pull/22247)
* [corrijo desbordamiento de pila en la pantalla de proyección](https://github.com/rust-lang/rust-analyzer/pull/22215)
* [asa vacía expr en tupla expr](https://github.com/rust-lang/rust-analyzer/pull/22201)
* [mejora 'prettify_macro_expansion()'](https://github.com/rust-lang/rust-analyzer/pull/22058)
* [mejorar los espacios en blanco para completar el ítem de rasgo](https://github.com/rust-lang/rust-analyzer/pull/22240)
* [infierir el tipo esperado como el tipo de retorno para bloques asíncronos definidos por fns asíncronos](https://github.com/rust-lang/rust-analyzer/pull/22275)
* [matriz de puertos y ref exprs inferencia de rustc](https://github.com/rust-lang/rust-analyzer/pull/22271)
* [calificar .nueva ruta y no completar parámetros genéricos](https://github.com/rust-lang/rust-analyzer/pull/22210)
* [eliminar el uso de 'references_error()' en la inferencia de UPVAR](https://github.com/rust-lang/rust-analyzer/pull/22276)
* [mostrar el mensaje del usuario para '#[must_use]'](https://github.com/rust-lang/rust-analyzer/pull/22253)
* [usar 'Pattern_White_Space' para el manejo de espacios en blanco](https://github.com/rust-lang/rust-analyzer/pull/22008)
* [varias correcciones para 'lower_coroutine_body_with_moved_arguments()'](https://github.com/rust-lang/rust-analyzer/pull/22207)
* [envuelven el nivel superior o patrones en paréntesis en 'convert_match_to_let_else'](https://github.com/rust-lang/rust-analyzer/pull/22229)
* [hir-ty: emitir diagnóstico para valores no utilizados de '#[must_use]](https://github.com/rust-lang/rust-analyzer/pull/22239)
* [IDE-diagnóstico: emitir error para duplicar campo en la expresión de registro](https://github.com/rust-lang/rust-analyzer/pull/22235)
* [IDE-diagnóstico: error de emisión por longitud de patrón de array desajustada](https://github.com/rust-lang/rust-analyzer/pull/22238)
* [migrar función de generación a SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/22159)
* [PERF: almacena en caché más cosas relacionadas con objetos lang (rasgos de paréntesis, tipos/funciones asociadas a hijos/hermanos) pero que no son objetos lang en sí mismos](https://github.com/rust-lang/rust-analyzer/pull/22280)
* [interpretación: no hacer prácticas en 'AdtDef'](https://github.com/rust-lang/rust-analyzer/pull/22187)
* [perf: mejora el rendimiento de símbolos basados en enteros](https://github.com/rust-lang/rust-analyzer/pull/22267)
* [eliminar añadir predicado para sintaxis Where (https://github.com/rust-lang/rust-analyzer/pull/22246)
* [eliminar un método no utilizado en 'edit_in_place'](https://github.com/rust-lang/rust-analyzer/pull/22242)
* [sustituye insert use e insert use as alias por su variante de editor](https://github.com/rust-lang/rust-analyzer/pull/22241)
* [usar syntaxFactory en arg genérico en lugar de make original](https://github.com/rust-lang/rust-analyzer/pull/22243)

### Triaje de rendimiento del compilador Rust

El resultado de esta semana es bastante neutral. Parece negativo en números de iconteo, pero eso es falso, el tiempo de pared se mantuvo prácticamente sin cambios. Algunas mejoras importantes de rendimiento llegaron al nuevo solver, que aún no está activado por defecto.

Triaje hecho por **@panstromek**.
Rango de revisión: [ca9a134e.. 1d72d7e8](https://perf.rust-lang.org/?start=ca9a134e0985765ded9cfdde4030a5df4db7e2bd&end=1d72d7e8136faaebad3a85eeed432e6ea1b2ffab&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| ¿Regresiones? <br /> (primaria) | 0,6% | [0,2%, 1,2%] | 106 |
| ¿Regresiones? <br /> (secundario) | 0,7% | [0,2%, 2,4%] | 67 |
| ¿Mejoras? <br /> (primaria) | -0,6% | [-1,7%, -0,2%] | 66 |
| ¿Mejoras? <br /> (secundario) | -0,6% | [-2,8%, -0,0%] | 60 |
| ¿Todos?? (primaria) | 0,1% | [-1,7%, 1,2%] | 172 |

1 regresión, 2 mejoras, 9 mixtas; 5 de ellos en rollups
34 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/477a72d1755b1b8adb3c4b7eef2ed34e0c954de7/triage/2026/2026-05-05.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Hacer que las referencias de rasgos y las rutas asociadas de ty inducan correctamente los valores predeterminados de por vida útil de los objetos de rasgo](https://github.com/rust-lang/rust/pull/129543)
* [validar los parámetros '#[link_name = "..."]' & '#[link(name = "...")]](https://github.com/rust-lang/rust/pull/155817)
* [Mejorar la precisión de las operaciones de flotación de duración](https://github.com/rust-lang/rust/pull/150933)
* [Problema de seguimiento para 'unsafe_cell_access'](https://github.com/rust-lang/rust/issues/136327)
* [Problema de seguimiento para producir un 'Resultado<(), E>' a partir de un 'bool'](https://github.com/rust-lang/rust/issues/142748)
* [Permitir acortar la vida útil en CoerceUnsized for &mut](https://github.com/rust-lang/rust/pull/149219)
* [Asegúrate de que el envío/sincronización no esté implementado para std::env::Vars{,Os}](https://github.com/rust-lang/rust/pull/155153)
* [dote (rustdoc): estabilizar bandera '--emitir'](https://github.com/rust-lang/rust/pull/146220)
* [Haz 'Infalible = !'](https://github.com/rust-lang/rust/issues/155924)
* [Añadir lint contra definiciones inválidas de símbolos en tiempo de ejecución](https://github.com/rust-lang/rust/pull/155521)
* [error en 'export_name' vacío](https://github.com/rust-lang/rust/pull/155515)
* [Comprobar argumentos de atributos donde no se esperan argumentos](https://github.com/rust-lang/rust/pull/155193)
* [estabilizar 'función(cfg_target_has_atomic_equal_alignment)'](https://github.com/rust-lang/rust/pull/155006)
* [corrección: corregir el comportamiento de captura de 'si dejar' en cierres](https://github.com/rust-lang/rust/pull/154210)
* [Resolver: Resolución de importación por lotes](https://github.com/rust-lang/rust/pull/145108)
* [Asegurar enviar/sincronizar impl para std::p rocess::CommandArgs](https://github.com/rust-lang/rust/pull/155113)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Convertir opciones -C obsoletas en errores](https://github.com/rust-lang/compiler-team/issues/978)
* [Promocionar loongarch32-desconocido-ninguno* a Nivel 2](https://github.com/rust-lang/compiler-team/issues/968)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Propone el concepto de un nombre de usuario crates.io para la identidad](https://github.com/rust-lang/rfcs/pull/3946)

##### [Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Revisar el proceso de decisión: decisiones campeón vs FCP](https://github.com/rust-lang/lang-team/pull/360)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC inicial de matemáticas Rustdoc LaTeX](https://github.com/rust-lang/rfcs/pull/3958)
* [Política LLM a nivel de proyecto](https://github.com/rust-lang/rfcs/pull/3959)

## Próximos eventos

Eventos Rusty entre el 06-05-2026 - el 03-06-2026 🦀

### Virtual
* 2026-05-06 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Introducción práctica a SIMD**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314301861/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/314323890/)
* 2026-05-07 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprendiendo Rust por las malas: Construyendo una partida de ajedrez TUI**](https://luma.com/u436v3d7)
* 2026-05-12 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-12 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Introducción al acceso a bases de datos usando Rust SQLx**](https://www.meetup.com/code-mavens/events/314642118/)
* 2026-05-17 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/548kbqhl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #4 - ¡Codificación de cápsulas en QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Ver tu código - Una guía práctica para rastrear en Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/9v7hv2g1)
* 2026-06-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcjbfb/)

### África
* 2026-05-12 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust por ejemplo - Flujo de control**](https://www.meetup.com/johannesburg-rust-meetup/events/314614331/)

### Asia
* 2026-05-13 | Malasia, MI | [Rust Meetup Malasia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup mayo 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
* 2026-05-14 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/314649688/)
* 2026-05-16 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de mayo 2026**](https://hasgeek.com/rustbangalore/may-2026-rustacean-meetup/)

### Europa
* 2026-05-06 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en mayo: Rust para empezar, Parte 2**](https://www.meetup.com/rustcologne/events/314552161/)
* 2026-05-06 | Milano, MI, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Rust Milan @ Python Milano: ¿Python o Rust? ¡Sí!**](https://www.meetup.com/rust-language-milan/events/314521855/)
* 2026-05-06 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Construyendo LLMs desde cero**](https://www.meetup.com/oxford-rust-meetup-group/events/314456933/)
* 2026-05-07 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust May habla: Aetherus + Bevy**](https://www.meetup.com/rust-and-friends/events/314300802/)
* 2026-05-11 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #19**: Tiago Manczak - Juego con Rust & Pico](https://rust-augsburg.github.io/meetup/Meetup_19.html)
* 2026-05-13 | Girona, ES | [Rust Girona](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 - 2026-05-23 | Ámsterdam, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-18 | Milano, MI, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**SemanaOxidación 2026**](https://www.meetup.com/rust-language-milan/events/314329200/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hacks**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**anuncio de RustWeek 2026**](https://www.meetup.com/rust-nederland/events/312861992/)
* 2026-05-19 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Construcción y Prueba Cruzada**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | Londres, Reino Unido | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Comida de la Semana del Rust**](https://www.meetup.com/women-in-rust/events/314313054/)
* 2026-05-21 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Hackathon de la Semana del Rust**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Ámsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Tour en bicicleta por Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Programación Agente - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Noche de Código de Mayo de Rust Manchester**](https://www.meetup.com/rust-manchester/events/314452972/)
* 2026-05-29 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396588/)

### Norteamérica
* 2026-05-07 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Revirtiendo el Gran Cortafuegos y el Rust Geoespacial**](https://www.meetup.com/rust-nyc/events/314567143/)
* 2026-05-07 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Noche de Proyecto Abierto**](https://www.meetup.com/stl-rust/events/313807225/)
* 2026-05-09 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Back Bay Rust, 9 de mayo**](https://www.meetup.com/bostonrust/events/314480529/)
* 2026-05-14 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314469265/)
* 2026-05-14 | Portland, OR, EE. UU. [PDXRust](https://www.meetup.com/pdxrust)
    * [**De ondas de radio a píxeles - Visualizaciones en tiempo real con Rust y WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust May Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 2026-05-16 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de Lechmere, 16 de mayo**](https://www.meetup.com/bostonrust/events/314480531/)
* 2026-05-19 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 2026-05-20 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Control de ratón con Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, EE. UU. [Encuentro de Rust en el Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Rust en el Área de la Bahía**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de mayo de 2026**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro comunitario**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 23 de mayo**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust en sistemas embebidos y autónomos en sistemas paralelos en DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 30-05-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Central de Cambridge, 30 de mayo**](https://www.meetup.com/bostonrust/events/314480537/)

### Oceanía
* 2026-05-14 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne - mayo 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
* 2026-05-26 | Barton, ACT, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

### Sudamérica
* 2026-05-13 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Rust Uruguay meetup de Mayo**](https://www.meetup.com/rust-uruguay/events/314532884/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién contrata en r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Desde el punto de vista empresarial, deberíamos tener una confianza razonable en que se mantendrá y será saludable durante más de 10 años. También nos gustaría un ecosistema robusto de código y herramientas en los que podamos confiar, y expertos en los que podamos contratar.

– [David Anderson en el blog tailscale](https://tailscale.com/blog/tailscale-rs-rust-tsnet-library-preview)

¡Gracias a [Ivan Fraixedes](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1764) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1t5zbvq/this_week_in_rust_650/)</small>
