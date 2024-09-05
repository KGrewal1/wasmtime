use crate::cdsl::{isa::TargetIsa, settings::SettingGroupBuilder};

pub(crate) fn define() -> TargetIsa {
    let mut settings = SettingGroupBuilder::new("wasm32");
    settings.add_enum(
        "pointer_width",
        "The width of pointers for this Wasm target",
        "Supported values:\n\
         * 'pointer32'\n",
        vec!["pointer32"],
    );
    TargetIsa::new("wasm32", settings.build())
}
