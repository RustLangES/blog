Experiments with Leptos and SSR / SSG.

I provided some tools to:
- inject tags into `<head>`
- have top-level async components
- clean Leptos SSR output by removing extra tags since we won't hydrate the app

This is based on the `islands` branch of Leptos, that by default won't generate
hydration ids.


## Run

```
# generate your site
cargo watch -x run --shell "npx tailwindcss -i ./input.css -o ./out/output.css && cargo run"

# launch a webserver
python3 -m http.server -d out 

# open your browser
open http://locahost:3000
```
