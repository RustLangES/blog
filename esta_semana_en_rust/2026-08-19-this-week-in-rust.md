---
title: "Esta semana en Rust #123"
number_of_week: 123
description: El crate de esta semana es tokio_with_wasm, una caja que permite que una única base de código de tokio se ejecute tanto de forma nativa como en navegadores web.
date: 2026-08-19
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
* [Experimento para reducir el tamaño del directorio objetivo cada noche](https://blog.rust-lang.org/inside-rust/2026/08/18/reducing-target-dir-size-on-nightly/)

### Boletines
* [El Rustaceo Incrustado Número #78](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-78)

### Actualizaciones de proyectos/herramientas
* [OxiSH: un servidor SSH moderno y seguro para la memoria](https://dirkjan.ochtman.nl/writing/2026/08/13/announcing-oxish.html)
* [Una crítica de Xilem en 2026](https://hackmd.io/@s_haMSbyTAOWfoXc1aYNUg/Hka74gCwZg)
* [Rama v0.4.0](https://github.com/plabayo/rama/releases/tag/rama-0.4.0)

### Observaciones/Pensamientos
* [De Go to Rust](https://rolandsdev.blog/posts/from-go-to-rust/)
* [Cómo se sintió Zig, viniendo de Rust](https://besok.github.io/posts/what-zig-felt-like-coming-from-rust/)
* [Quiero un "fil-c"" externo](https://domenkozar.com/2026/08/13/i-want-extern-fil-c/)
* [Cuatro niveles de inicialización in situ](https://blog.yoshuawuyts.com/four-levels-of-in-place-initialization/)
* [Protegiendo la biblioteca estándar Rust de roturas accidentales](https://predr.ag/blog/protecting-the-rust-stdlib-from-breakage/)
* [Renderizado wgpu de copia cero dentro de una app Electron](https://murlet.com/blog/rendering-wgpu-under-electron/)
* [La pelusa que la habría atrapado está apagada por defecto](https://ai2rules.dev/blog/the-lint-that-was-off-by-default/)
* [vídeo] [serie] [Implementando máquinas de estados (Parte 1)](https://youtu.be/q4GdfJNKI-M)

### Guías de Rust
* [Una introducción suave a Embedded Rust](https://niss36.github.io/blog/01-gentle-intro-to-embedded-rust/)
* [Construcción de una API RESTful Contenedorizada](https://learning-rust.github.io/labs/building-a-containerized-restful-api/)

### Investigación
* [descarga de GPU en Rust: portátil, seguro y rápido](https://arxiv.org/pdf/2608.13759)

### Miscelánea
* [Zerocopy con Joshua Liebow-Feeser](https://joshlf.com/posts/netstack-fm-ep-10/)

## Crate de la semana

El crate de esta semana es [tokio_with_wasm](https://crates.io/crates/tokio_with_wasm), una caja que permite que una única base de código de tokio se ejecute tanto de forma nativa como en navegadores web.

¡Gracias a [Kim Dong-Hyun](https://users.rust-lang.org/t/crate-of-the-week/2704/1654) por la autosugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización.

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas.

##### [Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Problema de seguimiento para '-zembed-metadata'](https://github.com/rust-lang/cargo/issues/15495)

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
* [sysknife - Exportar las filas de la cadena de auditoría, no solo el veredicto de verificación](https://github.com/lacs-project/sysknife/issues/215)
* [sysknife - Exponer las acciones de solo lectura como herramientas MCP sin exponer AptUpdate](https://github.com/lacs-project/sysknife/issues/216)
* [sysknife - Grabar una validación atómica actual de Fedora](https://github.com/lacs-project/sysknife/issues/217)
* [YantrikDB - Migrar los 7 sitios manuales restantes de SAVEPOINT a SavepointGuard (agujero de desenrollar en pánico + 7 copias enrolladas a mano de la regla de desenrollar)](https://github.com/yantrikos/yantrikdb/issues/100)
* [RustAPI - tarea: plantillas de problemas, etiqueta de triaje faltante, MSRV 1.85 (fácil)](https://github.com/Tuntii/RustAPI/issues/261)
* [KayaDB - prueba: un caso extra de WAL malformado / decodificador de trama de comando (fácil)](https://github.com/Tuntii/KayaDB/issues/46)
* [Cordial - GameActivity.getWaterfallInsets tiene el descriptor JNI incorrecto](https://github.com/luohoa97/cordial/issues/11)
* [Cordial - ro.soc.manufacturer se responde con una cadena vacía](https://github.com/luohoa97/cordial/issues/12)
* [Cordial - Mapa de qué canales FLog toman un número y cuáles un nombre de severidad](https://github.com/luohoa97/cordial/issues/13)
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

613 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-11..2026-08-18

#### Compilador
* [en línea con algunas funciones nuevas de resolución calientes](https://github.com/rust-lang/rust/pull/160892)
* [estabilizar '-Zprofile-sample-use'](https://github.com/rust-lang/rust/pull/155942)
* [estabilizar 'extern "custom"'](https://github.com/rust-lang/rust/pull/158504)
* [hacer que 'ShardedHashMap::with_capacity' se reparta la capacidad entre fragmentos](https://github.com/rust-lang/rust/pull/161127)
* [tres aceleraciones de nuevo solucionador](https://github.com/rust-lang/rust/pull/160605)
* [usar 'TyOrConstInferVar' en el siguiente solucionador, corregir #158441](https://github.com/rust-lang/rust/pull/158436)

#### Biblioteca
* [añadir 'núcleo::num::Complejo'](https://github.com/rust-lang/rust/pull/158885)
* ['Arc': eliminar el uso innecesario de 'fmt::D isplay' para la afirmación de desbordamiento](https://github.com/rust-lang/rust/pull/160731)
* [núcleo/num: implementar la función 'float_nan_to'](https://github.com/rust-lang/rust/pull/161250)
* [núcleo: implementar 'FusedIterator' para 'StepBy'](https://github.com/rust-lang/rust/pull/159963)
* [experimento: añadir 'núcleo::cmp::más pequeño' y 'núcleo::cmp::más grande'](https://github.com/rust-lang/rust/pull/160687)
* ['Iterador::{min,max}(_by_key)' debe usar 'min'/'max'/'lt'](https://github.com/rust-lang/rust/pull/160203)
* [macro funcional tipo 'offload!'](https://github.com/rust-lang/rust/pull/161055)
* [optimizar las comprobaciones de límites de corte de respaldo de CStr](https://github.com/rust-lang/rust/pull/161040)
* [buscador ASCII de un solo byte para 'StrSearcherImpl(pattern.rs)'](https://github.com/rust-lang/rust/pull/160408)

#### Carga
* ['trim-paths': Honor Workspace Prefijo Override desde env](https://github.com/rust-lang/cargo/pull/17349)
* ['frontmatter': No te asustes en una valla corta antes de un personaje que no es ASCII](https://github.com/rust-lang/cargo/pull/17274)
* ['min-public-age': eliminar 'registro.min-publish-age'](https://github.com/rust-lang/cargo/pull/17353)
* [activar '-Zembed-metadata=no' por defecto en Carga nocturna](https://github.com/rust-lang/cargo/pull/17267)
* [re-estabilizar el diseño del director-construcción v2](https://github.com/rust-lang/cargo/pull/17354)
* [eliminar el archivo de desreasignación al ejecutar carga limpia -p en la nueva disposición de build-dir](https://github.com/rust-lang/cargo/pull/17356)

#### Rustdoc
* [añadir soporte básico de 'splat' a 'rustdoc'](https://github.com/rust-lang/rust/pull/160882)
* [añadir nueva pelusa de 'unused_footnote_definition' en el doc de Rust](https://github.com/rust-lang/rust/pull/137858)
* [también avisar si se usa un atributo 'doc' inválido en una invocación de macro](https://github.com/rust-lang/rust/pull/161003)

#### Clippy
* [añadir pelusa de 'option_zip_none'](https://github.com/rust-lang/rust-clippy/pull/17465)
* [limpieza 'used_underscore_*'](https://github.com/rust-lang/rust-clippy/pull/17308)
* [arreglar ICE en 'unnecessary_rest_pattern' para TyAlias](https://github.com/rust-lang/rust-clippy/pull/17557)
* [corregir 'unfulfilled_lint_expectations' activado incorrectamente por '#[expect(clippy::let_and_return)]'](https://github.com/rust-lang/rust-clippy/pull/17045)
* [corregir diagnósticos duplicados para 'min_rust_version_invalid_attr'](https://github.com/rust-lang/rust-clippy/pull/17396)
* [PERF: Consulta el tipo FN antes de la expansión en 'missing_const_for_thread_local'](https://github.com/rust-lang/rust-clippy/pull/17581)
* [perf: resuelve el llamado antes de la expansión en 'VecArgs::hir'](https://github.com/rust-lang/rust-clippy/pull/17582)
* [PERF: corre 'in_external_macro' tras los cheques baratos en cinco caminos de pelusa caliente](https://github.com/rust-lang/rust-clippy/pull/17276)

#### Analizador de Rust
* [analizador: ruta de error frontmatter para UTF-8](https://github.com/rust-lang/rust-analyzer/pull/23159)
* [evitar el pánico por parámetros de tipo asociados desajustados](https://github.com/rust-lang/rust-analyzer/pull/23118)
* [comprobar el tipo original para 'replace_arith_op'](https://github.com/rust-lang/rust-analyzer/pull/22225)
* [considera que el bucle que contiene 'break expr' diverge si 'expr' está divergiendo](https://github.com/rust-lang/rust-analyzer/pull/23127)
* [no entrar en pánico cuando esté definido en macro desde la entrada](https://github.com/rust-lang/rust-analyzer/pull/23122)
* [no te equivoques en la coma de cola para alguna macro](https://github.com/rust-lang/rust-analyzer/pull/23134)
* [emitir E0600 cuando se aplica unario '!'/'-' al tipo no soportado](https://github.com/rust-lang/rust-analyzer/pull/23147)
* [cada espacio de trabajo debería tener un servidor proc-macro](https://github.com/rust-lang/rust-analyzer/pull/23111)
* [corregir soporte de 'rustc_private' para 'rustc_proc_macro'](https://github.com/rust-lang/rust-analyzer/pull/23140)
* [Expresiones de rango inferior en HiR bajando](https://github.com/rust-lang/rust-analyzer/pull/23115)
* [devolver una const de error al solucionador cuando falla la constevell](https://github.com/rust-lang/rust-analyzer/pull/23138)
* [ofrece 'replace_arith' sobre referencias a ints](https://github.com/rust-lang/rust-analyzer/pull/23109)
* [soportan derivados integrados Reborrow y CoerceShared](https://github.com/rust-lang/rust-analyzer/pull/22325)

### Triaje de rendimiento del compilador Rust

Casi no hubo regresiones esta semana, mientras que el siguiente solucionador de rasgos mostró varios resultados significativos
¡Mejoras!

Triaje hecho por **@kobzol**.
Rango de revisión: [771916f9.. 8FA1c96C](https://perf.rust-lang.org/?start=771916f9028e7fe56d2685f2c4f698de5d7d6a45&end=8fa1c96cfd489e4c27654c144ae871ce2c4db6c6&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,2%, 0,5%] | 6 |
| Regresiones ❌ <br /> (secundario) | 0,6% | [0,2%, 1,0%] | 17 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-1,7%, -0,2%] | 166 |
| Mejoras ✅ <br /> (secundario) | -2,3% | [-16,0%, -0,1%] | 219 |
| Todos ❌✅ (primario) | -0,5% | [-1,7%, 0,5%] | 172 |

0 regresiones, 6 mejoras, 7 mixtas; 4 de ellos en rollups
50 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/ca70287715cb2c2b10aed04506acb0ee5574c3fe/triage/2026/2026-08-18.md).

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [RFC: Refactorizar el equipo liberal](https://github.com/rust-lang/rfcs/pull/3984)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Siempre escape de los extensores de grafema en 'str::escape_debug'](https://github.com/rust-lang/rust/pull/158303)
* [feat: añadir Ecuaciones Parciales simétricas para Vec, &[T], &mut [T] frente a Vaca<'_, [T]>](https://github.com/rust-lang/rust/pull/156160)
* [estabilizar funciones inteligentes de mapeo de puntero](https://github.com/rust-lang/rust/pull/160534)
* [Estabilizar 'windows_process_extensions_main_thread_handle'](https://github.com/rust-lang/rust/pull/160108)
* [Añadir implementación 'por defecto' para 'std::sync::Once'](https://github.com/rust-lang/rust/pull/160136)
* [target_features: sse (o al menos avx2) es incompatible con soft-float ABI](https://github.com/rust-lang/rust/pull/160302)
* [Quitar 'De<!> para T' *reserva* impl](https://github.com/rust-lang/rust/pull/160705)
* [estabilizar 'Box::take'](https://github.com/rust-lang/rust/pull/160436)
* [Extiende 'dropping_{references,copy_types}' lints a 'drop_in_place'](https://github.com/rust-lang/rust/pull/160229)
* [pelusa en usos más incorrectos de 'core::ffi::c_void'](https://github.com/rust-lang/rust/pull/159986)
* [Hacer que let-else respete macro_rules agrupación de metavariables expr](https://github.com/rust-lang/rust/pull/158515)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [dote(resolver): Estabilizar min-publicación-edad](https://github.com/rust-lang/cargo/pull/17335)
* [feat(diag): Estabilizar los lints de carga ](https://github.com/rust-lang/cargo/pull/17298)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Expone 'target_abi = "v8plus"' en sparc-unknown-linux-gnu](https://github.com/rust-lang/compiler-team/issues/1028)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[RFCs de Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [Cambiar 'i686-pc-windows-msvc' de Tier 1 con herramientas de host => Tier 1 sin herramientas de host](https://github.com/rust-lang/rfcs/pull/3999)
* [Propietarios de referencias ('& propia T')](https://github.com/rust-lang/rfcs/pull/4000)

<!-- Mensaje de Llamada para Pruebas (publicar en GH 'problema' y eliminar la etiqueta 'llamada para pruebas') -->
Este RFC aparecerá en la sección **Llamada para Pruebas** del próximo número (#) de This Week in Rust (TWiR).
Puedes eliminar la etiqueta de 'llamada para pruebas'.  Por favor, siéntete libre de dejar la etiqueta 'llamada para pruebas' si quieres que este RFC vuelva a aparecer en otro número de TWiR.

## Próximos eventos

Eventos Rusty entre el 19-08-2026 y el 16-09-2026 🦀

### Virtual
* 2026-08-19 | Híbrido (Vancouver, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, EE. UU.) | [Encuentro de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Parte #5 — Comunicación inalámbrica con el protocolo IEEE 802.15.4**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sesión semanal de codificació / Sesión semanal de codificación**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254775/)
* 2026-08-26 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Lotería y Programación Multi-CPU**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316083375/)
* 27-08-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxid**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/arkkrcj5)
* 2026-08-31 | Virtual | [Experta del Rust 🦀](https://luma.com/rust-maven)
    * [**Workshop: Añadir pruebas a un proyecto Rust de código abierto**](https://luma.com/nwfmsdtf)
* 2026-09-01 | Virtual | [Experta del Rust 🦀](https://luma.com/rust-maven)
    * [**Tauri: Aplicaciones de escritorio multiplataforma con Rust y tecnologías web**](https://luma.com/d9w26vav)
* 2026-09-02 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-02 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/316107210/)
* 2026-09-04 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Sesión semanal de codificación**](https://luma.com/sqf4ux01)
* 2026-09-06 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316133872/)
* 2026-09-08 - 2026-09-11 | Híbrido (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-08 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315859305/)
* 2026-09-10 | Virtual | [Experta del Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/314233757/)

### África
* 2026-09-08 | Johannesburgo, ZA | [Encuentro de Johannesburgo Rust](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Biblioteca estándar extendida de Rust**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Rustacean de agosto 2026**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Encuentro de Rust Delhi X SciPy India**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Computación científica en Rust y pitón**](https://scipy.in/sci-py-rs/)
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: agosto 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europa
* 2026-08-20 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**¡Fiesta de Verano Oxid!**](https://www.meetup.com/rust-berlin/events/316151073/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Construcción de una cámara acústica con egui y embajada**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 2026-08-21 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/316115136/)
* 2026-08-26 | Dresde, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Tercer encuentro**](https://pretix.eu/rust-dresden/on-location-3)
* 27-08-2026 | Manchester, Reino Unido | [Manchester Rust](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)
* 2026-08-29 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #29**](https://www.meetup.com/stockholm-rust/events/316130996/)
* 2026-09-08 | París, FR | [París Rust](https://www.meetup.com/rust-paris)
    * [**Reunión de Rust #87**](https://www.meetup.com/rust-paris/events/316169040/)
* 2026-09-14 - 2026-09-16 | Berlín, DE | [Oxidar 2026](https://oxidizeconf.com/)
    * [**Oxidar 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproduciendo artículos científicos - con Rust & "IA"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)

### Norteamérica
* 2026-08-19 | Híbrido (Vancouver, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Lidiando con Dependencias**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-19 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Encuentro de Agosto de Rust en el Área de la Bahía**](https://luma.com/00f2s7q9)
* 2026-08-20 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de agosto de 2026 con SRUG (Seattle Rust User Group)](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315929355/)
* 2026-08-20 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: 'Los datos moldean tu memoria' y 'Oxida en paz'](https://www.meetup.com/rust-nyc/events/316056830/)
* 2026-08-26 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA August! Rust en la computación cuántica**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 27-08-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316124372/)
* 2026-09-03 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Criptografía + Ordenadores Cuánticos**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 - 2026-09-11 | Híbrido (Montreal, CA) | [RustConf](https://rustconf.com/)
    * [**RustConf**](https://rustconf.com/)
* 2026-09-08 | Montreal, QC, CA | [Cimiento de Rust](https://rustfoundation.org/)
    * [**Cumbre de Salud de los Equipos Rust**](https://rustfoundation.org/event/rust-teams-health-summit/)
* 2026-09-08 - 2026-09-11 | Montreal, QC, CA | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026, organizado por la Fundación Rust**](https://rustconf.com/schedule/)
* 2026-09-09 | Montreal, CA | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Encuentro de pausa café de RustConf**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 2026-09-15 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | Híbrido (Vancouver, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Estudio de Rust/Hack/Encuentro**](https://www.meetup.com/vancouver-rust/events/314233757/)

### Oceanía
* 27-08-2026 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne agosto 2026**](https://www.meetup.com/rust-melbourne/events/315039490/)
* 27-08-2026 | Melbourne, AU | [Rust Melbourne](https://luma.com/rustmelbourne)
    * [**Rust Melbourne Meetup**](https://luma.com/d0rndgyv)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién Contrata en r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> lío es el máximo

– [Clar Fon sobre la discusión de rust-zulip "suposiciones sobre carpetas" abreviaturas](https://rust-lang.zulipchat.com/#narrow/channel/326132-t-types.2Fmeetings/topic/2026-08-11/near/615874481)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1790) por la sugerencia!

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

<small>[Comenta en r/rust](https://www.reddit.com/r/rust/comments/1vt8nni/this_week_in_rust_665/)</small>
