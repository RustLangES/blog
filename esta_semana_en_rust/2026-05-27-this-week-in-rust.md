---
title: "Esta semana en Rust #111"
number_of_week: 111
description: El crate de esta semana es inline\_tweak, una caja para incrustar constantes ajustables dentro de tu aplicación Rust sin necesidad de recompilación completa.
date: 2026-05-27
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

<!--

Estimados colaboradores de la comunidad:
Por favor, lee README.md para obtener orientación sobre las presentaciones.
Cada enlace enviado debe ser del siguiente tipo:

* [Título de la página enlazada](https://example.com/my_article)

Si añades un enlace a un contenido que no sea textual, por favor prefijadlo con '[vídeo]' o '[audio]':

* [vídeo] [Título del vídeo enlazado](https://example.com/my_video_article)
* [audio] [Título del archivo de audio enlazado](https://example.com/my_podcast)

Si no sabes qué categoría usar, siéntete libre de enviar una marca permanente de todas formas
Y simplemente pide a los editores que seleccionen la categoría.

-->

### Boletines

* [Scientific Computing in Rust #18 (mayo 2026)](https://scientificcomputing.rs/monthly/2026-05)

### Actualizaciones de proyectos/herramientas

* [gitRust - 26 de mayo](https://github.com/GitoxideLabs/gitoxide/discussions/2621)
* [resultados de la encuesta de usuarios hiper 2025](https://seanmonstar.com/blog/hyper-user-survey-2025-results/)
* [Actualización de Rust: ¡gRPC da la bienvenida a Tonic!](https://grpc.io/blog/grpc-welcomes-tonic/)
* [serde-const-default v0.1: Elimina el boilerplate al usar valores const como valores de campo por defecto](https://github.com/ifsheldon/serde-const-default/releases/tag/v0.1)
* [BoquilaHUB 0.5: IAs para la naturaleza. Ahora incluye modelos bioacústicos de IA SOTA y modelos de incrustaciones](https://github.com/boquila/boquilahub/releases/tag/v0.5)
* [splog: un TUI visor de registros con categorización automática de etiquetas](https://www.sextianbytes.fr/blog/imperfect-by-design/)
* [rgx v0.12.3 — Construcción de un depurador de regex para el terminal en Rust](https://dev.to/brevity1swos/building-a-regex-debugger-for-the-terminal-in-rust-977)
* [Las pruebas de interfaz son las barreras de seguridad que necesita una IA: la historia del portapapeles](https://davefx.com/en/2026/05/clipboardwire-construction-story/)
* [slintcn 0.22: componentes de copia y pega al estilo shadcn/ui para aplicaciones nativas de Slint](https://github.com/stevekwon211/slintcn/blob/main/docs/INTRODUCING_SLINTCN.md)
* [Lanzamiento de dtact v0.2.2 y rssn-advanced v0.1.0: el motor concurrente asincrónico de próxima generación y motor de computación científica](https://users.rust-lang.org/t/releasing-dtact-v0-2-2-and-rssn-advanced-v0-1-0/140278)

### Observaciones/Pensamientos

* [Noroboto: Fuentes mentiras y mitigación en Rust](https://tritium.legal/blog/noroboto)
* [Borrando existenciales](https://wolfgirl.dev/blog/2026-05-20-erasing-existentials/)
* [libwce: la capa de entropía de un códec wavelet, por sí sola](https://yogthos.net/posts/2026-05-24-libwce.html)
* [Notas técnicas: Theseus: traduciendo win32 a wasm](https://neugierig.org/software/blog/2026/05/theseus-wasm.html)
* [Explicación visual del motor de juego de Bevy](https://aibodh.com/posts/bevy-game-engine/)
* [El reflejo de derivar rasgos 'serde'](https://verrchu.github.io/blog/3-the-reflex-of-deriving-serde-traits/)
* [La IA física necesita un modelo de mundo tipado, no una base de datos vectorial](https://aimdb.dev/blog/typed-world-model)
* [Mantén la calma y usa (Rust) monorepos](https://kerkour.com/rust-monorepos)
* [audio] [Rust for Linux Live con Alice Ryhl y Greg Kroah-Hartman](https://corrode.dev/podcast/s06e04-rust4linux/)
* [audio] [Netstack.FM episodio 38 — Construcción y prueba de pilas de red con Rama](https://netstack.fm/#episode-38)
* [vídeo] [¿Se puede hacer un código QR de estrellas?](https://www.youtube.com/watch?v=RbmkNSqMvZY)

### Guías de Rust

* [Patrones de Rust y tutoriales de ingeniería](https://microsoft.github.io/RustTraining/rust-patterns-book/)
* [Errores de Laissez-Faire](https://hemomorphic.alexblood.net/posts/laissez-faire-errors/)
* [Aprende Rust HashMap e Iteradores construyendo un lector de almacenamiento de objetos git](https://blog.sheerluck.dev/posts/learn-hashmap-iterators-by-building-a-git-object-store-reader/)
* [Aprende lo básico de Bevy construyendo y desplegando Pong para Itch.io](https://blog.sheerluck.dev/posts/learn-the-basics-of-bevy-by-building-and-deploying-pong-to-itch-io/)
* [La ralentización que no aparece en los perfiles](https://cong-or.xyz/false-sharing-cache-lines.html)
* [Construyendo un ejecutor AsyncIO para la 3DS](https://blog.cat-girl.gay/3ds-async-part-one/)
* [vídeo] [Nueve formas de hacer herencia en Rust, un lenguaje sin herencia](https://www.youtube.com/watch?v=3IyKC5EtNkM)

### Miscelánea

* [Builds de Rust con contenido (o, lo que kache realmente almacena en caché)](https://kunobi.ninja/blog/what-kache-actually-caches)

## Crate de la semana

El crate de esta semana es [inline\_tweak](https://docs.rs/inline_tweak), una caja para incrustar constantes ajustables dentro de tu aplicación Rust sin necesidad de recompilación completa.

¡Gracias a [Kill The Mule](https://users.rust-lang.org/t/crate-of-the-week/2704/1607) por la sugerencia!

[Por favor, enviad vuestras sugerencias y votos para la próxima semana][submit_crate]!

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
[RFCs en lenguaje oxidado](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Cuéntanos](https://github.com/rust-lang/this-week-in-rust/issues) si quieres que tu característica se registre como parte de esta lista.

## Llamado a la participación; Proyectos y ponentes

### CFP - Proyectos

Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar.
Cada semana destacamos algunas tareas de la comunidad de Rust para que elijas y empieces.

Algunas de estas tareas también pueden tener mentores disponibles, visita la página de la tarea para más información.

* [Rust cookbook - Expandir la sección de línea de comandos con clap derive, subcomandos y vars ambientales](https://github.com/rust-lang-nursery/rust-cookbook/issues/760)
<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
<!-- o si no se ha presentado ninguna convocatoria esta semana.* -->

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->
- *No se presentaron convocatorias ni presentaciones esta semana.*

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

352 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-19..2026-05-26

#### Compilador
* ['rustc_on_unimplemented': introducir especificadores de formato](https://github.com/rust-lang/rust/pull/156161)
* [cuenta para los scans de macros de activación en diagnósticos de 'do_not_recommend'](https://github.com/rust-lang/rust/pull/156763)
* [implementa fast path para 'derive(PartialOrd)' al derivar 'Ord'](https://github.com/rust-lang/rust/pull/155598)
* [hacer que el bitset 'would_modify_words' sea más compatible con vectorizador](https://github.com/rust-lang/rust/pull/153640)
* [analizar restricciones de 'mut'](https://github.com/rust-lang/rust/pull/156824)
* [deja de necesitar lugares materializados para la mayoría de los intrínsecos](https://github.com/rust-lang/rust/pull/156116)

#### Biblioteca
* [añadir rasgo de compartir inestable](https://github.com/rust-lang/rust/pull/156828)
* [estabilizar 'bool_to_result'](https://github.com/rust-lang/rust/pull/156594)
* [usar índices envueltos fuertemente tipados en 'VecDeque'](https://github.com/rust-lang/rust/pull/152112)

#### Carga
* [compilador: reenviar bandera verbosa a rustc para cajas locales](https://github.com/rust-lang/cargo/pull/17006)
* [no uses la red para una prueba de prueba de publicación](https://github.com/rust-lang/cargo/pull/17027)
* [separar 'RegistryConfig' y 'crate_url' para interpretar 'RegistryConfig::d l'](https://github.com/rust-lang/cargo/pull/17011)
* [corregir CVE-2026-5222 y CVE-2026-5223](https://github.com/rust-lang/cargo/pull/17031)
* [artefacto: eliminar modo de compat de los artefactos](https://github.com/rust-lang/cargo/pull/17016)

#### Rustdoc
* [estabilizar '--remap-path-path-prefix' en rustdoc](https://github.com/rust-lang/rust/pull/155307)

#### Clippy
* ['useless_format': fuego encendido envuelto en un macro productor de bloques](https://github.com/rust-lang/rust-clippy/pull/17060)
* ['return' puede eliminarse del último stmt de un bloque si tiene un expr](https://github.com/rust-lang/rust-clippy/pull/16959)
* [añadir comprobación para el punto medio usando multiplicación por '0.5' y '>> 1'](https://github.com/rust-lang/rust-clippy/pull/17025)
* [evitar asignaciones innecesarias de 'String' en operaciones aritméticas 'MinifyingSugg'](https://github.com/rust-lang/rust-clippy/pull/17057)
* [extender 'clippy::missing_safety_doc' a campos inseguros](https://github.com/rust-lang/rust-clippy/pull/16767)
* [arreglar el manejo de 'manual_range_contains' NAN](https://github.com/rust-lang/rust-clippy/pull/17065)
* [mensaje de error de corrección para 'useless_borrows_in_formatting' para préstamos mutables](https://github.com/rust-lang/rust-clippy/pull/17036)
* [mueve 'unnecessary_get_then_check' a 'complejidad'](https://github.com/rust-lang/rust-clippy/pull/16998)
* [simplificar 'is_some() & ... desenrollar()' a 'is_some_and' en 'unit_arg'](https://github.com/rust-lang/rust-clippy/pull/17055)

#### Analizador de Rust
* ['diagnóstico: diagnóstico de características de enlace mut_ref'](https://github.com/rust-lang/rust-analyzer/pull/22406)
* ['asistes/add_reference_here: _modify_' el tipo de referencia al tratar con '&T->&mut T'](https://github.com/rust-lang/rust-analyzer/pull/22342)
* ['cfg': índice de separador correcto en el bucle de desactivación de CfgDiff](https://github.com/rust-lang/rust-analyzer/pull/22426)
* ['hir-ty': saturar float-to-uint fundido en evaluación de constrancia](https://github.com/rust-lang/rust-analyzer/pull/22430)
* ['Test-utils': drenar 'inactive_regions' por 'inactive_line_region'](https://github.com/rust-lang/rust-analyzer/pull/22427)
* [añadir diagnóstico para E0033](https://github.com/rust-lang/rust-analyzer/pull/22411)
* [añadir diagnóstico para E0608](https://github.com/rust-lang/rust-analyzer/pull/22404)
* [completaciones, importaciones excluyen soportes subelementos](https://github.com/rust-lang/rust-analyzer/pull/22416)
* [características con alcance de paquete de filtro](https://github.com/rust-lang/rust-analyzer/pull/22432)
* ['extract_module' falta importación para llamadas de macros](https://github.com/rust-lang/rust-analyzer/pull/22437)
* [añade 'type_match' para 'struct_pat'](https://github.com/rust-lang/rust-analyzer/pull/22452)
* [permitir parámetros comodines en declaraciones FN extranjeras](https://github.com/rust-lang/rust-analyzer/pull/22415)
* [análisis esperado ty en variante 'enum'](https://github.com/rust-lang/rust-analyzer/pull/22449)
* [variantes 'enum' autoimportadas](https://github.com/rust-lang/rust-analyzer/pull/22385)
* [no autorefer en el método probe en modo camino](https://github.com/rust-lang/rust-analyzer/pull/22392)
* [no completar punto y coma en el lugar de expansión de coincidencia](https://github.com/rust-lang/rust-analyzer/pull/22408)
* [no consideres que el camino de la macro en una llamada macro esté dentro de una llamada macro](https://github.com/rust-lang/rust-analyzer/pull/22397)
* [emitir diagnóstico para patrones de arreglos de reposo sin arreglos de longitud fija](https://github.com/rust-lang/rust-analyzer/pull/22424)
* [corregir los internados válidos técnicamente superpuestos de 'SyntaxContext::root](https://github.com/rust-lang/rust-analyzer/pull/21566)
* [Lanza 'coerce_never type_mismatch' Gracias](https://github.com/rust-lang/rust-analyzer/pull/22451)
* [tienen un error específico para macros incorporadas no implementadas](https://github.com/rust-lang/rust-analyzer/pull/22383)
* [no sugiere coincidencia de árbitro cuando se espera árbitro genérico](https://github.com/rust-lang/rust-analyzer/pull/22409)
* [No use patrón triste en el brazo feliz con protector](https://github.com/rust-lang/rust-analyzer/pull/22369)
* [normalizar el campo pat esperado de la tupla](https://github.com/rust-lang/rust-analyzer/pull/22425)
* [manejo refactorizado de parámetros genéricos en 'hir::Type'](https://github.com/rust-lang/rust-analyzer/pull/22252)
* [los consts de soporte nombrados en tipos de patrones de rango](https://github.com/rust-lang/rust-analyzer/pull/22396)
* [usar anotación agrupada para 'add_label_to_loop'](https://github.com/rust-lang/rust-analyzer/pull/22419)
* [proporcionar mejor incrementalidad para módulos](https://github.com/rust-lang/rust-analyzer/pull/22322)

### Triaje de rendimiento del compilador Rust

Esta semana ha sido mayormente positiva, con la mayoría de las mejoras derivadas del cambio de algoritmo en la comprobación de visibilidad: [#156228](https://github.com/rust-lang/rust/pull/156228).

Triaje hecho por **@panstromek**.
Rango de revisión: [281c97c3.. 783eb8c8](https://perf.rust-lang.org/?start=281c97c3240a9abd984ca0c6a2cd7389115e80d5&end=783eb8c8682ddde0807c60ed8293670ef523794f&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,1%, 0,7%] | 5 |
| Regresiones ❌ <br /> (secundario) | 0,5% | [0,1%, 1,1%] | 16 |
| Mejoras ✅ <br /> (primaria) | -0,9% | [-6,6%, -0,1%] | 164 |
| Mejoras ✅ <br /> (secundario) | -0,4% | [-1,3%, -0,1%] | 51 |
| Todos ❌✅ (primario) | -0,9% | [-6,6%, 0,7%] | 169 |

2 regresiones, 2 mejoras, 5 mixtas; 2 de ellos en rollups
34 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/4e9e90ee6ec008cadd1f351541185eff56319998/triage/2026/2026-05-25.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* [Propone el concepto de un nombre de usuario crates.io para la identidad](https://github.com/rust-lang/rfcs/pull/3946)

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)
* [Promociona 5 objetivos de brazo en modo pulgar desnudo a Nivel 2](https://github.com/rust-lang/compiler-team/issues/985)
* [Añadir -Z dead-fn-eliminación para saltarse el código de funciones BFS-inalcanzables](https://github.com/rust-lang/compiler-team/issues/976)

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Actualizar 'transmute_copy' a ub_checks y '? Sized'](https://github.com/rust-lang/rust/pull/155989)
* [Problema de seguimiento para los intrínsecos del producto escalar NEON](https://github.com/rust-lang/rust/issues/117224)
* [Nunca rompas entre paréntesis vacíos](https://github.com/rust-lang/rust/issues/152761)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [¡Evita poner 'unreachable_code' en 'todo!' ()'](https://github.com/rust-lang/rfcs/pull/3928)

##### [Directrices del Código de Peligros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [¿Cuáles son los valores de un tipo sindical? (en particular, ¿cuál es el invariante de validez de una unión)](https://github.com/rust-lang/unsafe-code-guidelines/issues/438)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*
Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 27-05-2026 - el 24-06-2026 🦀

### Virtual
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/9v7hv2g1)
* 2026-06-02 | Virtual | [libp2p Eventos](https://luma.com/libp2p)
    * [**Llamada de Mantenedores Abiertos de rust-libp2p**](https://luma.com/ukfh0mcf)
* 2026-06-02 | Virtual (Tel Aviv-yafo, IL) | [TLV de Rust 🦀](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/314871990/)
* 2026-06-03 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-04 | Virtual (Tel Aviv-yafo, IL) | [Expertos 🦀 en el Código - 🐍 - 🐪 ](https://www.meetup.com/code-mavens/)
    * [**Explorando FalkorDB - Aprendiendo a usar una base de datos de grafos en Rust**](https://www.meetup.com/code-mavens/events/314979560/) 
* 2026-06-06 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro del Círculo Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-06-07 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-09 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful a mitad de mes**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ekws5nr4)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto martes**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: ¿Qué demonios son las mónadas - y cómo las falsificamos en Rust**](https://www.meetup.com/women-in-rust/events/313767883/)

### Asia
* 2026-06-02 | Pekín, CN | [Voice AI y Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Agentes de IA y LLM de código abierto (Llamada a ponentes)**](https://www.meetup.com/wasm-rust-meetup/events/314750465/)

### Europa
* 2026-05-28 | Copenhague, DK | [Comunidad Copenhague Rust](https://www.meetup.com/copenhagen-rust-community)
    * [**Reunión de Rust #68**](https://www.meetup.com/copenhagen-rust-community/events/314868448/)
* 2026-05-28 | Londres, Reino Unido | [Grupo de Usuarios de Rust London](https://www.meetup.com/rust-london-user-group)
    * [**LDN habla de la muestra comunitaria de mayo**](https://www.meetup.com/rust-london-user-group/events/314846861/)
* 2026-05-29 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin habla: La próxima generación**](https://www.meetup.com/rust-berlin/events/314396588/)
* 30-05-2026 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust)
    * [**Foro Fika de Ferris #26**](https://www.meetup.com/stockholm-rust/events/314926826/)
* 2026-06-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**gRPC con Rust y Tónica**](https://www.meetup.com/rust-rhein-main/events/314051727/)
* 2026-06-03 | Dublín, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Únete en directo e INPERSONA para Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-10 | Múnich, DE | [Rust Múnich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Noche de Hacking**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Suiza, CH | [Después de TenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Cracovia, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactivo: Todo es de código abierto**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Milán en lengua oxidada](https://www.meetup.com/rust-language-milano)
    * [**Planificación en tiempo real en Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla en Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)

### Norteamérica
* 2026-05-27 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Adiós**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust en sistemas embebidos y autónomos en sistemas paralelos en DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 2026-05-28 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314716463/)
* 30-05-2026 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Central de Cambridge, 30 de mayo**](https://www.meetup.com/bostonrust/events/314480537/)
* 2026-06-04 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust)
    * [**Pruebas, Cobertura, Tracey y Mutaciones**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de la Óxida Común de Boston, 6 de junio**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, EE. UU. [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Encuentro de junio de Utah Rust**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**ENCUENTRO DE RUST en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, EE. UU. [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - ¡De vuelta en persona!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Encuentro de SRUG (Grupo de Usuarios de Seattle Rust) de junio de 2026**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Afar**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Solucionadores de restricciones basados en Rust en bocetos 2D con Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)

### Sudamérica
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién contrata en r/rust](https://www.reddit.com/r/rust/comments/1sobu1s/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> Esto satura hoy en día tanto al solucionador de rasgos como a mi cerebro

– [Nadrieril en su blog](https://nadrieril.github.io/blog/2026/05/14/when-can-traits-depend-on-themselves.html)

¡Gracias a [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1774) por la sugerencia!

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

<small>[Discutir en r/rust](https://www.reddit.com/r/rust/comments/1tptzbz/this_week_in_rust_653/)</small>
