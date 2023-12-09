use leptos::For;
#[cfg(feature = "ssr")]
use leptos::IntoClass;
use leptos::IntoView;
use leptos::Show;
use leptos::SignalGet;
use leptos::SignalGetUntracked;
use leptos::StoredValue;
use leptos::component;
use leptos::event_target_value;
use leptos::tracing;
use leptos::view;
use tukosmo_domain::core::language::model::Language;

use crate::core::shared::leptos_ui::GlobalContext;
use crate::core::shared::leptos_ui::use_global_context;
use crate::t_error;
use super::super::model::I18nTextFormFieldValue;

// TODO: Improve and simplify code; it's a mess!
#[component]
pub fn TapFormI18nTextField(
    name: Box<dyn Fn() -> String>,
    required: bool,
    value: StoredValue<I18nTextFormFieldValue>
) -> impl IntoView {
    let GlobalContext { current_language_reader, languages_reader, .. } =
        use_global_context();

    let view_translation_field = move |language: Language| {
        let language_id = language.id.value().clone();
        view! {
            <div class="field has-addons is-marginless">
                <div class="control">
                    <span class="button is-static">
                        {move || language.name.translate(current_language_reader.get().id)}
                    </span>
                </div>
                <div class="control is-expanded">
                    <input
                        class="input"
                        class=(
                            "is-danger",
                            move || value.get_value().translation_has_error(&language_id),
                        )
                        on:input=move |event| {
                            let event_value = event_target_value(&event);
                            value.get_value().set_translation(&language_id, event_value);
                        }
                        prop:value=value.get_value().get_translation_signal(&language_id)
                        type="text"
                    />
                    <Show when=move || value.get_value().translation_has_error(&language_id)>
                        <p class="help is-danger">
                            {move || {
                                let domain_error = value.get_value().get_translation_validation_error(&language_id);
                                match domain_error {
                                    Some(domain_error) => {
                                        let full_code = domain_error.get_full_code();
                                        t_error!(&full_code, &domain_error.context)()
                                    },
                                    None => "".to_string(),
                                }
                            }}
                        </p>
                    </Show>
                </div>
            </div>
        }
    };

    view! {
        <div class="field">
            <label class="label">
                {move || name()}
                <Show when=move || required>
                    "*"
                </Show>
            </label>
            <p class="control">
                <div>
                    <div class="field has-addons is-marginless">
                        <div class="control">
                            <span class="button is-static">
                                "*"
                            </span>
                        </div>
                        <div class="control is-expanded">
                            <input
                                class="input"
                                class=(
                                    "is-danger",
                                    move || value.get_value().default_text.get_value().has_error(),
                                )
                                on:input=move |event| {
                                    let event_value = event_target_value(&event);
                                    value.get_value().default_text.get_value().set(event_value);
                                }
                                prop:value=value.get_value().default_text.get_value().signal
                                type="text"
                            />
                            <Show when=move || value.get_value().default_text.get_value().has_error()>
                                <p class="help is-danger">
                                    {move || {
                                        let domain_error = value.get_value().default_text.get_value().get_validation_error();
                                        match domain_error {
                                            Some(domain_error) => {
                                                let full_code = domain_error.get_full_code();
                                                t_error!(&full_code, &domain_error.context)()
                                            },
                                            None => "".to_string(),
                                        }
                                    }}
                                </p>
                            </Show>
                        </div>
                    </div>

                    <For
                        children=view_translation_field
                        each=move || languages_reader.get_untracked()
                        key=|language| language.id.value().to_string()
                    />
                </div>
            </p>
        </div>
    }
}
