//! Built-in skin families and colour schemes (`Preferences › Appearance › Skin`).
//!
//! Includes:
//! - `Aurora` (`default`): preserves the exact original `Theme::default()`
//!   palette so existing users see no change unless they select another skin.
//! - `SunSet`: a dedicated twilight sunset theme (`sunset_default`) along with
//!   Rose, Rose Pastel, Violet, Purple, Lilac, and Lilac Light schemes.
//! - Firestorm skin families (`Firestorm`, `AnsaStorm`, `AnsaStorm Modern`,
//!   `MetaHarper Modern`, `StarLight`, `StarLight CUI`, `Vintage`), adapted from
//!   `phoenix-firestorm/indra/newview/skins` for live switching without restart.

use crate::theme::Theme;

pub const DEFAULT_SKIN: &str = "aurora";
pub const DEFAULT_SCHEME: &str = "default";

/// Semantic chat, friend, and mention colours derived from a skin preset
/// when the user has not overridden those entries in Preferences › Colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkinChatColors {
    pub chat_mine: [u8; 3],
    pub chat_others: [u8; 3],
    pub chat_friends: [u8; 3],
    pub sender_avatar: [u8; 3],
    pub tag_friend: [u8; 3],
    pub mention_residents: [u8; 3],
    pub mention_me: [u8; 3],
    pub chat_urls: [u8; 3],
}

impl SkinChatColors {
    pub const fn rgba(c: [u8; 3]) -> crate::settings::Rgba {
        [c[0], c[1], c[2], 255]
    }
}

/// One colour scheme inside a skin family.
#[derive(Debug, Clone, Copy)]
pub struct SkinSchemeSpec {
    pub id: &'static str,
    pub fallback_name: &'static str,
    pub preview_accent: [u8; 3],
    pub preview_surface: [u8; 3],
    pub preview_text: [u8; 3],
    pub build_theme: fn() -> Theme,
    pub chat: SkinChatColors,
}

impl SkinSchemeSpec {
    /// Returns preview swatches `[surface, accent, typed_text]` for the selector UI.
    pub fn preview_swatches(&self) -> [egui::Color32; 3] {
        [
            egui::Color32::from_rgb(self.preview_surface[0], self.preview_surface[1], self.preview_surface[2]),
            egui::Color32::from_rgb(self.preview_accent[0], self.preview_accent[1], self.preview_accent[2]),
            egui::Color32::from_rgb(self.preview_text[0], self.preview_text[1], self.preview_text[2]),
        ]
    }
}

/// A named skin family grouping one or more colour schemes.
#[derive(Debug, Clone, Copy)]
pub struct SkinFamilySpec {
    pub id: &'static str,
    pub fallback_name: &'static str,
    pub default_scheme: &'static str,
    pub schemes: &'static [SkinSchemeSpec],
}

#[derive(Clone, Copy)]
struct ColorsSpec {
    violet: &'static str,
    indigo: &'static str,
    teal: &'static str,
    navy: &'static str,
    panel: &'static str,
    bar: &'static str,
    field: &'static str,
    raised: &'static str,
    violet_light: &'static str,
    violet_pale: &'static str,
    ink: &'static str,
    typed_text: &'static str,
    muted: &'static str,
    muted_dim: &'static str,
    indigo_light: &'static str,
    on_accent: &'static str,
    light_states: bool,
    panel_alpha: u8,
}

fn build(c: ColorsSpec) -> Theme {
    let mut t = Theme {
        violet: c.violet.into(),
        indigo: c.indigo.into(),
        teal: c.teal.into(),
        navy: c.navy.into(),
        panel: c.panel.into(),
        bar: c.bar.into(),
        field: c.field.into(),
        raised: c.raised.into(),
        violet_light: c.violet_light.into(),
        violet_pale: c.violet_pale.into(),
        ink: c.ink.into(),
        muted: c.muted.into(),
        muted_dim: c.muted_dim.into(),
        indigo_light: c.indigo_light.into(),
        on_accent: c.on_accent.into(),
        typed_text: c.typed_text.into(),
        panel_alpha: c.panel_alpha,
        ..Theme::default()
    };
    if c.light_states {
        t.amber = "#7C3A05".into();
        t.rose = "#9D174D".into();
        t.danger = "#B91C1C".into();
        t.success = "#15803D".into();
        t.warn = "#9A3412".into();
        t.danger_dim = "#991B1B".into();
    }
    t
}

fn aurora_default_theme() -> Theme {
    Theme::default()
}

fn sunset_twilight_theme() -> Theme {
    build(ColorsSpec {
        violet: "#F97316",
        indigo: "#E11D48",
        teal: "#FBBF24",
        navy: "#0D0714",
        panel: "#1F1726",
        bar: "#160F1D",
        field: "#120B18",
        raised: "#342440",
        violet_light: "#FB923C",
        violet_pale: "#FDBA74",
        ink: "#FFF1E6",
        typed_text: "#FFE8D6",
        muted: "#D8B4C8",
        muted_dim: "#9E7B96",
        indigo_light: "#EC4899",
        on_accent: "#160C1C",
        light_states: false,
        panel_alpha: 235,
    })
}

fn sunset_rose_theme() -> Theme {
    build(ColorsSpec {
        violet: "#EC4899",
        indigo: "#9D174D",
        teal: "#F9A8D4",
        navy: "#12070F",
        panel: "#2A1925",
        bar: "#1E101A",
        field: "#1C1019",
        raised: "#3D2535",
        violet_light: "#F472B6",
        violet_pale: "#FBCFE8",
        ink: "#FCE7F3",
        typed_text: "#FCE7F3",
        muted: "#DFB3C9",
        muted_dim: "#A67890",
        indigo_light: "#F472B6",
        on_accent: "#190611",
        light_states: false,
        panel_alpha: 235,
    })
}

fn sunset_rose_pastel_theme() -> Theme {
    build(ColorsSpec {
        violet: "#BE185D",
        indigo: "#9D174D",
        teal: "#0F766E",
        navy: "#FCE7F3",
        panel: "#F8D7EA",
        bar: "#F3C4E0",
        field: "#FFF5FA",
        raised: "#F3C4E0",
        violet_light: "#9D174D",
        violet_pale: "#831843",
        ink: "#3B0D26",
        typed_text: "#2A061A",
        muted: "#6E294E",
        muted_dim: "#824063",
        indigo_light: "#9D174D",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 244,
    })
}

fn sunset_violet_theme() -> Theme {
    build(ColorsSpec {
        violet: "#7C3AED",
        indigo: "#5B21B6",
        teal: "#38BDF8",
        navy: "#090613",
        panel: "#1D192B",
        bar: "#140F21",
        field: "#100C1A",
        raised: "#2E2747",
        violet_light: "#A855F7",
        violet_pale: "#DDD6FE",
        ink: "#F3EFFE",
        typed_text: "#EDE9FE",
        muted: "#B6A8DC",
        muted_dim: "#8576AD",
        indigo_light: "#C4B5FD",
        on_accent: "#FFFFFF",
        light_states: false,
        panel_alpha: 235,
    })
}

fn sunset_purple_theme() -> Theme {
    build(ColorsSpec {
        violet: "#9333EA",
        indigo: "#6B21A8",
        teal: "#E879F9",
        navy: "#0A0410",
        panel: "#1C1226",
        bar: "#140B1D",
        field: "#100818",
        raised: "#38234C",
        violet_light: "#C084FC",
        violet_pale: "#E9D5FF",
        ink: "#F5EAFF",
        typed_text: "#EFE2FF",
        muted: "#C5A8DF",
        muted_dim: "#8D6FA8",
        indigo_light: "#D8B4FE",
        on_accent: "#FFFFFF",
        light_states: false,
        panel_alpha: 235,
    })
}

fn sunset_lilac_theme() -> Theme {
    build(ColorsSpec {
        violet: "#C084FC",
        indigo: "#805AD5",
        teal: "#81E6D9",
        navy: "#0F0B1A",
        panel: "#261F33",
        bar: "#1C1728",
        field: "#171222",
        raised: "#392F4D",
        violet_light: "#A78BFA",
        violet_pale: "#E9D8FD",
        ink: "#F6F0FF",
        typed_text: "#EDE4FF",
        muted: "#C4B5FD",
        muted_dim: "#9482BF",
        indigo_light: "#D6BCFA",
        on_accent: "#171222",
        light_states: false,
        panel_alpha: 235,
    })
}

fn sunset_lilac_light_theme() -> Theme {
    build(ColorsSpec {
        violet: "#7E22CE",
        indigo: "#581C87",
        teal: "#0F766E",
        navy: "#EFE6FA",
        panel: "#F7F1FE",
        bar: "#E2D3F5",
        field: "#FAF6FF",
        raised: "#E2D3F5",
        violet_light: "#581C87",
        violet_pale: "#4C1D95",
        ink: "#231438",
        typed_text: "#180A29",
        muted: "#553C7B",
        muted_dim: "#6B5091",
        indigo_light: "#6B21A8",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 244,
    })
}

fn firestorm_grey_theme() -> Theme {
    build(ColorsSpec {
        violet: "#A66A4A",
        indigo: "#8F5C40",
        teal: "#DBA88C",
        navy: "#121212",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#161616",
        raised: "#313131",
        violet_light: "#DBA88C",
        violet_pale: "#ECC8B4",
        ink: "#E6E6E6",
        typed_text: "#F0E6E0",
        muted: "#B5B5B5",
        muted_dim: "#828282",
        indigo_light: "#DBA88C",
        on_accent: "#0A0604",
        light_states: false,
        panel_alpha: 238,
    })
}

fn firestorm_dark_theme() -> Theme {
    build(ColorsSpec {
        violet: "#A66A4A",
        indigo: "#8F5C40",
        teal: "#DBA88C",
        navy: "#050505",
        panel: "#181818",
        bar: "#101010",
        field: "#080808",
        raised: "#2B2B2B",
        violet_light: "#DBA88C",
        violet_pale: "#ECC8B4",
        ink: "#FFFFFF",
        typed_text: "#FFFFFF",
        muted: "#BFBFBF",
        muted_dim: "#888888",
        indigo_light: "#DBA88C",
        on_accent: "#0A0604",
        light_states: false,
        panel_alpha: 242,
    })
}

fn firestorm_blue_theme() -> Theme {
    build(ColorsSpec {
        violet: "#6EB4FF",
        indigo: "#1240AB",
        teal: "#A8E6FF",
        navy: "#133547",
        panel: "#214D69",
        bar: "#193D54",
        field: "#133043",
        raised: "#2F6587",
        violet_light: "#9CD0FF",
        violet_pale: "#CFE8FF",
        ink: "#FFFFFF",
        typed_text: "#E8F6FF",
        muted: "#C4E0EE",
        muted_dim: "#92BDD1",
        indigo_light: "#9CD0FF",
        on_accent: "#081726",
        light_states: false,
        panel_alpha: 240,
    })
}

fn firestorm_high_contrast_theme() -> Theme {
    build(ColorsSpec {
        violet: "#C4825E",
        indigo: "#8F5C40",
        teal: "#FFD0B5",
        navy: "#080808",
        panel: "#1A1A1A",
        bar: "#121212",
        field: "#0D0D0D",
        raised: "#313131",
        violet_light: "#E8B79A",
        violet_pale: "#FFE2D1",
        ink: "#FFFFFF",
        typed_text: "#FFFFFF",
        muted: "#D4D4D4",
        muted_dim: "#9E9E9E",
        indigo_light: "#E8B79A",
        on_accent: "#0A0705",
        light_states: false,
        panel_alpha: 255,
    })
}

fn firestorm_ctrlaltstudio_theme() -> Theme {
    build(ColorsSpec {
        violet: "#9C89C8",
        indigo: "#7762AA",
        teal: "#B6A2E1",
        navy: "#121018",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#17171A",
        raised: "#313131",
        violet_light: "#B6A2E1",
        violet_pale: "#D4C6F2",
        ink: "#E6E6E6",
        typed_text: "#EFEAFF",
        muted: "#B8B2C8",
        muted_dim: "#847E96",
        indigo_light: "#B6A2E1",
        on_accent: "#14121D",
        light_states: false,
        panel_alpha: 238,
    })
}

fn ansastorm_bright_blue_theme() -> Theme {
    build(ColorsSpec {
        violet: "#009ED8",
        indigo: "#0077A8",
        teal: "#00D1FF",
        navy: "#101418",
        panel: "#202020",
        bar: "#1A1A1A",
        field: "#141414",
        raised: "#313131",
        violet_light: "#38C8FF",
        violet_pale: "#9EE5FF",
        ink: "#E6E6E6",
        typed_text: "#E0F7FF",
        muted: "#A8BAC4",
        muted_dim: "#7D8F99",
        indigo_light: "#38C8FF",
        on_accent: "#081218",
        light_states: false,
        panel_alpha: 236,
    })
}

fn ansastorm_classic_brown_theme() -> Theme {
    firestorm_grey_theme()
}

fn ansastorm_ectoplasma_theme() -> Theme {
    build(ColorsSpec {
        violet: "#00D912",
        indigo: "#009E0D",
        teal: "#52FF62",
        navy: "#040804",
        panel: "#0D0F0D",
        bar: "#080A08",
        field: "#050705",
        raised: "#1C221C",
        violet_light: "#4DFF5C",
        violet_pale: "#A6FFAD",
        ink: "#E6F2E6",
        typed_text: "#C8FFCC",
        muted: "#94B896",
        muted_dim: "#688A6A",
        indigo_light: "#4DFF5C",
        on_accent: "#041205",
        light_states: false,
        panel_alpha: 242,
    })
}

fn ansastorm_blood_theme() -> Theme {
    build(ColorsSpec {
        violet: "#DC2626",
        indigo: "#991B1B",
        teal: "#FF5C64",
        navy: "#0A0404",
        panel: "#120B0B",
        bar: "#0B0606",
        field: "#080404",
        raised: "#261616",
        violet_light: "#FF6B72",
        violet_pale: "#FFB3B7",
        ink: "#F2E6E6",
        typed_text: "#FFD6D8",
        muted: "#C49E9E",
        muted_dim: "#946F6F",
        indigo_light: "#FF6B72",
        on_accent: "#FFFFFF",
        light_states: false,
        panel_alpha: 242,
    })
}

fn ansastorm_modern_blue_theme() -> Theme {
    build(ColorsSpec {
        violet: "#4CD1FF",
        indigo: "#1CA3D4",
        teal: "#7FE0FF",
        navy: "#0E1418",
        panel: "#202020",
        bar: "#1A1A1A",
        field: "#151515",
        raised: "#313131",
        violet_light: "#7FE0FF",
        violet_pale: "#BCEFFF",
        ink: "#E6E6E6",
        typed_text: "#E6F9FF",
        muted: "#AEBEC4",
        muted_dim: "#7F8E94",
        indigo_light: "#7FE0FF",
        on_accent: "#09151C",
        light_states: false,
        panel_alpha: 236,
    })
}

fn metaharper_cool_ocean_theme() -> Theme {
    build(ColorsSpec {
        violet: "#7AA4B8",
        indigo: "#445863",
        teal: "#94B7C5",
        navy: "#0E1316",
        panel: "#202326",
        bar: "#191C1F",
        field: "#14171A",
        raised: "#31363B",
        violet_light: "#94B7C5",
        violet_pale: "#C2D9E3",
        ink: "#E6ECEF",
        typed_text: "#E0F0F6",
        muted: "#A4B6BF",
        muted_dim: "#7A8C96",
        indigo_light: "#A48E74",
        on_accent: "#0E1316",
        light_states: false,
        panel_alpha: 236,
    })
}

fn metaharper_appalachian_sky_theme() -> Theme {
    build(ColorsSpec {
        violet: "#B38063",
        indigo: "#445863",
        teal: "#94B7C5",
        navy: "#101417",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#171717",
        raised: "#313131",
        violet_light: "#DBA88C",
        violet_pale: "#94B7C5",
        ink: "#E6E6E6",
        typed_text: "#F0E8E2",
        muted: "#B0BCC2",
        muted_dim: "#828E94",
        indigo_light: "#94B7C5",
        on_accent: "#101417",
        light_states: false,
        panel_alpha: 236,
    })
}

fn metaharper_black_glass_theme() -> Theme {
    build(ColorsSpec {
        violet: "#2574D4",
        indigo: "#0052A6",
        teal: "#5CA8FF",
        navy: "#04070B",
        panel: "#0E1014",
        bar: "#181B20",
        field: "#090B0E",
        raised: "#282D35",
        violet_light: "#66B0FF",
        violet_pale: "#A8D2FF",
        ink: "#E6ECF2",
        typed_text: "#E0EEFF",
        muted: "#9EB0C2",
        muted_dim: "#708294",
        indigo_light: "#66B0FF",
        on_accent: "#FFFFFF",
        light_states: false,
        panel_alpha: 242,
    })
}

fn starlight_orange_theme() -> Theme {
    build(ColorsSpec {
        violet: "#FF8C44",
        indigo: "#CC6222",
        teal: "#FFB27D",
        navy: "#14100C",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#171717",
        raised: "#313131",
        violet_light: "#FFA870",
        violet_pale: "#FFD0B0",
        ink: "#E6E6E6",
        typed_text: "#FFF0E5",
        muted: "#C4B4A8",
        muted_dim: "#918277",
        indigo_light: "#FFA870",
        on_accent: "#180D06",
        light_states: false,
        panel_alpha: 236,
    })
}

fn starlight_teal_theme() -> Theme {
    build(ColorsSpec {
        violet: "#61B192",
        indigo: "#3E876B",
        teal: "#8CD4B8",
        navy: "#0C1411",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#171717",
        raised: "#313131",
        violet_light: "#8CD4B8",
        violet_pale: "#BCEAD8",
        ink: "#E6E6E6",
        typed_text: "#E5FAF2",
        muted: "#A8C2B8",
        muted_dim: "#7B948B",
        indigo_light: "#8CD4B8",
        on_accent: "#081711",
        light_states: false,
        panel_alpha: 236,
    })
}

fn starlight_mono_teal_theme() -> Theme {
    build(ColorsSpec {
        violet: "#2E5E4B",
        indigo: "#224739",
        teal: "#1E5240",
        navy: "#DCE3E0",
        panel: "#E0E0E0",
        bar: "#EFEFEF",
        field: "#FFFFFF",
        raised: "#CCCCCC",
        violet_light: "#224739",
        violet_pale: "#19382C",
        ink: "#141A17",
        typed_text: "#0D1411",
        muted: "#42524B",
        muted_dim: "#5C6B64",
        indigo_light: "#2E5E4B",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 245,
    })
}

fn starlight_nostalgia_blue_theme() -> Theme {
    build(ColorsSpec {
        violet: "#6070BF",
        indigo: "#4D5BA0",
        teal: "#98A5E3",
        navy: "#0F111A",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#171717",
        raised: "#313131",
        violet_light: "#98A5E3",
        violet_pale: "#C5CDF5",
        ink: "#E6E6E6",
        typed_text: "#EAEDFF",
        muted: "#AEB4CC",
        muted_dim: "#80869E",
        indigo_light: "#98A5E3",
        on_accent: "#FFFFFF",
        light_states: false,
        panel_alpha: 236,
    })
}

fn starlight_silver_blue_theme() -> Theme {
    build(ColorsSpec {
        violet: "#0F3A6D",
        indigo: "#001E3C",
        teal: "#124E78",
        navy: "#D0DDF0",
        panel: "#C4D2E6",
        bar: "#D6E3F5",
        field: "#F2F7FD",
        raised: "#AEBFD8",
        violet_light: "#0A2B52",
        violet_pale: "#001E3C",
        ink: "#0C1826",
        typed_text: "#06101C",
        muted: "#334761",
        muted_dim: "#4B607A",
        indigo_light: "#0F3A6D",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 245,
    })
}

fn starlight_silver_pink_theme() -> Theme {
    build(ColorsSpec {
        violet: "#962D64",
        indigo: "#751F4C",
        teal: "#7A2855",
        navy: "#F7D9EA",
        panel: "#F1C4DE",
        bar: "#FAD4EB",
        field: "#FFF4FA",
        raised: "#E3A8CB",
        violet_light: "#751F4C",
        violet_pale: "#591438",
        ink: "#260B1A",
        typed_text: "#1C0512",
        muted: "#5C2844",
        muted_dim: "#753E5C",
        indigo_light: "#962D64",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 245,
    })
}

fn starlight_cui_dark_theme() -> Theme {
    build(ColorsSpec {
        violet: "#94A3B8",
        indigo: "#64748B",
        teal: "#CBD5E1",
        navy: "#101012",
        panel: "#202020",
        bar: "#2B2B2B",
        field: "#171717",
        raised: "#313131",
        violet_light: "#CBD5E1",
        violet_pale: "#F1F5F9",
        ink: "#E6E6E6",
        typed_text: "#FFFFFF",
        muted: "#B0B5BD",
        muted_dim: "#82878F",
        indigo_light: "#CBD5E1",
        on_accent: "#111318",
        light_states: false,
        panel_alpha: 238,
    })
}

fn starlight_cui_light_theme() -> Theme {
    build(ColorsSpec {
        violet: "#262626",
        indigo: "#404040",
        teal: "#1F2937",
        navy: "#E5E5E5",
        panel: "#E0E0E0",
        bar: "#EFEFEF",
        field: "#FFFFFF",
        raised: "#CCCCCC",
        violet_light: "#262626",
        violet_pale: "#111111",
        ink: "#111111",
        typed_text: "#000000",
        muted: "#454545",
        muted_dim: "#5E5E5E",
        indigo_light: "#262626",
        on_accent: "#FFFFFF",
        light_states: true,
        panel_alpha: 245,
    })
}

fn vintage_classic_theme() -> Theme {
    build(ColorsSpec {
        violet: "#AD7251",
        indigo: "#754A32",
        teal: "#93A9D5",
        navy: "#1A1A1A",
        panel: "#2E2E2E",
        bar: "#222222",
        field: "#1C1C1C",
        raised: "#424242",
        violet_light: "#B8CCE8",
        violet_pale: "#DCE7F7",
        ink: "#F0F2F5",
        typed_text: "#FFFFFF",
        muted: "#B8C4D9",
        muted_dim: "#8B98B0",
        indigo_light: "#93A9D5",
        on_accent: "#160E09",
        light_states: false,
        panel_alpha: 242,
    })
}

const AURORA_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x9A, 0xA6, 0xC6],
    chat_others: [0xE8, 0xED, 0xFB],
    chat_friends: [0x81, 0x8C, 0xF8],
    sender_avatar: [0xC4, 0xB5, 0xFD],
    tag_friend: [0x81, 0x8C, 0xF8],
    mention_residents: [0x8B, 0x5C, 0xF6],
    mention_me: [0x4F, 0x46, 0xE5],
    chat_urls: [0x5E, 0xEA, 0xD4],
};

const SUNSET_TWILIGHT_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xFD, 0xBA, 0x74],
    chat_others: [0xFF, 0xF3, 0xE6],
    chat_friends: [0xF4, 0x72, 0xB6],
    sender_avatar: [0xFB, 0x92, 0x3C],
    tag_friend: [0xFB, 0xBF, 0x24],
    mention_residents: [0xF9, 0x73, 0x16],
    mention_me: [0xE8, 0x79, 0xF9],
    chat_urls: [0xFB, 0xBF, 0x24],
};

const ROSE_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xF9, 0xA8, 0xD4],
    chat_others: [0xFD, 0xF2, 0xF8],
    chat_friends: [0xF4, 0x72, 0xB6],
    sender_avatar: [0xFB, 0xCF, 0xE8],
    tag_friend: [0xF4, 0x72, 0xB6],
    mention_residents: [0xF4, 0x72, 0xB6],
    mention_me: [0xDB, 0x27, 0x77],
    chat_urls: [0xF9, 0xA8, 0xD4],
};

const ROSE_PASTEL_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x6E, 0x29, 0x4E],
    chat_others: [0x3B, 0x0D, 0x26],
    chat_friends: [0xBE, 0x18, 0x5D],
    sender_avatar: [0x9D, 0x17, 0x4D],
    tag_friend: [0xBE, 0x18, 0x5D],
    mention_residents: [0x9D, 0x17, 0x4D],
    mention_me: [0x83, 0x18, 0x43],
    chat_urls: [0x0F, 0x76, 0x6E],
};

const VIOLET_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xC4, 0xB5, 0xFD],
    chat_others: [0xF3, 0xEF, 0xFE],
    chat_friends: [0xA7, 0x8B, 0xFA],
    sender_avatar: [0xDD, 0xD6, 0xFE],
    tag_friend: [0xA7, 0x8B, 0xFA],
    mention_residents: [0xA7, 0x8B, 0xFA],
    mention_me: [0x7C, 0x3A, 0xED],
    chat_urls: [0x38, 0xBD, 0xF8],
};

const PURPLE_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xD8, 0xB4, 0xFE],
    chat_others: [0xF7, 0xEE, 0xFF],
    chat_friends: [0xC0, 0x84, 0xFC],
    sender_avatar: [0xE9, 0xD5, 0xFF],
    tag_friend: [0xC0, 0x84, 0xFC],
    mention_residents: [0xC0, 0x84, 0xFC],
    mention_me: [0x93, 0x33, 0xEA],
    chat_urls: [0xE8, 0x79, 0xF9],
};

const LILAC_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xD6, 0xBC, 0xFA],
    chat_others: [0xF8, 0xF4, 0xFF],
    chat_friends: [0xB7, 0x94, 0xF4],
    sender_avatar: [0xE9, 0xD8, 0xFD],
    tag_friend: [0xB7, 0x94, 0xF4],
    mention_residents: [0xB7, 0x94, 0xF4],
    mention_me: [0xD6, 0xBC, 0xFA],
    chat_urls: [0x81, 0xE6, 0xD9],
};

const LILAC_LIGHT_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x55, 0x3C, 0x7B],
    chat_others: [0x23, 0x14, 0x38],
    chat_friends: [0x6B, 0x21, 0xA8],
    sender_avatar: [0x58, 0x1C, 0x87],
    tag_friend: [0x6B, 0x21, 0xA8],
    mention_residents: [0x6B, 0x21, 0xA8],
    mention_me: [0x4C, 0x1D, 0x95],
    chat_urls: [0x0F, 0x76, 0x6E],
};

const WARM_BRONZE_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xD8, 0xC4, 0xB8],
    chat_others: [0xE6, 0xE6, 0xE6],
    chat_friends: [0xDB, 0xA8, 0x8C],
    sender_avatar: [0xEC, 0xC8, 0xB4],
    tag_friend: [0xDB, 0xA8, 0x8C],
    mention_residents: [0xDB, 0xA8, 0x8C],
    mention_me: [0xC4, 0x82, 0x5E],
    chat_urls: [0xDB, 0xA8, 0x8C],
};

const COOL_CYAN_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xA8, 0xD8, 0xEA],
    chat_others: [0xE6, 0xF4, 0xF8],
    chat_friends: [0x4C, 0xD1, 0xFF],
    sender_avatar: [0x9E, 0xE5, 0xFF],
    tag_friend: [0x4C, 0xD1, 0xFF],
    mention_residents: [0x38, 0xC8, 0xFF],
    mention_me: [0x00, 0x9E, 0xD8],
    chat_urls: [0x7F, 0xE0, 0xFF],
};

const NEON_GREEN_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xA6, 0xFF, 0xAD],
    chat_others: [0xE6, 0xF2, 0xE6],
    chat_friends: [0x52, 0xFF, 0x62],
    sender_avatar: [0x94, 0xE8, 0x9A],
    tag_friend: [0x52, 0xFF, 0x62],
    mention_residents: [0x00, 0xD9, 0x12],
    mention_me: [0x52, 0xFF, 0x62],
    chat_urls: [0x52, 0xFF, 0x62],
};

const BLOOD_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0xE0, 0xB4, 0xB4],
    chat_others: [0xF2, 0xE6, 0xE6],
    chat_friends: [0xFF, 0x6B, 0x72],
    sender_avatar: [0xFF, 0xB3, 0xB7],
    tag_friend: [0xFF, 0x6B, 0x72],
    mention_residents: [0xFF, 0x5C, 0x64],
    mention_me: [0xDC, 0x26, 0x26],
    chat_urls: [0xFF, 0x8C, 0x90],
};

const LIGHT_SLATE_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x33, 0x47, 0x61],
    chat_others: [0x0C, 0x18, 0x26],
    chat_friends: [0x0F, 0x3A, 0x6D],
    sender_avatar: [0x12, 0x4E, 0x78],
    tag_friend: [0x0F, 0x3A, 0x6D],
    mention_residents: [0x0F, 0x3A, 0x6D],
    mention_me: [0x00, 0x1E, 0x3C],
    chat_urls: [0x12, 0x4E, 0x78],
};

const LIGHT_TEAL_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x3B, 0x4A, 0x44],
    chat_others: [0x14, 0x1A, 0x17],
    chat_friends: [0x1E, 0x52, 0x40],
    sender_avatar: [0x2E, 0x5E, 0x4B],
    tag_friend: [0x22, 0x47, 0x39],
    mention_residents: [0x2E, 0x5E, 0x4B],
    mention_me: [0x19, 0x38, 0x2C],
    chat_urls: [0x1E, 0x52, 0x40],
};

const LIGHT_PINK_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x5C, 0x28, 0x44],
    chat_others: [0x26, 0x0B, 0x1A],
    chat_friends: [0x96, 0x2D, 0x64],
    sender_avatar: [0x75, 0x1F, 0x4C],
    tag_friend: [0x96, 0x2D, 0x64],
    mention_residents: [0x96, 0x2D, 0x64],
    mention_me: [0x59, 0x14, 0x38],
    chat_urls: [0x7A, 0x28, 0x55],
};

const LIGHT_MONO_CHAT: SkinChatColors = SkinChatColors {
    chat_mine: [0x45, 0x45, 0x45],
    chat_others: [0x11, 0x11, 0x11],
    chat_friends: [0x1F, 0x29, 0x37],
    sender_avatar: [0x26, 0x26, 0x26],
    tag_friend: [0x1F, 0x29, 0x37],
    mention_residents: [0x26, 0x26, 0x26],
    mention_me: [0x11, 0x11, 0x11],
    chat_urls: [0x1F, 0x29, 0x37],
};

const AURORA_DEFAULT_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "default",
    fallback_name: "Aurora (Default)",
    preview_accent: [0x8B, 0x5C, 0xF6],
    preview_surface: [0x25, 0x28, 0x30],
    preview_text: [0xE8, 0xED, 0xFB],
    build_theme: aurora_default_theme,
    chat: AURORA_CHAT,
};

const SUNSET_DEFAULT_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "sunset_default",
    fallback_name: "SunSet Theme (Twilight)",
    preview_accent: [0xF9, 0x73, 0x16],
    preview_surface: [0x1F, 0x17, 0x26],
    preview_text: [0xFF, 0xF1, 0xE6],
    build_theme: sunset_twilight_theme,
    chat: SUNSET_TWILIGHT_CHAT,
};

const SUNSET_ROSE_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "rose",
    fallback_name: "Rose",
    preview_accent: [0xEC, 0x48, 0x99],
    preview_surface: [0x2A, 0x19, 0x25],
    preview_text: [0xFC, 0xE7, 0xF3],
    build_theme: sunset_rose_theme,
    chat: ROSE_CHAT,
};

const SUNSET_ROSE_PASTEL_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "rose_pastel",
    fallback_name: "Rose Pastel (Light)",
    preview_accent: [0xBE, 0x18, 0x5D],
    preview_surface: [0xF8, 0xD7, 0xEA],
    preview_text: [0x3B, 0x0D, 0x26],
    build_theme: sunset_rose_pastel_theme,
    chat: ROSE_PASTEL_CHAT,
};

const SUNSET_VIOLET_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "violet",
    fallback_name: "Violet",
    preview_accent: [0x7C, 0x3A, 0xED],
    preview_surface: [0x1D, 0x19, 0x2B],
    preview_text: [0xF3, 0xEF, 0xFE],
    build_theme: sunset_violet_theme,
    chat: VIOLET_CHAT,
};

const SUNSET_PURPLE_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "purple",
    fallback_name: "Purple",
    preview_accent: [0x93, 0x33, 0xEA],
    preview_surface: [0x1C, 0x12, 0x26],
    preview_text: [0xF5, 0xEA, 0xFF],
    build_theme: sunset_purple_theme,
    chat: PURPLE_CHAT,
};

const SUNSET_LILAC_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "lilac",
    fallback_name: "Lilac",
    preview_accent: [0xC0, 0x84, 0xFC],
    preview_surface: [0x26, 0x1F, 0x33],
    preview_text: [0xF6, 0xF0, 0xFF],
    build_theme: sunset_lilac_theme,
    chat: LILAC_CHAT,
};

const SUNSET_LILAC_LIGHT_SPEC: SkinSchemeSpec = SkinSchemeSpec {
    id: "lilac_light",
    fallback_name: "Lilac Light",
    preview_accent: [0x7E, 0x22, 0xCE],
    preview_surface: [0xF7, 0xF1, 0xFE],
    preview_text: [0x23, 0x14, 0x38],
    build_theme: sunset_lilac_light_theme,
    chat: LILAC_LIGHT_CHAT,
};

const AURORA_SCHEMES: &[SkinSchemeSpec] = &[
    AURORA_DEFAULT_SPEC,
    SUNSET_DEFAULT_SPEC,
    SUNSET_ROSE_SPEC,
    SUNSET_ROSE_PASTEL_SPEC,
    SUNSET_VIOLET_SPEC,
    SUNSET_PURPLE_SPEC,
    SUNSET_LILAC_SPEC,
    SUNSET_LILAC_LIGHT_SPEC,
];

const SUNSET_SCHEMES: &[SkinSchemeSpec] = &[
    SUNSET_DEFAULT_SPEC,
    SUNSET_ROSE_SPEC,
    SUNSET_ROSE_PASTEL_SPEC,
    SUNSET_VIOLET_SPEC,
    SUNSET_PURPLE_SPEC,
    SUNSET_LILAC_SPEC,
    SUNSET_LILAC_LIGHT_SPEC,
    AURORA_DEFAULT_SPEC,
];

const FIRESTORM_SCHEMES: &[SkinSchemeSpec] = &[
    SkinSchemeSpec {
        id: "grey",
        fallback_name: "Grey",
        preview_accent: [0xA6, 0x6A, 0x4A],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: firestorm_grey_theme,
        chat: WARM_BRONZE_CHAT,
    },
    SkinSchemeSpec {
        id: "dark",
        fallback_name: "Dark",
        preview_accent: [0xA6, 0x6A, 0x4A],
        preview_surface: [0x18, 0x18, 0x18],
        preview_text: [0xFF, 0xFF, 0xFF],
        build_theme: firestorm_dark_theme,
        chat: WARM_BRONZE_CHAT,
    },
    SkinSchemeSpec {
        id: "blue",
        fallback_name: "Blue",
        preview_accent: [0x6E, 0xB4, 0xFF],
        preview_surface: [0x21, 0x4D, 0x69],
        preview_text: [0xFF, 0xFF, 0xFF],
        build_theme: firestorm_blue_theme,
        chat: COOL_CYAN_CHAT,
    },
    SkinSchemeSpec {
        id: "high_contrast",
        fallback_name: "High Contrast",
        preview_accent: [0xC4, 0x82, 0x5E],
        preview_surface: [0x1A, 0x1A, 0x1A],
        preview_text: [0xFF, 0xFF, 0xFF],
        build_theme: firestorm_high_contrast_theme,
        chat: WARM_BRONZE_CHAT,
    },
    SkinSchemeSpec {
        id: "ctrlaltstudio",
        fallback_name: "CtrlAltStudio",
        preview_accent: [0x9C, 0x89, 0xC8],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: firestorm_ctrlaltstudio_theme,
        chat: LILAC_CHAT,
    },
];

const ANSASTORM_SCHEMES: &[SkinSchemeSpec] = &[
    SkinSchemeSpec {
        id: "bright_blue",
        fallback_name: "Bright Blue",
        preview_accent: [0x00, 0x9E, 0xD8],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: ansastorm_bright_blue_theme,
        chat: COOL_CYAN_CHAT,
    },
    SkinSchemeSpec {
        id: "classic_brown",
        fallback_name: "Classic Brown",
        preview_accent: [0xA6, 0x6A, 0x4A],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: ansastorm_classic_brown_theme,
        chat: WARM_BRONZE_CHAT,
    },
    SkinSchemeSpec {
        id: "ectoplasma",
        fallback_name: "Ectoplasma",
        preview_accent: [0x00, 0xD9, 0x12],
        preview_surface: [0x0D, 0x0F, 0x0D],
        preview_text: [0xE6, 0xF2, 0xE6],
        build_theme: ansastorm_ectoplasma_theme,
        chat: NEON_GREEN_CHAT,
    },
    SkinSchemeSpec {
        id: "blood",
        fallback_name: "Blood",
        preview_accent: [0xDC, 0x26, 0x26],
        preview_surface: [0x12, 0x0B, 0x0B],
        preview_text: [0xF2, 0xE6, 0xE6],
        build_theme: ansastorm_blood_theme,
        chat: BLOOD_CHAT,
    },
];

const ANSASTORM_MODERN_SCHEMES: &[SkinSchemeSpec] = &[SkinSchemeSpec {
    id: "blue",
    fallback_name: "Blue",
    preview_accent: [0x4C, 0xD1, 0xFF],
    preview_surface: [0x20, 0x20, 0x20],
    preview_text: [0xE6, 0xE6, 0xE6],
    build_theme: ansastorm_modern_blue_theme,
    chat: COOL_CYAN_CHAT,
}];

const METAHARPER_SCHEMES: &[SkinSchemeSpec] = &[
    SkinSchemeSpec {
        id: "cool_ocean",
        fallback_name: "CoolOcean",
        preview_accent: [0x7A, 0xA4, 0xB8],
        preview_surface: [0x20, 0x23, 0x26],
        preview_text: [0xE6, 0xEC, 0xEF],
        build_theme: metaharper_cool_ocean_theme,
        chat: COOL_CYAN_CHAT,
    },
    SkinSchemeSpec {
        id: "appalachian_sky",
        fallback_name: "AppalachianSky",
        preview_accent: [0xB3, 0x80, 0x63],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: metaharper_appalachian_sky_theme,
        chat: WARM_BRONZE_CHAT,
    },
    SkinSchemeSpec {
        id: "black_glass",
        fallback_name: "BlackGlass",
        preview_accent: [0x25, 0x74, 0xD4],
        preview_surface: [0x0E, 0x10, 0x14],
        preview_text: [0xE6, 0xEC, 0xF2],
        build_theme: metaharper_black_glass_theme,
        chat: COOL_CYAN_CHAT,
    },
];

const STARLIGHT_SCHEMES: &[SkinSchemeSpec] = &[
    SkinSchemeSpec {
        id: "original_orange",
        fallback_name: "Original Orange",
        preview_accent: [0xFF, 0x8C, 0x44],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: starlight_orange_theme,
        chat: SUNSET_TWILIGHT_CHAT,
    },
    SkinSchemeSpec {
        id: "original_teal",
        fallback_name: "Original Teal",
        preview_accent: [0x61, 0xB1, 0x92],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: starlight_teal_theme,
        chat: COOL_CYAN_CHAT,
    },
    SkinSchemeSpec {
        id: "mono_teal",
        fallback_name: "Mono Teal",
        preview_accent: [0x2E, 0x5E, 0x4B],
        preview_surface: [0xE0, 0xE0, 0xE0],
        preview_text: [0x14, 0x1A, 0x17],
        build_theme: starlight_mono_teal_theme,
        chat: LIGHT_TEAL_CHAT,
    },
    SkinSchemeSpec {
        id: "nostalgia_blue",
        fallback_name: "Nostalgia Blue",
        preview_accent: [0x60, 0x70, 0xBF],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: starlight_nostalgia_blue_theme,
        chat: AURORA_CHAT,
    },
    SkinSchemeSpec {
        id: "silver_blue",
        fallback_name: "Silver Blue",
        preview_accent: [0x0F, 0x3A, 0x6D],
        preview_surface: [0xC4, 0xD2, 0xE6],
        preview_text: [0x0C, 0x18, 0x26],
        build_theme: starlight_silver_blue_theme,
        chat: LIGHT_SLATE_CHAT,
    },
    SkinSchemeSpec {
        id: "silver_pink",
        fallback_name: "Silver Pink",
        preview_accent: [0x96, 0x2D, 0x64],
        preview_surface: [0xF1, 0xC4, 0xDE],
        preview_text: [0x26, 0x0B, 0x1A],
        build_theme: starlight_silver_pink_theme,
        chat: LIGHT_PINK_CHAT,
    },
];

const STARLIGHT_CUI_SCHEMES: &[SkinSchemeSpec] = &[
    SkinSchemeSpec {
        id: "custom_dark",
        fallback_name: "Custom Dark",
        preview_accent: [0x94, 0xA3, 0xB8],
        preview_surface: [0x20, 0x20, 0x20],
        preview_text: [0xE6, 0xE6, 0xE6],
        build_theme: starlight_cui_dark_theme,
        chat: AURORA_CHAT,
    },
    SkinSchemeSpec {
        id: "custom_light",
        fallback_name: "Custom Light",
        preview_accent: [0x26, 0x26, 0x26],
        preview_surface: [0xE0, 0xE0, 0xE0],
        preview_text: [0x11, 0x11, 0x11],
        build_theme: starlight_cui_light_theme,
        chat: LIGHT_MONO_CHAT,
    },
];

const VINTAGE_SCHEMES: &[SkinSchemeSpec] = &[SkinSchemeSpec {
    id: "classic",
    fallback_name: "Classic",
    preview_accent: [0xAD, 0x72, 0x51],
    preview_surface: [0x2E, 0x2E, 0x2E],
    preview_text: [0xF0, 0xF2, 0xF5],
    build_theme: vintage_classic_theme,
    chat: WARM_BRONZE_CHAT,
}];

static FAMILIES: &[SkinFamilySpec] = &[
    SkinFamilySpec {
        id: "aurora",
        fallback_name: "Aurora (Default)",
        default_scheme: "default",
        schemes: AURORA_SCHEMES,
    },
    SkinFamilySpec {
        id: "sunset",
        fallback_name: "SunSet",
        default_scheme: "sunset_default",
        schemes: SUNSET_SCHEMES,
    },
    SkinFamilySpec {
        id: "firestorm",
        fallback_name: "Firestorm",
        default_scheme: "grey",
        schemes: FIRESTORM_SCHEMES,
    },
    SkinFamilySpec {
        id: "ansastorm",
        fallback_name: "AnsaStorm",
        default_scheme: "bright_blue",
        schemes: ANSASTORM_SCHEMES,
    },
    SkinFamilySpec {
        id: "ansastorm_modern",
        fallback_name: "AnsaStorm Modern",
        default_scheme: "blue",
        schemes: ANSASTORM_MODERN_SCHEMES,
    },
    SkinFamilySpec {
        id: "metaharper_modern",
        fallback_name: "MetaHarper Modern",
        default_scheme: "cool_ocean",
        schemes: METAHARPER_SCHEMES,
    },
    SkinFamilySpec {
        id: "starlight",
        fallback_name: "StarLight",
        default_scheme: "original_orange",
        schemes: STARLIGHT_SCHEMES,
    },
    SkinFamilySpec {
        id: "starlight_cui",
        fallback_name: "StarLight CUI",
        default_scheme: "custom_dark",
        schemes: STARLIGHT_CUI_SCHEMES,
    },
    SkinFamilySpec {
        id: "vintage",
        fallback_name: "Vintage",
        default_scheme: "classic",
        schemes: VINTAGE_SCHEMES,
    },
];

pub fn families() -> &'static [SkinFamilySpec] {
    FAMILIES
}

fn normalize_id(raw: &str) -> String {
    raw.trim().to_ascii_lowercase().replace(['-', ' '], "_")
}

pub fn find_family(skin: &str) -> &'static SkinFamilySpec {
    let key = normalize_id(skin);
    FAMILIES.iter().find(|f| f.id == key).unwrap_or(&FAMILIES[0])
}

pub fn find_scheme(skin: &str, scheme: &str) -> (&'static SkinFamilySpec, &'static SkinSchemeSpec) {
    let fam = find_family(skin);
    let mut key = normalize_id(scheme);
    if fam.id == "sunset" && (key == "default" || key == "twilight" || key == "sunset") {
        key = "sunset_default".into();
    }
    let chosen = fam
        .schemes
        .iter()
        .find(|s| s.id == key)
        .or_else(|| fam.schemes.iter().find(|s| s.id == fam.default_scheme))
        .unwrap_or(&fam.schemes[0]);
    (fam, chosen)
}

pub fn normalize_pair(skin: &str, scheme: &str) -> (String, String) {
    let (fam, s) = find_scheme(skin, scheme);
    (fam.id.to_owned(), s.id.to_owned())
}

pub fn family_label(family: &SkinFamilySpec) -> String {
    match family.id {
        "aurora" => aurora_i18n::tr!("skin-family-aurora").to_owned(),
        "sunset" => aurora_i18n::tr!("skin-family-sunset").to_owned(),
        _ => family.fallback_name.to_owned(),
    }
}

pub fn scheme_label(scheme: &SkinSchemeSpec) -> String {
    match scheme.id {
        "default" => aurora_i18n::tr!("skin-scheme-aurora-default").to_owned(),
        "sunset_default" => aurora_i18n::tr!("skin-scheme-sunset-default").to_owned(),
        "rose" => aurora_i18n::tr!("skin-scheme-sunset-rose").to_owned(),
        "rose_pastel" => aurora_i18n::tr!("skin-scheme-sunset-rose-pastel").to_owned(),
        "violet" => aurora_i18n::tr!("skin-scheme-sunset-violet").to_owned(),
        "purple" => aurora_i18n::tr!("skin-scheme-sunset-purple").to_owned(),
        "lilac" => aurora_i18n::tr!("skin-scheme-sunset-lilac").to_owned(),
        "lilac_light" => aurora_i18n::tr!("skin-scheme-sunset-lilac-light").to_owned(),
        _ => scheme.fallback_name.to_owned(),
    }
}

/// Optional demo override (`AURORA_DEMO_SKIN=sunset:rose` or `AURORA_DEMO_SKIN=sunset`).
pub fn demo_override() -> Option<(String, String)> {
    let raw = std::env::var("AURORA_DEMO_SKIN").ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let (skin, scheme) = trimmed.split_once(':').unwrap_or((trimmed, ""));
    Some(normalize_pair(skin, scheme))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface_colors::contrast;

    #[test]
    fn aurora_default_matches_theme_default_exact() {
        let (fam, scheme) = find_scheme(DEFAULT_SKIN, DEFAULT_SCHEME);
        assert_eq!(fam.id, "aurora");
        assert_eq!(scheme.id, "default");
        let a = serde_json::to_value((scheme.build_theme)()).expect("serialize preset");
        let b = serde_json::to_value(Theme::default()).expect("serialize default");
        assert_eq!(a, b);
    }

    #[test]
    fn sunset_default_is_distinct_twilight_palette() {
        let (_, sunset) = find_scheme("sunset", "sunset_default");
        let (_, alias) = find_scheme("sunset", "twilight");
        assert_eq!(sunset.id, "sunset_default");
        assert_eq!(alias.id, "sunset_default");
        let t = (sunset.build_theme)();
        assert_ne!(t.violet, Theme::default().violet);
        assert_ne!(t.panel, Theme::default().panel);
    }

    #[test]
    fn all_skin_presets_meet_readability_ratios() {
        let mut issues = Vec::new();
        for fam in families() {
            assert!(
                fam.schemes.iter().any(|s| s.id == fam.default_scheme),
                "{} default scheme exists",
                fam.id
            );
            for scheme in fam.schemes {
                let t = (scheme.build_theme)();
                let p = t.skin_palette();
                let id = format!("{}/{}", fam.id, scheme.id);
                if contrast(p.ink, p.panel) < 4.5 {
                    issues.push(format!("{id}: ink contrast {}", contrast(p.ink, p.panel)));
                }
                if contrast(p.typed_text, p.field) < 4.5 {
                    issues.push(format!("{id}: typed_text contrast on field {}", contrast(p.typed_text, p.field)));
                }
                if contrast(p.typed_text, p.raised) < 4.5 {
                    issues.push(format!("{id}: typed_text contrast on raised {}", contrast(p.typed_text, p.raised)));
                }
                if contrast(p.muted, p.panel) < 4.5 {
                    issues.push(format!("{id}: muted contrast {}", contrast(p.muted, p.panel)));
                }
                if contrast(p.muted_dim, p.panel) < 3.0 {
                    issues.push(format!("{id}: muted_dim contrast {}", contrast(p.muted_dim, p.panel)));
                }
                let min_on_accent = if scheme.id == "default" { 4.2 } else { 4.5 };
                if contrast(p.on_accent, p.violet) < min_on_accent {
                    issues.push(format!("{id}: on_accent contrast {}", contrast(p.on_accent, p.violet)));
                }
                if contrast(p.violet, p.panel) < 3.0 {
                    issues.push(format!("{id}: accent on panel contrast {}", contrast(p.violet, p.panel)));
                }
            }
        }
        assert!(issues.is_empty(), "Readability failures:\n{}", issues.join("\n"));
    }
}
