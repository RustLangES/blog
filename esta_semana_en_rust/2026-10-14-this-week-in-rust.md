---
title: "Esta semana en Rust #129"
number_of_week: 129
description: El crate de esta semana es hotcode, una caja para recargar en caliente funciones de Rust en tiempo de ejecución. 
date: 2026-10-14
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenidos a otro número de *Esta Semana en Rust*! 
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que permite a todos crear software fiable y eficiente. 
Este es un resumen semanal de su progreso y comunidad. 
¿Quieres que se mencione algo? Etiquetanos en
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) en Bluesky o
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o
[mándanos una solicitud de retirada](https://github.com/rust-lang/this-week-in-rust). 
¿Quieres participar? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md). 

*This Week in Rust* está desarrollado abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos pueden consultarse en [this-week-in-rust.org](https://this-week-in-rust.org/). 
Si encuentras algún error en el número de esta semana, [por favor presenta un RP](https://github.com/rust-lang/this-week-in-rust/pulls). 

¿Quieres TWIR en tu bandeja de entrada? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485). 

## Actualizaciones de la comunidad Rust

<!-- Queridos colaboradores de la comunidad: Por favor, leed README.md para orientarse sobre las aportaciones. Cada enlace enviado debe ser de la forma: * [Título de la página enlazada](https://example.com/my_article) Si añades un enlace a un contenido no textual, por favor prefijadlo con '[vídeo]' o '[audio]': * [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article) * [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast) Si no sabes qué categoría usar, siéntete libre de enviar un PR de todas formas y simplemente pide a los editores que seleccionen la categoría. -->

### Oficial

* [Resumen del Equipo de Infraestructura 2026 y Plan del Cuarto trimestre](https://blog.rust-lang.org/inside-rust/2026/10/12/infrastructure-team-q3-recap-and-q4-plan/)

### Fundación

* [Actualizaciones de la Junta de la Fundación Rust | Octubre 2026](https://rustfoundation.org/media/rust-foundation-board-updates-october-2026/)

### Boletines

* [El Rustaceano Incrustado Número #82](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-82)

### Actualizaciones de proyectos/herramientas

* [Bevy 0,20](https://bevy.org/news/bevy-0-20/)
* [Creusot 0.14.0](https://devlog.creusot.rs/v0-14-0/)
* [Laboratorio de Señales Yagi](https://brian-armstrong.github.io/yagi/lab.html)

<!-- NOTA IMPORTANTE: Ya no aceptamos solicitudes de arrastre para la sección de Actualizaciones de Proyectos/Herramientas. Consulta aquí para más detalles: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observaciones/Pensamientos

* [Todo el código convergerá en Rust](https://mainmatter.com/blog/2026/10/08/all-code-will-converge-on-rust/)
* [Por qué las pruebas 'externalizadas' de los rasgos cíclicos no funcionan](https://smallcultfollowing.com/babysteps/blog/2026/10/10/modular-vs-external-proofs/)
* [Visualizando la Recuperación Basada en Época](https://sofiabelen.github.io/projects/visualizing-crossbeam-epoch-based-reclamation/)
* [Cómo Signal y WhatsApp despliegan Rust en miles de millones de dispositivos Android e iOS](https://kerkour.com/rust-ios-android-signal-whatsapp)
* [Actualización sobre los objetivos del proyecto de Rust](https://lwn.net/SubscriberLink/1098617/d020e93fea8f12d2/)
* [Analizando programas de Rust con Caronte](https://lwn.net/SubscriberLink/1097198/ebf7de6c107d042e/)
* [Poseer tipos C del código Rust](https://lwn.net/SubscriberLink/1099015/54401bed7cd9c3ef/)
* [Vídeo] [Entrevista RustConf 2025: Manish Goregaokar, Rust Devtools](https://www.youtube.com/watch?v=l-GaaivVJCE)

### Guías de Rust

* [JA] [同じ質問に2回お金を払うのをやめた: LLM の前に置くだけの重複排除プロキシを Rust (Tokio) で作った](https://qiita.com/mattbusel/items/98045dcee4b4420ed981)
* [Usando TrustZone-M en Rust en la Raspberry Pi Pico 2](https://tweedegolf.nl/en/blog/241/using-trustzone-m-in-rust-on-the-raspberry-pi-pico-2/)
* [Escribiendo una aplicación hipermedia en Rust](https://benjamingeer.eu/en/post/hypermedia-rust/)
* [estructuras personalizadas como Nodos de Árbol](https://gitlab.gnome.org/World/Rust/markup-rs/xml-tech-stack-in-rust/-/wikis/Papers/Custom-structs-as-Tree-Nodes)
* [Cómo perfilar el rendimiento de Rust usando Grafana y Prometheus](https://hotpath.rs/blog/rust-performance-grafana)
* [Vídeo] [RustCurious lección 11: Représtamo y las vidas invisibles de Rust](https://www.youtube.com/watch?v=zaDvLfljbMk)
* [Vídeo] [Telemetría de BAML alimentada por Rust: 1000× más rápida y sin pérdidas](https://www.youtube.com/watch?v=XhtLk0gEu40)
* [Vídeo] [Prueba de borrón en un simulador de eventos discretos](https://www.youtube.com/watch?v=yMYBCxrRJyo)

## Crate de la semana

El crate de esta semana es [hotcode](https://github.com/alordash/hotcode), una caja para recargar en caliente funciones de Rust en tiempo de ejecución. 

¡Gracias a [alordash](https://users.rust-lang.org/t/crate-of-the-week/2704/1697) por la autosugerencia! 

[Por favor, enviad vuestras sugerencias y votos para la próxima semana] [submit_crate]! 

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llama a pruebas
Un paso importante para la implementación de RFC es que las personas experimenten con el
Implementación y dar retroalimentación, especialmente antes de la estabilización. 

Si eres un implementador de funciones y quieres que tu RFC aparezca en esta lista, añade una
Etiqueta de 'llamada para pruebas' a tu RFC junto con un comentario que ofrece instrucciones de prueba y/o
orientación sobre qué aspecto(s) de la funcionalidad necesitan pruebas. 

*Esta semana no se emitieron llamadas para realizar pruebas por
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Carga](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen), 
[Ruído](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) o
[RFCs en lengua oxidada](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).* 

[Haznos saber](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu reportaje se registre como parte de esta lista. 

## Llamamiento a la participación; proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar. 
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces. 

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información. 

*Esta semana no se presentaron convocatorias para participar.* 

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente. 

*Esta semana no se presentaron convocatorias ni presentaciones.* 

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)! 

## Actualizaciones del Proyecto Rust

603 pull requests se han [fusionado en la última semana][fusionado]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-06..2026-10-13

#### Compilador
* [permitir que los tipos de operandos unarios se infieran más adelante](https://github.com/rust-lang/rust/pull/159744)
* [evitar clonar nodos AST de rasgos impl en resolución tardía](https://github.com/rust-lang/rust/pull/164074)
* [evita marcar dos veces 'self.bound_vars'' en TypeVisitor impl de ValidateBoundVars](https://github.com/rust-lang/rust/pull/164206)
* [marca 'is_indirect_first_projection' al reemplazar en RefProp](https://github.com/rust-lang/rust/pull/163778)
* [implementa 'clone_from' para 'BorrowckDomain'](https://github.com/rust-lang/rust/pull/164089)
* [Eliminación de movimiento MIR (3/6): PreciseLiveness](https://github.com/rust-lang/rust/pull/163337)
* [perf: caché FFI de seguridad resulta en pelusa 'improper_ctypes'](https://github.com/rust-lang/rust/pull/163348)
* [reserva más capacidad para el internador de símbolos](https://github.com/rust-lang/rust/pull/164150)
* [Saltar la detección de FMA en tiempo de ejecución cuando está activada en tiempo de compilación en 'x86_64'](https://github.com/rust-lang/compiler-builtins/pull/1330)
* [estabilizar la función de 'aritmética amplia' de WebAssembly](https://github.com/rust-lang/rust/pull/160877)
* [estabilizar el atributo 'optimizar'](https://github.com/rust-lang/rust/pull/157273)
* [tratan los campos de unión como aliasing entre sí en 'value_analysis'](https://github.com/rust-lang/rust/pull/163425)

#### Biblioteca
* [añadir 'fs::rename_noreplace'](https://github.com/rust-lang/rust/pull/162027)
* [desasigna directamente los nodos en lugar de recorrer cada elemento cuando BTreemap cae](https://github.com/rust-lang/rust/pull/162931)
* [camino rápido para 'BTreeMap::Iter::next'](https://github.com/rust-lang/rust/pull/163521)
* [hacer que 'NonNull::{read,write}_volatile' conste inestablemente](https://github.com/rust-lang/rust/pull/164051)
* [estabilizar 'ptr::try_cast_aligned'](https://github.com/rust-lang/rust/pull/154170)
* [pre-escaneo vectorizado 'vec::d edup'](https://github.com/rust-lang/rust/pull/162810)

#### Carga
* [evitar rutas absolutas en las huellas si son relativas a la raíz del espacio de trabajo](https://github.com/rust-lang/cargo/pull/17573)
* [corrección: la instalación de carga desde un código fuente falla para paquetes binarios sin dependencias](https://github.com/rust-lang/cargo/pull/17552)
* ['config': No procesar inclusiones para archivos 'credentials.toml'](https://github.com/rust-lang/cargo/pull/17569)
* ['git': Añadir advertencia de incompatibilidad futura en preparación para eliminar la detección automática](https://github.com/rust-lang/cargo/pull/17568)
* ['pistas': soportar niveles mínimos de optimización](https://github.com/rust-lang/cargo/pull/17368)
* ['trabajo': Permitir la separación explícita de procesos hijos en Windows](https://github.com/rust-lang/cargo/pull/17527)
* ['layout': excluye un director-compilación creado como padre de las copias de seguridad](https://github.com/rust-lang/cargo/pull/17560)
* ['mock-std': Establecer versiones estables de caja a 0.0.0](https://github.com/rust-lang/cargo/pull/17565)
* ['pkgid-spec': Análisis y formato de especificaciones de paquetes incorporados](https://github.com/rust-lang/cargo/pull/17539)
* [refactorización: modernizar algunos usos de la API para la versión 1.99](https://github.com/rust-lang/cargo/pull/17584)
* [restringir dep-info relativa al espacio de trabajo a los paquetes de espacio de trabajo](https://github.com/rust-lang/cargo/pull/17575)
* ['rustdoc': suprimir advertencias no especificadas de la edición](https://github.com/rust-lang/cargo/pull/17557)
* ['prueba': marca que requiere SSH para las pruebas que le pagan](https://github.com/rust-lang/cargo/pull/17563)

#### Rustdoc
* [arreglar más enlaces de 'incoherent_impl'](https://github.com/rust-lang/rust/pull/163841)
* [prefiere rutas locales sobre remotas cuando un artículo extranjero se reexporta localmente](https://github.com/rust-lang/rust/pull/164007)
* [comprobar cadena de reexportación si está en línea antes de ocultar](https://github.com/rust-lang/rust/pull/164033)

#### Clippy
* [configuración: añadir pelusa de 'too_long_first_doc_paragraph' a la lista de 'comprobar-elementos-privados'](https://github.com/rust-lang/rust-clippy/pull/17476)
* [no activar 'manual_is_multiple_of' en divisores literales](https://github.com/rust-lang/rust-clippy/pull/16585)
* [asegurarse de que se emita una advertencia de decatividad para 'inicializadores de campo-estructura-inconsistente de lint'](https://github.com/rust-lang/rust-clippy/pull/17841)
* [nuevos lints 'deprecated_attributes_without_'{'desde', 'nota'}](https://github.com/rust-lang/rust-clippy/pull/17609)
* [arreglar {'pánico', 'todo', 'no implementado', 'inalcanzable'}](https://github.com/rust-lang/rust-clippy/pull/17704)
* [corregir 'module_name_repetitions' falso negativo en reexportaciones agrupadas](https://github.com/rust-lang/rust-clippy/pull/17848)
* [corregir 'needless_lifetimes' linting donde las cláusulas se viven](https://github.com/rust-lang/rust-clippy/pull/17850)
* ['manual_filter': no pelusas cuando la condición necesita '&mut' a...](https://github.com/rust-lang/rust-clippy/pull/17810)
* ['multiple_unsafe_ops_per_block': fijar falso positivo en con escritura al campo de unión](https://github.com/rust-lang/rust-clippy/pull/17457)
* [nueva pelusa: 'unnecessary_box_pin'](https://github.com/rust-lang/rust-clippy/pull/13598)
* [saltar 'try_from_instead_of_from_str' en expansiones macro](https://github.com/rust-lang/rust-clippy/pull/17843)
* [apoyo tras '::self' para 'unnecessary_self_imports'](https://github.com/rust-lang/rust-clippy/pull/17785)

#### Analizador de Rust
* [soporte llamada en línea en llamada macro](https://github.com/rust-lang/rust-analyzer/pull/23428)
* [añadir manejo descendente en captura de cierre 'enum'](https://github.com/rust-lang/rust-analyzer/pull/23476)
* [asistir no añadir solo turbofish de por vida](https://github.com/rust-lang/rust-analyzer/pull/23493)
* [check call sin paren después de los dos puntos de ruta](https://github.com/rust-lang/rust-analyzer/pull/23487)
* [clave-valor completo cfg en cadena dentro de 'cfg! ()'](https://github.com/rust-lang/rust-analyzer/pull/23507)
* [completo en el nivel superior tras el inner-doc](https://github.com/rust-lang/rust-analyzer/pull/23486)
* [no asignar el código fuente a cadenas '#[doc]' que contienen escapes](https://github.com/rust-lang/rust-analyzer/pull/23443)
* [la decodificación de nicho de enum debería coincidir con rustc](https://github.com/rust-lang/rust-analyzer/pull/23508)
* [arreglar el manejo de parámetros genéricos en consts anónimos](https://github.com/rust-lang/rust-analyzer/pull/23179)
* [pasar ruta rustc a unstable-book-gen](https://github.com/rust-lang/rust-analyzer/pull/23503)
* [cuando el inicializador en 'let' diverge, permite 'else' no divergente](https://github.com/rust-lang/rust-analyzer/pull/23500)
* [mini refactorización en la inferencia de captura de cierre](https://github.com/rust-lang/rust-analyzer/pull/23482)
* [PERF: Interrogar algunas cosas que el solucionador llama y que son bastante caras de calcular](https://github.com/rust-lang/rust-analyzer/pull/23207)
* [estabilizar optimizar atributo](https://github.com/rust-lang/rust-analyzer/pull/22511)
* [implicaciones de la característica objetivo de sincronización de 'rustc_target'](https://github.com/rust-lang/rust-analyzer/pull/23483)
* [error de solución rápida de tipoMismatch: 'to_owned()'](https://github.com/rust-lang/rust-analyzer/pull/22185)

### Triaje de rendimiento del compilador Rust

No hubo mejoras ni regresiones significativas esta semana. La mayoría de los PR marcados como mejoras habían tenido
un resultado similar. que fue causado por el ruido del codegen en las mediciones. 

Sin embargo, hubo algunas buenas victorias en el uso de memoria, gracias a [#163915](https://github.com/rust-lang/rust/pull/163915) y especialmente a [#163887](https://github.com/rust-lang/rust/pull/163887).  

Triaje hecho por **@Kobzol**. 
Rango de revisiones: [cc9a14f7.. 7211ea59](https://perf.rust-lang.org/?start=cc9a14f721fac5226338c61dcec7d5ab785bde82&end=7211ea594cbf137ef6118b1752b959dea7cb9088&absolute=false&stat=instructions%3Au)

**Resumen**: 

| (instrucciones:u) | media | alcance | cuenta |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,2% | [0,1%, 0,6%] | 73 |
| Regresiones ❌ <br /> (secundario) | 0,3% | [0,1%, 1,4%] | 57 |
| Mejoras ✅ <br /> (primaria) | -0,3% | [-1,0%, -0,1%] | 21 |
| Mejoras ✅ <br /> (secundario) | -0,3% | [-1,7%, -0,1%] | 28 |
| Todos ❌✅ (primario) | 0,1% | [-1,0%, 0,6%] | 94 |

3 regresiones, 21 mejoras, 2 mixtas; 1 de ellas en rollups
27 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/c0ecdb487d59cfbae4516ca92bef60b4906dd98f/triage/2026/2026-10-11.md). 

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/main)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana? 

* *No se aprobaron RFC esta semana.* 

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora. 

#### Problemas de seguimiento y marcas personales

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [política objetivo de nivel 3: CPU objetivo y características deben ser compatibles con backend upstream](https://github.com/rust-lang/rust/pull/162840)
* [Implementa 'AsMut<T>' y 'AsRef<T>' para '!'.](https://github.com/rust-lang/rust/pull/161253)
* [borrowck dataflow: respetar la superficialidad de borrow cuando se sacrifican préstamos que entran en conflicto con las asignaciones](https://github.com/rust-lang/rust/pull/160902)
* [mover el cálculo implícito de los límites a partir de borrowck](https://github.com/rust-lang/rust/pull/160491)
* [Permitir ':' después de 'pat_param'](https://github.com/rust-lang/rust/pull/162264)
* [1.100-beta regresión: se han eliminado los patrones de 'caja' (característica 'box_patterns')](https://github.com/rust-lang/rust/issues/163830)
* [Estabilizar '-Zembed-metadata'](https://github.com/rust-lang/rust/pull/163436)
* [Haz que 'std::fs::{File, ReadDir, DirEntry}' siempre sea 'needs_drop' incluso cuando no está soportado.](https://github.com/rust-lang/rust/pull/162444)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Permitir atributos cfg (externos) sobre elementos de tipos de tuplas y patrones de tuplas (struct)(https://github.com/rust-lang/rfcs/pull/3532)

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Definir las reglas finales de validez: referencias y sindicatos](https://github.com/rust-lang/reference/pull/2337)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Reserva previa de hotel 2027 de All Hands](https://github.com/rust-lang/leadership-council/issues/340)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), 
[Equipo Compilador](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen), 
[Equipo de Idiomas](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código Peligroso](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).* 
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista. 

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Retención del registro de apoyo](https://github.com/rust-lang/rfcs/pull/4016)
* [RFC: inseguro 'global_asm'](https://github.com/rust-lang/rfcs/pull/4014)
* [RFC para eliminación limitada de versiones crates.io de autoservicio](https://github.com/rust-lang/rfcs/pull/4015)

## Próximos eventos

Eventos Rusty entre el 14-10-2026 - el 11-11-2026 🦀

### Virtual
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 15-10-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/316943302/)
* 2026-10-18 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, EE.UU.) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/316788257/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Club de Lectura de Sistemas Operativos: Mecanismos de Paginación**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-222 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust y la GPU de Nativo a Web: Introducción a 'wgpu'](https://luma.com/k1978ath)
* 2026-10-222 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-26 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Ningún Python resultó dañado: enseñando a un pequeño MCU a aprender sobre la marcha**](https://luma.com/1byfc495)
* 2026-10-27 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Club de Lectura del Rust del Martes**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Razonamiento con Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creando un editor hexadecimal en Rust**](https://luma.com/q8i3385k)
* 01-11-2026 | Virtual | [Experto del Rust 🦀](https://luma.com/rust-maven)
    * [**Rust Maldito**](https://luma.com/omlooqv8)
* 2026-11-01 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Experto en el Rust 🦀](https://luma.com/rust-maven)
    * [**Resolviendo problemas de planificación del mundo real en Rust con SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust)
    * [** 👋 Reunión comunitaria**](https://www.meetup.com/women-in-rust/events/315773682/)
* 04-11-2026 | Virtual (Indianápolis, INA, EE.UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 05-11-2026 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackeo y Aprendiza**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, EE.UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254770/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Rust en persona noviembre de 2026 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)

### Europa
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22ª sesión de bcnrust**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Híbrido (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-14 | Colonia, DE | [Colonia Oxidada](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust en octubre: Macro-magia**](https://www.meetup.com/rustcologne/events/316891100/)
* 20-10-2026 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Creando un juego de mazmorra multiusuario web en tiempo real con Dioxus**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)
* 2026-10-22 | Karlsruhe, DE | [Rust Hack & Aprende Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Hack y Aprendizaje de Karlsruhe Hack and Learn bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/316799366/)
* 2026-10-22 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Interoperabilidad entre Rust y Python**](https://www.meetup.com/rust-community-toulouse/events/316880185/)
* 2026-10-27 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de Hack: Rust se encuentra con IA**](https://www.meetup.com/rust-aarhus/events/316796329/)
* 2026-10-27 | París, FR | [París Oxidado](https://www.meetup.com/rust-paris)
    * [**Rust meetup #89**](https://www.meetup.com/rust-paris/events/316865643/)
* 2026-10-31 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 01-11-2026 - 3 de 11-2026 | Italia, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - La Conferencia Internacional sobre el Rust en Italia**](https://rustlab.it/schedule)
* 2026-11-04 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Empezando con el nuevo controlador Rust para la base de datos de IA de Oracle**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milán, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Trenes Modelo Digitales & Rust: Estación de Mando DCC de Metal Desnudo en ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)

### Norteamérica
* 2026-10-14 | Los Ángeles, CA, EE.UU. [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust con Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 15-10-2026 | Híbrido (Seattle, WA, EE.UU.) | [Grupo de usuarios de Seattle Rust](https://www.meetup.com/join-srug)
    * [**Octubre de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/316943302/)
* 2026-10-17 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Central de Cambridge, 17 de octubre**](https://www.meetup.com/bostonrust/events/316924092/)
* 20-10-2026 | San Francisco, CA, EE.UU. | [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Híbrido (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Areneros de Agentes Desechables en Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, EE. UU. [Rust del Área de la Bahía](https://luma.com/bayarearust)
    * [**Rust del Área de la Bahía - Encuentro Incrustado**](https://luma.com/ur4pm34i)
* 2026-10-22 | Nueva York, NY, EE.UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC con Bryan Cantrill - Code Matters: Reflexiones sobre Rust en la era de los LLM**](https://www.meetup.com/rust-nyc/events/316901518/)
* 2026-10-24 | Boston, MA, EE.UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo Davis Square Rust, 24 de octubre**](https://www.meetup.com/bostonrust/events/316924143/)
* 2026-10-28 | Austin, TX, EE.UU. [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/316655403/)
* 31-10-2026 | Boston, MA, EE.UU. | [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en el North End, 31 de octubre**](https://www.meetup.com/bostonrust/events/316924288/)
* 05-11-2026 | Saint Louis, MO, EE.UU. [Rust STL](https://www.meetup.com/stl-rust)
    * [**HTMX y Rust**](https://www.meetup.com/stl-rust/events/316896797/)

### Sudamérica
* 2026-11-07 | Planalto Paulista, BR | [Encuentro de Rust São Paulo](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento. 
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información. 

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1wzctie/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> creo que hay un número distinto de cero de personas expertas en el campo que responderían genuinamente a "¿cómo funciona C?" con "no funciona, siguiente pregunta". 

– [Clar Fon en el zulipán de la lengua de Rust](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/optimize_for_size/near/629933186)

¡Gracias a [kleines Filmröllchen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1824) por la sugerencia! 

[¡Por favor, enviad citas y votad para la semana que viene!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

Esta semana en el Rust está editado por: 

* [Nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [Marianne Goldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilista](https://github.com/tzilist)

*El alojamiento de la lista de correo está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)* 

<small>[Debatir en r/rust](https://www.reddit.com/r/rust/comments/1x70ut5/this_week_in_rust_673/)</small>