use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub line: Option<usize>,
    pub text: String,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(line) = self.line.as_ref() {
                format!("L{}: ", line)
            } else {
                String::new()
            },
            self.text,
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Report {
    pub entries: HashMap<PathBuf, Vec<Entry>>,
}

impl Report {
    pub fn add_entry<P: Into<PathBuf>, S: Into<String>>(
        &mut self,
        file: P,
        line: Option<usize>,
        text: S,
    ) {
        self.entries
            .entry(file.into())
            .or_insert_with(|| vec![])
            .push(Entry {
                line,
                text: text.into(),
            });
    }

    pub fn sort_entries(&mut self) {
        self.entries.iter_mut().for_each(|(_, entries)| {
            entries.sort_by(|a, b| {
                a.line
                    .unwrap_or_else(|| 0)
                    .cmp(&b.line.unwrap_or_else(|| 0))
            });
        });
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (path, entries)) in self.entries.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            writeln!(f, "{}:", path.to_string_lossy())?;

            for entry in entries.iter() {
                writeln!(f, "\t{entry}")?;
            }
        }

        Ok(())
    }
}
