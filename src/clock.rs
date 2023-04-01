use time::{format_description::FormatItem, macros::format_description, OffsetDateTime};

const DATE_FORMAT: &[FormatItem<'_>] = format_description!(
    version = 2,
    "[weekday repr:short] [year]-[month]-[day] [hour]:[minute]"
);

pub fn clock() -> String {
    format!(
        "\u{f017} {}",
        &OffsetDateTime::now_local()
            .unwrap()
            .format(DATE_FORMAT)
            .unwrap(),
    )
}
