---
title: "Esta semana en Rust #95"
number_of_week: 95
description: El crate de esta semana es dynamodb-crud, una API segura para tipos para trabajar con tablas de DynamoDB.
date: 2026-01-28
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
* [crates.io: actualización de desarrollo | Rust Blog](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/)
* [Anunciando Rust 1.93.0 | Rust Blog](https://blog.rust-lang.org/2026/01/22/Rust-1.93.0/)

### Actualizaciones de proyectos/herramientas
* [Vetis: Un servidor HTTP muy pequeño y muy rápido en Rust](https://dev.to/rogrio_arajo_55dae16f0d/vetis-a-very-tiny-very-fast-http-server-in-rust-1ldn)
* [cai 0.13: Herramienta de CLI fácil de usar para tareas de IA](https://github.com/ad-si/cai/releases/tag/v0.13.0)
* [Nio v0.1.0: Adoptando la arquitectura Hilos por Núcleo](https://nurmohammed840.github.io/posts/embracing-thread-per-core-architecture/)
* [ 🦀 La reflexión en tiempo de compilación por fin ha llegado](https://weeklyrust.substack.com/p/compile-time-reflection-is-finally)
* [r3bl_tui v0.7.7: modern async TUI lib — readline, editor md, flexbox, renderizado optimizado para SSH](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.7.7-tui)
* [r3bl-cmdr v0.0.25: Aplicaciones de productividad TUI - giti (git helper) y edi (beautiful md editor)](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.25-cmdr)
* [R3bl-build-infra v0.0.1: cargo-rustdoc-fmt — tablas MD más bonitas y enlaces estilo ref](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.1-build-infra)

### Observaciones/Pensamientos
* [Inmersión profunda en Turso, la "reescritura de SQLite en Rust"](https://kerkour.com/turso-sqlite)
* [Rust a escala: una capa adicional de seguridad para WhatsApp](https://engineering.fb.com/2026/01/27/security/rust-at-scale-security-whatsapp/)
* [Rust vs JavaScript & TypeScript: rendimiento, WebAssembly y experiencia de desarrollador](https://blog.jetbrains.com/rust/2026/01/27/rust-vs-javascript-typescript/)
* [Las variables atómicas no se tratan solo de atomicidad](https://sander.saares.eu/2026/01/25/atomic-variables-are-not-only-about-atomicity/)
* ['si dejamos proteger' camino estabilizador](https://kivooeo.github.io/blog/if-let-guard/)
* [audio] [Netstack.FM episodio 24 — WebAssembly y Rust en la práctica, una conversación con Alex Crichton](https://netstack.fm/#episode-24)
* [audio] [Novedades en Rust 1.88 a 1.90 :: Estación Rustacean](https://rustacean-station.org/episode/rust-1.88-1.89-1.90/)
* [audio] [Novedades en Rust 1.85 a 1.87 :: Estación Rustacean](https://rustacean-station.org/episode/rust-1.85-1.86-1.87/)
* [vídeo] [IA debería escribir Rust y solo Rust ;)](https://www.youtube.com/watch?v=2lhr-QDWv-k)
  
### Guías de Rust
* [La guía completa para publicar tu primera caja de Rust en crates.io](https://dev.to/ajitkumar/the-complete-guide-to-publishing-your-first-rust-crate-to-cratesio-14pg)
* [Diseño de tipos de error en aplicaciones de Rust](https://home.expurple.me/posts/designing-error-types-in-rust-applications/)
* [serie] [Parte 4: Infraestructura de formación, Construcción de un LLM desde cero en Rust](https://www.tag1.com/how-to/part4-training-infrastructure-building-an-llm-from-scratch/)
* [Usando Oracle db26ai de Rust con la caja oráculo (2)](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_oracle_crate_2/)
* [serie] [La Guía del Programador Impaciente para Envase y Rust: Capítulo 6 - Que haya partículas](https://aibodh.com/posts/bevy-rust-game-development-chapter-6/)
* [Construyendo una IA offline de 24MB con Rust + Quemadura](https://snaetwarre.github.io/My-Portofolio/blog/intelligent-disease-detection.html)
* [El cuello de botella oculto: bloqueando en Rust asíncrono](https://cong-or.xyz/blocking-async-rust.html)
* [Sustituyendo Protobuf por Rust para ir 5 veces más rápido](https://pgdog.dev/blog/replace-protobuf-with-rust)
* [Empaqueté mi CLI de Rust en demasiados sitios, esto es lo que aprendí](https://ivaniscoding.github.io/posts/rustpackaging1/)

## Crate de la semana

El crate de esta semana es [dynamodb-crud](https://github.com/dariocurr/dynamodb-crud), una API segura para tipos para trabajar con tablas de DynamoDB.

¡Gracias a [dario curreri](https://users.rust-lang.org/t/crate-of-the-week/2704/1524) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

### [Carga](https://github.com/rust-lang/cargo/labels/call-for-testing)

* [Problema de seguimiento para el backend de renderizado SVG del informe de temporización](https://github.com/rust-lang/cargo/issues/16440)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Ruído](https://github.com/rust-lang/rustup/labels/call-for-testing) o
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

* [Diésel - Argumento diésel puerto-cli analizando de constructores de aplausos a derivados de clap](https://github.com/diesel-rs/diesel/issues/4955)

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP cierra el 16-02-2026 | Montreal, Quebec, Canadá | 2026-09-08 - 2026-09-11

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

479 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-20..2026-01-27

#### Compilador
* ['const' bloquea como elemento 'mod'](https://github.com/rust-lang/rust/pull/149174)
* [mejora el mensaje de error para 'assert! ()' macro en funciones que devuelve bool](https://github.com/rust-lang/rust/pull/151457)
* [cálculo de restricciones tipográficas localizadas constantes en línea](https://github.com/rust-lang/rust/pull/149639)

#### Biblioteca
* ['ptr_aligment_type': añadir más APIs](https://github.com/rust-lang/rust/pull/148764)
* [añadir 'simd_splat' intrínseco](https://github.com/rust-lang/rust/pull/151346)
* [evita activar unicode al llamar a 'io::Error::kind'](https://github.com/rust-lang/rust/pull/151418)
* [evitar desgarrar las impresiones de 'dbg!'](https://github.com/rust-lang/rust/pull/149869)
* [constify boolean methods](https://github.com/rust-lang/rust/pull/151489)
* [asegurarse de que el plazo ha pasado en 'sleep_until'](https://github.com/rust-lang/rust/pull/151494)
* [corregir la regresión de rendimiento 'is_ascii' en CPUs AVX-512 al compilar con -C target-cpu=native](https://github.com/rust-lang/rust/pull/151259)
* [mejora el rendimiento de 'is_ascii' en 'x86_64' con intrínsecos explícitos de SSE2](https://github.com/rust-lang/rust/pull/151611)
* [hacer que 'simd_insert_dyn' y 'simd_extract_dyn' consten](https://github.com/rust-lang/rust/pull/151453)
* [optimizar 'vec.extend(slice.to_vec())', toma 2](https://github.com/rust-lang/rust/pull/151337)
* [usar 'Display' de 'ByteStr' para 'OsStr'](https://github.com/rust-lang/rust/pull/151010)

#### Carga
* [añadir -Z json-target-spec](https://github.com/rust-lang/cargo/pull/16557)
* [no compruebes la salida específica del build-std](https://github.com/rust-lang/cargo/pull/16551)
* [Fix build-std lto test para ejecutarse en otras plataformas](https://github.com/rust-lang/cargo/pull/16550)
* [arreglar: mostrar 'implicit_minimum_version_req' fuente emitida una vez por paquete](https://github.com/rust-lang/cargo/pull/16535)
* [aumentar el tiempo de espera de la prueba de 'cache_lock'](https://github.com/rust-lang/cargo/pull/16545)
* [pelusa: Añadir pelusa de 'redundant_readme'](https://github.com/rust-lang/cargo/pull/16552)
* [lints: Añadir 'non_*_case_features'](https://github.com/rust-lang/cargo/pull/16560)
* [lints: Añadir pelusa de 'non_kebab_case_bin'](https://github.com/rust-lang/cargo/pull/16524)
* [lints: Añadir 'non_{kebab,snake}_case_packages'](https://github.com/rust-lang/cargo/pull/16554)
* [lints: plural 'non_kebab_case_bins'](https://github.com/rust-lang/cargo/pull/16553)
* [rm: Sugiere banderas de tabla cuando no se especifica ninguna](https://github.com/rust-lang/cargo/pull/16533)

#### Rustdoc
* [añadir el enlace "Saltar al contenido principal" para la navegación con teclado en rustdoc](https://github.com/rust-lang/rust/pull/151482)
* [Haz que el contenido de los menús emergentes sea desplazable en dispositivos móviles](https://github.com/rust-lang/rust/pull/151216)

#### Clippy
* [también ignora casos con comentarios en 'let_and_return'](https://github.com/rust-lang/rust-clippy/pull/16461)
* [corregir 'manual_dangling_ptr' falso positivo cuando el tipo de punta no está 'Sized'](https://github.com/rust-lang/rust-clippy/pull/16469)
* [corregir 'test_attr_in_doctest' falso positivo en 'test_harness'](https://github.com/rust-lang/rust-clippy/pull/16454)
* [hacer 'manual_is_variant_and' para cubrir el manual 'is_none_or'](https://github.com/rust-lang/rust-clippy/pull/16424)
* ['manual_let_else': añadir coma final a patrones de 'struct' que terminan en '.. '](https://github.com/rust-lang/rust-clippy/pull/16442)
* [RHS de la expresión por cortocircuito no siempre se ejecuta](https://github.com/rust-lang/rust-clippy/pull/16463)

#### Analizador de Rust
* ['hir-ty': añadir método 'references_only_ty_error' para detectar errores de tipo](https://github.com/rust-lang/rust-analyzer/pull/21497)
* [añade punto y coma para 'toggle_macro_delimiter'](https://github.com/rust-lang/rust-analyzer/pull/21522)
* [ruta correcta de desgramática en el parche](https://github.com/rust-lang/rust-analyzer/pull/21523)
* ['default_field_values'](https://github.com/rust-lang/rust-analyzer/pull/21408)
* [no mezclar el orden de derivados incorporados/regulares en "Expandir la macro recursivamente"](https://github.com/rust-lang/rust-analyzer/pull/21490)
* [no ofrezcas 'apply_demorgan' en 'si lo hagas'](https://github.com/rust-lang/rust-analyzer/pull/21499)
* [corrección no completa 'else' antes de tupla](https://github.com/rust-lang/rust-analyzer/pull/21495)
* [corrección incorrecta continuar para 'convert_range_for_to_while'](https://github.com/rust-lang/rust-analyzer/pull/21514)

### Triaje de rendimiento del compilador Rust

Esta semana ha tenido una victoria muy buena por haber hecho menos trabajo en general en el compilador (https://github.com/rust-lang/rust/pull/151382). Hubo algunas regresiones, pero solo en pruebas de esfuerzo artificiales,
Los estamos vigilando.

Triaje hecho por **@kobzol**.
Rango de revisión: [3d087e60.. ebf13cca](https://perf.rust-lang.org/?start=3d087e6044bddc65723bf42c76fe4cc33a0076b0&end=ebf13cca58b551b83133d4895e123f7d1e795111&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,6% | [0,2%, 1,8%] | 9 |
| Regresiones ❌ <br /> (secundario) | 3,1% | [0,1%, 19,9%] | 47 |
| Mejoras ✅ <br /> (primaria) | -1,0% | [-3,1%, -0,2%] | 195 |
| Mejoras ✅ <br /> (secundario) | -1,4% | [-10,1%, -0,1%] | 157 |
| Todos ❌✅ (primario) | -1,0% | [-3,1%, 1,8%] | 204 |

2 regresiones, 2 mejoras, 6 mixtas; 6 de ellos en rollups
42 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/480fd97adc2ee4f7bbecea3e62e32503ebcc27d7/triage/2026/2026-01-27.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?
* [Método de rasgos implica restricciones (métodos 'finales')](https://github.com/rust-lang/rfcs/pull/3678)
* [RFC: '#[export_visibility = ...]' atributo](https://github.com/rust-lang/rfcs/pull/3834)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Dar un signo a los literales enteros en lugar de depender de expresiones de negación](https://github.com/rust-lang/compiler-team/issues/835)
* [También activar volcados de archivos ICE en stable](https://github.com/rust-lang/compiler-team/issues/809)
* [Nueva propuesta objetivo de nivel 3: loongarch64-linux-android](https://github.com/rust-lang/compiler-team/issues/806)

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Estabilizar 'núcleo::pista::cold_path'](https://github.com/rust-lang/rust/pull/151576)
* [Problema de seguimiento para métodos de 'ControlFlow' de constancia mínima ('min_const_control_flow')](https://github.com/rust-lang/rust/issues/148738)
* [Problema de seguimiento para 'new_range_api' (parte del RFC 3550)](https://github.com/rust-lang/rust/issues/125687)
* [Estabilizar 'assert_matches'](https://github.com/rust-lang/rust/pull/137487)
* [resolver: Reportar más ambigüedades de resolución temprana relacionadas con la visibilidad para importaciones](https://github.com/rust-lang/rust/pull/149596)
* [Añadir FCW para derivar atributos ayudantes que entren en conflicto con los atributos incorporados](https://github.com/rust-lang/rust/pull/151152)
* [Constify 'fmt::from_fn'](https://github.com/rust-lang/rust/pull/150300)
* [Patrones 'mut ref' en la puerta de características en la taquigrafía del campo de patrón struct](https://github.com/rust-lang/rust/pull/151102)
* [Problema de seguimiento para métodos de conversión de puntero crudo a referencia](https://github.com/rust-lang/rust/issues/122034)
* [implementa 'ParcialEq<<U>Vec>' para \[T; N\] y &\[T; N\]](https://github.com/rust-lang/rust/pull/149045)
* [thread::scope: documenta cómo interactúa join con destructores TLS](https://github.com/rust-lang/rust/pull/149482)

#### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [docs(informe): mejorar las páginas de usuario para 'informe de carga *'](https://github.com/rust-lang/cargo/pull/16430)

#### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Garantía de desplazamiento de campo de unión repr(C)(https://github.com/rust-lang/reference/pull/2128)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Desambiguación de métodos naturales](https://github.com/rust-lang/rfcs/pull/3913)
* [Añadir 'derive(Deref)' RFC](https://github.com/rust-lang/rfcs/pull/3911)
* [Abi Descriptores](https://github.com/rust-lang/rfcs/pull/3910)
* [Autenticación del registro mTLS de carga](https://github.com/rust-lang/rfcs/pull/3907)
* [Sea 'Opción' derivar '#[must_use]](https://github.com/rust-lang/rfcs/pull/3906)
* [CFGS con tipo de versión](https://github.com/rust-lang/rfcs/pull/3905)

## Próximos eventos

Eventos Rusty entre el 28-01-2026 - el 25-02-2026 🦀

### Virtual
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/9h9n0dau)
* 2026-01-28 | Virtual (Lima-Perú, PE) | [Perú Oxidado](https://rust.pe/)
    * [**Meetup: Oxidación del hardware a la web (Embebido + Backend desde cero)**](https://calendar.app.google/jc9DziLWVTUn1qNVA)
* 2026-01-29 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Encuentro de Bevy #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #2 - Procesos oxidados, límites de memoria y cápsulas básicas**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 1: Conceptos Comunes de Programación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens)
    * [**Lectura de código oxidado y contribución de código abierto (UTC 18:00; Inglés)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Empezando con Rust Parte 2: Propiedad y Estructuras**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/ir8s81ec)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-24 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254788/)
* 2026-02-24 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Almuerzo y aprendizaje: Patrón de Rust Coincidiendo Desempacado**](https://www.meetup.com/women-in-rust/events/312799411/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/fvcjjuv8)

### Asia
* 2026-02-05 | Seúl, KR | [Seoul Rust (lenguaje de programación) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Encuentro de Seúl Rust**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MI | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Encuentro de Malasia Rust febrero 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de febrero de 2026**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**En persona Rust febrero 2026 en Nuvoton en Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europa
* 2026-01-28 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - enero 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-01-28 | Praga, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Praga (enero de 2026)**](https://www.meetup.com/rust-prague/events/312895761/)
* 2026-01-28 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - WASM & Elegant CLI**](https://www.meetup.com/rust-community-toulouse/events/312782796/)
* 2026-01-29 | Ostrava, CZ | [Encuentro con Actualización Ostrava](https://www.meetup.com/meetupdate-ostrava)
    * [**MeetUpdate Ostrava #28: Rust**](https://www.meetup.com/meetupdate-ostrava/events/312747904/)
* 2026-01-31 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #23**](https://www.meetup.com/stockholm-rust/events/312919934/)
* 2026-02-04 | Darmstadt, HE, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo un servicio de suscripción a un boletín con axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-04 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en febrero: Acelera tu Python**](https://www.meetup.com/rustcologne/events/313111752/)
* 2026-02-04 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, Reino Unido | [Encuentro Oxford ACCU/Rust.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Más allá del código: Diseñando servicios que resisten la prueba del tiempo**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Hack Rust & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Hack y Aprendizaje de Karlsruhe Meetup en BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, Reino Unido | [Leyendo el Taller de Rust](https://www.meetup.com/reading-rust-workshop)
    * [**Encuentro de Rust leyendo**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Ginebra, CH | [Laboratorio posterior a Tenebras](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-18 - 2026-02-19 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312851079)

### Norteamérica
* 2026-01-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Ángeles: Construyendo reemplazos de Git-LFS en Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Programación Rust 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust del Noreste, 31 de enero**](https://www.meetup.com/bostonrust/events/312483767/)
* 2026-02-03 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Renderizado y tiempos de construcción de Bevy en Amazon**](https://www.meetup.com/rust-nyc/events/312871242/)
* 2026-02-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**Social mensual de febrero**](https://www.meetup.com/rust-montreal/events/313068358/)
* 2026-02-05 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en el DOJO HACKER**](https://www.meetup.com/hackerdojo/events/312859472/)
* 2026-02-05 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Renderizando el set de Mandelbrot en Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Allston Rust, 7 de febrero**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-12 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Desarrollo web Full Stack en Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Lugar de Encuentro**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de febrero de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro y Saludo Comunitario**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-25 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Lugar de Comida**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust como capa de pegamento- Infraestructura para aplicaciones nativas de IA**](https://www.meetup.com/rust-los-angeles/events/313097225/)

### Oceanía
* 2026-02-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane febrero 2026**](https://www.meetup.com/rust-brisbane/events/313087789/)
* 2026-02-11 | Sídney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
    * [**Bienvenidos 🦀 a 2026**](https://www.meetup.com/rust-sydney/events/313074935/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Decirle a un programador que ya existe una biblioteca para hacer X es como decirle a un compositor que ya existe una canción sobre el amor.

– [Pete Cordell citado por @blonk sobre rust-users](https://users.rust-lang.org/t/i-am-looking-for-feedback-for-my-own-game-engine-which-have-written-in-rust/137509/4)

¡Gracias a [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1752) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1qqujzs/this_week_in_rust_636/)</small>
