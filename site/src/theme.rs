pub const NAVY: &str = "#151E33";
pub const NAVY_DEEP: &str = "#0F1626";
pub const CURVE: &str = "#EFE6EC";
pub const PAPER: &str = "#F5F4F1";

pub struct Accent {
    pub key: &'static str,
    pub name: &'static str,
    pub tile: &'static str,
    pub light: &'static str,
}

pub const ACCENTS: [Accent; 8] = [
    Accent { key: "beere", name: "Beere", tile: "#6B2049", light: "#A3305F" },
    Accent { key: "tanne", name: "Tanne", tile: "#0F5C52", light: "#1C8474" },
    Accent { key: "wald", name: "Waldgrün", tile: "#23523A", light: "#3A8259" },
    Accent { key: "petrol", name: "Petrol", tile: "#0E5A73", light: "#1A87A8" },
    Accent { key: "stahl", name: "Stahlblau", tile: "#2F4C7A", light: "#4E76B0" },
    Accent { key: "kupfer", name: "Kupfer", tile: "#8C4426", light: "#C1703F" },
    Accent { key: "messing", name: "Messing", tile: "#8A6A15", light: "#B9932A" },
    Accent { key: "burgund", name: "Burgund", tile: "#8C1C3A", light: "#B93353" },
];

pub fn accent(key: &str) -> &'static Accent {
    ACCENTS
        .iter()
        .find(|a| a.key == key)
        .unwrap_or(&ACCENTS[0])
}

pub fn theme_css() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        ":root{{--navy:{};--navy-deep:{};--curve:{};--paper:{};}}\n",
        NAVY, NAVY_DEEP, CURVE, PAPER
    ));
    for a in ACCENTS.iter() {
        out.push_str(&format!(
            ".t-{k}{{--tile:{t};--light:{l};}}\n",
            k = a.key,
            t = a.tile,
            l = a.light
        ));
    }
    out
}
