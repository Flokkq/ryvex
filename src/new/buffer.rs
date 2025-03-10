use crate::core::error;
use std::path::PathBuf;

use crate::file_access::FileAccess;

use super::iter::BufferContent;

struct Buffer {
    content: BufferContent,
    path: Option<PathBuf>,
}

impl Buffer {
    pub fn scratch() -> Self {
        Self {
            content: BufferContent::new(String::new()),
            path: None,
        }
    }

    pub fn open(path: PathBuf) -> Result<Self, error::Error> {
        let mut content = String::new();
        FileAccess::read_from_file_if_exists(&path, &mut content)?;

        Ok(Self {
            content: BufferContent::new(content),
            path: Some(path),
        })
    }

    pub fn save(&mut self) -> Result<(), error::Error> {
        match &self.path {
            Some(path) => {
                FileAccess::write_to_file(&path, self.content.inner())
            }

            // No filename
            None => Err(error::Error::Unexpected),
        }
    }

    pub fn save_to(&mut self, path: PathBuf) -> Result<(), error::Error> {
        if path.exists() {
            // File exists (add ! to override)
            return Err(error::Error::Unexpected);
        }

        self.path = Some(path);
        self.save()
    }

    pub fn save_to_forced(
        &mut self,
        path: PathBuf,
    ) -> Result<(), error::Error> {
        self.path = Some(path);
        self.save()
    }
}
