---
title: "Esta semana en Rust #118"
number_of_week: 118
description: El crate de esta semana es dashu, un conjunto puro de bibliotecas de Rust con números de precisión arbitrarios.
date: 2026-07-15
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
* [Anunciando Rust 1.97.0](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/)
* [crates.io: actualización de desarrollo](https://blog.rust-lang.org/2026/07/13/crates-io-development-update/)

### Actualizaciones de proyectos/herramientas
* [Reescribiendo Bun in Rust](https://bun.com/blog/bun-in-rust)
* [Anunciando BullMQ por Rust](https://bullmq.io/news/260712/rust-release/)
* [prost-protovalidation 0.6 — buf.validate (protovalidate) para prost y buffa: codegen en tiempo de compilación + conformidad con el CEL, 2872/2872](https://github.com/zs-dima/prost-protovalidate/releases/tag/v0.6.0)
* [plaza 1.0: un TUI gestor de paquetes ratatui que busca en pacman, el AUR, apt, dnf y Flatpak a la vez](https://github.com/StaszeKrk/plaza/releases/tag/v1.0.0)
* [Danubio v0.15.1: integración nativa de Apache Iceberg para exportación de streaming a lakehouse](https://github.com/danube-messaging/danube/releases/tag/v0.15.1)
* [Guardian Centinela. La interfaz de usuario del terminal para la base de datos descentralizada Guardian - P2P](https://www.willsearch.com.br/sentinel/)
* [kobe 0.33.0: un operador Rust para clústeres instantáneos de CI Kubernetes](https://github.com/kunobi-ninja/kobe/releases/tag/v0.33.0)
* [Elara Mesh: lo que realmente hace la caja negra para agentes de IA](https://navigatorbuilds.github.io/elara-mesh/blog/black-box-for-ai-agents.html)
* [Kache 0.10.0: descarga instantánea dedup, sin más encuestas](https://github.com/kunobi-ninja/kache/releases/tag/v0.10.0)

* [Cochlea 0.1.0: Un motor de audio determinista y sin cabeza para agentes de IA](https://richer-richard.github.io/cochlea/)

### Observaciones/Pensamientos
* [Podcast de seguridad de código abierto: Fondo de mantenedores de la Fundación Rust con Lori y Niko](https://opensourcesecurity.io/2026/2026-07-rfmf-lori-niko/)
* [Moviendo un SFU WebRTC Rust a hilos por núcleo](https://pulsebeam.dev/blog/moving-to-thread-per-core)
* [Pruebas de Rust más rápidas en CI con pasos paralelos](https://abundance.build/blog/2026-07-11-faster-rust-tests-in-ci-with-parallel-steps/)
* [vídeo] [El único diagrama que necesitas para entender la propiedad de Rust](https://www.youtube.com/watch?v=fugcSHD-9Jw)
* [Hemos compilado nuestro analizador TypeScript a WASM](https://encore.dev/blog/typescript-parser-wasm)
* [Entendiendo el bombo de Rust para el desarrollador ocupado](https://kerkour.com/rust-hype)
* [Combiné con equipo directo mi propio gateway de seguridad LLM (Rust) en cuatro pasadas — cada brecha de detección y cómo la cerré](https://dev.to/akavlabs_69/i-red-teamed-my-own-llm-security-gateway-in-four-passes-heres-every-gap-i-found-5cl9)

### Guías de Rust
* [vídeo] [Conceptos de backend en Rust: servidores HTTP](https://www.youtube.com/watch?v=DJhhy6YQe8k)
* [Fearless Embedded Rust: Un coche de Lego FPV](https://dystroy.org/blog/picamobile/)
* [Lo que aprendí construyendo un formato de archivo autocorruptible en Rust](https://www.aravpanwar.com/writing/building-decayfmt-in-rust/)
* [Ven Asincrónico Eres](https://corentin-core.github.io/posts/ruxe-async-runtime-agnostic/)

### Miscelánea
* [Oxidar XIAO — Un programa comunitario de Rust incrustado](https://blog.theembeddedrustacean.com/oxidize-xiao)

## Crate de la semana

El crate de esta semana es [dashu](https://crates.io/crates/dashu), un conjunto puro de bibliotecas de Rust con números de precisión arbitrarios.

¡Gracias a [JacobZ](https://users.rust-lang.org/t/crate-of-the-week/2704/1628) por la autosugerencia!

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

* [Nika - showcase: CSV → chart PNG → report markdown (nika:chart aún no tiene ejemplo)](https://github.com/supernovae-st/nika/issues/424)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

550 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-07..2026-07-14

#### Compilador
* [alineando algunas funciones de 'Símbolo'](https://github.com/rust-lang/rust/pull/158931)
* [limpieza de predicados/cláusulas](https://github.com/rust-lang/rust/pull/157104)
* [eliminar algunos campos 'tokens' de AST](https://github.com/rust-lang/rust/pull/158942)
* [resolver: envolver arenas en 'WorkerLocal'](https://github.com/rust-lang/rust/pull/159019)
* [rework read deduplication con grabadoras de lectura agrupadas](https://github.com/rust-lang/rust/pull/158794)
* [reducir 'mir::Statement' a 40 bytes](https://github.com/rust-lang/rust/pull/159012)
* [explicación de la caída sin operación del psicólogo](https://github.com/rust-lang/rust/pull/157491)
* [caso común especializado '(1, 1)' para la unificación de arg](https://github.com/rust-lang/rust/pull/158865)
* [usar SmallVec para los lugares de devolución en MIR](https://github.com/rust-lang/rust/pull/158842)

#### Biblioteca
* [añadir expl 'Iterator::count' impl para 'ChunkBy'](https://github.com/rust-lang/rust/pull/158866)
* [permiten que 'Allocator's se usen como '#[global_allocator]'s](https://github.com/rust-lang/rust/pull/157153)
* [corregir múltiples fallos lógicos en 'Arc::make_mut'](https://github.com/rust-lang/rust/pull/158876)
* [implementar la característica 'char_to_u32'](https://github.com/rust-lang/rust/pull/158940)
* [hacer que las operaciones volátiles constren](https://github.com/rust-lang/rust/pull/159092)
* [mover 'std::io::Write' a 'core::io'](https://github.com/rust-lang/rust/pull/158541)
* [estabilizar 'Cuerda::from_utf8_lossy_owned'](https://github.com/rust-lang/rust/pull/159099)
* [estabilizar 'VecDeque::retain_back' desde 'truncate_front'](https://github.com/rust-lang/rust/pull/151379)

#### Carga
* ['instalar': Mover --depurar a Opciones de compilación](https://github.com/rust-lang/cargo/pull/17199)
* ['fuente': advertencia incorrecta de paquete duplicado](https://github.com/rust-lang/cargo/pull/17204)
* [corregir generación de esquema de manifiesto: 'TomlDebugInfo' enum-variants no se renombra](https://github.com/rust-lang/cargo/pull/17202)
* [no aplicar la puerta host-config a un comportamiento estable](https://github.com/rust-lang/cargo/pull/17198)
* [reducir la longitud de la ruta de búsqueda de biblioteca en la nueva disposición del director de construcción](https://github.com/rust-lang/cargo/pull/17191)
* [reducir args rustc '-L' usados en el nuevo diseño 'build-dir'](https://github.com/rust-lang/cargo/pull/17168)
* [renombrar '-Zno-embed-metadata' a '-Zembed-metadata=no'](https://github.com/rust-lang/cargo/pull/17149)
* [prueba: arreglar la raza en 'cargo_compile_with_invalid_code_in_deps'](https://github.com/rust-lang/cargo/pull/17203)

#### Clippy
* [añadir nuevos lints: 'rest_pattern_accessible_field' y 'unnecessary_rest_pattern'](https://github.com/rust-lang/rust-clippy/pull/15000)
* [nueva pelusa: 'definition_in_module_root'](https://github.com/rust-lang/rust-clippy/pull/16965)
* ['arbitrary_source_item_ordering': añadir modos de orden de elementos de rasgos configurables](https://github.com/rust-lang/rust-clippy/pull/17343)
* ['tests_outside_test_module': pon código en backticks en el mensaje de pelusa](https://github.com/rust-lang/rust-clippy/pull/17387)
* [cuenta la longitud del primer párrafo por su texto](https://github.com/rust-lang/rust-clippy/pull/17215)
* [corregir 'suboptimal_flops' falso negativo con literales ambiguos de flotación](https://github.com/rust-lang/rust-clippy/pull/16980)
* [desactiva parcialmente 'unneeded_wildcard_pattern' cuando 'rest_pattern_accessible_field' está activada](https://github.com/rust-lang/rust-clippy/pull/17416)
* [respetar la MSRV implicit_saturating_sub configurada en la reescritura 'if x != 0 { x -= 1 }'s en la reescritura](https://github.com/rust-lang/rust-clippy/pull/17404)
* [activar 'single_element_loop' si el bloque contiene solo una expresión final](https://github.com/rust-lang/rust-clippy/pull/16513)
* [optimizar 'nonstandard_macro_braces' en un 99,9683% (1,1b → 351K)](https://github.com/rust-lang/rust-clippy/pull/16808)
* [PERF: Salta de la regla de 'disallowed_methods' si la lista de prohibidos está vacía](https://github.com/rust-lang/rust-clippy/pull/17381)

#### Analizador de Rust
* [pide divulgación en contribuciones de IA](https://github.com/rust-lang/rust-analyzer/pull/22771)
* [añadir correcciones para la longitud del array para 'type_mismatch'](https://github.com/rust-lang/rust-analyzer/pull/22734)
* [añadir paréntesis en tipo dyn transformado en tipo de referencia](https://github.com/rust-lang/rust-analyzer/pull/22741)
* [evitar el pánico en las importaciones de fusión en el separador de camino de seguimiento](https://github.com/rust-lang/rust-analyzer/pull/22736)
* [cambia algunas cosas por '#[doc = macro!()]'expansión](https://github.com/rust-lang/rust-analyzer/pull/22654)
* [clamp cttz const-eval resultado a ancho de tipo](https://github.com/rust-lang/rust-analyzer/pull/22770)
* [Expedición de cola correctamente gestionada por CFG, toma 2](https://github.com/rust-lang/rust-analyzer/pull/22751)
* [fallo en acciones de código cuando hay un módulo sin resolver](https://github.com/rust-lang/rust-analyzer/pull/22749)
* [fallo al calcular diagnósticos con MIR y tipos de error](https://github.com/rust-lang/rust-analyzer/pull/22707)
* [no completar el default en default impl](https://github.com/rust-lang/rust-analyzer/pull/22744)
* [clasificación tardía temprana de las vidas](https://github.com/rust-lang/rust-analyzer/pull/22283)
* [corregir 'render_const_using_debug_impl' construyendo diseños de estándar estándar obsoletos](https://github.com/rust-lang/rust-analyzer/pull/22583)
* [corrigir las macros de activación 'TokenStream::from_str()' para comentarios en documentos](https://github.com/rust-lang/rust-analyzer/pull/22735)
* [ocultar campos privados al pasar el cursor según el contexto](https://github.com/rust-lang/rust-analyzer/pull/22464)
* [haz que el tipo 'Response' del servidor lsp se alinee más a JSON-RPC](https://github.com/rust-lang/rust-analyzer/pull/22753)
* [Const asociado bastante cuando el rasgo está en macro](https://github.com/rust-lang/rust-analyzer/pull/22535)
* [reimplementar la heurística sintáctica 'crate_supports_no_std'](https://github.com/rust-lang/rust-analyzer/pull/22747)
* [resolver correctamente caminos no planos en los bloques](https://github.com/rust-lang/rust-analyzer/pull/22773)
* [soporte Cargo 1.97.0 configuración de ruta de archivo bloqueado](https://github.com/rust-lang/rust-analyzer/pull/22683)
* [Hir-ty: Envía el contenedor de paseo explícita por 'unused_must_use'](https://github.com/rust-lang/rust-analyzer/pull/22405)
* [arreglar Enter borrando/interpretando erróneamente '$foo'](https://github.com/rust-lang/rust-analyzer/pull/22768)
* [sugieren correcciones de acciones de código producidas a partir de diagnósticos bajo cursor, incluso si tienen efectos en otros lugares](https://github.com/rust-lang/rust-analyzer/pull/22726)
* [trata los archivos de biblioteca como verdaderamente inmutables para el cliente](https://github.com/rust-lang/rust-analyzer/pull/22777)
* [convierte 'BlockLoc' en una estructura con orugas, toma 3](https://github.com/rust-lang/rust-analyzer/pull/22534)

### Triaje de rendimiento del compilador Rust

Esta semana han llegado muchas optimizaciones nuevas, haciendo de esta una semana muy buena para el rendimiento.
La única regresión real fue una corrección de una compilación errónea que probablemente se reaterrizará en el futuro.

Triaje hecho por **@JonathanBrouwer**.
Rango de revisión: [3659db0d.. 5503df87](https://perf.rust-lang.org/?start=3659db0d3e2cd634c766fcda79ed118eca31a9fd&end=5503df87342a73d0c29126a7e08dc9c1255c46ad&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,3% | [0,2%, 0,4%] | 3 |
| Regresiones ❌ <br /> (secundario) | 0,9% | [0,1%, 2,5%] | 25 |
| Mejoras ✅ <br /> (primaria) | -1,2% | [-9,9%, -0,2%] | 195 |
| Mejoras ✅ <br /> (secundario) | -3,4% | [-92,1%, -0,1%] | 174 |
| Todos ❌✅ (primario) | -1,2% | [-9,9%, 0,4%] | 198 |

2 regresiones, 10 mejoras, 10 mixtas; 7 de ellos en rollups
36 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/212da2d63f1edf2ab22293547a99f0fbf8cb68a8/triage/2026/2026-07-13.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [Parámetros de rasgo 'Fn' nombrados](https://github.com/rust-lang/rfcs/pull/3955)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [activar la pelusa 'unreachable_cfg_select_predicates' como parte del grupo de pelusas 'no utilizado'](https://github.com/rust-lang/rust/pull/159179)
* [Estabilizar 'Asignador dyn'](https://github.com/rust-lang/rust/issues/156906)
* [Problema de seguimiento para vec_try_remove](https://github.com/rust-lang/rust/issues/146954)
* [Estabilizar parcialmente 'box_vec_non_null'](https://github.com/rust-lang/rust/pull/157226)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Activar '-Zpolonius=next' cada noche](https://github.com/rust-lang/compiler-team/issues/1015)
* [Activar '-Znext-solver' por defecto por la noche para pruebas](https://github.com/rust-lang/compiler-team/issues/1014)
* [Estabilizando el estado del conjunto de pruebas debuginfo](https://github.com/rust-lang/compiler-team/issues/1012)
* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [tipo primitivo de bf16](https://github.com/rust-lang/rfcs/pull/3983)

## Próximos eventos

Eventos Rusty entre el 15-07-2026 y el 12-08-2026 🦀

### Virtual
* 2026-07-15 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió semanal de codificació / Sesión semanal de codificación**](https://luma.com/21k797xr)
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Aprendiendo el Rust como primer lenguaje de programación**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Tel Aviv-yafo, IL) | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/315676843/)
* 2026-07-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/315279653/)
* 222-07-2026 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/hd8mlw56)
* 2026-07-23 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315418155/)
* 2026-07-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254777/)
* 2026-07-29 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/uo5ek1f4)
* 30-07-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-08-02 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-04 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-08-05 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-08-05 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-11 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-12 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/f2hnzrug)
* 2026-07-19 | Virtual (Bangalore, IN) | [Discordia de Rust Incrustado](https://discord.gg/VJyv3NfVdw)
    * [**Domingos de Silicio**](https://discord.gg/6gwCNpFP?event=1526087936234225814)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de julio de 2026**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)
* 2026-07-19 | Virtual (Bangalore, IN) | [Discordia de Rust Incrustado](https://discord.gg/VJyv3NfVdw)
    * [**Domingos de Silicio**](https://discord.gg/6gwCNpFP?event=1526087936234225814)
* 2026-07-25 | Mumbai, IN | [Rust Mumbai](https://luma.com/mumbai)
    * [**Rust Mumbai — Encuentro 🦀 de julio **](https://luma.com/7ksabwbm/)
* 2026-07-26 | Pune, MA, IN | [Rust Pune](https://www.meetup.com/rust-pune/events/)
    * [**Rust Pune: julio 2026**](https://www.meetup.com/rust-pune/events/315651505/)

### Europa
* 2026-07-15 | Dortmund, DE | [Dortmund Oxidado](https://www.meetup.com/rust-dortmund/events/)
    * [**Enseña y Hackea en Projektspeicher**](https://www.meetup.com/rust-dortmund/events/315496876/)
* 2026-07-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Funciones de Supercharge Rust con argumentos implícitos y programación genérica de contexto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: La próxima generación**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | Londres, Reino Unido | [Grupo del Proyecto Rust de Londres](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework para Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN habla: Adquisición de Antítesis en julio de 2026 (https://www.meetup.com/rust-london-user-group/events/315612916/)
* 2026-07-23 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 2026-07-29 | Polonia, PL | [Polonia Oxidada](https://www.meetup.com/rust-poland-meetup)
    * [**Polonia oxidada x Cracovia #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 30-07-2026 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Noche de Código de Julio de Rust Manchester**](https://www.meetup.com/rust-manchester/events/315037685/)

### Norteamérica
* 2026-07-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Seattle Rust User Group) de julio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo North End Rust, 18 de julio**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 222-07-2026 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 222-07-2026 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: ¡Rust en sistemas distribuidos con ciencia del vuelo!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 222-07-2026 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Escribe un agente de codificación personalizado y wasm_zero**](https://www.meetup.com/rust-nyc/events/315636854/)
* 2026-07-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Porter Square Rust, 25 de julio**](https://www.meetup.com/bostonrust/events/315582650/)
* 2026-07-25 | Brooklyn, NY, EE. UU. | [Flor](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)
* 30-07-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Chinatown, 1 de agosto**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Encuentro nocturno de Boston Rust en Red Hat, 4 de agosto**](https://www.meetup.com/bostonrust/events/314660176/)
* 2026-08-06 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Envío temporal: Cómo un ecosistema global de Rust construyó la última API web de Chrome**](https://www.meetup.com/stl-rust/events/314701905/)

### Sudamérica
* 2026-08-08 | São Paulo, SP | [Rust-SP](https://luma.com/calendar/cal-bif2oHITU1aVvsr)
    * [**Rust SP - Ago/2026**](https://luma.com/41oiyhtk)

### Oceanía
* 2026-07-21 | Barton, AU | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra)
    * [**Encuentro de julio**](https://www.meetup.com/rust-canberra/events/315307280/)
* 2026-07-23 | Perth, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group)
    * [**¡Rust Perth: Encuentro de julio!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)
* 30-07-2026 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne julio 2026**](https://www.meetup.com/rust-melbourne/events/315039480/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Gracias por tu imagen personal, pero por favor edita la descripción como si fueras un maniático con motosierra que acaba de descubrir que las frases son de jóvenes adultos que llegaron al lago en un campamento de verano después del atardecer.

– [workingjubilee en Rust github](https://github.com/rust-lang/rust/pull/159039#issuecomment-4931084997)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1786) por la sugerencia!

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

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1uxsigp/this_week_in_rust_660/)</small>
