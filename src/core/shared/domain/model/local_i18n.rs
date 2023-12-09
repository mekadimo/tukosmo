use std::collections::HashMap;

use chrono::DateTime;
use chrono::Datelike;
use chrono::Timelike;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

// TODO: Research a good i18n system better than what we implemented here.
// If it's well maintained, we may migrate to it.

pub const DEFAULT_LANGUAGE_CODE: &'static str = "en";
const SUPPORTED_LANGUAGE_CODES: [&'static str; 2] = ["en", "es"];

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18n {
    pub bytesize: LocalI18nByteSize,
    pub date: LocalI18nDate,
    pub duration: LocalI18nDuration,
    pub error: HashMap<String, String>,
    pub language: LocalI18nLanguage,
    pub main: LocalI18nMain,
}

impl LocalI18n {
    pub fn get_applied_language_code(language_code: &str) -> String {
        if SUPPORTED_LANGUAGE_CODES.iter().any(|&code| code == language_code) {
            language_code.to_string()
        } else {
            DEFAULT_LANGUAGE_CODE.to_string()
        }
    }

    pub fn get_error_message(
        &self,
        error_code: &String,
        context: &Vec<(String, String)>
    ) -> String {
        match self.error.get(error_code) {
            Some(message) => {
                let mut formatted_message = message.to_string();
                for (key, value) in context {
                    let placeholder = format!("{{{}}}", key);
                    formatted_message = formatted_message.replace(
                        &placeholder,
                        value
                    );
                }
                formatted_message
            }
            None => self.error.get("UNKNOWN").unwrap().to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18nByteSize {
    pub decimal_separator: String,
    pub digit_separator: String,
    pub digits_by_group: u64,
}

impl LocalI18nByteSize {
    pub fn with(&self, nbytes: u64) -> String {
        // TODO: Move number formatting to its own struct, and use it here
        let (full_number, unit): (f64, String) = if nbytes < 1024 {
            (nbytes as f64, "bytes".to_string())
        } else if nbytes < 1048576 {
            ((nbytes as f64) / 1024.0, "KiB".to_string())
        } else if nbytes < 1073741824 {
            ((nbytes as f64) / 1048576.0, "MiB".to_string())
        } else {
            ((nbytes as f64) / 1073741824.0, "GiB".to_string())
        };

        let mut formatted_number: String = "".to_owned();
        let integer_part = full_number.floor() as u64;
        let integer_part_string = integer_part.to_string();
        let number_of_integer_digits = integer_part_string.len();
        let digits_by_group = self.digits_by_group as usize;
        let total_reminder = number_of_integer_digits % digits_by_group;
        for (index, digit) in integer_part_string.chars().enumerate() {
            formatted_number.push_str(&digit.to_string());
            let position = index + 1;
            let is_not_last_digit = position != number_of_integer_digits;
            let position_remainder = position % digits_by_group;
            let separator_comes_after = total_reminder == position_remainder;
            if separator_comes_after && is_not_last_digit {
                formatted_number.push_str(&self.digit_separator);
            }
        }

        let decimal_part = full_number - full_number.floor();
        if 0.0 != decimal_part {
            formatted_number.push_str(&self.decimal_separator);
            let decimal_part_str = &decimal_part.to_string();
            formatted_number.push_str(&decimal_part_str[2..4]);
        }

        format!("{} {}", formatted_number, unit)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18nDate {
    pub april: String,
    pub august: String,
    pub date_long: TStringDateLong,
    pub date_short: TStringDateShort,
    pub december: String,
    pub february: String,
    pub january: String,
    pub july: String,
    pub june: String,
    pub march: String,
    pub may: String,
    pub november: String,
    pub october: String,
    pub september: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18nDuration {
    pub day: String,
    pub days: String,
    pub duration: TStringDuration,
    pub hour: String,
    pub hours: String,
    pub minute: String,
    pub minutes: String,
    pub month: String,
    pub months: String,
    pub second: String,
    pub seconds: String,
    pub week: String,
    pub weeks: String,
    pub year: String,
    pub years: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18nLanguage {
    pub code: String,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LocalI18nMain {
    pub account: String,
    pub account_status: String,
    pub actions: String,
    pub active_w_account: String,
    pub add: String,
    pub add_language: String,
    pub add_tag: String,
    pub administration_panel: String,
    pub administrator: String,
    pub administrators_email: String,
    pub all_stats_of_pages_in_this_language: String,
    pub all_translations_in_this_language: String,
    pub any_other_data_related_to_this_language: String,
    pub appearance: String,
    pub author: String,
    pub blog: String,
    pub browser: String,
    pub cancel: String,
    pub change_domain: String,
    pub choose_a_file: String,
    pub close: String,
    pub code: String,
    pub copyright: TStringCopyright,
    pub cpu: String,
    pub cpu_current_usage: String,
    pub create_tag: String,
    pub create_user: String,
    pub current_favicon: String,
    pub current_password: String,
    pub current_size: String,
    pub dashboard: String,
    pub data: String,
    pub database: String,
    pub default_name: String,
    pub delete: String,
    pub delete_file: String,
    pub delete_language: String,
    pub delete_language_name: TStringWithName,
    pub delete_tag_name: TStringWithName,
    pub description: String,
    pub desire_great_day: String,
    pub disk_at_percentage: TStringWithDecimalNumber,
    pub disk_used_info: TStringDiskUsedInfo,
    pub domain_successfully_updated: String,
    pub domain_w_web: String,
    pub download_w_verb: String,
    pub downloads: String,
    pub drafts: String,
    pub edit_file: String,
    pub edit_language: String,
    pub edit_tag: String,
    pub edit_user: String,
    pub email: String,
    pub enabled_modules: String,
    pub export_backup: String,
    pub faq: String,
    pub favicon: String,
    pub favicon_note: String,
    pub file: String,
    pub files: String,
    pub forgotten_password_w_question: String,
    pub gallery: String,
    pub go_back_to_website: TStringWithName,
    pub hello_user: TStringWithName,
    pub help: String,
    pub i_understand_the_consequences_of_performing_this_action: String,
    pub if_you_delete_this_language_you_will_lose: String,
    pub import_backup: String,
    pub in_the_last_month: String,
    pub kernel_version: String,
    pub language: String,
    pub languages: String,
    pub last_update: String,
    pub login_w_noun: String,
    pub login_w_verb: String,
    pub logout_w_verb: String,
    pub management_system: String,
    pub memory: String,
    pub menu: String,
    pub menus: String,
    pub modules: String,
    pub n_results_of_m: TStringNResultsOfM,
    pub n_visitors: TStringWithIntegerNumber,
    pub name: String,
    pub name_in_each_language: String,
    pub new_password: String,
    pub new_password_repeat: String,
    pub next_w_page: String,
    pub no_file_uploaded: String,
    pub one_result_of_m: TStringWithIntegerNumber,
    pub operating_system: String,
    pub original_name: String,
    pub page_n: TStringWithIntegerNumber,
    pub pages: String,
    pub password: String,
    pub password_repeat: String,
    pub payments: String,
    pub permalink_identifier: String,
    pub platform_w_os: String,
    pub posts: String,
    pub previous_w_page: String,
    pub published_w_posts: String,
    pub remove: String,
    pub see_languages: String,
    pub select_a_language: String,
    pub server: String,
    pub server_os: TStringWithName,
    pub sessions: String,
    pub settings: String,
    pub shop: String,
    pub since: String,
    pub size: String,
    pub statistics: String,
    pub status: String,
    pub submit: String,
    pub subscriptions: String,
    pub suspended_account: String,
    pub suspended_w_account: String,
    pub tags: String,
    pub tasks: String,
    pub the_file_has_been_successfully_updated: String,
    pub the_language_has_been_successfully_updated: String,
    pub the_session_has_been_successfully_deleted: String,
    pub the_tag_has_been_successfully_deleted: String,
    pub the_tag_has_been_successfully_updated: String,
    pub the_user_has_been_successfully_updated: String,
    pub the_websites_favicon_has_been_successfully_updated: String,
    pub theme: String,
    pub this_action_is_irreversible: String,
    pub title: String,
    pub trash_w_bin: String,
    pub tukosmo: String,
    pub tukosmo_admin_panel: String,
    pub tukosmo_settings_successfully_updated: String,
    pub tukosmo_version: TStringWithName,
    pub tukosmo_will_automatically_restart_itself_apply_changes: String,
    pub untranslated: String,
    pub update_os: String,
    pub update_packages: String,
    pub updated_w_server: String,
    pub updated_w_tukosmo: String,
    pub upload_file: String,
    pub upload_new_favicon_png_image: String,
    pub uptime: String,
    pub users: String,
    pub visit_website: String,
    pub warning_domain_page: String,
    pub web_browsers_preview: String,
    pub website: String,
    pub website_subtitle: String,
    pub website_subtitle_in_this_language: String,
    pub website_title: String,
    pub website_title_in_this_language: String,
    pub widgets: String,
    pub your_account_has_been_successfully_updated: String,
    pub your_current_domain_is: TStringWithName,
    pub your_email: String,
    pub your_password: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringCopyright(String);

impl TStringCopyright {
    pub fn with(&self, year: i64, name: &str) -> String {
        let year_str = &year.to_string();
        self.0.replace("{year}", year_str).replace("{name}", name)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringDateLong(String);

impl TStringDateLong {
    pub fn with(
        &self,
        t_date: &LocalI18nDate,
        date_value: &DateTime<Utc>
    ) -> String {
        let year = date_value.year();
        let month = date_value.month();
        let day = date_value.day();
        let hour = date_value.hour();
        let minute = date_value.minute();
        let timezone = date_value.timezone();

        let month_name = match month {
            1 => &t_date.january,
            2 => &t_date.february,
            3 => &t_date.march,
            4 => &t_date.april,
            5 => &t_date.may,
            6 => &t_date.june,
            7 => &t_date.july,
            8 => &t_date.august,
            9 => &t_date.september,
            10 => &t_date.october,
            11 => &t_date.november,
            12 => &t_date.december,
            _ => panic!("Wrong month number."),
        };

        self.0
            .replace("{day}", &day.to_string())
            .replace("{month}", month_name)
            .replace("{year}", &year.to_string())
            .replace("{hour}", &hour.to_string())
            .replace("{min}", &minute.to_string())
            .replace("{tz}", &timezone.to_string())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringDateShort(String);

impl TStringDateShort {
    pub fn with(
        &self,
        t_date: &LocalI18nDate,
        date_value: &DateTime<Utc>
    ) -> String {
        let year = date_value.year();
        let month = date_value.month();
        let day = date_value.day();

        let month_name = match month {
            1 => &t_date.january,
            2 => &t_date.february,
            3 => &t_date.march,
            4 => &t_date.april,
            5 => &t_date.may,
            6 => &t_date.june,
            7 => &t_date.july,
            8 => &t_date.august,
            9 => &t_date.september,
            10 => &t_date.october,
            11 => &t_date.november,
            12 => &t_date.december,
            _ => panic!("Wrong month number."),
        };

        self.0
            .replace("{day}", &day.to_string())
            .replace("{month}", month_name)
            .replace("{year}", &year.to_string())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringDiskUsedInfo(String);

impl TStringDiskUsedInfo {
    pub fn with(
        &self,
        used: f64,
        used_unit: &str,
        total: f64,
        total_unit: &str
    ) -> String {
        self.0
            .replace("{used}", &used.to_string())
            .replace("{used_unit}", used_unit)
            .replace("{total}", &total.to_string())
            .replace("{total_unit}", total_unit)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringDuration(String);

impl TStringDuration {
    pub fn with(&self, t_duration: &LocalI18nDuration, seconds: u64) -> String {
        let (number, time_unit): (u64, String) = if 1 == seconds {
            (1, t_duration.second.clone())
        } else if seconds < 60 {
            (seconds, t_duration.seconds.clone())
        } else if seconds < 120 {
            (1, t_duration.minute.clone())
        } else if seconds < 3600 {
            (seconds / 60, t_duration.minutes.clone())
        } else if seconds < 7200 {
            (1, t_duration.hour.clone())
        } else if seconds < 86400 {
            (seconds / 3600, t_duration.hours.clone())
        } else if seconds < 172800 {
            (1, t_duration.day.clone())
        } else if seconds < 2592000 {
            (seconds / 86400, t_duration.days.clone())
        } else if seconds < 5184000 {
            (1, t_duration.month.clone())
        } else if seconds < 31536000 {
            (seconds / 2592000, t_duration.months.clone())
        } else if seconds < 63072000 {
            (1, t_duration.year.clone())
        } else {
            (seconds / 31536000, t_duration.years.clone())
        };

        self.0
            .replace("{number}", &number.to_string())
            .replace("{time_unit}", &time_unit)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringNResultsOfM(String);

impl TStringNResultsOfM {
    pub fn with(&self, total: i64, total_in_current_page: i64) -> String {
        self.0
            .replace("{total}", &total.to_string())
            .replace(
                "{total_in_current_page}",
                &total_in_current_page.to_string()
            )
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringWithDecimalNumber(String);

impl TStringWithDecimalNumber {
    pub fn with(&self, number: f64) -> String {
        self.0.replace("{number}", &number.to_string())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringWithName(String);

impl TStringWithName {
    pub fn with(&self, name: &str) -> String {
        self.0.replace("{name}", name)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TStringWithIntegerNumber(String);

impl TStringWithIntegerNumber {
    pub fn with(&self, number: i64) -> String {
        self.0.replace("{number}", &number.to_string())
    }
}
