---
title: "Esta semana en Rust #43"
number_of_week: 43
description: Esta Semana en Rust: oportunidades laborales, equilibrio entre precisión y educación, y lo mejor de la comunidad.
date: 2025-01-08
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---

¡Hola y bienvenido a otra edición de *This Week in Rust*!  
[Rust](https://www.rust-lang.org/) es un lenguaje de programación que empodera a todos para crear software confiable y eficiente.  
Este es un resumen semanal de su progreso y de su comunidad.  
¿Quieres que mencionemos algo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o en [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) en mastodon.social, o [envíanos un pull request](https://github.com/rust-lang/this-week-in-rust).  
¿Quieres involucrarte? [Nos encantan las contribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).  

*This Week in Rust* se desarrolla abiertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y los archivos se pueden consultar en [this-week-in-rust.org](https://this-week-in-rust.org/).  
Si encuentras algún error en la edición de esta semana, [envíanos un PR](https://github.com/rust-lang/this-week-in-rust/pulls).  

¿Quieres recibir TWIR en tu correo? [Suscríbete aquí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).  

## Actualizaciones de la comunidad de Rust

### Boletines informativos
* [The Embedded Rustacean Issue #36](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-36)

### Actualizaciones de proyectos/herramientas
* [Informe mensual de diciembre de 2024 de rust-gcc](https://rust-gcc.github.io/2025/01/07/2024-12-monthly-report.html)
* [Aspectos destacados del lanzamiento 25.01 de Helix](https://helix-editor.com/news/release-25-01-highlights/)

### Observaciones/Pensamientos
* [Grandes cosas sobre Rust que no se limitan al rendimiento](https://ntietz.com/blog/great-things-about-rust-beyond-perf/)  
* [¡Rust en el espacio! Cómo Rust está impulsando simuladores de misiones espaciales de próxima generación](https://www.howtocodeit.com/articles/rust-in-space)  
* [Eficiencia de Bevy en dispositivos móviles](https://rustunit.com/blog/2025/01-02-bevy-mobile-framerate/)  
* [Por qué nextest utiliza un proceso por prueba](https://sunshowers.io/posts/nextest-process-per-test/)  
* [De Go a Rust Parte 1: despacho async](https://medium.com/rustaceans/from-go-to-rust-1-async-dispatch-866e042cd98a)  
* [Este mes en @compiler-errors (contribuciones a rustc)](https://hackmd.io/@compiler-errors/errs-december-2024)  
* [Construir seguridad funcional a velocidad con Rust](https://www.sonair.com/journal/building-for-safety-with-rust)  
* [No necesitas multihilo para hacer más de una cosa a la vez](https://sander.saares.eu/2024/12/31/you-do-not-need-multithreading-to-do-more-than-one-thing-at-a-time/)  
* [Una travesía por los protocolos de transferencia de archivos en Rust](https://blog.veeso.dev/blog/en/a-journey-into-file-transfer-protocols-in-rust/)  
* [Conversor de punto flotante a hexadecimal: ahora compatible con floats de 16 bits, ¡en Rust y WebAssembly!](https://gregstoll.wordpress.com/2025/01/08/floating-point-to-hex-converter-now-supports-16-bit-floats-plus-i-rewrote-it-in-rust-and-webassembly/)  

### Tutoriales de Rust
* [Domina la arquitectura hexagonal en Rust Parte IV: compensaciones de la arquitectura hexagonal](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#trade-offs-of-hexagonal-architecture-in-rust)  
* [La guía definitiva para el manejo de errores en Rust Parte III: manejo estructurado de errores](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling#structured-error-handling-in-rust)  

### Miscelánea
* [video] [Tue Henriksen sobre Rust en sistemas embebidos, conceptos erróneos y construir comunidad](https://www.youtube.com/watch?v=qt7ZLYnlBzk)  
* [Bevy en RustWeek 2025: ¡ven a programar con nosotros!](https://bevyengine.org/news/bevy-at-rust-week/)  
* [Mi intento fallido de AGI en el runtime de Tokio](https://www.christo.sh/building-agi-on-the-tokio-runtime/)  
* [El desafío de la calculadora JIT](https://ochagavia.nl/blog/the-jit-calculator-challenge/)  
* [Gameboy en tu terminal escrito en Rust](https://github.com/raphamorim/gameboy)  
* [video] [BlockMesh Network: un proyecto open source de pila completa en Rust con Ohad Dahan](https://www.youtube.com/watch?v=4J8jxLnWmGs)  

## Crate de la semana

El crate de esta semana es [terminal-colorsaurus](https://crates.io/crates/terminal-colorsaurus), una pequeña biblioteca para detectar si la terminal está en modo claro u oscuro.  

Gracias a [Tau](https://users.rust-lang.org/t/crate-of-the-week/2704/1386) por la auto-sugerencia.  

[¡Por favor envía tus sugerencias y votos para la próxima semana!][submit_crate]  

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704  

## Llamados a pruebas

Un paso importante para la implementación de RFCs es que las personas experimenten con la implementación y den su opinión, especialmente antes de la estabilización. Los siguientes RFCs se beneficiarían de pruebas de usuario antes de avanzar:  

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitieron llamados a pruebas esta semana.*  

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* [Issue de seguimiento para RFC 3695: Permitir literales booleanos como predicados cfg](https://github.com/rust-lang/rust/issues/131204)  
  - [Pasos para probar](https://github.com/rust-lang/rust/issues/131204#issuecomment-2569314526)  

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitieron llamados a pruebas esta semana.*  

Si eres un implementador de funciones y te gustaría que tu RFC aparezca en la lista anterior, agrega la nueva etiqueta `call-for-testing` a tu RFC junto con un comentario que proporcione instrucciones y/o orientación sobre qué aspecto(s) de la función necesitan ser probados.  

## Convocatoria de Participación: Proyectos y Ponentes

### CFP - Proyectos

¿Siempre has querido contribuir a proyectos de código abierto pero no sabías por dónde empezar?  
Cada semana destacamos algunas tareas de la comunidad de Rust para que te animes a participar y comiences tu camino en el open-source.

Algunas de estas tareas pueden contar con mentores disponibles; visita la página de la tarea para más información.

<!-- Las convocatorias van aquí, usa este formato: * [nombre del proyecto - título de la tarea](URL de la tarea) -->
<!-- * [ - ]() -->
<!-- o si no hay ninguna - *Esta semana no se enviaron convocatorias para participar.* -->

Si eres propietario de un proyecto en Rust y estás buscando colaboradores, por favor envía tareas [aquí][directrices] o a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o ponte en contacto en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust).

[directrices]: https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Eres un ponente nuevo o experimentado buscando un lugar para compartir algo interesante?  
En esta sección destacamos eventos que están en planificación y que aceptan propuestas para participar como ponente.

<!-- Las convocatorias van aquí, usa este formato: * [**nombre del evento**](URL a la convocatoria) | Fecha límite en YYYY-MM-DD | ciudad, estado, país | Fecha del evento en YYYY-MM-DD -->
<!-- o si no hay ninguna - *Esta semana no se enviaron convocatorias para ponencias o presentaciones.* -->

* [**Rust Week (Rust NL)**](https://www.papercall.io/rust-week) | Fecha límite: 2024-01-12 | Utrecht, NL | Evento: 2025-05-13

Si eres organizador de un evento y deseas ampliar su alcance, envía un enlace al sitio web a través de un [PR a TWiR](https://github.com/rust-lang/this-week-in-rust) o ponte en contacto en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust).

---

## Actualizaciones del Proyecto Rust

375 pull requests fueron [fusionados en la última semana][merged].

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-31..2025-01-07

* [add m68k-unknown-none-elf target](https://github.com/rust-lang/rust/pull/135085)
* [target: add mips mti baremetal support](https://github.com/rust-lang/rust/pull/135074)
* [A few borrowck tweaks to improve 2024 edition migration lints](https://github.com/rust-lang/rust/pull/135147)
* [E0277: suggest dereferencing function arguments in more cases](https://github.com/rust-lang/rust/pull/133292)
* [Debuginfo: Force `enum DISCR_*` to `static const u64` to allow for inspection via LLDB](https://github.com/rust-lang/rust/pull/133990)
* [`ObligationCause` construction tweaks in typeck](https://github.com/rust-lang/rust/pull/134984)
* [`generic_assert` Constify methods used by the formatting system](https://github.com/rust-lang/rust/pull/135139)
* [`cg_llvm`: Use constants for DWARF opcodes, instead of FFI calls](https://github.com/rust-lang/rust/pull/135115)
* [`rustc_intrinsic`: support functions without body](https://github.com/rust-lang/rust/pull/135031)
* [add a notion of "some ABIs require certain target features"](https://github.com/rust-lang/rust/pull/134794)
* [add suggestion for wrongly ordered format parameters](https://github.com/rust-lang/rust/pull/134877)
* [add support for wasm exception handling to Emscripten target](https://github.com/rust-lang/rust/pull/131830)
* [avoid use of LFS64 symbols on Emscripten](https://github.com/rust-lang/rust/pull/134080)
* [borrowck diagnostics: make `add_move_error_suggestions` use the HIR rather than `SourceMap`](https://github.com/rust-lang/rust/pull/133486)
* [const-in-pattern: test that the PartialEq impl does not need to be const](https://github.com/rust-lang/rust/pull/135064)
* [deny usage of special FileCheck prefixes as revision names](https://github.com/rust-lang/rust/pull/134925)
* [don't enable anyhow's `backtrace` feature in opt-dist](https://github.com/rust-lang/rust/pull/135146)
* [don't ice on bad transmute in typeck in new solver](https://github.com/rust-lang/rust/pull/134744)
* [fix ICE when opaque captures a duplicated/invalid lifetime](https://github.com/rust-lang/rust/pull/135000)
* [force code generation in assembly generation smoke-tests](https://github.com/rust-lang/rust/pull/135088)
* [improve diagnostics for `HostEffectPredicate` in the new solver](https://github.com/rust-lang/rust/pull/132345)
* [improve infer (`_`) suggestions in `const`s and static`s`](https://github.com/rust-lang/rust/pull/135044)
* [pass objcopy args for stripping on OSX](https://github.com/rust-lang/rust/pull/135034)
* [pass the arch rather than full target name to `windows_registry::find_tool`](https://github.com/rust-lang/rust/pull/133955)
* [project to `TyKind::Error` when there are unconstrained non-lifetime (ty/const) impl params](https://github.com/rust-lang/rust/pull/135057)
* [provide structured suggestion for `impl Default` of type where all fields have defaults](https://github.com/rust-lang/rust/pull/134979)
* [remove allowing `static_mut_refs` lint](https://github.com/rust-lang/rust/pull/131439)
* [remove range-metadata amdgpu workaround](https://github.com/rust-lang/rust/pull/135027)
* [report correct `SelectionError` for `ConstArgHasType` in new solver fulfill](https://github.com/rust-lang/rust/pull/134771)
* [report impl method has stricter requirements even when RPITIT inference gets in the way](https://github.com/rust-lang/rust/pull/135055)
* [some type-outlives computation tweaks](https://github.com/rust-lang/rust/pull/135007)
* [suggest to replace tuple constructor through projection](https://github.com/rust-lang/rust/pull/135090)
* [suppress host effect predicates if underlying trait doesn't hold](https://github.com/rust-lang/rust/pull/134951)
* [switch rtems target to panic unwind](https://github.com/rust-lang/rust/pull/133420)
* [taint fcx on selection errors during unsizing](https://github.com/rust-lang/rust/pull/135042)
* [turn `rustc_box` into an intrinsic](https://github.com/rust-lang/rust/pull/135046)
* [use `PostBorrowckAnalysis` in `check_coroutine_obligations`](https://github.com/rust-lang/rust/pull/134742)
* [miri: concurrency: generalize UnblockCallback to MachineCallback](https://github.com/rust-lang/miri/pull/4106)
* [library: fix adler{ → 2}.debug](https://github.com/rust-lang/rust/pull/135110)
* [mark `slice::reverse` unstably const](https://github.com/rust-lang/rust/pull/135121)
* [std: sync to dep versions of backtrace](https://github.com/rust-lang/rust/pull/135070)
* [try to write the panic message with a single `write_all` call](https://github.com/rust-lang/rust/pull/122565)
* [char `to_digit`: avoid unnecessary casts to u64](https://github.com/rust-lang/rust/pull/134969)
* [core: implement `bool::select_unpredictable`](https://github.com/rust-lang/rust/pull/133964)
* [do not in-place-iterate over flatmap/flatten](https://github.com/rust-lang/rust/pull/135104)
* [cargo: fix `https::self_signed_should_fail` for macos](https://github.com/rust-lang/cargo/pull/15016)
* [cargo: fix: env table config can't trigger rebuild with `rerun-if-env-changed`](https://github.com/rust-lang/cargo/pull/14756)
* [rustdoc: fix mismatched capitalization in sidebar](https://github.com/rust-lang/rust/pull/135116)
* [rustdoc: treat `allowed_through_unstable_modules` as deprecation](https://github.com/rust-lang/rust/pull/135043)
* [clippy: `clippy::redundant_locals` is not a correctness lint](https://github.com/rust-lang/rust-clippy/pull/13747)
* [clippy: `needless_continue`: lint if the last stmt in loop is `continue` recurisvely](https://github.com/rust-lang/rust-clippy/pull/13891)
* [clippy: add lint for calling `Iterator::last()` on `DoubleEndedIterator`](https://github.com/rust-lang/rust-clippy/pull/13922)
* [clippy: check if deref target implements `is_empty` for `len_zero` lint](https://github.com/rust-lang/rust-clippy/pull/13871)
* [clippy: do not trigger `missing_const_for_fn` for tests](https://github.com/rust-lang/rust-clippy/pull/13945)
* [clippy: improve `slow_vector_initialization` suggestion](https://github.com/rust-lang/rust-clippy/pull/13912)
* [clippy: only emit `useless_vec` suggestion if the macro does not contain code comments](https://github.com/rust-lang/rust-clippy/pull/13911)
* [rust-analyzer: allow targetDir to be an absolute path](https://github.com/rust-lang/rust-analyzer/pull/18822)
* [rust-analyzer: disable `rustc_test` metrics again](https://github.com/rust-lang/rust-analyzer/pull/18829)
* [rust-analyzer: allow excluding specific traits from completion](https://github.com/rust-lang/rust-analyzer/pull/18179)
* [rust-analyzer: support the new `CoercePointee` derive](https://github.com/rust-lang/rust-analyzer/pull/18821)
* [rust-analyzer: support updating snapshot tests with codelens/hovering/runnables](https://github.com/rust-lang/rust-analyzer/pull/18757)
* [rust-analyzer: fix case where completion inside macro that expands to `#[test]` was unavailable](https://github.com/rust-lang/rust-analyzer/pull/18853)
* [rust-analyzer: fix metrics workflow](https://github.com/rust-lang/rust-analyzer/pull/18831)
* [rust-analyzer: fix no space insert before and after if value is only spaces](https://github.com/rust-lang/rust-analyzer/pull/18820)
* [rust-analyzer: be more permissive with completion resolve data](https://github.com/rust-lang/rust-analyzer/pull/18836)
* [rust-analyzer: clear diagnostics on cancel unconditionally](https://github.com/rust-lang/rust-analyzer/pull/18858)
* [rust-analyzer: clear flycheck diagnostics per package properly](https://github.com/rust-lang/rust-analyzer/pull/18826)
* [rust-analyzer: deduplicate crate graph](https://github.com/rust-lang/rust-analyzer/pull/18806)
* [rust-analyzer: fix a bug that was caused by fixup reversing](https://github.com/rust-lang/rust-analyzer/pull/18852)
* [rust-analyzer: fix flycheck cancellations leaving stale errors](https://github.com/rust-lang/rust-analyzer/pull/18817)
* [rust-analyzer: fix flycheck getting confused which package to check](https://github.com/rust-lang/rust-analyzer/pull/18845)
* [rust-analyzer: fix non-cargo flychecks immediately clearing received diagnostics](https://github.com/rust-lang/rust-analyzer/pull/18848)
* [rust-analyzer: fix overflow detection in MIR evaluation](https://github.com/rust-lang/rust-analyzer/pull/18819)
* [rust-analyzer: fix relative .cargo env vars not working](https://github.com/rust-lang/rust-analyzer/pull/18841)
* [rust-analyzer: handle newstyle `rustc_intrinsic` safety correctly](https://github.com/rust-lang/rust-analyzer/pull/18843)
* [rust-analyzer: hide synthetic locals from completions](https://github.com/rust-lang/rust-analyzer/pull/18835)
* [rust-analyzer: store token trees in contiguous `Vec` instead of as a tree](https://github.com/rust-lang/rust-analyzer/pull/18327)

Explora más detalles sobre estos cambios en los enlaces proporcionados. 🚀

### Triage de Rendimiento del Compilador de Rust

Una semana tranquila sin muchos eventos. Hubo una pequeña regresión causada por una corrección de errores relacionada con traits, pero fue algo compensada por una actualización de cargo que trajo una ligera mejora en el rendimiento.

Triage realizado por **@kobzol**.  
Rango de revisiones: [93722f7e..0f1e965f](https://perf.rust-lang.org/?start=93722f7ed56bcf27839a6355074095c4320b7d37&end=0f1e965fec3bc2f97b932e9dd8e85fca6d7faadc&absolute=false&stat=instructions%3Au)

**Resumen**:

| (instrucciones:u)                 | media | rango           | cantidad |
|:----------------------------------:|:-----:|:---------------:|:--------:|
| Regresiones ❌ <br /> (primarias)  | 0.4%  | [0.1%, 1.1%]    | 20       |
| Regresiones ❌ <br /> (secundarias)| 0.4%  | [0.1%, 2.5%]    | 19       |
| Mejoras ✅ <br /> (primarias)      | -0.4% | [-1.6%, -0.2%]  | 8        |
| Mejoras ✅ <br /> (secundarias)    | -1.3% | [-1.7%, -0.2%]  | 13       |
| Total ❌✅ (primarias)             | 0.1%  | [-1.6%, 1.1%]   | 28       |

0 Regresiones, 2 Mejoras, 4 Mixtos; 4 de ellos en rollups  
Se realizaron 51 comparaciones de artefactos en total.

[Informe completo aquí](https://github.com/rust-lang/rustc-perf/blob/e5ffd3c575a14d4a84f0e797c5006948424a2192/triage/2025-01-07.md)

---

### [RFCs Aprobados](https://github.com/rust-lang/rfcs/commits/master)

Los cambios en Rust siguen el proceso de [RFC (solicitud de comentarios)](https://github.com/rust-lang/rfcs#rust-rfcs).  
Estos son los RFCs aprobados para implementación esta semana:

* *No se aprobaron RFCs esta semana.*

---

### Periodo de Comentario Final

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el "periodo de comentario final" para RFCs y PRs clave que están llegando a una decisión. Expresa tus opiniones ahora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)  
* [ABIs explícitos en extern](https://github.com/rust-lang/rfcs/pull/3722)

#### Seguimiento de problemas y PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Seguimiento del problema para `float_next_up_down`](https://github.com/rust-lang/rust/issues/91399)
* [`Impl TryFrom<Vec<u8>> for String`](https://github.com/rust-lang/rust/pull/132268)
* [Seguimiento del problema para la API de tuberías anónimas](https://github.com/rust-lang/rust/issues/127154)
* [Convertir `struct FromBytesWithNulError` en un enum](https://github.com/rust-lang/rust/pull/134143)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Simplificar SourceID Ord/Eq](https://github.com/rust-lang/cargo/pull/14980)

##### [Equipo de Lenguaje](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuesta del Equipo de Lenguaje entró en el período de comentarios finales esta semana.*

##### [Referencia del Lenguaje](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo RFCs de la referencia del lenguaje en período de comentarios finales esta semana.*

##### [Guías de Código Inseguro](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problemas o PRs de las guías de código inseguro en el período de comentarios finales esta semana.*

#### [Nuevos y actualizados RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No se crearon nuevos ni actualizados RFCs esta semana.*

## Próximos eventos

Eventos sobre Rust entre el 2025-01-08 y el 2025-02-05 🦀

### Virtuales
* 2025-01-08 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implementado en Rust con Ohad Dahan (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/304951805)
* 2025-01-09 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Creando intérpretes en Rust de forma colaborativa**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 2025-01-09 | Virtual (Miami, FL, US) | [Rust Miami](https://www.meetup.com/rust-miami/)
    * [**Rust / Wasm en Serverless y Frontend**](https://www.meetup.com/rust-miami/events/305122950)
* 2025-01-09 | Virtual (Núremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Núremberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 2025-01-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/302815031)
* 2025-01-15 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Introducción a WASM en Rust con Márk Tolmács (Virtual, Inglés)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)
* 2025-01-16 | Virtual (Berlín, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633278/)
* 2025-01-16 | Virtual y presencial (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**Encuentro de enero**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-21 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Explorando los enums en Rust con Yoni Peleg (Virtual, Hebreo)**](https://www.meetup.com/rust-tlv/events/305110744)
* 2025-01-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Rust en la mitad del mes**](https://www.meetup.com/rustdc/events/rdhhptyhccbcc)
* 2025-01-22 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Encuentro Bevy #8**](https://www.meetup.com/bevy-game-development/events/305111151)
* 2025-01-23 y 2025-01-24 | Virtual | [Taller Mainmatter Rust](https://ti.to/mainmatter/)
    * [**Taller remoto: Pruebas en proyectos Rust – más allá de lo básico**](https://ti.to/mainmatter/rust-testing-jan-2025)
* 2025-01-26 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Rust y programación embebida con Leon Vak (online, Hebreo)**](https://www.meetup.com/rust-tlv/events/304971264)
* 2025-01-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Último martes**](https://www.meetup.com/dallasrust/events/305361243)
* 2025-01-30 | Virtual (Berlín, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Espejo: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/299468340)
* 2025-01-30 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**¡Los computadores cuánticos no pueden oxidar esto!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 2025-01-30 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**¿Estamos embebidos aún? - Implementando un pequeño servidor HTTP en un microcontrolador**](https://www.meetup.com/code-mavens/events/305382647)
* 2025-02-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Grupo de usuarios Rust en Buffalo**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
 
### Asia
* 2025-01-12 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Encuentro presencial de Rust en enero de 2025 en Abra, Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europa
* 2025-01-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona - Enero 2025**](https://lu.ma/ckf2s00f)
* 2025-01-08 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust en enero: Cómo (no) estructurar tus proyectos en Rust**](https://www.meetup.com/rustcologne/events/305388321)
* 2025-01-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Encuentro de Rust en Reading**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 2025-01-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Hack'n'Learn de Rust en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)
* 2025-01-14 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar/events/)
    * [**Rust Your Engines #5**](https://www.meetup.com/hackschool-rhein-neckar/events/305230542)
* 2025-01-16 | Ámsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Encuentro en Avalor AI**](https://www.meetup.com/rust-amsterdam-group/events/305339712)
* 2025-01-16 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Encuentro de Rust Hack and Learn en Karlsruhe, en BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/305144321)
* 2025-01-18 | Estocolmo, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #8**](https://www.meetup.com/stockholm-rust/events/305475761)
* 2025-01-21 | Gante, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Charlas técnicas y cena: Proyectos en Rust: Leptos, Karyon, FunDSP**](https://www.meetup.com/systems-programming-ghent/events/305201540/?slug=systems-programming-ghent&eventId=305201540)
* 2025-01-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Redes peer-to-peer autoorganizadas con Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303604074)
* 2025-01-22 | Oberursel, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust: Edición 2024 y más allá**](https://www.meetup.com/rust-rhein-main/events/305330873)
* 2025-01-23 | Barcelona, ES | [Barcelona Free Software](https://www.meetup.com/barcelona-free-software/events/)
    * [**¿Por qué construir un nuevo motor de navegador en Rust?**](https://www.meetup.com/barcelona-free-software/events/305179554)
* 2025-01-23 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentro de Rust #74**](https://www.meetup.com/rust-paris/events/305455221)
* 2025-01-27 | Praga, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Encuentro de Rust en Praga (enero de 2025)**](https://www.meetup.com/rust-prague/events/305455153)
* 2025-01-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Noche de hackeo - Advent of Code**](https://www.meetup.com/rust-aarhus/events/304487851)
* 2025-01-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Desarrollo guiado por hipermedia en Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 2025-01-30 | Berlín, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299421383)
* 2025-02-01 | Bruselas, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**Devroom de Rust en FOSDEM**](https://fosdem.org/2025/schedule/track/rust/)
* 2025-02-01 | Núremberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Museo de Tecnología Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 2025-02-05 | Oxford, GB | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Encuentro social de Rust y C++ en Oxford**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)

### América del Norte
* 2025-01-08 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Cena de Rust - Pinthouse Pizza South Lamar**](https://www.meetup.com/rust-atx/events/305125929)
* 2025-01-09 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/hackerdojo/events/305044124)
* 2025-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo Rust en Lechmere, 10 de enero**](https://www.meetup.com/bostonrust/events/304951467)
* 2025-01-14 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Happy Hour de Rust**](https://www.meetup.com/chicago-rust-meetup/events/305460360)
* 2025-01-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Serie de desarrollo de juegos en Rust 1: Introducciones comunitarias**](https://www.meetup.com/music-city-rust-developers/events/304333017)
* 2025-01-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events)
    * [**Encuentro de enero**](https://www.meetup.com/join-srug/events/305505409/)
* 2025-01-17 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Multithreading y Async en Rust 101 - HolaMundo - Parte 3**](https://www.meetup.com/rust-mx/events/305464827)
* 2025-01-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Almuerzo Rust en Back Bay, 18 de enero**](https://www.meetup.com/bostonrust/events/304951470)
* 2025-01-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hackeo Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638258)
* 2025-01-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/305325657)
* 2025-01-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**Encuentro de Rust en Hacker Dojo**](https://www.meetup.com/hackerdojo/events/305414182)

Si organizas un evento sobre Rust, añádelo al [calendario] para que aparezca aquí. Recuerda incluir un enlace al evento. También puedes enviar un correo al [Equipo Comunitario de Rust][community] para obtener acceso.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com  
[community]: mailto:community-team@rust-lang.org  

## Empleos

<!--

Ofertas de empleo en Rust:

TWiR ha dejado de incluir publicaciones individuales de empleos. Puedes leer más sobre este cambio aquí:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Consulta el último [hilo "Who's Hiring" en r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/).

# Cita de la Semana

> Además, a menudo hay un equilibrio entre precisión y educación. Por ejemplo, cuando corrijo a mi hijo pequeño diciéndole que el Sol en realidad no se está moviendo, sino que nosotros estamos rotando. Eso es incorrecto: el Sol *sí* se mueve, pero probablemente es menos incorrecto que su impresión inicial. (Una vez intenté darle la explicación completa, pero a mitad de camino salió corriendo a jugar con sus trenes).
>
> No es que los lectores del libro de Rust sean niños pequeños, pero el principio se generaliza según mi experiencia.

– [Andrew Gallant, alias @BurntSushi, en rust-users](https://users.rust-lang.org/t/why-do-some-people-confound-t-with-stack-memory/123336/8)

¡Gracias a [Aleksander Krauze](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1648) por la sugerencia!

[¡Por favor envía citas y vota para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*Esta Semana en Rust está editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*El servicio de alojamiento de la lista de correo está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/).*

<small>[Discútelo en r/rust](https://www.reddit.com/r/rust/comments/1hx6z9b/this_week_in_rust_581/)</small>
