---
title: "Esta semana en Rust #47"
number_of_week: 47
description: L[Equipo de la comunidad de Rust][comunidad]aj[Equipo de la comunidad de Rust][comunidad]e esta semana es esp32-mender-client, u[Equipo de la comunidad de Rust][comunidad]liente para ESP32 para ejecutar actualizacione[Equipo de la comunidad de Rust][comunidad]e firmware y comandos remotos.
date: 2025-02-12
tags:
  - rust
  - comunidad
  - "esta semana en rust"
---


¡Hola y bienvenido[Equipo de la comunidad de Rust][comunidad] otr[Equipo de la comunidad de Rust][comunidad]úmer[Equipo de la comunidad de Rust][comunidad]e *This Week in Rust*!
[Rust](https://www.rust-lang.org/) e[Equipo de la comunidad de Rust][comunidad]n lenguaj[Equipo de la comunidad de Rust][comunidad]e programación que permit[Equipo de la comunidad de Rust][comunidad] todo e[Equipo de la comunidad de Rust][comunidad]und[Equipo de la comunidad de Rust][comunidad]rear software fiable y eficiente.
Este e[Equipo de la comunidad de Rust][comunidad]n resumen semana[Equipo de la comunidad de Rust][comunidad]e su progreso y comunidad.
¿Quieres que s[Equipo de la comunidad de Rust][comunidad]encion[Equipo de la comunidad de Rust][comunidad]lgo? Etiquétanos en [@ThisWeekInRust](https://x.com/ThisWeekInRust) en X (antes Twitter) o [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) e[Equipo de la comunidad de Rust][comunidad]astodon.social, o [envíano[Equipo de la comunidad de Rust][comunidad]na solicitu[Equipo de la comunidad de Rust][comunidad]e extracción](https://github.com/rust-lang/this-week-in-rust).
¿Quieres participar? [Nos encantan la[Equipo de la comunidad de Rust][comunidad]ontribuciones](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* s[Equipo de la comunidad de Rust][comunidad]esarroll[Equipo de la comunidad de Rust][comunidad]biertamente [en GitHub](https://github.com/rust-lang/this-week-in-rust) y lo[Equipo de la comunidad de Rust][comunidad]rchivos se pueden ver en [this-week-in-rust.org](https://this-week-in-rust.org/).
Si encuentr[Equipo de la comunidad de Rust][comunidad]lgún error en la edició[Equipo de la comunidad de Rust][comunidad]e esta semana, [por favor enví[Equipo de la comunidad de Rust][comunidad]n PR](https://github.com/rust-lang/this-week-in-rust/pulls).

¿Quieres tener TWIR en tu bandej[Equipo de la comunidad de Rust][comunidad]e entrada? [Suscríbet[Equipo de la comunidad de Rust][comunidad]quí](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Actualizacione[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]omunida[Equipo de la comunidad de Rust][comunidad]e Rust 🥰

<!-- Estimado[Equipo de la comunidad de Rust][comunidad]olaboradore[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]omunidad: Por favor, lea README.md par[Equipo de la comunidad de Rust][comunidad]btener orientación sobre las presentaciones. Cada enlace enviad[Equipo de la comunidad de Rust][comunidad]ebe tener la forma: * [Títul[Equipo de la comunidad de Rust][comunidad]e la página enlazada](https://example.com/my_article) S[Equipo de la comunidad de Rust][comunidad]o sabes qué categorí[Equipo de la comunidad de Rust][comunidad]sar, siéntete libr[Equipo de la comunidad de Rust][comunidad]e enviar un P[Equipo de la comunidad de Rust][comunidad]e todo[Equipo de la comunidad de Rust][comunidad]odos y simplemente pid[Equipo de la comunidad de Rust][comunidad] los editores que seleccionen l[Equipo de la comunidad de Rust][comunidad]ategoría. -->

### Oficial
* [crates.io: actualizació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]esarrollo](https://blog.rust-lang.org/2025/02/05/crates-io-development-update.html)

### Fundación
* [Nell Shamrell-Harrington elegid[Equipo de la comunidad de Rust][comunidad]omo President[Equipo de la comunidad de Rust][comunidad]e la Junta Directiv[Equipo de la comunidad de Rust][comunidad]e la Fundación Rust](https://rustfoundation.org/media/nell-shamrell-harrington-elected-as-rust-foundation-board-of-directors-chair/)

### Boletines
* [Est[Equipo de la comunidad de Rust][comunidad]es en Rust OSDev: ener[Equipo de la comunidad de Rust][comunidad]e 2025](https://rust-osdev.com/this-month/2025-01/)

### Actualizacione[Equipo de la comunidad de Rust][comunidad]e proyectos/herramientas
* [Motor de juego Fyrox 0.36](https://fyrox.rs/blog/post/fyrox-game-engine-0-36/)
* [Refinado: tipo[Equipo de la comunidad de Rust][comunidad]e refinamiento simples para Rust](https://jordankaye.dev/posts/refined/)
* [Opinione[Equipo de la comunidad de Rust][comunidad]e lo[Equipo de la comunidad de Rust][comunidad]antenedores sobre Rust-for-Linux](https://lwn.net/SubscriberLink/1007921/9020dbb12585d48f/)
* [Reescribiendo paquetes esenciale[Equipo de la comunidad de Rust][comunidad]e Linux en Rust](https://lwn.net/SubscriberLink/1007907/a9db87fc233bceae/)
* [Nutype 0.6.0 - newtyp[Equipo de la comunidad de Rust][comunidad]on garantías soporta funcione[Equipo de la comunidad de Rust][comunidad]ons[Equipo de la comunidad de Rust][comunidad]hora](https://github.com/greyblake/nutype/releases/tag/v0.6.0)
* [Glues v0.6.0 – Aplicación para tomar nota[Equipo de la comunidad de Rust][comunidad]e TUI co[Equipo de la comunidad de Rust][comunidad]n[Equipo de la comunidad de Rust][comunidad]mportant[Equipo de la comunidad de Rust][comunidad]ctualizació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]iseño](https://github.com/gluesql/glues/releases/tag/v0.6.0)
* [Lanzamient[Equipo de la comunidad de Rust][comunidad]e Gladius 0.2 - un editor d[Equipo de la comunidad de Rust][comunidad]ódigo GPLv3, líne[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omandos, multicursor con soporte LSP](https://codeberg.org/njskalski/bernardo/src/branch/master/docs/beta_release_notes/beta_1_release.md)

### Observaciones/Pensamientos
* [Un estudi[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ada variant[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]terador](https://blog.yoshuawuyts.com/a-survey-of-every-iterator-variant)
* [Actualizació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]na bas[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ódigo grand[Equipo de la comunidad de Rust][comunidad] Rust 2024](https://codeandbitters.com/rust-2024-upgrade/)
* [Cóm[Equipo de la comunidad de Rust][comunidad]prendí [Equipo de la comunidad de Rust][comunidad]ejar de preocuparme y amar el LLM](https://smallcultfollowing.com/babysteps/blog/2025/02/10/love-the-llm/)
* [Las Tres Reglas Básica[Equipo de la comunidad de Rust][comunidad]e Seguridad, Higiene](https://jack.wrenn.fyi/blog/safety-hygiene/)
* [video] [Bevy Meetup#1 - Alice - ¿Qué diablos es la reflexión?](https://www.youtube.com/watch?v=vxPKWb0dSqQ)
* [video] [E[Equipo de la comunidad de Rust][comunidad]as[Equipo de la comunidad de Rust][comunidad]el sans-io](https://www.youtube.com/watch?v=RYHYiXMJdZI)

### Tutoriale[Equipo de la comunidad de Rust][comunidad]e Rust
* [Primeros pasos en e[Equipo de la comunidad de Rust][comunidad]esarroll[Equipo de la comunidad de Rust][comunidad]e juego[Equipo de la comunidad de Rust][comunidad]on Rus[Equipo de la comunidad de Rust][comunidad]nd Bevy](https://blog.jetbrains.com/rust/2025/02/04/first-steps-in-game-development-with-rust-and-bevy/)
* [Aprovisionamient[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ertificados TLS en Rus[Equipo de la comunidad de Rust][comunidad]on ACME](https://www.shuttle.dev/blog/2025/02/06/provisioning-tls-certificates-with-acme-in-rust)
* [Consej[Equipo de la comunidad de Rust][comunidad]e[Equipo de la comunidad de Rust][comunidad]ía #4: Escrib[Equipo de la comunidad de Rust][comunidad]notaciones en los patrone[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]oincidenci[Equipo de la comunidad de Rust][comunidad]e Rust](https://gaultier.github.io/blog/tip_of_the_day_4.html)
* [De hora[Equipo de la comunidad de Rust][comunidad] 360 ms: sobre-ingenierí[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]na solució[Equipo de la comunidad de Rust][comunidad]e rompecabezas](https://blog.danielh.cc/blog/puzzle)
* [Escribir u[Equipo de la comunidad de Rust][comunidad]ontrolador simple en Rust](https://scorpiosoftware.net/2025/02/08/writing-a-simple-driver-in-rust/)
* [Resolviendo el problem[Equipo de la comunidad de Rust][comunidad]e ABA en Rus[Equipo de la comunidad de Rust][comunidad]on punteros etiquetados](https://minikin.me/blog/solving-the-aba-problem-in-rust-tagged-pointers)
* [Mezcla Rus[Equipo de la comunidad de Rust][comunidad]on Java (¡o Kotlin!)](https://tweedegolf.nl/en/blog/147/mix-in-rust-with-java-or-kotlin)
* [Cómo escribir código DRY en Rust](https://baarse.substack.com/p/how-to-write-dry-code-in-rust)

### FOSDEM
* [Rust para Linux](https://fosdem.org/2025/schedule/event/fosdem-2025-6507-rust-for-linux/)
* [¿Reescribiendo el futur[Equipo de la comunidad de Rust][comunidad]e los paquetes esenciale[Equipo de la comunidad de Rust][comunidad]e Linux en Rust?](https://fosdem.org/2025/schedule/event/fosdem-2025-6196-rewriting-the-future-of-the-linux-essential-packages-in-rust-/)
* [Escribiend[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ontrolador de Kubernetes... Pero en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4189-writing-a-kubernetes-controller-but-in-rust/)
* [Us[Equipo de la comunidad de Rust][comunidad]e Rus[Equipo de la comunidad de Rust][comunidad]ntegrado par[Equipo de la comunidad de Rust][comunidad]onstruir u[Equipo de la comunidad de Rust][comunidad]ispositiv[Equipo de la comunidad de Rust][comunidad]esatendido que funcion[Equipo de la comunidad de Rust][comunidad]on baterías](https://fosdem.org/2025/schedule/event/fosdem-2025-6300-using-embedded-rust-to-build-an-unattended-battery-powered-device/)
* [Augures: u[Equipo de la comunidad de Rust][comunidad]onjunt[Equipo de la comunidad de Rust][comunidad]e herramienta[Equipo de la comunidad de Rust][comunidad]e series temporales para Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4668-augurs-a-time-series-toolkit-for-rust/)
* [Construcció[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n vatímetro y un backen[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ohetes](https://fosdem.org/2025/schedule/event/fosdem-2025-5470-building-a-watt-meter-esp-rs-and-a-rocket-backend/)
* [Enorm[Equipo de la comunidad de Rust][comunidad]nálisi[Equipo de la comunidad de Rust][comunidad]e gráficos en su propio servidor con WebGraph en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-4773-huge-graph-analysis-on-your-own-server-with-webgraph-in-rust/)
* [Llevando la estétic[Equipo de la comunidad de Rust][comunidad]e los terminale[Equipo de la comunidad de Rust][comunidad] la Web con Rust (y viceversa)](https://fosdem.org/2025/schedule/event/fosdem-2025-5496-bringing-terminal-aesthetics-to-the-web-with-rust-and-vice-versa-/)
* [Abusar de los préstamos par[Equipo de la comunidad de Rust][comunidad]ivertirse, obtener ganancias y un recolector de basur[Equipo de la comunidad de Rust][comunidad]e punto seguro](https://fosdem.org/2025/schedule/event/fosdem-2025-4394-abusing-reborrowing-for-fun-profit-and-a-safepoint-garbage-collector/)
* [Leccione[Equipo de la comunidad de Rust][comunidad]e la reescritur[Equipo de la comunidad de Rust][comunidad]e softwar[Equipo de la comunidad de Rust][comunidad]e sistemas en Rust](https://fosdem.org/2025/schedule/event/fosdem-2025-5088-lessons-from-rewriting-systems-software-in-rust/)
* [Oxidació[Equipo de la comunidad de Rust][comunidad]el programador del kerne[Equipo de la comunidad de Rust][comunidad]e Linux (en el espaci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario)](https://fosdem.org/2025/schedule/event/fosdem-2025-4620-rust-ifying-the-linux-kernel-scheduler-in-user-space-/)
* [Aventuras en l[Equipo de la comunidad de Rust][comunidad]xidació[Equipo de la comunidad de Rust][comunidad]e la gestió[Equipo de la comunidad de Rust][comunidad]e paquete[Equipo de la comunidad de Rust][comunidad]e Arch Linux](https://fosdem.org/2025/schedule/event/fosdem-2025-6259-adventures-in-oxidizing-arch-linux-package-management/)
* [Modelando la herencia en SeaORM](https://www.sea-ql.org/blog/2025-01-08-sea-orm-inheritance/)

### Miscelánea
* [Inform[Equipo de la comunidad de Rust][comunidad]e empleo[Equipo de la comunidad de Rust][comunidad]e Rus[Equipo de la comunidad de Rust][comunidad]e ener[Equipo de la comunidad de Rust][comunidad]e 2025](https://filtra.io/rust/jobs-report/jan-25)

## Caj[Equipo de la comunidad de Rust][comunidad]e la semana

L[Equipo de la comunidad de Rust][comunidad]aj[Equipo de la comunidad de Rust][comunidad]e esta semana es [esp32-mender-client](https://github.com/virust-ai/esp32-mender-client), u[Equipo de la comunidad de Rust][comunidad]liente para ESP32 para ejecutar actualizacione[Equipo de la comunidad de Rust][comunidad]e firmware y comandos remotos.

¡Gracia[Equipo de la comunidad de Rust][comunidad] [Kelvin](https://users.rust-lang.org/t/crate-of-the-week/2704/1399) por l[Equipo de la comunidad de Rust][comunidad]utosugerencia!

[Por favor, envíen sus sugerencias y votos para la próxima semana][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Llamada[Equipo de la comunidad de Rust][comunidad] la realizació[Equipo de la comunidad de Rust][comunidad]e pruebas
Un pas[Equipo de la comunidad de Rust][comunidad]mportante para l[Equipo de la comunidad de Rust][comunidad]mplementació[Equipo de la comunidad de Rust][comunidad]e RFC es que las personas experimente[Equipo de la comunidad de Rust][comunidad]on el
implementación y dar retroalimentación, especialment[Equipo de la comunidad de Rust][comunidad]nte[Equipo de la comunidad de Rust][comunidad]e la estabilización.  Lo siguiente
Las RFC se beneficiaría[Equipo de la comunidad de Rust][comunidad]e las prueba[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suari[Equipo de la comunidad de Rust][comunidad]nte[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]vanzar:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No se emitiero[Equipo de la comunidad de Rust][comunidad]onvocatorias para pruebas esta semana.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No se emitiero[Equipo de la comunidad de Rust][comunidad]onvocatorias para pruebas esta semana.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No se emitiero[Equipo de la comunidad de Rust][comunidad]onvocatorias para pruebas esta semana.*

Si e[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]mplementador de funciones y desea que su RFC aparezca en la list[Equipo de la comunidad de Rust][comunidad]nterior, agregue l[Equipo de la comunidad de Rust][comunidad]ueva 'llamada para pruebas'
a su RFC junt[Equipo de la comunidad de Rust][comunidad]o[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentario que proporcion[Equipo de la comunidad de Rust][comunidad]nstruccione[Equipo de la comunidad de Rust][comunidad]e prueba y/[Equipo de la comunidad de Rust][comunidad]rientación sobre qué aspecto(s) de la función
necesitan pruebas.

## Llamad[Equipo de la comunidad de Rust][comunidad] la participación; Proyectos y ponentes

### CFP - Proyectos

¿Siempre quisist[Equipo de la comunidad de Rust][comunidad]ontribuir a proyecto[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ódig[Equipo de la comunidad de Rust][comunidad]bierto per[Equipo de la comunidad de Rust][comunidad]o sabías por dónde empezar?
¡Cada seman[Equipo de la comunidad de Rust][comunidad]estacamo[Equipo de la comunidad de Rust][comunidad]lgunas tarea[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]omunida[Equipo de la comunidad de Rust][comunidad]e Rust para que elijas y comiences!

Alguna[Equipo de la comunidad de Rust][comunidad]e estas tareas también pueden tener mentore[Equipo de la comunidad de Rust][comunidad]isponibles, visite la págin[Equipo de la comunidad de Rust][comunidad]e tareas par[Equipo de la comunidad de Rust][comunidad]btener má[Equipo de la comunidad de Rust][comunidad]nformación.

*Esta seman[Equipo de la comunidad de Rust][comunidad]o se han presentad[Equipo de la comunidad de Rust][comunidad]onvocatorias para participar.*

Si eres propietari[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n proyect[Equipo de la comunidad de Rust][comunidad]e Rust y estás buscand[Equipo de la comunidad de Rust][comunidad]olaboradores, por favor envía tareas [aquí][directrices] [Equipo de la comunidad de Rust][comunidad] travé[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) o poniéndote e[Equipo de la comunidad de Rust][comunidad]ontact[Equipo de la comunidad de Rust][comunidad]on [X (antes Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

[directrices]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Eventos

¿Ere[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]rador nuev[Equipo de la comunidad de Rust][comunidad] experimentado que busc[Equipo de la comunidad de Rust][comunidad]n lugar par[Equipo de la comunidad de Rust][comunidad]ompartir algo genial? Esta secció[Equipo de la comunidad de Rust][comunidad]estaca los eventos que se están planificando y que está[Equipo de la comunidad de Rust][comunidad]ceptando presentaciones par[Equipo de la comunidad de Rust][comunidad]nirs[Equipo de la comunidad de Rust][comunidad] su event[Equipo de la comunidad de Rust][comunidad]om[Equipo de la comunidad de Rust][comunidad]rador.

*No se han presentad[Equipo de la comunidad de Rust][comunidad]onvocatoria[Equipo de la comunidad de Rust][comunidad]i presentaciones esta semana.*

S[Equipo de la comunidad de Rust][comunidad]sted e[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]rganizador de eventos que espera expandir e[Equipo de la comunidad de Rust][comunidad]lcanc[Equipo de la comunidad de Rust][comunidad]e su evento, enví[Equipo de la comunidad de Rust][comunidad]n enlac[Equipo de la comunidad de Rust][comunidad]l sitio web a travé[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) [Equipo de la comunidad de Rust][comunidad]omunicándose en [X (anteriormente Twitter)](https://x.com/ThisWeekInRust) o [Mastodon](https://mastodon.social/@thisweekinrust)!

## Actualizacione[Equipo de la comunidad de Rust][comunidad]el Proyecto Rust

Se [fusionaron 462 solicitude[Equipo de la comunidad de Rust][comunidad]e extracción en la última semana][fusionadas]

[fusionado]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-02-04..2025-02-11

* [agregar destin[Equipo de la comunidad de Rust][comunidad]mdgpu](https://github.com/rust-lang/rust/pull/134740)
* ['#[contratos::requiere(...)]' + '#[contratos::asegura(...)]«](https://github.com/rust-lang/rust/pull/128045)
* ['rustc_middle': parallel: TyCtxt: eliminar "imp[Equipo de la comunidad de Rust][comunidad]nseguro DynSend/DynSync"](https://github.com/rust-lang/rust/pull/136731)
* [añadir nota sobre el rasgo 'FnPtr' que se expon[Equipo de la comunidad de Rust][comunidad]omo enlace público](https://github.com/rust-lang/rust/pull/136518)
* [permitir e[Equipo de la comunidad de Rust][comunidad]s[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]onst[Equipo de la comunidad de Rust][comunidad]o[Equipo de la comunidad de Rust][comunidad]ombre en tipo[Equipo de la comunidad de Rust][comunidad]e patrones](https://github.com/rust-lang/rust/pull/136284)
* [Calcule siempre e[Equipo de la comunidad de Rust][comunidad]iseñ[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]orrutinas para emitir errore[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]iseño recursivo[Equipo de la comunidad de Rust][comunidad]on entusiasmo](https://github.com/rust-lang/rust/pull/136073)
* [evite llamar a l[Equipo de la comunidad de Rust][comunidad]onsulta 'layout_of' en 'lit_to_const'](https://github.com/rust-lang/rust/pull/136302)
* [evit[Equipo de la comunidad de Rust][comunidad]sar 'make_direct_deprecated()' en extern "ptx-kernel"](https://github.com/rust-lang/rust/pull/133932)
* [comprobar el tamañ[Equipo de la comunidad de Rust][comunidad]el tip[Equipo de la comunidad de Rust][comunidad]e retorno en WF](https://github.com/rust-lang/rust/pull/136274)
* [compilador: puerta 'extern "{abi}"' en 'ast_lowering'](https://github.com/rust-lang/rust/pull/136603)
* [Un par d[Equipo de la comunidad de Rust][comunidad]ambios para ejecutar rustc en Miri](https://github.com/rust-lang/rust/pull/136580)
* [Cobertura: aplazar part[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]ontra-creación hasta CodeGen](https://github.com/rust-lang/rust/pull/136053)
* [debuginfo para la función ZST[Equipo de la comunidad de Rust][comunidad]ebe tener un[Equipo de la comunidad de Rust][comunidad]lineació[Equipo de la comunidad de Rust][comunidad]e 8 bits, n[Equipo de la comunidad de Rust][comunidad]e 1 bit](https://github.com/rust-lang/rust/pull/136640)
* [detectar préstamos (no sin procesar) de punteros ZST nulos en CheckNull](https://github.com/rust-lang/rust/pull/136601)
* [no permitir 'repr()' en elemento[Equipo de la comunidad de Rust][comunidad]o válidos](https://github.com/rust-lang/rust/pull/133925)
* [visualizació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]úmeros enteros sin punteros sin formato y sin 'overflowing_literals'](https://github.com/rust-lang/rust/pull/135265)
* [no reiniciar el tip[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]onversión si[Equipo de la comunidad de Rust][comunidad]ctualizar también e[Equipo de la comunidad de Rust][comunidad]perando en 'simplify_cast' en GVN](https://github.com/rust-lang/rust/pull/136450)
* [emitir un error si se solicita '-Zdwarf-version=1'](https://github.com/rust-lang/rust/pull/136746)
* [asegúres[Equipo de la comunidad de Rust][comunidad]e qu[Equipo de la comunidad de Rust][comunidad]unc[Equipo de la comunidad de Rust][comunidad]ntentamo[Equipo de la comunidad de Rust][comunidad]onomorfizar las llamada[Equipo de la comunidad de Rust][comunidad]pcasting o vtabl[Equipo de la comunidad de Rust][comunidad]e tipo[Equipo de la comunidad de Rust][comunidad]y[Equipo de la comunidad de Rust][comunidad]mposibles](https://github.com/rust-lang/rust/pull/136311)
* [arreglar 'rustc_hidden_type_of_opaques' para RPITITs si[Equipo de la comunidad de Rust][comunidad]uerpo predeterminado](https://github.com/rust-lang/rust/pull/136550)
* [arreglar la pelusa 'unreachable_pub' para e[Equipo de la comunidad de Rust][comunidad]bjetivo ermitaño](https://github.com/rust-lang/rust/pull/136595)
* [arreglar accidentalmente que y[Equipo de la comunidad de Rust][comunidad]o emitía lints literale[Equipo de la comunidad de Rust][comunidad]esbordados en patrones](https://github.com/rust-lang/rust/pull/136393)
* [Sugerenci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]orrección para 'dependency_on_unit_never_type_fallback' qu[Equipo de la comunidad de Rust][comunidad]nvolucr[Equipo de la comunidad de Rust][comunidad]ierres + expansione[Equipo de la comunidad de Rust][comunidad]e format[Equipo de la comunidad de Rust][comunidad]rgs](https://github.com/rust-lang/rust/pull/136598)
* [arreglar la[Equipo de la comunidad de Rust][comunidad]omprobacione[Equipo de la comunidad de Rust][comunidad]e llamada[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ola wrt '#[track_caller]'](https://github.com/rust-lang/rust/pull/135973)
* [Corregir error d[Equipo de la comunidad de Rust][comunidad]esenvoltura en el litera[Equipo de la comunidad de Rust][comunidad]n[Equipo de la comunidad de Rust][comunidad]esbordado](https://github.com/rust-lang/rust/pull/136760)
* [fucsia: permitir que Rus[Equipo de la comunidad de Rust][comunidad]s[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]úmer[Equipo de la comunidad de Rust][comunidad]e llamada[Equipo de la comunidad de Rust][comunidad]l sistem[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]rchivos libc](https://github.com/rust-lang/rust/pull/136213)
* [generar el bloqu[Equipo de la comunidad de Rust][comunidad]e terminació[Equipo de la comunidad de Rust][comunidad]orrecto bajo Wasm EH](https://github.com/rust-lang/rust/pull/136200)
* [introducir CoercePointeeValidated par[Equipo de la comunidad de Rust][comunidad]omprobacione[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]oherencia en la etap[Equipo de la comunidad de Rust][comunidad]e tipeck](https://github.com/rust-lang/rust/pull/136107)
* [etiquetar parámetro[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]oincidentes en el siti[Equipo de la comunidad de Rust][comunidad]ef para funciones foráneas](https://github.com/rust-lang/rust/pull/136651)
* [hacer 'AsyncFnOnce', 'AsyncFnMut', 'AsyncFn' no-'#[fundamental]'](https://github.com/rust-lang/rust/pull/136724)
* [hacer de 'cenum_impl_drop_cast' un error grave](https://github.com/rust-lang/rust/pull/135964)
* [asegúres[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]sar el rasgo 'Receptor' al extraer e[Equipo de la comunidad de Rust][comunidad]andidat[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]étod[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]bjeto](https://github.com/rust-lang/rust/pull/135179)
* [solo resalte los parámetro[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]oincidentes en el siti[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]efinición](https://github.com/rust-lang/rust/pull/136583)
* [el pas[Equipo de la comunidad de Rust][comunidad]barc[Equipo de la comunidad de Rust][comunidad]lrededor de[Equipo de la comunidad de Rust][comunidad]uevo solucionador](https://github.com/rust-lang/rust/pull/136269)
* [patrón Migración 2024: intente sugerir la eliminació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]odificadore[Equipo de la comunidad de Rust][comunidad]e enlace redundantes](https://github.com/rust-lang/rust/pull/136577)
* [elija la versió[Equipo de la comunidad de Rust][comunidad]áxim[Equipo de la comunidad de Rust][comunidad]e DWARF cuando s[Equipo de la comunidad de Rust][comunidad]tilice[Equipo de la comunidad de Rust][comunidad]ódulos LTO co[Equipo de la comunidad de Rust][comunidad]iferentes versiones](https://github.com/rust-lang/rust/pull/136659)
* [Escríbeme par[Equipo de la comunidad de Rust][comunidad]ambios relacionado[Equipo de la comunidad de Rust][comunidad]o[Equipo de la comunidad de Rust][comunidad]tributos](https://github.com/rust-lang/rust/pull/136643)
* [rechazar literale[Equipo de la comunidad de Rust][comunidad]egativos para tipos sin sign[Equipo de la comunidad de Rust][comunidad] char en rango[Equipo de la comunidad de Rust][comunidad]e patrones y literales](https://github.com/rust-lang/rust/pull/136304)
* [Se eliminó l[Equipo de la comunidad de Rust][comunidad]ependenci[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]aj[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]esplazamient[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ampo, enfoqu[Equipo de la comunidad de Rust][comunidad]lternativo](https://github.com/rust-lang/rust/pull/136201)
* [Reportar desajustes genérico[Equipo de la comunidad de Rust][comunidad]l llamar a las funcione[Equipo de la comunidad de Rust][comunidad]e rasgos si[Equipo de la comunidad de Rust][comunidad]uerpo](https://github.com/rust-lang/rust/pull/136497)
* [resuelva la ruta 'llvm-config' correctamente e[Equipo de la comunidad de Rust][comunidad]ompilacione[Equipo de la comunidad de Rust][comunidad]ruzadas](https://github.com/rust-lang/rust/pull/136681)
* [mostrar el format[Equipo de la comunidad de Rust][comunidad]e sugerenci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]iferencias en el reemplaz[Equipo de la comunidad de Rust][comunidad]etallado](https://github.com/rust-lang/rust/pull/127541)
* [algunas limpiezas 'rustc_middle'](https://github.com/rust-lang/rust/pull/136465)
* [alguno[Equipo de la comunidad de Rust][comunidad]juste[Equipo de la comunidad de Rust][comunidad]e biblioteca relacionado[Equipo de la comunidad de Rust][comunidad]on la edició[Equipo de la comunidad de Rust][comunidad]iscelánea](https://github.com/rust-lang/rust/pull/136705)
* [transmutabilidad: arreglar IC[Equipo de la comunidad de Rust][comunidad]l pasar ADT incorrecto para ASUMIR](https://github.com/rust-lang/rust/pull/136730)
* [UEFI: Proceso: Agregar soporte para variable[Equipo de la comunidad de Rust][comunidad]e entorn[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omandos](https://github.com/rust-lang/rust/pull/136418)
* [Actualizar ELSA a la versió[Equipo de la comunidad de Rust][comunidad]ás reciente](https://github.com/rust-lang/rust/pull/136094)
* [use +secure-plt para powerpc-unknown-linux-gnu{,spe}](https://github.com/rust-lang/rust/pull/136154)
* [use 'widening_mul' en lugar d[Equipo de la comunidad de Rust][comunidad]na función separada](https://github.com/rust-lang/rust/pull/136409)
* [us[Equipo de la comunidad de Rust][comunidad]na 'Opción' para 'FindNextFileHandle' en 'ReadDir' en lugar del valor centinela 'INVALID_FILE_HANDLE'](https://github.com/rust-lang/rust/pull/136552)
* [us[Equipo de la comunidad de Rust][comunidad]n[Equipo de la comunidad de Rust][comunidad]adena Ty corta para errore[Equipo de la comunidad de Rust][comunidad]e binop y unop](https://github.com/rust-lang/rust/pull/136315)
* [visita toda l[Equipo de la comunidad de Rust][comunidad]nformació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]epuración en MIR Visitor](https://github.com/rust-lang/rust/pull/136722)
* [Bloque[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ap[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]signació[Equipo de la comunidad de Rust][comunidad]e particiones](https://github.com/rust-lang/rust/pull/136115)
* [miri: permitir que e[Equipo de la comunidad de Rust][comunidad]ódigo llam[Equipo de la comunidad de Rust][comunidad] 'geteuid()'](https://github.com/rust-lang/miri/pull/4179)
* [Miri: Lanzar error UB a[Equipo de la comunidad de Rust][comunidad]nvocar un[Equipo de la comunidad de Rust][comunidad]orrecció[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ompatibilidad qu[Equipo de la comunidad de Rust][comunidad]o sea vararg con l[Equipo de la comunidad de Rust][comunidad]mportació[Equipo de la comunidad de Rust][comunidad]e vararg](https://github.com/rust-lang/miri/pull/4181)
* [miri: use el bloqueo fcntl en Solaris en lugar de flock que falt[Equipo de la comunidad de Rust][comunidad]llí](https://github.com/rust-lang/miri/pull/4177)
* [implementar 'eat_until' aprovechand[Equipo de la comunidad de Rust][comunidad]emchr en lexer](https://github.com/rust-lang/rust/pull/136585)
* [estabilizar 'Cursor::{get_mut, set_position}' en escenarios 'const'](https://github.com/rust-lang/rust/pull/136634)
* [estabilizar 'característica(trait_upcasting)'](https://github.com/rust-lang/rust/pull/134367)
* [Estabilizar la función 'map_many_mut'](https://github.com/rust-lang/rust/pull/136152)
* [estabilizar 'vec_pop_if'](https://github.com/rust-lang/rust/pull/135488)
* [añadir 'unchecked_disjoint_bitor' por ACP373](https://github.com/rust-lang/rust/pull/135760)
* [añadir API[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]cces[Equipo de la comunidad de Rust][comunidad]irecto 'UnsafeCell'](https://github.com/rust-lang/rust/pull/136398)
* [implementar la función 'new_range' inestable](https://github.com/rust-lang/rust/pull/136167)
* [implementar constructores str inherentes](https://github.com/rust-lang/rust/pull/136517)
* [añadir lo[Equipo de la comunidad de Rust][comunidad]étodos 'cast_signed' y 'cast_unsigned' para los tipos 'NonZero'](https://github.com/rust-lang/rust/pull/136511)
* [marque 'std::fmt::from_fn' como '#[must_use]'](https://github.com/rust-lang/rust/pull/136502)
* [std: mover e[Equipo de la comunidad de Rust][comunidad]ódig[Equipo de la comunidad de Rust][comunidad]e re[Equipo de la comunidad de Rust][comunidad] 'sys'](https://github.com/rust-lang/rust/pull/136449)
* [std::fs: simplificar aú[Equipo de la comunidad de Rust][comunidad]ás e[Equipo de la comunidad de Rust][comunidad]anej[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]irent64](https://github.com/rust-lang/rust/pull/136479)
* [optimizar l[Equipo de la comunidad de Rust][comunidad]mplementació[Equipo de la comunidad de Rust][comunidad]e 'Rc:<str>:::d efault()'](https://github.com/rust-lang/rust/pull/136099)
* [cambiar e[Equipo de la comunidad de Rust][comunidad]ombr[Equipo de la comunidad de Rust][comunidad]e lo[Equipo de la comunidad de Rust][comunidad]étodos 'slice::take...' a 'split_off... «](https://github.com/rust-lang/rust/pull/136555)
* [Windows: Eliminar archivo[Equipo de la comunidad de Rust][comunidad]e solo lectura](https://github.com/rust-lang/rust/pull/134679)
* [cargo: n[Equipo de la comunidad de Rust][comunidad]sar en Solaris 'libc::LOCK_*' que se eliminaro[Equipo de la comunidad de Rust][comunidad]e libc en la versión ...](https://github.com/rust-lang/cargo/pull/15143)
* [cargo: feat: añadir soporte 'cargo pkgid' par[Equipo de la comunidad de Rust][comunidad]argo-script](https://github.com/rust-lang/cargo/pull/14961)
* [Cargo: feat: emitir error si el paquet[Equipo de la comunidad de Rust][comunidad]o se encuentr[Equipo de la comunidad de Rust][comunidad]entr[Equipo de la comunidad de Rust][comunidad]el espaci[Equipo de la comunidad de Rust][comunidad]e trabajo](https://github.com/rust-lang/cargo/pull/15071)
* [cargo: arreglar enlac[Equipo de la comunidad de Rust][comunidad]e registr[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ambios](https://github.com/rust-lang/cargo/pull/15142)
* [Carga: Arreglar l[Equipo de la comunidad de Rust][comunidad]ondició[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]arrera en 'panic_abort_tests'](https://github.com/rust-lang/cargo/pull/15169)
* [Cargo: corrección: alinear la primera líne[Equipo de la comunidad de Rust][comunidad]e la list[Equipo de la comunidad de Rust][comunidad]esordenad[Equipo de la comunidad de Rust][comunidad]on la siguiente](https://github.com/rust-lang/cargo/pull/15161)
* [cargo: corrección: n[Equipo de la comunidad de Rust][comunidad]se "quis[Equipo de la comunidad de Rust][comunidad]ecir" en los errores](https://github.com/rust-lang/cargo/pull/15138)
* [Cargo: Hacer que el seguimient[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]aché sea resistent[Equipo de la comunidad de Rust][comunidad] archivo[Equipo de la comunidad de Rust][comunidad]nesperados](https://github.com/rust-lang/cargo/pull/15147)
* [Cargo: Simplificar retroceso](https://github.com/rust-lang/cargo/pull/15150)
* [Carga: Limpieza[Equipo de la comunidad de Rust][comunidad]e pequeños resolutores](https://github.com/rust-lang/cargo/pull/15040)
* [cargo: sugerir nombre[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]aracterísticas similares en CLI](https://github.com/rust-lang/cargo/pull/15133)
* [rustdoc: use ThinVec para pieza[Equipo de la comunidad de Rust][comunidad]rg genéricas](https://github.com/rust-lang/rust/pull/136265)
* [Habilitar la función "Saltar a DEF" en Rustc Docs](https://github.com/rust-lang/rust/pull/136589)
* [rustfmt: 'check_diff' reescritur[Equipo de la comunidad de Rust][comunidad]e la función](https://github.com/rust-lang/rustfmt/pull/6390)
* [clippy: añade pelusa 'single_option_map'](https://github.com/rust-lang/rust-clippy/pull/14033)
* [clippy: 'path_buf_push_overwrite': marcar sugerenci[Equipo de la comunidad de Rust][comunidad]omo 'MaybeIncorrect'](https://github.com/rust-lang/rust-clippy/pull/14010)
* [clippy: 'useless_asref': no hay pelusa si está e[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ierre par[Equipo de la comunidad de Rust][comunidad]ambiar la profundida[Equipo de la comunidad de Rust][comunidad]e la referencia](https://github.com/rust-lang/rust-clippy/pull/14090)
* [clippy: agregue la verificació[Equipo de la comunidad de Rust][comunidad]e MSRV para 'lines_filter_map_ok'](https://github.com/rust-lang/rust-clippy/pull/14130)
* [clippy: agregue la verificació[Equipo de la comunidad de Rust][comunidad]e MSRV para 'manual_flatten'](https://github.com/rust-lang/rust-clippy/pull/14086)
* [clippy: permitir 'assign_op_pattern' en la prueb[Equipo de la comunidad de Rust][comunidad]e 'string_add'](https://github.com/rust-lang/rust-clippy/pull/14143)
* [clippy: autofix para 'range_zip_with_len'](https://github.com/rust-lang/rust-clippy/pull/14136)
* [clippy: cambia l[Equipo de la comunidad de Rust][comunidad]plicabilida[Equipo de la comunidad de Rust][comunidad]e 'if_then_some_else_none' a 'MachineApplicable'](https://github.com/rust-lang/rust-clippy/pull/14106)
* [clippy: corrige "Affected lints" para 'allow-one-hash-in-raw-strings'](https://github.com/rust-lang/rust-clippy/pull/14186)
* [clippy: versió[Equipo de la comunidad de Rust][comunidad]orrect[Equipo de la comunidad de Rust][comunidad]e 'doc_overindented_list_items'](https://github.com/rust-lang/rust-clippy/pull/14152)
* [clippy: desapruebe la pelusa redundante 'option_map_or_err_ok' y saque 'manual_ok_or' de pedante](https://github.com/rust-lang/rust-clippy/pull/14027)
* [clippy: n[Equipo de la comunidad de Rust][comunidad]ctivar '[size_of_in_element_count]' para 'u8'](https://github.com/rust-lang/rust-clippy/pull/14011)
* [clippy: no emitir sugerenci[Equipo de la comunidad de Rust][comunidad]entr[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]acro en 'manual_async_fn'](https://github.com/rust-lang/rust-clippy/pull/14142)
* [clippy: n[Equipo de la comunidad de Rust][comunidad]sar bloques etiquetado[Equipo de la comunidad de Rust][comunidad]omo bloque[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ivel superior](https://github.com/rust-lang/rust-clippy/pull/14102)
* [clippy: arreglar ICE en 'unnecessary_mut_passed'](https://github.com/rust-lang/rust-clippy/pull/14065)
* [clippy: arregla 'let_and_return' con variables temporales, y distingue entre las edicione[Equipo de la comunidad de Rust][comunidad]e Rust](https://github.com/rust-lang/rust-clippy/pull/14180)
* [clippy: arreglar la sugerenci[Equipo de la comunidad de Rust][comunidad]e 'obfuscated_if_else' en el lad[Equipo de la comunidad de Rust][comunidad]zquierd[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n expr binario](https://github.com/rust-lang/rust-clippy/pull/14124)
* [clippy: arreglar documentos para '#[clippy::format_args]'](https://github.com/rust-lang/rust-clippy/pull/14161)
* [clippy: corrección: 'manual_unwrap_or_default' sugiere falsament[Equipo de la comunidad de Rust][comunidad]uando el tip[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ondición e[Equipo de la comunidad de Rust][comunidad]ncierto](https://github.com/rust-lang/rust-clippy/pull/13889)
* [clippy: manej[Equipo de la comunidad de Rust][comunidad]á[Equipo de la comunidad de Rust][comunidad]asos en 'is_normalizable'](https://github.com/rust-lang/rust-clippy/pull/13833)
* [clippy: hacer línea vacía-despué[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]na pelus[Equipo de la comunidad de Rust][comunidad]lippy temprana](https://github.com/rust-lang/rust/pull/136657)
* [clippy: hacer que 'manual_map' ignore los tipos qu[Equipo de la comunidad de Rust][comunidad]ontienen 'dyn'](https://github.com/rust-lang/rust-clippy/pull/12712)
* [clippy: mover 'mutex_integer' a restricción y mejorar 'mutex_'{'integer', 'atomic'} docs](https://github.com/rust-lang/rust-clippy/pull/14110)
* [clippy: omite 'use_self' dentr[Equipo de la comunidad de Rust][comunidad]e las expansione[Equipo de la comunidad de Rust][comunidad]acr[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n bloque 'impl Self'](https://github.com/rust-lang/rust-clippy/pull/13128)
* [clippy: do[Equipo de la comunidad de Rust][comunidad]ejora[Equipo de la comunidad de Rust][comunidad] 'disallowed_*'](https://github.com/rust-lang/rust-clippy/pull/13669)
* [clippy: use e[Equipo de la comunidad de Rust][comunidad]uerpo MIR par[Equipo de la comunidad de Rust][comunidad]dentificar más llamadas "equivalentes predeterminadas" para 'derivable_impls'](https://github.com/rust-lang/rust-clippy/pull/13988)
* [clippy: use paréntesi[Equipo de la comunidad de Rust][comunidad]uando se[Equipo de la comunidad de Rust][comunidad]ecesario en 'nonminimal_bool' lint](https://github.com/rust-lang/rust-clippy/pull/14187)
* [rust-analyzer: corregir el error off-by-one en RangeFormatting](https://github.com/rust-lang/rust-analyzer/pull/19124)
* [rust-analyzer: no emit[Equipo de la comunidad de Rust][comunidad]currenci[Equipo de la comunidad de Rust][comunidad]e scip vacío para lo[Equipo de la comunidad de Rust][comunidad]ncorporados](https://github.com/rust-lang/rust-analyzer/pull/19105)
* [rust-analyzer: arreglar la resolución ID[Equipo de la comunidad de Rust][comunidad]e 'uso' dentr[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]uerpo](https://github.com/rust-lang/rust-analyzer/pull/19094)
* [Rust-analyzer: si el elemento existe en e[Equipo de la comunidad de Rust][comunidad]ódulo, resuelv[Equipo de la comunidad de Rust][comunidad]om[Equipo de la comunidad de Rust][comunidad]ódulo en lugar de tipo](https://github.com/rust-lang/rust-analyzer/pull/19088)
* [Rust-analyzer: Resuelva los tipo[Equipo de la comunidad de Rust][comunidad]e proyecció[Equipo de la comunidad de Rust][comunidad]nte[Equipo de la comunidad de Rust][comunidad]e verificar los lanzamientos](https://github.com/rust-lang/rust-analyzer/pull/19106)
* [Rust-analyzer: Rango[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ap[Equipo de la comunidad de Rust][comunidad]scendente e[Equipo de la comunidad de Rust][comunidad]sistencia 'convert_tuple_struct_to_named_struct'](https://github.com/rust-lang/rust-analyzer/pull/18912)
* [rust-analyzer: line-index: don't try t[Equipo de la comunidad de Rust][comunidad]se (unavailable) neo[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]n big-endia[Equipo de la comunidad de Rust][comunidad]arch64](https://github.com/rust-lang/rust-analyzer/pull/19083)
* [rust-analyzer: opción par[Equipo de la comunidad de Rust][comunidad]esactivar las sugerencia[Equipo de la comunidad de Rust][comunidad]e tip[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ncrustación para los parámetro[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ierre](https://github.com/rust-lang/rust-analyzer/pull/19104)
* [Rust-analyzer: Organizar capítulos en la barra latera[Equipo de la comunidad de Rust][comunidad]e MDBoer](https://github.com/rust-lang/rust-analyzer/pull/19115)
* [Analizador de Rust: evite que los pánico[Equipo de la comunidad de Rust][comunidad]erriben las rosca[Equipo de la comunidad de Rust][comunidad]e los trabajadores](https://github.com/rust-lang/rust-analyzer/pull/19093)
* [Rust-Analyzer: Dividir e[Equipo de la comunidad de Rust][comunidad]ebad[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]aché e[Equipo de la comunidad de Rust][comunidad]istintas fases](https://github.com/rust-lang/rust-analyzer/pull/19084)
* [rust-analyzer: use l[Equipo de la comunidad de Rust][comunidad]utabilida[Equipo de la comunidad de Rust][comunidad]nterior para 'ProcMacrorv::expanders' cargados](https://github.com/rust-lang/rust-analyzer/pull/19099)

### Clasificació[Equipo de la comunidad de Rust][comunidad]el rendimient[Equipo de la comunidad de Rust][comunidad]e[Equipo de la comunidad de Rust][comunidad]ompilador de Rust

Una semana relativament[Equipo de la comunidad de Rust][comunidad]eutra, co[Equipo de la comunidad de Rust][comunidad]ucho[Equipo de la comunidad de Rust][comunidad]ambios reales, pero l[Equipo de la comunidad de Rust][comunidad]ayoría pequeños en
magnitud. E[Equipo de la comunidad de Rust][comunidad]ambi[Equipo de la comunidad de Rust][comunidad]ás significativo es e[Equipo de la comunidad de Rust][comunidad]ovimient[Equipo de la comunidad de Rust][comunidad]e rustdo[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]inificació[Equipo de la comunidad de Rust][comunidad]e JS/CSS a
tiemp[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ompilación, lo que redujo los tiempo[Equipo de la comunidad de Rust][comunidad]e generació[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ocumentos en l[Equipo de la comunidad de Rust][comunidad]ayorí[Equipo de la comunidad de Rust][comunidad]e los punto[Equipo de la comunidad de Rust][comunidad]e referenci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]anera justa
significativamente.

Triaje realizado por **@simulacrum**.
Rang[Equipo de la comunidad de Rust][comunidad]e revisión: [01e4f19c.. C03C38D5](https://perf.rust-lang.org/?start=01e4f19cc8027925ffe0885a86388b700e46bfab&end=c03c38d5c2368cd2aa0e056dba060b94fc747f4e&absolute=false&stat=instructions%3Au)

3 regresiones, 5 mejoras, 1 mixta; 2 de ellos en rollups
32 comparacione[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]rtefactos realizadas en total

[Inform[Equipo de la comunidad de Rust][comunidad]omplet[Equipo de la comunidad de Rust][comunidad]quí](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-02-10.md)

### [RFC aprobados](https://github.com/rust-lang/rfcs/commits/master)

Lo[Equipo de la comunidad de Rust][comunidad]ambios en Rust siguen el proces[Equipo de la comunidad de Rust][comunidad]e Rust [RFC (solicitu[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentarios)](https://github.com/rust-lang/rfcs#rust-rfcs). Estos
son las RFC que s[Equipo de la comunidad de Rust][comunidad]probaron para s[Equipo de la comunidad de Rust][comunidad]mplementación esta semana:

* *No s[Equipo de la comunidad de Rust][comunidad]probaron RFC esta semana.*

### Período fina[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentarios

Cada semana, [el equipo](https://www.rust-lang.org/team.html) anuncia el 'período fina[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentarios' para las RFC y las RP clave
que están llegand[Equipo de la comunidad de Rust][comunidad] un[Equipo de la comunidad de Rust][comunidad]ecisión. Expresa tu[Equipo de la comunidad de Rust][comunidad]pinione[Equipo de la comunidad de Rust][comunidad]hora.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: meta[Equipo de la comunidad de Rust][comunidad]el proyecto para 2025h1](https://github.com/rust-lang/rfcs/pull/3764)

#### Seguimient[Equipo de la comunidad de Rust][comunidad]e problemas y relaciones públicas
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Estabilizar función 'num_midpoint_signed'](https://github.com/rust-lang/rust/pull/134340)
* [N[Equipo de la comunidad de Rust][comunidad]eduplicar la list[Equipo de la comunidad de Rust][comunidad]e tipo[Equipo de la comunidad de Rust][comunidad]sociados proporcionada por dyn principal](https://github.com/rust-lang/rust/pull/136458)
* [regresión: un valor de tipo 'HashMap<Pulse, u64>' no se pued[Equipo de la comunidad de Rust][comunidad]onstruir a partir de](https://github.com/rust-lang/rust/issues/135669)
* [Problem[Equipo de la comunidad de Rust][comunidad]e seguimiento para 'inherent_str_constructors'](https://github.com/rust-lang/rust/issues/131114)
* [Problem[Equipo de la comunidad de Rust][comunidad]e seguimiento para 'os_str_display'](https://github.com/rust-lang/rust/issues/120048)
* [No permitir atributos en los patrone[Equipo de la comunidad de Rust][comunidad]e repos[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]amp[Equipo de la comunidad de Rust][comunidad]e estructura](https://github.com/rust-lang/rust/pull/136490)
* [feat(core): impl Step for NonZero < u* >](https://github.com/rust-lang/rust/pull/127534)
* [Avis[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ncompatibilidad futura 'unsupported_fn_ptr_calling_conventions': Advertir también e[Equipo de la comunidad de Rust][comunidad]ependencias](https://github.com/rust-lang/rust/pull/135767)
* [Inform[Equipo de la comunidad de Rust][comunidad] Mir[Equipo de la comunidad de Rust][comunidad]e las filtracione[Equipo de la comunidad de Rust][comunidad]e 'Box'/'Vec'/'String'](https://github.com/rust-lang/rust/pull/135811)
* [Problem[Equipo de la comunidad de Rust][comunidad]e seguimiento para 'integer_sign_cast'](https://github.com/rust-lang/rust/issues/125882)

##### [Carga](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay problema[Equipo de la comunidad de Rust][comunidad]e seguimient[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]arg[Equipo de la comunidad de Rust][comunidad]i P[Equipo de la comunidad de Rust][comunidad]ngresaro[Equipo de la comunidad de Rust][comunidad]l períod[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentarios finales esta semana.*

##### [Equipo lingüístico](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *Ninguna propuest[Equipo de la comunidad de Rust][comunidad]e equipo lingüístico entró en el Período Fina[Equipo de la comunidad de Rust][comunidad]e Comentarios esta semana.*

##### [Referencia lingüística](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hay RFC de referenci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]diom[Equipo de la comunidad de Rust][comunidad]ngresó al Período Fina[Equipo de la comunidad de Rust][comunidad]e Comentarios esta semana.*

##### [Directrice[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ódigo[Equipo de la comunidad de Rust][comunidad]nseguros](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No hubo problema[Equipo de la comunidad de Rust][comunidad]e seguimient[Equipo de la comunidad de Rust][comunidad]e pauta[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ódig[Equipo de la comunidad de Rust][comunidad]nsegur[Equipo de la comunidad de Rust][comunidad] P[Equipo de la comunidad de Rust][comunidad]ngresaro[Equipo de la comunidad de Rust][comunidad]l períod[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omentarios finales esta semana.*

#### [RFC nuevas y actualizadas](https://github.com/rust-lang/rfcs/pulls)
* [#[doc(consts)]](https://github.com/rust-lang/rfcs/pull/3770)

## Próximos eventos

Evento[Equipo de la comunidad de Rust][comunidad]xidados entre 2025-02-12 - 2025-03-12 🦀

### Virtual
* 2025-02-12 | Virtual (Cardiff, Reino Unido) | [Rust y C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Encuentro en líne[Equipo de la comunidad de Rust][comunidad]onjunt[Equipo de la comunidad de Rust][comunidad]on Rust Bristol!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/305833952)
* 13/02/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 14/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativ[Equipo de la comunidad de Rust][comunidad]el Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305815307)
* 19/02/2025 | Virtual (Vancouver, Columbia Británica, CA) | [Rus[Equipo de la comunidad de Rust][comunidad]e Vancouver](https://www.meetup.com/vancouver-rust/events/)
    * [**Procedenci[Equipo de la comunidad de Rust][comunidad]el puntero**](https://www.meetup.com/vancouver-rust/events/304051783)
* 2025-02-20 | Híbrido (Redmond, WA, EE. UU.) | [Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrer[Equipo de la comunidad de Rust][comunidad]e 2025 SRUG (Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Virtual (Jersey City, Nueva Jersey, EE. UU.) | [Jersey City, Elegante y Curiosa Cooperativ[Equipo de la comunidad de Rust][comunidad]el Club de Codificadores](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbcc)
* 25/02/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunió[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Cuarto Martes**](https://www.meetup.com/dallasrust/events/305361428)
* 25/02/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: E[Equipo de la comunidad de Rust][comunidad]omplicad[Equipo de la comunidad de Rust][comunidad]und[Equipo de la comunidad de Rust][comunidad]e String[Equipo de la comunidad de Rust][comunidad]n Rust**](https://www.meetup.com/women-in-rust/events/305716182)
* 25/02/2025 | Virtual (Washington, DC, EE. UU.) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mita[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]es Rustful—Everett Pompeii presenta Bencher 🐰 **](https://www.meetup.com/rustdc/events/305170682)
* 27/02/2025 | Virtual (EE. UU.) | [Laboratorios Ardan](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Introducció[Equipo de la comunidad de Rust][comunidad] Rust**](https://www.eventbrite.com/e/intro-to-rust-tickets-1237517059839)
* 27/02/2025 | Virtual (Berlín, DE) | [Rust Berlín](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820295)
* 27/02/2025 | Virtual (Charlottesville, VA, EE. UU.) | [Reunió[Equipo de la comunidad de Rust][comunidad]e Rust en Charlottesville](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Analizand[Equipo de la comunidad de Rust][comunidad]pcione[Equipo de la comunidad de Rust][comunidad]e líne[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]omando[Equipo de la comunidad de Rust][comunidad]on teorí[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ategorías y asíncrono**](https://www.meetup.com/charlottesville-rust-meetup/events/305948365)
* 01/03/2025 | Virtual (Kampala, UG) | [Círcul[Equipo de la comunidad de Rust][comunidad]e Rust Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Reunió[Equipo de la comunidad de Rust][comunidad]el Círcul[Equipo de la comunidad de Rust][comunidad]e Rust**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-62876317658/)
* 05/03/2025 | Virtual (Indianápolis, IN, EE. UU.) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - co[Equipo de la comunidad de Rust][comunidad]istanciamiento social**](https://www.meetup.com/indyrs/events/302031659)
* 06/03/2025 | Virtual (Nürnberg, DE) | [Rust, Núremberg, DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820281/)
* 06/03/2025 | Virtual (Róterdam, Países Bajos) | [Desarroll[Equipo de la comunidad de Rust][comunidad]e juego[Equipo de la comunidad de Rust][comunidad]e Bevy](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #9**](https://www.meetup.com/bevy-game-development/events/306131557)
* 11/03/2025 | Virtual (Dallas, TX, EE. UU.) | [Reunió[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Dallas Rust](https://www.meetup.com/dallasrust/events/)
    * [**Segundo Martes**](https://www.meetup.com/dallasrust/events/303522529)
* 11/03/2025 | Virtual (Londres, Reino Unido) | [Mujeres en Rust](https://www.meetup.com/women-in-rust/events/)
    * [** 👋 L[Equipo de la comunidad de Rust][comunidad]omunidad se pon[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]ía**](https://www.meetup.com/women-in-rust/events/305716839)

### Asia
* 24/02/2025 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Presencial Rust febrero 2025 en AWS en Tel Aviv**](https://www.meetup.com/rust-tlv/events/305570131)
* 01/03/2025 | Bangalore/Bangalore, IN | [Rus[Equipo de la comunidad de Rust][comunidad]e Bangalore](https://hasgeek.com/rustbangalore)
    * [**Encuentr[Equipo de la comunidad de Rust][comunidad]e Zig y Rust**](https://lu.ma/460w8v58)

### Europa
* 2025-02-12 | Reading, Reino Unido | [Taller de lectur[Equipo de la comunidad de Rust][comunidad]e Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunió[Equipo de la comunidad de Rust][comunidad]e lectur[Equipo de la comunidad de Rust][comunidad]e Rust**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 14/02/2025 | Edimburgo, Reino Unido | [Rust y su[Equipo de la comunidad de Rust][comunidad]migos](https://www.meetup.com/rust-edi/events/)
    * [**Rus[Equipo de la comunidad de Rust][comunidad]nd Friends (café diurno)**](https://www.meetup.com/rust-and-friends/events/305791536)
* 18/02/2025 | Leipzig, SN, DE | [Rust - Programació[Equipo de la comunidad de Rust][comunidad]e sistema[Equipo de la comunidad de Rust][comunidad]odernos en Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introducció[Equipo de la comunidad de Rust][comunidad] la Programación Contextual-Genérica en Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 18/02/2025 | Londres, Reino Unido | [Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Rust London](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks Febrero: Reunión previ[Equipo de la comunidad de Rust][comunidad] l[Equipo de la comunidad de Rust][comunidad]onferenci[Equipo de la comunidad de Rust][comunidad]e Rust Nation 2025 **](https://www.meetup.com/rust-london-user-group/events/306036448)
* 2025-02-19 - 2025-02-20 | Londres, Reino Unido | [Nación Rust Reino Unido](https://www.rustnationuk.com/)
    * [**Rust Nation Reino Unido 2025**](https://www.rustnationuk.com/)
* 2025-02-20 | Berna, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #1 @Puzzle ITC**](https://www.meetup.com/rust-bern/events/305597994)
* 21/02/2025 | Londres, Reino Unido | [Rust Global: Londres 2025](https://rustfoundation.org/event/rust-global-london-2025/)
    * [**Rust Global: Londres 2025**](https://rustfoundation.org/event/rust-global-london-2025/)
* 2025-02-22 | Estocolmo, SE | [Estocolmo Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Foro Fik[Equipo de la comunidad de Rust][comunidad]e Ferris #9**](https://www.meetup.com/stockholm-rust/events/305848723)
* 25/02/2025 | Madrid, ES | [Rust loco](https://www.meetup.com/madrust/events/)
    * [**Rus[Equipo de la comunidad de Rust][comunidad]esd[Equipo de la comunidad de Rust][comunidad]ero: Cargo y tipos**](https://www.meetup.com/madrust/events/305896258)
* 26/02/2025 | Darmstadt, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Ajust[Equipo de la comunidad de Rust][comunidad]e[Equipo de la comunidad de Rust][comunidad]ompilador de Rust**](https://www.meetup.com/rust-rhein-main/events/305990886/)
* 27/02/2025 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn en Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809675)
* 27/02/2025 | París, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Encuentr[Equipo de la comunidad de Rust][comunidad]e Rust #75**](https://www.meetup.com/rust-paris/events/305791655)
* 01/03/2025 | Nürnberg, DE | [Rus[Equipo de la comunidad de Rust][comunidad]e Núremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Speyer**](https://www.meetup.com/rust-noris/events/305361732/)
* 05/03/2025 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**17º Encuentr[Equipo de la comunidad de Rust][comunidad]e BcnRust**](https://www.meetup.com/bcnrust/events/305887675)
* 12/03/2025 | Reading, Reino Unido | [Taller de lectur[Equipo de la comunidad de Rust][comunidad]e Rust](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reunió[Equipo de la comunidad de Rust][comunidad]e lectur[Equipo de la comunidad de Rust][comunidad]e Rust**](https://www.meetup.com/reading-rust-workshop/events/305045445)

### Améric[Equipo de la comunidad de Rust][comunidad]el Norte
* 13/02/2025 | Portland, Oregón, Estados Unidos | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Arañando Wikipedia educadamente en Rus[Equipo de la comunidad de Rust][comunidad]síncrono**](https://www.meetup.com/pdxrust/events/306044189)
* 14/02/2025 | Boston, MA, EE. UU. | [Encuentr[Equipo de la comunidad de Rust][comunidad]e Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en e[Equipo de la comunidad de Rust][comunidad]entr[Equipo de la comunidad de Rust][comunidad]e Rust, 14 de febrero**](https://www.meetup.com/bostonrust/events/305804954)
* 18/02/2025 | San Francisco, CA, EE. UU. | [Grup[Equipo de la comunidad de Rust][comunidad]e Estudi[Equipo de la comunidad de Rust][comunidad]e la Roy[Equipo de la comunidad de Rust][comunidad]e San Francisco](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Hacking de Rust en persona**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)
* 2025-02-20 | Chicago, Illinois, Estados Unidos | [Encuentr[Equipo de la comunidad de Rust][comunidad]e Rust en Chicago](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Hora feliz de Rust**](https://www.meetup.com/chicago-rust-meetup/events/306087854)
* 2025-02-20 | Nashville, Tennessee, Estados Unidos | [Desarrolladore[Equipo de la comunidad de Rust][comunidad]e Music City Rust](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 2: Fundamento[Equipo de la comunidad de Rust][comunidad]e[Equipo de la comunidad de Rust][comunidad]esarroll[Equipo de la comunidad de Rust][comunidad]e juegos**](https://www.meetup.com/music-city-rust-developers/events/304333047)
* 2025-02-20 | Redmond, WA, EE. UU. | [Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Seattle Rust](https://www.meetup.com/join-srug/events/)
    * [**Febrer[Equipo de la comunidad de Rust][comunidad]e 2025 SRUG (Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Seattle Rust) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 21/02/2025 | Ciuda[Equipo de la comunidad de Rust][comunidad]e México, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y cienci[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]atos**](https://www.meetup.com/rust-mx/events/305793010)
* 2025-02-22 | Boston, MA, EE. UU. | [Encuentr[Equipo de la comunidad de Rust][comunidad]e Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerz[Equipo de la comunidad de Rust][comunidad]e Rust en Union Square en Somerville, 22 de febrero**](https://www.meetup.com/bostonrust/events/305805059)
* 26/02/2025 | Austin, TX, Estados Unidos | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Almuerzo Rust - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)
* 27/02/2025 | Atlanta, Georgia, Estados Unidos | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Comenzando la reunió[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]uevo**](https://www.meetup.com/rust-atl/events/305776081)
* 02/03/2025 | Boston, MA, EE. UU. | [Encuentr[Equipo de la comunidad de Rust][comunidad]e Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerz[Equipo de la comunidad de Rust][comunidad]e Rus[Equipo de la comunidad de Rust][comunidad]e Beacon Hill, 2 d[Equipo de la comunidad de Rust][comunidad]arzo**](https://www.meetup.com/bostonrust/events/305805164)
* 06/03/2025 | Saint Louis, MO, EE. UU. | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**CRDTs en Rust**](https://www.meetup.com/stl-rust/events/305187783)
* 2025-03-10 | Boston, MA, EE. UU. | [Encuentr[Equipo de la comunidad de Rust][comunidad]e Boston Rust](https://www.meetup.com/bostonrust/events/)
    * [**Almuerzo en Davis Square, 10 d[Equipo de la comunidad de Rust][comunidad]arzo**](https://www.meetup.com/bostonrust/events/305805192)

### Oceanía
* 24/02/2025 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Encuentr[Equipo de la comunidad de Rust][comunidad]e febrer[Equipo de la comunidad de Rust][comunidad]e 2025 en Rust Melbourne**](https://www.meetup.com/rust-melbourne/events/306040785)
* 25/02/2025 | Barton, AC, AU | [Grup[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]suario[Equipo de la comunidad de Rust][comunidad]e Canberra Rust](https://www.meetup.com/rust-canberra/events/)
    * [**Encuentr[Equipo de la comunidad de Rust][comunidad]e febrero**](https://www.meetup.com/rust-canberra/events/306090406)
* 04/03/2025 | Perth, WA, AU | [Grup[Equipo de la comunidad de Rust][comunidad]e encuentr[Equipo de la comunidad de Rust][comunidad]e Rust Perth](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**Cómo Orica está usando Rust en su lugar de trabajo**](https://www.meetup.com/perth-rust-meetup-group/events/306131753)

Si está ejecutand[Equipo de la comunidad de Rust][comunidad]n event[Equipo de la comunidad de Rust][comunidad]e Rust, agréguel[Equipo de la comunidad de Rust][comunidad]l [calendario] par[Equipo de la comunidad de Rust][comunidad]btener
que s[Equipo de la comunidad de Rust][comunidad]encion[Equipo de la comunidad de Rust][comunidad]quí. Por favor, recuerd[Equipo de la comunidad de Rust][comunidad]gregar un enlac[Equipo de la comunidad de Rust][comunidad]l evento también.
Enví[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]orreo electrónic[Equipo de la comunidad de Rust][comunidad]l [Equip[Equipo de la comunidad de Rust][comunidad]e l[Equipo de la comunidad de Rust][comunidad]omunida[Equipo de la comunidad de Rust][comunidad]e Rust] [comunidad] par[Equipo de la comunidad de Rust][comunidad]cceder.

[calendario]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[comunidad]: mailto:community-team@rust-lang.org

## Empleos
<!-- Trabajo[Equipo de la comunidad de Rust][comunidad]e Rust: TWiR h[Equipo de la comunidad de Rust][comunidad]ejad[Equipo de la comunidad de Rust][comunidad]e presentar oferta[Equipo de la comunidad de Rust][comunidad]e trabaj[Equipo de la comunidad de Rust][comunidad]ndividuales. Puedes leer más sobre est[Equipo de la comunidad de Rust][comunidad]ambi[Equipo de la comunidad de Rust][comunidad]quí: https://github.com/rust-lang/this-week-in-rust/issues/3412 -->

Por favor, consulte el último hil[Equipo de la comunidad de Rust][comunidad]e [Quién está contratando en r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Fras[Equipo de la comunidad de Rust][comunidad]e la semana

> El hech[Equipo de la comunidad de Rust][comunidad]e que la[Equipo de la comunidad de Rust][comunidad]osas sean útile[Equipo de la comunidad de Rust][comunidad]o significa que sea[Equipo de la comunidad de Rust][comunidad]ágicamente sólidas.

– [Ralf Jung en github](https://github.com/rust-lang/rust/issues/132442#issuecomment-2636065726)

¡Gracia[Equipo de la comunidad de Rust][comunidad] [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1658) por la sugerencia!

[¡Por favor, envíe su[Equipo de la comunidad de Rust][comunidad]otizaciones y vote para la próxima semana!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust es editado por: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo]( https://github.com/bdillo)*

*E[Equipo de la comunidad de Rust][comunidad]lojamient[Equipo de la comunidad de Rust][comunidad]e la list[Equipo de la comunidad de Rust][comunidad][Equipo de la comunidad de Rust][comunidad]orreo electrónico está patrocinado por [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discusión en r/rust](https://www.reddit.com/r/rust/comments/1ioajhc/this_week_in_rust_586_this_week_in_rust/)</small>
