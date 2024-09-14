---
title: "Esta semana en Rust #29"
number_of_week: 29
description: El crate de esta semana es cargo-override, un complemento de carga para anular rápidamente las dependencias.
date: 2024-09-11
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

### Oficial
* [Anunciando Rust 1.81.0](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html)
* [Cambios en 'impl Trait' en Rust 2024](https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html)

### Fundación
* [Actualización del Consejo de Liderazgo de septiembre de 2024](https://blog.rust-lang.org/inside-rust/2024/09/06/leadership-council-update.html)

### Boletines informativos
* [Computación Científica en Rust #2 (septiembre de 2024)](https://scientificcomputing.rs/monthly/2024-09)

### Actualizaciones de proyectos/herramientas
* [Versión 🎉 2.2 de la macro del constructor de próxima generación. Derivar sintaxis y soporte 🚀 cfg ](https://elastio.github.io/bon/blog/bon-builder-v2-2-release)
* [Redox OS 0.9.0 - Redox - Su próximo sistema operativo (Gen)](https://www.redox-os.org/news/release-0.9.0/)

### Observaciones/Pensamientos
* [Rust en RP2350](https://www.raspberrypi.com/news/rust-on-rp2350/)
* [Cómo construimos la detección de errores tipográficos de 300 μs para 1,3 millones de palabras en Rust](https://trieve.ai/building-blazingly-fast-typo-correction-in-rust/)
* [Compresión de cadenas con FSST](https://blog.spiraldb.com/compressing-strings-with-fsst/)
* [Información de código local usando Ollama con Rust, Qdrant, FastEmbed y OpenTelemetry](https://bosun.ai/posts/ollama-and-telemetry/)
* [WebP: El formato de compresión de la página web](https://purplesyringa.moe/blog/webp-the-webpage-compression-format/)
* [Portar C a Rust para un decodificador de medios AV1 rápido y seguro](https://www.memorysafety.org/blog/porting-c-to-rust-for-av1/)
* [Optimizando rav1d, un decodificador AV1 en Rust](https://www.memorysafety.org/blog/rav1d-performance-optimization/)
* [¡Una optimización que es imposible en Rust!](https://tunglevo.com/note/an-optimization-that's-impossible-in-rust/)
* [¿Por qué comencé a transmitir en vivo como desarrollador de Rust?](https://blog.orhun.dev/livestreaming/)
* [¿Qué tiene de difícil hacer hash de datos?](https://www.dfns.co/article/unambiguous-hashing)
* [Módulo-compañero para una función independiente - elementos asociados para una función](https://www.reddit.com/r/rust/comments/1fcmk37/rust_modulecompanion_for_a_standalone_function/)
* [No más consultas SQLx sin marcar](http://www.matildasmeds.com/posts/no-more-unchecked-sqlx-queries/)
* [video] [Renacimiento de las interfaces de usuario de terminales con Rust — FrOSCon 2024](https://www.youtube.com/watch?v=OxfxkWoHhxM)

### Tutoriales de Rust
* [video] [Build with Naz : Explore Linux TTY, proceso, señales con Rust - Parte 3/3 'tokio::p rocess::Command'](https://www.youtube.com/watch?v=8JeL1sGozO4)

## Crate de la semana

El crate de esta semana es [cargo-override](https://github.com/eopb/cargo-override/), un complemento de carga para anular rápidamente las dependencias.

¡Gracias a [Ajith](https://users.rust-lang.org/t/crate-of-the-week/2704/1344) por la sugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana] [submit_crate]!

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

* [Esta semana en Rust - ¡Estamos reclutando más editores!](https://docs.google.com/forms/d/e/1FAIpQLSecGti4F4PiEjSMX92YtvCShVutkXBxx2TFW4gFWWYmyAt7Bg/viewform?usp=sf_link)

### CFP - Eventos

¿Eres un orador nuevo o experimentado que busca un lugar para compartir algo genial? Esta sección destaca los eventos que se están planificando y que están aceptando presentaciones para unirse a su evento como orador.

<!-- los CFP van aquí, use este formato: * [**nombre del evento**](URL a CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad,estado,país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias para trabajos o presentaciones esta semana.* -->

Si usted es un organizador de eventos que espera expandir el alcance de su evento, envíe un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o comunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

Se [fusionaron 399 solicitudes de extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-09-03..2024-09-10

* [añadir soporte para GNU/Hurd en 'x86_64'](https://github.com/rust-lang/rust/pull/128345)
* [agregar soporte de destino para RTEMS Arm](https://github.com/rust-lang/rust/pull/127021)
* ['impl_trait_overcaptures': No te preocupes por las vidas contravariantes no capturadas si sobreviven a una vida capturada](https://github.com/rust-lang/rust/pull/129028)
* [añadir sugerencias para palabras clave mal escritas](https://github.com/rust-lang/rust/pull/129899)
* [agregar una pelusa interna que avisa cuando se accede a datos no rastreados](https://github.com/rust-lang/rust/pull/128919)
* [Tipos de sí mismo arbitrarios v2: punteros característica de puerta](https://github.com/rust-lang/rust/pull/129664)
* [autodiff Upstream - enzima backend](https://github.com/rust-lang/rust/pull/129176)
* [omitir la configuración del enlazador y la verificación de objetivos cruzados para comandos específicos](https://github.com/rust-lang/rust/pull/128871)
* [verifique WF de la firma del tipo de fuente en la transmisión del puntero fn](https://github.com/rust-lang/rust/pull/129021)
* [manejar correctamente la estabilidad de los atributos '#[diagnóstico]']](https://github.com/rust-lang/rust/pull/130036)
* [Cobertura: Cuenta para esperar cuando el Futuro esté listo de inmediato](https://github.com/rust-lang/rust/pull/130013)
* [Delegación: Apoyar los genéricos en los artículos de delegación asociados](https://github.com/rust-lang/rust/pull/126161)
* [distribuir 'rustc_codegen_cranelift' para Windows](https://github.com/rust-lang/rust/pull/128939)
* [no intentes probar metas desconocidas](https://github.com/rust-lang/rust/pull/129896)
* [No llamar a la consulta para calcular el diseño de corrutina para el cuerpo sintético del cierre asíncrono](https://github.com/rust-lang/rust/pull/129847)
* [No solicite desinfectantes para funciones desnudas](https://github.com/rust-lang/rust/pull/129891)
* [No omitir la configuración del enlazador para compilaciones de 'verificación'](https://github.com/rust-lang/rust/pull/130135)
* [no sugiera etiquetar los bloques 'const' y 'inseguros'](https://github.com/rust-lang/rust/pull/128701)
* [no construya un cuerpo por movimiento cuando el cierre asíncrono esté contaminado](https://github.com/rust-lang/rust/pull/129677)
* [no emita 'esperar'/'asumir' en opt-level=0](https://github.com/rust-lang/rust/pull/121614)
* [no almacenar la región en 'CapturedPlace'](https://github.com/rust-lang/rust/pull/129987)
* [arreglar ICE causado por la falta de un intervalo en un error de región](https://github.com/rust-lang/rust/pull/130137)
* [arreglar ICE en la validación de tipo CMSE](https://github.com/rust-lang/rust/pull/130064)
* [arreglar ICE cuando 'asm_const' y 'const_refs_to_static' están combinados](https://github.com/rust-lang/rust/pull/129472)
* [arreglar el doble manejo en 'collect_tokens'](https://github.com/rust-lang/rust/pull/129346)
* [Arreglar la habilitación de wasm-component-ld para que coincida con otras herramientas](https://github.com/rust-lang/rust/pull/130034)
* [corrección: obtener el tipo de LLVM de valor global](https://github.com/rust-lang/rust/pull/128820)
* [implementar vidas y etiquetas sin procesar (''r#ident')](https://github.com/rust-lang/rust/pull/126452)
* [implementar sugerencias para 'elided_named_lifetimes'](https://github.com/rust-lang/rust/pull/129840)
* [interpretar: hacer copias mecanografiadas con pérdida de procedencia y relleno](https://github.com/rust-lang/rust/pull/129778)
* [hacer que las consultas de superrasgos y predicados implícitos sean predeterminadas](https://github.com/rust-lang/rust/pull/129752)
* [las estructuras no exhaustivas pueden estar vacías](https://github.com/rust-lang/rust/pull/128934)
* [renombrar el volcado de corrutina by-move-body para que sea más consistente, arreglar ICE en 'dump_mir'](https://github.com/rust-lang/rust/pull/129706)
* [S390X: Corregir una regresión relacionada con la función de backchain](https://github.com/rust-lang/rust/pull/129940)
* [sugerir 'impl trait' para referencias a rasgo desnudo en el encabezado de la función](https://github.com/rust-lang/rust/pull/127692)
* [suprimir nichos en corrutinas para evitar violaciones de aliasing](https://github.com/rust-lang/rust/pull/129313)
* [use 'DeepRejectCtxt' para rechazar rápidamente a los candidatos de 'ParamEnv'](https://github.com/rust-lang/rust/pull/128776)
* [Miri: un poco de refactorización en "sync"](https://github.com/rust-lang/miri/pull/3874)
* [Miri: Detecta cuando se mueve 'pthread_mutex_t'](https://github.com/rust-lang/miri/pull/3784)
* [Miri: Detecta cuando se mueve 'pthread_rwlock_t'](https://github.com/rust-lang/miri/pull/3871)
* [miri: habilitar bibliotecas nativas en macOS](https://github.com/rust-lang/miri/pull/3856)
* [Miri: arreglar comentario en 'mutex_id_offset'](https://github.com/rust-lang/miri/pull/3865)
* [miri: variable renombrada y comentarios corregidos referentes a FileDescriptor renombrado](https://github.com/rust-lang/miri/pull/3867)
* [estabilizar '-Znext-solver=coherencia'](https://github.com/rust-lang/rust/pull/121848)
* [estabilizar 'char::MIN'](https://github.com/rust-lang/rust/pull/130154)
* [estabilizar 'const_float_bits_conv'](https://github.com/rust-lang/rust/pull/129555)
* [estabilizar 'waker_getters'](https://github.com/rust-lang/rust/pull/129919)
* [Reparar CVE-2024-43402](https://github.com/rust-lang/rust/pull/129962)
* [irrumpir en el depurador (si está conectado) en pánicos (Windows, Linux, macOS, FreeBSD)](https://github.com/rust-lang/rust/pull/129019)
* [const: hacer que 'ptr.is_null()' detenga la ejecución en la ambigüedad](https://github.com/rust-lang/rust/pull/130107)
* [hacer que 'Resultado::copiado' conste de manera inestable](https://github.com/rust-lang/rust/pull/130090)
* [str: hacer 'as_mut_ptr' y 'as_bytes_mut' de manera inestable const](https://github.com/rust-lang/rust/pull/130046)
* [use el algoritmo div trifecta para div de 128 bits en wasm](https://github.com/rust-lang/compiler-builtins/pull/685)
* [cargo: resolve: Reporte versión compatible con MSRV en lugar de incomptible](https://github.com/rust-lang/cargo/pull/14471)
* [cargo: new: Agregar al espacio de trabajo relativo al manifiesto, no al directorio actual](https://github.com/rust-lang/cargo/pull/14505)
* [Cargo: fianza antes de empaquetar en la misma versión](https://github.com/rust-lang/cargo/pull/14448)
* [Cargo: no incluya automáticamente la caja actual al empaquetar](https://github.com/rust-lang/cargo/pull/14488)
* [cargo: arreglar el agregado de carga que se comporta diferente al traducir el nombre del paquete](https://github.com/rust-lang/cargo/pull/13765)
* [cargo: Arreglar el análisis de valores separados por comas en --crate-type flag](https://github.com/rust-lang/cargo/pull/14499)
* [Cargo: Incluir el estado de dependencia público/privado en 'Metadatos de carga'](https://github.com/rust-lang/cargo/pull/14504)
* [cargo: espacio de trabajo de publicación](https://github.com/rust-lang/cargo/pull/14433)
* [Cargo: eliminar símbolos innecesarios](https://github.com/rust-lang/cargo/pull/14519)
* [cargo: uplift windows gnullvm import libraries](https://github.com/rust-lang/cargo/pull/14451)
* [rustdoc-search: permitir la búsqueda de arg 'Foo →' al final de la película](https://github.com/rust-lang/rust/pull/130009)
* [rustdoc: Ordena los artículos asociados por tipos y luego por apariencia](https://github.com/rust-lang/rust/pull/129471)
* [rustdoc: añadir mapa de cabecera a la tabla de contenidos](https://github.com/rust-lang/rust/pull/120736)
* [rustdoc: normalizar los nombres de tipos/campos](https://github.com/rust-lang/rust/pull/128667)
* [rustdoc: use strategic boxing para reducir 'clean::Item'](https://github.com/rust-lang/rust/pull/129789)
* [rustfmt: impl 'rewrite_result' para ForeignItem, TraitAliasBounds, WherePredicate](https://github.com/rust-lang/rustfmt/pull/6309)
* [rustfmt: impl 'rewrite_result' por 'ast::Expr'](https://github.com/rust-lang/rustfmt/pull/6311)
* [rustfmt: implementar version-sort para importaciones en 'style_edition' 2024](https://github.com/rust-lang/rustfmt/pull/6284)
* [bindgen: estabilizar '--wrap-static-fns'](https://github.com/rust-lang/rust-bindgen/pull/2928)
* [clippy: 'single_match', 'single_match_else': sugerencia de corrección cuando la coincidencia es irrefutable](https://github.com/rust-lang/rust-clippy/pull/13324)
* [clippy: 'manual_div_ceil': init](https://github.com/rust-lang/rust-clippy/pull/12987)
* [clippy: añadir una nueva comprobación para pasar punteros a un bloque 'asm!' con la opción 'nomem'](https://github.com/rust-lang/rust-clippy/pull/13247)
* [clippy: añadir nueva pelusa 'manual_is_power_of_two'](https://github.com/rust-lang/rust-clippy/pull/13327)
* [clippy: se agregó una nueva pelusa 'non_zero_suggestions'](https://github.com/rust-lang/rust-clippy/pull/13167)
* [clippy: arreglar 'needless_return' falso negativo](https://github.com/rust-lang/rust-clippy/pull/13214)
* [clippy: mover 'manual_c_str_literals' a la complejidad](https://github.com/rust-lang/rust-clippy/pull/13263)
* [clippy: solo lint 'manual_non_exhaustive' para los tipos exportados](https://github.com/rust-lang/rust-clippy/pull/13345)
* [clippy: visita los campos 'struct' recursivamente en la verificación de reserva de uninit](https://github.com/rust-lang/rust-clippy/pull/13367)
* [rust-analyzer: Soporte IDE para expresiones 'asm!](https://github.com/rust-lang/rust-analyzer/pull/18022)
* [Rust-analyzer: Sugerencias de mejores nombres para FN](https://github.com/rust-lang/rust-analyzer/pull/18041)
* [rust-analyzer: siempre establece explícitamente los tipos propios 'TraitRef' al bajar](https://github.com/rust-lang/rust-analyzer/pull/18068)
* [Rust-Analyzer: Atrape los pánicos de la computación de diagnóstico](https://github.com/rust-lang/rust-analyzer/pull/18065)
* [Rust-Analyzer: ¡Par de ASM! correcciones de análisis y descenso](https://github.com/rust-lang/rust-analyzer/pull/18053)
* [Rust-analyzer: No cunda el pánico en el hilo del escritor LSP en el receptor caído](https://github.com/rust-lang/rust-analyzer/pull/18066)
* [Rust-analyzer: Arreglar la disminución de los bucles for que sueltan el bloque de bucle](https://github.com/rust-lang/rust-analyzer/pull/18045)
* [Analizador de Rust: Prevenir adecuadamente la construcción de Mir con tipos desconocidos presentes](https://github.com/rust-lang/rust-analyzer/pull/18067)
* [Rust-analyzer: la actualización de la configuración no debería obstaculizar los proyectos descubiertos](https://github.com/rust-lang/rust-analyzer/pull/18059)

### Clasificación del rendimiento del compilador de Rust

Una semana relativamente tranquila, con la mayoría de las regresiones en rollups, lo que dificulta la investigación. Afortunadamente, las regresiones son relativamente pequeñas y, en general, la semana fue una ligera mejora en el rendimiento del compilador.

Triaje realizado por **@rylev**.
Rango de revisión: [6199b69c.. 263a3aee](https://perf.rust-lang.org/?start=6199b69c53a8c275ca3cd59647ea0af5ca29aae2&end=263a3aeeb8f2d0e9cc85eee61774d1f5f23dc3f5&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Gama | Recuento |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0.6% | [0.2%, 1.4%] | 57 |
| Regresiones ❌ <br /> (secundaria) | 0.7% | [0.2%, 1.5%] | 23 |
| Mejoras ✅ <br /> (primario) | -2,2% | [-4,0%, -0,4%] | 23 |
| Mejoras ✅ <br /> (secundaria) | -0,3% | [-0.3%, -0.2%] | 10 |
| Todos ❌✅ (primarios) | -0,2% | [-4,0%, 1,4%] | 80 |

3 regresiones, 1 mejora, 2 mixtas; 3 de ellos en rollups
26 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/fe90cb350de95f6947af4a84dbdb1100ba5d07ea/triage/2024-09-10.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que se aprobaron para su implementación esta semana:

* [Fusionar RFC 3637: Patrones de protección](https://github.com/rust-lang/rfcs/pull/3637)

### Período final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período final de comentarios' para las RFC y las RP clave
que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *Ninguna RFC entró en el Período Final de Comentarios esta semana.*

#### Seguimiento de problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposición: fusionar] [Constitución interna: aceptar punteros mutables interiores en el valor final](https://github.com/rust-lang/rust/pull/128543)
* [disposición: fusionar] [Estabilizar '&mut' (y '*mut') así como '&Cell' (y '*const Cell') en const](https://github.com/rust-lang/rust/pull/129195)
* [disposición: fusionar] [[library/std/src/process.rs] 'PartialEq' para 'ExitCode'](https://github.com/rust-lang/rust/pull/127633)
* [disposición: fusionar] [Relacionar el receptor invariablemente en la sonda del método para 'Modo::P ath'](https://github.com/rust-lang/rust/pull/129073)
* [disposición: fusionar] [(Anti)regresión entre Rust 1.78.0 y Rust 1.79.0 con estructura que contiene 'Cow<[Self]>](https://github.com/rust-lang/rust/issues/129541)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de carga o PR ingresaron al Período de comentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No hay problemas de seguimiento de equipos lingüísticos ni PR ingresados al período final de comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *Ninguna RFC de referencia lingüística entró en el Período Final de Comentarios esta semana.*

##### [Directrices para códigos inseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problemas de seguimiento de pautas de código inseguro o PR ingresados al período de comentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevas o actualizadas esta semana.*

## Próximos eventos

Eventos oxidados entre 2024-09-11 - 2024-10-09 🦀

### Virtual
* 2024-09-10 - 2024-09-13 | Híbrido: virtual y presencial (Montreal, QC, CA) | [Conf. Rust](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 12/09/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633268/)
* 12/09/2024 | Virtual (Róterdam, Países Bajos) | [Desarrollo de juegos Bevy](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 12/09/2024 | Virtual (San Diego, CA, EE. UU.) | [Rust de San Diego](https://www.meetup.com/san-diego-rust/)   
    * [**San Diego Rust septiembre de 2024 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/303363549/)
* 16/09/2024 | Virtual | [Mujeres en Rust](https://www.meetup.com/women-in-rust/)
    * [** 👋 Puesta al día de la comunidad**](https://www.meetup.com/women-in-rust/events/302827971/)
* 17/09/2024 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Oxidado a mediados de mes**](https://www.meetup.com/rustdc/events/299346969/)
* 18/09/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Celdas**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 18/09/2024 - 20/09/2024 | Híbrido - Virtual y Presencial (Viena, AT) | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2024**](https://lpc.events/event/18/sessions/186/)
* 19/09/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897973/)
* 19/09/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Encuentro de septiembre**](https://www.meetup.com/join-srug/events/303067835/)
* 24/09/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/301585670/)
* 26/09/2024 | Virtual (Berlín, DE) | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Espejo: Encuentro de Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298633269/)
* 26/09/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Comunicación segura oxidada en dispositivos integrados**](https://www.meetup.com/charlottesville-rust-meetup/events/303159380/)
* 02/10/2024 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/302031649/)
* 02/10/2024 | Virtual (Vancouver, Columbia Británica, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Aprovechando una extensión PL/RUST para proteger datos confidenciales en PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)
* 03/10/2024 | Virtual (Charlottesville, Carolina del Norte, EE. UU.) | [Reunión de Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Creación de intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298897992/)
* 08/10/2024 | Virtual (Dallas, TX, EE. UU.) | [Rust de Dallas](https://www.meetup.com/dallasrust/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/299346983/)

### África
* 05/10/2024 | Kampala, UG | [Círculo de Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunión de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 14/09/2024 | Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de septiembre de 2024**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)
* 21/09/2024 | Bangalore/Bangalore, IN | [Rust de Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentro rustáceo de septiembre de 2024**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europa
* 11/09/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 17/09/2024 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**2D-gamedev mit "bevy"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425049/)
* 17/09/2024 | Manchester, Reino Unido | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester September Code Night**](https://www.meetup.com/rust-manchester/events/303112113/)
* 17/09/2024 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Haciendo que los modelos de IA realicen tareas, en Rust!**](https://www.meetup.com/rust-trondheim/events/302957040/)
* 18/09/2024 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Encuentro de Rust Moravia (septiembre de 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 18/09/2024 | Viena, AT + Virtual | [Conferencia de plomeros de Linux](https://lpc.events)
    * [**Microconferencia de Rust en LPC 2024 (18-20 de septiembre)**](https://lpc.events/event/18/sessions/186/)
* 21/09/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Foro Fika de Ferris #5**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 23/09/2024 | Bratislava, SK | [Grupo de encuentro de Bratislava Rust](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup por Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)
* 24/09/2024 | París, FR | [Rust París](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #70**](https://www.meetup.com/rust-paris/events/303212378/)
* 24/09/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust)
    * [**Encuentro de Rust #70**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 26/09/2024 | Aarhus, Dinamarca | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Noche de charlas**](https://www.meetup.com/rust-aarhus/events/301522991/)
* 26/09/2024 | Augsburgo, DE | [Encuentro de Rust Augsburgo](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Rust Meetup #9: De bucles a pliegues**](https://www.meetup.com/rust-meetup-augsburg/events/302437593)
* 26/09/2024 | Berlín, DE | [OpenTechSchool Berlín](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Título**](https://www.meetup.com/rust-berlin/events/299421380/)
* 26/09/2024 | Praga, CZ | [Rust Praga](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Praga (septiembre de 2024)**](https://www.meetup.com/rust-prague/events/303346494/)
* 27/09/2024 | Mannheim, DE | [Hackerstolz e.V.](https://www.hackerstolz.de/en/)
    * [**Hackerstolz Stammtisch Rhein-Neckar**](https://www.hackerstolz.de/en/)
* 02/10/2024 | Oxford, Reino Unido | [Grupo de Encuentro de Oxfrod Rust](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust y C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123395/)
* 02/10/2024 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/Stockholm-Rust/events/303213095)
* 03/10/2024 | Nürnberg, DE | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820273/)
* 03/10/2024 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154268/)
* 09/10/2024 | Reading, Reino Unido | [Taller de lectura de Rust](https://rustworkshop.co/meetup/)
    * [**Encuentro de lectura de Rust**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)

### América del Norte
* 11/09/2024 | Boulder, CO, EE. UU. | [Encuentro de Boulder Rust](https://www.meetup.com/boulder-rust-meetup/)
    * [**Encuentro de Elixir de Rocas**](hhttps://www.meetup.com/boulder-elixir/events/302991078/)
* 12/09/2024 | Chicago, Illinois, Estados Unidos | [Rust de plato profundo](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/303286998/)
* 16/09/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Rust en Union Square en Somerville, 16 de septiembre**](https://www.meetup.com/bostonrust/events/302498750/)
* 17/09/2024 | Minneapolis, MN Estados Unidos | [Encuentro de Rust en Minneapolis](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/303374015/)
* 17/09/2024 | San Francisco, CA, EE. UU. | [Grupo de Estudio de la Roya de San Francisco](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638248/)
* 18/09/2024 | Virtual y presencial (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust/)
    * [**Celdas**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 19/09/2024 | Virtual y presencial (Seattle, WA, EE. UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Encuentro de septiembre**](https://www.meetup.com/join-srug/events/303067835/)
* 21/09/2024 | Longview, TX, EE. UU. | [Código de Longview y café](https://www.meetup.com/longview-code-and-coffee/)
    * [**Código de Longview y café**](https://www.meetup.com/longview-code-and-coffee/events/301976355/)
* 24/09/2024 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ferndale**](https://www.meetup.com/detroitrust/events/zfcbntygcmbgc/)
* 25/09/2024 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/302274449/)
* 26/09/2024 | Nashville, Tennessee, Estados Unidos | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Develpers : Presentaciones comunitarias**](https://www.meetup.com/music-city-rust-developers/events/301420118/)
* 27/09/2024 | Cambridge, MA, EE. UU. | [Reunión de Boston Rust](https://www.meetup.com/bostonrust/)
    * [**Almuerzo de Beacon Hill Rust, 27 de septiembre**](https://www.meetup.com/bostonrust/events/302498761/)
* 03/10/2024 | St. Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Iteradores en Rust**](https://www.meetup.com/stl-rust/events/302371456/)
* 08/10/2024 | Detroit, MI, EE. UU. | [Rust de Detroit](https://www.meetup.com/detroitrust/)
    * [**Encuentro de la comunidad de Rust - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcnblb/)

Si está ejecutando un evento de Rust, agréguelo al [calendario] para obtener
que se menciona aquí. Por favor, recuerde agregar un enlace al evento también.
Envíe un correo electrónico al [Equipo de la comunidad de Rust] [comunidad] para acceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajos de Rust: TWiR ha dejado de presentar ofertas de trabajo individuales. Puedes leer más sobre este cambio aquí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hilo de [Who's Hiring en r/rust](https://www.reddit.com/r/rust/comments/1fa0tf6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Frase de la semana

> ¡Ay! Una vez más estamos desamparados  
> de una cita para elatar o explicar  
> por lo que este editor simplemente ha dejado  
> la opción en rima para quejarse.

– llogiq

[¡Por favor, envíe sus cotizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin]( https://github.com/mariannegoldin), [BennyVasquez](https://github.com/bennyvasquez).*

*El alojamiento de la lista de correo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1fevt7b/this_week_in_rust_564/)</small>
