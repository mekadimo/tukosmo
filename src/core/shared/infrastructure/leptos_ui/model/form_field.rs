use leptos::create_effect;
use leptos::create_rw_signal;
use leptos::store_value;
use leptos::RwSignal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::SignalUpdate;
use leptos::StoredValue;
use tukosmo_domain::core::shared::model::DomainError;

#[derive(Clone)]
pub struct FormFieldValue<T> where T: Clone + 'static {
    must_be_validated_signal: RwSignal<bool>,
    pub signal: RwSignal<T>,
    validation_error: RwSignal<Option<DomainError>>,
}

impl<T: Clone + 'static> FormFieldValue<T> {
    pub fn get_validation_error(&self) -> Option<DomainError> {
        self.validation_error.get()
    }

    pub fn has_error(&self) -> bool {
        self.must_be_validated_signal.get() &&
            self.validation_error.get().is_some()
    }

    pub fn init(
        initial_value: T,
        validate: fn(&T) -> Option<DomainError>
    ) -> StoredValue<Self> {
        let must_be_validated_signal = create_rw_signal(false);
        let signal = create_rw_signal(initial_value);
        let validation_error = create_rw_signal(None);
        create_effect(move |_| {
            let value = signal.get();
            let error = validate(&value);
            validation_error.set(error.clone());
        });
        store_value(Self {
            must_be_validated_signal,
            signal,
            validation_error,
        })
    }

    pub fn set(&self, new_value: T) {
        self.signal.set(new_value);
        self.must_be_validated_signal.set(true);
    }

    pub fn set_validation_error(&self, domain_error: DomainError) {
        self.validation_error.set(Some(domain_error));
    }

    pub fn update(&self, update_function: impl FnOnce(&mut T)) {
        self.signal.update(update_function);
        self.must_be_validated_signal.set(true);
    }

    pub fn validate(&self) {
        self.must_be_validated_signal.set(true);
    }
}
