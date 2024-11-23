---
title: "Esta semana en Rust #36"
number_of_week: 36
description: El crate de esta semana es fixed-slice-vec, un Vec de longitud dinámica sin estándar con capacidad máxima determinada por el tiempo de ejecución respaldada por un segmento.
date: 2024-11-20
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
* [Anunciando cuatro nuevos miembros del equipo compilador](https://blog.rust-lang.org/inside-rust/2024/11/12/compiler-team-new-members.html)

### Fundación
* [Anunciando a la nueva directora de proyectos de la Fundación Rust: Carol Nichols](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-newest-project-director-carol-nichols/)
* [Rust Foundation colabora con la iniciativa de AWS para verificar las bibliotecas estándar de Rust](https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries/)

## EuroRust 2024
* [A través del fuego y las llamas - Jon Gjengset](https://youtu.be/8-KLX1PGg8Q)
* [Construir más grande en menos tiempo: pruebas de código más allá de lo básico - Predrag Gruevski](https://youtu.be/3EFue8PDyic)
* [Una suave introducción a las macros procedimentales - Sam Van Overmeire](https://youtu.be/02vpyrR1hqk)
* [Rust práctico para audio web - Attila Haraszti](https://youtu.be/RTjQFJ5XmKg)
* [Documentos aumentados: una carta de amor a rustdoc y docs.rs - Francois Mockers](https://youtu.be/Uy13wS9VyNU)
* [El Impacto de los Asignadores de Memoria en el Rendimiento: Una Inmersión Profunda - Arthur Pastel](https://youtu.be/pJ-FRRB5E84)
* [Demostrando la expansión macro con expandible - Sasha Pourcelot](https://youtu.be/0cbclh4F2VU)
* [Runtime Scripting for Rust Applications - Niklas Korz](https://youtu.be/M8dpH3rO-2M)
* [Desatando 🦀 la fortuna interior - Víctor Ciura](https://youtu.be/kiG5-LzIQ54)
* [Los primeros seis años en el desarrollo de Polonio - Amanda Stjerna](https://youtu.be/uCN_LRcswts)
* [Rust no binario: entre lo seguro y lo inseguro - Boxy Uwu](https://youtu.be/KGLsKpMHJdw)
* [Escribir un controlador de tarjeta SD en Rust - Johnathan Pallant](https://youtu.be/-ewuFNKIAVI)
* [Mi Viaje De WebDev a la Visualización Médica Rustacean - David Peherstorfer](https://youtu.be/ZzQaVH-9Dzs)
* [Código para contratar a código: haciendo APIs blindadas - Adam Chalmers](https://youtu.be/bjgGboWCTDw)
* [Rust Irgendwie, Irgendwo, Irgendwann - Henk Oordt](https://youtu.be/aI2UXOcaRhw)
* [Linting with Dylint - Samuel Moelius](https://youtu.be/MjlPUA7sAmA)

## [RustConf 2024](https://www.youtube.com/playlist?list=PL2b0df3jKKiTWZeF7cip6ZUsaVXxWioRi)
* [Dra. Rebecca Rumbul (Directora Ejecutiva de la Fundación Rust): "Palabras de bienvenida"](https://youtu.be/wTV0WCLERGg)
* [Aeva Black: "Hacer que el código abierto sea seguro por diseño" | CONFERENCIA MAGISTRAL](https://youtu.be/-4UD-yGrv5s)
* [Marc-André Moreau (CTO, Devolutions): Charla del Patrocinador Diamante](https://youtu.be/6JtaVM7Pyjg)
* [Nick Cameron: "El eterno resplandor de la mente oxidada"](https://youtu.be/83CoPbrvvKE)
* [Jack Wrenn: "Gafas de seguridad para alquimistas"](https://youtu.be/HyRrbHN6BdY)
* [Rohit Dandamundi: "Ensanchando la red de la fortuna"](https://youtu.be/YREBjo-d2Ac)
* [Isabel Atkinson: "Rustifique su API: Un viaje desde la especificación hasta la implementación"](https://youtu.be/1nXW-mYGTiM)
* [Sparrow Li: "El estado actual y el futuro del rendimiento del compilador de Rust"](https://youtu.be/Lye2xeJ3O5w)
* [Nathan Stocks: "¡Estrellas fugaces! Codifica en vivo un juego en menos de 30 minutos"](https://youtu.be/Ee-VWKtkmVg)
[Pedro Rittner y Sean Lawlor: "Actores y fábricas en Rust"](https://youtu.be/zQ6EyQJRxIs)
* [David Koloski: "Los (muchos) errores que cometí en rkyv"](https://youtu.be/ON4z2LbTD-4)
* [Kyler Chin: "Cómo construimos un mapa de transporte público en tiempo real oxidado"](https://youtu.be/Lc8lBMEJQdo)
* [Adam Chalmers: "Creación de un lenguaje de programación para el diseño 3D"](https://youtu.be/f11kfaKAPzw)
* [Martin Pool: "Encontrando bichos con mutantes de carga"](https://youtu.be/PjDHe-PkOy8)
* [1Password, Adobe, Woven by Toyota: Patrocinador de Oro Lightning Talks](https://youtu.be/EY2KT0QZnkg)
* [Miguel Ojeda (Rust para Linux): KEYNOTE](https://youtu.be/FRMJzNYut4g)
* [JetBrains, K2 Space, Zed: Patrocinador de Oro Lightning Talks](https://youtu.be/rME_t6Jn_Kw)
* [Jonathan Pallant: "Ciclo de seis relojes por píxel - Gráficos en el Neotrol Pico"](https://youtu.be/W45_KnLZ804)
* [Joannah Nanjekye: "Interoperabilidad de Rust: Seguridad de la memoria a través de los límites de funciones extranjeras"](https://youtu.be/ohG-qxd4x6s)
* [Jacob Pratt: "Desarrollo impulsado por compiladores: haciendo que Rust funcione para ti"](https://youtu.be/_oaGNy3_798)
* [Angus Morrison: "Cómo Rust está impulsando los simuladores de misiones espaciales de próxima generación"](https://youtu.be/sAqNvH19Sxo)
* [Michael Gattozzi: "¿Qué pasa cuando se ejecuta la construcción de carga?"](https://youtu.be/fOApf4ZMX4w)
* [Pallavi Thukral: "Rust en movimiento: construcción de sistemas robóticos confiables y de alto rendimiento"](https://youtu.be/a82TJDjUZn0)
* [Marc-André Giroux: "Observabilidad de baja sobrecarga en servidores de alto RPS"](https://youtu.be/TfJMXXBUvAQ)
* [Predrag Gruevski: "Poner fin a los cambios accidentales que rompen el semVer"](https://youtu.be/KKf14ZXyTSo)
* [Chris Biscardi: "Sitios Web, Aplicaciones Web y Ensamblaje Web"](https://youtu.be/geH69jl8vOY)
* [Nicholas Matsakis (Co-Líder, Equipo de Diseño de Rust): "Hoja de ruta de Rust 2.0" | CONFERENCIA MAGISTRAL](https://youtu.be/7YjomcXNvTk)
* [Frédéric Ameye: "El Rust en las industrias reguladas heredadas"](https://youtu.be/_uYOd3ExJII)
* [Walter Pearce: "Amigo, ¿dónde está mi C?"](https://youtu.be/LZli45PPlss)
* [Ed Jones: "Refactorización intrépida y el arte de la oxidación sin argumentos"](https://youtu.be/39utxTvS6hE)
* [Dra. Rebecca Rambul: Palabras de apertura](https://youtu.be/1jGOoinjde4)
* [Charla patrocinada por OxidOS](https://youtu.be/I_A1Q5ynU9U)
* [Martin Geisler: "Entrenamiento de Rust a escala"](https://youtu.be/7h5KyMqt2-Q)
* [Quanyi Ma: "Abrazando la evolución de Monorepo y LLM"](https://youtu.be/qHcfiCmcIf8)
* [Joshua Liebow-Feeser: "Seguridad en un mundo inseguro"](https://youtu.be/qd3x5MCUrhw)
[Jack Huey y James Munns: "An Outsider's Guide to the Rust Project"](https://youtu.be/kXtL_YSZ0Xs)

### Boletines
* [Este mes en Rust OSDev: octubre de 2024](https://rust-osdev.com/this-month/2024-10/)

### Actualizaciones de proyectos/herramientas
* [hiper en rizo necesita un campeón](https://seanmonstar.com/blog/hyper-in-curl-needs-a-champion/)
* [godot-rust actualización de desarrollo de noviembre de 2024](https://godot-rust.github.io/dev/november-2024-update/)
* [Seguridad en hickory-dns](https://ferrous-systems.com/blog/hickory-dns-client/)
* [Geometría virtual en Bevy 0.15](https://jms55.github.io/posts/2024-11-14-virtual-geometry-bevy-0-15/)
* [Pegamento v0.5 - Pestañas del editor y comandos Vim mejorados](https://github.com/gluesql/glues/releases/tag/v0.5.0)
* [Análisis de datos de streaming, versión 0.13.0 de Fluvio](https://www.fluvio.io/news/this-week-in-fluvio-0066)
* [Repetición 0.20 - Datos geoespaciales y soporte completo para H.264](https://rerun.io/blog/maps)
* [¡Lanzamiento de Git-Cliff 2.7.0! (un generador de registro de cambios altamente personalizable)](https://git-cliff.org/blog/2.7.0)   

### Observaciones/Pensamientos
* [No necesitas (siempre) async](https://blog.veeso.dev/blog/en/you-don't-always-need-async/)
* [El WASM zlib más rápido](https://trifectatech.org/blog/fastest-wasm-zlib/)
* [Un bicho de solidez de rustc en la naturaleza](https://specy.app/blog/posts/a-rustc-soundness-bug-in-the-wild)
* [audio] [Compile Time Crimes](https://sdr-podcast.com/episodes/compile-time-crimes/)
* [audio] [Oxide con Steve Klabnik](https://corrode.dev/podcast/s03e03-oxide/)

### Tutoriales de Rust
* [Optimizaciones de Zed Rope, Parte 1](https://zed.dev/blog/zed-decoded-rope-optimizations-part-1)
* [Futexes en casa](https://specificprotagonist.net/jvm-futex.html)
* [Construya su propio SQLite, Parte 3: Análisis sintáctico SQL 101](https://blog.sylver.dev/build-your-own-sqlite-part-3-sql-parsing-101)
* [dtype_dispatch: un truco muy hermoso](https://graphallthethings.com/posts/dtype-dispatch)
* [Envío de eventos a Bevy desde cualquier lugar](https://rustunit.com/blog/2024/11-15-bevy-channel-trigger/)
* [Construyendo un analizador de direcciones de correo electrónico en Rust con nom](https://blog.arcjet.com/building-an-email-address-parser-in-rust-with-nom/)
* [Explorando los tiempos de ejecución asíncronos mediante la creación de los nuestros](https://blog.maguire.tech/posts/explorations/exploring-async-runtimes/)
* [Rasgos para unificar todos los vectores](https://orxfun.github.io/orxfun-notes/#/v-for-vectors-2024-11-18)
* [Conceptos básicos de fijación en Rust](https://garden.christophertee.dev/tech/rust/Pinning)
* [Construyendo un coche controlado por WiFi con Rust y ESP32](https://jamesmcm.github.io/blog/esp32-wifi-tank/)
* [video] [Construir con Naz: Diesel ORM, SQLite y Rust](https://www.youtube.com/watch?v=d9x_5X9R5LI)

## Crate de la semana

El crate de esta semana es [fixed-slice-vec](https://crates.io/crates/fixed-slice-vec), un Vec de longitud dinámica sin estándar con capacidad máxima determinada por el tiempo de ejecución respaldada por un segmento.

¡Gracias a [Jay Oster](https://users.rust-lang.org/t/crate-of-the-week/2704/1376) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamadas a la realización de pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de avanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*
  - [Pasos de prueba](https://github.com/rust-lang/cargo/issues/13873)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*
  - [Pasos de prueba]()

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron convocatorias para pruebas esta semana.*
  - [Pasos de prueba]()

Si es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP van aquí, use este formato: * [nombre del proyecto - título del problema](URL al problema) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para participar esta semana.* -->

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote en contacto con [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

480 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-12..2024-11-19

* [Comprobaciones ABI: añadir soporte para algunos arcos de nivel 3, advertir sobre otros](https://github.com/rust-lang/rust/pull/133029)
* [Comprobaciones de ABI: añadir soporte para arcos de nivel 2](https://github.com/rust-lang/rust/pull/132842)
* [CFI: anexar la ubicación de depuración a los bloques CFI](https://github.com/rust-lang/rust/pull/132702)
* [AIX: Añadir caja "desenredar" al enlace con libunwind](https://github.com/rust-lang/rust/pull/132905)
* [illumos: use pipe2 para crear tuberías anónimas](https://github.com/rust-lang/rust/pull/132984)
* ['check_consts': se corrige el error al solicitar la puerta de funciones cuando esa puerta no es realmente necesaria](https://github.com/rust-lang/rust/pull/132992)
* ['const_panic': en línea en las compilaciones de arranque para evitar bloqueos de f16/f128](https://github.com/rust-lang/rust/pull/133182)
* ['rustc_metadata': Rutas de búsqueda previas para un mejor rendimiento](https://github.com/rust-lang/rust/pull/132910)
* ['suggest_borrow_generic_arg': instanciar cláusulas correctamente](https://github.com/rust-lang/rust/pull/133130)
* [añadir 'visit_coroutine_kind' a 'ast::Visitor'](https://github.com/rust-lang/rust/pull/132956)
* [añadir paréntesis cuando se necesita la sugerencia de desempaquetar](https://github.com/rust-lang/rust/pull/132944)
* [agregar anotaciones de referencia para atributos de diagnóstico](https://github.com/rust-lang/rust/pull/133187)
* [permitir CFGuard en windows-gnullvm](https://github.com/rust-lang/rust/pull/132965)
* [siempre en línea: funciones que contienen 'f16' o 'f128'](https://github.com/rust-lang/rust/pull/133050)
* [Diagnóstico de Borrowck: sugerir tomar prestadas entradas de función en posiciones genéricas](https://github.com/rust-lang/rust/pull/132172)
* [cambie 'Visitor::visit_precise_capturing_arg' para que devuelva un 'Visitor::Result'](https://github.com/rust-lang/rust/pull/133049)
* [cambiar las declaraciones intrínsecas a un nuevo estilo](https://github.com/rust-lang/rust/pull/132907)
* [marque 'use<..>' en RPITIT para refinamiento](https://github.com/rust-lang/rust/pull/132795)
* [consolidar la evaluación del sistema de tipos const en 'rasgos::evaluate_const'](https://github.com/rust-lang/rust/pull/132927)
* [borrar el compilador en serie 'cfg(not(parallel))'](https://github.com/rust-lang/rust/pull/132282)
* [denegar la captura de parámetros ty/const enlazados en tiempo de ejecución en opacos anidados](https://github.com/rust-lang/rust/pull/132832)
* [diagnóstico para dejar mutar en el contexto del artículo](https://github.com/rust-lang/rust/pull/133143)
* [extender la lógica "if-unchanged" para compilaciones de compiladores](https://github.com/rust-lang/rust/pull/131831)
* [las expresiones de rendimiento de la puerta de características no están en 2024](https://github.com/rust-lang/rust/pull/132668)
* [arreglar ICE al pasar argumentos que crean DefId a 'legacy_const_generics'](https://github.com/rust-lang/rust/pull/130443)
* [Arreglar 'REGISTRY_USERNAME' para reutilizar la caché entre trabajos automáticos y de relaciones públicas](https://github.com/rust-lang/rust/pull/132967)
* [soluciona un problema de copiar y pegar en la definición de tipo sin procesar de NuttX](https://github.com/rust-lang/rust/pull/133027)
* [corregir error de compilación en Solaris debido al uso de flock](https://github.com/rust-lang/rust/pull/132977)
* [corrección de la edición de intervalo para RPIT 2024 proveniente de una macro externa](https://github.com/rust-lang/rust/pull/133080)
* [para expr 'return (_ = 42); unused_paren' lint no debe activarse](https://github.com/rust-lang/rust/pull/132936)
* [manejar inferir vars en anon consts en estable](https://github.com/rust-lang/rust/pull/132971)
* [mejorar VecCache en frontend paralelo](https://github.com/rust-lang/rust/pull/124780)
* [aumentar la precisión de la sugerencia de dispersión incorrecta de la condición 'si'](https://github.com/rust-lang/rust/pull/133051)
* [libera 'aarch64-gnu-debug' de los grilletes de '--test-args=clang'](https://github.com/rust-lang/rust/pull/132646)
* [probable solución poco probable](https://github.com/rust-lang/rust/pull/120370)
* [hacer que la máquina de sugerencias de captura precisa sea aplicable solo si no tiene APITs](https://github.com/rust-lang/rust/pull/132938)
* [asegúrese de ignorar las vidas eliminadas cuando apunte a args para errores de cumplimiento](https://github.com/rust-lang/rust/pull/132935)
* [Mencione tanto la versión * como la rotura de la edición para nunca escribir pelusas](https://github.com/rust-lang/rust/pull/132978)
* [mover todas las comprobaciones de tiempo único a su propia carpeta, y su propia consulta](https://github.com/rust-lang/rust/pull/132843)
* [Soporte adecuado para comprobaciones de estabilidad const recursivas entre cajas](https://github.com/rust-lang/rust/pull/132541)
* [querify MonoItem collection](https://github.com/rust-lang/rust/pull/132566)
* [se recursa en APITs en 'impl_trait_overcaptures'](https://github.com/rust-lang/rust/pull/132817)
* [refactorizar 'configure_annotatable'](https://github.com/rust-lang/rust/pull/133021)
* [eliminar atributos de genéricos en macros de derivación integradas](https://github.com/rust-lang/rust/pull/132651)
* [cambiar el nombre de 'rustc_const_stable_intrinsic' → 'rustc_intrinsic_const_stable_indirect'](https://github.com/rust-lang/rust/pull/133142)
* [omitir el intervalo de bloqueo para algunas comprobaciones de contexto de sintaxis](https://github.com/rust-lang/rust/pull/128197)
* [recorte el espacio adicional cuando sugiera eliminar los 'lets' incorrectos](https://github.com/rust-lang/rust/pull/132996)
* [recortar espacio en blanco en el intervalo primario de RemoveLet](https://github.com/rust-lang/rust/pull/133060)
* [Ajustar atributos para la macro de pánico constante](https://github.com/rust-lang/rust/pull/132662)
* [unificar FnKind entre los visitantes de AST y hacer que WalkItemKind sea más sencillo](https://github.com/rust-lang/rust/pull/132787)
* [use 'TypingMode' en todo el compilador en lugar de 'ParamEnv'](https://github.com/rust-lang/rust/pull/132460)
* [advertir sobre nombres de pase 'mir-enable-passes' no válidos](https://github.com/rust-lang/rust/pull/132901)
* [Miri: implementar el bloqueo eventfd](https://github.com/rust-lang/miri/pull/3939)
* [Miri: Refactor: Refinar variante de hilo para Windows](https://github.com/rust-lang/miri/pull/4035)
* [Miri: renombrado 'this' a 'ecx' en 'extern_static'](https://github.com/rust-lang/miri/pull/4030)
* [miri: use -Zroot-dir en lugar de --remap-path-prefix para el manejo de dir de diagnóstico](https://github.com/rust-lang/miri/pull/4039)
* [estabilizar 'const_atomic_from_ptr'](https://github.com/rust-lang/rust/pull/131717)
* [estabilizar 'const_option_ext'](https://github.com/rust-lang/rust/pull/132966)
* [estabilizar 'const_ptr_is_null'](https://github.com/rust-lang/rust/pull/133116)
* [estabilizar 'const_unicode_case_lookup'](https://github.com/rust-lang/rust/pull/132948)
* [vectorizar 'slice::is_sorted'](https://github.com/rust-lang/rust/pull/132883)
* ['#[inline]' funciones de análisis de enteros](https://github.com/rust-lang/rust/pull/132870)
* [agregar 'as_slice/into_slice' para IoSlice/IoSliceMut](https://github.com/rust-lang/rust/pull/132790)
* [generalizar 'NonNull::from_raw_parts' por ACP362](https://github.com/rust-lang/rust/pull/132895)
* [rwlock downgrade](https://github.com/rust-lang/rust/pull/128219)
* [implementar 'mixed_integer_ops_unsigned_sub'](https://github.com/rust-lang/rust/pull/126046)
* [mejorar el código de 'fmt_num' para eliminar el pánico inalcanzable](https://github.com/rust-lang/rust/pull/122770)
* [Tipos de float: mover copysign, abs, signum a libcore](https://github.com/rust-lang/rust/pull/131304)
* [hacer que 'CloneToUninit' sea compatible con dyn](https://github.com/rust-lang/rust/pull/133003)
* [marque 'is_val_statically_known' intrínseco como establemente invocable](https://github.com/rust-lang/rust/pull/132449)
* [optimizar 'char::to_digit' y afirmar que radix es al menos 2](https://github.com/rust-lang/rust/pull/132709)
* [hashbrown: secuestrar aún más el código 'Grupo'/'Etiqueta'](https://github.com/rust-lang/hashbrown/pull/568)
* [hashbrown: marque los constructores const fn como 'rustc_const_stable_indirect'](https://github.com/rust-lang/hashbrown/pull/586)
* [codegen\_gcc: arreglar cargas y almacenes volátiles](https://github.com/rust-lang/rustc_codegen_gcc/pull/572)
* [resolución de carga: Estabilizar resolución v3](https://github.com/rust-lang/cargo/pull/14754)
* [cargo rustdoc: diplay env vars en modo extra detallado](https://github.com/rust-lang/cargo/pull/14812)
* [Cargo fix: contexto de error para la especificación de referencia 'git_fetch' no encontrada](https://github.com/rust-lang/cargo/pull/14806)
* [cargo: incluir siempre Cargo.lock en las cajas publicadas](https://github.com/rust-lang/cargo/pull/14815)
* [cargo: migrate build-rs to the Cargo repo](https://github.com/rust-lang/cargo/pull/14786)
* [cargo: simplificar el inglés utilizado en la guía](https://github.com/rust-lang/cargo/pull/14825)
* [Rustdoc Search: Permitir que las consultas terminen en un segmento de ruta vacío](https://github.com/rust-lang/rust/pull/132569)
* [rustdoc-search: distingue entre mayúsculas y minúsculas solo cuando se usan mayúsculas](https://github.com/rust-lang/rust/pull/133043)
* [rustdoc-search: use smart binary search in bitmaps](https://github.com/rust-lang/rust/pull/133185)
* [rustdoc: tratar las macros declarativas más como otros tipos de elementos](https://github.com/rust-lang/rust/pull/132302)
* [rustdoc: use un trie para la búsqueda basada en nombres](https://github.com/rust-lang/rust/pull/133005)
* [rustdoc: Arreglar identificaciones de notas al pie duplicadas](https://github.com/rust-lang/rust/pull/133000)
* [rustdoc: Arreglado el manejo de la referencia de nota al pie en la definición de nota al pie](https://github.com/rust-lang/rust/pull/133040)
* [rustdoc: Arreglar elementos con genéricos que no tienen su enlace de salto a def generado](https://github.com/rust-lang/rust/pull/133180)
* [rustdoc: Realiza menos trabajo al limpiar los argumentos genéricos entre paréntesis 'middle::ty'](https://github.com/rust-lang/rust/pull/132886)
* [clippy: 'missing_safety_doc' acepta "SEGURIDAD" mayúscula](https://github.com/rust-lang/rust-clippy/pull/13701)
* [clippy: permitir futuros condicionales 'Enviar' en 'future_not_send'](https://github.com/rust-lang/rust-clippy/pull/13590)
* [clippy: no activar 'if_let_mutex' a partir de la Edición 2024](https://github.com/rust-lang/rust-clippy/pull/13695)
* [clippy: no pelar literales de CStr, hacer literales de flotación de lint en 'redundant_guards'](https://github.com/rust-lang/rust-clippy/pull/13698)
* [clippy: handle 'Option::map_or(true, ...)' en 'unnecessary_map_or' lint](https://github.com/rust-lang/rust-clippy/pull/13653)
* [clippy: nueva pelusa: 'unnecessary_map_or'](https://github.com/rust-lang/rust-clippy/pull/11796)
* [clippy: soporta macros similares al formato de usuario](https://github.com/rust-lang/rust-clippy/pull/9948)
* [rust-analyzer: migrar la asistencia 'reorder_fields' para usar 'SyntaxFactory'](https://github.com/rust-lang/rust-analyzer/pull/18495)

### Clasificación del rendimiento del compilador de Rust

Vimos mejoras en una gran variedad de puntos de referencia con la consulta de
Colección MonoItem (PR #132566). También hubo algunas relaciones públicas en las que estamos dispuestos a
para pagar un costo en tiempo de compilación para el beneficio de tiempo de ejecución esperado (PR #132870, PR #120370),
o pagar un pequeño costo en la caja de un solo hilo a cambio de un gran paralelo
(PR #124780).

Triaje realizado por **@pnkfelix**.
Rango de revisión: [d4822c2d.. 7D40450B](https://perf.rust-lang.org/?start=d4822c2d84c242cc7403118b50c571464f38ef8f&end=7d40450b2df92bdc9dec414b30cf5f7a5979a92e&absolute=false&stat=instructions%3Au)

2 regresiones, 4 mejoras, 10 mixtas; 6 de ellos en rollups
47 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/d1b574c0c528c74491412625aa5bd3f27a9c2268/triage/2024-11-19.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [[RFC] Gancho de generación de subprocesos (heredando locales de subprocesos)](https://github.com/rust-lang/rfcs/pull/3642)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No se aprobaron RFC esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Mostrar siempre la primera línea de los bloques impl incluso cuando están colapsados](https://github.com/rust-lang/rust/pull/132155)
* [disposición: fusionar] [Estabilizar cierres asíncronos (RFC 3668)](https://github.com/rust-lang/rust/pull/132706)
* [disposición: fusionar] [Problema de seguimiento para fn const BuildHasherDefault::new()](https://github.com/rust-lang/rust/issues/123197)
* [disposición: fusionar] [Añadir 'AsyncFn*' al preludio en todas las ediciones](https://github.com/rust-lang/rust/pull/132611)
* [disposición: fusionar] [Problema de seguimiento para #![ característica(const_float_methods)]](https://github.com/rust-lang/rust/issues/130843)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Agregar advertencia de incompatibilidad futura contra palabras clave en cfgs y agregar identificadores sin procesar](https://github.com/rust-lang/cargo/pull/14671)

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* [disposición: fusionar] [Comprobación de consenso: las cadenas de permisos y es no son mutuamente excluyentes](https://github.com/rust-lang/lang-team/issues/297)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referencia de idioma ingresó al Período Final de Comentarios esta semana.*

##### [Directrices de códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas de seguimiento de pautas de código inseguro o PR ingresaron al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [Jerarquía de rasgos de tamaño](https://github.com/rust-lang/rfcs/pull/3729)

## Próximos eventos

Eventos oxidados entre 2024-11-20 - 2024-12-18 🦀

### Virtual
* 20/11/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Rust para el Club de Lectura de los Rustáceos: Capítulo 12: Rust sin la Biblioteca Estándar**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/304441931/)
* 20/11/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Taller de Rust incrustado**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 21/11/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633273/)
* 21/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**IoT confiable con Rust... ¡y contraseñas!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 21/11/2024 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos de Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 25/11/2024 | Virtual (Bratislava, SK) | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Charla ONLINE, patrocinada por Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 26/11/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Martes pasado**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 28/11/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 28/11/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 03/12/2024 | Virtual (Buffalo, NY, EE. UU.) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 04/12/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031652)
* 05/12/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633275/)
* 07/12/2024 | Virtual (Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-12-10 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346988/)
* 11/12/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Pasar el rato**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 12/12/2024 | Virtual (Charlottesville, Carolina del Norte, Estados Unidos) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 12/12/2024 | Virtual (Nürnberg, DE) | [Rust de Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 17/12/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346972/)

### África
* 2024-12-10 | Johannesburgo, ZA | [Reunión de Rust en Johannesburgo](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hola Mundo... otra vez**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)
* 07/12/2024 | Virtual( Kampala, UG) | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 21/11/2024 | Seúl, KR | [Encuentro de programación de Rust en Seúl](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Encuentro de Rust en Seúl**](https://www.meetup.com/rust-seoul-meetup/events/304590280/)
* 28/11/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Cumbre RustTechX 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 30/11/2024 | Tokio, JP | [Rust de Tokio](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europa
* 20/11/2024 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #72**](https://www.meetup.com/rust-paris/events/304396616/)
* 21/11/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #53 patrocinado por Microsoft**](https://www.meetup.com/copenhagen-rust-community/events/304608747/)
* 21/11/2024 | Edimburgo, Reino Unido | [Rust y sus amigos](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 21/11/2024 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**Taller de introducción a unit testing en Rust**](https://www.meetup.com/madrust/events/304484962/)
* 21/11/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 23/11/2024 | Basilea, CH | [Basilea Rust](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Taller #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 25/11/2024 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Rust Meetup 2024/11: Panel diskusija - Usvajanje Rusta i iskustva iz industrije**](https://www.meetup.com/zagreb-rust-meetup/events/304576915/)
* 26/11/2024 | Varsovia, PL | [Rust Varsovia](https://www.meetup.com/rust-warsaw/events/)
    * [**New Rust Warsaw Meetup #3**](https://www.meetup.com/rust-warsaw/events/304379707/)
* 27/11/2024 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 28/11/2024 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charla en Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 28/11/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 28/11/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421381/)
* 28/11/2024 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 28/11/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn con Mainmatter y Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 28/11/2024 | Manchester, Reino Unido | [Rust de Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/304556866/)
* 28/11/2024 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Praga (noviembre de 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 03/12/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advenimiento del Código**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 04/12/2024 | Oxford, Reino Unido | [Grupo de Meetup de Oxford Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 05/12/2024 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Encuentro de Rust Moravia (diciembre de 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 06/12/2024 | Moscú, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Rusia**](https://rustcon.ru/)
* 11/12/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 12/12/2024 | Ámsterdam, Países Bajos | [Grupo de desarrolladores de Rust en Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 17/12/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tipos, rasgos y mejores prácticas**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### América del Norte
* 21/11/2024 | Chicago, Illinois, Estados Unidos | [Encuentro de Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/304568425/)
* 23/11/2024 | Boston, MA, EE. UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust común de Boston, 23 de noviembre**](https://www.meetup.com/bostonrust/events/303708407/)
* 25/11/2024 | Ferndale, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 26/11/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530470/)
* 27/11/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 28/11/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/304468157/)
* 05/12/2024 | San Luis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cuerdas de Rust**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, Michigan, Estados Unidos | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 12/12/2024 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 16/12/2024 | Minneapolis, MN, Estados Unidos | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 17/12/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)

### Oceanía
* 04/12/2024 | Sídney, Australia | [Rust de Sídney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 08/12/2024 | Canberra, Australia | [Grupo de usuarios de Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Fiesta de Navidad CRUG**](https://www.meetup.com/rust-canberra/events/304282046/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1gf5ue1/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> El punto de Rust es que antes había dos mundos:
>
> * Idiomas ineficientes, recolectados y confiables
> * Idiomas eficientes, asignados manualmente y peligrosos
>
> Y la marca de ser un buen desarrollador en el primero fue mitigar bien la ineficiencia, y para el segundo fue que no se bloqueó, corrompió la memoria ni estuvo plagado de problemas de seguridad. En cambio, Rust hace la compensación de que ser bueno significa entender cómo evitar que el compilador te grite.

– [Simon Buchan sobre los usuarios de Rust]()

¡Gracias a [binarycat](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1632) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1gw6ztc/this_week_in_rust_574/)</small>
