---
title: "Esta semana en Rust #23"
number_of_week: 23
description: El crate de esta semana es layoutparser-ort, un puerto simplificado de LayoutParser para la detección de elementos de diseño de documentos basados en ML.
date: 2024-06-05
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (anteriormente Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Fundación
* [Damos la bienvenida al ingeniero de interoperabilidad de Rust-C++, Jon Bauman, al equipo de la Fundación Rust](https://foundation.rust-lang.org/news/welcoming-rust-c-interoperability-engineer-jon-bauman-to-the-rust-foundation-team/)

### RustNL 2024
* [Diseño visual de aplicaciones para Rust - Rik Arends](https://www.youtube.com/watch?v=NPP2_6KMA60)
* [ThRust in Space: Initial Momentum - Michaël Melchiore](https://www.youtube.com/watch?v=O09rje6yC90)
* [Arco en el Kernel de Linux - Alice Ryhl](https://www.youtube.com/watch?v=gr9v0FFXaZ8)
* [Haciendo conexiones - Mara Bos](https://www.youtube.com/watch?v=aENHzYAFkeE)
* [Reemplazo de OpenSSL paso a paso - Joe Birr-Pixton](https://www.youtube.com/watch?v=10ymtv1J7Os)
* [Fortificación de la FFI de Rust con funciones escapaculadas - Leon Schuermann](https://www.youtube.com/watch?v=O4sVw4YQB)
* [Educación oxidante - Henk Oordt](https://www.youtube.com/watch?v=KwZM0lSTvyk)
* [Postal: Una herramienta irrazonablemente eficaz para la comunicación máquina a máquina - James Munns](https://www.youtube.com/watch?v=HtBFvTH5ZKE)
* [Presentación de June - Sophia Turner](https://www.youtube.com/watch?v=c1isq1Bdmic)
* [Robius: Desarrollo de aplicaciones multiplataforma inmersivas y sin fisuras en Rust - Kevin Boos](https://www.youtube.com/watch?v=Dg4hlfettn8)
* [Compresión Carcinizada: Implementando zlib en Rust - Folkert de Vries](https://www.youtube.com/watch?v=mvzHQdCLkOY)
* [K23: Un sistema operativo de investigación seguro que ejecuta WASM - Jonas Kruckenberg](https://www.youtube.com/watch?v=GjDwj7RWOgs)
* [Rust asíncrono en sistemas embebidos con Embassy - Dario Nieuwenhuis](https://www.youtube.com/watch?v=H7NtzyP9q8E)
* [Xilem: Construyamos una interfaz de usuario de Rust de alto rendimiento - Raph Levien](https://www.youtube.com/watch?v=OvfNipIcRiQ)
* [Rust envenenando mi muñeca por diversión - Ulf Lilleengen](https://www.youtube.com/watch?v=u6q9l89EOXI)
* [Teoría de tipos para ingenieros ocupados - Niko Matsakis](https://www.youtube.com/watch?v=9qLACD9Bfbk)

### Boletines informativos
* [Este mes en Rust GameDev #51 - mayo de 2024](https://gamedev.rs/news/051/)

### Actualizaciones de proyectos/herramientas
* [Entra paradis — Un nuevo capítulo en la historia de paralelismo de Rust](https://andreaslongva.com/blog/enter-paradis/)
* [Tiny Glade, actuaciones de VJ e iluminación 2D](https://thisweekinbevy.com/issue/2024-06-03-tiny-glade-vj-performances-and-2d-lighting)
* [Diésel 2.2.0](https://diesel.rs/news/2_2_0_release.html)
* [Pigg 0.1.0](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.1.0)
* [¡Lanzamiento de Git-Cliff 2.3.0! (generador de registro de cambios altamente personalizable)](https://git-cliff.org/blog/2.3.0)

### Observaciones/Pensamientos
* [El verificador de préstamos dentro](https://smallcultfollowing.com/babysteps/blog/2024/06/02/the-borrow-checker-within/)
* [No te preocupes por las vidas](https://corrode.dev/blog/lifetimes/)
* [Rust no tiene que ver con la seguridad de la memoria](https://o-santi.github.io/blog/rust-is-not-about-memory-safety/)
* [Sobre el uso de dependencias en Rust](https://landaire.net/on-dependency-usage-in-rust/)
* [Context Managers: Undroppable Types for Free](https://blog.yoshuawuyts.com/achieving-undroppable-types-by-leveraging-context-managers/)
* [Rust y punteros delgados de tamaño dinámico](https://john-millikin.com/rust-and-dynamically-sized-thin-pointers)
* [Rust es para el motor, no para el juego](https://barretts.club/posts/rust-for-the-engine/)
* [audio] [Thunderbird - Brendan Abolivier, Ingeniero de Software](https://corrode.dev/podcast/s02e03-thunderbird/)

### Tutoriales de Rust
* [Compilar con Naz : Patrón de estado de tipo de Rust](https://developerlife.com/2024/05/28/typestate-pattern-rust/)
* [Cómo construir un sistema de plugins en Rust](https://www.arroyo.dev/blog/rust-plugin-systems)
* [Formando nubes](https://isaacdaou.st/blog/forming-clouds/)
* [Manejo de errores de Rust: Opción y resultado](https://bitfieldconsulting.com/posts/rust-errors-option-result)
* [Construyamos un Balanceador de Carga en Rust - Parte 3](https://marcobacis.com/blog/load-balancer-rust-3/)
* [La guía definitiva de los nuevos tipos de Rust](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)

### Miscelánea
* [Lo más destacado de "Pasé 6 años desarrollando un juego de rompecabezas en Rust y acaba de salir, AMA"](https://gamesbymason.com/2024/06/01/wor-ama/)

## Crate de la semana

El crate de esta semana es [layoutparser-ort](https://docs.rs/layoutparser-ort), un puerto simplificado de LayoutParser para la detección de elementos de diseño de documentos basados en ML.

A pesar de que no hay sugerencias, llogiq está razonablemente contento con su elección. ¿Estas?

[No importa cuál sea su respuesta, envíe sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatorias de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

### [RFC](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*

Si usted es un implementador de características y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*Esta semana no se han presentado convocatorias para participar en proyectos.*

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí] [directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [Computación científica en Rust 2024](https://scientificcomputing.rs/) | Cierra 14/06/2024 | En línea | Fecha del evento: 2024-07-17 - 2024-07-19
* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Cierra el 06/07/2024 | Online + Ucrania, Kiev | Fecha del evento: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose con [X (anteriormente twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

308 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-28..2024-06-04

* ['-Znext-solver': normalizar con entusiasmo al agregar goles](https://github.com/rust-lang/rust/pull/125343)
* ['fn_arg_sanity_check': corregir mensaje de pánico](https://github.com/rust-lang/rust/pull/125695)
* [add '--print=check-cfg' para obtener las configuraciones esperadas](https://github.com/rust-lang/rust/pull/124320)
* [añadir '-Zfixed-x18'](https://github.com/rust-lang/rust/pull/124655)
* [también InstSimplify '&raw*'](https://github.com/rust-lang/rust/pull/125796)
* [también resolver el tipo de constantes, incluso si ya lo convertimos en una constante de error](https://github.com/rust-lang/rust/pull/125807)
* [evite desenvolver diag.code directamente en 'note_and_explain_type_err'](https://github.com/rust-lang/rust/pull/125774)
* [comprobar índice 'valor <= 0xFFFF_FF00'](https://github.com/rust-lang/rust/pull/125821)
* [cobertura: evite el desbordamiento cuando se exceda el límite de condición MC/DC](https://github.com/rust-lang/rust/pull/125700)
* [cobertura: instrumentar opcionalmente el RHS de operadores lógicos diferidos](https://github.com/rust-lang/rust/pull/125756)
* [cobertura: cambiar el nombre de MC/DC 'conditions_num' a 'num_conditions'](https://github.com/rust-lang/rust/pull/125754)
* [crear DefIds de bloque const en typeck en lugar de reducir ast](https://github.com/rust-lang/rust/pull/124650)
* [no equiparar 'Const's ty en 'super_combine_const'](https://github.com/rust-lang/rust/pull/125671)
* [no sugiera métodos de construcción irresolubles](https://github.com/rust-lang/rust/pull/125397)
* [una pequeña mejora diagnóstica para 'dropping_copy_types'](https://github.com/rust-lang/rust/pull/125433)
* [no recalcular 'cola' en 'lower_stmts'](https://github.com/rust-lang/rust/pull/125790)
* [No sugiera convertir las expresiones no literales de caracteres de ty 'char' en literales de cadena](https://github.com/rust-lang/rust/pull/125640)
* [habilitar DestinationPropagation de forma predeterminada](https://github.com/rust-lang/rust/pull/115105)
* [doblar los límites de los elementos antes de probarlos en 'check_type_bounds' en el nuevo solucionador](https://github.com/rust-lang/rust/pull/125786)
* [Implementar 'needs_async_drop' en Rustc y optimizar el pegamento asíncrono](https://github.com/rust-lang/rust/pull/124662)
* [Mejorar la salida de diagnóstico de 'non_local_definitions' lint](https://github.com/rust-lang/rust/pull/125089)
* [hacer que 'ProofTreeBuilder' sea realmente genérico en lugar de 'Interner'](https://github.com/rust-lang/rust/pull/125598)
* [hacer que 'body_owned_by' devuelva el 'Body' en lugar de solo el 'BodyId'](https://github.com/rust-lang/rust/pull/125711)
* [hacer que los vectores 'repr(empaquetados)' funcionen con intrínsecos SIMD](https://github.com/rust-lang/rust/pull/125311)
* [hacer pelusa: 'lint_dropping_references lint_forgetting_copy_types lint_forgetting_references' dar sugerencia si es posible](https://github.com/rust-lang/rust/pull/125531)
* [omitir 'no needs_drop drop_in_place' en vtables](https://github.com/rust-lang/rust/pull/122662)
* [opt-in en la generación de 'FulfillmentError' para evitar hacer trabajo adicional en el nuevo solucionador](https://github.com/rust-lang/rust/pull/125864)
* [reintroducir la comprobación de resolución de nombres para intentar acceder a las variables locales desde una const en línea](https://github.com/rust-lang/rust/pull/125705)
* [rechazar 'CVarArgs' en 'parse_ty_for_where_clause'](https://github.com/rust-lang/rust/pull/125863)
* [mostrar archivos producidos por '--emit foo' en notificaciones de artefactos JSON](https://github.com/rust-lang/rust/pull/122597)
* [silenciar algunos errores de resolución cuando ha habido errores de importación de globos](https://github.com/rust-lang/rust/pull/125381)
* [dejar de usar 'translate_args' en el nuevo solucionador](https://github.com/rust-lang/rust/pull/125776)
* [soporta preprocesadores mdBook para TRPL en rustbook](https://github.com/rust-lang/rust/pull/125408)
* [test codegen for 'repr(packed,simd)' → 'repr(simd)'](https://github.com/rust-lang/rust/pull/125904)
* [ajustar las relaciones para que ya no dependan de 'TypeTrace'](https://github.com/rust-lang/rust/pull/125664)
* [desenrollar la primera iteración del bucle 'checked_ilog'](https://github.com/rust-lang/rust/pull/124294)
* [eleva '{Closure,Coroutine,CoroutineClosure}Args' y sus amigos a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/125775)
* [usar notación entre paréntesis para los rasgos 'Fn'](https://github.com/rust-lang/rust/pull/125778)
* [añadir algunas comprobaciones más específicas al validador MIR](https://github.com/rust-lang/rust/pull/125851)
* [Miri: evite hacer una copia completa de todas las nuevas asignaciones](https://github.com/rust-lang/rust/pull/125633)
* [Miri: Arreglar la detección de "caja local"](https://github.com/rust-lang/miri/pull/3644)
* [no inhibir el reordenamiento aleatorio de campos en 'repr(packed(1))'](https://github.com/rust-lang/rust/pull/125360)
* [Evite revisar la edición tanto como sea posible](https://github.com/rust-lang/rust/pull/125828)
* [Aumentar el tamaño del diseño de vtable](https://github.com/rust-lang/rust/pull/123572)
* [stabilise 'IpvNAddr::'{'BITS', 'to_bits', 'from_bits'} ('ip_bits')](https://github.com/rust-lang/rust/pull/125551)
* [Estabilizar la función 'custom_code_classes_in_docs'](https://github.com/rust-lang/rust/pull/124577)
* [stablize 'const_binary_heap_constructor'](https://github.com/rust-lang/rust/pull/125211)
* [hacer que 'std::env::'{'set_var', 'remove_var'} no sea seguro en la edición 2024](https://github.com/rust-lang/rust/pull/124636)
* [implementar la función 'integer_sign_cast'](https://github.com/rust-lang/rust/pull/125884)
* [NVPTX: evite 'PassMode::D irect' para args en C abi](https://github.com/rust-lang/rust/pull/117671)
* [genericize 'ptr::from_raw_parts'](https://github.com/rust-lang/rust/pull/125701)
* ['std::p al::unix::thread' obteniendo el tamaño mínimo de la pila en netbsd](https://github.com/rust-lang/rust/pull/125577)
* [añadir un intrínseco para 'ptr::metadata'](https://github.com/rust-lang/rust/pull/124251)
* [Cambiar 'F32::Midpoint' a Upcast a F64](https://github.com/rust-lang/rust/pull/121062)
* [rustc-hash: reemplaza el hash con un hash más rápido y mejor finalizado](https://github.com/rust-lang/rustc-hash/pull/37)
* [prueba de carga: Tiempo transcurrido de redacción automática](https://github.com/rust-lang/cargo/pull/13973)
* [cargo add: Evite escapar las comillas dobles mediante el uso de literales de cadena](https://github.com/rust-lang/cargo/pull/14006)
* [cargo config: Asegúrate de que se respeta '--config net.git-fetch-with-cli=true'](https://github.com/rust-lang/cargo/pull/13992)
* [cargo nuevo: No digas que se están agregando a un espacio de trabajo cuando un paquete normal está en root](https://github.com/rust-lang/cargo/pull/13987)
* [cargo toml: Asegúrese de que los objetivos estén en un orden determinista](https://github.com/rust-lang/cargo/pull/13989)
* [proveedor de carga: Asegúrese de que la clasificación se realice para el proveedor](https://github.com/rust-lang/cargo/pull/14004)
* [cargo: permite obtener la configuración predeterminada de git/gitoxide del ENV y config](https://github.com/rust-lang/cargo/pull/13687)
* [cargo: ajustar el error personalizado de la comprobación de certificados debido al cambio de libgit2 1.8](https://github.com/rust-lang/cargo/pull/13970)
* [cargo: omitir la deserialización de campos no relacionados con nombres superpuestos](https://github.com/rust-lang/cargo/pull/14000)
* [clippy: 'many_single_char_names': deduplicar diagnósticos](https://github.com/rust-lang/rust-clippy/pull/12859)
* [clippy: añadir pelusa 'needless_character_iteration'](https://github.com/rust-lang/rust-clippy/pull/12815)
* [clippy: desaprobar 'maybe_misused_cfg' y 'mismatched_target_os'](https://github.com/rust-lang/rust-clippy/pull/12875)
* [clippy: deshabilita 'indexing_slicing' para implicaciones personalizadas de 'Índice'](https://github.com/rust-lang/rust-clippy/pull/12488)
* [clippy: corrige 'redundant_closure' sugiriendo código incorrecto con 'F: Fn()'](https://github.com/rust-lang/rust-clippy/pull/12865)
* [clippy: deja que 'non_canonical_impls' omita proc marco](https://github.com/rust-lang/rust-clippy/pull/12857)
* [clippy: ignorar la matriz de 'deref_addrof' lint](https://github.com/rust-lang/rust-clippy/pull/12864)
* [clippy: hacer que 'str_to_string' sea aplicable a la máquina](https://github.com/rust-lang/rust-clippy/pull/12871)
* [rust-analyzer: add 'Function::fn_ptr_type(...) ' para obtener el tipo de función borrado por nombre](https://github.com/rust-lang/rust-analyzer/pull/17312)
* [rust-analyzer: no marque las funciones '#[rustc_deprecated_safe_2024]' como inseguras](https://github.com/rust-lang/rust-analyzer/pull/17329)
* [rust-analyzer: habilita las finalizaciones dentro de los atributos del ayudante de derivación](https://github.com/rust-lang/rust-analyzer/pull/17328)
* [rust-analyzer: corrige la búsqueda de contenedores que fallan para los tokens que se originan dentro de los atributos derivados](https://github.com/rust-lang/rust-analyzer/pull/17326)
* [Rust-analyzer: Se corrige el borrado de diagnósticos cuando se ejecutan controles de vuelo por espacio de trabajo](https://github.com/rust-lang/rust-analyzer/pull/17302)
* [rust-analyzer: solo genera fragmentos para 'extract_expressions_from_format_string' si los fragmentos son compatibles](https://github.com/rust-lang/rust-analyzer/pull/17333)
* [rustfmt: colapso anidado si es detectado por clippy](https://github.com/rust-lang/rustfmt/pull/6169)
* [rustfmt: rustfmt no debería eliminar los atributos internos de los bloques const en línea](https://github.com/rust-lang/rustfmt/pull/6173)
* [rustfmt: rust rewrite 'check_diff' (Skeleton)](https://github.com/rust-lang/rustfmt/pull/6166)
* [rustfmt: usa 'with_capacity' en 'rewrite_path'](https://github.com/rust-lang/rustfmt/pull/6174)

### Clasificación del rendimiento del compilador de Rust

Una semana tranquila; Tuvimos una regresión bastante seria (#115105, "habilitar
DestinationPropagation de forma predeterminada"), pero se revirtió en breve (#125794).
La única otra solicitud de incorporación de cambios identificada como potencialmente problemática fue la acumulación
[PR #125824](https://github.com/rust-lang/rust/pull/125824), pero incluso
que es relativamente limitado en su efecto.

Triaje realizado por **@pnkfelix**.
Rango de revisión: [a59072ec.. 1d52972d](https://perf.rust-lang.org/?start=a59072ec4fb6824213df5e9de8cae4812fd4fe97&end=1d52972dd8592edf4026aa577c8ce69acc0ac2d1&absolute=false&stat=instructions%3Au)

3 regresiones, 5 mejoras, 6 mixtas; 4 de ellos en rollups
57 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/fba75cc08937425ab274959581401b862a0b3068/triage/2024-06-03.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Cambiar crates.io política para no ofrecer mediación de transferencia de cajas](https://github.com/rust-lang/rfcs/pull/3646)

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Permitir restringir tipos opacos durante la subtipificación en el sistema de rasgos](https://github.com/rust-lang/rust/pull/125447)
* [disposición: fusionar] [Decisión del TAIT sobre "puede definir implica debe definir"](https://github.com/rust-lang/rust/issues/117861)
* [disposición: fusionar] [Estabilizar SIMD relajado de wasm](https://github.com/rust-lang/rust/pull/117468)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga ni PR en el período de comentarios finales de esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna RFC de equipo lingüístico entró en el período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

##### [Directrices sobre códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2024-06-05 - 2024-07-03 🦀

### Virtual

* 05/06/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/299047896/)
* 06/06/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Tu primera contribución a un proyecto de código abierto de Rust**](https://www.meetup.com/code-mavens/events/301156302/)
* 06/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477702/)
* 09/06/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Páginas de GitHub para desarrolladores de Rust (inglés)**](https://www.meetup.com/code-mavens/events/301215326/)
* 11/06/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341709/)
* 12/06/2024 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 8 - Programación asíncrona**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 13/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 13/06/2024 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 16/06/2024 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**Taller: Desarrollo web en Rust usando Rocket (Inglés)**](https://www.meetup.com/code-mavens/events/301294669/)
* 18/06/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346963/)
* 19/06/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 20/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477705/)
* 25/06/2024 | Virtual (Dallas, TX, EE. UU.)| [Grupo de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 27/06/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 02/07/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 03/07/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Crear aplicaciones web con Rust y Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 03/07/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/300328025/)

### Europa

* 05/06/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Junio 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 06/06/2024 | Madrid, ES | [Rust Loco](https://www.meetup.com/madrust/)
    * [**Introducción a Rust y el futuro de los sistemas DLT**](https://www.meetup.com/madrust/events/301318288/)
* 06/06/2024 | Vilnius, LT | [Vilna Rust](https://www.meetup.com/rust-in-vilnius/)
    * [**Disfruta de nuestro segundo evento de Rust y ZIG**](https://www.meetup.com/rust-in-vilnius/events/301012097/)
* 06/06/2024 | Wrocław, PL | [Rust de Breslavia](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #37**](https://www.meetup.com/rust-wroclaw/events/301322042/)
* 11/06/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust Hack Night #6: Bots de Discord**](https://www.meetup.com/copenhagen-rust-community/events/301439744/)
* 11/06/2024 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #69**](https://mobilizon.fr/events/681b42dd-a456-4bfc-99e2-d1c60e867cbb)
* 12/06/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Leyendo Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/301012491/)
* 18/06/2024 | Fráncfort del Meno, DE | [Reunión de Rust Frankfurt](https://www.meetup.com/rust-frankfurt)
    * [**¡Rust Frankfurt ha vuelto!**](https://www.meetup.com/rust-frankfurt/events/301441434/)
* 19/06/2024 - 24/06/2024 | Zúrich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 20/06/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 25/06/2024 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 27/06/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288965/)
* 27/06/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 patrocinado por Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)

### América del Norte

* 08/06/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Porter Square, 8 de junio**](https://www.meetup.com/bostonrust/events/300116799/)
* 11/06/2024 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Reunión mensual de Rust NYC**](https://www.meetup.com/rust-nyc/events/301386836/)
* 12/06/2024 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Detroit Rust Meet - Ann Arbor**](https://www.meetup.com/detroitrust/events/301387848/)
* 13/06/2024 | Spokane, WA, EE. UU. | [Rust de Spokane](https://www.meetup.com/spokane-rust/)
    * [**Reunión mensual: ¡Tema por determinar!**](https://www.meetup.com/spokane-rust/events/300020010/)
* 17/06/2024 | Minneapolis, MN Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301411625/)
* 18/06/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 20/06/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 26/06/2024 | Austin, TX, EE. UU. | [ATC de Rust](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 27/06/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)

### Oceanía

* 14/06/2024 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**Reunión de junio de 2024 de Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/301311680/)
* 20/06/2024 | Auckland, Nueva Zelanda | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Escribir un compilador por diversión y (no) beneficio**](https://www.meetup.com/rust-akl/events/301193761/)
* 25/06/2024 | Canberra, ACt, AU | [Grupo de usuarios de Canberra Rust (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/300749371/)

### América del Sur

* 06/06/2024 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Cada PR es especial™

– [Hieyou Xu describe estar en la rotación de revisión del compilador t](https://jieyouxu.github.io/blog/review-rotation/)

Lamentablemente, no hubo ninguna sugerencia, por lo que a llogiq se le ocurrió algo que esperaba que fuera adecuado.

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1d97pjo/this_week_in_rust_550/)</small>
