---
title: "Esta semana en Rust #21"
number_of_week: 21
description: El crate de esta semana es states-scope-guard, una biblioteca que admite un patrón RAII más flexible para la gestión de recursos declarados.
date: 2024-05-15
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
* [Actualización del Consejo de Liderazgo de mayo de 2024](https://blog.rust-lang.org/inside-rust/2024/05/14/leadership-council-update.html)

### Boletines informativos
* [ClearCoat, nuevos ejemplos y actualizaciones del juego](https://thisweekinbevy.com/issue/2024-05-13-clearcoat-new-examples-and-game-updates)

### Actualizaciones de proyectos/herramientas
* [Rust para kernels de Linux integrados](https://lwn.net/Articles/970216/)
* [Kira - Release v0.9.0](https://github.com/tesselode/kira/releases/tag/v0.9.0)
* [Cushy v0.3: Nuevos widgets, captura fuera de pantalla, integraciones de Plotters y Tokio, y más](https://ecton.dev/cushy-v0-3/)
* [bbolt-rs v1.3.8](https://github.com/ambaxter/bbolt-rs/blob/v1.3.8/docs/announcement.md)
* [Maelstrom: Un ejecutor de pruebas hermético y agrupado para Rust (y es rápido)](https://www.reddit.com/r/rust/comments/1chrshl/maelstrom_a_hermetic_clustered_test_runner_for/)
* [Publicada la versión r3bl_cmdr v0.0.12](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v0012-2024-05-12)
* [Iroh 0.16 - Un mejor 'cliente'](https://iroh.computer/blog/iroh-0-16-a-better-client)

### Observaciones/Pensamientos
* [Mantenimiento del proyecto Rust a largo plazo](https://corrode.dev/blog/long-term-rust-maintenance/)
* [Los métodos deben ser seguros para los objetos](https://nora.codes/post/methods-should-be-object-safe/)
* [Las referencias son como saltos](https://without.boats/blog/references-are-like-jumps/)
* [Rust 1.78: Impacto en el rendimiento de la corrección de alineación de memoria de 128 bits](https://codspeed.io/blog/rust-1-78-performance-impact-of-the-128-bit-memory-alignment-fix)
* [Cómo: Egui con webworkers](https://voelklmichael.github.io/Blog/2024/05/12/egui-wasm-threads.html)
* [Uso de build.rs para integrar aplicaciones de Rust con bibliotecas del sistema como un profesional](https://neosmart.net/blog/using-build-rs-to-integrate-rust-applications-with-system-libraries-like-a-pro/)
* [Actores de Rust + ArcMutex: manéjelo con cuidado](https://dgroshev.com/blog/rust-actors-mutex/)
* [Rust a través de los siglos](https://www.ncameron.org/blog/rust-through-the-ages/)
* [Mezcla de rayón y tokio por diversión y pérdida de cabello](https://blog.dureuill.net/articles/dont-mix-rayon-tokio/)
* [Tareas asíncronas de backend de larga duración en tauri v2](https://sneakycrow.dev/blog/2024-05-12-running-async-tasks-in-tauri-v2)
* [Listas enlazadas increíblemente rápidas](https://dygalo.dev/blog/blazingly-fast-linked-lists/)
* [Tipos existenciales en Rust](https://lwn.net/Articles/970186/)
* [Manejo de errores para grandes proyectos de Rust - Una inmersión profunda en las prácticas de GreptimeDB](https://greptime.com/blogs/2024-05-07-error-rust)

### Tutoriales de Rust
* [Construyamos un Balanceador de Carga en Rust - Parte 1](https://marcobacis.com/blog/load-balancer-rust-1/)
* [video] [Build with Naz : tokio tracing & OTel and how to use it in Rust](https://www.youtube.com/watch?v=Wf8JrLgBuKI)

### Miscelánea
* [Informe de empleos de Rust de abril de 2024](https://filtra.io/rust-apr-24)
* [Extensiones de VS Code y WebAssembly](https://code.visualstudio.com/blogs/2024/05/08/wasm)

## Crate de la semana

El crate de esta semana es [states-scope-guard](https://crates.io/crates/stated-scope-guard), una biblioteca que admite un patrón RAII más flexible para la gestión de recursos declarados.

¡Gracias a [Evian Zhang](https://users.rust-lang.org/t/crate-of-the-week/2704/1309) por la autosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria de pruebas
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

* [greptimedb - Añadir más pruebas para Copiar desde](https://github.com/GreptimeTeam/greptimedb/issues/3265)
* [greptimedb - Suma de comprobación para manifiestos](https://github.com/GreptimeTeam/greptimedb/issues/3004)
* [greptimedb - Adición de tipo JSON a GreptimeDB](https://github.com/GreptimeTeam/greptimedb/issues/3686)*
* [greptimedb - Marco de recursos restringidos para entornos integrados](https://github.com/GreptimeTeam/greptimedb/issues/3685)*
* [GreptimeTeam - Diseñar e implementar un programa de evaluación similar a TPC-DS/TPC-H para escenarios de series temporales](https://github.com/GreptimeTeam/greptime-bench)*

> "*" = Problemas abiertos para las solicitudes de los estudiantes a través de OSPP. A los estudiantes seleccionados se les asignará un mentor (s) y pueden recibir bonificaciones. Regístrese a través del [enlace OSPP](https://summer-ospp.ac.cn/org/orgdetail/32cda81d-a705-4ab7-8b13-7c27a86ac19a?lang=en).

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Ponentes

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y están aceptando presentaciones para unirse a su evento como orador.

* [Rust Argentina junio 2024](https://sessionize.com/rust-argentina-june/) | Cierra el 31/05/2024 | Buenos Aires, AR | Fecha del evento: 2024-06-04
* [EuroRust 2024](https://www.papercall.io/eurorust-2024) | Cierra el 03/06/2024 | Viena, Austria y en línea | Fecha del evento: 2024-10-10
* [Computación científica en Rust 2024](https://scientificcomputing.rs/) | Cierra 14/06/2024 | En línea | Fecha del evento: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Cierra 2024-07-22 | En línea | Fecha del evento: 2024-08-22

Si usted es un organizador de eventos que espera ampliar el alcance de su evento, envíe un enlace al sitio web de envío a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust).

## Actualizaciones del Proyecto Rust

329 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-07..2024-05-14

* [Analizar visitante: construir árbol de prueba en la sonda](https://github.com/rust-lang/rust/pull/124936)
* [consolidar los códigos de causa de obligación para las cláusulas where](https://github.com/rust-lang/rust/pull/124988)
* [corregir la estabilización const de 'last_chunk' para los segmentos](https://github.com/rust-lang/rust/pull/124836)
* [Cobertura: Soporte de cobertura de sucursal para let-else e if-let](https://github.com/rust-lang/rust/pull/124223)
* [cobertura: simplificar aún más la extracción de información cartográfica de MIR](https://github.com/rust-lang/rust/pull/124615)
* [Mostrar puntos de referencia de tiempo de pared con precisión de subnanosegundos](https://github.com/rust-lang/rust/pull/124774)
* [no hacer ICE en 'AnonConst's en 'diagnostic_hir_wf_check'](https://github.com/rust-lang/rust/pull/124219)
* [no hacer ICE en 'diagnóstico::on_unimplemented'] malformado en el extranjero](https://github.com/rust-lang/rust/pull/124683)
* [no añadir un asterisco inicial en el 'PartialEq'](https://github.com/rust-lang/rust/pull/124157)
* [no ice cuando no podemos evaluar una const a un valtree en el nuevo solucionador](https://github.com/rust-lang/rust/pull/124846)
* [no llames a 'env::set_var' en 'rustc_driver::install_ice_hook'](https://github.com/rust-lang/rust/pull/125063)
* [Se corrigieron los mensajes de error para 'break' dentro de las corrutinas](https://github.com/rust-lang/rust/pull/124777)
* [arreglar ICE al lanzar un tipo con error](https://github.com/rust-lang/rust/pull/124997)
* [arreglar los visitantes 'MemCategorization' y 'ExprUse' para el nuevo solucionador (esta vez es mejor)](https://github.com/rust-lang/rust/pull/124902)
* [corregir lógica insuficiente al buscar la asignación subyacente](https://github.com/rust-lang/rust/pull/124761)
* [arreglar más ICEs en 'diagnostic::on_unimplemented'](https://github.com/rust-lang/rust/pull/124875)
* [corregir el mensaje de error de análisis para metaelementos](https://github.com/rust-lang/rust/pull/124778)
* [manejar expresiones Deref en 'invalid_reference_casting'](https://github.com/rust-lang/rust/pull/124978)
* [manejar proyecciones de campo como la indexación de sectores en 'invalid_reference_casting'](https://github.com/rust-lang/rust/pull/124908)
* [ignorar 'RUSTC_WRAPPER' vacío en el arranque](https://github.com/rust-lang/rust/pull/124903)
* [ignorar los argumentos genéricos en las rutas de los atributos](https://github.com/rust-lang/rust/pull/124318)
* [implementar 'as_chunks' con 'split_at_unchecked'](https://github.com/rust-lang/rust/pull/124793)
* [implementar el formateador lldb para enumeraciones "codificadas por clang" (LLDB 18.1+) (V3)](https://github.com/rust-lang/rust/pull/124781)
* [mejorar la depurabilidad de 'rustc_parse::P arser](https://github.com/rust-lang/rust/pull/124779)
* [hacer '#! [característica]' sugerencia 'MaybeIncorrect'](https://github.com/rust-lang/rust/pull/124926)
* [hacer que 'Ty::builtin_deref' solo devuelva un 'Ty'](https://github.com/rust-lang/rust/pull/124957)
* [asegúrese de que consumimos un arg genérico cuando revisamos turbofish mal escrito](https://github.com/rust-lang/rust/pull/124930)
* [asegúrese de que no negamos las variables macro con nombres de palabras clave](https://github.com/rust-lang/rust/pull/124869)
* [Ergonomía del partido 2024: que los patrones '&' se coman a '&mut'](https://github.com/rust-lang/rust/pull/124567)
* [Match Ergonomics 2024: migration lint](https://github.com/rust-lang/rust/pull/124639)
* [pretty-print let-else con paréntesis añadidos cuando sea necesario](https://github.com/rust-lang/rust/pull/125051)
* [Eliminar llaves al fijar un árbol de uso anidado en un solo elemento](https://github.com/rust-lang/rust/pull/123344)
* [cambiar el nombre de 'Generics::p arams' a 'Generics::own_params'](https://github.com/rust-lang/rust/pull/124953)
* [simplificar las ocurrencias de 'usar caja::rustc_foo::bar'](https://github.com/rust-lang/rust/pull/124876)
* [separar 'ty::AliasTerm' de 'ty::AliasTy'](https://github.com/rust-lang/rust/pull/125076)
* [eleva 'TraitRef' a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/124982)
* [eleva varios tipos de '*Predicado' a 'rustc_type_ir'](https://github.com/rust-lang/rust/pull/125001)
* [usar menos orígenes al crear variables de tipo](https://github.com/rust-lang/rust/pull/124955)
* [patrones nunca: patrones nunca inferiores a 'Inalcanzable' en MIR](https://github.com/rust-lang/rust/pull/123332)
* [evite 'alloca's en codegen para sentencias simples 'mir::Aggregate'](https://github.com/rust-lang/rust/pull/123886)
* [interpretar/miri: mejores errores al fallar 'offset_from'](https://github.com/rust-lang/rust/pull/124923)
* [miri: 'io::Error' manejando: mantener el 'io::Error' completo durante más tiempo para que podamos dar mejores errores](https://github.com/rust-lang/miri/pull/3589)
* [Miri: Un poco de organización intrínseca](https://github.com/rust-lang/miri/pull/3601)
* [miri: permitir que los objetivos de prueba se establezcan a través de CLI args](https://github.com/rust-lang/miri/pull/3588)
* [Miri: Intrínsecos: Entra en pánico cuando se usan incorrectamente](https://github.com/rust-lang/miri/pull/3604)
* [Miri: hacer que 'MIRI_TEST_TARGET' y 'RUSTC_BLESS' sean algo completamente interno](https://github.com/rust-lang/miri/pull/3590)
* [miri: devuelve un puntero no nulo de 'malloc(0)'](https://github.com/rust-lang/miri/pull/3580)
* [miri: soporte 'f*_algebraic'](https://github.com/rust-lang/miri/pull/3596)
* [miri: use un puntero no nulo para el tamaño 0 posix memalign](https://github.com/rust-lang/miri/pull/3600)
* [codegen: memmove/memset no puede ser atemporal](https://github.com/rust-lang/rust/pull/124932)
* [codegen-cranelift: traducir MIR a clif ir en paralelo con rustc paralelo](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1489)
* [estabilizar 'byte_slice_trim_ascii' por '&[u8]'/'&str'](https://github.com/rust-lang/rust/pull/124928)
* [estabilizar 'seek_seek_relative'](https://github.com/rust-lang/rust/pull/123817)
* [relajar los requisitos del asignador en algunas API de Rc/Arc](https://github.com/rust-lang/rust/pull/124981)
* ['f16::is_sign_{positive,negative}' fueron compuertas en f128](https://github.com/rust-lang/rust/pull/124828)
* ['io::Write::write_fmt': pánico si el formateador falla cuando el flujo no falla](https://github.com/rust-lang/rust/pull/125012)
* ['std::net: Socket::new_raw' ahora establecido en 'SO_NOSIGPIPE' en freebsd](https://github.com/rust-lang/rust/pull/124470)
* ['std::rand': añadiendo solaris/illumos para el soporte de getrandom](https://github.com/rust-lang/rust/pull/124766)
* [cargo: Ejemplo de agregar scripts de compilación solo locales en los documentos de check-cfg](https://github.com/rust-lang/cargo/pull/13884)
* [cargo: corrección: construir solo la biblioteca de artefactos especificada cuando hay varios tipos disponibles](https://github.com/rust-lang/cargo/pull/13842)
* [rustdoc: formulario de búsqueda desduplicado HTML](https://github.com/rust-lang/rust/pull/124738)
* [rustdoc: usa la estabilidad, en lugar de las características, para decidir qué mostrar](https://github.com/rust-lang/rust/pull/124864)
* [clippy: 'significant_drop_in_scrutinee': Corregir falsos positivos debido a falsas caídas de expresiones de lugar](https://github.com/rust-lang/rust-clippy/pull/12764)
* [clippy: añadir nueva pelusa 'doc_lazy_continuation'](https://github.com/rust-lang/rust-clippy/pull/12770)
* [clippy: agregar nuevos lint que no permiten cambiar el nombre de los parámetros en las funciones de rasgo](https://github.com/rust-lang/rust-clippy/pull/11540)
* [clippy: corrige falso positivo por falta de consideración del llamador mutable](https://github.com/rust-lang/rust-clippy/pull/12650)
* [clippy: corrección: fusionar múltiples sugerencias en una sola sugerencia de varios tramos en 'needless_late_init'](https://github.com/rust-lang/rust-clippy/pull/12777)
* [clippy: fix: use 'hir_with_context' para producir los fragmentos correctos para 'assigning_clones'](https://github.com/rust-lang/rust-clippy/pull/12783)
* [clippy: maneja 'rustc_on_unimplemented' en 'duplicated_attributes'](https://github.com/rust-lang/rust-clippy/pull/12620)
* [clippy: ignora las pelusas '_to_string' en el código 'from_expansion'](https://github.com/rust-lang/rust-clippy/pull/12780)
* [clippy: conflictos de prioridad directa de lint en '[workspace.lints]'](https://github.com/rust-lang/rust-clippy/pull/12730)
* [clippy: hacer que 'from_str_radix_10' omita el contexto constante](https://github.com/rust-lang/rust-clippy/pull/12787)
* [clippy: nueva pelusa: 'macro_metavars_in_unsafe'](https://github.com/rust-lang/rust-clippy/pull/12107)
* [rust-analyzer: arreglar OOM causado por la búsqueda de términos](https://github.com/rust-lang/rust-analyzer/pull/17203)
* [rust-analyzer: arreglar 'source_range' por 'INT_NUMBER' en la finalización](https://github.com/rust-lang/rust-analyzer/pull/17192)
* [rust-analyzer: corrección: mejorar los desplazamientos literales confusos](https://github.com/rust-lang/rust-analyzer/pull/17220)
* [rust-analyzer: fix: mantener paréntesis cuando la precedencia de la expr interna es menor que la externa](https://github.com/rust-lang/rust-analyzer/pull/17187)
* [rust-analyzer: corrección: informar de todos los errores del protocolo LSP con 'invalid_data'](https://github.com/rust-lang/rust-analyzer/pull/17207)
* [rust-analyzer: corrección: informar tanto de errores de E/S como de errores 'main_loop'](https://github.com/rust-lang/rust-analyzer/pull/17208)
* [rust-analyzer: implementar análisis de atributos inseguros](https://github.com/rust-lang/rust-analyzer/pull/17195)
* [rust-analyzer: use el campo repository para enlazar al repositorio](https://github.com/rust-lang/rust-analyzer/pull/17188)

### Clasificación del rendimiento del compilador de Rust

Una semana bastante tranquila con solo unos pocos PR marcados para su análisis.
Más mejoras que regresiones esta semana, y también varias buenas
reducciones de tamaño binario causadas por la generación de menos IR de LLVM.

Triaje realizado por **@kobzol**.
Rango de revisión: [69f53f5e.. 9105c57b](https://perf.rust-lang.org/?start=69f53f5e5583381267298ac182eb02c7f1b5c1cd&end=9105c57b7f6623310e33f3ee7e48a3114e5190a7&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.4% | [0.2%, 0.9%] | 8 |
| Regresiones ❌ <br /> (secundaria) | 0.9% | [0,2%, 2,4%] | 18 |
| Mejoras ✅ <br /> (primaria) | -1,1% | [-2,3%, -0,2%] | 51 |
| Mejoras ✅ <br /> (secundaria) | -0,6% | [-1.4%, -0.3%] | 19 |
| Todos ❌✅ (primario) | -0,9% | [-2,3%, 0,9%] | 59 |

1 regresión, 0 mejoras, 3 mixtas; 0 de ellos en rollups
75 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/Kobzol/rustc-perf/blob/0ab8cfe4bdc3044f8e610349d90c1708675b1ccf/triage/2024-05-14.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y los PR clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### Seguimiento de problemas y solicitudes de incorporación de cambios
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Problema de seguimiento para 'IpvNAddr::{BITS, to_bits, from_bits}' ('ip_bits')](https://github.com/rust-lang/rust/issues/113744)
* [disposición: fusionar] [Añadir 'size_of' y 'size_of_val' y 'align_of' y 'align_of_val' al preludio](https://github.com/rust-lang/rust/pull/123168)
* [disposición: fusionar] [desplazamiento: permitir desplazamiento de cero bytes en punteros arbitrarios](https://github.com/rust-lang/rust/pull/117329)
* [disposición: fusionar] [Añadir soporte '-' (stdin) en rustdoc](https://github.com/rust-lang/rust/pull/124611)
* [disposición: fusionar] [Advertir (o error) cuando se hace referencia al ctor 'Self' del elemento externo en el elemento anidado interno](https://github.com/rust-lang/rust/pull/124187)
* [disposición: fusionar] [Golpear 'elided_lifetimes'_in_associated_constant para denegar](https://github.com/rust-lang/rust/pull/124211)

##### [Directrices sobre códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Decidir sobre la validez de los metadatos de puntero/referencia ancho con cola de corte](https://github.com/rust-lang/unsafe-code-guidelines/issues/510)

#### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [nuevo] [[RFC] estáticas definibles externamente](https://github.com/rust-lang/rfcs/pull/3635)
* [nuevo] [Ámbito 'impl Trait for Type'](https://github.com/rust-lang/rfcs/pull/3634)
* [nuevo] [[RFC] 'core::marker::Freeze' in bounds](https://github.com/rust-lang/rfcs/pull/3633)
* [nuevo] [[RFC] funciones implementables externamente](https://github.com/rust-lang/rfcs/pull/3632)
* [nuevo] [RFC para doc_cfg, doc_cfg_auto, doc_cfg_hide y doc_cfg_show características](https://github.com/rust-lang/rfcs/pull/3631)

## Próximos eventos

Eventos oxidados entre 2024-05-15 - 2024-06-12 🦀

### Virtual

* 15/05/2024 | Virtual (Ankara, TR) | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 7**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-7-0a97e784)
* 15/05/2024 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Club de lectura de Rustaceans: Capítulo 6 - Pruebas**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300819214/)
* 15/05/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**NativeLink**](https://www.meetup.com/vancouver-rust/events/298542331/)
* 16/05/2024 | Virtual (Ankara, TR) | [Comunidad de Rust de Türkiye](https://kommunity.com/turkiye-rust-community/events)
    * [**#RustSemineri - 8**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-8-ddfe6b15)
* 16/05/2024 | Virtual (Charlottesville, VA, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 17/05/2024 | Virtual | [Capacitación 4 Programadores LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust at Full Speed: Harnessing Concurrency with Confidence**](https://www.eventbrite.com/e/rust-at-full-speed-harnessing-concurrency-with-confidence-tickets-884842296127)
* 2024-05-21 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rustful de mediados de mes: análisis forense a través de Artemisa**](https://www.meetup.com/rustdc/events/299346490/)
* 23/05/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477699/)
* 23/05/2024 | Virtual (Israel) | [Rust en Israel](https://rust.org.il/)
    * [**Desarrollo web en Rust usando Rocket (hebreo)**](https://www.meetup.com/code-mavens/events/300974367/)
* 28/05/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-28 y 2024-05-28 | Virtual | [Materia principal](https://mainmatter.com/)
    * [**Taller remoto: Telemetría para las API de Rust: no se puede arreglar lo que no se puede ver (tarifa)**](https://ti.to/mainmatter/rust-telemetry-may-2024)
* 30/05/2024 | Virtual + Presencial (Barcelona, ES) | [Materia principal](https://mainmatter.com/) y [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust para la web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 30/05/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 04/06/2024 | Virtual (Búfalo, NY) | [Encuentro de Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Grupo de usuarios de Buffalo Rust**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 05/06/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/299047896/)
* 06/06/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Reunión de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298477702/)
* 11/06/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/298341709/)

### África

* 01/06/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Reunión del Círculo de Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-22 | Singapur, SG | [Reunión de SG Rust](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Actualizado - Reunión de SG Rust en CraftsforGreen Whole Studio**](https://www.meetup.com/rust-singapore/events/300988123/)

### Europa

* 16/05/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 16/05/2024 | París, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zúrich, CH | [Rust Zúrich](https://www.meetup.com/rust-zurich/)
    * [**Reserve la fecha - Encuentro de mayo**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Desarrollo de software preparado para el futuro por FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Sesión de Dojo de Codificación**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 23/05/2024 | Berna, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 23/05/2024 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / ¿Qué es todo este alboroto sobre Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 23/05/2024 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/300923207/)
* 24/05/2024 | Burdeos, FR | [Rust Burdeos](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discusiones**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-25 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #3 [Embedded lab edition]**](https://www.meetup.com/stockholm-rust/events/301014982/)
* 2024-05-28 - 2024-05-30 | Berlín, DE | [Oxidar](https://oxidizeconf.com/)
    * [**Oxidar Conf 2024**](https://oxidizeconf.com/)
* 30/05/2024 | Barcelona, ES | [Materia principal](https://mainmatter.com/) y [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust para la web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 30/05/2024 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299288963/)
* 30/05/2024 | Copenhague, Dinamarca | [Comunidad de Rust de Copenhague](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 patrocinado por Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 30/05/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 05/06/2024 | Hamburgo, DE | [Encuentro de Rust Hamburgo](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn Junio 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 06/06/2025 | Vilnius, LT | [Vilna Rust](https://www.meetup.com/rust-in-vilnius/)
    * [**Disfruta de nuestro segundo evento de Rust y ZIG**](https://www.meetup.com/rust-in-vilnius/events/301012097/)

### América del Norte

* 16/05/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Reunión de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775539/)
* 16/05/2024 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 20/05/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, 20 de mayo**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Función doble de Rust Talk**](https://www.meetup.com/deep-dish-rust/events/300665520/)
* 30/05/2024 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 31/05/2024 | Boston, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Boston Common Rust, 31 de mayo**](https://www.meetup.com/bostonrust/events/300116786/)
* 08/06/2024 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust de Porter Square, 8 de junio**](https://www.meetup.com/bostonrust/events/300116799/)

### Oceanía

* 28/05/2024 | Sídney, Nueva Gales del Sur, Australia | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [** una demostración 🤯 y un espectáculo ✨ relámpago ⚡**](https://www.meetup.com/rust-sydney/events/300854266/)

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

> Desafortunadamente, la mayoría de la gente parece haber tomado la lección equivocada de Rust. Ven todo este negocio con vidas y propiedad como un desastre sucio que Rust ha tenido que adoptar porque quería evitar la recolección de basura. ¡Pero esto es completamente al revés! Rust adoptó reglas en torno al estado mutable compartido y esto le permitió evitar la recolección de basura. De todos modos, estas reglas son una buena idea.

– [sin barcos](https://without.boats/blog/references-are-like-jumps/)

¡Gracias a [Jules Bertholet](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1567) por la sugerencia de última hora!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ct92nz/this_week_in_rust_547/)</small>
