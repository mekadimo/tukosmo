use leptos::SignalGetUntracked;
use leptos::StoredValue;
use leptos::store_value;
use tukosmo_application::core::shared::dto::DtoDeleteForm;
use tukosmo_domain::core::shared::error;
use tukosmo_domain::core::shared::model::DomainError;

use crate::core::shared::leptos_ui::FormFieldValue;

#[derive(Clone)]
pub struct DeleteForm {
    pub requested: StoredValue<FormFieldValue<bool>>,
}

#[derive(Clone)]
struct DeleteFormInitialValues {
    pub requested: bool,
}

impl DeleteForm {
    pub fn get_dto(&self) -> DtoDeleteForm {
        DtoDeleteForm {
            requested: self.requested.get_value().signal.get_untracked(),
        }
    }

    fn init(initial_values: DeleteFormInitialValues) -> StoredValue<Self> {
        let requested = FormFieldValue::init(
            initial_values.requested,
            Self::validate_mandatory_checkbox
        );

        store_value(Self {
            requested,
        })
    }

    pub fn init_empty() -> StoredValue<Self> {
        let initial_values = DeleteFormInitialValues {
            requested: false,
        };

        Self::init(initial_values)
    }

    pub fn validate(&self) {
        self.requested.get_value().validate();
    }

    pub fn validate_mandatory_checkbox(value: &bool) -> Option<DomainError> {
        if !value {
            return Some(error::FIELD_CANNOT_BE_EMPTY);
        }
        None
    }
}
