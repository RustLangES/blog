---
title: "Esta semana en Rust #106"
number_of_week: 106
description: El crate de esta semana es Myth Engine, un motor de renderizado multiplataforma de alto rendimiento.
date: 2026-04-15
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

### Oficial
* [Resumen del Equipo de Infraestructura 2026 y Plan del Segundo Q2](https://blog.rust-lang.org/inside-rust/2026/04/14/infrastructure-team-q1-recap-and-q2-plan/)

### Actualizaciones de proyectos/herramientas
* [pquantum.dev: Criptografía postcuántica en Rust](https://pquantum.dev/)
* [haproxy-spoe-rs: Una biblioteca de agentes SPOA de Rust para HAProxy](https://blog.none.at/blog/2026/2026-04-12-haproxy-spoa-rs/)
* [Fresh 0.2.23: El IDE del terminal añade codificación Windows-1251, barra de estado personalizable y un buscador de archivos más rápido](https://github.com/sinelaw/fresh/releases/tag/v0.2.23)
* [KAIO v0.2.0: Escribiendo núcleos de GPU en Rust al 92,5% de cuBLAS](https://netviper.gr/blog/kaio-v020/)
* [RustNet: Un TUI de análisis de tráfico de red en tiempo real](https://netbeez.net/blog/rustnet/)
* [AimDB: La próxima era de la arquitectura de software es la prioridad por los datos](https://aimdb.dev/blog/data-driven-design)
* [tailscale-rs v0.2.0: nuestra nueva vista previa de la biblioteca de Rust](https://tailscale.com/blog/tailscale-rs-rust-tsnet-library-preview)
* [Sinbo: un gestor de fragmentos de CLI, almacenar fragmentos de código localmente con búsqueda difusa, cifrado y completados de shell](https://dev.to/opmr0/i-built-a-cli-snippet-manager-in-rust-because-i-was-tired-of-googling-the-same-things-4j4g)
* [FLODL v0.4.0: DDP multi-GPU heterogéneo con entrenamiento más rápido y mejor convergencia que una GPU solitaria](https://flodl.dev/blog/ddp-benchmark)

### Observaciones/Pensamientos
* [El e-gráfico acíclico: optimizador de gama media de Cranelift](https://cfallin.org/blog/2026/04/09/aegraph/)
* [Rust debería tener llamadas de cola estables](https://trifectatech.org/blog/tail-calls-project-goal/)
* [Los códigos de error planos no son suficientes](https://home.expurple.me/posts/flat-error-codes-are-not-enough/)
* [Nadie te debe seguridad en la cadena de suministro](https://purplesyringa.moe/blog/no-one-owes-you-supply-chain-security/)
* [Todo debe estar tipificado: los tipos escalares no son suficientes](https://sot.dev/everything-should-be-typed.html)
* [Sorpresas con préstamos](https://www.scattered-thoughts.net/writing/borrow-checking-surprises/)
* [Hoja de ruta para construir una biblioteca estándar ampliada para Rust](https://kerkour.com/rust-extended-standard-library)
* [Vale, ¿qué REALMENTE usa Rust?](https://blog.goose.love/posts/what-actually-uses-rust/)
* [audio] [Netstack.FM episodio 34 — Tokio con Carl Lerche (Ep 5 Remasterizado)](https://netstack.fm/#episode-34)

### Guías de Rust
* [Desentrañando Tokio y Rayon en producción: de picos de latencia de 2s a 94 ms fijos](https://posthog.com/blog/untangling-rayon-and-tokio)
* [Entendiendo Traceroute](https://tech.stonecharioteer.com/posts/2026/traceroute/)
* [Trayendo Rust a la banda base del píxel](https://security.googleblog.com/2026/04/bringing-rust-to-pixel-baseband.html)
* [Corrección de la latencia de cola DNS con una configuración de 5 líneas y una función de 50 líneas](https://numa.rs/blog/posts/fixing-doh-tail-latency.html)
* [Desinflar tu Rust asíncrono](https://tweedegolf.nl/en/blog/235/debloat-your-async-rust)
* [Aprende la propiedad y el préstamo de Rust construyendo Mini Grep](https://blog.sheerluck.dev/posts/learn-rust-ownership-by-building-mini-grep/)
* [Oxidación de perfilado: Un Flamegraph vs PGO, BOLT y Segmentación Nativa de CPU](https://alphakhaw.com/blog/seqpacker-profiling-rust-flamegraph-pgo-bolt)
* [Bulletproof Rust Web: Una guía opinativa sobre aplicaciones Axum de grado de producción](https://gruberb.github.io/bulletproof-rust-web/)
* [Un VMM mínimo en Rust con KVM](https://gigapotential.dev/blog/minimal-vmm-in-rust-with-kvm-hypervisor/)
* [claudectl: Creando un panel TUI para agentes de codificación de IA en Rust](https://mercurialsolo.github.io/posts/claudectl-tui-dashboard/)
* [vídeo] [Build con Naz: Elimina la espera ocupada con Rust Condvar](https://www.youtube.com/watch?v=HvCptpU5r_4)

## Crate de la semana

El crate de esta semana es [Myth Engine](https://github.com/panxinmiao/myth), un motor de renderizado multiplataforma de alto rendimiento.

¡Gracias a [Pan Xinmiao](https://users.rust-lang.org/t/crate-of-the-week/2704/1590) por la autosugerencia!

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

<!-- CFPs van aquí, usa este formato: * [nombre del proyecto - título del número](URL del número) -->
<!-- * [ - ]() -->
*Esta semana no se presentaron convocatorias para participar.*

Si eres propietario de un proyecto Rust y buscas colaboradores, por favor envia tareas [aquí][directrices] o a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado que busca un lugar para compartir algo interesante? Esta sección destaca eventos que se están organizando y que están aceptando propuestas para unirse a su evento como ponente.

<!-- los CFPs van aquí, usa este formato: * [**nombre del evento**](URL del CFP)| Fecha de cierre del CFP en AAAA-MM-DD | ciudad, estado, país | Fecha del evento en AAAA-MM-DD -->
<!-- o si no hay ninguno - *No se presentaron convocatorias ni presentaciones esta semana.* -->

* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP abierto hasta el 27-04-2026 | Barcelona, España | 2026-10-14 - 2026-10-17

Si eres un organizador de eventos que espera ampliar el alcance de tu evento, por favor envia un enlace a la web a través de un [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o contactando en [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizaciones del Proyecto Rust

519 pull requests fueron [fusionadas en la última semana][fusionadas]

[fusionados]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-07..2026-04-14

#### Compilador
* [¡Añadir '#! [unstable_removed(..)]'atributo a las características eliminadas de la pista](https://github.com/rust-lang/rust/pull/153335)
* [añadir sugerencia a '.to_owned()' usada en 'Vaca' al tomar prestado](https://github.com/rust-lang/rust/pull/154646)
* [evitar el desbordamiento de pila en FindExprBySpan](https://github.com/rust-lang/rust/pull/153888)
* [activar '#[diagnostic::on_const]' para impls locales](https://github.com/rust-lang/rust/pull/154609)
* [introduce un atributo '#[diagnostic::on_unknown]](https://github.com/rust-lang/rust/pull/152901)
* [reducir tamaño de 'ImportData'](https://github.com/rust-lang/rust/pull/155167)
* ['ty::Refactorización de Alias](https://github.com/rust-lang/rust/pull/154758)
* [comprobaciones semánticas de restricciones 'impl'](https://github.com/rust-lang/rust/pull/154661)
* [estabilizar registros vectoriales S390X](https://github.com/rust-lang/rust/pull/154184)
* [guardar 'chunk_domain_size' explícitamente en 'Chunk'](https://github.com/rust-lang/rust/pull/147802)

#### Biblioteca
* [añadir 'const Default' implica para 'LazyCell' y 'LazyLock'](https://github.com/rust-lang/rust/pull/154929)
* [constify algunos métodos 'Iterator'](https://github.com/rust-lang/rust/pull/154729)
* [constify DoubleEndedIterator](https://github.com/rust-lang/rust/pull/151898)
* [constify 'Step for NonZero<u*>'](https://github.com/rust-lang/rust/pull/154825)
* [No filtres temporales internos de 'DBG!'](https://github.com/rust-lang/rust/pull/154994)
* [olvida explícitamente los cero elementos restantes en 'vec::IntoIter::fold()'](https://github.com/rust-lang/rust/pull/148486)
* [impl const Residual para ControlFlow](https://github.com/rust-lang/rust/pull/155142)
* [funciones iniciales para iniciar en transmutar v2](https://github.com/rust-lang/rust/pull/155084)
* [introduce '#[diagnostic::on_move]' en 'Rc'](https://github.com/rust-lang/rust/pull/154678)
* [haz que 'Box/Rc/Arc::into_array' sea consciente del asignador (y añadir doctest)](https://github.com/rust-lang/rust/pull/154925)
* [función de estabilización 'int_lowest_highest_one'](https://github.com/rust-lang/rust/pull/155147)
* [función de estabilización 'isolate_most_least_significant_one'](https://github.com/rust-lang/rust/pull/155130)
* [función de estabilización 'uint_bit_width'](https://github.com/rust-lang/rust/pull/155131)

#### Carga
* [limpio: añadir validación de directorio destino](https://github.com/rust-lang/cargo/pull/16712)
* ['manifest': permitir dependencia de git junto con registro alternativo](https://github.com/rust-lang/cargo/pull/16810)
* ['auth': añadir pista de esquema de autenticación al error de token rechazado para registros alternativos](https://github.com/rust-lang/cargo/pull/16794)
* ['núcleo': usa 'closest_msg' para sugerir un nombre similar para un '-p'](https://github.com/rust-lang/cargo/pull/16844)
* ['lints': ignorar el estado 'unused_crate_dependencies'](https://github.com/rust-lang/cargo/pull/16877)
* ['toml': forzar advertencias de edición de guion en silencio](https://github.com/rust-lang/cargo/pull/16848)
* [copiar pruebas de validación de objetivo y dirección de carga limpia a 'clean_new_layout.rs'](https://github.com/rust-lang/cargo/pull/16878)
* [nunca incluir usar nombre de archivo extra en scripts de compilación](https://github.com/rust-lang/cargo/pull/16855)
* [objetivo de apoyo.'cfg(..)'. rustdocflags de forma análoga a rustflags](https://github.com/rust-lang/cargo/pull/16846)

#### Rustdoc
* [corregir tipos de patrón renderizado](https://github.com/rust-lang/rust/pull/154955)
* [dep-info para entradas de markdown independientes](https://github.com/rust-lang/rust/pull/154352)
* [heredar atributos en línea para macros declarativas](https://github.com/rust-lang/rust/pull/154902)

#### Clippy
* ['fn_to_numeric_cast_any': No avisar al lanzador al puntero crudo](https://github.com/rust-lang/rust-clippy/pull/14109)
* [aún más correcciones para el manejo de macros](https://github.com/rust-lang/rust-clippy/pull/16443)
* [extiende 'manual_filter' para cubrir 'and_then'](https://github.com/rust-lang/rust-clippy/pull/16456)
* [corregir 'unused_async' falso positivo para artículos incompletos con args](https://github.com/rust-lang/rust-clippy/pull/16832)
* [corregir sugerencia incorrecta para 'println_empty_string' con delimitadores que no sean paréntesis](https://github.com/rust-lang/rust-clippy/pull/16846)
* [truncar constantes al tipo objetivo en comparación](https://github.com/rust-lang/rust-clippy/pull/16782)

#### Analizador de Rust
* [los cambios en los scripts de compilación y en config.toml siempre deben actualizarse](https://github.com/rust-lang/rust-analyzer/pull/21969)
* [degradando la relevancia de la finalización cuando ya existe una implicación inherente](https://github.com/rust-lang/rust-analyzer/pull/22031)
* [mejoran marcadores de comandos ejecutables](https://github.com/rust-lang/rust-analyzer/pull/21978)
* [soportar restricciones 'impl' y 'mut'](https://github.com/rust-lang/rust-analyzer/pull/22022)
* [corregir '[env]' en '.cargo/config.toml' anulando variables del entorno del proceso](https://github.com/rust-lang/rust-analyzer/pull/21995)
* [Corregir el comando personalizado relativo de Rustfmt](https://github.com/rust-lang/rust-analyzer/pull/22010)
* [Evaluación MIR de tamaño &T con const recursivo fn](https://github.com/rust-lang/rust-analyzer/pull/22030)
* [comprueba coerción, no unificación, en "Rellenar campos 'struct'", como criterio para usar un local existente como valor del campo](https://github.com/rust-lang/rust-analyzer/pull/21971)
* [variantes completas de enums ocultos mediante alias públicos](https://github.com/rust-lang/rust-analyzer/pull/22003)
* [considera el contexto de la ruta para 'ImportAssets'](https://github.com/rust-lang/rust-analyzer/pull/21973)
* [diagnosticar caja con CFGS](https://github.com/rust-lang/rust-analyzer/pull/21981)
* [desactiva la corrección de campos perdidos cuando los campos son privados](https://github.com/rust-lang/rust-analyzer/pull/21977)
* [activar vscode suggest en cuerdas](https://github.com/rust-lang/rust-analyzer/pull/22018)
* [corregir la posición de 'ref_match' cuando se prefija palabra clave](https://github.com/rust-lang/rust-analyzer/pull/21999)
* [mejora añadir algo en la expresión tipo bloque](https://github.com/rust-lang/rust-analyzer/pull/21953)
* [mejora la etiqueta en 'add_missing_match_arms' asistencia](https://github.com/rust-lang/rust-analyzer/pull/21920)
* [no hay expresiones de término completo en el camino cualificado](https://github.com/rust-lang/rust-analyzer/pull/22009)
* [sin indexación de ref para 'extract_function'](https://github.com/rust-lang/rust-analyzer/pull/22025)
* [no importaciones en la ruta calificada por tipo ancla](https://github.com/rust-lang/rust-analyzer/pull/22012)
* [analiza 'cfg_attr' y 'cfg' especialmente](https://github.com/rust-lang/rust-analyzer/pull/21965)
* [también la mutabilidad del token en el flujo de edición](https://github.com/rust-lang/rust-analyzer/pull/21975)
* [migrar extraer 'struct' de la variante 'enum' al nuevo SyntaxEditor y portar heurísticas de espacios en blanco a SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21996)
* [reemplazar make from generar un solo campo 'struct' desde con SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/21997)
* [desenrollar el retorno de resultado innecesario en 'view_crate_graph'](https://github.com/rust-lang/rust-analyzer/pull/21992)

### Triaje de rendimiento del compilador Rust

Esta semana fue negativa, principalmente causada por una corrección de Type System y porque tuvimos que revertir temporalmente algunas limpiezas de atributos que antes mejoraban el rendimiento.

Triaje hecho por **@panstromek**.
Rango de revisión: [e73c56ab.. dab8d9d1](https://perf.rust-lang.org/?start=e73c56abd0baf3dbaafbdc3ce6072a416aade867&end=dab8d9d1066c4c95008163c7babf275106ce3f32&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u) | media | Rango | conde |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regresiones ❌ <br /> (primaria) | 0,4% | [0,2%, 0,7%] | 46 |
| Regresiones ❌ <br /> (secundario) | 0,5% | [0,1%, 2,3%] | 102 |
| Mejoras ✅ <br /> (primaria) | -0,5% | [-0,6%, -0,4%] | 4 |
| Mejoras ✅ <br /> (secundario) | -0,4% | [-0,6%, -0,2%] | 5 |
| Todos ❌✅ (primario) | 0,4% | [-0,6%, 0,7%] | 50 |

4 regresiones, 1 mejora, 5 mixtas; 6 de ellos en rollups
41 comparaciones de artefactos realizadas en total

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/2112fbf8fd6dac95c7447cd62d6c0c55c413ee67/triage/2026/2026-04-13.md)

### [RFCs aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de Rust [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
¿Son los RFC que fueron aprobados para su implementación esta semana?

* *No se aprobaron RFC esta semana.*

### Periodo final de comentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'periodo final de comentarios' para los RFCs y PRs clave
que están tomando una decisión. Expresa tus opiniones ahora.

#### Problemas de seguimiento y marcas personales
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Verificar que el penúltimo segmento de la ruta variante de enum se refiere a enum si tiene args](https://github.com/rust-lang/rust/pull/154971)
* [despreciar constantes y funciones 'std::char'](https://github.com/rust-lang/rust/pull/153873)
* ['impl Default' para 'RepeatN'](https://github.com/rust-lang/rust/pull/139690)
* [Hacer que std::fs::Archivo se envíe en UEFI](https://github.com/rust-lang/rust/pull/154003)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [feat(config): Estabilizar 'resolver.lockfile-path' config](https://github.com/rust-lang/cargo/pull/16694)

##### [Equipo de compiladores](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(solo MCPs)](https://forge.rust-lang.org/compiler/mcp.html)

* [Optimizar los enums 'repr(Rust)' omitiendo etiquetas en más casos que involucren variantes deshabitadas.](https://github.com/rust-lang/compiler-team/issues/922)
* [Propuesta para una suite de pruebas dedicada para el frontend paralelo](https://github.com/rust-lang/compiler-team/issues/906)
* [Promocionar objetivos ESP-IDF de nivel 3 riscv32 a nivel 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Propuesta para Adapt Stack Protector para Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [RFCs Rust](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Propone el fondo de mantenimiento de la Fundación Rust](https://github.com/rust-lang/rfcs/pull/3931)

##### [Consejo de Liderazgo](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Equipo de Financiación del Contenido (asignación 2026)](https://github.com/rust-lang/leadership-council/pull/279)

*Sin artículos inscritos en el Periodo de Comentarios Finales esta semana para
[Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) o
[Directrices del Código de Peligro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Háznos saber si desea que sus registros permanentes, problemas de seguimiento o RFCs sean registrados como parte de esta lista.

### [RFCs nuevos y actualizados](https://github.com/rust-lang/rfcs/pulls)

* *No se crearon RFC nuevos ni actualizados esta semana.*

## Próximos eventos

Eventos Rusty entre el 15-04-2026 y el 13-05-2026 🦀

### Virtual
* 2026-04-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/jia7wtfv)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-19 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Tercer domingo**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, EE. UU.) | [Oxidación DC](https://www.meetup.com/rustdc)
    * [**Rustful de mitad de mes**](https://www.meetup.com/rustdc/events/314007434/)
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Ámsterdam, NL) | [Desarrollo del juego Bevy](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin)
    * [**Hack and Learn Rust**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-24 | Virtual (Nairobi, KE) | [RustaceansKenya](http://luma.com/RustaceansKenya)
    * [**Transición a Rust: La curva de aprendizaje**](https://luma.com/f4qezpay)
* 2026-04-28 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (Londres, Reino Unido) | [Mujeres con Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: De Protobuf a Producción - Guía para gRPC en Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris)
    * [**Caminata de Hacker 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Encuentro del Círculo de Rust](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Encuentro de Rust Circle**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Primer domingo**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - con distanciamiento social**](https://www.meetup.com/indyrs/events/wqzhftyjchbjb/)
* 2026-05-07 | Virtual (Berlín, DE) | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Hackear y Aprender Oxidado**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Núremberg, DE) | [Núremberg de Oxid](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-12 | Virtual (Dallas, TX, EE. UU.) | [Encuentro de usuarios de Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (Londres, GB) | [Mujeres con Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 Comunidad poniéndose al día**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-13 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sesión semanal de programación**](https://luma.com/ooub1kt0)

### Asia
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Taller de Rust India**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Conferencia Rust India**](https://rustindia.org/schedule)
* 2026-05-13 | Malasia, MI | [Rust Meetup Malasia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup Malasia**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)

### Europa
* 2026-04-16 | Berlín, DE | [Berlín Oxidado](https://www.meetup.com/rust-berlin/events/)
    * [**Berlín Oxidado en localización 🏳️ 🌈 - Edición 013**](https://www.meetup.com/rust-berlin/events/314249809/)
* 2026-04-21 | Leipzig, DE | [Rust - Programación de sistemas modernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**GUI nativas con Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Noche de charla y fiesta de cumpleaños en MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-04-24 - 2026-04-26 | Augsburgo, DE | [Reunión de Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Semana del Futuro Augsburgo: Camino al Game Jam – Spielend Bevy und Rust lernen bei Tuxedo Computers**](https://www.tuxedocomputers.com/de/Road-to-Game-Jam-2026-Bevy-Workshop.tuxedo)
* 2026-04-25 | Estocolmo, SE | [Rust de Estocolmo](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fika de Ferris #26**](https://www.meetup.com/stockholm-rust/events/314227099/)
* 2026-04-29 | París, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/74289178383)
    * [**Rust Meetup en París**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1984135342220)
* 30-04-2026 | Manchester, GB | [Manchester Rust](https://www.meetup.com/rust-manchester/events/)
    * [**Charla de abril de Rust Manchester**](https://www.meetup.com/rust-manchester/events/314229892/)
* 2026-05-02 | Augsburgo, DE | [Rust Múnich](https://rust-munich.de/) y [Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Augsburger Linux-Infotag 2026: Gemeinschaftsstand Rust Augsburg und Rust München**](https://www.luga.de/static/LIT-2026/)
* 2026-05-04 | Ámsterdam, NH, NL | [Grupo Rust Developers Ámsterdam](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/314268909/)
* 2026-05-04 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Escribiendo una simulación de cartera de acciones en Rust con Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)
* 2026-05-05 | Olomouc, CZ | [Moravia Oxidada](https://www.meetup.com/rust-moravia/events/)
    * [**5. Rust Moravia Meetup (Ukaž testy!)**](https://www.meetup.com/rust-moravia/events/314218493/)

### Norteamérica
* 2026-04-15 | Híbrido (Vancouver, BC, CA) | [Rust de Vancouver](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Híbrido (Seattle, WA, EE. UU.) [Grupo de usuarios Seattle Rust](https://www.meetup.com/join-srug)
    * [**Abril de 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-16 | Mountain View, CA, EE. UU. | [Dojo Hacker](https://www.meetup.com/hackerdojo/events/)
    * [**RUST Meetup en HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313995065/)
* 2026-04-16 | Nashville, TN, EE. UU. | [Desarrolladores de Music City Rust](https://www.meetup.com/music-city-rust-developers)
    * [**Encuentro Comunitario**](https://www.meetup.com/music-city-rust-developers/events/314090462/)
* 2026-04-18 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust en la Plaza de Harvard, 18 de abril**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, EE. UU. [Grupo de Estudio sobre el Rust de San Francisco](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Hackeo de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, EE. UU. | [ATX de Rust](https://www.meetup.com/rust-atx)
    * [**Almuerzo Oxidado - Ahorro**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-22 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Solucionadores de Rust y SAT Verificados formalmente**](https://www.meetup.com/rust-nyc/events/314167944/)
* 2026-04-22 | Portland, OR | [**Apache DataFusion Meetup**](https://luma.com/dsp3ud82)
    * [**Encuentro de Portland Apache DataFusion**](https://luma.com/dsp3ud82)
* 2026-04-23 | Los Ángeles, CA, EE. UU. | [Rust Los Ángeles](https://www.meetup.com/rust-los-angeles)
    * [**¡Oxida LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, EE. UU. [Encuentro de Boston Rust](https://www.meetup.com/bostonrust)
    * [**Almuerzo de Rust de la Estación Sur, 25 de abril**](https://www.meetup.com/bostonrust/events/313883704/)
* 2026-04-28 | Nueva York, NY, EE. UU. [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC x OpenAI: Más seguro 'inseguro' & Barnum: El motor de flujo de trabajo agente.**](https://www.meetup.com/rust-nyc/events/314180711/)
* 30-04-2026 | Atlanta, GA, EE. UU. [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Atl de Rust**](https://www.meetup.com/rust-atl/events/311228662/)
* 2026-05-07 | Saint Louis, MO, EE. UU. [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Noche de Proyecto Abierto**](https://www.meetup.com/stl-rust/events/313807225/)

### Sudamérica
* 2026-04-17 | Río de Janeiro, BR | [Encuentros con Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Reunión Rust RJ**](https://luma.com/ce46pl7z)

Si organizas un evento de Rust, por favor añádelo al [calendario] para obtener
Lo menciona aquí. Por favor, recuerda añadir también un enlace al evento.
Envía un correo electrónico al [Rust Community Team][community] para acceder a la información.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Trabajos

Por favor, consulta el último [hilo de Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1rmra27/official_rrust_whos_hiring_thread_for_jobseekers/)

# Cita de la semana

> la cantidad de veces que paso 15 minutos en la documentación + programando, que acaban en un monstruoso 'o().flatten().map().is_ok_and)' solo para que Clippy me dé una bofetada diciendo 'reemplaza tu monstruo por esta única función, por favor' es demasiado alta 😀

– [Teufelchen en RIOT chat de matriz fuera de tema](https://matrix.to/#/!zcNfLwklXSZMQlTOLN:matrix.org/$1dc35m_KEs4r2vCu3DJ44NCjrrz3EtxWBQUdRQzlBe4)

¡Gracias a [chrysn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1760) por la sugerencia!

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

<small>[Debate en r/rust](https://www.reddit.com/r/rust/comments/1smskwm/this_week_in_rust_647/)</small>
