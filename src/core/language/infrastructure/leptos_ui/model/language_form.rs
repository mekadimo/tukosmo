use leptos::SignalGetUntracked;
use leptos::StoredValue;
use leptos::store_value;
use tukosmo_application::core::language::dto::DtoLanguageForm;
use tukosmo_domain::core::language::error;
use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::language::model::LanguageCode;
use tukosmo_domain::core::language::model::LanguageOriginalName;
use tukosmo_domain::core::language::model::LanguageWebsiteSubtitle;
use tukosmo_domain::core::language::model::LanguageWebsiteTitle;

use crate::core::shared::leptos_ui::FormFieldValue;
use super::I18nTextFormFieldInitialValue;
use super::I18nTextFormFieldValue;

#[derive(Clone)]
pub struct LanguageForm {
    pub code: StoredValue<FormFieldValue<String>>,
    pub name: StoredValue<I18nTextFormFieldValue>,
    pub original_name: StoredValue<FormFieldValue<String>>,
    pub website_subtitle: StoredValue<FormFieldValue<String>>,
    pub website_title: StoredValue<FormFieldValue<String>>,
}

#[derive(Clone)]
struct LanguageFormInitialValues {
    pub code: String,
    pub name: I18nTextFormFieldInitialValue,
    pub original_name: String,
    pub website_subtitle: String,
    pub website_title: String,
}

impl LanguageForm {
    pub fn get_dto(&self) -> DtoLanguageForm {
        DtoLanguageForm {
            code: self.code.get_value().signal.get_untracked(),
            name: self.name.get_value().get_i18n_text_value(),
            original_name: self.original_name
                .get_value()
                .signal.get_untracked(),
            website_title: self.website_title
                .get_value()
                .signal.get_untracked(),
            website_subtitle: self.website_subtitle
                .get_value()
                .signal.get_untracked(),
        }
    }

    fn init(initial_values: LanguageFormInitialValues) -> StoredValue<Self> {
        let code = FormFieldValue::init(
            initial_values.code,
            LanguageCode::validate
        );
        let original_name = FormFieldValue::init(
            initial_values.original_name,
            LanguageOriginalName::validate
        );
        let name = I18nTextFormFieldValue::init(
            initial_values.name,
            Language::validate_name_default_value,
            Language::validate_name_translation_value
        );
        let website_title = FormFieldValue::init(
            initial_values.website_title,
            LanguageWebsiteTitle::validate
        );
        let website_subtitle = FormFieldValue::init(
            initial_values.website_subtitle,
            LanguageWebsiteSubtitle::validate
        );

        store_value(Self {
            code,
            name,
            original_name,
            website_subtitle,
            website_title,
        })
    }

    pub fn init_empty() -> StoredValue<Self> {
        let initial_values = LanguageFormInitialValues {
            code: "".to_string(),
            name: I18nTextFormFieldInitialValue::empty(),
            original_name: "".to_string(),
            website_subtitle: "".to_string(),
            website_title: "".to_string(),
        };

        Self::init(initial_values)
    }

    pub fn init_filled(language: Language) -> StoredValue<Self> {
        let initial_values = LanguageFormInitialValues {
            code: language.code.value().to_string(),
            name: I18nTextFormFieldInitialValue::filled(language.name),
            original_name: language.original_name.value().to_string(),
            website_subtitle: language.website_subtitle.value().to_string(),
            website_title: language.website_title.value().to_string(),
        };

        Self::init(initial_values)
    }

    pub fn validate(&self, server_error_code: &str) {
        if
            server_error_code ==
            &error::LANGUAGE_CODE_ALREADY_EXISTS.get_full_code()
        {
            self.code
                .get_value()
                .set_validation_error(error::LANGUAGE_CODE_ALREADY_EXISTS);
        } else {
            self.code.get_value().validate();
        }

        self.original_name.get_value().validate();
        self.name.get_value().validate();
        self.website_title.get_value().validate();
        self.website_subtitle.get_value().validate();
    }
}
