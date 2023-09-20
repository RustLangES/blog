![image](https://github.com/RustLangES/blog/assets/56278796/ba1ac759-3fda-4983-80d2-965398bf8d35)

<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/blog/ci.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/blog/deploy.yml?label=deploy" />
</p>

## 🤝🏼 Agrega tu articulo

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

## Ejecutar

```
# generar la web
cargo watch -x run --shell "npx tailwindcss -i ./input.css -o ./out/output.css && cargo run"

# lanzar un servidor web provicional con python3
python3 -m http.server -d out
```

```
# En cualquier linux distro

## Iniciar cargo watch y http server
./server start

## Apagar ambos servicios
./server stop
```
