---
title: "Esta semana en Rust #47"
number_of_week: 47
description: La crate de esta semana es esp32-mender-client, un cliente para ESP32 para ejecutar actualizaciones de firmware y comandos remotos.
date: 2025-02-12
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenido a otro número de *esta semana en Rust*! [Rust](https://www.rust-lang.org/)
en lenguaje de programación que permite todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o envíanos un [solicitud extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan la contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

Esta semana en Rust se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición en esta semana, [por favor envía un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandeja e entrada? [Suscríbet aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizaciones de la comunidad en Rust 🥰

### Oficial
* [crates.io: actualización de desarrollo](https://blog.rust-lang.org/2025/02/05/crates-io-development-update.html)

### Fundación
* [Nell Shamrell-Harrington elegida como Presidenta de la Junta Directiva de la Fundación Rust](https://rustfoundation.org/media/nell-shamrell-harrington-elected-as-rust-foundation-board-of-directors-chair/)

### Boletines
* [Este mes en Rust OSDev: enero de 2025](https://rust-osdev.com/this-month/2025-01/)

### Actualizaciones de proyectos/herramientas
* [Motor de juego Fyrox 0.36](https://fyrox.rs/blog/post/fyrox-game-engine-0-36/)
* [Refinado: tipos de refinamiento simples para Rust](https://jordankaye.dev/posts/refined/)
* [Opiniones de los mantenedores sobre Rust-for-Linux](https://lwn.net/SubscriberLink/1007921/9020dbb12585d48f/)
* [Reescribiendo paquetes esenciales de Linux en Rust](https://lwn.net/SubscriberLink/1007907/a9db87fc233bceae/)
* [Nutype 0.6.0 - newtype con garantías y soporte para funciones ahora](https://github.com/greyblake/nutype/releases/tag/v0.6.0)
* [Gluesql v0.6.0 – Aplicación para tomar notas tipo TUI con importante actualización de diseño](https://github.com/gluesql/glues/releases/tag/v0.6.0)
* [Lanzamiento de Gladius 0.2 - editor de código GPLv3, línea de comandos, multicursor con soporte LSP](https://codeberg.org/njskalski/bernardo/src/branch/master/docs/beta_release_notes/beta_1_release.md)

### Observaciones/Pensamientos
* [Un estudio de cada variante de iterador](https://blog.yoshuawuyts.com/a-survey-of-every-iterator-variant)
* [Actualización de una base de código grande a Rust 2024](https://codeandbitters.com/rust-2024-upgrade/)
* [Cómo aprendí a dejar de preocuparme y amar el LLM](https://smallcultfollowing.com/babysteps/blog/2025/02/10/love-the-llm/)
* [Las Tres Reglas Básicas de Seguridad e Higiene](https://jack.wrenn.fyi/blog/safety-hygiene/)
* [video] [Bevy Meetup#1 - Alice - ¿Qué diablos es la reflexión?](https://www.youtube.com/watch?v=vxPKWb0dSqQ)
* [video] [Érase una vez el sans-io](https://www.youtube.com/watch?v=RYHYiXMJdZI)

### Tutoriales de Rust
* [Primeros pasos en el desarrollo de juegos con Rust y Bevy](https://blog.jetbrains.com/rust/2025/02/04/first-steps-in-game-development-with-rust-and-bevy/)
* [Aprovisionamiento de certificados TLS en Rust con ACME](https://www.shuttle.dev/blog/2025/02/06/provisioning-tls-certificates-with-acme-in-rust)
* [Consejo del día #4: Escribir notaciones en los patrones de coincidencia de Rust](https://gaultier.github.io/blog/tip_of_the_day_4.html)
* [De horas a 360 ms: sobre-ingeniería en una solución de rompecabezas](https://blog.danielh.cc/blog/puzzle)
* [Escribir un controlador simple en Rust](https://scorpiosoftware.net/2025/02/08/writing-a-simple-driver-in-rust/)
* [Resolviendo el problema ABA en Rust con punteros etiquetados](https://minikin.me/blog/solving-the-aba-problem-in-rust-tagged-pointers)
* [Mezclar Rust con Java (¡o Kotlin!)](https://tweedegolf.nl/en/blog/147/mix-in-rust-with-java-or-kotlin)
* [Cómo escribir código DRY en Rust](https://baarse.substack.com/p/how-to-write-dry-code-in-rust)

### FOSDEM
* [Rust para Linux](https://fosdem.org/2025/schedule/event/fosdem-2025-6507-rust-for-linux/)
* [¿Reescribiendo el futuro de los paquetes esenciales de Linux en Rust?](https://fosdem.org/2025/schedule/event/fosdem-2025-6196-rewriting-the-future-of-the-linux-essential-packages-in-rust-/)
* [Escribiendo un controlador de Kubernetes... Pero en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4189-writing-a-kubernetes-controller-but-in-rust/)
* [Uso de Rust integrado para construir un dispositivo no atendido que funciona con baterías](https://fosdem.org/2025/schedule/event/fosdem-2025-6300-using-embedded-rust-to-build-an-unattended-battery-powered-device/)
* [Augures: un conjunto de herramientas de series temporales para Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4668-augurs-a-time-series-toolkit-for-rust/)
* [Construcción de un vatímetro y un backend de cohetes](https://fosdem.org/2025/schedule/event/fosdem-2025-5470-building-a-watt-meter-esp-rs-and-a-rocket-backend/)
* [Enorme análisis de gráficos en tu propio servidor con WebGraph en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4773-huge-graph-analysis-on-your-own-server-with-webgraph-in-rust/)
* [Llevando la estética de los terminales a la Web con Rust (y viceversa)](https://fosdem.org/2025/schedule/event/fosdem-2025-5496-bringing-terminal-aesthetics-to-the-web-with-rust-and-vice-versa-/)
* [Abusar de los préstamos para divertirse, obtener ganancias y un recolector de basura de punto seguro](https://fosdem.org/2025/schedule/event/fosdem-2025-4394-abusing-reborrowing-for-fun-profit-and-a-safepoint-garbage-collector/)
* [Lecciones de la reescritura de software de sistemas en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-5088-lessons-from-rewriting-systems-software-in-rust/)
* [Oxidando el programador del kernel de Linux (en el espacio de usuario)](https://fosdem.org/2025/schedule/event/fosdem-2025-4620-rust-ifying-the-linux-kernel-scheduler-in-user-space-/)
* [Aventuras en la oxidación de la gestión de paquetes de Arch Linux](https://fosdem.org/2025/schedule/event/fosdem-2025-6259-adventures-in-oxidizing-arch-linux-package-management/)
* [Modelando la herencia en SeaORM](https://www.sea-ql.org/blog/2025-01-08-sea-orm-inheritance/)

### Miscelánea
* [Informe de empleo de Rust de enero de 2025](https://filtra.io/rust/jobs-report/jan-25)

## `Crate` de la semana

La `crate` de esta semana es [esp32-mender-client](https://github.com/virust-ai/esp32-mender-client), un cliente para ESP32 para ejecutar actualizaciones de firmware y comandos remotos.

¡Gracias a [Kelvin](https://users.rust-lang.org/t/crate-of-the-week/2704/1399) por sugerir su propia crate!

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

### CFP - Proyectos

¿Siempre quisiste contribuir a proyecto de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tarea en la comunida de Rust para que elijas y comiences!

Alguna de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

*Esta semana o se han presentado convocatorias para participar.*

Si eres propietario de un proyect de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que está aceptando presentaciones par unirse a su evento como orador.

*No se han presentado convocatoria ni presentaciones esta semana.*

Si eres organizador de un evento y esperas ampliar su alcance, envía un enlace a la página web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o contactándonos en [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust).!

## Actualizaciones del Proyecto Rust

462 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-02-04..2025-02-11

* [agregar destino amdgpu](https://github.com/rust-lang/rust/pull/134740)
* [`#[contratos::requiere(...)]` + `#[contratos::asegura(...)]`](https://github.com/rust-lang/rust/pull/128045)
* [`rustc_middle`: paralelizar TyCtxt: eliminar "impl unsafe DynSend/DynSync"](https://github.com/rust-lang/rust/pull/136731)
* [añadir nota sobre el rasgo `FnPtr` que se expone como enlace público](https://github.com/rust-lang/rust/pull/136518)
* [permitir usar const en nombres de tipos de patrones](https://github.com/rust-lang/rust/pull/136284)
* [Calcular siempre el diseño de corrutinas para emitir errores de diseño recursivo entusiastamente](https://github.com/rust-lang/rust/pull/136073)
* [evitar llamar a la consulta `layout_of` en `lit_to_const`](https://github.com/rust-lang/rust/pull/136302)
* [evitar usar `make_direct_deprecated()` en extern "ptx-kernel"](https://github.com/rust-lang/rust/pull/133932)
* [comprobar el tamaño del tipo de retorno en WF](https://github.com/rust-lang/rust/pull/136274)
* [compilador: puerta `extern "{abi}"` en `ast_lowering`](https://github.com/rust-lang/rust/pull/136603)
* [Un par de cambios para ejecutar rustc en Miri](https://github.com/rust-lang/rust/pull/136580)
* [Cobertura: aplazar parte de la contra-creación hasta CodeGen](https://github.com/rust-lang/rust/pull/136053)
* [debuginfo para la función ZST debe tener una lineación de 8 bits, no de 1 bit](https://github.com/rust-lang/rust/pull/136640)
* [detectar préstamos (no raw) de punteros ZST nulos en CheckNull](https://github.com/rust-lang/rust/pull/136601)
* [no permitir `repr()` en elementos no válidos](https://github.com/rust-lang/rust/pull/133925)
* [visualización de números enteros sin punteros sin formato y sin `overflowing_literals`](https://github.com/rust-lang/rust/pull/135265)
* [no reiniciar el tipo de conversión si actualizar también el operando en `simplify_cast` en GVN](https://github.com/rust-lang/rust/pull/136450)
* [emitir un error si se solicita `-Zdwarf-version=1`](https://github.com/rust-lang/rust/pull/136746)
* [asegurarse de que no intentamos monomorfizar las llamadas a upcasting o vtable de tipo y posibles](https://github.com/rust-lang/rust/pull/136311)
* [arreglar `rustc_hidden_type_of_opaques` para RPITITs con cuerpo predeterminado](https://github.com/rust-lang/rust/pull/136550)
* [arreglar la pelusa `unreachable_pub` para el objetivo hermitaño](https://github.com/rust-lang/rust/pull/136595)
* [arreglar accidentalmente que ya no emitía lints de literales desbordados en patrones](https://github.com/rust-lang/rust/pull/136393)
* [Sugerencia de corrección para `dependency_on_unit_never_type_fallback` que involucra closures + expansiones de format args](https://github.com/rust-lang/rust/pull/136598)
* [arreglar las comprobaciones de llamada fría wrt `#[track_caller]`](https://github.com/rust-lang/rust/pull/135973)
* [Corregir error de desenvoltura en el literal desbordado](https://github.com/rust-lang/rust/pull/136760)
* [fuchsia: permitir que Rust use números de llamada al sistema de archivos libc](https://github.com/rust-lang/rust/pull/136213)
* [generar el bloque de terminación correcto bajo Wasm EH](https://github.com/rust-lang/rust/pull/136200)
* [introducir `CoercePointeeValidated` para comprobaciones de coerción en la etapa de typeck](https://github.com/rust-lang/rust/pull/136107)
* [etiquetar parámetros coincidentes en el sitio de definición para funciones foráneas](https://github.com/rust-lang/rust/pull/136651)
* [hacer `AsyncFnOnce`, `AsyncFnMut`, `AsyncFn` no-`#[fundamental]`](https://github.com/rust-lang/rust/pull/136724)
* [hacer de `cenum_impl_drop_cast` un error grave](https://github.com/rust-lang/rust/pull/135964)
* [asegurarse de usar el rasgo `Receptor` al extraer candidatos de método objeto](https://github.com/rust-lang/rust/pull/135179)
* [solo resaltar los parámetros coincidentes en el sitio de definición](https://github.com/rust-lang/rust/pull/136583)
* [el paso de barrera alrededor del nuevo solucionador](https://github.com/rust-lang/rust/pull/136269)
* [patrón Migración 2024: intentar sugerir la eliminación de modificadores de enlace redundantes](https://github.com/rust-lang/rust/pull/136577)
* [elegir la versión máxima de DWARF cuando se usan módulos LTO con diferentes versiones](https://github.com/rust-lang/rust/pull/136659)
* [Escríbeme para cambios relacionados con atributos](https://github.com/rust-lang/rust/pull/136643)
* [rechazar literales negativos para tipos sin signo/char en rangos de patrones y literales](https://github.com/rust-lang/rust/pull/136304)
* [Se eliminó la dependencia del eje de desplazamiento de campo, enfoque alternativo](https://github.com/rust-lang/rust/pull/136201)
* [Reportar desajustes genéricos al llamar a las funciones de rasgo con cuerpo](https://github.com/rust-lang/rust/pull/136497)
* [resolver la ruta `llvm-config` correctamente en compilaciones cruzadas](https://github.com/rust-lang/rust/pull/136681)
* [mostrar el formato de sugerencia de diferencias en el reemplazo detallado](https://github.com/rust-lang/rust/pull/127541)
* [algunas limpiezas de `rustc_middle`](https://github.com/rust-lang/rust/pull/136465)
* [algunos ajustes de biblioteca relacionados con la edición miscelánea](https://github.com/rust-lang/rust/pull/136705)
* [transmutabilidad: arreglar IC al pasar ADT incorrecto para ASUMIR](https://github.com/rust-lang/rust/pull/136730)
* [UEFI: Proceso: Agregar soporte para variables de entorno/argumentos](https://github.com/rust-lang/rust/pull/136418)
* [Actualizar ELSA a la versión más reciente](https://github.com/rust-lang/rust/pull/136094)
* [usar +secure-plt para powerpc-unknown-linux-gnu{,spe}](https://github.com/rust-lang/rust/pull/136154)
* [usar `widening_mul` en lugar de una función separada](https://github.com/rust-lang/rust/pull/136409)
* [usar una `Opción` para `FindNextFileHandle` en `ReadDir` en lugar del valor centinela `INVALID_FILE_HANDLE`](https://github.com/rust-lang/rust/pull/136552)
* [usar una cadena Ty corta para errores de binop y unop](https://github.com/rust-lang/rust/pull/136315)
* [visitar toda la información de depuración en MIR Visitor](https://github.com/rust-lang/rust/pull/136722)
* [Bloque de asignación de particiones](https://github.com/rust-lang/rust/pull/136115)
* [miri: permitir que el código llame a `geteuid()`](https://github.com/rust-lang/miri/pull/4179)
* [Miri: Lanzar error UB al invocar una corrección de compatibilidad que no sea vararg con la importación de vararg](https://github.com/rust-lang/miri/pull/4181)
* [miri: usar el bloqueo fcntl en Solaris en lugar de flock que falta allí](https://github.com/rust-lang/miri/pull/4177)
* [implementar `eat_until` aprovechando memchr en lexer](https://github.com/rust-lang/rust/pull/136585)
* [estabilizar `Cursor::{get_mut, set_position}` en escenarios `const`](https://github.com/rust-lang/rust/pull/136634)
* [estabilizar `característica(trait_upcasting)`](https://github.com/rust-lang/rust/pull/134367)
* [Estabilizar la función `map_many_mut`](https://github.com/rust-lang/rust/pull/136152)
* [estabilizar `vec_pop_if`](https://github.com/rust-lang/rust/pull/135488)
* [añadir `unchecked_disjoint_bitor` por ACP373](https://github.com/rust-lang/rust/pull/135760)
* [añadir API de acceso directo a `UnsafeCell`](https://github.com/rust-lang/rust/pull/136398)
* [implementar la función inestable `new_range`](https://github.com/rust-lang/rust/pull/136167)
* [implementar constructores str inherentes](https://github.com/rust-lang/rust/pull/136517)
* [añadir los métodos `cast_signed` y `cast_unsigned` para los tipos `NonZero`](https://github.com/rust-lang/rust/pull/136511)
* [marcar `std::fmt::from_fn` como `#[must_use]`](https://github.com/rust-lang/rust/pull/136502)
* [std: mover código de red a `sys`](https://github.com/rust-lang/rust/pull/136449)
* [`std::fs`: simplificar aún más el manejo de dirent64](https://github.com/rust-lang/rust/pull/136479)
* [optimizar la implementación de `Rc<str>::default()`](https://github.com/rust-lang/rust/pull/136099)
* [cambiar el nombre de los métodos `slice::take...` a `split_off...`](https://github.com/rust-lang/rust/pull/136555)
* [Windows: Eliminar archivos de solo lectura](https://github.com/rust-lang/rust/pull/134679)
* [cargo: no usar en Solaris `libc::LOCK_*` que se eliminaron de libc en la versión...](https://github.com/rust-lang/cargo/pull/15143)
* [cargo: feat: añadir soporte `cargo pkgid` para cargo-script](https://github.com/rust-lang/cargo/pull/14961)
* [Cargo: feat: emitir error si el paquete no se encuentra entre el espacio de trabajo](https://github.com/rust-lang/cargo/pull/15071)
* [cargo: arreglar enlace de registro cambiado](https://github.com/rust-lang/cargo/pull/15142)
* [Cargo: Arreglar la condición de carrera en `panic_abort_tests`](https://github.com/rust-lang/cargo/pull/15169)
* [Cargo: corrección: alinear la primera línea de la lista desordenada con la siguiente](https://github.com/rust-lang/cargo/pull/15161)
* [cargo: corrección: no usar "quiso decir" en los errores](https://github.com/rust-lang/cargo/pull/15138)
* [Cargo: Hacer que el seguimiento de caché sea resistente a archivos inesperados](https://github.com/rust-lang/cargo/pull/15147)
* [Cargo: Simplificar retroceso](https://github.com/rust-lang/cargo/pull/15150)
* [Cargo: Limpieza de pequeños resolutores](https://github.com/rust-lang/cargo/pull/15040)
* [cargo: sugerir nombres de características similares en CLI](https://github.com/rust-lang/cargo/pull/15133)
* [rustdoc: usar ThinVec para piezas de generics](https://github.com/rust-lang/rust/pull/136265)
* [Habilitar la función "Saltar a DEF" en Rustc Docs](https://github.com/rust-lang/rust/pull/136589)
* [rustfmt: reescritura de la función `check_diff`](https://github.com/rust-lang/rustfmt/pull/6390)
* [clippy: añade pelusa `single_option_map`](https://github.com/rust-lang/rust-clippy/pull/14033)
* [clippy: `path_buf_push_overwrite`: marcar sugerencia como `MaybeIncorrect`](https://github.com/rust-lang/rust-clippy/pull/14010)
* [clippy: `useless_asref`: no hacer pelusa si está en closure para cambiar la profundidad de la referencia](https://github.com/rust-lang/rust-clippy/pull/14090)
* [clippy: agregar verificación de MSRV para `lines_filter_map_ok`](https://github.com/rust-lang/rust-clippy/pull/14130)
* [clippy: agregar verificación de MSRV para `manual_flatten`](https://github.com/rust-lang/rust-clippy/pull/14086)
* [clippy: permitir `assign_op_pattern` en la prueba de `string_add`](https://github.com/rust-lang/rust-clippy/pull/14143)
* [clippy: autofix para `range_zip_with_len`](https://github.com/rust-lang/rust-clippy/pull/14136)
* [clippy: cambiar la aplicabilidad de `if_then_some_else_none` a `MachineApplicable`](https://github.com/rust-lang/rust-clippy/pull/14106)
* [clippy: corregir "Affected lints" para `allow-one-hash-in-raw-strings`](https://github.com/rust-lang/rust-clippy/pull/14186)
* [clippy: versión correcta de `doc_overindented_list_items`](https://github.com/rust-lang/rust-clippy/pull/14152)
* [clippy: desaprueba la pelusa redundante `option_map_or_err_ok` y saca `manual_ok_or` de pedante](https://github.com/rust-lang/rust-clippy/pull/14027)
* [clippy: no activar `[size_of_in_element_count]` para `u8`](https://github.com/rust-lang/rust-clippy/pull/14011)
* [clippy: no emitir sugerencia entre macro en `manual_async_fn`](https://github.com/rust-lang/rust-clippy/pull/14142)
* [clippy: usar bloques etiquetados como bloques de nivel superior](https://github.com/rust-lang/rust-clippy/pull/14102)
* [clippy: arreglar ICE en `unnecessary_mut_passed`](https://github.com/rust-lang/rust-clippy/pull/14065)
* [clippy: arregla `let_and_return` con variables temporales, y distingue entre las ediciones de Rust](https://github.com/rust-lang/rust-clippy/pull/14180)
* [clippy: arreglar la sugerencia de `obfuscated_if_else` en el lado izquierdo de expr binario](https://github.com/rust-lang/rust-clippy/pull/14124)
* [clippy: arreglar documentos para `#[clippy::format_args]`](https://github.com/rust-lang/rust-clippy/pull/14161)
* [clippy: corrección: `manual_unwrap_or_default` sugiere falsamente cuando el tipo de condición es incierto](https://github.com/rust-lang/rust-clippy/pull/13889)
* [clippy: manejar casos en `is_normalizable`](https://github.com/rust-lang/rust-clippy/pull/13833)
* [clippy: hacer línea vacía-después de una pelusa clippy temprana](https://github.com/rust-lang/rust/pull/136657)
* [clippy: hacer que `manual_map` ignore los tipos que contienen `dyn`](https://github.com/rust-lang/rust-clippy/pull/12712)
* [clippy: mover `mutex_integer` a restricción y mejorar `mutex_{integer, atomic}` docs](https://github.com/rust-lang/rust-clippy/pull/14110)
* [clippy: omitir `use_self` dentro de las expansiones de macro en bloque `impl Self`](https://github.com/rust-lang/rust-clippy/pull/13128)
* [clippy: mejorar `disallowed_*`](https://github.com/rust-lang/rust-clippy/pull/13669)
* [clippy: usar el cuerpo MIR para identificar más llamadas "equivalentes predeterminadas" para `derivable_impls`](https://github.com/rust-lang/rust-clippy/pull/13988)
* [clippy: usar paréntesis cuando sea necesario en `nonminimal_bool` lint](https://github.com/rust-lang/rust-clippy/pull/14187)
* [rust-analyzer: corregir el error off-by-one en RangeFormatting](https://github.com/rust-lang/rust-analyzer/pull/19124)
* [rust-analyzer: no emitir scip vacío para los incorporados](https://github.com/rust-lang/rust-analyzer/pull/19105)
* [rust-analyzer: arreglar la resolución ID de `use` dentro del cuerpo](https://github.com/rust-lang/rust-analyzer/pull/19094)
* [Rust-analyzer: si el elemento existe en el módulo, resolver como módulo en lugar de tipo](https://github.com/rust-lang/rust-analyzer/pull/19088)
* [Rust-analyzer: Resolver los tipos de proyección antes de verificar los lanzamientos](https://github.com/rust-lang/rust-analyzer/pull/19106)
* [Rust-analyzer: Rango de persistencia ascendente para `convert_tuple_struct_to_named_struct`](https://github.com/rust-lang/rust-analyzer/pull/18912)
* [rust-analyzer: line-index: no intentar usar (no disponible) neo en big-endian arch64](https://github.com/rust-lang/rust-analyzer/pull/19083)
* [rust-analyzer: opción para desactivar las sugerencias de tipo en cascada para los parámetros de cierre](https://github.com/rust-lang/rust-analyzer/pull/19104)
* [Rust-analyzer: Organizar capítulos en la barra lateral de MDBoer](https://github.com/rust-lang/rust-analyzer/pull/19115)
* [Analizador de Rust: evitar que los pánicos derriben las roscas de los trabajadores](https://github.com/rust-lang/rust-analyzer/pull/19093)
* [Rust-Analyzer: Dividir la caché en distintas fases](https://github.com/rust-lang/rust-analyzer/pull/19084)
* [rust-analyzer: usar la mutabilidad interior para `ProcMacro::expanders` cargados](https://github.com/rust-lang/rust-analyzer/pull/19099)

### Clasificación del rendimiento del compilador de Rust

Una semana relativamente neutra, con pocos cambios reales, pero la mayoría pequeños en
magnitud. El cambio más significativo es el movimiento de `rustdoc` de la unificación de JS/CSS 
en tiempo de compilación, lo que redujo los tiempos de generación de documentos en la mayoría de los puntos
de referencia de manera bastante significativa.

Triaje realizado por **@simulacrum**.
Rango de revisión: [01e4f19c.. C03C38D5](https://perf.rust-lang.org/?start=01e4f19cc8027925ffe0885a86388b700e46bfab&end=c03c38d5c2368cd2aa0e056dba060b94fc747f4e&absolute=false&stat=instructions%3Au)

3 regresiones, 5 mejoras, 1 mixta; 2 de ellos en rollups
32 comparaciones  artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-02-10.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se probaron para su implementación esta semana:

* *No se probaron RFC esta semana.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar función `num_midpoint_signed`](https://github.com/rust-lang/rust/pull/134340)
* [No duplicar la lista de tipos asociados proporcionada por `dyn` principal](https://github.com/rust-lang/rust/pull/136458)
* [Regresión: un valor de tipo `HashMap<Pulse, u64>` no se puede construir a partir de](https://github.com/rust-lang/rust/issues/135669)
* [Problema de seguimiento para `inherent_str_constructors`](https://github.com/rust-lang/rust/issues/131114)
* [Problema de seguimiento para `os_str_display`](https://github.com/rust-lang/rust/issues/120048)
* [No permitir atributos en los patrones de reposición de estructura](https://github.com/rust-lang/rust/pull/136490)
* [feat(core): impl `Step` for `NonZero<u*>`](https://github.com/rust-lang/rust/pull/127534)
* [Aviso de incompatibilidad futura `unsupported_fn_ptr_calling_conventions`: Advertir también en dependencias](https://github.com/rust-lang/rust/pull/135767)
* [Informe de Mir de las filtraciones de `Box`/`Vec`/`String`](https://github.com/rust-lang/rust/pull/135811)
* [Problema de seguimiento para `integer_sign_cast`](https://github.com/rust-lang/rust/issues/125882)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problema de seguimiento de Cargo o PR que ingresó al período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna propuesta de equipo lingüístico entró en el Período Final de Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia idiomática que ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problema de seguimiento de pautas de código inseguro o PR que ingresó al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [#[doc(consts)]](https://github.com/rust-lang/rfcs/pull/3770)

## Próximos eventos

Evento Rust entre 2025-02-12 - 2025-03-12 🦀

### Virtual
* 2025-02-12 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Encuentro en línea conjunto con Rust Bristol!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/305833952)
* 13/02/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 14/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Club de Codificadores Elegante y Curioso Cooperativo](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305815307)
* 19/02/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rust en Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Procedencia del puntero**](https://www.meetup.com/vancouver-rust/events/304051783)
* 2025-02-20 | Híbrido (Redmond, WA, EE. UU.) | [Grupo de Usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrero de 2025 SRUG (Grupo de Usuarios de Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Club de Codificadores Elegante y Curioso Cooperativo](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbcc)
* 25/02/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de Usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361428)
* 25/02/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: El complicado mundo de String en Rust**](https://www.meetup.com/women-in-rust/events/305716182)
* 25/02/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mitad de Rustful—Everett Pompeii presenta Bencher 🐰**](https://www.meetup.com/rustdc/events/305170682)
* 27/02/2025 | Virtual (EE. UU.) | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Introducción a Rust**](https://www.eventbrite.com/e/intro-to-rust-tickets-1237517059839)
* 27/02/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820295)
* 27/02/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Analizando opciones de línea de comando con teoría de categorías y asíncrono**](https://www.meetup.com/charlottesville-rust-meetup/events/305948365)
* 01/03/2025 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-62876317658/)
* 05/03/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - co-instanciamiento social**](https://www.meetup.com/indyrs/events/302031659)
* 06/03/2025 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820281/)
* 06/03/2025 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos con Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #9**](https://www.meetup.com/bevy-game-development/events/306131557)
* 11/03/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunión de Usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/303522529)
* 11/03/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 La comunidad se pone en marcha**](https://www.meetup.com/women-in-rust/events/305716839)

### Asia
* 24/02/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Presencial Rust febrero 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305570131)
* 01/03/2025 | Bangalore/Bangalore, IN | [Rust en Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro de Zig y Rust**](https://lu.ma/460w8v58)

### Europa
* 2025-02-12 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 14/02/2025 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust y Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/305791536)
* 18/02/2025 | Leipzig, SN, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introducción a la Programación Contextual-Genérica en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 18/02/2025 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Febrero: Reunión previa a la conferencia de Rust Nation 2025**](https://www.meetup.com/rust-london-user-group/events/306036448)
* 2025-02-19 - 2025-02-20 | Londres, Reino Unido | [Nación Rust Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation Reino Unido 2025**](https://www.rustnationuk.com/)
* 2025-02-20 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #1 @Puzzle ITC**](https://www.meetup.com/rust-bern/events/305597994)
* 21/02/2025 | Londres, Reino Unido | [Rust Global: Londres 2025](https://rustfoundation.org/event/rust-global-london-2025/)
    * [**Rust Global: Londres 2025**](https://rustfoundation.org/event/rust-global-london-2025/)
* 2025-02-22 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #9**](https://www.meetup.com/stockholm-rust/events/305848723)
* 25/02/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**Rust es de cero: Cargo y tipos**](https://www.meetup.com/madrust/events/305896258)
* 26/02/2025 | Darmstadt, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Ajuste del compilador de Rust**](https://www.meetup.com/rust-rhein-main/events/305990886/)
* 27/02/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809675)
* 27/02/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentro de Rust #75**](https://www.meetup.com/rust-paris/events/305791655)
* 01/03/2025 | Nürnberg, DE | [Rust en Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Speyer**](https://www.meetup.com/rust-noris/events/305361732/)
* 05/03/2025 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**17º Encuentro de BcnRust**](https://www.meetup.com/bcnrust/events/305887675)
* 12/03/2025 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunión de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/305045445)

### América del Norte
* 13/02/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Arañando Wikipedia educadamente en Rust asíncrono**](https://www.meetup.com/pdxrust/events/306044189)
* 14/02/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en el centro de Rust, 14 de febrero**](https://www.meetup.com/bostonrust/events/305804954)
* 18/02/2025 | San Francisco, CA, EE. UU. | [Grupo de Estudio de Rust en San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)
* 2025-02-20 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306087854)
* 2025-02-20 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 2: Fundamentos de desarrollo de juegos**](https://www.meetup.com/music-city-rust-developers/events/304333047)
* 2025-02-20 | Redmond, WA, EE. UU. | [Grupo de Usuarios de Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrero de 2025 SRUG (Grupo de Usuarios de Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y ciencia de datos**](https://www.meetup.com/rust-mx/events/305793010)
* 2025-02-22 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Union Square en Somerville, 22 de febrero**](https://www.meetup.com/bostonrust/events/305805059)
* 26/02/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)
* 27/02/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Comenzando la reunión de nuevo**](https://www.meetup.com/rust-atl/events/305776081)
* 02/03/2025 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo de Rust en Beacon Hill, 2 de marzo**](https://www.meetup.com/bostonrust/events/305805164)
* 06/03/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**CRDTs en Rust**](https://www.meetup.com/stl-rust/events/305187783)
* 2025-03-10 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en Davis Square, 10 de marzo**](https://www.meetup.com/bostonrust/events/305805192)

### Oceanía
* 24/02/2025 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Encuentro de febrero de 2025 en Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/306040785)
* 25/02/2025 | Barton, AC, AU | [Grupo de Usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentro de febrero**](https://www.meetup.com/rust-canberra/events/306090406)
* 04/03/2025 | Perth, WA, AU | [Grupo de encuentro de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**Cómo Orica está usando Rust en su lugar de trabajo**](https://www.meetup.com/perth-rust-meetup-group/events/306131753)

Si estás organizando un evento de Rust, agrégalo al [calendario] para que se mencione aquí.
Por favor, recuerda agregar un enlace al evento también.
Envía un correo electrónico para solicitar acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos

Por favor, consulte el último hilo en [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Fras e la semana

> El hecho de que la cosas sean útiles no significa que sea mágicamente sólidas.

– [Ralf Jung en github](https://github.com/rust-lang/rust/issues/132442#issuecomment-2636065726)

¡Gracias a [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1658) por la sugerencia!

[¡Por favor, envía tus citas y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*El alojamiento de la lista correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ioajhc/this_week_in_rust_586_this_week_in_rust/)</small>
