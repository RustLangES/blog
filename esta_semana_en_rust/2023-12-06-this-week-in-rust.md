---
title: "Esta semana en Rust #9"
number_of_week: 9
description: El crate de esta semana es symbols, una utilidad para crear rápidamente macros proc para solidificar las tablas de la base de datos en enumeraciones que permiten comprobaciones de claves externas en tiempo de compilación.
date: 2023-12-06
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

### Actualizaciones de proyectos/herramientas
* [Trippy (herramienta de diagnóstico de red) - versión 0.9.0](https://github.com/fujiapple852/trippy/releases/tag/0.9.0)

### Observaciones/Pensamientos
* [Vida útil temporal de Rust y "super let"](https://blog.m-ou.se/super-let/)
* [Detrás de escena del formato de cadenas de Rust: ¡format_args! ()](https://blog.m-ou.se/format-args/)
* [Perfilar programas de Rust de la manera más fácil](https://ntietz.com/blog/profiling-rust-programs-the-easy-way/)
* [Tres problemas de fijación](https://without.boats/blog/three-problems-of-pinning/)
* [¿¡Rust std fs más lento que Python!? ¡No, es hardware!](https://xuanwo.io/2023/04-rust-std-fs-slower-than-python/)
* [Leafpipe - Convirtiendo montones y montones de material audiovisual en bonitos estímulos](https://half-shot.uk/blog/leafpipe/)
* [Por qué es importante la seguridad de los tipos](https://www.shuttle.rs/blog/2023/11/29/type-safety)

### Tutoriales de Rust
* ["Rustificación" sin servidor: Aumente el rendimiento de AWS Lambda con Rust - AWS re:Invent talk](https://www.youtube.com/watch?v=Mdh_2PXe9i8)
* [video] [Acelerando el código de Rust con benchmarks y flamegraphs](https://www.youtube.com/watch?v=2IHPvPmzS8g)
* [Hacer una ruta segura de Axum](https://blog.sedrik.se/posts/secure-axum)

### Miscelánea
* [Funciones divergentes - funciones que nunca regresan](https://rust.code-maven.com/diverging-functions)
* [Primeros pasos con Loco, el framework web unipersonal para Rust](https://rust.code-maven.com/getting-started-with-loco)
* [Construyendo un interpolador de hipercubo (en Rust)](https://jlogan03.github.io/interpn/)
* [Embajada en ESP: GPIO](https://apollolabsblog.hashnode.dev/embassy-on-esp-gpio)
* [video] [Axum 0.6 a 0.7 en 5 pasos rápidos](https://www.youtube.com/watch?v=MvWCX5ckuDE)

## Crate de la semana

El crate de esta semana es [symbols](https://crates.io/crates/symbols), una utilidad para crear rápidamente macros proc para solidificar las tablas de la base de datos en enumeraciones que permiten comprobaciones de claves externas en tiempo de compilación.

¡Gracias a [Marco Napetti](https://users.rust-lang.org/t/crate-of-the-week/2704/1267) por la autosugestión!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Convocatoria a la participación

¿Siempre quisiste contribuir a proyectos de código abierto pero no sabías por dónde empezar?
¡Cada semana destacamos algunas tareas de la comunidad de Rust para que las elijas y comiences!

Algunas de estas tareas también pueden tener mentores disponibles, visite la página de tareas para obtener más información.

* [Ockam - Biblioteca - Validar estructuras CBOR de acuerdo con el esquema cddl para 'nodos/models' misc](https://github.com/build-trust/ockam/issues/6690)
* [Ockam - Comando - refactorizar para usar interfaces tipadas para implementar comandos para 'nodos'](https://github.com/build-trust/ockam/issues/6701)
* [Ockam - Biblioteca - Adelgazar el 'NodeManagerWorker' para 'nodo / estado del nodo'](https://github.com/build-trust/ockam/issues/6707)
* [zerocopy - Probar la salida de zerocopy-derive](https://github.com/google/zerocopy/issues/367)
* [zerocopy - Use cargo-semver-checks para asegurarse de que la función 'derivar' no cambie la superficie de la API](https://github.com/google/zerocopy/issues/422)
* [zerocopy - Verifique que el trabajo de CI 'all-jobs-succeeded' dependa de todos los demás trabajos](https://github.com/google/zerocopy/issues/444)
* [Hyperswitch - Refactor - Nuvei - Validación de metadatos MCA](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - Función - Mediodía - Sincronización con Referencia de Hyperswitch](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - Función - Payme - Referencia de sincronización con Hyperswitch](https://github.com/juspay/hyperswitch/issues/2906)
* [Hyperswitch - Error - Los errores de deserialización de metadatos MCA deben ser 4xx](https://github.com/juspay/hyperswitch/issues/2899)
* [Hyperswitch - Función - Zen - Referencia de sincronización con Hyperswitch](https://github.com/juspay/hyperswitch/issues/2908)

Si eres propietario de un proyecto de Rust y estás buscando colaboradores, envía tareas [aquí][directrices].

[directrices]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Actualizaciones del Proyecto Rust

369 solicitudes de extracción fueron [fusionadas en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-27..2023-12-04

* [add '-Zfunction-return={keep,thunk-extern}' option](https://github.com/rust-lang/rust/pull/116892)
* [cuenta por '!' brazo en cola 'match' expr](https://github.com/rust-lang/rust/pull/117526)
* [añadir puerta de función 'never_patterns'](https://github.com/rust-lang/rust/pull/118157)
* [añadir 'pretty_terminator' a bastante estable-mir](https://github.com/rust-lang/rust/pull/118172)
* [añadir una suposición de que el índice es entrante a 'slice::get_unchecked'](https://github.com/rust-lang/rust/pull/116915)
* ['rustc_span': Usar la distancia de edición correcta para las sugerencias](https://github.com/rust-lang/rust/pull/118381)
* [se agregaron métodos de rasgo de enlazador 'linker_arg(s)' para link-arg para que tengan el prefijo "-Wl", para argumentos de enlazador similares a cc y no textualmente](https://github.com/rust-lang/rust/pull/118202)
* [Permitir la configuración de etiquetas 'RLA' a través de 'rustbot'](https://github.com/rust-lang/rust/pull/114708)
* [evitar expansiones de cierre por registro](https://github.com/rust-lang/rust/pull/118347)
* ['generic_const_exprs': sugerir agregar la función, no usarla](https://github.com/rust-lang/rust/pull/118486)
* [cambiar la representación de 'SwitchTarget' en StableMIR](https://github.com/rust-lang/rust/pull/118461)
* ['rustc_hir_typeck': Arreglar ICE al sondear la alternativa de función no ASCII](https://github.com/rust-lang/rust/pull/118514)
* [constProp: eliminar correctamente const si se le asigna un valor desconocido](https://github.com/rust-lang/rust/pull/118426)
* [Cobertura: omitir intervalos que no se pueden desexpandir de nuevo al cuerpo de la función](https://github.com/rust-lang/rust/pull/118525)
* [Tamaño del código de corte para hash de características](https://github.com/rust-lang/rust/pull/118348)
* [detectar cortes similares a los de Python y sugerir cómo solucionarlo](https://github.com/rust-lang/rust/pull/111133)
* [detectar y rechazar sugerencias 'repr(Rust)' malformadas](https://github.com/rust-lang/rust/pull/118366)
* [dispose 'llvm::TargetMachines' antes de que se elimine 'llvm::Context'](https://github.com/rust-lang/rust/pull/118464)
* [no sugiera '!' para la ruta en la llamada a la función si tiene args genéricos](https://github.com/rust-lang/rust/pull/118342)
* [devuelve ansiosamente 'ExprKind::Err' en 'yield'/'await' en un contexto de corrutina incorrecto](https://github.com/rust-lang/rust/pull/118419)
* [efectos: ejecuta 'enforce_context_effects' para todas las llamadas a métodos](https://github.com/rust-lang/rust/pull/118282)
* [explique una buena razón por la que LocalValue no almacena el tipo de local](https://github.com/rust-lang/rust/pull/118482)
* [arreglar ICE: 'fn_arg_names: elemento inesperado DefId(..) «](https://github.com/rust-lang/rust/pull/118526)
* [corrige los argumentos de 'PartialEq' cuando '#[const_trait]' está habilitado](https://github.com/rust-lang/rust/pull/118379)
* [arreglar un ICE cuando un valtree no se evaluó](https://github.com/rust-lang/rust/pull/118498)
* [arreglar el ICE del analizador de attrs](https://github.com/rust-lang/rust/pull/118542)
* [Se soluciona el problema de sugerir desenvolver/esperar para el campo abreviado](https://github.com/rust-lang/rust/pull/118413)
* [dar un mensaje de error amigable para el desarrollo para perfiles de configuración incorrectos](https://github.com/rust-lang/rust/pull/118323)
* [Controlar el límite de recursividad para subtipos y predicados bien formados](https://github.com/rust-lang/rust/pull/117754)
* [implementar el estacionamiento de subprocesos para XOUS](https://github.com/rust-lang/rust/pull/116839)
* [Errores más específicos cuando los tipos externos terminan en lugares donde no deberían](https://github.com/rust-lang/rust/pull/118551)
* [Nuevo solucionador: Mejorar las anotaciones de los instrumentos](https://github.com/rust-lang/rust/pull/118454)
* [en Fn arg discrepancia para una ruta fn, sugerir un cierre](https://github.com/rust-lang/rust/pull/117805)
* [pase + función de atómicos forzados para riscv32{i,im,imc}-unknown-none-elf](https://github.com/rust-lang/rust/pull/114499)
* [realizar optimizaciones de LTO con wasm-ld + -Clinker-plugin-lto](https://github.com/rust-lang/rust/pull/118378)
* [Imprimir lista de características de destino faltantes al llamar a una función con características de destino fuera de un bloque no seguro](https://github.com/rust-lang/rust/pull/118333)
* [proporcionar una sugerencia estructurada para la discordancia de tipos en el bucle](https://github.com/rust-lang/rust/pull/118072)
* [eliminar la suposición memcpy-on-equal-ptrs](https://github.com/rust-lang/rust/pull/118265)
* [reemplace 'once_cell::sync::OnceCell' con std 'OnceLock'](https://github.com/rust-lang/rust/pull/118528)
* [Informar de errores en el servidor de trabajo heredados a través de variables de entorno](https://github.com/rust-lang/rust/pull/113730)
* [restaurar '#! [no_builtins]' en la LTO](https://github.com/rust-lang/rust/pull/113923)
* [restringir qué símbolos se pueden usar en cadenas de formato '#[diagnostic::on_unimplemented]']](https://github.com/rust-lang/rust/pull/118495)
* [rustc: armonizar 'DefKind' y 'DefPathData'](https://github.com/rust-lang/rust/pull/118573)
* [simplificar la sangría en la impresión THIR](https://github.com/rust-lang/rust/pull/118341)
* [mensaje de ajuste en ADT con construcción de campos privados](https://github.com/rust-lang/rust/pull/118453)
* [Ajustar la recuperación de análisis sintáctico de enumeraciones, para exprs y patrones de brazo de coincidencia](https://github.com/rust-lang/rust/pull/117565)
* [advertir contra el uso de intrínsecos que se salgan del alcance de nuestro modelo de memoria](https://github.com/rust-lang/rust/pull/118128)
* [añadir más información a la instancia de StableMIR](https://github.com/rust-lang/rust/pull/118524)
* [codegen, miri: se corrige el cálculo del desplazamiento de un campo sin tamaño en una 'estructura' empaquetada](https://github.com/rust-lang/rust/pull/118540)
* [Miri: Admite una alineación 'prometedora' para la verificación de alineación simbólica](https://github.com/rust-lang/rust/pull/117840)
* [miri: Máscaras de bits SIMD: use 'redondear a múltiplo de 8' en lugar de 'sujetar al menos a 8'](https://github.com/rust-lang/miri/pull/3206)
* [miri: añadir nuevos intrínsecos SIMD](https://github.com/rust-lang/miri/pull/3204)
* [miri: eliminar la heurística de GC de préstamos apilados](https://github.com/rust-lang/miri/pull/3194)
* [añadir también 'is_empty' a const raw slices](https://github.com/rust-lang/rust/pull/118231)
* [mover las API de procedencia expuesta a una puerta de características separada](https://github.com/rust-lang/rust/pull/118487)
* [estabilizar literales de cadena C](https://github.com/rust-lang/rust/pull/117472)
* [agregar API de subcadena para 'OsStr'](https://github.com/rust-lang/rust/pull/118484)
* [optimize 'str::iter::Chars::advance_by'](https://github.com/rust-lang/rust/pull/115331)
* [añadir 'track_caller' para operaciones aritivas](https://github.com/rust-lang/rust/pull/114841)
* [expandir la especialización de iteración in situ a Flatten, FlatMap y ArrayChunks](https://github.com/rust-lang/rust/pull/110353)
* [solucionador de carga: Quitar prioridad a la versión sin Rust en el solucionador de MSRV](https://github.com/rust-lang/cargo/pull/13066)
* [solucionador de carga: Eliminar el manejo de errores de deps públicos no utilizados](https://github.com/rust-lang/cargo/pull/13036)
* [cargo toml: Desacoplar la lógica del esquema](https://github.com/rust-lang/cargo/pull/13080)
* [cargo: añadir '--public' para 'cargo add'](https://github.com/rust-lang/cargo/pull/13046)
* [Cargo: añadir más comentarios en el documento para los cambios en GC](https://github.com/rust-lang/cargo/pull/13055)
* [cargo: reorder los indicadores '--remap-path-prefix' para '-Zbuild-std'](https://github.com/rust-lang/cargo/pull/13065)
* [cargo: se corrigió que la desinstalación de un binario en ejecución falló en Windows](https://github.com/rust-lang/cargo/pull/13053)
* [Cargo: corrige que la visualización del recuento de errores es diferente cuando solo queda un error](https://github.com/rust-lang/cargo/pull/12484)
* [cargo: haz que cargo agregue --optional '' <dep>crea una característica '' <dep>= '"dep:<dep>'](https://github.com/rust-lang/cargo/pull/13071)
* [cargo: incluir la lista declarada de características en la huella dactilar para '-Zcheck-cfg'](https://github.com/rust-lang/cargo/pull/13012)
* [cargo: eliminar el comentario obsoleto](https://github.com/rust-lang/cargo/pull/13076)
* [rustdoc: Añadido resaltado para comentarios en la declaración de artículos](https://github.com/rust-lang/rust/pull/117869)
* [rustdoc-search: permitir espacios alrededor de ':' en la consulta de ruta](https://github.com/rust-lang/rust/pull/118452)
* [clippy: 'missing_asserts_for_indexing': aceptar comprobaciones de igualdad de longitud](https://github.com/rust-lang/rust-clippy/pull/11837)
* [clippy: 'option_if_let_else': no se activa en expresiones que devuelven '()'](https://github.com/rust-lang/rust-clippy/pull/11896)
* [clippy: 'redundant_closure_call': evita la palabra clave 'async' duplicada cuando se activa en el cierre que devuelve el bloque 'async'](https://github.com/rust-lang/rust-clippy/pull/11363)
* [clippy: 'redundant_guards': atrapa 'is_empty', 'starts_with' y 'ends_with' en rebanadas y 'str's](https://github.com/rust-lang/rust-clippy/pull/11818)
* [clippy: agregar lint contra pruebas unitarias en doctests](https://github.com/rust-lang/rust-clippy/pull/11872)
* [clippy: permitir 'permitir' 'upper_case_acronyms' en variantes de 'enumeración'](https://github.com/rust-lang/rust-clippy/pull/11898)
* [clippy: expandiendo lint 'blocks_in_if_conditions' para verificar que coincida con expr también](https://github.com/rust-lang/rust-clippy/pull/11853)
* [clippy: nueva pelusa: 'repeat_vec_with_capacity'](https://github.com/rust-lang/rust-clippy/pull/11597)
* [rust-analyzer: depurar el uso de la raíz del espacio de trabajo de carga como 'cwd'](https://github.com/rust-lang/rust-analyzer/pull/15993)
* [rust-analyzer: implementa la finalización de los campos invocables](https://github.com/rust-lang/rust-analyzer/pull/15879)
* [Rust-analyzer: soporte inicial para sugerencia implícita de incrustación de caída](https://github.com/rust-lang/rust-analyzer/pull/16000)
* [rust-analyzer: no hacer el diagnóstico 'MissingMatchArms' para el cuerpo del fósforo vacío](https://github.com/rust-lang/rust-analyzer/pull/15971)
* [rust-analyzer: mejora el manejo de errores para las sentencias 'let' de nivel superior](https://github.com/rust-lang/rust-analyzer/pull/15961)

### Clasificación del rendimiento del compilador de Rust

Un pequeño número de cambios de rendimiento que, por desgracia, dieron lugar a bastantes regresiones de rendimiento. Una gran parte de esas regresiones estaban en rustdoc y se consideraron aceptables, ya que rustdoc ahora está haciendo estrictamente más trabajo. Algunas otras regresiones ya se han corregido y es de esperar que pronto se fusionen. La última de las regresiones aún está bajo investigación, pero esperemos que se resuelva pronto.

Triaje realizado por **@rylev**.
Rango de revisión: [df0295f0.. 9358642E](https://perf.rust-lang.org/?start=df0295f07175acc7325ce3ca4152eb05752af1f2&end=9358642e3b8560eee89e6f40aa996c8394a3db31&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 3.0% | [0,2%, 12,3%] | 53 |
| Regresiones ❌ <br /> (secundaria) | 4.1% | [0.2%, 11.6%] | 102 |
| Mejoras ✅ <br /> (primaria) | -0,3% | [-0.5%, -0.1%] | 65 |
| Mejoras ✅ <br /> (secundaria) | -0,6% | [-1,2%, -0,2%] | 25 |
| Todos ❌✅ (primario) | 1.1% | [-0,5%, 12,3%] | 118 |

4 regresiones, 1 mejoras, 1 mixta; 1 de ellos en rollups
60 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/9086dc608bbb15310b2063ab690be021339e3850/triage/2023-12-05.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son los RFC que se aprobaron para su implementación esta semana:

<!-- RFC aprobados vaya aquí, use este formato: * [Tema](URL) -->
<!-- o si no se aprobó ninguno esta semana, use: * *No se aprobaron RFC esta semana.* -->
<!-- * []() -->
<!-- ### [Propuestas de Cambio Mayor Aprobadas (MCP)](https://forge.rust-lang.org/compiler/mcp.html) <!~~ Los MCP ocurren con poca frecuencia, por lo que esta sección está comentada de forma predeterminada. ~~>
<!~~ Los MCP que han sido aprobados o rechazados esta semana van aquí, use este formato: * [cambio importante aceptado|rechazado] [Tema](URL) ~~>
-->
* *Esta semana no se aprobaron RFC.*

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las relaciones públicas clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFC](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *Ninguna RFC entró en el Período de Comentarios Final esta semana.*

#### [Seguimiento de problemas y solicitudes de incorporación de cambios](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Agregar lint contra comparaciones ambiguas de puntero ancho ](https://github.com/rust-lang/rust/pull/117758)
* [disposición: fusionar] [Estabilizar 'ptr::{from_ref, from_mut'}](https://github.com/rust-lang/rust/pull/117824)
* [disposición: fusionar] [Problema de seguimiento para 'any::type_name_of_val'](https://github.com/rust-lang/rust/issues/66359)
* [disposición: fusionar] [rustdoc: permitir cambiar el tamaño de la barra lateral / ocultar la barra superior](https://github.com/rust-lang/rust/pull/115660)
* [disposición: fusionar] [No fusionar los atributos cfg y doc(cfg) para reexportaciones](https://github.com/rust-lang/rust/pull/113091)
* [disposición: fusionar] [Convertir 'IMPLIED_BOUNDS_ENTAILMENT' en un error grave de una pelusa](https://github.com/rust-lang/rust/pull/117984)
* [disposición: fusionar] [Exhaustividad: revelar correctamente los tipos opacos](https://github.com/rust-lang/rust/pull/116821)
* [Disposición: Fusionar] [Arreglar los límites implícitos de los elementos Fn/Const y la comprobación de WF](https://github.com/rust-lang/rust/pull/104098)
* [disposición: fusionar] [garantizar que char y u32 son compatibles con ABI](https://github.com/rust-lang/rust/pull/118032)
* [disposición: fusionar] [Usar la clasificación de versiones para todas las clasificaciones](https://github.com/rust-lang/rust/pull/115046)
* [disposición: fusionar] [Problema de seguimiento para 'arc_unwrap_or_clone'](https://github.com/rust-lang/rust/issues/93610)
* [disposición: fusionar] [añade un número de columna a 'dbg!() «](https://github.com/rust-lang/rust/pull/114962)
* [disposition: merge] [Permitir cualquier bloque de expresión 'const' en 'thread_local!'](https://github.com/rust-lang/rust/pull/116392)
* [disposición: fusionar] [comprobar el tipo de retorno 'FnDef' para WF](https://github.com/rust-lang/rust/pull/115538)

### [Referencia del idioma](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

### [Directrices de código inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de la Guía de Código Inseguro entró en el Período Final de Comentarios esta semana.*

### [RFCs nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Hacer que Cargo respete la versión mínima admitida de Rust (MSRV) al seleccionar dependencias](https://github.com/rust-lang/rfcs/pull/3537)
* [RFC: Rasgo para ! Punteros finos de tamaño](https://github.com/rust-lang/rfcs/pull/3536)

### [Convocatoria de pruebas](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
Un paso importante para la implementación de RFC es que las personas experimenten con el método
implementación y dar retroalimentación, especialmente antes de la estabilización.  Lo siguiente
Las RFC se beneficiarían de las pruebas de usuario antes de seguir adelante:

* *Ninguna RFC emitió una convocatoria para pruebas esta semana.*

Si usted es un implementador de funciones y desea que su RFC aparezca en la lista anterior, agregue la nueva 'llamada para pruebas'
a su RFC junto con un comentario que proporcione instrucciones de prueba y/o orientación sobre qué aspecto (s) de la función
necesitan pruebas.

## Próximos eventos

Eventos oxidados entre 2023-12-06 - 2024-01-03 🦀

### Virtual

* 06/12/2023 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/297172140)
* 10/12/2023 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583652/)
* 12/12/2023 | Virtual | [Materia principal](https://mainmatter.com)
    * [**Taller: Telemetría para aplicaciones de Rust**](https://rust-telemetry-workshop.mainmatter.com)
* 12/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo martes**](https://www.meetup.com/dallasrust/events/297532862/)
* 14/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 14/12/2023 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 17/12/2023 | Virtual (Tel Aviv, IL) | [Expertos en código](https://www.meetup.com/code-mavens/)
    * [**¡Que no cunda el pánico! - Nuestro viaje hacia el manejo de errores en Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 18/12/2023 | Virtual (Múnich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Espejo**](https://berline.rs/)
* 19/12/2023 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/297128156/)
* 20/12/2023 | Virtual (Vancouver, Columbia Británica, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Aventuras en el desarrollo de aplicaciones egui**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 26/12/2023 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 28/12/2023 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Charlottesville Rust](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 03/01/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)

### Asia

* 10/12/2023 | Tokio, JP | [Reunión de Rust en Tokio](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Diversión con etiquetas, macros y Rust inseguro**](https://www.meetup.com/tokyo-rust-meetup/events/297674153)
* 16/12/2023 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europa

* 06/12/2023 | Colonia, DE | [Colonia Rust](https://www.meetup.com/rustcologne/events)
    * [**Advenimiento del Rust**](https://www.meetup.com/rustcologne/events/297100007/)
* 07/12/2023 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 07/12/2023 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 07/12/2023 | Londres, Reino Unido | [Grupo de usuarios de Rust London](https://www.meetup.com/rust-london-user-group/)
    * [**Fiesta de Navidad de Rust London con Realm (RSVP en Eventbrite)**](https://www.meetup.com/rust-london-user-group/events/297703135/)
* 14/12/2023 | Augsburgo, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 14/12/2023 | Basilea, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Pruebas: Aprende de los profesionales**](https://www.meetup.com/rust-basel/events/297599586/)
* 18/12/2023 | Múnich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - híbrido**](https://www.meetup.com/rust-munich/events/296429053/)
* 19/12/2023 | Heidelberg, DE | [Elimina tus insectos y oxida tus motores](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 19/12/2023 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, una alternativa al electrón**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### América del Norte

* 07/12/2023 | Seattle, WA, EE. UU. | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/)
    * [**Reunión del grupo de usuarios de Seattle Rust**](https://www.meetup.com/seattle-rust-user-group/events/297533440/)
* 11/12/2023 | Minneapolis, MN, EE. UU. | [Reunión de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760172/)
* 12/12/2023 | Seattle, WA, EE. UU. | [Cap Hill Rust Codificación/Hackeo/Aprendizaje](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 12/12/2023 | Nueva York, NY, EE. UU. | [Rust de Nueva York](https://www.meetup.com/rust-nyc/)  
    * [**Rust NYC Monthly Mixer: ¡Comparte, muestra y cuenta! 🦀 **](https://www.meetup.com/rust-nyc/events/297659937/)
* 13/12/2023 | Chicago, IL, EE. UU. | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/297671182/)
* 14/12/2023 | Lehi, UT, EE. UU. | [Rust de Utah](https://www.meetup.com/utah-rust/)
    * [**Envío de datos con canales con Herbert Wolverson**](https://www.meetup.com/utah-rust/events/297720170/)
* 14/12/2023 | Mountain View, CA, EE. UU. | [Reunión de Rust de Mountain View](https://www.meetup.com/mv-rust-meetup/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628069/)
* 15/12/2023 | Somerville, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297633899/)
* 19/12/2023 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)
* 27/12/2023 | Austin, TX, EE. UU. | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)

### Oceanía

* 11/12/2023 | Perth, WA, AU | [Grupo de Meetup de Rust Perth](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Evento de fin de año de Rust**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, vea el último hilo de [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/182f6dv/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> el firmware de NVIDIA, dijo Airlie, viene con un conjunto de archivos de inclusión que, a su vez, definen estructuras que cambian con el tiempo. Para hacer frente a estos cambios, el controlador va a necesitar algún tipo de generación automatizada de ABI; señaló que los desarrolladores que trabajan en el controlador de GPU M1 de Apple se han encontrado con el mismo problema. Este problema podría ser más fácil de abordar, sugirió, si el controlador, como el controlador M1, se reescribiera en Rust.

– [Jonathan Corbet parafraseando a David Airlie en Linux Weekly News](https://lwn.net/SubscriberLink/953144/b85b695d0c760692)

¡Gracias a [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1494) por la sugerencia!

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/18dax97/this_week_in_rust_524/)</small>
