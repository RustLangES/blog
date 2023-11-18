---
title: "Esta semana en Rust #6"
number_of_week: 6
description: Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.
date: 2023-11-15
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---

¡Hola y bienvenidos a otro número de *This Week in Rust*!
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todo el mundo crear software fiable y eficiente.
Este es un resumen semanal de su progreso y comunidad.
¿Quieres que se mencione algo? Etiquétanos en [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) en Twitter o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos una solicitud de extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentra algún error en la edición de esta semana, [envíe un PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Actualizaciones de la comunidad de Rust 🥰

<!-- Estimados colaboradores de la comunidad: Por favor, lea README.md para obtener orientación sobre las presentaciones. Cada enlace enviado debe tener la forma: * [Título de la página enlazada](https://example.com/my_article) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todos modos y solo pide a los editores que seleccionen la categoría. -->

### Oficial
* [Compilación más rápida con el front-end paralelo en nightly](https://blog.rust-lang.org/2023/11/09/parallel-rustc.html)
* [Actualización del Consejo de Liderazgo de noviembre de 2023](https://blog.rust-lang.org/inside-rust/2023/11/13/leadership-council-update.html)
* [Nuestra visión para la especificación de Rust](https://blog.rust-lang.org/inside-rust/2023/11/15/spec-vision.html)

### Fundación
* [La Fundación Rust desarrollará un programa de capacitación y certificación](https://foundation.rust-lang.org/news/the-rust-foundation-to-develop-training-and-certification-program/)

### Actualizaciones de proyectos/herramientas
* [Slint 1.3 lanzado con estilos nativos renovados y API de JavaScript](https://slint.dev/blog/slint-1.3-released)
* [rustc_codegen_gcc: Informe de Progreso #27](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-27)
* [registro de cambios de rust-analyzer #207](https://rust-analyzer.github.io/thisweek/2023/11/13/changelog-207.html)
* [migas de pan 0.1.4](https://crates.io/crates/breadcrumbs)

### Observaciones/Pensamientos
* [¿Por qué Rust en la producción?](https://corrode.dev/why-rust/)
* [Creación de una aplicación web en Rust](https://www.yieldcode.blog/post/building-a-webapp-in-rust/)
* [Rust sin crates.io](https://thomask.sdf.org/blog/2023/11/14/rust-without-crates-io.html)
* [Cómo mejoré mis tiempos de compilación de Rust en un 75%](https://benw.is/posts/how-i-improved-my-rust-compile-times-by-seventy-five-percent)
* [Iterador como alias](https://blog.yoshuawuyts.com/iterator-as-an-alias/)
* [¿Qué es un equipo?](https://blog.yoshuawuyts.com/what-is-a-team/)

### Tutoriales de Rust
* [Construcción e implementación de un generador de sitios estáticos](https://www.shuttle.rs/blog/2023/11/15/ssg-in-rust)
* [Seguimiento del proceso activo actual en Windows con Rust](https://hellocode.co/blog/post/tracking-active-process-windows-rust/)
* [Edge IoT con Rust en ESP: Suscriptor MQTT](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-mqtt-subscriber)
* [Construcción de un servidor de autenticación central con Rust, PostgreSQL, Kafka y gRPC](https://medium.com/@adefemiadeoye/building-a-central-authentication-server-with-rust-postgresql-kafka-and-grpc-f1b44de099ea)
* [Tengo un dúo de Milk-V (y está corriendo Rust)](https://barretts.club/posts/i-got-a-milkv-duo/)
* [video] [Una introducción a Veilid, por Christien Rioux](https://www.youtube.com/watch?v=h288gZTjJOM)
* [video] [Código en Rust con RustRover, por Vitaly Bragilevsky](https://www.youtube.com/watch?v=pnFS0YIKUJ8)
* [video] [¡Crea una CLI ficticia de GitHub en Rust!](https://www.youtube.com/watch?v=pyeUkQg8z9A)

### Miscelánea
* [audio] [RustShip: Corrode.dev y lichi con Matthias Endler](https://ieni.dev/2023/11/%EF%B8%8F-corrode.dev-and-lychee-with-matthias-endler-rustship-5/)

## Crate de la semana

El crate de esta semana es [cargo-msrv](https://github.com/foresterre/cargo-msrv), un subcomando de carga para averiguar la versión mínima admitida de Rust (MSRV) de tu caja.

Llogiq está un poco preocupado por no haber recibido sugerencias durante dos semanas seguidas, pero aún así te ofrece su elección.

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

Se fusionaron 364 solicitudes de extracción en la última semana[fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-06..2023-11-13

* [acelerar 'x limpiar'](https://github.com/rust-lang/rust/pull/117723)
* [llvm-wrapper: eliminar la inclusión de Vectorize.h](https://github.com/rust-lang/rust/pull/117675)
* ['rustc_llvm': Enlace a libkstat en Solaris/SPARC](https://github.com/rust-lang/rust/pull/114224)
* [add -Zcross-crate-inline-threshold=yes](https://github.com/rust-lang/rust/pull/117650)
* [add 'std::hash::{DefaultHasher, RandomState}' exports](https://github.com/rust-lang/rust/pull/115694)
* [añadir una nueva opción 'download-ci-llvm = if-unchanged' y habilitarla por defecto para 'profile = codegen'](https://github.com/rust-lang/rust/pull/116881)
* [permitir configurar el repositorio principal de GitHub](https://github.com/rust-lang/rust/pull/117122)
* [construir un mejor cuerpo MIR cuando se encuentran errores](https://github.com/rust-lang/rust/pull/117418)
* [Diseño de cómputo con intervalos para mejorar los errores de ciclo en corrutinas](https://github.com/rust-lang/rust/pull/117858)
* [Calcular los ámbitos de los préstamos de Polonio sobre el gráfico de la región](https://github.com/rust-lang/rust/pull/117560)
* [cobertura: evitar la creación de intervalos de nombres de macro mal formados](https://github.com/rust-lang/rust/pull/117827)
* [Cobertura: Cambiar el nombre del modo de prueba 'Cobertura-Cobertura' a 'Cobertura-Ejecución'](https://github.com/rust-lang/rust/pull/117700)
* [denegar más límites de rasgos '~const'](https://github.com/rust-lang/rust/pull/117817)
* [extender los argumentos de rasgos incorporados/automáticos con error cuando tienen el argumento \>1](https://github.com/rust-lang/rust/pull/117645)
* [formatear literales de macro const con una impresora bonita](https://github.com/rust-lang/rust/pull/115485)
* [Diseño del generador: ignorar préstamos falsos](https://github.com/rust-lang/rust/pull/117712)
* [dar un mejor diagnóstico para los paréntesis faltantes en los límites de Fn*](https://github.com/rust-lang/rust/pull/117297)
* [maneja el caso cuando no se encuentra el ID de cambio](https://github.com/rust-lang/rust/pull/117263)
* [Mejorar el diagnóstico para const ctors en expresiones repetidas de matrices](https://github.com/rust-lang/rust/pull/113925)
* [hacer que 'FatalErrorMarker' tenga una prioridad más baja que otros pánicos](https://github.com/rust-lang/rust/pull/117557)
* [en caso de error en la expresión de la cadena de métodos, busque el método que falta en los segmentos anteriores de la cadena](https://github.com/rust-lang/rust/pull/115229)
* [solo instanciar Binder durante el sondeo de candidato a rasgo integrado de Dyn una vez](https://github.com/rust-lang/rust/pull/117610)
* [solo use 'normalize_param_env' cuando normalice el predicado en 'check_item_bounds'](https://github.com/rust-lang/rust/pull/117542)
* [patrones: rechaza punteros sin procesar que no son solo números enteros](https://github.com/rust-lang/rust/pull/116930)
* [Recuperación de palabras clave de función mal ordenadas/duplicadas](https://github.com/rust-lang/rust/pull/117282)
* [reordenar las comprobaciones para asegurarse de que la posible expectativa faltante en la Opción/Resultado...](https://github.com/rust-lang/rust/pull/117695)
* [Mensaje de error de restauración de la corrección de compatibilidad de rustc](https://github.com/rust-lang/rust/pull/117724)
* [atrapar '{' en let-chains](https://github.com/rust-lang/rust/pull/117770)
* [sugerir eliminar ';' por ';' dentro de let-chains](https://github.com/rust-lang/rust/pull/117743)
* [correcciones de inseguridad](https://github.com/rust-lang/rust/pull/117229)
* [advertir cuando se usa una característica inestable con -Ctarget-feature](https://github.com/rust-lang/rust/pull/117616)
* [Cuando no encuentre assoc fn en el tipo, busque builder fn](https://github.com/rust-lang/rust/pull/117006)
* [Miri: 'data_race': enlace a documentos para condiciones de carrera 'inusuales'](https://github.com/rust-lang/miri/pull/3155)
* [Miri: FreeBSD añadiendo soporte para intercepción de getentropía](https://github.com/rust-lang/miri/pull/3161)
* [miri: implementar round.ps y round.pd SSE4.1 intrínsecos](https://github.com/rust-lang/miri/pull/3159)
* [Miri: Comparte la corrección de compatibilidad de getentropía en varios Unixes](https://github.com/rust-lang/miri/pull/3162)
* [Miri: Tratar la estática local de la rosca en la rosca principal como raíces estáticas para el análisis de fugas](https://github.com/rust-lang/miri/pull/2931)
* [emitit '#[inline]' on 'derive(Debug)'](https://github.com/rust-lang/rust/pull/117727)
* [estabilizar 'result_option_inspect'](https://github.com/rust-lang/rust/pull/116866)
* [mover 'BorrowedBuf' y 'BorrowedCursor' de 'std:io' a 'core::io'](https://github.com/rust-lang/rust/pull/117694)
* [funciones auxiliares que consumen cierres para ayudantes 'fmt::D ebug'](https://github.com/rust-lang/rust/pull/117730)
* [no entres en pánico en '<BorrowedCursor as io::Write>::write'](https://github.com/rust-lang/rust/pull/115460)
* [Futuros: proporcionan un mecanismo no destructivo para determinar si un sumidero y una corriente están emparejados](https://github.com/rust-lang/futures-rs/pull/2797)
* [codegen-cranelift: implementar los intrínsecos AArch64 necesarios para simd-json](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1416)
* [codegen-cranelift: implemente intrínsecos criptográficos AES-NI y SHA256 usando asm en línea](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1425)
* [codegen-cranelift: implementa una gran cantidad de intrínsecos SIMD](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1417)
* [codegen\_gcc: no emita la sección '.eh_frame' si usa -Cpanic=abort](https://github.com/rust-lang/rustc_codegen_gcc/pull/374)
* [cargo: 'query'{'_vec'} use 'IndexSummary'](https://github.com/rust-lang/cargo/pull/12970)
* [Cargo: Agrega un mejor mensaje de error cuando no puede encontrar la sección de búsqueda](https://github.com/rust-lang/cargo/pull/12865)
* [cargo: agregar recolección de basura en caché](https://github.com/rust-lang/cargo/pull/12634)
* [Credencial de carga: incluir archivos de licencia en todas las cajas publicadas](https://github.com/rust-lang/cargo/pull/12953)
* [cargo: no permitir nombre de característica vacío](https://github.com/rust-lang/cargo/pull/12928)
* [cargo: hacer enlaces de navegador a partir de rutas de archivos HTML](https://github.com/rust-lang/cargo/pull/12889)
* [cargo: filtrar las dependencias 'cargo-credential-*' por sistema operativo](https://github.com/rust-lang/cargo/pull/12949)
* [cargo: arreglar '--quiet' que se usa con subcomandos anidados](https://github.com/rust-lang/cargo/pull/12959)
* [Cargo: Corregir el comportamiento no determinista en la repoblación de último uso](https://github.com/rust-lang/cargo/pull/12958)
* [Cargo: no entres en pánico cuando no puedas analizar rustc commit-hash](https://github.com/rust-lang/cargo/pull/12965)
* [cargo: conservar los descriptores del archivo del servidor de trabajo en la invocación de rustc en 'fix_exec_rustc'](https://github.com/rust-lang/cargo/pull/12951)
* [Cargo: Reportar errores de semver más detallados](https://github.com/rust-lang/cargo/pull/12924)
* [rustdoc: elide correctamente los args del efecto anfitrión de cajas cruzadas](https://github.com/rust-lang/rust/pull/117531)
* [clippy: 'arc_with_non_send_sync' Mejorar la resolución sugerida](https://github.com/rust-lang/rust-clippy/pull/11772)
* [clippy: 'map_identity': respeta la ergonomía del partido](https://github.com/rust-lang/rust-clippy/pull/11792)
* [clippy: 'mod_module_files' No emita pelusa para mod.rs en las pruebas](https://github.com/rust-lang/rust-clippy/pull/11779)
* [clippy: añadir detalles de tipo a la nota 'unnecessary_fallible_conversions'](https://github.com/rust-lang/rust-clippy/pull/11767)
* [clippy: desestructurar 'Conf' en 'register_lints'](https://github.com/rust-lang/rust-clippy/pull/11790)
* [clippy: deshabilita 'vec_box' cuando se usan diferentes asignadores](https://github.com/rust-lang/rust-clippy/pull/11780)
* [clippy: no compruebes si hay variables enlazadas en el último tiempo, comprueba si hay variables enlazadas que escapan](https://github.com/rust-lang/rust-clippy/pull/11760)
* [clippy: corrige la comprobación de divergencia de 'manual_let_else'](https://github.com/rust-lang/rust-clippy/pull/11787)
* [clippy: pelusa 'needless_borrow' y 'explicit_auto_deref' en la mayoría de los accesos al campo de la Unión](https://github.com/rust-lang/rust-clippy/pull/11508)
* [clippy: mover 'suspicious_doc_comments' a doc pass](https://github.com/rust-lang/rust-clippy/pull/11798)
* [clippy: reemplaza 'if_chain' con let chains](https://github.com/rust-lang/rust-clippy/pull/11750)
* [rust-analyzer: add config para preferir/ignorar los módulos de preludio al insertar importaciones](https://github.com/rust-lang/rust-analyzer/pull/15871)
* [Rust-analyzer: Vista previa del campo ADT al pasar el mouse](https://github.com/rust-lang/rust-analyzer/pull/15847)
* [rust-analyzer: find 'Self' reference](https://github.com/rust-lang/rust-analyzer/pull/15864)
* [rust-analyzer: ignora el attr 'doc(hidden)' si no hay ningún cuerpo presente](https://github.com/rust-lang/rust-analyzer/pull/15854)
* [Rust-Analyzer: Lugar de captura de cierre truncado para puntero sin procesar](https://github.com/rust-lang/rust-analyzer/pull/15860)
* [Rust-analyzer: Mejorar la comprobación de la macro de inclusión](https://github.com/rust-lang/rust-analyzer/pull/15866)

### Clasificación del rendimiento del compilador de Rust

Una semana dominada por una mejora de rendimiento en particular que condujo a enormes ganancias de rendimiento: ¡un promedio de mejora del 5% en 121 casos de prueba! La mejora del rendimiento proviene de la adición de una sugerencia '#[inline]' a la salida de '#[derive(Debug)]', lo que presumiblemente permite al compilador eliminar más fácilmente el código muerto, reduciendo el tamaño binario y la cantidad de código que realmente necesita ser generado por código.

Triaje realizado por **@rylev**.
Rango de revisión: [7b97a5ca.. 173b6e68](https://perf.rust-lang.org/?start=7b97a5ca8422d1495a8918106d3249aa405812d4&end=173b6e686b158dbad7d072c64bef3ced2052312b&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.4% | [0.2%, 0.9%] | 10 |
| Regresiones ❌ <br /> (secundaria) | 1.9% | [0,2%, 3,6%] | 12 |
| Mejoras ✅ <br /> (primaria) | -5,6% | [-49,2%, -0,1%] | 111 |
| Mejoras ✅ <br /> (secundaria) | -3,5% | [-25,0%, -0,2%] | 155 |
| Todos ❌✅ (primario) | -5,1% | [-49,2%, 0,9%] | 121 |

2 regresiones, 2 mejoras, 3 mixtas; 3 de ellos en rollups
55 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/e036aa707afc1495783004ee018aada4dfa9d192/triage/2023-11-14.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: fusionar] [RFC: Sustitución de dependencias públicas/privadas](https://github.com/rust-lang/rfcs/pull/3516)

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para mutex_unpoison](https://github.com/rust-lang/rust/issues/96469)
* [disposición: fusionar] [Problema de seguimiento para la coerción de conversión de dyn](https://github.com/rust-lang/rust/issues/65991)
* [disposición: fusionar] [rustdoc-search: añadir soporte para rasgos y tipos asociados](https://github.com/rust-lang/rust/pull/116085)

#### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

#### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay pautas de código inseguro que ingresaron al período final de comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Añadir bases de ruta con nombre a la carga (v2)](https://github.com/rust-lang/rfcs/pull/3529)
* [nuevo] [RFC: Subrayado const asociado](https://github.com/rust-lang/rfcs/pull/3527)
* [nuevo] [Añadir función prohibida convierte RFC](https://github.com/rust-lang/rfcs/pull/3526)
* [nuevo] [Características de destino de estructura RFC](https://github.com/rust-lang/rfcs/pull/3525)
* [nuevo] [Crear 0000-cargo-dns.md](https://github.com/rust-lang/rfcs/pull/3523)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-11-15 - 2023-12-13 🦀

### Virtual

* 15/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Construyendo Nuestras Propias Cerraduras (Atómicas y Cerraduras Capítulo 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 15/11/2023 | Virtual (Richmond, VA, EE. UU.) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 15/11/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 16/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 16/11/2023 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust noviembre de 2023 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/297376463/)
* 16/11/2023 | Virtual (Vilnius, LT) | [Grupo de Meetup de Vilnius Rust and Go](https://www.meetup.com/vilnius-rust-go-meetup-group/)
    * [**Disfruta de nuestro primer evento de Rust**](https://www.meetup.com/vilnius-rust-go-meetup-group/events/297133832/)
* 21/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 21/11/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/296807537/)
* 23/11/2023 | Virtual (Edmonton, AB, CA) | [Grupo de usuarios de Edmonton R - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Reunión del grupo de usuarios de Edmonton R: R y Rust, como una pareja hecha en el cielo**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 28/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 29/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [** ¡Capítulo final del Club de Lectura de Atomics & Locks! (Capítulo 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 30/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 30/11/2023 | Virtual (Dublín, IE) | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Automatización de la experiencia con comprobaciones de carga de carga**](https://www.meetup.com/rust-dublin/events/296346693/)
* 01/12/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 02/12/2023 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 05/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Espejo**](https://berline.rs/)
* 05/12/2023 | Virtual (Búfalo, NY, EE. UU.) | [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)

### Europa

* 21/11/2023 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Procesamiento de GPU en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 23/11/2023 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)
* 07/12/2023 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 30/11/2023 | Bruselas, BE | [Lambda Bruselas](https://lambda-brussels.glitch.me/)
    * [**Lambda Bruselas**](https://lambda-brussels.glitch.me/)

### América del Norte

* 15/11/2023 | Richmond, VA, EE. UU. + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 16/11/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 16/11/2023 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**¡A Python le encanta Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 16/11/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 21/11/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 22/11/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 28/11/2023 | Pasadena, CA, EE. UU. | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Grupo mensual de Rust**](https://www.meetup.com/thursday-go/events/297062186/)
* 12/12/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)

### Oceanía

* 21/11/2023 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Reunión de Christchurch Rust**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 28/11/2023 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Encuentro de noviembre**](https://www.meetup.com/rust-canberra/events/296391733/)
* 11/12/2023 | Perth, WA, AU | [Grupo de Meetup de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Evento de fin de año de Rust**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> decidí seguir aprendiendo Rust porque me gustaba la sintaxis. Me gustó la velocidad. Me gustó la comunidad. Me gustó todo. Se sintió como un soplo de aire fresco: una sintaxis más intuitiva que Python, JavaScript o C, pero aún más rápida.

– [Goren Barak en su blog](https://digital-goobers.vercel.app/posts/learning-rust)

¡Gracias a [Goren Barak](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1488) por la autosugestión!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](REDDIT_LINK_HERE)</small>

