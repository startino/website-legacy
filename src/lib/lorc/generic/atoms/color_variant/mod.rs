#[derive(PartialEq)]
pub enum ColorVariant {
    Primary,
    Secondary,
    Tertiary,

    OnPrimary,
    OnSecondary,
    OnTertiary,

    Background,
    Surface,
    SurfaceVariant,
    Outline,

    OnBackground,
    OnSurface,
    OnSurfaceVariant,

    // This is to be used if the color is provided in the class of the element.
    // Like if a custom color wants to be used, this enum variant is used as the color so that the "as_string()" returns an empty string.
    None,
}

impl ColorVariant {
    pub fn as_string(&self) -> &str {
        match self {
            ColorVariant::Primary => "text-primary-light dark:text-primary-dark",
            ColorVariant::Secondary => "text-secondary-light dark:text-secondary-dark",
            ColorVariant::Tertiary => "text-tertiary-light dark:text-tertiary-dark",
            ColorVariant::OnPrimary => "text-primary-on-light dark:text-primary-on-dark",
            ColorVariant::OnSecondary => "text-secondary-on-light dark:text-secondary-on-dark",
            ColorVariant::OnTertiary => "text-tertiary-on-light dark:text-tertiary-on-dark",

            ColorVariant::Background => "text-background-light dark:text-background-dark",
            ColorVariant::Surface => "text-surface-light dark:text-surface-dark",
            ColorVariant::SurfaceVariant => {
                "text-surface-variant-light dark:text-surface-variant-dark"
            }
            ColorVariant::Outline => "text-outline-light dark:text-outline-dark",
            ColorVariant::OnBackground => "text-background-on-light dark:text-background-on-dark",
            ColorVariant::OnSurface => "text-surface-on-light dark:text-surface-on-dark",
            ColorVariant::OnSurfaceVariant => {
                "text-surface-variant-on-light dark:text-surface-variant-on-dark"
            }

            ColorVariant::None => "",
        }
    }
}
