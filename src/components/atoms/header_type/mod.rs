// These enums will be used to preset the header's color. This allows us to easily change the headers color without having to input the styles in the html.
// Instead of "text-background-on-light dark:text-background-on-dark", we can just do <H1 color=HeaderColor::OnBackground>""
#[derive(PartialEq)]
pub enum HeaderColor {
    OnPrimary,
    OnSecondary,
    OnTertiary,

    OnBackground,
    OnSurface,
    OnSurfaceVariant,
    /*
    OnBackground,
    LeftOnBackground,
    CenterOnBackground,
    RightOnBackground,

    OnSurface,
    LeftOnSurface,
    CenterOnSurface,
    RightOnSurface,

    OnSurfaceVariant,
    LeftOnSurfaceVariant,
    CenterOnSurfaceVariant,
    RightOnSurfaceVariant,
     */
}
