---
title: "Esta semana en Rust #2"
number_of_week: 2
description: Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.
date: 2023-10-18
rfc_date: Wed, 18 Oct 2023 00:00:00 -0400
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

### Fundación
* [Resumen del tercer trimestre de 2023 de Rebecca Rumbul](https://foundation.rust-lang.org/news/q3-2023-recap-from-rebecca-rumbul/)

### Boletines informativos
* [Este mes en Rust OSDev: septiembre de 2023](https://rust-osdev.com/this-mes/2023-09/)

### Actualizaciones de proyectos/herramientas
* [Anuncio de EtherCrab, una implementación de Rust del protocolo de automatización industrial EtherCAT](https://wapl.es/announcing-ethercrab-the-rust-ethercat-controller/)
* [registro de cambios de rust-analyzer #203](https://rust-analyzer.github.io/thisweek/2023/10/16/changelog-203.html)

### Observaciones/Pensamientos
* [¿Por qué Rust asíncrono?](https://without.boats/blog/why-async-rust/)
* [Compile Times and Code Graphs](https://blog.danhhz.com/compile-times-and-code-graphs)
* [Containerizar aplicaciones de Rust en Ubuntu y Alpine, con GitHub Actions](https://medium.com/@vapor.schitcrafter/containerise-rust-applications-on-ubuntu-alpine-with-github-actions-or-docker-builders-9378a02b98fd)

### Tutoriales de Rust
* [Un nivel de tipo contiene una operación para una lista heterogénea que utiliza tipos asociados](https://blog.weiznich.de/blog/eurorust-non-overlapping-contains-for-hlists/)
* [Uso de GraphQL en Rust](https://www.shuttle.rs/blog/2023/10/16/graphql-in-rust)
* [Escribir analizadores sintácticos en Winnow](https://www.youtube.com/watch?v=QF3kMyzMC40)

### Investigación
* [Yuga: Detección automática de errores de anotación de por vida en el lenguaje Rust](https://arxiv.org/abs/2310.08507)
* [Análisis rápido de todo el programa basado en resúmenes para identificar accesos inseguros a la memoria en Rust](https://arxiv.org/abs/2310.10298)

### Miscelánea
* [Reflexiones de Eurorust](https://smallcultfollowing.com/babysteps/blog/2023/10/14/eurorust-reflexiones/)
* [Reflexiones de EuroRust 2023: ¿Para qué sirve una conferencia?](https://lucumr.pocoo.org/2023/10/14/eurorust-whats-a-conference/)
* [audio] [RustShip: Graphite - Gráficos Rasterizados y Vectoriales en Rust](https://ieni.dev/2023/10/%EF%B8%8F-graphite-raster-and-vector-graphics-in-rust-rustship-4/)

## Crate de la semana

El crate de esta semana es [rinf](https://github.com/cunarist/rinf), una biblioteca para escribir Rust en Flutter.

¡Gracias a [Kim Dong-Hyun](https://users.rust-lang.org/t/crate-of-the-week/2704/1249) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->
* [Hyperswitch (Hacktoberfest) - [OpenNode] Conversión de unidades monetarias](https://github.com/juspay/hyperswitch/issues/2240)
* [Hyperswitch (Hacktoberfest) - [Stax] Conversión de unidades monetarias](https://github.com/juspay/hyperswitch/issues/2246)
* [Hyperswitch (Hacktoberfest) - [ACI] Conversión de unidades monetarias](https://github.com/juspay/hyperswitch/issues/2198)
* [Ockam - Hacer que 'ockam space show' (sin argumentos) sea interactivo pidiendo al usuario que elija de una lista de nombres de espacios para mostrar (tuify)](https://github.com/build-trust/ockam/issues/6472)
* [Ockam - Mejorar el texto 'ockam tcp-inlet delete --help' (comando 'clap')](https://github.com/build-trust/ockam/issues/6645)
* [Ockam - Inscribir "correo electrónico: no se permite el carácter '+'"](https://github.com/build-trust/ockam/issues/6095)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

Se presentaron 409 solicitudes de incorporación de cambios [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-09..2023-10-16

* ['const_eval': permite firmas de puntero de función que contengan &mut T en contextos const](https://github.com/rust-lang/rust/pull/116015)
* [Limpiezas 'rustc_hir_pretty'](https://github.com/rust-lang/rust/pull/116625)
* [añadir la devolución de llamada 'Config::hash_untracked_state'](https://github.com/rust-lang/rust/pull/116731)
* [agregue la extensión V (vector) a la especificación de destino riscv64-linux-android](https://github.com/rust-lang/rust/pull/116618)
* [también considere call y yield como MIR SSA](https://github.com/rust-lang/rust/pull/113915)
* [ampliar las consecuencias de la inicialización recursiva de TLS](https://github.com/rust-lang/rust/pull/116172)
* [limpiar 'rustc_features' un poco más](https://github.com/rust-lang/rust/pull/116550)
* [calcular los ámbitos de préstamo de NLL utilizando el modelo de Polonio](https://github.com/rust-lang/rust/pull/113218)
* [const-eval: permitir llamar a funciones con características targat deshabilitadas en tiempo de compilación en WASM](https://github.com/rust-lang/rust/pull/116576)
* [const-eval: hacer que la desalineación sea un error grave](https://github.com/rust-lang/rust/pull/115524)
* [Cobertura: Separe la extracción inicial del intervalo del procesamiento del intervalo](https://github.com/rust-lang/rust/pull/116409)
* [Detectar cierre de estilo rubí en el analizador](https://github.com/rust-lang/rust/pull/116645)
* [No comprobar si hay predicados imposibles en const-prop lint](https://github.com/rust-lang/rust/pull/116315)
* [no haga UB en la deref ptr colgante, en su lugar verifique los entrantes en las proyecciones](https://github.com/rust-lang/rust/pull/114330)
* [exhaustividad: reelaboración de la división del constructor](https://github.com/rust-lang/rust/pull/116391)
* [Manejar explícitamente la fuga de rasgos automáticos en coherencia](https://github.com/rust-lang/rust/pull/116689)
* [arreglar el estado de salida / estado de espera en plataformas 'cfg(unix)' que no sean Unix](https://github.com/rust-lang/rust/pull/115108)
* [Corregir la comprobación de desbordamiento en los patrones de rango](https://github.com/rust-lang/rust/pull/116623)
* [manejar varios atributos '#[diagnostic::on_unimplemented]' correctamente](https://github.com/rust-lang/rust/pull/116642)
* [implementar la opción de exclusión de '-Clink-self-contained=-linker'](https://github.com/rust-lang/rust/pull/116014)
* [Mejorar el diagnóstico de check-cfg](https://github.com/rust-lang/rust/pull/116666)
* [Mejorar el manejo de errores de aserción con condiciones muy largas](https://github.com/rust-lang/rust/pull/116548)
* [en smir use 'FxIndexMap' para almacenar identificadores indexados](https://github.com/rust-lang/rust/pull/116560)
* [enlazador: también pasa los indicadores de compresión debuginfo](https://github.com/rust-lang/rust/pull/116702)
* [hacer que las revisiones de "solicitar cambios" se apliquen 'S-waiting-on-author'](https://github.com/rust-lang/rust/pull/116661)
* [en caso de error de tipo que implique el cierre, evite ICE](https://github.com/rust-lang/rust/pull/116676)
* [en el error de tipo del argumento de llamada de cierre, apunte a llamadas anteriores que afectaron a la inferencia](https://github.com/rust-lang/rust/pull/116250)
* [opt-dist: deshabilitar características no utilizadas para crates en mesa](https://github.com/rust-lang/rust/pull/116790)
* [Pasar las marcas de corrección de compatibilidad de rustc usando la variable de entorno](https://github.com/rust-lang/rust/pull/116448)
* [evitar más pelusas de patrón inalcanzables espurias](https://github.com/rust-lang/rust/pull/116715)
* [evitar que se muestren métodos de impls generales de rasgos extranjeros no disponibles para que aparezcan en los resultados de búsqueda](https://github.com/rust-lang/rust/pull/116597)
* [evitar pelusas espurias de 'patrón inalcanzable'](https://github.com/rust-lang/rust/pull/115937)
* [relacionar alias ty con varianza](https://github.com/rust-lang/rust/pull/116219)
* [eliminar 'DefiningAnchor::Bubble' de la comprobación de wf opaco](https://github.com/rust-lang/rust/pull/116802)
* [muestra el discriminante 'enum' si se usa un repr compatible](https://github.com/rust-lang/rust/pull/116600)
* [estabilizar 'async fn' y return-position 'impl Trait' en trait](https://github.com/rust-lang/rust/pull/115822)
* [normalizar estructuralmente para el cierre](https://github.com/rust-lang/rust/pull/116436)
* [sugiera agregar 'return' si el para semi que puede coaccionar al tipo de retorno fn](https://github.com/rust-lang/rust/pull/115196)
* [sugerir el bloque de etiquetado si 'break' está en el bloque desnudo](https://github.com/rust-lang/rust/pull/116366)
* [sugerir límites de rasgos para el tipo asociado utilizado en el parámetro de tipo](https://github.com/rust-lang/rust/pull/116257)
* [soporte AIX en la biblioteca estándar de Rust](https://github.com/rust-lang/rust/pull/109882)
* [use 'PatKind::Error' cuando un valor const de ADT tenga violación](https://github.com/rust-lang/rust/pull/116522)
* [use la variable env para controlar los identificadores de subprocesos en 'rustc_log'](https://github.com/rust-lang/rust/pull/116586)
* [agregar la capacidad de obtener líneas/nombre de archivo para Span en smir](https://github.com/rust-lang/rust/pull/116630)
* [miri: implementa los intrínsecos 'llvm.x86.sse41.*'](https://github.com/rust-lang/miri/pull/3118)
* [miri: hacer que la generación de NaN no sea determinista](https://github.com/rust-lang/rust/pull/116551)
* [copiar matrices de 1 elemento como escalares, no vectores](https://github.com/rust-lang/rust/pull/116510)
* [optimizar 'librustc_driver.so' con BOLT](https://github.com/rust-lang/rust/pull/116352)
* [optimizar el archivo leído en 'Config::verify'](https://github.com/rust-lang/rust/pull/116635)
* [optimizar la compresión sobre los iteradores de matriz](https://github.com/rust-lang/rust/pull/115515)
* [estabilizar 'atomic_from_ptr'](https://github.com/rust-lang/rust/pull/115719)
* [estabilizar 'const_maybe_uninit_assume_init_read'](https://github.com/rust-lang/rust/pull/116233)
* [stabilize '{IpAddr, Ipv6Addr}::to_canonical'](https://github.com/rust-lang/rust/pull/115955)
* [impl Not, Bit{And,Or}{,Assign} para direcciones IP](https://github.com/rust-lang/rust/pull/113747)
* [impl Default for ExitCode](https://github.com/rust-lang/rust/pull/114589)
* [añadir invariante a 'Vec::p op' que lleva la tapa '<' si aparece correctamente](https://github.com/rust-lang/rust/pull/114370)
* [implementar 'BufRead' para 'VecDeque<u8>'](https://github.com/rust-lang/rust/pull/110604)
* [implementar 'OnceCell/Lock::try_insert()'](https://github.com/rust-lang/rust/pull/116540)
* [implementar 'slice::split_once' y 'slice::rsplit_once'](https://github.com/rust-lang/rust/pull/112818)
* [añadir variantes de 'String::from_utf16' explicit-endian](https://github.com/rust-lang/rust/pull/95967)
* [implemente FusedIterator para DecodeUtf16 cuando el iterador interno lo haga](https://github.com/rust-lang/rust/pull/110729)
* [implementar 'sys::args' para UEFI](https://github.com/rust-lang/rust/pull/116341)
* [en línea 'Bytes::next' y 'Bytes::size_hint'](https://github.com/rust-lang/rust/pull/116775)
* [make 'try_exists' return 'Ok(true)' para Windows Unix Sockets](https://github.com/rust-lang/rust/pull/116683)
* [marque 'new_in' como 'const' para las colecciones BTree](https://github.com/rust-lang/rust/pull/116559)
* [regex-automata/meta: revertir la ampliación de la optimización del sufijo inverso](https://github.com/rust-lang/regex/pull/1111)
* [regex-lite: ajustar el límite de anidamiento en la prueba de desbordamiento de pila](https://github.com/rust-lang/regex/pull/1106)
* [regex: aflojar las reglas compatibles con ASCII + mejorar la optimización del sufijo inverso](https://github.com/rust-lang/regex/pull/1105)
* [regex, regex-automata: corrige la compilación de doctests en arquitecturas de 32 bits](https://github.com/rust-lang/regex/pull/1107)
* [regex-lite: corrige la compilación de doctests en arquitecturas de 32 bits](https://github.com/rust-lang/regex/pull/1101)
* [regex: revertir las optimizaciones recientes del conjunto de intervalos de sintaxis de expresiones regulares](https://github.com/rust-lang/regex/pull/1102)
* [cargo: 'fix(install)': Sugerir una versión alternativa en caso de fallo de MSRV](https://github.com/rust-lang/cargo/pull/12798)
* [cargo: añadir un mensaje detallado cuando la ruta de la carpeta de destino no es válida](https://github.com/rust-lang/cargo/pull/12820)
* [cargo: añadir el nombre y la versión del paquete a los mensajes de advertencia](https://github.com/rust-lang/cargo/pull/12799)
* [cargo: admite la configuración de dependencias 'públicas' con el espacio de trabajo deps](https://github.com/rust-lang/cargo/pull/12817)
* [rustfmt: soporte let-chains](https://github.com/rust-lang/rustfmt/pull/5910)
* [rustdoc-search: añadir desambiguador impl a los elementos asociados duplicados](https://github.com/rust-lang/rust/pull/109422)
* [rustdoc: oculta '#[repr(transparent)]' si no forma parte de la ABI pública](https://github.com/rust-lang/rust/pull/115439)
* [rustdoc: mostrar el nombre de la crate junto al logotipo más pequeño](https://github.com/rust-lang/rust/pull/115948)
* [clippy: 'get_first': pelusa en rebanadas no primitivas](https://github.com/rust-lang/rust-clippy/pull/11609)
* [clippy: 'manual_is_ascii_check': Comprueba también 'is_ascii_hexdigt'](https://github.com/rust-lang/rust-clippy/pull/11659)
* [clippy: 'unnecessary_lazy_eval': reduce la aplicabilidad si el cierre tiene una anotación de tipo de retorno](https://github.com/rust-lang/rust-clippy/pull/11673)
* [clippy: arreglar ICE en la pelusa interna del autor](https://github.com/rust-lang/rust-clippy/pull/11664)
* [rust-analyzer: add 'replace_is_ok_with_if_let_ok' assist](https://github.com/rust-lang/rust-analyzer/pull/15752)
* [rust-analyzer: add 'replace_is_some_with_if_let_some' assist](https://github.com/rust-lang/rust-analyzer/pull/15743)
* [rust-analyzer: agregar mensajes de diagnóstico para caracteres y errores literales de bytes](https://github.com/rust-lang/rust-analyzer/pull/15744)
* [rust-analyzer: haga que el cursor seleccione en '_tmp'](https://github.com/rust-lang/rust-analyzer/pull/15755)
* [rust-analyzer: diagnóstico de literales de cadena](https://github.com/rust-lang/rust-analyzer/pull/15746)

### Clasificación del rendimiento del compilador de Rust

En general, una semana interesante en cuanto a rendimiento, con pequeñas mejoras en un vasto
número de puntos de referencia que parecen superar a un conjunto aislado de índices de referencia (ligeramente)
Regresiones. Incluía una serie de recuentos de instrucciones de regresión de PR, pero no
no importa para los tiempos de ciclo, además de una misteriosa regresión a 'check_match' y
'mir_borrowck' de la reelaboración de la división del constructor (consulte el informe sobre la 116391 de PR para
detalles), y un impresionante conjunto de mejoras a partir de la inserción automática
pequeñas funciones en todas las crates (consulte el informe sobre PR 116505 para obtener más detalles).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [84d44dd1.. B9832E72](https://perf.rust-lang.org/?start=84d44dd1d8ec1e98fff94272ba4f96b2a1f044ca&end=b9832e72c9223f4e96049aa5911effd258b92591&absolute=false&stat=instructions%3Au)

4 regresiones, 1 mejoras, 4 mixtas; 3 de ellos en rollups
84 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/420012f0bb12281b5a3e897280d3f38b241a4735/triage/2023-10-18.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](http®s://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposición: fusionar] [Edición 2024](https://github.com/rust-lang/rfcs/pull/3501)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para result_option_inspect](https://github.com/rust-lang/rust/issues/91345)
* [disposición: fusionar] [Permitir valores parcialmente movidos en coincidencia](https://github.com/rust-lang/rust/pull/103208)
* [disposición: fusionar] ['read_dir' tiene un comportamiento inesperado para '""'](https://github.com/rust-lang/rust/issues/114149)
* [disposición: fusionar] [rustdoc: alinear la insignia de estabilidad con la línea de base en lugar de con la parte inferior](https://github.com/rust-lang/rust/pull/105666)

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [RFC: Sustitución de dependencias públicas/privadas](https://github.com/rust-lang/rfcs/pull/3516)
* [nuevo] [añadir semántica flotante RFC](https://github.com/rust-lang/rfcs/pull/3514)
* [nuevo] [Reserve la palabra clave 'gen' en la edición de 2024 e inicie una implementación experimental de los generadores 'Iterator'](https://github.com/rust-lang/rfcs/pull/3513)

### [Llamada para pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-10-18 - 2023-11-15 🦀

### Virtual

* 18/10/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Primitivas del sistema operativo (Atomics & Locks Capítulo 8)**](https://www.meetup.com/rust-y-c-plus-plus-in-cardiff/events/296531173/)
* 18/10/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Pinning**](https://www.meetup.com/vancouver-rust/events/295057159/)
* 19/10/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcnbzb/)
* 19/10/2023 | Virtual (Stuttgart, DE) | [Comunidad Rust Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 24/10/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679778/) | [**Espejo**](https://berline.rs/)
* 24/10/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidación de fin de mes—Diviértete con 🍌 y 🔎!**](https://www.meetup.com/rustdc/events/296217448/)
* 31/10/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcnbpc/)
* 01/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS con Bevy Game Engine**](https://www.meetup.com/rust-y-c-plus-plus-in-cardiff/events/296583207/)
* 01/11/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 02/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 09/11/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 15/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Construyendo Nuestras Propias Cerraduras (Atómicas y Cerraduras Capítulo 9)**](https://www.meetup.com/rust-y-c-plus-plus-en-cardiff/events/296582223/)
* 15/11/2023 | Virtual (Richmond, VA, EE. UU.) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)

### Asia

* 18/10/2023 | Tokio, JP | [Reunión de Rust en Tokio](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Rust y la era de los lenguajes de alta integridad**](https://www.meetup.com/tokyo-rust-meetup/events/296551482)
* 20/10/2023 | Singapur, SG | [Encuentro de Rust Singapur](https://www.eventbrite.com/e/rust-meetup-google-developer-space-tickets-736345678747)
    * [**Rust meetup @ Google Developer Space**](https://www.eventbrite.com/e/rust-meetup-google-developer-space-tickets-736345678747)
* 21/10/2023 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**Rust 101 y Decoding Fearless Concurrency**](https://www.meetup.com/rust-pune/events/296765951/)

### Europa

* 19/10/2023 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust Amsterdam](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Amsterdam Meetup @ Terraform**](https://www.meetup.com/rust-amsterdam-group/events/296495570/)
* 19/10/2023 | Wrocław, PL | [Rust de Breslavia](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #35**](https://www.meetup.com/rust-wroclaw/events/296507983/)
* 20/10/2023 | Saarbrücken, DE | [Sarre herrumbroso](https://www.meetup.com/rust-saar/)
    * [**Sesión de codificación en vivo de Rustlings**](https://www.meetup.com/rust-saar/events/296800552/)
* 24/10/2023 | Bucarest, RO | [Reunión de Rust Lang Bucarest](https://www.meetup.com/rust-lang-bucharest-meetup/)
    * [**Rust Bucarest Meetup #4**](https://www.meetup.com/rust-lang-bucarest-meetup/events/296789945/)
* 25/10/2023 | Dublín, IE | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Biome, herramientas de desarrollo web con Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 25/10/2023 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [Rust para la web - Encuentro en París #61](https://mobilizon.fr/events/149c0367-66cb-42c6-aa0c-8495bf6d2a52)
* 25/10/2023 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/zagreb-rust-meetup)
    * [Rust Meetup 2023/10: Lunático](https://www.meetup.com/zagreb-rust-meetup/events/296765355/)
* 26/10/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)
* 26/10/2023 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust en la Universidad Técnica de Delft**](https://www.meetup.com/rust-nederland/events/296488286/)
* 26/10/2023 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #4 en SFEIR**](https://www.meetup.com/meetup-group-zgphbyet/events/296766699/)
* 30/10/2022 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Aira + Netlight**](https://www.meetup.com/Stockholm-Rust/events/296578336/)
* 01/11/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Rust-Meetup Noviembre, Reserva la fecha**](https://www.meetup.com/rustcologne/events/296540949/)
* 07/11/2023 | Bruselas, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Edición para principiantes de Rust and Talk**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 09/11/2023 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/296567395)
* 09/11/2023 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)

### América del Norte

* 18/10/2023 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de la Universidad de Boston**](https://www.meetup.com/bostonrust/events/296223807/)
* 19/10/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369976/)
* 19/10/2023 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust va a donde le plazca Pt2 - ¡Rust en la interfaz!**](https://www.meetup.com/music-city-rust-developers/events/296254420/)
* 19/10/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Grupo de usuarios de Seattle Rust - Reunión de octubre**](https://www.meetup.com/seattle-rust-user-group/events/296110729)
* 24/10/2023 | Pasadena, CA, EE. UU. | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Grupo mensual de Rust**](https://www.meetup.com/thursday-go/events/296422277)
* 25/10/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)
* 25/10/2023 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/296657993/)
* 04/11/2023 | Boulder, CO, EE. UU. | [Reunión de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**¡Hagamos un bot de Discord!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 15/11/2023 | Richmond, VA, EE. UU. + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)

### Oceanía

* 17/10/2023 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296324436/)
* 9/10/2023 | Sídney, Nueva Gales del Sur, Australia | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Día 🤯 de la demolición **](https://www.meetup.com/rust-sydney/events/296672158/)
* 26/10/2023 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**Reunión de octubre**](https://www.meetup.com/rust-brisbane/events/296628243/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> Cuando los tiempos de compilación de Rust se vuelven más lentos después de agregar algunas macros de procedimiento:
>
> A eso lo llamamos la sintaxis 🦀

– [Janet en fosstodon.org](https://fosstodon.org/@janet/111223564960983226)

¡Gracias a [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1472) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/17b86sn/this_week_in_rust_517/)</small>

