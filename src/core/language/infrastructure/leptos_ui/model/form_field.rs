use leptos::RwSignal;
use leptos::SignalGetUntracked;
use leptos::StoredValue;
use leptos::store_value;
use tukosmo_domain::core::language::model::I18nText;
use tukosmo_domain::core::language::model::I18nTextValue;
use tukosmo_domain::core::shared::model::DomainError;
use uuid::Uuid;

use crate::core::shared::leptos_ui::FormFieldValue;
use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::use_global_context;

#[derive(Clone)]
pub struct I18nTextFormFieldInitialValue {
    pub default_text: String,
    pub translations: Vec<(Uuid, String)>,
}

#[derive(Clone)]
pub struct I18nTextFormFieldValue {
    pub default_text: StoredValue<FormFieldValue<String>>,
    pub translations: Vec<(Uuid, StoredValue<FormFieldValue<String>>)>,
}

impl I18nTextFormFieldValue {
    pub fn get_i18n_text_value(&self) -> I18nTextValue {
        let default_text = self.default_text.get_value().signal.get_untracked();
        let translations = self.translations
            .iter()
            .map(|(language_id, value)| {
                let text_value = value.get_value().signal.get_untracked();
                (language_id.clone(), text_value)
            })
            .collect();

        I18nTextValue {
            default_text,
            translations,
        }
    }

    pub fn get_translation_signal(
        &self,
        language_id: &Uuid
    ) -> RwSignal<String> {
        let (_, stored_value) = self.translations
            .iter()
            .find(|(uuid, _)| uuid == language_id)
            .unwrap();
        stored_value.get_value().signal
    }

    pub fn get_translation_validation_error(
        &self,
        language_id: &Uuid
    ) -> Option<DomainError> {
        let (_, stored_value) = self.translations
            .iter()
            .find(|(uuid, _)| uuid == language_id)
            .unwrap();
        stored_value.get_value().get_validation_error()
    }

    pub fn init(
        initial_value: I18nTextFormFieldInitialValue,
        validate_default_text: fn(&String) -> Option<DomainError>,
        validate_translation: fn(&String) -> Option<DomainError>
    ) -> StoredValue<Self> {
        let default_text = FormFieldValue::init(
            initial_value.default_text,
            validate_default_text
        );

        let translations = initial_value.translations
            .iter()
            .map(|(language_id, text_value)| {
                (
                    language_id.clone(),
                    FormFieldValue::init(
                        text_value.clone(),
                        validate_translation
                    ),
                )
            })
            .collect();

        store_value(Self {
            default_text,
            translations,
        })
    }

    pub fn set_translation(&self, language_id: &Uuid, new_value: String) {
        let (_, stored_value) = self.translations
            .iter()
            .find(|(uuid, _)| uuid == language_id)
            .unwrap();
        stored_value.get_value().set(new_value);
    }

    pub fn translation_has_error(&self, language_id: &Uuid) -> bool {
        let (_, stored_value) = self.translations
            .iter()
            .find(|(uuid, _)| uuid == language_id)
            .unwrap();
        stored_value.get_value().has_error()
    }

    pub fn validate(&self) {
        self.default_text.get_value().validate();
        for (_, translation) in self.translations.iter() {
            translation.get_value().validate();
        }
    }
}

impl I18nTextFormFieldInitialValue {
    pub fn empty() -> Self {
        let GlobalContext { languages_reader, .. } = use_global_context();
        let available_languages = languages_reader.get_untracked();

        let initial_translation_values = available_languages
            .iter()
            .map(|available_language| {
                let available_language_id = available_language.id
                    .value()
                    .clone();
                (available_language_id.clone(), "".to_string())
            })
            .collect();

        Self {
            default_text: "".to_string(),
            translations: initial_translation_values,
        }
    }

    pub fn filled(i18n_text: I18nText) -> Self {
        let GlobalContext { languages_reader, .. } = use_global_context();
        let available_languages = languages_reader.get_untracked();

        let initial_translation_values = available_languages
            .iter()
            .map(|available_language| {
                let available_language_id = available_language.id
                    .value()
                    .clone();
                let translation = i18n_text.translations
                    .iter()
                    .find(|t| {
                        t.language_id.value() == &available_language_id
                    });
                let text_value = match translation {
                    Some(t) => t.text.value().to_string(),
                    None => "".to_string(),
                };
                (available_language_id.clone(), text_value)
            })
            .collect();

        Self {
            default_text: i18n_text.default_text.value().to_string(),
            translations: initial_translation_values,
        }
    }
}
