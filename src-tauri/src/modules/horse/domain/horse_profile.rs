use super::{HorseDomainError, Sex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorseProfile {
    name: String,
    sex: Option<Sex>,
    breed: Option<String>,
    date_of_birth: Option<String>,
    coat_color: Option<String>,
    identification_number: Option<String>,
    notes: Option<String>,
}

impl HorseProfile {
    pub fn new(
        name: String,
        sex: Option<Sex>,
        breed: Option<String>,
        date_of_birth: Option<String>,
        coat_color: Option<String>,
        identification_number: Option<String>,
        notes: Option<String>,
    ) -> Result<Self, HorseDomainError> {
        let name = name.trim().to_string();

        if name.is_empty() {
            return Err(HorseDomainError::EmptyName);
        }

        if name.chars().count() > 80 {
            return Err(HorseDomainError::NameTooLong);
        }

        Ok(Self {
            name,
            sex,
            breed: normalize_optional_text(breed),
            date_of_birth: normalize_optional_text(date_of_birth),
            coat_color: normalize_optional_text(coat_color),
            identification_number: normalize_optional_text(identification_number),
            notes: normalize_optional_text(notes),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sex(&self) -> Option<Sex> {
        self.sex
    }

    pub fn breed(&self) -> Option<&str> {
        self.breed.as_deref()
    }

    pub fn date_of_birth(&self) -> Option<&str> {
        self.date_of_birth.as_deref()
    }

    pub fn coat_color(&self) -> Option<&str> {
        self.coat_color.as_deref()
    }

    pub fn identification_number(&self) -> Option<&str> {
        self.identification_number.as_deref()
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}
