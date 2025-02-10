use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct FrontMatter {
    id: uuid::Uuid,
    date: DateTime<Utc>,
    draft: Option<bool>,
    title: String,
    description: String,
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

    fn get_content(&self) -> Result<String, FrontmatterError> {}
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
