use std::{fmt::Display, fs, io};

use chrono::{DateTime, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct FrontMatter {
    pub id: uuid::Uuid,
    pub date: DateTime<Utc>,
    pub draft: Option<bool>,
    pub title: String,
    pub description: String,
}

enum FrontmatterError {
    ParseError(toml::de::Error),
    MissingFrontMatter,
}

impl From<toml::de::Error> for FrontmatterError {
    fn from(value: toml::de::Error) -> Self {
        FrontmatterError::ParseError(value)
    }
}

impl FrontMatter {
    pub fn new(title: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            date: chrono::Utc::now(),
            draft: Some(true),
            title,
            description: String::default(),
        }
    }
    pub fn from_file(file: String) -> Result<Self, FrontmatterError> {
        Ok(deserialize_frontmatter::<Self>(&file)?.0)
    }

    fn get_content(&self) -> Result<String, FrontmatterError> {
        let md = read_post_to_string(&self.title).unwrap_or("Unable to load post.".to_string());
        let content = deserialize_frontmatter::<Self>(&md)?.1;
        Ok(markdown_to_html(&content, &ComrakOptions::default()))
    }
}

impl Display for FrontMatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://mhbtech.dev/blog/{}", self.title);
        let readable_title = self.title.replace("_", " ");
        let content = self
            .get_content()
            .unwrap_or("Unable to load post".to_string());
        write!(
            f,
            r#"
            <entry>
                <title>{}</title>
                <description>{}</description>
                <link rel="alternate" href="{}" type="text/html" title="{}"/>
                <published>{}</published>
                <id>{}</id>
                <content type="html" xml:base="https://mhbtech.dev/blog/{}">{}</content>
                <author>
                <name>botnus</name>
                </author>
            </entry>"#,
            readable_title, self.description, url, self.title, self.date, url, self.title, content
        )
    }
}

pub fn read_post_to_string(post_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(format!("content/blog/{post_name}.md"))
}

pub fn deserialize_frontmatter<T: DeserializeOwned>(
    file_string: &str,
) -> Result<(T, String), FrontmatterError> {
    if !file_string.starts_with("+++") {
        return Err(FrontmatterError::MissingFrontMatter);
    }

    let split_data = file_string
        .split("+++")
        .map(Into::into)
        .collect::<Vec<String>>();

    let frontmatter = match split_data.get(1) {
        Some(f) => Ok(f),
        None => Err(FrontmatterError::MissingFrontMatter),
    }?;

    let content = match split_data.get(2) {
        Some(s) => s.clone(),
        None => String::new(),
    };

    Ok((toml::de::from_str(&frontmatter)?, content))
}
