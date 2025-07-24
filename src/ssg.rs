use std::path::Path;
use tokio::fs;

use leptos::{provide_context, IntoView};

use crate::render;

pub struct Ssg<'a> {
    out_dir: &'a Path,
}

impl<'a> Ssg<'a> {
    #[must_use]
    pub fn new(out_dir: &'a Path) -> Self {
        Self { out_dir }
    }

    /// # Errors
    /// This can return an error if `fs::write` fails.
    pub async fn gen<F, V>(
        &'a self,
        path: String,
        view: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce() -> V + 'static,
        V: IntoView,
    {
        // SsgContext will be available to all components in the view
        let ssg_ctx = SsgContext { path: path.clone() };

        // Render the view to a string
        let res =
            render::render(move || view().into_view(), move || provide_context(ssg_ctx)).await;

        // Write the string to a file
        let out_file = self.out_dir.join(path);
        fs::write(&out_file, res).await?;
        println!("wrote {}", out_file.display());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SsgContext {
    pub path: String,
}
