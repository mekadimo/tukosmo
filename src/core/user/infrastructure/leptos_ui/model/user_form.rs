use leptos::SignalGetUntracked;
use leptos::StoredValue;
use leptos::store_value;
use tukosmo_application::core::user::dto::DtoAddUserForm;
use tukosmo_application::core::user::dto::DtoChangeUserPasswordForm;
use tukosmo_application::core::user::dto::DtoEditUserForm;
use tukosmo_domain::core::user::error;
use tukosmo_domain::core::user::model::User;
use tukosmo_domain::core::user::model::UserEmail;
use tukosmo_domain::core::user::model::UserIsAdmin;
use tukosmo_domain::core::user::model::UserIsSuspended;
use tukosmo_domain::core::user::model::UserPlaintextPassword;

use crate::core::language::leptos_ui::I18nTextFormFieldInitialValue;
use crate::core::language::leptos_ui::I18nTextFormFieldValue;
use crate::core::shared::leptos_ui::FormFieldValue;

#[derive(Clone)]
pub struct AddUserForm {
    pub email: StoredValue<FormFieldValue<String>>,
    pub is_admin: StoredValue<FormFieldValue<bool>>,
    pub is_suspended: StoredValue<FormFieldValue<bool>>,
    pub name: StoredValue<I18nTextFormFieldValue>,
    pub plaintext_password: StoredValue<FormFieldValue<String>>,
    pub plaintext_password_repeated: StoredValue<FormFieldValue<String>>,
}

#[derive(Clone)]
pub struct AddUserFormInitialValues {
    pub email: String,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub name: I18nTextFormFieldInitialValue,
    pub plaintext_password: String,
    pub plaintext_password_repeated: String,
}

#[derive(Clone)]
pub struct ChangeUserPasswordForm {
    pub plaintext_new_password: StoredValue<FormFieldValue<String>>,
    pub plaintext_new_password_repeated: StoredValue<FormFieldValue<String>>,
    pub plaintext_old_password: StoredValue<FormFieldValue<String>>,
}

#[derive(Clone)]
pub struct ChangeUserPasswordFormInitialValues {
    pub plaintext_new_password: String,
    pub plaintext_new_password_repeated: String,
    pub plaintext_old_password: String,
}

#[derive(Clone)]
pub struct EditUserForm {
    pub email: StoredValue<FormFieldValue<String>>,
    pub is_admin: StoredValue<FormFieldValue<bool>>,
    pub is_suspended: StoredValue<FormFieldValue<bool>>,
    pub name: StoredValue<I18nTextFormFieldValue>,
}

#[derive(Clone)]
pub struct EditUserFormInitialValues {
    pub email: String,
    pub is_admin: bool,
    pub is_suspended: bool,
    pub name: I18nTextFormFieldInitialValue,
}

impl AddUserForm {
    pub fn get_dto(&self) -> DtoAddUserForm {
        DtoAddUserForm {
            email: self.email.get_value().signal.get_untracked(),
            is_admin: self.is_admin.get_value().signal.get_untracked(),
            is_suspended: self.is_suspended.get_value().signal.get_untracked(),
            name: self.name.get_value().get_i18n_text_value(),
            plaintext_password: self.plaintext_password
                .get_value()
                .signal.get_untracked(),
            plaintext_password_repeated: self.plaintext_password_repeated
                .get_value()
                .signal.get_untracked(),
        }
    }

    fn init(initial_values: AddUserFormInitialValues) -> StoredValue<Self> {
        let email = FormFieldValue::init(
            initial_values.email,
            UserEmail::validate
        );
        let name = I18nTextFormFieldValue::init(
            initial_values.name,
            User::validate_name_default_value,
            User::validate_name_translation_value
        );
        let is_admin = FormFieldValue::init(
            initial_values.is_admin,
            UserIsAdmin::validate
        );
        let is_suspended = FormFieldValue::init(
            initial_values.is_suspended,
            UserIsSuspended::validate
        );
        let plaintext_password = FormFieldValue::init(
            initial_values.plaintext_password,
            UserPlaintextPassword::validate
        );
        let plaintext_password_repeated = FormFieldValue::init(
            initial_values.plaintext_password_repeated,
            UserPlaintextPassword::validate
        );

        store_value(Self {
            email,
            is_admin,
            is_suspended,
            name,
            plaintext_password,
            plaintext_password_repeated,
        })
    }

    pub fn init_empty() -> StoredValue<Self> {
        let initial_values = AddUserFormInitialValues {
            email: "".to_string(),
            is_admin: false,
            is_suspended: false,
            name: I18nTextFormFieldInitialValue::empty(),
            plaintext_password: "".to_string(),
            plaintext_password_repeated: "".to_string(),
        };

        Self::init(initial_values)
    }

    pub fn validate(&self, server_error_code: &str) {
        if
            server_error_code ==
            &error::USER_EMAIL_ALREADY_EXISTS.get_full_code()
        {
            self.email
                .get_value()
                .set_validation_error(error::USER_EMAIL_ALREADY_EXISTS);
        } else {
            self.email.get_value().validate();
        }

        self.name.get_value().validate();
        self.plaintext_password.get_value().validate();
        self.plaintext_password_repeated.get_value().validate();
        if
            self.plaintext_password.get_value().signal.get_untracked() !=
            self.plaintext_password_repeated.get_value().signal.get_untracked()
        {
            self.plaintext_password_repeated
                .get_value()
                .set_validation_error(
                    error::USER_REPEATED_PASSWORD_DOES_NOT_MATCH
                );
        } else {
            self.plaintext_password_repeated.get_value().validate();
        }
        self.is_admin.get_value().validate();
        self.is_suspended.get_value().validate();
    }
}

impl ChangeUserPasswordForm {
    pub fn get_dto(&self) -> DtoChangeUserPasswordForm {
        DtoChangeUserPasswordForm {
            plaintext_new_password: self.plaintext_new_password
                .get_value()
                .signal.get_untracked(),
            plaintext_new_password_repeated: self.plaintext_new_password_repeated
                .get_value()
                .signal.get_untracked(),
            plaintext_old_password: self.plaintext_old_password
                .get_value()
                .signal.get_untracked(),
        }
    }

    fn init(
        initial_values: ChangeUserPasswordFormInitialValues
    ) -> StoredValue<Self> {
        let plaintext_new_password = FormFieldValue::init(
            initial_values.plaintext_new_password,
            UserPlaintextPassword::validate
        );
        let plaintext_new_password_repeated = FormFieldValue::init(
            initial_values.plaintext_new_password_repeated,
            UserPlaintextPassword::validate
        );
        let plaintext_old_password = FormFieldValue::init(
            initial_values.plaintext_old_password,
            UserPlaintextPassword::validate
        );

        store_value(Self {
            plaintext_new_password,
            plaintext_new_password_repeated,
            plaintext_old_password,
        })
    }

    pub fn init_empty() -> StoredValue<Self> {
        let initial_values = ChangeUserPasswordFormInitialValues {
            plaintext_new_password: "".to_string(),
            plaintext_new_password_repeated: "".to_string(),
            plaintext_old_password: "".to_string(),
        };

        Self::init(initial_values)
    }

    pub fn validate(&self) {
        self.plaintext_old_password.get_value().validate();
        self.plaintext_new_password.get_value().validate();
        if
            self.plaintext_new_password.get_value().signal.get_untracked() !=
            self.plaintext_new_password_repeated
                .get_value()
                .signal.get_untracked()
        {
            self.plaintext_new_password_repeated
                .get_value()
                .set_validation_error(
                    error::USER_REPEATED_PASSWORD_DOES_NOT_MATCH
                );
        } else {
            self.plaintext_new_password_repeated.get_value().validate();
        }
    }
}

impl EditUserForm {
    pub fn get_dto(&self) -> DtoEditUserForm {
        DtoEditUserForm {
            email: self.email.get_value().signal.get_untracked(),
            is_admin: self.is_admin.get_value().signal.get_untracked(),
            is_suspended: self.is_suspended.get_value().signal.get_untracked(),
            name: self.name.get_value().get_i18n_text_value(),
        }
    }

    fn init(initial_values: EditUserFormInitialValues) -> StoredValue<Self> {
        let email = FormFieldValue::init(
            initial_values.email,
            UserEmail::validate
        );
        let name = I18nTextFormFieldValue::init(
            initial_values.name,
            User::validate_name_default_value,
            User::validate_name_translation_value
        );
        let is_admin = FormFieldValue::init(
            initial_values.is_admin,
            UserIsAdmin::validate
        );
        let is_suspended = FormFieldValue::init(
            initial_values.is_suspended,
            UserIsSuspended::validate
        );

        store_value(Self {
            email,
            is_admin,
            is_suspended,
            name,
        })
    }

    pub fn init_filled(user: User) -> StoredValue<Self> {
        let initial_values = EditUserFormInitialValues {
            email: user.email.value().to_string(),
            is_admin: user.is_admin.value().clone(),
            is_suspended: user.is_admin.value().clone(),
            name: I18nTextFormFieldInitialValue::filled(user.name),
        };

        Self::init(initial_values)
    }

    pub fn validate(&self, server_error_code: &str) {
        if
            server_error_code ==
            &error::USER_EMAIL_ALREADY_EXISTS.get_full_code()
        {
            self.email
                .get_value()
                .set_validation_error(error::USER_EMAIL_ALREADY_EXISTS);
        } else {
            self.email.get_value().validate();
        }

        self.name.get_value().validate();
        self.is_admin.get_value().validate();
        self.is_suspended.get_value().validate();
    }
}
