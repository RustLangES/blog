---
title: Hicimos el sitio web de nuestra boda en Angular y Rust
description: La historia comienza en agosto del año pasado nos comprometimos durante nuestras vacaciones en...
author: Julieta Campos Guzmán
github_user: juliescript
date: 2020-04-24
tags:
  - rust
  - angular
  - devto
  - devjournal
social:
  github: https://github.com/juliescript
  twitter: https://twitter.com/juliescriptdev
  website: https://phosphorus-moscu.gitlab.io
---

## La Historia

En agosto del año pasado nos comprometimos durante nuestras vacaciones en Japón.

Decidimos planear nuestra boda en México porque aunque vivimos en Alemania, nuestras familias están en México y es donde queremos celebrar con todos nuestros seres queridos.

Una de las partes más importantes de planear una boda son las invitaciones. Usualmente se hacen de forma física, son cosas muy hermosas y elaboradas que se envían a los invitados. En ellas se encuentran todos los datos sobre la boda como:

- Fecha
- Lugar
- Hora
- Programa
- Etiqueta
- Mesa de regalos
- Boletos para la recepción

Hacer invitaciones no es una opción para nosotros. Tenemos que coordinar invitados que vienen de distintas ciudades y de distintos países. Además de que mandarlas a hacer puede ser muy caro y enviarlas es mucho trabajo.

Por eso fue que decidimos usar nuestras habilidades como desarrolladores y unir fuerzas para crear un sitio web para nuestra boda.

## El sitio

Nuestro sitio va a tener dos funciones principales:

- Dar la información sobre la boda
- Administrar la asistencia de los invitados

Así que pensamos en crear un sistema que sirviera para que los invitados confirmen su asistencia y que después podamos enviar la invitación más formal en PDF antes del mero día de la boda.

Para lograr esto nos dividimos el trabajo. Mi prometido se encargó de hacer todo el backend y yo me encargué de hacer el frontend. Entre los dos decidimos en un diseño y agregamos el contenido a la página. Mi prometido se encargó de traducir los textos porque necesitamos tener el sitio en español e inglés.

Va sin recalcar que tenemos excepciones para invitados que no saben o que no tienen acceso a la web.

### El tech stack

Para el backend, todo fue manejaod por mi prometido así que no entraré en muchos detalles.

El lenguaje de programación fue Rust porque es el lenguaje que está usando ahora.

El stack del backend terminó así:

- [Rust](https://github.com/ngx-translate/core)
  - [Gotham](https://gotham.rs/) - para manejar el API
  - [Diesel](http://diesel.rs/) - para conectar y administrar la base de datos
- [PostgreSQL](https://www.postgresql.org/)
- [GitHub Actions](https://github.com/features/actions)
- Hosting en [Digital Ocean](https://www.digitalocean.com/)

El stack del frontend fue el siguiente:

- [Angular 9](https://angular.io/)
- [SASS](https://sass-lang.com/)
- Deploy en [Netlify](https://www.netlify.com/)

Para el manejo de usuarios decidimos usar Facebook y Google login. La verdad no queríamos quedarnos con información personal del usuario y no quisimos lidiar con GDPR.

### El proceso

En el momento en que empezamos a planear el sitio, mi prometido estaba tomando una clase de administración de proyectos web para su maestría. Por mi lado, he tomado varios talleres de generación de ideas y de crear proyectos de forma ágil.

De nuevo juntamos recursos e hicimos una sesión para definir que es lo que necesitaba la página y que es lo que queríamos lograr. Al final terminamos poniendo todas las tareas en un tablero tipo Kanban en JIRA. Esto nos ayudó mucho a mantener nuestro objetivo en la mira.

### El diseño

El diseño fue decisión principalmente mía. La verdad soy pésima diseñando así que me puse a buscar inspiración en Pinterest y otros sitios como Wix y Squarespace.

Al final decidí reproducir una plantilla de sitio para boda de Squarespace. El diseño nos gustó mucho porque era sencillo y elegante. El esquema de colores es neutral y no se ve super feminino o masculino.

Es un diseño bastante sobrio y la tipografía me encantó.

Además de que ya viene con diseño móvil que siempre es un viacrucis incluir.

[![Plantilla Squarespace home](/assets/images/julie-y-vic-home.png "Plantilla Squarespace home")](/assets/images/julie-y-vic-home.png)

[![Plantilla Squarespace about](/assets/images/julie-y-vic-about.jpeg "Plantilla Squarespace about")](/assets/images/julie-y-vic-about.jpeg)

_Plantilla [Morena](https://morena-demo.squarespace.com/) de Squarespace_

A partir del diseño creamos las demás páginas que no estaban definidas.

No tiene nada de malo reproducir un diseño ya creado si no eres bueno en diseño o si no puedes pagarle a un diseñador.

### El frontend

Jugué con la idea de hacer el frontend con React y Gatsby pero la verdad es que me siento mucho más cómoda con Angular. Puedo resolver problemas mejor y no tengo que sufrir tanto conectándome al backend.

Además de que estilizar Angular es algo que es un sueño cuando lo haces con SASS. Es mi tech stack favorito y me ha servido bien varios años ya.

El mapa del sitio quedó de la siguiente manera:

- Página principal
- Información de la boda
- Información de viaje
- 

RSVP

  - Login
  - Redirección de login de facebook
  - Página de perfil
- 404

#### Diseño responsivo

Hacer sitios responsivos creo que es algo que nos llega a dar mucha flojera a varios programadores. Hay muchas variables y hay que escribir mucho código. Afortunadamente pude usar casi puro CSS para manejar el diseño responsivo.

La única ocasión donde tuve que incorporar Javascript fue con el menú para dispositivos móviles. Necesitaba manejar cuando activo y desactivo el menú y no me quise complicar la vida. Así que fue con Javascript.

#### Facebook y Google Login

Para el manejo de usuario usamos Google y Facebook login. Toda la implementación la hizo mi prometido en Rust, así que del lado del frontend me tocó manejar las redirecciones.

El flujo que tenemos es el siguiente:

1. Usuario recibe un link de invitación con un código único
2. En la página, el usuario puede elegir entre iniciar sesión con Facebook o con Google
3. Ya que se inicia la sesión, se redirecciona al usuario de regreso al sitio
4. El usuario puede elegir si asistirá o no a la boda y si necesita llevar pareja

### Traducciones

Como lo mencioné al principio, necesitamos traducciones para el sitio. Tenía muchas ganas de usar las traducciones nativas de Angular pero me hubiera tomado mucho tiempo configurarlas.

Decidí ir por un paquete que usé mucho tiempo en mi trabajo anterior llamado [@ngx-translate/core](https://github.com/ngx-translate/core). Este paquete me permite generar variables y mantener los idiomas con base en archivos json. La configuración es muy corta y maneja el cambio de idioma de inmediato y a nivel de aplicación.

### El producto terminado

Al final el sitio terminó así:

[![Julie y Vic home](/assets/images/julie-y-vic-home.png "Julie y Vic home")](/assets/images/julie-y-vic-home.png)

[![Julie y Vic about](/assets/images/julie-y-vic-about.jpeg "Julie y Vic about")](![/assets/images/julie-y-vic-about.jpeg))

## Conclusiones

**¿Lo volvería a hacer?**

La verdad es que si no fuera por la funcionalidad especial que queríamos para administrar a los usuarios, hubiera utilizado alguna herramienta ya existente para hacerlo. Incluso contratar Squarespace para usar la plantilla que reproduje.

No queríamos lidiar con procesar formularios a mano o conservar datos de invitados, por eso el login con Facebook o Google fue muy importante y eso es algo que no vimos en otras plataformas para hacer sitios de boda.

**Altervativas para tu propio sitio de boda**

Puedes usar una herramienta como [Squarespace](https://www.squarespace.com/) o [Wix](https://www.wix.com/) para hacer un sitio. Incluso hay portales exclusivos de bodas como [The Knot](https://www.theknot.com/) que te permiten crear un pequeño sitio con links a todo lo que necesitas.

**¿Es necesario?**

Probablemente no. Si tu boda puede llevarse con invitaciones normales y es la ruta que quieres tomar, es lo único que se necesita. Si quieres manejar todo con un evento de Facebook se puede. Todo depende de lo que quieras para mantener a tus invitados enterados de todos los detalles de tu evento.

Me gustó mucho la experiencia de hacer el sitio de la boda. Mi prometido y yo nunca habíamos trabajado en un proyecto juntos, así que fue una bonita experiencia. Además esto nos ayudó a pensar en más detalles del evento y organizar a nuestros invitados de mejor manera.

Lamentablemente tuvimos que posponer la boda debido al COVID-19 pero cuando tengamos una nueva fecha pondremos el sitio en línea.