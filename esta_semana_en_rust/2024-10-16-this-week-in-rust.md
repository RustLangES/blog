---
title: "Esta semana en Rust #32"
number_of_week: 32
description: El crate de esta semana es bacon, una aplicación de terminal para ejecutar sus tareas de carga en segundo plano.
date: 2024-10-16
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [por favor envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y simplemente pide a los editores que seleccionen la categoría. -->

### Fundación
* [Anuncio de los becarios 2024 de la Fundación Rust](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2024-fellows/)

### Boletines
* [Esta semana en Bevy - Eventos de animación, curvas y no_std](https://thisweekinbevy.com/issue/2024-10-14-animation-events-curves-and-nostd)

### Actualizaciones de proyectos/herramientas
* [Sobre Rust en los kernels empresariales](https://lwn.net/SubscriberLink/993337/68c7e5af573545e6/)
* [Discrepancias de tipo FFI en Rust para Linux](https://lwn.net/SubscriberLink/993163/3c2b06af07814cd2/)
* [Lanzando punteros de la nada](https://lwn.net/SubscriberLink/993484/385b176aa8939e7b/)
* [Uso de la energía atómica LKMM en Rust](https://lwn.net/SubscriberLink/993785/cbe8cf5846d6864a/)
* ["pigg" (la GUI GPIO de Raspberry Pi) 0.4.0 lanzado](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.4.0)

### Observaciones/Pensamientos
* [Por qué 'Pin' es parte de las firmas de rasgos (y por qué eso es un problema)](https://blog.yoshuawuyts.com/why-pin/)
* [El rasgo 'Sobrescribir' y 'Pin'](https://smallcultfollowing.com/babysteps/blog/2024/10/14/overwrite-and-pin)
* [Mejorar el rendimiento de un algoritmo paso a paso](https://blog.mapotofu.org/blogs/rabitq-bench/)
* [Reemplazando nginx por axum](https://felix-knorr.net/posts/2024-10-13-replacing-nginx-with-axum.html)
* [Un experimento en Rust asíncrono](https://ochagavia.nl/blog/an-experiment-in-async-rust/)
* [Diseño de una tabla hash concurrente rápida](https://ibraheem.ca/posts/designing-papaya/)
* [Repensar a los constructores... con Genéricos Perezosos](https://geo-ant.github.io/blog/2024/rust-rethinking-builders-lazy-generics/)
* [CIP en Rust](https://pranitha.rs/posts/rust-ipc-ping-pong/)
* [Rasgo de Serde - Parte 3: Deserialización](https://voelklmichael.github.io/Blog/serde-trait-part3.html)
* [Memoria para nada: Por qué Vec&lt;ussize&gt; es (probablemente) una mala idea](https://pwy.io/posts/memory-for-nothing/)
* [Actualice el registro en sus pruebas de Rust](https://tylerjw.dev/posts/20241012-rust-logging-in-tests/)
* [Nueve reglas para ejecutar Rust en sistemas integrados](https://medium.com/towards-data-science/nine-rules-for-running-rust-on-embedded-systems-b0c247ee877e)
* [Por qué Rust está arrasando en el mundo de la ingeniería de datos](https://kerkour.com/rust-data-engineering)

### Tutoriales de Rust
* [Lectura de un archivo remoto usando Rust](https://crustc.com/reading-a-remote-file-rust/)
* [video] [Construir con Naz : Dirección y tamaño de memoria de Rust](https://www.youtube.com/watch?v=ivqIty5EOf8)

## Crate de la semana

El crate de esta semana es [bacon](https://dystroy.org/bacon), una aplicación de terminal para ejecutar sus tareas de carga en segundo plano.

¡Gracias a [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1351) por la autosugestión!
[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*Esta semana no se han presentado convocatorias para participar.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 468 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-08..2024-10-15

* [literales de cadena protegidos de reserva](https://github.com/rust-lang/rust/pull/123951) (RFC [#3593](https://rust-lang.github.io/rfcs/3593-unprefixed-guarded-strings.html))
* [ABI: pasar agregados por valor en AIX](https://github.com/rust-lang/rust/pull/131208)
* ['codegen_ssa': consolidar cheques de objetivos vinculados](https://github.com/rust-lang/rust/pull/130308)
* ['rustc_target': Añadir sme-b16b16 como una característica de destino explícita de aarch64](https://github.com/rust-lang/rust/pull/130741)
* [añadir '&pin (mut|const) T' posición de tipo azúcar](https://github.com/rust-lang/rust/pull/130635)
* [añadir sugerencia para eliminar la ruta inválida sep ':': en fn def](https://github.com/rust-lang/rust/pull/130870)
* [también use const-anon más externo para elementos impl en 'non_local_defs' lint](https://github.com/rust-lang/rust/pull/131660)
* [comprobar la compatibilidad del objetivo ABI para punteros de función](https://github.com/rust-lang/rust/pull/128784)
* [No asumas que los rasgos utilizados como tipo son objetos de rasgo en la edición de 2021](https://github.com/rust-lang/rust/pull/131239)
* [no haga ICE cuando encuentre un error de ciclo de diseño post-mono](https://github.com/rust-lang/rust/pull/131420)
* [emitir un error para atributos inestables que hacen referencia a características ya estables](https://github.com/rust-lang/rust/pull/131567)
* [corregir 'clobber_abi' y no permitir registros relacionados con SVE en el ensamblaje en línea Arm64EC](https://github.com/rust-lang/rust/pull/131332)
* [mejorar los mensajes de error para las funciones 'C-cmse-nonsecure-entry'](https://github.com/rust-lang/rust/pull/130747)
* [introducir la relación de tipo SolverRelating con el nuevo solucionador](https://github.com/rust-lang/rust/pull/131263)
* [hacer la sugerencia de 'unused_parens' considerando los atributos de expr](https://github.com/rust-lang/rust/pull/131546)
* [mover la lógica de confirmación ficticia a 'x86_64-gnu-llvm-18'](https://github.com/rust-lang/rust/pull/131531)
* [mover la polaridad a 'PolyTraitRef' en lugar de almacenarla en un lado](https://github.com/rust-lang/rust/pull/131652)
* [Captura precisa en rasgos](https://github.com/rust-lang/rust/pull/131033)
* [coincidir superficialmente con la clave opaca en el almacenamiento](https://github.com/rust-lang/rust/pull/131599)
* [soporte 'clobber_abi' en el ensamblaje en línea MSP430](https://github.com/rust-lang/rust/pull/131310)
* [suprimir errores de importación para rasgos que podrían haberse aplicado para el error de búsqueda de métodos](https://github.com/rust-lang/rust/pull/131702)
* [use throw intrinsic from stdarch en wasm libunwind](https://github.com/rust-lang/rust/pull/131418)
* [miri: no almacenar los identificadores primitivos de sincronización en la memoria direccionable](https://github.com/rust-lang/miri/pull/3966)
* [miri: asegúrese de que un 'os_unfair_lock' de macOS que se mueve mientras se sostiene no se desbloquea implícitamente](https://github.com/rust-lang/miri/pull/3973)
* [Miri: Evento de epoll agregando: No hay necesidad de unirse, no hay reloj viejo aquí](https://github.com/rust-lang/miri/pull/3961)
* [miri: arreglar el comportamiento de 'release_clock()'](https://github.com/rust-lang/miri/pull/3951)
* [Miri: Arreglar la sincronización de epoll](https://github.com/rust-lang/miri/pull/3946)
* [Miri: Se corrigió el impl 'pthread_getname_np' para glibc](https://github.com/rust-lang/miri/pull/3953)
* [miri: se corrigieron las implementaciones de nombres de hilos get/set para macOS y FreeBSD](https://github.com/rust-lang/miri/pull/3957)
* [Miri: usa el nuevo ayudante 'check_min_arg_count' en más lugares](https://github.com/rust-lang/miri/pull/3974)
* [Obligaciones imposibles vía rápida](https://github.com/rust-lang/rust/pull/131491)
* [eliminar ordenaciones innecesarias en 'rustc_hir_analysis'](https://github.com/rust-lang/rust/pull/131328)
* [estabilizar 'const_char_encode_utf8'](https://github.com/rust-lang/rust/pull/131463)
* [estabilizar 'const_make_ascii'](https://github.com/rust-lang/rust/pull/131496)
* [estabilizar 'Pin::as_deref_mut()'](https://github.com/rust-lang/rust/pull/129424)
* [Estabilizar la prueba 'ci_rustc_if_unchanged_logic'](https://github.com/rust-lang/rust/pull/131444)
* [estabilizar 'const_option'](https://github.com/rust-lang/rust/pull/131120)
* [estabilizar 'const_result'](https://github.com/rust-lang/rust/pull/131287)
* [estabilizar 'debug_more_non_exhaustive'](https://github.com/rust-lang/rust/pull/131109)
* [estabilizar 'duration_consts_float'](https://github.com/rust-lang/rust/pull/131289)
* [estabilizar const 'ptr::write*' y 'mem::replace'](https://github.com/rust-lang/rust/pull/130954)
* [estabilizar const '{slice,array}::from_mut'](https://github.com/rust-lang/rust/pull/130538)
* [optimizar 'escape_ascii' usando una tabla de búsqueda](https://github.com/rust-lang/rust/pull/125679)
* [migrar '&Option' de lib<T>' a 'Option<&T>'](https://github.com/rust-lang/rust/pull/130962)
* [intrínsecos fmuladdf{32,64}: exponer llvm.fmuladd.* semántica](https://github.com/rust-lang/rust/pull/124874)
* [añadir '#[track_caller]' a la asignación de métodos de 'Vec' y 'VecDeque'](https://github.com/rust-lang/rust/pull/126557)
* [core/net: add 'Ipv[46]Addr::from_octets, Ipv6Addr::from_segments'](https://github.com/rust-lang/rust/pull/130629)
* [desacoplar los zócalos WASIp2 de WasiFd](https://github.com/rust-lang/rust/pull/131449)
* [se corrige el error donde 'option_env!' devolvería 'None' cuando env var está presente pero no es Unicode válido](https://github.com/rust-lang/rust/pull/122670)
* [implementado 'FromStr' para 'CString' y 'TryFrom<CString>' para 'String'](https://github.com/rust-lang/rust/pull/130608)
* [biblioteca: const-stabilize 'MaybeUninit::assume_init_mut'](https://github.com/rust-lang/rust/pull/131274)
* [std: arreglar stdout-before-main](https://github.com/rust-lang/rust/pull/131233)
* [hashbrown: ci: prueba el MSRV con versiones de dependencia mínimas](https://github.com/rust-lang/hashbrown/pull/572)
* [hashbrown: revertir "feat: borsh serde"](https://github.com/rust-lang/hashbrown/pull/570)
* [cargo: complete: No completes archivos por ningún valor](https://github.com/rust-lang/cargo/pull/14653)
* [cargo: git: don't fetch tags by default](https://github.com/rust-lang/cargo/pull/14688)
* [cargo: resolver: compartir caché de conflictos entre reintentos de activación](https://github.com/rust-lang/cargo/pull/14692)
* [cargo: agregar más pruebas de resolución SAT](https://github.com/rust-lang/cargo/pull/14614)
* [cargo: docs: las herramientas solo deben interpretar una línea que comience con '{' como JSON](https://github.com/rust-lang/cargo/pull/14677)
* [Cargo: feat: agregar completador personalizado para completar el nombre del registro](https://github.com/rust-lang/cargo/pull/14656)
* [Cargo: Arreglar el pánico al ejecutar el árbol de carga en un paquete con un bindep compilado cruzado](https://github.com/rust-lang/cargo/pull/14593)
* [cargo: fix: evite insertar 'dylib_path_envvar' duplicado al llamar a 'cargo run' recursivamente](https://github.com/rust-lang/cargo/pull/14464)
* [Carga: mejorar la velocidad del resolutor](https://github.com/rust-lang/cargo/pull/14663)
* [cargo: versión inicial de frescura basada en suma de comprobación](https://github.com/rust-lang/cargo/pull/14137)
* [cargo: eliminar el soporte para 'Cargo.toml' del script-cargo](https://github.com/rust-lang/cargo/pull/14670)
* [cargo: admite opciones de selección de paquetes como '--exclude' en 'cargo publish'](https://github.com/rust-lang/cargo/pull/14659)
* [rustdoc-json: cambiar el repr del ID del artículo de una cadena a un int](https://github.com/rust-lang/rust/pull/130078)
* [rustdoc: añadir espacio entre los campos 'struct' y sus descripciones](https://github.com/rust-lang/rust/pull/131394)
* [rustfmt: sugerencia de qué 'style_edition' usar en lugar de versión](https://github.com/rust-lang/rustfmt/pull/6361)
* [clippy: 'implicit_saturating_sub': sugerencia de corrección con un enfoque menos volátil](https://github.com/rust-lang/rust-clippy/pull/13542)
* [clippy: 'module_name_repetitions': no avisa si el artículo está en un módulo privado](https://github.com/rust-lang/rust-clippy/pull/13444)
* [clippy: añade pelusa 'manual_ignore_cast_cmp'](https://github.com/rust-lang/rust-clippy/pull/13334)
* [clippy: marque el recuento de argumentos 'MethodCall'/'Call' antes o en absoluto](https://github.com/rust-lang/rust-clippy/pull/13540)
* [Clippy: ¡Comprueba si hay cadenas sin procesar innecesarias en 'format_args! ()' también](https://github.com/rust-lang/rust-clippy/pull/13504)
* [clippy: no advertir sobre el código generado por la macro proc en 'needless_return'](https://github.com/rust-lang/rust-clippy/pull/13464)
* [clippy: arreglar la activación de 'large_stack_arrays' al anidar elementos const](https://github.com/rust-lang/rust-clippy/pull/13534)
* [clippy: arreglar la pelusa 'manual_slice_size_calculation' cuando un corte es referenciado más de una vez](https://github.com/rust-lang/rust-clippy/pull/13487)
* [clippy: arreglar el problema del lapso en 'implicit_saturating_sub'](https://github.com/rust-lang/rust-clippy/pull/13533)
* [clippy: marque 'unnecessary_first_then_check' y 'byte_char_slices' como 'Aplicable'](https://github.com/rust-lang/rust-clippy/pull/13537)
* [clippy: mover 'clippy::module_name_repetitions' a 'restricción' (de 'pedante')](https://github.com/rust-lang/rust-clippy/pull/13541)
* [clippy: mover 'too_long_first_doc_paragraph' a 'guardería'](https://github.com/rust-lang/rust-clippy/pull/13551)
* [clippy: solo emitir 'manual_c_str_literals' en ≥ Edición 2021](https://github.com/rust-lang/rust-clippy/pull/13532)
* [clippy: use el prefijo std/core correcto en la salida de lint](https://github.com/rust-lang/rust-clippy/pull/13454)
* [rust-analyzer: añadir soporte para LLDB-DAP](https://github.com/rust-lang/rust-analyzer/pull/18265)
* [rust-analyzer: analizar correctamente 'use' en parámetros genéricos](https://github.com/rust-lang/rust-analyzer/pull/18226)
* [rust-analyzer: no considere match/let/ref of place that evaluate to ! para divergir, no permita coerciones de ellos también](https://github.com/rust-lang/rust-analyzer/pull/18278)
* [rust-analyzer: manejar el auto-parámetro fuera de los métodos al cambiar el nombre](https://github.com/rust-lang/rust-analyzer/pull/18292)
* [Rust-analyzer: resaltar los puntos de salida de los bloques asíncronos](https://github.com/rust-lang/rust-analyzer/pull/18152)
* [rust-analyzer: respeta 'references.exclude_tests' en la jerarquía de llamadas](https://github.com/rust-lang/rust-analyzer/pull/18291)
* [Rust-analyzer: Soluciona el pánico cuando el proyecto JSON tiene rutas de archivo de compilación relativas](https://github.com/rust-lang/rust-analyzer/pull/18289)
* [Rust-analyzer: Comentar las comprobaciones de lanzamiento para un tipo de RPP desconocido](https://github.com/rust-lang/rust-analyzer/pull/18217)
* [rust-analyzer: no consideres el uso mutable de deref a '*mut T' como 'deref_mut'](https://github.com/rust-lang/rust-analyzer/pull/18252)
* [rust-analyzer: arreglar 'prettify_macro_expansion()' cuando el nodo no es el archivo completo](https://github.com/rust-lang/rust-analyzer/pull/18246)
* [Rust-analyzer: incluya la descripción en los detalles de la etiqueta cuando el campo de detalle esté marcado para ...](https://github.com/rust-lang/rust-analyzer/pull/18245)
* [rust-analyzer: autofix incorrecto para la unidad envuelta faltante en la devolución expr](https://github.com/rust-lang/rust-analyzer/pull/18299)
* [rust-analyzer: unir rustfmt overrideCommand con la raíz del proyecto](https://github.com/rust-lang/rust-analyzer/pull/18229)
* [rust-analyzer: hir-ty: cambiar el formato del constructor de variantes 'struct' + 'enum'](https://github.com/rust-lang/rust-analyzer/pull/18269)
* [rust-analyzer: lsp: arreglar 'completion_item something_to_resolve' que no es un pestillo para verdadero](https://github.com/rust-lang/rust-analyzer/pull/18247)
* [rust-analyzer: ejecutar subprocesos asíncronos en la extensión vscode](https://github.com/rust-lang/rust-analyzer/pull/18281)
* [rust-analyzer: omitir la expansión '#[test_case]']](https://github.com/rust-lang/rust-analyzer/pull/18275)

### Clasificación del rendimiento del compilador de Rust

No hay grandes cambios esta semana.

Triaje realizado por **@simulacrum**.
Rango de revisión: [e6c46db4.. 5ceb623a](https://perf.rust-lang.org/?start=e6c46db4e9fd11e3183c397a59d946731034ede6&end=5ceb623a4abd66e91e7959d25caaf0523f1a7f7c&absolute=false&stat=instructions%3Au)

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-10-14.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *No se aprobaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [estabilizar las API de procedencia estricta y procedencia expuesta](https://github.com/rust-lang/rust/pull/130350)
* [disposición: fusionar] [hacer unsupported_calling_conventions un error grave](https://github.com/rust-lang/rust/pull/129935)
* [disposición: fusionar] [Decidir el nombre de 'derivar(SmartPtr)'](https://github.com/rust-lang/rust/issues/129104)
* [disposición: fusionar] [Estabilizar 'shorter_tail_lifetime'](https://github.com/rust-lang/rust/issues/131445)
* [disposición: fusionar] [Estabilización de fin de 'result_ffi_guarantees'](https://github.com/rust-lang/rust/pull/130628)
* [disposición: fusionar] [Implementar restricciones ergonómicas para partidos de edición 2024](https://github.com/rust-lang/rust/pull/131381)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: No (opsem) Cajas Mágicas](https://github.com/rust-lang/rfcs/pull/3712)
* [nuevo] ['#[en línea(obligatorio)]'](https://github.com/rust-lang/rfcs/pull/3711)
* [nuevo] [RFC: introducir el patrón de diseño sintáctico de sabor](https://github.com/rust-lang/rfcs/pull/3710)

## Próximos eventos

Eventos oxidados entre 2024-10-16 - 2024-11-13 🦀

### Virtual
* 16/10/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 17/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Parte 3 de 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 17/10/2024| Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
* 17/10/2024 | Mountain View, CA, EE. UU. | [Encuentro de Rust en Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/303987275/)
* 22/10/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fkmcntygcnbdc/)
* 24/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Parte 4 de 4 - Hackathon Showcase: Proyectos Finales y Presentaciones**](https://www.meetup.com/women-in-rust/events/303213835/)
* 24/10/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633271/)
* 26/10/2024 | Virtual (Gdansk, PL) | [Stacja IT trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 29/10/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/301585671/)
* 31/10/2024| Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Elaborando intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 31/10/2022 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 07/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633272/)
* 12/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346985/)

### África
* 02/11/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 17/10/2024 - 18/10/2024 | Pekín, CN | [Encuentro Mundial de Innovación de Código Abierto (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 19/10/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de octubre de 2024**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europa
* 16/10/2024 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 17/10/2024 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/303792888/)
* 17/10/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 17/10/2024 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #4**](https://www.meetup.com/rust-gdansk/events/303370455/)
* 22/10/2024 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London x London Python: *ENTRADAS AGOTADAS***](https://www.meetup.com/rust-london-user-group/events/303922750/)
* 22/10/2024 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/)
    * [**New Rust Warsaw Meetup #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 26/10/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Foro Fika de Ferris #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 28/10/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 29/10/2024 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 30/10/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Octubre 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 31/10/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/300820289/)
* 06/11/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 06/11/2024 | París, FR | [Rustáceos de París](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)

### América del Norte
* 16/10/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 17/10/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/303919296/)
* 17/10/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Encuentro de octubre**](https://www.meetup.com/join-srug/events/303545170/)
* 19/10/2024 | Cambridge, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de North End Rust, 19 de octubre**](https://www.meetup.com/bostonrust/events/303708335/)
* 23/10/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 27/10/2024 | Cambridge, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, 27 de octubre**](https://www.meetup.com/bostonrust/events/303708359/)
* 28/10/2024 | Boulder, CO, EE. UU. | [Encuentro de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**Genéricos desde la base**](https://www.meetup.com/boulder-rust-meetup/events/303766925/)
* 28/10/2024 | Ferndale, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/303909299/)
* 28/10/2024 | Minneapolis, MN Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/303884468/)
* 29/10/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Estado del grupo y expectativas para 2025**](https://www.meetup.com/music-city-rust-developers/events/301425524/)
* 04/11/2024 | Brookline, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust en Coolidge Corner Brookline, 4 de noviembre**](https://www.meetup.com/bostonrust/events/303708387/)
* 06/11/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031651/)
* 07/11/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Desarrollo de juegos con Rust y el motor Bevy**](https://www.meetup.com/stl-rust/events/302371464/)
* 12/11/2024 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)

### Oceanía
* 28/10/2024 | Melbourne, VIC, Australia | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**Encuentro de Rust Melbourne de octubre de 2024**](https://www.meetup.com/rust-melbourne/events/304034898/)
* 29/10/2024 | Canberra, ACT, AU | [Grupo de usuarios de Canberra Rust (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/303128131/)
* 31/10/2024 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Rust on AWS: Sustainability + Peace: Zero Stress Automation**](https://www.meetup.com/rust-akl/events/303824919/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1fa0tf6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Tendríamos botones en la pantalla para controlar los ventiladores del coche. Tuve que escribir un montón de código antes de poder compilarlo todo, una gran torre de jenga. Pero una vez que se compiló, ¡los fanáticos comenzaron a trabajar! Muy impresionado.

– [Julius Gustavsson en el blog de Tweedegolf](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line)

¡Gracias a [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1619) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1g5iqq8/this_week_in_rust_569/)</small>
