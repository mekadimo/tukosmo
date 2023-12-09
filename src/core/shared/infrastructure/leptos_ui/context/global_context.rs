use leptos::ReadSignal;
use leptos::SignalSet;
use leptos::SignalWith;
use leptos::WriteSignal;
use leptos::create_signal;
use leptos::use_context;

use tukosmo_domain::core::language::model::Language;
use tukosmo_domain::core::shared::model::LocalI18n;

#[derive(Copy, Clone, Debug)]
pub struct GlobalContext {
    pub current_language_reader: ReadSignal<Language>,
    current_language_writer: WriteSignal<Language>,
    pub current_local_i18n_reader: ReadSignal<LocalI18n>,
    current_local_i18n_writer: WriteSignal<LocalI18n>,
    pub languages_reader: ReadSignal<Vec<Language>>,
    languages_writer: WriteSignal<Vec<Language>>,
    pub loading_top_bar_enabled_reader: ReadSignal<bool>,
    pub loading_top_bar_enabled_writer: WriteSignal<bool>,
    pub website_subtitle_reader: ReadSignal<String>,
    website_subtitle_writer: WriteSignal<String>,
    pub website_title_reader: ReadSignal<String>,
    website_title_writer: WriteSignal<String>,
}

impl GlobalContext {
    pub fn change_language(
        &self,
        new_current_language: Language,
        local_i18n: LocalI18n
    ) {
        self.website_subtitle_writer.set(
            new_current_language.website_subtitle.value().to_string()
        );
        self.website_title_writer.set(
            new_current_language.website_title.value().to_string()
        );
        self.current_language_writer.set(new_current_language);
        self.current_local_i18n_writer.set(local_i18n);
    }

    pub fn init(
        language_code: &str,
        languages: Vec<Language>,
        local_i18n: LocalI18n
    ) -> Self {
        let current_language: Language = languages
            .iter()
            .find(|l| l.code.value() == language_code)
            // TODO: Raise 404 error if language_code is not in the list
            .unwrap()
            .clone();

        let website_subtitle = current_language.website_subtitle
            .value()
            .to_string();
        let website_title = current_language.website_title.value().to_string();

        let (current_language_reader, current_language_writer) =
            create_signal(current_language);
        let (current_local_i18n_reader, current_local_i18n_writer) =
            create_signal(local_i18n);
        let (languages_reader, languages_writer) = create_signal(languages);
        let (website_subtitle_reader, website_subtitle_writer) =
            create_signal(website_subtitle);
        let (website_title_reader, website_title_writer) =
            create_signal(website_title);

        let (loading_top_bar_enabled_reader, loading_top_bar_enabled_writer) =
            create_signal(false);

        GlobalContext {
            current_language_reader,
            current_language_writer,
            current_local_i18n_reader,
            current_local_i18n_writer,
            languages_reader,
            languages_writer,
            loading_top_bar_enabled_reader,
            loading_top_bar_enabled_writer,
            website_subtitle_reader,
            website_subtitle_writer,
            website_title_reader,
            website_title_writer,
        }
    }

    pub fn refresh_languages(
        &self,
        current_language_code: Option<String>,
        languages: Vec<Language>
    ) {
        let current_language: Language = match current_language_code {
            Some(language_code) => {
                languages
                    .iter()
                    .find(|l| l.code.value() == &language_code)
                    // TODO: Raise error if language_code is not in the list;
                    // or, maybe, try to redirect using the default language code
                    // (at this point, we can be sure the current language was deleted)
                    .unwrap()
                    .clone()
            }
            None => languages.first().unwrap().clone(),
        };

        self.website_subtitle_writer.set(
            current_language.website_subtitle.value().to_string()
        );
        self.website_title_writer.set(
            current_language.website_title.value().to_string()
        );
        self.current_language_writer.set(current_language);
        self.languages_writer.set(languages);
    }
}

pub fn local_translation_callback<T>(
    parse: impl Fn(&LocalI18n) -> T
) -> impl Fn() -> T {
    let GlobalContext { current_local_i18n_reader, .. } = use_global_context();
    let local_translation = move ||
        current_local_i18n_reader.with(|x| parse(x));
    local_translation
}

#[macro_export]
macro_rules! t {
    ($($field_path:ident).+) => {
        crate::core::shared::leptos_ui::local_translation_callback(
            // TODO: Avoid using clone to improve efficiency
            |local_i18n| local_i18n.$($field_path).+.clone()
        )
    };
}

#[macro_export]
macro_rules! t_date_long {
    ($expr:expr) => {
        crate::core::shared::leptos_ui::local_translation_callback(
            move |local_i18n| local_i18n.date.date_long.with(&local_i18n.date, $expr)
        )
    };
}

#[macro_export]
macro_rules! t_date_short {
    ($expr:expr) => {
        crate::core::shared::leptos_ui::local_translation_callback(
            move |local_i18n| local_i18n.date.date_short.with(&local_i18n.date, $expr)
        )
    };
}

#[macro_export]
macro_rules! t_error {
    ($code:expr, $context:expr) => {
        crate::core::shared::leptos_ui::local_translation_callback(
            move |local_i18n| local_i18n.get_error_message($code, $context)
        )
    };
}

pub fn use_global_context() -> GlobalContext {
    use_context::<GlobalContext>().unwrap()
}
