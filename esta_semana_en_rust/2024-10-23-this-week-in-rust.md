---
title: "Esta semana en Rust #32"
number_of_week: 32
description: El crate de esta semana es trait-gen, una macro de atributos para generar las implementaciones de rasgos para varios tipos sin necesidad de macros declarativas personalizadas, repetición de código o implementaciones generales.
date: 2024-10-23
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
* [Anunciando Rust 1.82.0](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html)

### Actualizaciones de proyectos/herramientas
* [Ratatui 0.29.0](https://ratatui.rs/highlights/v029/)
* [Vida de una extensión Zed: Rust, Ingenio, Wasm](https://zed.dev/blog/zed-decoded-extensions)
* [La nueva plataforma de Shuttle: redefiniendo el desarrollo de backend](https://www.shuttle.dev/blog/2024/10/10/shuttle-redefining-backend-development)
* [Hifitime versión 4.0.0: Un salto adelante en la gestión del tiempo](https://nyxspace.com/blog/2024/10/17/hifitime-version-400-a-leap-forward-in-time-management/)
* [Fjall 2.2 - ahora soporta el aislamiento de instantáneas serializables (transacciones multi-escritor)](https://fjall-rs.github.io/post/announcing-fjall-22/)
* [pg-extras-rs - Información sobre el rendimiento de la base de datos PostgreSQL](https://github.com/pawurb/pg-extras-rs)

### Observaciones/Pensamientos
* [Los objetivos de diseño de Rust deberían ser sobre el código](https://tmandry.gitlab.io/blog/posts/the-main-thing/)
* [Desanclar](https://without.boats/blog/unpin-cell/)
* [El código de bloqueo es una abstracción con fugas](https://notgull.net/blocking-leaky/)
* [Colaborador del proyecto Life as Rust](https://yaah.dev/staying-involved)
* [Rustls supera a OpenSSL y BoringSSL](https://www.memorysafety.org/blog/rustls-performance-outperforms/)
* [Usando los atlas de texturas de libgdx en Bevy](https://rustunit.com/blog/2024/10-21-bevy-libgdx-atlas/)
* [audio] [Rust en producción - Zed con Conrad Irwin](https://corrode.dev/podcast/s03e01-zed/)
* [audio] [Asignadores asíncronos](https://sdr-podcast.com/episodes/async-allocators/)
* [audio] [PubNub con Stephen Blum](https://rustacean-station.org/episode/stephen-blum/)

### Tutoriales de Rust
* [Desmitificando la alineación y el diseño de la memoria en Rust](https://garden.christophertee.dev/blogs/Memory-Alignment-and-Layout/Part-1)
* [Uso de Rust en servidores que no son Rust para mejorar el rendimiento](https://github.com/pretzelhammer/rust-blog/blob/master/posts/rust-in-non-rust-servers.md)
* [Rust asíncrono en tres partes](https://jacko.io/async_intro.html)
* [ ¿Cuándo debo usar String vs &str? ](https://steveklabnik.com/writing/when-should-i-use-string-vs-str/)
* [Uso de Web Workers en Rust Webapps](https://allwright.io/#/blog/20241016-using-web-workers.md)

### Miscelánea
* [Iniciar una comunidad de Rust](https://www.understandingrecruitment.com/knowledge-hub/blog/the-rust-review-starting-a-rust-community/)

## Crate de la semana

El crate de esta semana es [trait-gen](https://crates.io/crates/trait-gen), una macro de atributos para generar las implementaciones de rasgos para varios tipos sin necesidad de macros declarativas personalizadas, repetición de código o implementaciones generales.

¡Gracias a [Luke Peterson](https://users.rust-lang.org/t/crate-of-the-week/2704/1358) por la sugerencia!

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

### Ecosistema de cajas
* [Solicitud de entrada: 'zerocopy': ¿Necesita soporte para 'IntoBytes' en 'union's?  Por favor, opine.](https://github.com/google/zerocopy/discussions/1802)

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Rama — añadir "Denegar todo" Dns Resolver](https://github.com/plabayo/rama/issues/329)
* [Rama — ampliar el soporte para secuestrar basado en datos de contexto](https://github.com/plabayo/rama/issues/328)
* [Rama — soporte vec/array impl para DnsResolver](https://github.com/plabayo/rama/issues/332)
* [Rama — Admite modos IP en conector y resolución](https://github.com/plabayo/rama/issues/331)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

*No se han presentado convocatorias ni presentaciones esta semana.*

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se fusionaron 464 solicitudes de extracción en la última semana[fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-15..2024-10-22

* [hacer que 'rustc_abi' se compile en estable de nuevo](https://github.com/rust-lang/rust/pull/131997)
* [atributo 'optimize' aplicado a cosas distintas de métodos/funciones/c...](https://github.com/rust-lang/rust/pull/131814)
* ['rust_for_linux: -Zregparm=<N>' bandera de línea de comandos para X86](https://github.com/rust-lang/rust/pull/130432)
* ['rustc_llvm': Arreglar argumentos CLI aplanados](https://github.com/rust-lang/rust/pull/131805)
* [añadir '&pin (mut|const) T' posición de tipo azúcar](https://github.com/rust-lang/rust/pull/130635)
* [añadir getentropía para RTEMS](https://github.com/rust-lang/rust/pull/131774)
* [Se agregaron más escenarios donde se debe eliminar la coma en la función arg](https://github.com/rust-lang/rust/pull/126588)
* [permitir '#[denegar]' dentro de '#[prohibir]' como un no-op](https://github.com/rust-lang/rust/pull/121560)
* [permitir la eliminación del principal de dyn](https://github.com/rust-lang/rust/pull/131857)
* [especifique siempre 'llvm_abiname' para los objetivos RISC-V](https://github.com/rust-lang/rust/pull/131807)
* [autodiff Upstream - enzyme frontend](https://github.com/rust-lang/rust/pull/129458)
* [Cambia la sugerencia huérfana de "sólo" a "cualquier tipo descubierto dentro..."](https://github.com/rust-lang/rust/pull/128391)
* [comprobar si hay directivas filecheck en los archivos marcados como 'skip-filecheck'](https://github.com/rust-lang/rust/pull/131927)
* [compilador: adopta las implicaciones del analizador de Rust para 'LayoutCalculatorError'](https://github.com/rust-lang/rust/pull/131942)
* [compilador: error en el diseño de enumeraciones con reprs no válidos](https://github.com/rust-lang/rust/pull/131843)
* [compilador: use el soporte Comdat de LLVM](https://github.com/rust-lang/rust/pull/131876)
* [continúa deshazte de 'ty::Const::{try_}eval*'](https://github.com/rust-lang/rust/pull/130950)
* [Cobertura: hacer que la creación de contadores maneje los contadores de nodo/borde de manera más uniforme](https://github.com/rust-lang/rust/pull/131918)
* [por defecto al modelo de código medio en el destino LoongArch de OpenHarmony](https://github.com/rust-lang/rust/pull/131874)
* [retrasar la resolución ambigua del enlace intra-doc después de que se haya llenado la 'Caché'](https://github.com/rust-lang/rust/pull/131691)
* [No ejecutar la prueba donde no se puede ejecutar](https://github.com/rust-lang/rust/pull/131835)
* [no marcar el objetivo de destamaño en la validación MIR cuando permanecen opacos](https://github.com/rust-lang/rust/pull/130989)
* [no reportar el mensaje de 'on_unimplemented' por rasgos negativos](https://github.com/rust-lang/rust/pull/131701)
* [No informe de error de bivarianza al anidar una 'estructura' con errores de campo en otra 'estructura'](https://github.com/rust-lang/rust/pull/131754)
* [no ice cuando calcule la cobertura del cuerpo de cierre asíncrono sintético](https://github.com/rust-lang/rust/pull/131802)
* [no consideres predicados que pueden ser imposibles en 'is_impossible_associated_item'](https://github.com/rust-lang/rust/pull/131840)
* [habilitar la instrumentación XRay para objetivos LoongArch Linux](https://github.com/rust-lang/rust/pull/131818)
* [corregir error de coherencia para tuplas™ muy grandes](https://github.com/rust-lang/rust/pull/132001)
* [Arreglar el acceso de campo engañoso al rango](https://github.com/rust-lang/rust/pull/131537)
* [manejar con gracia verdadero/falso en 'cfg(objetivo(..))' compacto](https://github.com/rust-lang/rust/pull/131771)
* [Implementar restricciones ergonómicas de la edición 2024 para partidos](https://github.com/rust-lang/rust/pull/131381)
* [hacer de 'unsupported_calling_conventions' un error grave](https://github.com/rust-lang/rust/pull/129935)
* [hacer que se ejecuten destructores en tramas 'extern "C"'](https://github.com/rust-lang/rust/pull/129582)
* [hacer que algunos métodos flotantes sean inestables 'const fn'](https://github.com/rust-lang/rust/pull/130568)
* [asegúrese de que los opacos exteriores capturen la vida útil de los opacos internos incluso con una sintaxis de captura precisa](https://github.com/rust-lang/rust/pull/131789)
* [nunca emitas 'vptr' para las características empty/auto](https://github.com/rust-lang/rust/pull/131864)
* [registrar 'src/tools/unicode-table-generator' como una herramienta ejecutable](https://github.com/rust-lang/rust/pull/131647)
* [Eliminar diagnósticos de ayuda no válidos para el puntero const](https://github.com/rust-lang/rust/pull/127675)
* [devuelve valores mayores que 2 registros usando un puntero de área de retorno](https://github.com/rust-lang/rust/pull/131211)
* [Configurando el acceso indirecto a datos externos para Loongarch64-linux-{musl,ohos}](https://github.com/rust-lang/rust/pull/131583)
* [intente mejorar los mensajes de error que involucran alias en el solucionador](https://github.com/rust-lang/rust/pull/131699)
* [Advertir menos sobre lo no exhaustivo en FFI](https://github.com/rust-lang/rust/pull/116863)
* [miri: 'epoll_ctl': arroja error no soportado en código de operación no soportado](https://github.com/rust-lang/miri/pull/3982)
* [Miri: Android: Añadido soporte para el manejo de nombres de hilos PRCTL](https://github.com/rust-lang/miri/pull/3899)
* [Miri: Mejorar el soporte para 'F16' y 'F128'](https://github.com/rust-lang/miri/pull/3977)
* [agregar ruta rápida al calcular la visibilidad predeterminada](https://github.com/rust-lang/rust/pull/131686)
* [use 'ThinVec' para el almacenamiento de PredicateDuty](https://github.com/rust-lang/rust/pull/131422)
* [Finalización de la estabilización de 'result_ffi_guarantees'](https://github.com/rust-lang/rust/pull/130628)
* [estabilizar las API de procedencia estricta y procedencia expuesta](https://github.com/rust-lang/rust/pull/130350)
* [estabilizar '-Znext-solver=coherence' de nuevo](https://github.com/rust-lang/rust/pull/130654)
* [añadir constructores 'from_ref' y 'from_mut' a 'core::p tr::NonNull'](https://github.com/rust-lang/rust/pull/130822)
* [añadir 'must_use' a 'CommandExt::exec'](https://github.com/rust-lang/rust/pull/131833)
* [evite usar importaciones en 'thread_local_inner!'](https://github.com/rust-lang/rust/pull/131866)
* [marcar la const inestable 'LazyCell::into_inner'](https://github.com/rust-lang/rust/pull/131712)
* [optimizar 'Box::d efault' y 'Arc::d efault' para construir más tipos en su lugar](https://github.com/rust-lang/rust/pull/131460)
* [optimizar str.reemplazar](https://github.com/rust-lang/rust/pull/130223)
* [estabilizar parcialmente 'const_pin'](https://github.com/rust-lang/rust/pull/130136)
* [refactorizar algunas macros 'core::fmt'](https://github.com/rust-lang/rust/pull/131730)
* [evite las comprobaciones superfluas de UB en 'IndexRange'](https://github.com/rust-lang/rust/pull/131572)
* [relaja un orden de memoria en 'once_box'](https://github.com/rust-lang/rust/pull/131746)
* [Acelerar el recorrido del directorio en Windows](https://github.com/rust-lang/rust/pull/131972)
* [std: uefi: añadir variables básicas de Env](https://github.com/rust-lang/rust/pull/127462)
* [UEFI: implementa getcwd y chdir](https://github.com/rust-lang/rust/pull/129794)
* [cargo: registry: HttpRegistry 'block_until_ready' regresa temprano cuando el trabajo aún está pendiente](https://github.com/rust-lang/cargo/pull/14694)
* [cargo: resolver: evite clonar al iterar usando RcVecIter](https://github.com/rust-lang/cargo/pull/14690)
* [cargo: estabilizar la configuración de resolución compatible con MSRV](https://github.com/rust-lang/cargo/pull/14639)
* [rustdoc-json-types: introducen la función rustc-hash](https://github.com/rust-lang/rust/pull/131936)
* [rustdoc-json-types: marcar enumeraciones simples como copia](https://github.com/rust-lang/rust/pull/131976)
* [rustdoc: cambiar de FxHash a sha256 para hash de archivos estáticos](https://github.com/rust-lang/rust/pull/131908)
* [rustfmt 'for<'a> async' correctamente](https://github.com/rust-lang/rust/pull/131657)
* [rustfmt: 'compile_rustfmt' reescritura](https://github.com/rust-lang/rustfmt/pull/6275)
* [rustfmt: aplicar el algoritmo de clasificación de la versión 2024 a los mods](https://github.com/rust-lang/rustfmt/pull/6368)
* [rustfmt: aplazar cambios para funciones de argumento cero hasta 'style_edition=2027'](https://github.com/rust-lang/rustfmt/pull/6362)
* [clippy: agregue lint para el retorno '&str' limitado de por vida innecesario](https://github.com/rust-lang/rust-clippy/pull/13395)
* [clippy: permitir pasar a través de la página clippy lints sin javascript](https://github.com/rust-lang/rust-clippy/pull/13539)
* [clippy: cambia la categoría de 'manual_is_power_of_two' a 'pedante'](https://github.com/rust-lang/rust-clippy/pull/13553)
* [clippy: deja de linting 'manual_bits' en cualquier invocación de macro](https://github.com/rust-lang/rust-clippy/pull/13564)
* [rust-analyzer: añadir el tipo de retorno wrap/unwrap en la opción](https://github.com/rust-lang/rust-analyzer/pull/18294)
* [analizador de Rust: sujete 'Posición::carácter' a la longitud de la línea](https://github.com/rust-lang/rust-analyzer/pull/18243)
* [rust-analyzer: no considere match/let/ref of place that evaluate to ! para divergir, no permita coerciones de ellos también](https://github.com/rust-lang/rust-analyzer/pull/18278)
* [Rust-analyzer: Mejores terminaciones para bloques externos](https://github.com/rust-lang/rust-analyzer/pull/18360)
* [rust-analyzer: goto definición en operadores de rango](https://github.com/rust-lang/rust-analyzer/pull/18362)
* [rust-analyzer: soporte inicial para 'safe_kw' en bloques externos](https://github.com/rust-lang/rust-analyzer/pull/18350)
* [rust-analyzer: soporte para la configuración initializeStopped](https://github.com/rust-lang/rust-analyzer/pull/18359)
* [Rust-analyzer: arreglar el mensaje de la barra de estado que no se marca como Markdown](https://github.com/rust-lang/rust-analyzer/pull/18366)
* [rust-analyzer: clasificar 'seguro' como una palabra clave contextual](https://github.com/rust-lang/rust-analyzer/pull/18354)
* [rust-analyzer: arreglar el error de asignación descendente de tokens para las entradas include!](https://github.com/rust-lang/rust-analyzer/pull/18361)
* [Rust-analyzer: los elementos privados se muestran en las terminaciones de los módulos en el cuerpo de FN](https://github.com/rust-lang/rust-analyzer/pull/18337)

### Clasificación del rendimiento del compilador de Rust

Algunas mejoras ordenadas al cambiar al solucionador de rasgos de próxima generación (únicamente para verificar la coherencia) y al simplificar nuestro marco de análisis de flujo de datos. Hubo algunas regresiones de tamaño binario asociadas con 126557 PR (agregando '#[track_caller]' a los métodos de asignación de 'Vec' y 'VecDeque'), que he entregado a T-libs para que elijan si investigar más a fondo.

Triaje realizado por **@pnkfelix**.
Rango de revisión: [5ceb623a.. 3e33bda0](https://perf.rust-lang.org/?start=5ceb623a4abd66e91e7959d25caaf0523f1a7f7c&end=3e33bda0326586a6e1e34d0f5c060ca6d116e6a4&absolute=false&stat=instructions%3Au)

0 Regresiones, 3 Mejoras, 6 Mixtas; 3 de ellos en rollups
47 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/81de3d5e2cc599cc49bc11c64f9a5b911f3a83dd/triage/2024-10-21.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [RFC: Permitir literales booleanos como predicados 'cfg'](https://github.com/rust-lang/rfcs/pull/3695)

### Período final de comentarios
Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposición: fusionar] [RFC: Dar a los usuarios control sobre la unificación de características](https://github.com/rust-lang/rfcs/pull/3692)
* [disposición: fusionar] [[RFC] Valores de campo predeterminados](https://github.com/rust-lang/rfcs/pull/3681)
* [disposición: fusionar] [Restricciones de impl del método de rasgos](https://github.com/rust-lang/rfcs/pull/3678)

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Agregar lint contra comparaciones de punteros de función](https://github.com/rust-lang/rust/pull/118833)
* [disposición: fusionar] [Implementar 'From<&mut {slice}>'1 para 'Box/Rc/Arc<{slice}>'](https://github.com/rust-lang/rust/pull/129329)
* [disposición: fusionar] [Problema de seguimiento para 'const_arguments_as_str'](https://github.com/rust-lang/rust/issues/103900)
* [disposición: no especificada] [Agregar implementaciones de LowerExp y UpperExp a NonZero](https://github.com/rust-lang/rust/pull/131377)
* [disposición: fusionar] [Estabilizar 'Ipv6Addr::is_unique_local' y 'Ipv6Addr::is_unicast_link_local'](https://github.com/rust-lang/rust/pull/129238)
* [disposición: fusionar] [Hacer público 'std::os::d arwin'](https://github.com/rust-lang/rust/pull/123723)
* [disposición: fusionar] [Problema de seguimiento para 'const_char_encode_utf16'](https://github.com/rust-lang/rust/issues/130660)
* [disposición: fusionar] [tipos flotantes: mover copysign, abs, signum a libcore](https://github.com/rust-lang/rust/pull/131304)
* [disposición: fusionar] [Problema de seguimiento para '{u8,i8,...}::isqrt'](https://github.com/rust-lang/rust/issues/116226)
* [disposición: fusionar] [Añadir '--print host-triple' para imprimir el triple del objetivo del host](https://github.com/rust-lang/rust/pull/125579)
* [disposición: fusionar] [Lint contra '&T' a '&mut T' y '&T' a '&UnsafeCell<T>' transmuta](https://github.com/rust-lang/rust/pull/128351)
* [disposición: fusionar] [Pelusa contra la obtención de punteros de temporales eliminados inmediatamente](https://github.com/rust-lang/rust/pull/128985)
* [disposición: cerrar] [Considerar la desaprobación de UB-happy 'static mut'](https://github.com/rust-lang/rust/issues/53639)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hay problemas de seguimiento de carga ni PR ingresaron al período de comentarios finales esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: no especificada] [los elementos 'estáticos' distintos nunca se superponen](https://github.com/rust-lang/reference/pull/1657)

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Derivaciones y atributos no seguros](https://github.com/rust-lang/rfcs/pull/3715)
* [nuevo] [Campos de macrofragmentos](https://github.com/rust-lang/rfcs/pull/3714)
* [nuevo] [Ordenación de campo DST relajada](https://github.com/rust-lang/rfcs/pull/3713)

## Próximos eventos

Eventos oxidados entre 2024-10-23 - 2024-11-20 🦀

### Virtual
* 24/10/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [**Parte 4 de 4 - Hackathon Showcase: Proyectos Finales y Presentaciones**](https://www.meetup.com/women-in-rust/events/303213835/)
* 24/10/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633271/)
* 25/10/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304100127/)
* 26/10/2024 | Virtual (Gdansk, PL) | [Stacja IT trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 29/10/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/301585671/)
* 31/10/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Elaborando intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 31/10/2024 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 01/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbcb/)
* 02/11/2024 | Virtual( Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 06/11/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031651/)
* 07/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633272/)
* 08/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 12/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346985/)
* 14/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 14/11/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Encuentro de noviembre**](https://www.meetup.com/join-srug/events/304166747/)
* 15/11/2024 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativa del Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 19/11/2024 | Virtual (Los Ángeles, CA, EE. UU.) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discusión - Tema: Rust para UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 19/11/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346971/)
* 20/11/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Taller de Rust incrustado**](https://www.meetup.com/vancouver-rust/events/304047664/)

### Asia
* 29/10/2024 | Tokio, JP | [Encuentro de Rust en Tokio](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Simultaneidad sin errores en Rust**](https://www.meetup.com/tokyo-rust-meetup/events/304107583/)

### Europa
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
* 14/11/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 19/11/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)

### América del Norte
* 23/10/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 26/10/2024 | Newark, Nueva Jersey, Estados Unidos | [Código de NJ y café](https://www.meetup.com/nj-code-coffee/)
    * [**Introducción a Rust: Construye un juego de aventuras de texto x NJ Code & Coffee**](https://www.meetup.com/nj-code-coffee/events/304149949/)
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
* 30/10/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Workshop: desplegando tu código**](https://www.meetup.com/deep-dish-rust/events/304071348/)
* 04/11/2024 | Brookline, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust en Coolidge Corner Brookline, 4 de noviembre**](https://www.meetup.com/bostonrust/events/303708387/)
* 07/11/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Desarrollo de juegos con Rust y el motor Bevy**](https://www.meetup.com/stl-rust/events/302371464/)
* 12/11/2024 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 15/11/2024 | Ciudad de México, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 15/11/2024 | Somerville, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, 15 de noviembre**](https://www.meetup.com/bostonrust/events/303708398/)

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

> Tu problema es que estás tratando de pedir prestado a los muertos.

– [/u/masklinn en /r/rust](https://old.reddit.com/r/rust/comments/1g3a2ul/hey_rustaceans_got_a_question_ask_here_422024/lrzqed7/)

¡Gracias a [Maciej Dziardziel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1622) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](REDDIT_LINK_HERE)</small>
