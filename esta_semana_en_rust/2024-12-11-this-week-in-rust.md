---
title: "Esta semana en Rust #39"
number_of_week: 39
description: El crate de esta semana es include-utils, un reemplazo más potente para la macro 'include_str' de la biblioteca estándar.
date: 2024-12-11
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

### Oficial
* [Este mes en nuestra infra de pruebas: noviembre de 2024](https://blog.rust-lang.org/inside-rust/2024/12/09/test-infra-nov-2024.html)
* [Actualización del Consejo de Liderazgo de diciembre de 2024](https://blog.rust-lang.org/inside-rust/2024/12/09/leadership-council-update.html)

### Boletines
* [El Rustáceo Incrustado Edición #34](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-34)
* [Tendencias de Rust Edición #55](https://rust-trends.com/newsletter/your-weekly-rust-fix-templates-dependencies-and-a-big-giveaway/)

### Actualizaciones de proyectos/herramientas
* [Dioxus 0.6](https://dioxuslabs.com/blog/release-060/)
* [Repetición impecable](https://flawless.dev/replay/)
* [Actualización de Rust9x: Rust 1.84.0-beta](https://seri.tools/blog/rust9x-1-84/)
* [Novedades de SeaQuery 0.32.x](https://www.sea-ql.org/blog/2024-12-03-whats-new-in-seaquery-0.32.x/)

### Observaciones/Pensamientos
* [Rust Macros: Un cuento con moraleja](https://andrzej.lichnerowicz.pl/en/blog/rust-macros-a-cautionary-tale/)
* [Ejecutar bots de telRust a bajo costo en Fly.io](https://blog.valyagolev.net/fly-teloxide/)
* [Acelerando Ruby reescribiendo C... en Rubí](https://jpcamara.com/2024/12/01/speeding-up-ruby.html)
* [Los decodificadores PNG seguros para memoria ahora superan ampliamente a las bibliotecas PNG de C](https://www.reddit.com/r/rust/comments/1ha7uyi/memorysafe_png_decoders_now_vastly_outperform_c)
* [Estado de las cajas 2025](https://ohadravid.github.io/posts/2024-12-state-of-the-crates/)
* [Comparación de cajas de base de datos de Rust](https://diesel.rs/compare_diesel.html)

### Tutoriales de Rust
* [Análisis de mensajes MIDI en Rust](https://www.ntietz.com/blog/parsing-midi-rust/)
* [Arrastrar y soltar imágenes en Bevy 0.15 en la web](https://rustunit.com/blog/2024/12-10-rust-web-drag-drop-image/)
* [Faltan rasgos iterables y cómo introducirlos sin esfuerzo](https://orxfun.github.io/orxfun-notes/#/missing-iterable-traits-2024-12-13)
* [EuroRust: Introducción al Diesel: conceptos básicos y avanzados en la práctica](https://blog.weiznich.de/eurorust_2024.html)

### Miscelánea
* [Mi historia de Rust](https://softwaremill.com/andre-bogus-my-rust-story/)
* [Informe de empleos de Rust de noviembre de 2024](https://filtra.io/rust/jobs-report/nov-24)
* [video] [Cómo integrar C++ y Rust](https://www.youtube.com/playlist?list=PL6CJYn40gN6jOg_cPqRfXMNriHknKy4VW)
* [video] [Reunión de desarrolladores de LLVM 2024 - Rust ❤️ LLVM](https://www.youtube.com/watch?v=Kqz-umsAnk8)

## Crate de la semana

El crate de esta semana es [include-utils](https://github.com/alekseysidorov/include-utils), un reemplazo más potente para la macro 'include_str' de la biblioteca estándar.

¡Gracias a [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1381) por la autosugestión!

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

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

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

*Esta semana no se han presentado convocatorias ni presentaciones.*x	

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 462 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-03..2024-12-10

* ['dataflow_const_prop': no evalúe una dirección ptr en SwitchInt](https://github.com/rust-lang/rust/pull/134073)
* ['fn_sig_for_fn_abi' debería devolver un 'ty::FnSig', sin necesidad de un encuadernador](https://github.com/rust-lang/rust/pull/133874)
* ['rust_for_linux': -Zreg-struct-return bandera de línea de comandos para X86](https://github.com/rust-lang/rust/pull/130777)
* [en realidad camina hacia vidas y attrs en 'EarlyContextAndPass'](https://github.com/rust-lang/rust/pull/133992)
* [agregue 'allocate_bytes' y refactorice 'allocate_str' en InterpCx para byte sin procesar...](https://github.com/rust-lang/rust/pull/133861)
* [añadir contexto a los errores de "const in pattern"](https://github.com/rust-lang/rust/pull/133233)
* [agregar lint contra comparaciones de punteros de función](https://github.com/rust-lang/rust/pull/118833)
* [añadir más información sobre las discrepancias de tipo/rasgo para diferentes versiones de cajas](https://github.com/rust-lang/rust/pull/133767)
* [evitar errores de 'tipo opaco no restringido' en presencia de otros errores](https://github.com/rust-lang/rust/pull/133850)
* [evite obtener el nodo anon const hir que ya está disponible](https://github.com/rust-lang/rust/pull/133936)
* [Normalizar profundamente cuando se calculan los límites implícitos de vida superada](https://github.com/rust-lang/rust/pull/133517)
* [No implementar características automáticas no seguras para tipos con campos no seguros](https://github.com/rust-lang/rust/pull/133934)
* [No sugiera restringir el límite con rasgos inestables en estable y mencione que es inestable en Nightly](https://github.com/rust-lang/rust/pull/133522)
* [no use un SyntheticProvider para literalmente todos los tipos](https://github.com/rust-lang/rust/pull/133134)
* [arreglar las implementaciones predeterminadas de MutVisitor para visitar los tramos de Stmt y BinOp](https://github.com/rust-lang/rust/pull/133784)
* [Sugerencia de corrección cuando la abreviatura 'self' tiene un tipo erróneo](https://github.com/rust-lang/rust/pull/122161)
* [Modificador de enlace de rasgo fn asincrónico en 'async_trait_bounds'](https://github.com/rust-lang/rust/pull/132612)
* [manejar '--json-output' correctamente](https://github.com/rust-lang/rust/pull/133875)
* [ocultar errores cuyas sugerencias contendrían constantes o tipos de error](https://github.com/rust-lang/rust/pull/133954)
* [implementar comprobaciones para llamadas de cola](https://github.com/rust-lang/rust/pull/133607)
* [mejorar los documentos de 'TagEncoding::Niche', la verificación de cordura y las verificaciones de UB](https://github.com/rust-lang/rust/pull/133681)
* [incluir visualizadores LLDB y GDB en la distribución MSVC](https://github.com/rust-lang/rust/pull/133737)
* [introducir la función 'default_field_values'](https://github.com/rust-lang/rust/pull/129514)
* [lint contra 'Symbol::intern' en un literal de cadena](https://github.com/rust-lang/rust/pull/133545)
* [lint: cambiar la ayuda para punteros a tipos dyn en FFI](https://github.com/rust-lang/rust/pull/131669)
* [hacer traducibles los errores de CoercePointee](https://github.com/rust-lang/rust/pull/133774)
* [Asegúrese de registrar las descripciones de la tarea almacenada en caché en el nuevo solucionador en la primera ejecución](https://github.com/rust-lang/rust/pull/133828)
* [mover la mayoría de las pruebas para '-l' y '#[link(..)]' en 'tests/ui/link-native-libs'](https://github.com/rust-lang/rust/pull/133996)
* [no es necesario crear marcadores de posición para los argumentos de GAT en 'confirm_object_candidate'](https://github.com/rust-lang/rust/pull/133872)
* [solo permite 'PassMode::D irect' para agregados en wasm cuando se usa el C ABI](https://github.com/rust-lang/rust/pull/133931)
* [analizar patrones de protección](https://github.com/rust-lang/rust/pull/133424)
* [reducir los falsos positivos en algunos casos comunes de if-let-rescope lint](https://github.com/rust-lang/rust/pull/133753)
* [reimplementar la especialización de rasgos '~const'](https://github.com/rust-lang/rust/pull/133325)
* [resolver estructuralmente en 'adjust_for_branches'](https://github.com/rust-lang/rust/pull/133559)
* [resuelve estructuralmente en 'probe_adt'](https://github.com/rust-lang/rust/pull/133558)
* [unificar el manejo de 'sysroot_target_{bin,lib}dir'](https://github.com/rust-lang/rust/pull/132723)
* [use el 'hir_id' correcto para la matriz const arg infers](https://github.com/rust-lang/rust/pull/133779)
* [Miri: Limpieza: Evite pasar el corte de bytes a 'anonsocket_read'](https://github.com/rust-lang/miri/pull/4074)
* [miri: arreglar la lógica de la valla SC](https://github.com/rust-lang/miri/pull/4076)
* [miri: corrige la emulación de memoria débil para evitar generar comportamientos que están prohibidos bajo C++ 20](https://github.com/rust-lang/miri/pull/4057)
* [miri: implementar 'simd_relaxed_fma'](https://github.com/rust-lang/miri/pull/4071)
* [extender Miri para pasar correctamente punteros mutables a través de FFI](https://github.com/rust-lang/rust/pull/133211)
* [eliminar polimorfización](https://github.com/rust-lang/rust/pull/133883)
* [introduce 'MixedBitSet'](https://github.com/rust-lang/rust/pull/133891)
* [estabilizar 'const_collections_with_hasher' y 'build_hasher_default_const_new'](https://github.com/rust-lang/rust/pull/133696)
* [estabilizar 'const_{tamaño,alinear}_of_val'](https://github.com/rust-lang/rust/pull/133762)
* [estabilizar 'noop_waker'](https://github.com/rust-lang/rust/pull/133089)
* [estabilizar 'std::io::ErrorKind::CrossesDevices'](https://github.com/rust-lang/rust/pull/130209)
* [estabilizar 'std::io::ErrorKind::QuotaExceeded'](https://github.com/rust-lang/rust/pull/130254)
* [añadir 'core::arch::breakpoint' y test](https://github.com/rust-lang/rust/pull/133726)
* [implementación de 'fmt::FormattingOptions'](https://github.com/rust-lang/rust/pull/118159)
* [agregar Extender implicaciones para tuplas de aridad 1 a 12](https://github.com/rust-lang/rust/pull/132187)
* [cargo: 'docs(fingerprint)': las banderas adicionales de cargo-rustc no afectan a los metadatos](https://github.com/rust-lang/cargo/pull/14898)
* [cargo: 'feat(build-rs)': Añade la directiva 'error'](https://github.com/rust-lang/cargo/pull/14910)
* [cargo: 'fix(add)': No seleccionar versiones extraídas al normalizar nombres](https://github.com/rust-lang/cargo/pull/14895)
* [cargo: 'fix(build-rs)': Refiérase correctamente al elemento en assert](https://github.com/rust-lang/cargo/pull/14913)
* [cargo: 'fix(build-std)': determine las cajas raíz por la especificación objetivo 'std:bool'](https://github.com/rust-lang/cargo/pull/14899)
* [cargo: 'fix(fingerprint)': No tirar el caché en los cambios de RUSTFLAGS](https://github.com/rust-lang/cargo/pull/14830)
* [cargo: 'fix(fix)': Migrar dependencias del espacio de trabajo](https://github.com/rust-lang/cargo/pull/14890)
* [cargo: 'test(build-std)': hacer mock-std más cerca del mundo real](https://github.com/rust-lang/cargo/pull/14896)
* [cargo: fix(build-rs)!: eliminar ''cargo_cfg_debug_assertions''](https://github.com/rust-lang/cargo/pull/14901)
* [cargo: refactor: use 'Path::p ush' para construir remap-path-prefix](https://github.com/rust-lang/cargo/pull/14908)
* [cargo: semVer: añadir sección sobre captura RPIT](https://github.com/rust-lang/cargo/pull/14849)
* [rustdoc: eliminar el ecualizador para 'clean::Attributes'](https://github.com/rust-lang/rust/pull/133960)
* [rustdoc: renombrar las pruebas 'issue-\d+.rs' para que tengan nombres significativos (parte 10)](https://github.com/rust-lang/rust/pull/134053)
* [rustdoc: cambiar el nombre de 'set_back_info' a 'restore_module_data'](https://github.com/rust-lang/rust/pull/133764)
* [rustdoc: siempre muestra la primera línea de los bloques impl incluso cuando están colapsados](https://github.com/rust-lang/rust/pull/132155)
* [mejorar el código para la recuperación de nombres de archivo en rustdoc](https://github.com/rust-lang/rust/pull/133804)
* [clippy: 'doc_lazy_continuation': Contar correctamente la sangría con barras invertidas](https://github.com/rust-lang/rust-clippy/pull/13742)
* [clippy: extender la 'precedencia' para el enmascaramiento de bits y el desplazamiento](https://github.com/rust-lang/rust-clippy/pull/13743)
* [clippy: nueva pelusa para los lanzamientos de puntero 'as *const _' y 'as *mut _'](https://github.com/rust-lang/rust-clippy/pull/13251)
* [rust-analyzer: agregar una opción configurable para excluir caracteres de activación para asistentes de escritura](https://github.com/rust-lang/rust-analyzer/pull/18522)
* [Rust-analyzer: añadir sugerencias implícitas de inserción de inseguridad para bloques externos](https://github.com/rust-lang/rust-analyzer/pull/18610)
* [rust-analyzer: agregar controlador de escritura para la tubería de lista de parámetros](https://github.com/rust-lang/rust-analyzer/pull/18628)
* [rust-analyzer: atributos completos del ayudante de derivación](https://github.com/rust-lang/rust-analyzer/pull/18604)
* [Analizador de Rust: diagnóstico completo en el trabajo preliminar de reducción de ty y sirve para un primer diagnóstico 🎉 ](https://github.com/rust-lang/rust-analyzer/pull/18541)
* [Rust-analyzer: Extender las operaciones inseguras reportadas](https://github.com/rust-lang/rust-analyzer/pull/18609)
* [rust-analyzer: soporta rasgos 'AsyncFnX'](https://github.com/rust-lang/rust-analyzer/pull/18594)
* [rust-analyzer: arreglar el análisis de los argumentos de tipo entre paréntesis y RTN](https://github.com/rust-lang/rust-analyzer/pull/18593)
* [Rust-analyzer: Mejor recuperación del analizador para rutas](https://github.com/rust-lang/rust-analyzer/pull/18608)
* [rust-analyzer: forzar dos 'FnDef' a fn punteros incluso si son iguales, si son subtipos](https://github.com/rust-lang/rust-analyzer/pull/18633)
* [rust-analyzer: deshabilite el controlador de escritura '<' nuevamente](https://github.com/rust-lang/rust-analyzer/pull/18616)
* [rust-analyzer: no reportar advertencias de macros proc, nunca](https://github.com/rust-lang/rust-analyzer/pull/18611)
* [rust-analyzer: corrige un error cuando se buscaban nodos AST sintéticos en el mapa de ID de AST y causaba pánicos](https://github.com/rust-lang/rust-analyzer/pull/18555)
* [rust-analyzer: arreglar el analizador que se atasca por malas expresiones de ASM](https://github.com/rust-lang/rust-analyzer/pull/18625)
* [rust-analyzer: arreglado el análisis de dyn T en arg genérico en la edición 2015](https://github.com/rust-lang/rust-analyzer/pull/18622)
* [rust-analyzer: arreglar el análisis de referencias de nombres enteros/palabras clave en varios lugares](https://github.com/rust-lang/rust-analyzer/pull/18618)
* [Rust-analyzer: Arreglar el sombreado de la variante de registro 'enum' en patrones](https://github.com/rust-lang/rust-analyzer/pull/18607)
* [rust-analyzer: corregido otro error con las importaciones de globos](https://github.com/rust-lang/rust-analyzer/pull/18605)
* [rust-analyzer: mapear nuevos nodos de reemplazo a sus equivalentes mutables en 'SyntaxEditor'](https://github.com/rust-lang/rust-analyzer/pull/18531)
* [Rust-analyzer: las estructuras no exhaustivas pueden estar vacías](https://github.com/rust-lang/rust-analyzer/pull/18645)
* [rust-analyzer: pánico al mostrar parámetros genéricos con valores predeterminados](https://github.com/rust-lang/rust-analyzer/pull/18619)
* [rust-analyzer: analizar los límites de vida útil en el parámetro de vida en TypeBoundList](https://github.com/rust-lang/rust-analyzer/pull/18620)
* [Rust-analyzer: Resolver parámetros genéricos dentro de las capturas de uso](https://github.com/rust-lang/rust-analyzer/pull/18621)
* [Rust-analyzer: Deshabilita temporalmente el soporte de resolución de finalización para Helix y Neovim](https://github.com/rust-lang/rust-analyzer/pull/18630)
* [Rust-analyzer: Mejorar la heurística para la inserción de punto y coma al escribir](https://github.com/rust-lang/rust-analyzer/pull/18627)
* [Rust-analyzer: hacer que el controlador de escritura de corchetes funcione en más cosas](https://github.com/rust-lang/rust-analyzer/pull/18474)
* [rust-analyzer: migrar 'add_turbo_fish' a 'SyntaxEditor'](https://github.com/rust-lang/rust-analyzer/pull/18551)
* [rust-analyzer: migrar 'introduce_named_generic' Assist para usar 'SyntaxFactory'](https://github.com/rust-lang/rust-analyzer/pull/18483)
* [rust-analyzer: migrar 'sort_items' Assist para usar 'SyntaxFactory'](https://github.com/rust-lang/rust-analyzer/pull/18538)
* [Rust-analyzer: VSCod: solo muestra el elemento de la barra de estado en los archivos relevantes](https://github.com/rust-lang/rust-analyzer/pull/18592)

### Clasificación del rendimiento del compilador de Rust

Una semana bastante tranquila, con pocos récords personales y sin grandes cambios en el rendimiento.

Triaje realizado por **@simulacrum**.
Rango de revisión: [490b2cc0.. 1b3fb316](https://perf.rust-lang.org/?start=490b2cc09860dd62a7595bb07364d71c12ce4e60&end=1b3fb316751227d30b1523ed0e3f00d83956d4d0&absolute=false&stat=instructions%3Au)

0 Regresiones, 0 Mejoras, 7 Mixtas; 4 de ellos en rollups
25 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-12-09.md)

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
* [disposición: fusionar] [Agregar la opción '--doctest-compilation-args' para agregar banderas de compilación a la compilación doctest](https://github.com/rust-lang/rust/pull/128780)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [fix(cargo-rustc): estabilizar las banderas finales de mayor precedencia](https://github.com/rust-lang/cargo/pull/14900)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Desestructuración de tipo de gota](https://github.com/rust-lang/rfcs/pull/3738)
* [nuevo] [#[must_use = falso]](https://github.com/rust-lang/rfcs/pull/3737)
* [nuevo] [RFC: Tipos parciales (v3)](https://github.com/rust-lang/rfcs/pull/3736)

## Próximos eventos

Eventos oxidados entre 2024-12-11 - 2025-01-08 🦀

### Virtual
* 11/12/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**egui**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Reunión de Rust en Seattle](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Encuentro de diciembre**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 12/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 12/12/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 13/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 17/12/2024 | Virtual (San Francisco, CA, EE. UU.) | [Centro Blockchain SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust en Web3: Serie para desarrolladores**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 17/12/2024 | Virtual (Tel Aviv-Yafo, IL) | [Expertos en código 🦀 - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/events/)
    * [**Lectura del código fuente de Rust: La caja de los miles (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/304824684/)
* 17/12/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346972/)
* 19/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633276/)
* 19/12/2024 | Virtual (Ciudad de México, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 20/12/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 24/12/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 26/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 2024-01-02| Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633277/)
* 04/01/2025 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 14/12/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro/taller rustáceo de diciembre de 2024**](https://hasgeek.com/rustbangalore/december-2024-rustacean-meetup-workshop/)

### Europa
* 11/12/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 12/12/2024 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 12/12/2024 | München, DE | [Rust Múnich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2024 / 4 - Noche de hacking**](https://www.meetup.com/rust-munich/events/304827279/)
* 12/12/2024 | Viena, AT | [Rust Viena](https://www.meetup.com/rust-vienna/events/)
    * [**Rust Viena - Diciembre | en Sentry.io 🦀 **](https://www.meetup.com/rust-vienna/events/304815850/)
* 17/12/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Amigo invisible en Rust: Desenvolviendo pruebas de propiedades**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 18/12/2024 | Gante, BE | [Programación de Sistemas Gante](https://sysghent.be)
    * [**Lanzamiento de una nueva comunidad para desarrolladores de Rust y C++*](https://sysghent.be)

### América del Norte
* 12/12/2024 | Híbrido: presencial y virtual (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug/)
    * [**Encuentro de diciembre**](https://www.meetup.com/join-srug/events/304806455/)
* 12/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 16/12/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 17/12/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 26/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)

### Oceanía
* 16/12/2024 | Collingwood, Australia | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Reunión de Rust Melbourne de diciembre de 2024**](https://www.meetup.com/rust-melbourne/events/304820598/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Las implementaciones seguras para la memoria de PNG ([png](https://crates.io/crates/png), [zune-png](https://crates.io/crates/zune-png), [wuffs](https://github.com/google/wuffs/)) ahora superan drásticamente a las que no son seguras para la memoria ([libpng](http://www.libpng.org/), [spng](https://libspng.org/), [stb_image](https://github.com/nothings/stb)) al decodificar imágenes.
>
> caja Rust [png](https://crates.io/crates/png) que encabeza nuestro benchmark muestra una mejora de **1,8x** sobre 'libpng' en x86 y de **1,5x** en ARM.

– [Shnatsel en /r/rust](https://www.reddit.com/r/rust/comments/1ha7uyi/memorysafe_png_decoders_now_vastly_outperform_c/)

¡Gracias a [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1641) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1hcj88m/this_week_in_rust_577/)</small>
