![2024-05-25 06 10 19 blog rustlang-es org be8dd0ac9870](https://github.com/RustLangES/blog/assets/19656993/055ed112-e100-449d-bea6-e5110e7f483a)

<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/blog/ci.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/blog/deploy.yml?label=deploy" />
</p>

# 🤝🏼 Agrega tu articulo

Pasos:

- Haz fork de este proyecto
- Crea un archivo Markdown en la carpeta `articles`
- Escribe tu articulo con este formato

  ```md
  ---
  title: Mi Articulo
  description: La descripcion de mi articulo
  author: RustLangES
  github_user: RustLangES
  date: 2023-09-17
  tags:
    - rust
    - comunidad
  # Aqui compartes tus redes sociales
  social:
    github: https://github.com/RustLangES
  # twitter:
  # website:
  ---

  El Contenido de tu articulo
  ```

- Haz una PR con tus cambios
- Espera nuestra revision
- Disfruta de tu articulo publicado 🎊

---

# Desarrollo

## Requisitos

- [Rust](https://rust-lang.org/tools/install)
- [NodeJs](https://nodejs.org)
- [cargo-watch](https://crates.io/crates/cargo-watch)
- [leptosfmt](https://crates.io/crates/leptosfmt)

# Generar la web
```
- npm install

# Instalar Linter 
- cargo install leptosfmt

- cargo watch -x run --shell "npx tailwindcss -i ./input.css -o ./out/output.css && cargo run"

# lanzar un servidor web provicional con python3
- python3 -m http.server -d out

# [alternativa] lanzar un servidor web rústico 😏
- cargo install basic-http-server
- basic-http-server out -a "0.0.0.0:8000"
```

# En cualquier linux distro
```
# Iniciar cargo watch y http server
./server start

# Apagar ambos servicios
./server stop
```

# En cualquier windows
```
## Iniciar cargo watch y http server
./server.bat start

## Apagar ambos servicios
./server.bat stop
```

## Aclaraciones

Si commiteas habra un githook que corra los linters.
Es posible que encuentre errores de formato o mejoras que se pueden hacer.

Para ver estos cambios puedes ejecutar

```
cargo clippy
```

Esto te mostrara algunos cambios que puedes hacer para mejorar el codigo.
Cosas redudantes o que quizás no tengan sentido.

¡Hara tu código más idiomático!

Otro en menor medida podría ser:

```
leptosfmt src
```

Este te formateara el código de forma automática.
Puede llegar a romper algunas cosas de los componentes si se hizo un cambio allí.

## Agradecimientos
Para la generacion de previews se utilizaron
<a target="_blank" href="https://icons8.com/icon/8416/etiqueta-de-precio">Etiqueta de precio</a> icono de <a target="_blank" href="https://icons8.com">Icons8</a>
