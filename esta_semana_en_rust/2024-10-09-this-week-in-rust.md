---
title: "Esta semana en Rust #30"
number_of_week: 30
description: El crate de esta semana es float8, una implementación float de 8 bits.
date: 2024-10-09
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

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Fundación
* [Gracias por unirse a nosotros en RustConf 2024](https://foundation.rust-lang.org/news/thanks-for-joining-us-at-rustconf-2024/)

### Boletines informativos
* [Este mes en Rust OSDev: septiembre de 2024](https://rust-osdev.com/this-month/2024-09/)

### Actualizaciones de proyectos/herramientas
* [¡Anunciando zerocopy 0.8!](https://github.com/google/zerocopy/discussions/1680)
* [Introducción a la ejecución duradera](https://flawless.dev/docs/)
* [Búsqueda del rendimiento en la construcción de un compilador de JavaScript](https://oxc.rs/docs/learn/performance.html)
* [gitoxide septiembre de 2024](https://github.com/Byron/gitoxide/discussions/1614)
* [Tauri 2.0 Versión Estable](https://v2.tauri.app/blog/tauri-20/)
* [Una nueva versión de modversions](https://lwn.net/Articles/986892/)
* [Punteros inteligentes para el kernel](https://lwn.net/SubscriberLink/992055/104fe7d0d355faba/)
* [Puntos de seguimiento de Rust eficientes](https://lwn.net/SubscriberLink/992455/6c61de6764f17830/)
* [Mejorando el enlace para el kernel](https://lwn.net/SubscriberLink/992693/d4d6587f6faaf524/)
* [términoscp 0.15.0](https://blog.veeso.dev/blog/en/announcing-termscp-015/)

### Observaciones/Pensamientos
* [regalloc III](https://d-sonuga.netlify.app/gsoc/regalloc-iii/)
* [Sobre la dicotomía de Ousterhout](https://matklad.github.io/2024/10/06/ousterhouts-dichotomy.html)
* [El Rust está saliendo de la línea de montaje de Volvo](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line)
* [Tres tipos de desenvoltura](https://zkrising.com/writing/three-unwraps/)
* [5 impresionantes (y menos conocidos) proyectos de Rust](https://kerkour.com/awesome-rust-projects-2024)
* [Nueve reglas para ejecutar Rust en el navegador: Lecciones prácticas de portar range-set-blaze a WASM](https://towardsdatascience.com/nine-rules-for-running-rust-in-the-browser-8228353649d1)
* [video] [Renace un Framework Web Legendario... En Rust](https://www.youtube.com/watch?v=7utPutDORb4)
* [audio] [Arreglando los tiempos de construcción con rubicon](https://sdr-podcast.com/episodes/dynamic-linking/)

### Tutoriales de Rust
* [Construcción de E/S asíncronas en Rust: cómo funcionan juntos los futuros, los despertadores y los grupos de subprocesos](https://www.spaghetti-coder.com/building-async-io-in-rust-how-futures-wakers-and-thread-pools-work-together)
* [Rasgo de índice, elementos anclados y vector de empuje inmutable](https://orxfun.github.io/orxfun-notes/#/imp-vec-motivation-2024-10-03)
* [serie] [Serde para el objeto rasgo 2: serialización](https://voelklmichael.github.io/Blog/2024/10/08/serde-trait-part2.html)
* [video] [Build with Naz : Create an async shell in Rust](https://www.youtube.com/watch?v=jXzFCDIJQag)

### Miscelánea
* [Informe de empleo de Rust de septiembre de 2024](https://filtra.io/rust-sep-24)

## Crate de la semana

El crate de esta semana es [float8](https://crates.io/crates/float8), una implementación float de 8 bits.

Llogiq sigue satisfecho con su elección, pero cada vez está más descontento por la falta de sugerencias.

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la prueba
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

### [Oxidación](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron llamados para pruebas esta semana.*

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspectos(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 437 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-01..2024-10-08

* [añadir 'x86_64-desconocido-confiable' como objetivo de nivel 3](https://github.com/rust-lang/rust/pull/130453)
* [soporte inicial para 'riscv32{e|em|emc}_unknown_none_elf'](https://github.com/rust-lang/rust/pull/130555)
* [aumentar el tamaño de la pila para AIX](https://github.com/rust-lang/rust/pull/131116)
* [permitir literales booleanos como predicados cfg](https://github.com/rust-lang/rust/pull/131034) (RFC [#3695](https://rust-lang.github.io/rfcs/3695-cfg-boolean-literals.html))
* [cuenta para 'impl Trait {' cuando 'impl Trait for Type {' era intencionado](https://github.com/rust-lang/rust/pull/131273)
* [añadir la macro 'naked_asm!' para usar en las funciones '#[naked]']](https://github.com/rust-lang/rust/pull/128651)
* [agregar un lint para que el puntero se transmute en los números enteros en consts](https://github.com/rust-lang/rust/pull/130540)
* [Agregar almacenamiento en caché a la mayoría de las carpetas de tipos, uniquificación de la región RM](https://github.com/rust-lang/rust/pull/130821)
* [añadir los indicadores de módulo que faltan para '-Zfunction-return=thunk-extern'](https://github.com/rust-lang/rust/pull/130824)
* [Agregar soporte para volver a tomar prestados receptores de métodos anclados](https://github.com/rust-lang/rust/pull/130633)
* [Agregar soporte inestable para la salida de sumas de comprobación de archivos para su uso en Cargo](https://github.com/rust-lang/rust/pull/126930)
* [evite ICE en construcciones de cobertura con mal '#[coverage(..)]' atributos](https://github.com/rust-lang/rust/pull/131187)
* [consulte las proyecciones elaboradas de dyn no mencionan las vidas tardías sin restricciones](https://github.com/rust-lang/rust/pull/130367)
* [Calcular la longitud de la matriz a partir del tipo para unconditional panic lint](https://github.com/rust-lang/rust/pull/129517)
* [Un par de cambios para facilitar la compilación de rustc para wasm](https://github.com/rust-lang/rust/pull/130899)
* [Cobertura: Múltiples pequeños ajustes para contrarrestar la creación](https://github.com/rust-lang/rust/pull/131325)
* [deshabilitar el subproceso de salto 'UnOp::Not' para no bool](https://github.com/rust-lang/rust/pull/131201)
* [no consideres match/let/ref de un lugar que se evalúe como '!' para divergir, no permitas coerciones de ellos también](https://github.com/rust-lang/rust/pull/129392)
* [no permitir el atributo '#[pointee]' donde no pertenece](https://github.com/rust-lang/rust/pull/128721)
* [no dar sugerencias de métodos cuando la sonda de método falla debido a una mala implementación de 'Deref'](https://github.com/rust-lang/rust/pull/131024)
* [Mejorar el diagnóstico de los rasgos constantes para el nuevo desazucarado](https://github.com/rust-lang/rust/pull/131152)
* [instanciar carpetas en 'supertrait_vtable_slot'](https://github.com/rust-lang/rust/pull/131042)
* [hacer de 'deprecated_cfg_attr_crate_type_name' un error grave](https://github.com/rust-lang/rust/pull/129670)
* [hacer que la prueba 'test_lots_of_insertions' tome menos tiempo en Miri](https://github.com/rust-lang/rust/pull/131085)
* [hacer que los tipos opacos sean nodos HIR regulares](https://github.com/rust-lang/rust/pull/129244)
* [solo consulta 'params_in_repr' si el tipo def es adt](https://github.com/rust-lang/rust/pull/131150)
* [pánico cuando un error de intérprete se descarta involuntariamente](https://github.com/rust-lang/rust/pull/130885)
* [parser: mejores mensajes de error para '@' en patrones 'struct'](https://github.com/rust-lang/rust/pull/130725)
* [reemplace -Z default-hidden-visibility por -Z default-visibility](https://github.com/rust-lang/rust/pull/130005)
* [restringir las directivas 'ignore-mode-*'](https://github.com/rust-lang/rust/pull/131346)
* [Admite 'clobber_abi' y registros vectoriales/de acceso (solo clobber) en el ensamblaje en línea S390X](https://github.com/rust-lang/rust/pull/130630)
* [interpretar: habilitar siempre las comprobaciones de cordura 'write_immediate'](https://github.com/rust-lang/rust/pull/131006)
* [Miri: Añadir reloj vectorial a las listas de epoll ready](https://github.com/rust-lang/miri/pull/3932)
* [miri: añadidas instrucciones del analizador de Rust para Helix](https://github.com/rust-lang/miri/pull/3936)
* [Miri: Evita 'pthread_attr_t' en los exámenes](https://github.com/rust-lang/miri/pull/3945)
* [miri: implementar intrínsecos LLVM x86 gfni](https://github.com/rust-lang/miri/pull/3895)
* [Miri: Prefiero los patrones de corte refutables sobre la verificación de len + la operación de índice](https://github.com/rust-lang/miri/pull/3940)
* [miri: pthread mutex: mejor error en reentrant-locking-UB](https://github.com/rust-lang/miri/pull/3943)
* [Limpiezas 'rustc_infer'](https://github.com/rust-lang/rust/pull/131226)
* [estabilizar 5 API dependientes de 'const_mut_refs'](https://github.com/rust-lang/rust/pull/131177)
* [estabilizar 'BufRead::skip_until'](https://github.com/rust-lang/rust/pull/131267)
* [estabilizar 'const_float_classify'](https://github.com/rust-lang/rust/pull/130157)
* [estabilizar 'const_slice_from_raw_parts_mut'](https://github.com/rust-lang/rust/pull/130403)
* [estabilizar 'const_slice_split_at_mut' y 'const_slice_first_last_chunk'](https://github.com/rust-lang/rust/pull/130428)
* [estabilizar el especificador de fragmento 'expr_2021' en todas las ediciones](https://github.com/rust-lang/rust/pull/129972)
* [estabilizar los métodos 'map'/'value' en 'ControlFlow'](https://github.com/rust-lang/rust/pull/130518)
* [liballoc: introduce String, Vec const-slicing](https://github.com/rust-lang/rust/pull/128399)
* [hacer que la celda conste de manera inestable](https://github.com/rust-lang/rust/pull/131281)
* [Habilitar F16 y F128 en destinos Windows-gnullVM](https://github.com/rust-lang/rust/pull/131308)
* [transmuteFrom: maneja con gracia los tipos no normalizados y los errores de normalización](https://github.com/rust-lang/rust/pull/131112)
* [pequeña optimización para la implementación de la pantalla de números enteros](https://github.com/rust-lang/rust/pull/128204)
* [añadir '[Opción<T>; N]::transponer'](https://github.com/rust-lang/rust/pull/130829)
* [añadir comprobaciones de condiciones previas a 'ptr::offset, ptr::add, ptr::sub'](https://github.com/rust-lang/rust/pull/130251)
* [evitar la comprobación de vacío en 'PeekMut::p op'](https://github.com/rust-lang/rust/pull/131197)
* [no uses 'Immediate::offset' para transmutar punteros a números enteros](https://github.com/rust-lang/rust/pull/131068)
* [Agregar canal multiproductor, multiconsumidor (MPMC)](https://github.com/rust-lang/rust/pull/126839)
* [impl 'Default' para los iteradores 'HashMap'/'HashSet' que aún no lo tienen](https://github.com/rust-lang/rust/pull/128711)
* [std: hacer que 'thread::current' esté disponible en todos los destructores de 'thread_local!](https://github.com/rust-lang/rust/pull/127912)
* [std: reemplaza 'LazyBox' por 'OnceBox'](https://github.com/rust-lang/rust/pull/131094)
* [futuros: arreglar el uso después de liberar de tarea en FuturesUnordered cuando se caen pánicos futuros](https://github.com/rust-lang/futures-rs/pull/2886)
* [hashbrown: añade 'Tag(u8)' newtype en un intento de dejar de usar punteros de bytes para todo](https://github.com/rust-lang/hashbrown/pull/565)
* [hashbrown: cambiar el hasher predeterminado a foldhash](https://github.com/rust-lang/hashbrown/pull/563)
* [cargo: fix 'cargo:version_number' - tiene solo un ':'](https://github.com/rust-lang/cargo/pull/14637)
* [cargo: corrección: eliminar la eliminación de características implícitas](https://github.com/rust-lang/cargo/pull/14630)
* [cargo: mejorar el reporte de errores cuando la función no se encuentra en 'activated_features'](https://github.com/rust-lang/cargo/pull/14647)
* [rustdoc: Errores de limpieza en desambiguadores/discrepancias de espacios de nombres](https://github.com/rust-lang/rust/pull/131260)
* [rustdoc: mejorar <wbr>la ''-inserción para 'SCREAMING_CAMEL_CASE'](https://github.com/rust-lang/rust/pull/131370)
* [rustdoc: las listas de elementos que contienen varios párrafos son más claras](https://github.com/rust-lang/rust/pull/130933)
* [rustdoc: evitar que los ctors resuelvan](https://github.com/rust-lang/rust/pull/131224)
* [clippy: 'infinite_loop': continuando un bucle exterior se deja el bucle interior](https://github.com/rust-lang/rust-clippy/pull/13512)
* [clippy: 'rustc_tools_util': volver a ejecutar cuando cambia la confirmación de git](https://github.com/rust-lang/rust-clippy/pull/13329)
* [clippy: 'zombie_processes': considera las llamadas 'wait()' en cuerpos anidados](https://github.com/rust-lang/rust-clippy/pull/13462)
* [clippy: compara correctamente las referencias de rasgos en 'trait_duplication_in_bounds'](https://github.com/rust-lang/rust-clippy/pull/13493)
* [clippy: corrige 'mut_mutex_lock' cuando la referencia no es mutable en última instancia](https://github.com/rust-lang/rust-clippy/pull/13122)
* [clippy: implementa lint para la compilación 'regex::Regex' dentro de un bucle](https://github.com/rust-lang/rust-clippy/pull/13412)
* [clippy: reducir el umbral predeterminado de 'matriz grande'](https://github.com/rust-lang/rust-clippy/pull/13485)
* [clippy: muestra la cadena de mutabilidad interior en 'mutable_key_type'](https://github.com/rust-lang/rust-clippy/pull/13496)
* [clippy: simplificar negativo 'Opción::{is_some_and,is_none_or}'](https://github.com/rust-lang/rust-clippy/pull/13443)
* [clippy: estilo: no usar defensivamente 'saturating_sub()'](https://github.com/rust-lang/rust-clippy/pull/13513)
* [Rust-analyzer: corrección: arreglar el mensaje de error de arranque que es incorrecto](https://github.com/rust-lang/rust-analyzer/pull/18219)
* [rust-analyzer: use external stack in borrowck DFS](https://github.com/rust-lang/rust-analyzer/pull/18255)

### Clasificación del rendimiento del compilador de Rust

Una regresión dominó esta semana (lidiando con una corrección de corrección en torno al almacenamiento en caché del sistema de tipos que se consideró necesaria), pero afortunadamente no produjo grandes regresiones en ningún punto de referencia. En general, el rendimiento terminó relativamente en el mismo lugar que al comienzo de la semana.

Triaje realizado por **@rylev**.
Rango de revisión: [c87004a1.. E6C46DB4](https://perf.rust-lang.org/?start=c87004a1f5be671e3f03f69fb13d8915bdbb6a52&end=e6c46db4e9fd11e3183c397a59d946731034ede6&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.3% | [0.1%, 1.0%] | 63 |
| Regresiones ❌ <br /> (secundaria) | 1.1% | [0.1%, 3.4%] | 81 |
| Mejoras ✅ <br /> (primario) | -0,5% | [-3,0%, -0,1%] | 19 |
| Mejoras ✅ <br /> (secundaria) | -0,5% | [-1,5%, -0,1%] | 46 |
| Todos ❌✅ (primarios) | 0.1% | [-3.0%, 1.0%] | 82 |

2 regresiones, 3 mejoras, 7 mixtas; 3 de ellos en rollups
57 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/e6fcc69d6b3483f737140ff5c9fdba1ccac44776/triage/2024-10-08.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC: Permitir literales booleanos como predicados 'cfg'](https://github.com/rust-lang/rfcs/pull/3695)
* [Mover la caja 'rustdoc-types' a la propiedad de T-Rustdoc.](https://github.com/rust-lang/rfcs/pull/3505)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Estabilizar const 'ptr::write*' y 'mem::replace'](https://github.com/rust-lang/rust/pull/130954)
* [disposición: fusionar] [Comprobar la compatibilidad del objetivo ABI para punteros de función](https://github.com/rust-lang/rust/pull/128784)
* [disposición: fusionar] [Propuesta: estabilizar 'if_let_rescope' para la edición 2024](https://github.com/rust-lang/rust/issues/131154)
* [disposición: fusionar] [Arreglar las rutas literales de Windows cuando se usa con la macro 'include!'](https://github.com/rust-lang/rust/pull/125205)
* [disposición: fusionar] [Implementado 'FromStr' para 'CString' y 'TryFrom<CString>' para 'String'](https://github.com/rust-lang/rust/pull/130608)
* [disposición: fusionar] [Problema de seguimiento para 'debug_more_non_exhaustive'](https://github.com/rust-lang/rust/issues/127942)
* [disposición: fusionar] [Problema de seguimiento para 'const_make_ascii'](https://github.com/rust-lang/rust/issues/130698)
* [disposición: fusionar] [Problema de seguimiento para 'const_char_encode_utf8'](https://github.com/rust-lang/rust/issues/130512)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [hazaña: Estabilizar la configuración de resolución compatible con MSRV](https://github.com/rust-lang/cargo/pull/14639)
* [disposición: fusionar] [API oficial para scripts de compilación](https://github.com/rust-lang/cargo/issues/12432)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* [disposición: fusionar] [Propuesta de reunión: cambiar el nombre de "seguridad de objetos" a "compatibilidad con dyn"](https://github.com/rust-lang/lang-team/issues/286)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

##### [Directrices para códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro o PR ingresados al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Promover riscv64gc-unknown-linux-gnu a Tier-1 (sin herramientas de host)](https://github.com/rust-lang/rfcs/pull/3707)
* [new] [[RFC] Add Option::todo y Result::todo](https://github.com/rust-lang/rfcs/pull/3706)

## Próximos eventos

Eventos oxidados entre 2024-10-09 - 2024-11-06 🦀

### Virtual
* 10/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Parte 2 de 4 - Navegando por los marcos web de Rust: Axum, Actix y Rocket**](https://www.meetup.com/women-in-rust/events/303213792/)
* 10/10/2024 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Duración](https://www.codurance.com/) + [Constructores de servicio pesado](https://heavyduty.builders/)
    * [**15º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/303443195/)
* 10/10/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633270/)
* 10/10/2024 | Virtual (Girona, ES) | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Aprovechando Rust para mejorar los fundamentos de programación y de Rust a Solana**](https://www.meetup.com/rust-girona/events/303484509/)
* 2024-10-10 - 2024-10-11 | Virtual y presencial (Viena, AT) | [Rust del euro](Eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 14/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [** 👋 Puesta al día de la comunidad**](https://www.meetup.com/women-in-rust/events/302828025/)
* 14/10/2024 | Virtual | [Rust para el almuerzo](https://lunch.rs/)
    * [**Rust for Lunch - Primeros pasos con Rust integrado (Ponente: Sandro Stikić)**](https://lunch.rs/meetups/2024-07-09/) | [Enlace a la reunión en línea](https://lecture.senfcall.de/hay-gmh-wox-mru)
* 15/10/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346970/)
* 16/10/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 17/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Parte 3 de 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 17/10/2024| Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
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
* 31/10/2022 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)

### África
* 02/11/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 09/10/2024 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malasia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Rasgos y rasgos de cómo leer (octubre de 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)
* 17/10/2024 - 18/10/2024 | Pekín, CN | [Encuentro Global de Innovación de Código Abierto (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 19/10/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de octubre de 2024**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europa
* 09/10/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)
* 2024-10-10 - 2024-10-11 | Virtual y presencial (Viena, AT) | [Rust del euro](Eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 15/10/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 17/10/2024 | Darmstadr, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Código de Rust juntos**](https://www.meetup.com/rust-rhein-main/events/303240000/)
* 15/10/2024 | Cambridge, Reino Unido | [Encuentro de Cambridge Rust](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Encuentro mensual de Rust**](https://www.meetup.com/cambridge-rust-meetup/events/303606799/)
* 15/10/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tema por determinar**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 15/10/2024 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 3 - híbrido**](https://www.meetup.com/rust-munich/events/303273953/)
* 16/10/2024 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 17/10/2024 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/303792888/)
* 17/10/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 22/10/2024 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/)
    * [**Nuevo Encuentro de Rust Varsovia #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 26/10/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Foro Fika de Ferris #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 28/10/2024 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 29/10/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de Hackeo**](https://www.meetup.com/rust-aarhus/events/303479865)
* 30/10/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Octubre 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 31/10/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/300820289/)
* 06/11/2024 | París, FR | [Rústicos de París](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Encuentro de Rust en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)

### América del Norte
* 15/10/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638250/)
* 16/10/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 17/10/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Encuentro de octubre**](https://www.meetup.com/join-srug/events/303545170/)
* 19/10/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de North End Rust, 19 de octubre**](https://www.meetup.com/bostonrust/events/303708335/)
* 23/10/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 27/10/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Kendall Rust, 27 de octubre**](https://www.meetup.com/bostonrust/events/303708359/)
* 04/11/2024 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo en Coolidge Corner Brookline Rust, 4 de noviembre**](https://www.meetup.com/bostonrust/events/303708387/)

### Oceanía
* 29/10/2024 | Canberra, ACT, AU | [Grupo de Usuarios de Rust de Canberra (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de junio**](https://www.meetup.com/rust-canberra/events/303128131/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](INSERT_LINK_HERE)

# Frase de la semana

> estoy en el lado equivocado de los 45. No tengo ningún interés en perder el tiempo que podría haber dejado escribiendo C desde cero. Escribir Rust es pura alegría. Puedo pasar de una idea a una implementación funcional, probada, robusta, publicada y empaquetada en el tiempo que me llevaría incluso comenzar las primeras líneas de una versión en C. Las herramientas son hermosas, hacen que la programación sea divertida y el resultado final generalmente supera al C equivalente. Una vez que se construye, sé que funcionará perfectamente en todas las plataformas que me importan, y no tengo que ir por ahí probándolas manualmente para encontrar todas las peculiaridades de la plataforma y el compilador que lo romperán.

– [Jonathan Perkins en la lista de correo de NetBSD](http://mail-index.netbsd.org/pkgsrc-users/2024/08/25/msg040053.html)

¡Gracias a [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1617) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [BennyVasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1g0bncp/this_week_in_rust_568/)</small>
