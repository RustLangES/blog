---
title: "Esta semana en Rust #46"
number_of_week: 46
description: La comunidad  de esta semana es embed\_it, una crate que te ayuda  incrustar activos en tu binario y generar implementaciones de struct/trait para archivo o directorio.
date: 2025-01-29
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenido a otro número de *esta semana en Rust*! [Rust](https://www.rust-lang.org/)
en lenguaje programación que permite todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o envíanos un [solicitud extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan la contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

Esta semana en Rust se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición en esta semana, [por favor envía un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja e entrada? [Suscríbet aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad en Rust 🥰

### Oficial
* [Rust 2024 en el canal beta | Blog de Rust](https://blog.rust-lang.org/2025/01/22/rust-2024-beta.html)
* [Actualización de los Objetivos del Proyecto de Diciembre | Blog de Rust](https://blog.rust-lang.org/2025/01/23/Project-Goals-Dec-Update.html)

### Fundación
* [Explicación de la actualización de la política de marca comercial de Rust Language](https://rustfoundation.org/media/rust-language-trademark-policy-updates-explained/)

### Actualizaciones de proyectos/herramientas
* [La edición 2024 de The Rust toma forma](https://lwn.net/SubscriberLink/1002456/0d3c386d8c401be9/)
* [Defmt v1.0](https://ferrous-systems.com/blog/defmt-1-0/)
* [registro cambio de rust-analyzer #270](https://rust-analyzer.github.io/thisweek/2025/01/27/changelog-270.html)
* [¡Lanzamiento de Git-Cliff 2.8.0! (un generador de registro cambio altamente personalizable)](https://git-cliff.org/blog/2.8.0)

### Observaciones/Pensamientos
* [Reflexiones sobre los nombres de `trait` de los iteradores](https://blog.yoshuawuyts.com/musings-on-iterator-trait-names/)
* [La función mágica](https://bitfieldconsulting.com/posts/magic-function)
* [Ecuatabilidad personalizada en Rust: más allá de los pasos estándar](https://minikin.me/blog/custom-equatability-in-rust)
* [audio] [Rahul Kumar: ¿Por qué verificar la biblioteca estándar de Rust?](https://timclicks.dev/podcast/rahul-kumar-why-verify-rusts-stdlib)
* [audio] [Volvo con Julius Gustavsson](https://corrode.dev/podcast/s03e08-volvo/)

### Tutoriales en Rust
* [Consejos para compilaciones más rápida de Rust CI | corrode Rust Consulting](https://corrode.dev/blog/tips-for-faster-ci-builds/)
* [Hola Rust asíncrono](https://weipin.github.io/hello-async-rust/index.html)

## `Crate` de la semana

La `crate` de esta semana es [embed\_it](https://github.com/riberk/embed_it), una `crate` que te ayuda a incrustar activos en tu binario y generar implementacione de `struct/trait` para cada archivo o directorio.

¡Gracias a [Riberk](https://users.rust-lang.org/t/crate-of-the-week/2704/1390) por sugerir su propia crate!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamada a pruebas
Un paso importante en la implementación de una RFC es que las personas experimenten con la
implementación y brinden retroalimentación, especialmente antes de su estabilización.
Las siguientes RFCs se beneficiarían de pruebas por parte de los usuarios antes de continuar avanzando:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue una nueva 'call-for-testing'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan ser evaluados.

## Convocatorio a la participación

* [bytes - Llamada par cambio importantes](https://github.com/tokio-rs/bytes/issues/758)

### CFP - Proyectos

¿Siempre quisiste contribuir a proyecto de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tarea en la comunida de Rust para que elijas y comiences!

Alguna de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [rama - primera versión de Rama-Unix](https://github.com/plabayo/rama/issues/370)
* [rama - añadir servicio de router web a rama-http](https://github.com/plabayo/rama/issues/396)

Si eres propietario de un proyect de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que está aceptando presentaciones par unirse a su evento como orador.

Si eres organizador de un evento y esperas ampliar su alcance, envía un enlace a la página web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o contactándonos en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust).!

## Actualizaciones del Proyecto Rust

408 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-21..2025-01-28

* [Soporte para QNX 7.1 con 'io-sock'+libstd y QNX 8.0 (solo 'no_std')](https://github.com/rust-lang/rust/pull/133631)
* [Marcar todos los objetivos NuttX como objetivos de nivel 3 y admitir la biblioteca estándar](https://github.com/rust-lang/rust/pull/136037)
* [Añadir soporte NuttX para objetivos AArch64 y ARMv7-A](https://github.com/rust-lang/rust/pull/135757)
* [Añadir '#[optimize(none)]'](https://github.com/rust-lang/rust/pull/128657)
* [Considerar el préstamo mutable en las sugerencias de argumentos](https://github.com/rust-lang/rust/pull/136032)
* [Añadir sugerencia para convertir 'target_feature' en punteros fn](https://github.com/rust-lang/rust/pull/136064)
* [Agregar solución alternativa para bloqueo paralelo en Rust con errores retrasados](https://github.com/rust-lang/rust/pull/135988)
* [Cambiar tipo de retorno de tupla 'collect_and_partition_mono_items' a struct](https://github.com/rust-lang/rust/pull/136118)
* [Codegen: Almacenar ScalarPair a través de memset cuando un lado es undef y el otro puede ser memset](https://github.com/rust-lang/rust/pull/135335)
* [Compilador: Establecer 'target_abi = "ilp32e"' en todos los objetivos riscv32e](https://github.com/rust-lang/rust/pull/134358)
* [Cobertura: Preparación para cambios en creación de contadores](https://github.com/rust-lang/rust/pull/135873)
* [Detectar campos faltantes con valores predeterminados y sugerir '..'](https://github.com/rust-lang/rust/pull/135794)
* [Imprimir parámetros const después de parámetros de tipo](https://github.com/rust-lang/rust/pull/135749)
* [Mejorar coerción cuando autoderef falla en nuevo solver](https://github.com/rust-lang/rust/pull/134746)
* [No soltar tipos sin glue para construir caídas en llamadas FFI](https://github.com/rust-lang/rust/pull/135976)
* [Evitar selección de objetivos anidados 'T: FnPtr' en diagnósticos del nuevo solver](https://github.com/rust-lang/rust/pull/135866)
* [Habilitar lint 'unreachable_pub' en crates 'Test' y 'proc_macro'](https://github.com/rust-lang/rust/pull/135366)
* [Habilitar sanitizadores de kernel para aarch64-unknown-none-softfloat](https://github.com/rust-lang/rust/pull/135905)
* [Corregir proveedor GDB para 'OsString' en Windows](https://github.com/rust-lang/rust/pull/135812)
* [Corregir ICE: brazo múltiple sin 'false_edge_start_block' en patrones](https://github.com/rust-lang/rust/pull/135409)
* [Prohibir uso de variantes const/ty 'hir::Infer' en contextos ambiguos](https://github.com/rust-lang/rust/pull/135272)
* [Manejar límites de traits globales que definen tipos asociados](https://github.com/rust-lang/rust/pull/135766)
* [Implementar directiva 'needs-subprocess' y limpiar tests](https://github.com/rust-lang/rust/pull/135926)
* [Mejorar diagnóstico de nombres esperados en check-cfg](https://github.com/rust-lang/rust/pull/136016)
* [Alinear 'DIFlags' con 'LLVMDIFlags' en API de LLVM-C](https://github.com/rust-lang/rust/pull/135156)
* [Convertir advertencia 'wasm_c_abi' en error grave](https://github.com/rust-lang/rust/pull/133951)
* [Puntuar intervalos UTF-8 inválidos en código fuente](https://github.com/rust-lang/rust/pull/135557)
* [Registrar correctamente intervalos let-var para expansiones no TT](https://github.com/rust-lang/rust/pull/134478)
* [Reportar error correcto cuando tipo de objeto default se autoreferencia](https://github.com/rust-lang/rust/pull/135971)
* [Eliminar soporte para atributo inestable '#[start]'](https://github.com/rust-lang/rust/pull/134299)
* [Eliminar usos de 'QueryNormalizer' en el compilador](https://github.com/rust-lang/rust/pull/135914)
* [Reformular errores por posible falta de crate en árbol de DEP](https://github.com/rust-lang/rust/pull/133154)
* [Rediseñar reducción de trait para Dyn](https://github.com/rust-lang/rust/pull/133830)
* [Acortar salida del enlazador sin '--verbose'](https://github.com/rust-lang/rust/pull/135707)
* [Mostrar salida del enlazador incluso con éxito](https://github.com/rust-lang/rust/pull/119286)
* [Simplificar 'parse_format::Parser::ws' usando 'next_if'](https://github.com/rust-lang/rust/pull/135920)
* [Omitir lint 'if-let-rescope' excepto en migración](https://github.com/rust-lang/rust/pull/132666)
* [Omitir sugerencias en código derivado](https://github.com/rust-lang/rust/pull/136027)
* [Soporte para ensamblaje en línea wasm en 'naked_asm!'](https://github.com/rust-lang/rust/pull/135648)
* [Mejorar coordenadas en Python](https://github.com/rust-lang/rust/pull/135950)
* [Elevar lint 'clippy::double_neg' como 'double_negations'](https://github.com/rust-lang/rust/pull/126604)
* [Usar 'structurally_normalize' en errores de alias](https://github.com/rust-lang/rust/pull/135816)
* [Usar identificadores en código de diagnóstico](https://github.com/rust-lang/rust/pull/136114)
* [Usar tipo corto en etiqueta de tramo E0308](https://github.com/rust-lang/rust/pull/135949)
* [Miri: many-seeds: usar 8 hilos](https://github.com/rust-lang/miri/pull/4152)
* [Miri: Nombrar pre-pase al buscar hijos de módulo](https://github.com/rust-lang/miri/pull/4153)
* [Implementar tipos 'ByteStr' y 'ByteString'](https://github.com/rust-lang/rust/pull/135073)
* [Implementar 'VecDeque::pop_front_if' y 'VecDeque::pop_back_if'](https://github.com/rust-lang/rust/pull/135890)
* [Implementar marcadores de varianza fantasma](https://github.com/rust-lang/rust/pull/135807)
* [Windows x86: Cambiar i128 para usar vector ABI](https://github.com/rust-lang/rust/pull/134290)
* [Cargo: Reemplazar claves específicas al fusionar configs](https://github.com/rust-lang/cargo/pull/15066)
* [Cargo: Deprecar token CLI en 'login'](https://github.com/rust-lang/cargo/pull/15057)
* [Cargo: Corregir 'shared_std_dependency_rebuild' en Windows](https://github.com/rust-lang/cargo/pull/15111)
* [Cargo: Corregir enlaces rotos en libro de Cargo](https://github.com/rust-lang/cargo/pull/15109)
* [Cargo: Hacer '--allow-dirty' implícito con '--allow-staged'](https://github.com/rust-lang/cargo/pull/15013)
* [Cargo: Mostrar globs cuando no se encuentran miembros del workspace](https://github.com/rust-lang/cargo/pull/15093)
* [Cargo: Eliminar enlace '-C link-arg=-fuse-ld=lld'](https://github.com/rust-lang/cargo/pull/15097)
* [Rustdoc: Corregir sangría en elementos de traits móviles](https://github.com/rust-lang/rust/pull/135998)
* [Rustfmt: Corregir 'wrap_comments' creando bloques inválidos](https://github.com/rust-lang/rustfmt/pull/6417)
* [Clippy: 'arithmetic_side_effects': verificar tipos de expresión](https://github.com/rust-lang/rust-clippy/pull/14062)
* [Clippy: 'match_bool': sugerir corrección con guardias](https://github.com/rust-lang/rust-clippy/pull/14039)
* [Clippy: 'short_circuit_statement': manejar macros y paréntesis](https://github.com/rust-lang/rust-clippy/pull/14047)
* [Clippy: 'unnecessary_semicolon': omitir si causa errores de préstamo](https://github.com/rust-lang/rust-clippy/pull/14049)
* [Clippy: Añadir ajustes para '.into_iter()' redundante](https://github.com/rust-lang/rust-clippy/pull/14035)
* [Clippy: Nueva lint 'doc_overindented_list_items'](https://github.com/rust-lang/rust-clippy/pull/13711)
* [Clippy: Nueva lint 'non_std_lazy_statics'](https://github.com/rust-lang/rust-clippy/pull/13770)
* [Clippy: Sugerencias correctas en 'no_std'](https://github.com/rust-lang/rust-clippy/pull/13999)
* [Clippy: Desactivar 'needless_late_init' con macros](https://github.com/rust-lang/rust-clippy/pull/14053)
* [Clippy: Mejorar 'unnecessary_map_or' con referencias](https://github.com/rust-lang/rust-clippy/pull/14024)
* [Clippy: Nueva lint 'sliced_string_as_bytes'](https://github.com/rust-lang/rust-clippy/pull/14002)
* [Clippy: Aplicabilidad correcta para 'obfuscated_if_else'](https://github.com/rust-lang/rust-clippy/pull/14061)
* [Clippy: Sugerir 'Vec::extend()' en 'same_item_push'](https://github.com/rust-lang/rust-clippy/pull/13987)
* [Clippy: Detectar '.then(..).unwrap_or(..)' en 'obfuscated_if_else'](https://github.com/rust-lang/rust-clippy/pull/14021)
* [rust-analyzer: Verificar CFG al recopilar macro refs](https://github.com/rust-lang/rust-analyzer/pull/19014)
* [rust-analyzer: Añadir archivos de compilación a ProjectFolders](https://github.com/rust-lang/rust-analyzer/pull/19019)
* [rust-analyzer: Implementar 'autotipos arbitrarios'](https://github.com/rust-lang/rust-analyzer/pull/19012)
* [rust-analyzer: Implementar 'default-field-values'](https://github.com/rust-lang/rust-analyzer/pull/19001)
* [rust-analyzer: Configurar inserción automática de 'await' e 'iter()'](https://github.com/rust-lang/rust-analyzer/pull/18993)
* [rust-analyzer: Soporte para '#[target_feature]' en funciones seguras](https://github.com/rust-lang/rust-analyzer/pull/19038)
* [rust-analyzer: Completar variantes ocultas en enums](https://github.com/rust-lang/rust-analyzer/pull/19034)
* [rust-analyzer: No sugerir 'into_iter().method()' en iteradores](https://github.com/rust-lang/rust-analyzer/pull/19050)
* [rust-analyzer: Corregir 'ItemScope' con imports glob](https://github.com/rust-lang/rust-analyzer/pull/19016)
* [rust-analyzer: Corregir token faltante en resaltado semántico](https://github.com/rust-lang/rust-analyzer/pull/19045)
* [rust-analyzer: Corregir pánico en flycheck con estrategia "una vez"](https://github.com/rust-lang/rust-analyzer/pull/19017)
* [rust-analyzer: Corregir filtrado en Flyimport](https://github.com/rust-lang/rust-analyzer/pull/19028)
* [rust-analyzer: Corregir resaltado sintáctico para renombres](https://github.com/rust-lang/rust-analyzer/pull/19047)
* [rust-analyzer: Mejorar expansión de finalización considerando recursividad](https://github.com/rust-lang/rust-analyzer/pull/19037)
* [rust-analyzer: Evitar recursión infinita en formato de límites](https://github.com/rust-lang/rust-analyzer/pull/19020)
* [rust-analyzer: Marcar punteros FN inseguros como unsafe](https://github.com/rust-lang/rust-analyzer/pull/19051)
* [rust-analyzer: Ordenar finalizaciones con 'await'/'iter()'](https://github.com/rust-lang/rust-analyzer/pull/18988)
* [rust-analyzer: Ir a 'Display::fmt' desde 'to_string'](https://github.com/rust-lang/rust-analyzer/pull/18986)
* [rust-analyzer: Aumentar límite de recursión de autoderef a 20](https://github.com/rust-lang/rust-analyzer/pull/19004)
* [rust-analyzer: Mantener propiedades de sugerencias calculadas](https://github.com/rust-lang/rust-analyzer/pull/18991)
* [rust-analyzer: Mejorar presentación de dichos](https://github.com/rust-lang/rust-analyzer/pull/18973)
* [rust-analyzer: Recopilar símbolos implícitos solo con renombres](https://github.com/rust-lang/rust-analyzer/pull/19026)
* [rust-analyzer: Priorizar tareas de formateo](https://github.com/rust-lang/rust-analyzer/pull/19052)
* [rust-analyzer: Separar 'ExpressionStore' de 'Body'](https://github.com/rust-lang/rust-analyzer/pull/19036)
* [rust-analyzer: Usar 'strict_provenance'](https://github.com/rust-lang/rust-analyzer/pull/18909)

### Clasificación del rendimiento de compilador de Rust

Semana relativamente tranquila, con una gran regresión que se revertirá.
[#132666](https://github.com/rust-lang/rust/pull/132666) produjo un buen rendimiento. ganar, saltando
trabajo nnecesario. Esta PR en realidad revirtió una regresión causada por una [PR anterior](https://github.com/rust-lang/rust/pull/131984).

Triaje realizado por **@kobzol**.

Rango de revisión: [9a1d156f.. F7538506](https://perf.rust-lang.org/?start=9a1d156f38c51441ee51e5a068f1d0caf4bb0f27&end=f753850659bdf5788332525f3fe395685929c682&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primario) | 0.5% | [0.2%, 2.2%] | 42 |
| Regresiones ❌ <br /> (secundaria) | 2.1% | [0.1%, 11.6%] | 56 |
| Mejoras ✅ <br /> (primario) | -0.8% | [-4.2%, -0.1%] | 107 |
| Mejoras ✅ <br /> (secundaria) | -1,2% | [-4.0%, -0.1%] | 77 |
| Todos ❌✅ (primarios) | -0,5% | [-4,2%, 2,2%] | 149 |

2 regresiones, 3 mejoras, 2 mixtas; 4 de ellos en rollups
45 comparaciones artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d0d5d03231a952b9f4a71a9c94ee73c33610e561/triage/2025-01-27.md).

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitu  omentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se probaron para la implementación esta semana:

* *No se probaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [std: imprimir un backtrace en stackoverflow](https://github.com/rust-lang/rust/pull/133170)
* [Estabilizar const_slice_flatten](https://github.com/rust-lang/rust/pull/134995)
* [derivar 'copy' y 'hash' para 'IntErrorKind'](https://github.com/rust-lang/rust/pull/131923)
* [Problema de seguimiento para 'map_many_mut'](https://github.com/rust-lang/rust/issues/97601)
* [Problema de seguimiento para 'const_vec_string_slice'](https://github.com/rust-lang/rust/issues/129041)
* [Problema de seguimiento para 'const_mut_cursor'](https://github.com/rust-lang/rust/issues/130801)
* [Estabilizar 'const_is_char_boundary' y 'const_str_split_at'.](https://github.com/rust-lang/rust/pull/134016)
* [Problema de seguimiento para 'NonZero*::count_ones'](https://github.com/rust-lang/rust/issues/120287)
* [Estabilizar 'const_black_box'](https://github.com/rust-lang/rust/pull/135414)
* [Hacer de cenum_impl_drop_cast un error grave](https://github.com/rust-lang/rust/pull/135964)
* [Problem de seguimiento para 'once_wait'](https://github.com/rust-lang/rust/issues/127527)
* [[rustdoc] Añadir configuración de fuente sans-serif](https://github.com/rust-lang/rust/pull/133636)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problema de seguimiento cargo o PRS ingresaro al período  final de comentarios esta semana.*

##### [Equipo de idioma](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Fina de Comentarios esta semana.*

##### [Referencia de idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrice de código inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problema de seguimiento e pauta  código inseguros o PRS ingresó el período final de comentarios esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Evento Rust entre 2025-01-29 - 2025-02-26 🦀

### Virtual
* 30/01/2025 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468340)
* 30/01/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡Las computadoras cuánticas no pueden proteger esto contra Rust!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 30/01/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos de código 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Are We Embedded Yet? - Implementación de un pequeño servidor HTTP en microcontrolador**](https://www.meetup.com/code-mavens/events/305382647)
* 31/01/2025 | Virtual (Delhi, IN) | [Asociación de Hackathon Raptors](https://www.meetup.com/hackathon-raptors-association/)
    * [**Hackathon de Rust increíblemente rápido**](https://www.meetup.com/hackathon-raptors-association/events/305435372/)
* 31/01/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305560416/)
* 01/02/2025 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 04/02/2025 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
* 04/02/2025 | Virtual (Londres, GB) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Rust Nation UK Talks**](https://www.meetup.com/women-in-rust/events/305647334)
* 05/02/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - Co-working social**](https://www.meetup.com/indyrs/events/302031658)
* 06/02/2025 | Virtual (Nürnberg, DE) | [Rust en Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820280/)
* 07/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbkb/)
* 11/02/2025 | Virtual (Tel Aviv-Yafo, IL) | [Expertos de código 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Conoce Elusion: Nueva biblioteca de arco datus impulsada por Rust 🦀 con Borivoj Grujicic**](https://www.meetup.com/code-mavens/events/305513416)
* 13/02/2025 | Virtual (Berlín, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 14/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305815307)
* 19/02/2025 | Virtual (Vancouver, BC, CA) | [Rust en Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Proveniencia del puntero**](https://www.meetup.com/vancouver-rust/events/304051783)
* 20/02/2025 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrero 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbcc)
* 25/02/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Últimas partes**](https://www.meetup.com/dallasrust/events/305361428)
* 25/02/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: El complicado mundo de String en Rust**](https://www.meetup.com/women-in-rust/events/305716182)
* 25/02/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mitad mes Rustful - Everett Pompeii presenta Bencher 🐰**](https://www.meetup.com/rustdc/events/305170682)

### Asia
* 24/02/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Presencial Rust febrero 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305570131)

### Europa
* 30/01/2025 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Desarrollo impulsado por hipermedios en Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 30/01/2025 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421383)
* 30/01/2025 | Copenhague, Dinamarca | [Comunidad de Rust en Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Meetup #54 patrocinado por Google**](https://www.meetup.com/copenhagen-rust-community/events/305453880)
* 01/02/2025 | Bruselas, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 01/02/2025 | Helsinki, FI | [Grupo Rust-lang de Finlandia](https://www.meetup.com/finland-rust-meetup/events/)
    * [**Encuentro de febrero**](https://www.meetup.com/finland-rust-meetup/events/305666104)
* 01/02/2025 | Nürnberg, DE | [Rust en Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 05/02/2025 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)
* 06/02/2025 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #7**](https://www.meetup.com/rust-gdansk/events/305742562)
* 12/02/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 14/02/2025 | Edimburgo, Reino Unido | [Rust y amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/305791536)
* 18/02/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introducción a la Programación Contextual-Genérica en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 19-20/02/2025 | Londres, Reino Unido | [Rust Nation Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation Reino Unido 2025**](https://www.rustnationuk.com/)
* 20/02/2025 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #1 @Puzzle ITC**](https://www.meetup.com/rust-bern/events/305597994)
* 21/02/2025 | Londres, Reino Unido | [Rust Global: Londres 2025](https://rustfoundation.org/event/rust-global-london-2025/)
    * [**Rust Global: Londres 2025**](https://rustfoundation.org/event/rust-global-london-2025/)
* 22/02/2025 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika Ferris #9**](https://www.meetup.com/stockholm-rust/events/305848723)

### América del Norte
* 31/01/2025 | Detroit, MI, EE. UU. | [Rust Detroit](https://www.meetup.com/detroitrust/events/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/305847640)
* 03/02/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en el centro de Cambridge Rust, 3 de febrero**](https://www.meetup.com/bostonrust/events/305804837)
* 06/02/2025 | Mountain View, CA, EE. UU. | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305517476)
* 06/02/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Async, el futuro de los futuros**](https://www.meetup.com/stl-rust/events/304959018)
* 11/02/2025 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/305720765)
* 14/02/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en el centro de Rust, 14 de febrero**](https://www.meetup.com/bostonrust/events/305804954)
* 18/02/2025 | San Francisco, CA, EE. UU. | [Grupo de estudio de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)
* 20/02/2025 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 2: Fundamentos de desarrollo de juegos**](https://www.meetup.com/music-city-rust-developers/events/304333047)
* 20/02/2025 | Redmond, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrero 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y ciencia de datos**](https://www.meetup.com/rust-mx/events/305793010)
* 22/02/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo Rust en Union Square en Somerville, 22 de febrero**](https://www.meetup.com/bostonrust/events/305805059)
* 26/02/2025 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)

### Oceanía
* 04/02/2025 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust AKL: Cómo aprendemos Rust**](https://www.meetup.com/rust-akl/events/305583693)
* 04/02/2025 | Sídney, AU | [Rust Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**2025 🦀 Kickstart ✨ Talks**](https://www.meetup.com/rust-sydney/events/305816610)

Si estás organizando un evento de Rust, agrégalo al [calendario] para que se mencione aquí.
Por favor, recuerda agregar un enlace al evento también.
Envía un correo electrónico para solicitar acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, consulte el último hilo en [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Tengo experiencia en múltiples estilo de MMA obtenida de la lucha contra el verificador de préstamos, si es que uenta.

– [Richard Neumann sobre un usuario de Rust](https://users.rust-lang.org/t/is-it-worth-getting-a-degree-in-rust/124678/2)

¡Gracias a [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1654) por la sugerencia!

[¡Por favor, envía tus citas y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1idm0o8/this_week_in_rust_584/)</small>
