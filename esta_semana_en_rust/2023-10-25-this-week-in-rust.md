---
title: "Esta semana en Rust #3"
number_of_week: 3
description: Esta semana en Rust es un blog semanal sobre el lenguaje de programación Rust, sus comunidades y su ecosistema.
date: 2023-10-25
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

## Actualizaciones de proyectos/herramientas

* [Caja estroboscópica](https://jlogan03.github.io/jekyll/update/2023/10/21/strobe.html)
* [Las dependencias del sistema son difíciles (así que las hicimos más fáciles)](https://blog.axo.dev/2023/10/dependencies)

### Observaciones/Pensamientos

* [Tratando de inventar un mejor algoritmo de búsqueda de subcadenas](https://marcos.unsafe.rs/trying-para-inventar-un-mejor-algoritmo de búsqueda de subcadenas/)
* [Mejorando Node.js con la biblioteca Rust-Wasm](https://elvisbrevi.hashnode.dev/improving-nodejs-with-rust-wasm-library)
* [Mezcla de C# y Rust - Interoperabilidad](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_0_3_2.html)
* [Una nueva mirada a la serialización incremental de cero copias](https://udoprog.github.io/rust/2023-10-19/musli-zerocopy.html)
* [Haz que el compilador de Rust sea un 5% más rápido con este extraño truco](https://kobzol.github.io/rust/rustc/2023/10/21/make-rust-compiler-5percent-faster.html)
* [Parte 3: Remo de botes de tipo de datos a flote](https://thunderseethe.dev/posts/row-types/)
* [Recreación de combinadores de futuros concurrentes en smol](https://notgull.net/futures-concurrencia-en-smol/)
* [Desempaquetando algunas ergonomías de Rust: obteniendo un solo resultado de un iterador de ellas](https://ntietz.com/blog/rust-vec-of-result/)
* [Idea: "Usando Rust", un documento vivo](https://smallcultfollowing.com/babysteps/blog/2023/10/20/using-rust/)
* [La sopa de objetos está hecha de índices](https://jacko.io/object_soup.html)
* [Análisis de datos 180.000 veces más rápido con Rust](https://willcrichton.net/notes/k-corrset/)
* [Issue #10: Serving HTML](https://www.shuttle.rs/launchpad/issues/2023-10-17-issue-10-Serving-HTML)
* [Rust vs C en un ATTiny85; una historia de guerra incrustada](https://diziet.dreamwidth.org/16771.html)

### Tutoriales de Rust

* [Análisis de datos /,000 veces más rápido con Rust](https://willcrichton.net/notes/k-corrset/)
* [Lanzamientos totalmente automatizados para proyectos de Rust](https://blog.orhun.dev/automated-rust-releases/)
* [Haga que su unidad de código de Rust sea comprobable con inversión de dependencias](https://worldwithouteng.com/articles/make-your-code-unit-testable-with-dependency-inversion/)
* [Nueve reglas para validar formalmente los algoritmos de Rust con Dafny (Parte 2): Lecciones de la verificación de la caja range-set-fire](https://medium.com/towards-data-science/nine-rules-to-formally-validate-rust-algorithms-with-dafny-part-2-f2a279686700)
* [video] [Vamos a escribir un corredor de mensajes usando QUIC - Broke But Quick Episode 1](https://www.youtube.com/watch?v=lpsduJy2EIM)
* [video] [Publicación de mensajes a través de QUIC Streams! - Broke But Quick episodio 2](https://www.youtube.com/watch?v=auXpVgUMZjU)

### Miscelánea

* [video] [Tipos asociados en los límites del iterador](https://youtu.be/Sa5bYF5a-Ng)
* [video] [Rust y la era de las lenguas de alta integridad](https://www.youtube.com/watch?v=pJoATjuc50w)
* [video] [Implementando (parte de) un cliente BitTorrent en Rust](https://www.youtube.com/watch?v=jf_ddGnum_4)

## Crate de la semana

El crate de esta semana es [cargo-show-asm](https://lib.rs/crates/cargo-show-asm), un subcomando de carga para mostrar el ensamblaje optimizado de cualquier función.

¡Gracias a [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1250) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

<!-- los CFP vayan aquí, use este formato: * [nombre del proyecto - título del problema](enlace al problema) -->
<!-- * [ - ]() -->
* [Hyperswitch (Hacktoberfest)- [CARACTERÍSTICA] separar payments_session del núcleo de pagos](https://github.com/juspay/hyperswitch/issues/888)
* [Hyperswitch (Hacktoberfest)- [NMI] Usar connector_response_reference_id como referencia al comerciante](https://github.com/juspay/hyperswitch/issues/2338)
* [Hyperswitch (Hacktoberfest)- [Airwallex] Usar connector_response_reference_id como referencia al comerciante](https://github.com/juspay/hyperswitch/issues/2322)
* [Hyperswitch (Hacktoberfest)- [Worldline] Usar connector_response_reference_id como referencia al comerciante](https://github.com/juspay/hyperswitch/issues/2351)
* [Ockam - Hacer que 'ockam project delete' (sin argumentos) sea interactivo pidiendo al usuario que elija de una lista de nombres de espacios y proyectos para eliminar (tuify)](https://github.com/build-trust/ockam/issues/6461)
* [Ockam - Validar estructuras CBOR de acuerdo con el esquema cddl para 'authenticator/direct/types'](https://github.com/build-trust/ockam/issues/6682)
* [Ockam - Adelgazar el 'NodeManagerWorker' para 'nodo / estado del nodo'](https://github.com/build-trust/ockam/issues/6707)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

397 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-16..2023-10-23

* [Reescribir el registro de la impresora bonita de GDB](https://github.com/rust-lang/rust/pull/116978)
* [añadir anotaciones de FileCheck a las pruebas mir-opt](https://github.com/rust-lang/rust/pull/116810)
* [añadir MonoItems e Instancia a 'stable_mir'](https://github.com/rust-lang/rust/pull/116719)
* [añadir un objetivo 'csky-unknown-linux-gnuabiv2hf'](https://github.com/rust-lang/rust/pull/117049)
* [Agregue una prueba que muestre la inferencia de firma de cierre fallida en el nuevo solucionador](https://github.com/rust-lang/rust/pull/116899)
* [añadir una nueva sintaxis más simple y explícita para check-cfg](https://github.com/rust-lang/rust/pull/111072)
* [añadir estable 'Instance::body()' y RustcInternal trait](https://github.com/rust-lang/rust/pull/116964)
* [Habilitar automáticamente la inserción de cajas cruzadas para funciones pequeñas](https://github.com/rust-lang/rust/pull/116505)
* [evite un 'track_errors' burbujeando la mayoría de los errores de 'check_well_formed'](https://github.com/rust-lang/rust/pull/116849)
* [evite que 'rustc_smir' dependa de 'rustc_interface' o 'rustc_driver'](https://github.com/rust-lang/rust/pull/116837)
* [Cobertura: Emitir asignaciones para funciones no utilizadas sin generar stubs](https://github.com/rust-lang/rust/pull/116922)
* [Cobertura: Emitir la sección de nombres de archivo antes de codificar las asignaciones por función](https://github.com/rust-lang/rust/pull/117042)
* [Cobertura: Corregir el manejo inconsistente de los intervalos de firma de la función](https://github.com/rust-lang/rust/pull/116974)
* [cobertura: mover la mayor parte de la información de cobertura por función a 'mir::Body'](https://github.com/rust-lang/rust/pull/116046)
* [Cobertura: simplificar la inyección de declaraciones de cobertura](https://github.com/rust-lang/rust/pull/116917)
* [deshabilitar la pelusa 'missing_copy_implementations' en los tipos 'non_exhaustive'](https://github.com/rust-lang/rust/pull/116812)
* [no poner en negrita el mensaje principal en '--error-format=short'](https://github.com/rust-lang/rust/pull/116962)
* [no haga hielo cuando encuentre regiones no resueltas en 'fully_resolve'](https://github.com/rust-lang/rust/pull/116663)
* [no comparar el parámetro de host por nombre](https://github.com/rust-lang/rust/pull/116870)
* [no se bloquee en una coincidencia vacía en la pelusa 'nonexhaustive_omitted_patterns'](https://github.com/rust-lang/rust/pull/117034)
* [duplicar los límites '~const' con uno que no sea const en el desazúcar de efectos](https://github.com/rust-lang/rust/pull/116756)
* [eliminar 'rustc_attrs::builtin::handle_errors' a favor de emitir errores directamente](https://github.com/rust-lang/rust/pull/117064)
* [Se corrigió una regresión de rendimiento en la deduplicación de obligaciones](https://github.com/rust-lang/rust/pull/116826)
* [corregir la comprobación de vidas superactivas implícitas para GAT en RPITIT](https://github.com/rust-lang/rust/pull/116800)
* [Se corrigieron los intervalos para eliminar '.await' en las expresiones 'for'](https://github.com/rust-lang/rust/pull/117019)
* [Sugerencia de corrección para la función de corrutinas renombrada](https://github.com/rust-lang/rust/pull/117073)
* [implementar una pelusa interna que fomente el uso de 'Span::eq_ctxt'](https://github.com/rust-lang/rust/pull/116787)
* [implementar roscado de salto MIR opt](https://github.com/rust-lang/rust/pull/107009)
* [implementar la parte rustc de las rutas de recorte RFC 3127](https://github.com/rust-lang/rust/pull/115214)
* [Mejorar la visualización de trabajos paralelos en el script de prueba rustdoc-gui](https://github.com/rust-lang/rust/pull/116798)
* [iniciar el uso interno de 'cfg_match' (Compilador)](https://github.com/rust-lang/rust/pull/116312)
* [lint 'non_exhaustive_omitted_patterns' por columnas](https://github.com/rust-lang/rust/pull/116734)
* [polonio insensible a la ubicación: considere que un préstamo se escapa si una SCC solo tiene restricciones de miembros aplicadas](https://github.com/rust-lang/rust/pull/116960)
* [hacer que '#[repr(Rust)]' sea incompatible con otras sugerencias de representación (no modificadoras) como 'C' y 'simd'](https://github.com/rust-lang/rust/pull/116829)
* [hacer que la ruta de exportación 'rustc_onunimplemented' sea agnóstica](https://github.com/rust-lang/rust/pull/116805)
* [mencione 'into_iter' en las sugerencias de errores de préstamo cuando corresponda](https://github.com/rust-lang/rust/pull/116990)
* [mencione la sintaxis de 'use' en 'mod foo;' si 'foo' no existe](https://github.com/rust-lang/rust/pull/116992)
* [pánico cuando el asignador global intenta registrar un destructor TLS](https://github.com/rust-lang/rust/pull/116402)
* [punto en la definición de assoc fn sobre la divergencia de parámetros de tipo](https://github.com/rust-lang/rust/pull/116995)
* [conserva los escapes Unicode en literales de cadena de formato cuando se imprime AST](https://github.com/rust-lang/rust/pull/116811)
* [Tener en cuenta adecuadamente la autoambiguación en la sugerencia de desambiguación de métodos](https://github.com/rust-lang/rust/pull/116713)
* [informe 'unused_import' para reexportaciones vacías incluso si es pub](https://github.com/rust-lang/rust/pull/116033)
* [Caso especial de la cadena de iteradores comprueba si hay sugerencias](https://github.com/rust-lang/rust/pull/116717)
* [Procedencia estricta desenrollar](https://github.com/rust-lang/rust/pull/114534)
* [sugerir ';' después de la expresión 'match' E0308](https://github.com/rust-lang/rust/pull/106601)
* [sugerir restringir los tipos de asociación en más casos](https://github.com/rust-lang/rust/pull/116865)
* [sugerir relajar el implícito 'type Assoc: Sized;' bound](https://github.com/rust-lang/rust/pull/116911)
* [sugiero eliminar los argumentos redundantes en 'format! ()»(https://github.com/rust-lang/rust/pull/115324)
* [Eleva la movilidad y la mutabilidad, de la manera más sencilla](https://github.com/rust-lang/rust/pull/116946)
* [miri: evite un escaneo lineal sobre todo el 'int_to_ptr_map' en cada deallocation](https://github.com/rust-lang/miri/pull/3134)
* [miri: se corrigió la verificación del modo de redondeo en las funciones de ronda SSE4.1](https://github.com/rust-lang / miri / pull / 3124)
* [Miri: Intptrcast: Eliminar información sobre asignaciones muertas](https://github.com/rust-lang/miri/pull/3122)
* [deshabilitar efectos en libcore de nuevo](https://github.com/rust-lang/rust/pull/116856)
* [añadir '#[track_caller]' a 'Opción::unwrap_or_else'](https://github.com/rust-lang/rust/pull/116795)
* [especialice 'Bytes:<R>:next' cuando 'R' es un 'BufReader'](https://github.com/rust-lang/rust/pull/116785)
* [hacer que TCP conecte la manija EINTR correctamente](https://github.com/rust-lang/rust/pull/116132)
* [en Windows hacer un error 'read_dir' en la ruta vacía](https://github.com/rust-lang/rust/pull/116606)
* [hashbrown: agregar API 'HashTable' de bajo nivel](https://github.com/rust-lang/hashbrown/pull/466)
* [codegen\_gcc: agrega soporte para el atributo de función NonNull](https://github.com/rust-lang/rustc_codegen_gcc/pull/326)
* [codegen\_gcc: corrige el atributo '#[inline(always)]' y admite la comparación sin signo para enteros con signo](https://github.com/rust-lang/rustc_codegen_gcc/pull/352)
* [codegen\_gcc: arreglar endianness](https://github.com/rust-lang/rustc_codegen_gcc/pull/346)
* [codegen\_gcc: arreglar la alineación de los tipos int](https://github.com/rust-lang/rustc_codegen_gcc/pull/353)
* [codegen\_gcc: optimizar la implementación de popcount](https://github.com/rust-lang/rustc_codegen_gcc/pull/348)
* [CodeGen\_gcc: Optimizar aún más los recuentos de pop de U128/i128](https://github.com/rust-lang/rustc_codegen_gcc/pull/354)
* [cargo add: Conservar más comentarios](https://github.com/rust-lang/cargo/pull/12838)
* [cargo remove: Conservar comentarios de características](https://github.com/rust-lang/cargo/pull/12837)
* [reemplazo de carga: soporte de especificaciones de versión parcial](https://github.com/rust-lang/cargo/pull/12806)
* [cargo: Proporcione los siguientes pasos para la bandera -Z incorrecta](https://github.com/rust-lang / cargo / pull / 12857)
* [cargo: Sugerir búsqueda de carga en comandos incorrectos](https://github.com/rust-lang/cargo/pull/12840)
* [cargo: ajuste '-Zcheck-cfg' para la nueva sintaxis y comportamiento de rustc](https://github.com/rust-lang/cargo/pull/12845)
* [cargo: si hay una versión en el archivo de bloqueo, use solo esa versión exacta](https://github.com/rust-lang/cargo/pull/12772)
* [cargo: hacer que el campo preciso de una fuente sea una enumeración](https://github.com/rust-lang/cargo/pull/12849)
* [cargo: imprime variables de entorno para ejecuciones de scripts de compilación con '-vv'](https://github.com/rust-lang/cargo/pull/12829)
* [cargo: advertir sobre el formato del nombre de la caja al crear una nueva caja](https://github.com/rust-lang/cargo/pull/12766)
* [rustdoc: alinear la insignia de estabilidad con la línea de base en lugar de con la parte inferior](https://github.com/rust-lang/rust/pull/105666)
* [rustdoc: evitar la asignación de cadenas de impresión de enlaces primitivos](https://github.com/rust-lang/rust/pull/117007)
* [clippy: 'map_identity': permitir el cierre con anotaciones de tipo](https://github.com/rust-lang/rust-clippy/pull/11521)
* [clippy: 'map_identity': reconocer la función de identidad de la tupla](https://github.com/rust-lang/rust-clippy/pull/10943)
* [clippy: agregar lint para los nombres de los campos 'struct'](https://github.com/rust-lang/rust-clippy/pull/11496)
* [clippy: no emita 'needless_pass_by_ref_mut' si la variable se usa en un bloque o función insegura](https://github.com/rust-lang/rust-clippy/pull/11624)
* [clippy: hacer que 'multiple_unsafe_ops_per_block' ignore await desugaring](https://github.com/rust-lang/rust-clippy/pull/11646)
* [clippy: paso innecesario por ref mut closure non async fn](https://github.com/rust-lang/rust-clippy/pull/11621)
* [clippy: ahora 'declare_interior_mutable_const' y 'borrow_interior_mutable_const' respetan la entrada de configuración 'ignore-interior-mutability'](https://github.com/rust-lang/rust-clippy/pull/11678)
* [clippy: omite la pelusa 'if_not_else' para las comprobaciones de estilo '!= 0'](https://github.com/rust-lang/rust-clippy/pull/11028)
* [clippy: sugiere pasar la función en lugar de llamarla en el cierre de 'option_if_let_else'](https://github.com/rust-lang/rust-clippy/pull/11460)
* [clippy: advertir 'missing_enforced_import_renames' por defecto](https://github.com/rust-lang/rust-clippy/pull/11539)
* [rust-analyzer: generar descriptores para todas las características inestables](https://github.com/rust-lang/rust-analyzer/pull/15727)
* [rust-analyzer: agregue un comando solo para abrir documentos externos e intentar solucionar el problema de vscode-remote](https://github.com/rust-lang/rust-analyzer/pull/15779)
* [rust-analyzer: agregar diagnósticos de casos incorrectos para los nombres de los módulos](https://github.com/rust-lang/rust-analyzer/pull/15736)
* [rust-analyzer: corregir la detección de VS Code para la versión de Insiders](https://github.com/rust-lang/rust-analyzer/pull/15786)
* [rust-analyzer: importar rasgo si es necesario para la asistencia 'unqualify_method_call'](https://github.com/rust-lang/rust-analyzer/pull/15780)
* [rust-analyzer: elija un nombre mejor para las variables introducidas por 'replace_is_some_with_if_let_some'](https://github.com/rust-lang/rust-analyzer/pull/15775)
* [rust-analyzer: almacena el modo de enlace para cada instancia de un enlace de forma independiente](https://github.com/rust-lang/rust-analyzer/pull/15789)
* [perf: agregar punto de referencia de tiempo de ejecución de emulación de NES](https://github.com/rust-lang/rustc-perf/pull/1730)

### Clasificación del rendimiento del compilador de Rust

<!-- resultados de Perf van aquí -->

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

* [Agregar tipos de flotación f16 y f128](https://github.com/rust-lang/rfcs/pull/3453)
* [Unicode y códigos de escape en literales](https://github.com/rust-lang/rfcs/pull/3349)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Considere los límites de alias al calcular la vida en NLL (pero esta vez suena con suerte)](https://github.com/rust-lang/rust/pull/116733)
* [disposición: cerrar] [regresión: el tipo de parámetro puede no vivir lo suficiente](https://github.com/rust-lang/rust/issues/117055)
* [disposición: fusionar] [Eliminar el soporte para complementos del compilador.](https://github.com/rust-lang/rust/pull/116412)
* [disposición: fusionar] [rustdoc: Documentar la falta de seguridad de los objetos en los rasgos afectados](https://github.com/rust-lang/rust/pull/113241)
* [disposición: fusionar] [Estabilizar características de destino RISC-V ratificadas](https://github.com/rust-lang/rust/pull/116485)
* [disposición: fusionar] [Problema de seguimiento para const mem::d iscriminant](https://github.com/rust-lang/rust/issues/69821)

### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [eRFC: #[should_move] atributo para la exclusión voluntaria por función de la semántica de copia](https://github.com/rust-lang/rfcs/pull/3518)

### [Llamada para pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-10-25 - 2023-11-22 🦀

### Virtual

* 30/10/2023 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Híbrido - en línea y en persona) Reunión de Rust Melbourne de octubre de 2023**](https://www.meetup.com/rust-melbourne/events/296902361/)
* 31/10/2023 | Virtual (Europa / África) | [Rust para el almuerzo](https://lunch.rs/)
    * [**Rust Meet-up**](https://lunch.rs/meetups/2023-10-31/)
* 01/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS con Bevy Game Engine**](https://www.meetup.com/rust-y-c-plus-plus-in-cardiff/events/296583207/)
* 01/11/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 02/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 07/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679790/) | [**Espejo**](https://berline.rs/)
* 07/11/2023 | Virtual (Búfalo, NY, EE. UU.) | [Reunión de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust, Primeros martes**](https://www.meetup.com/buffalo-rust-meetup/events/296827010/)
* 09/11/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 14/11/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/fvdtgtyfcpbsb/)
* 15/11/2023 | Virtual (Cardiff, Reino Unido)| [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Construyendo Nuestras Propias Cerraduras (Atómicas y Cerraduras Capítulo 9)**](https://www.meetup.com/rust-y-c-plus-plus-en-cardiff/events/296582223/)
* 15/11/2023 | Virtual (Richmond, VA, EE. UU.) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 15/11/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Estudio de Rust/Hackeo/Hang-out**](https://www.meetup.com/vancouver-rust/eventos/296600976/)
* 16/11/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 07/11/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/) | [**Espejo**](https://berline.rs/)
* 21/11/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/296807537/)

### Europa

* 25/10/2023 | Dublín, IE | [Rust Dublín](https://www.meetup.com/rust-dublin/)
    * [**Biome, herramientas de desarrollo web con Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 25/10/2023 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [Rust para la web - Encuentro en París #61](https://mobilizon.fr/events/149c0367-66cb-42c6-aa0c-8495bf6d2a52)
* 25/10/2023 | Zagreb, RRHH | [impl Zagreb para Rust](https://www.meetup.com/zagreb-rust-meetup)
    * [Rust Meetup 2023/10: Lunático](https://www.meetup.com/zagreb-rust-meetup/events/296765355/)
* 26/10/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)
* 26/10/2023 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #41 patrocinado por Factbird**](https://www.meetup.com/copenhagen-rust-community/events/296819462/)
* 26/10/2023 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust en la Universidad Técnica de Delft**](https://www.meetup.com/rust-nederland/events/296488286/)
* 26/10/2023 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #4 en SFEIR**](https://www.meetup.com/meetup-group-zgphbyet/events/296766699/)
* 30/10/2022 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Aira + Netlight**](https://www.meetup.com/Stockholm-Rust/events/296578336/)
* 01/11/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Aplicaciones web con axum: ¡Hola CRUD!**](https://www.meetup.com/rustcologne/events/296540949/)
* 07/11/2023 | Bratislava, SK | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/296809100/)
* 07/11/2023 | Bruselas, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Edición para principiantes de Rust and Talk**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 07/11/2023 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #7**](https://www.meetup.com/rust-lyon/events/296853019/)
* 09/11/2023 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11ª reunión de BcnRust**](https://www.meetup.com/bcnrust/events/296567395)
* 09/11/2023 | Reading, Reino Unido | [Taller de lectura de Rust](https://www.meetup.com/reading-rust-workshop/)
    * [**Encuentro de lectura de Rust en Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)
* 21/11/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Procesamiento de GPU en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 23/11/2023 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)

### América del Norte

* 25/10/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)
* 25/10/2023 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Hora feliz de Rust**](https://www.meetup.com/deep-dish-rust/events/296657993/)
* 01/11/2023 | Brookline, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de la roya común de Boston**](https://www.meetup.com/bostonrust/events/296223910/)
* 08/11/2023 | Boulder, CO, EE. UU. | [Reunión de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**¡Hagamos un bot de Discord!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 14/11/2023 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀 **](https://www.meetup.com/rust-nyc/events/296895126/)
* 14/11/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/seattle-rust-user-group/events/296540653)
* 15/11/2023 | Richmond, VA, EE. UU. + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2023 (13-16 de noviembre)**](https://lpc.events/event/17/sessions/170/)
* 16/11/2023 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**¡A Python le encanta Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 16/11/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 21/11/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 22/11/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)

### Oceanía

* 26/10/2023 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**Reunión de octubre**](https://www.meetup.com/rust-brisbane/events/296628243/)
* 30/10/2023 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Híbrido - en persona y en línea) Octubre 2023 Rust Melbourne Meetup **](https://www.meetup.com/rust-melbourne/events/296902362/)
* 21/11/2023 | Christchurch, Nueva Zelanda | [Grupo de encuentro de Christchurch Rust](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)

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
> A eso lo llamamos syn tax :ferris:

– [Janet en Fosstodon](https://fosstodon.org/@janet/111223564960983226)

¡Gracias a [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1472) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/17gndm2/this_week_in_rust_518/)</small>

