use crate::theme::{ACCENTS, CURVE, NAVY};
use maud::{html, Markup, PreEscaped};

pub const RIDGE_FILL: &str = "M12 70 L26 57 L32 62 L46 40 L52 46 L62 18 L78 70 Z";
pub const RIDGE_LINE: &str = "M12 70 L26 57 L32 62 L46 40 L52 46 L62 18";

pub fn mark(size: u32, animated: bool) -> Markup {
    let cls = if animated { "mark mark-anim" } else { "mark" };
    let svg = format!(
        r#"<svg class="{cls}" width="{size}" height="{size}" viewBox="0 0 90 90" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Mountyhill">
<rect class="mark-tile" x="0" y="0" width="90" height="90" rx="28"/>
<path class="mark-fill" d="{fill}" fill="{navy}"/>
<path class="mark-line" d="{line}" fill="none" stroke="{curve}" stroke-width="2.5" stroke-linejoin="round" stroke-linecap="round"/>
</svg>"#,
        cls = cls,
        size = size,
        fill = RIDGE_FILL,
        line = RIDGE_LINE,
        navy = NAVY,
        curve = CURVE
    );
    PreEscaped(svg)
}

pub fn wordmark() -> Markup {
    html! {
        span class="wordmark" { "MOUNTY" span class="wordmark-accent" { "HILL" } }
    }
}

pub fn lockup(size: u32, animated: bool) -> Markup {
    html! {
        span class="lockup" {
            (mark(size, animated))
            span class="lockup-rule" {}
            (wordmark())
        }
    }
}

pub fn header(nav: &[(&str, &str)]) -> Markup {
    html! {
        header class="site-header" {
            a class="brand" href="/" aria-label="Mountyhill Startseite" {
                (lockup(38, false))
            }
            nav class="site-nav" {
                @for (href, label) in nav.iter() {
                    a href=(href) { (label) }
                }
            }
            button class="nav-toggle" type="button" aria-label="Menü" {
                span {} span {} span {}
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer class="site-footer" {
            div class="footer-inner" {
                div class="footer-brand" {
                    (lockup(34, false))
                    p { "Systematische Kapitalanlage aus dem Siegerland." }
                }
                div class="footer-cols" {
                    div {
                        h4 { "Unternehmen" }
                        a href="/#ansatz" { "Ansatz" }
                        a href="/#strategien" { "Strategien" }
                        a href="/#technologie" { "Technologie" }
                    }
                    div {
                        h4 { "Rechtliches" }
                        a href="/impressum/" { "Impressum" }
                        a href="/datenschutz/" { "Datenschutz" }
                    }
                    div {
                        h4 { "Kontakt" }
                        a href="mailto:kontakt@mountyhill.de" { "kontakt@mountyhill.de" }
                    }
                }
            }
            div class="footer-base" {
                span { "© 2026 Mountyhill" }
                span class="palette" {
                    @for a in ACCENTS.iter() {
                        span class="dot" style=(format!("background:{}", a.tile)) title=(a.name) {}
                    }
                }
            }
        }
    }
}
