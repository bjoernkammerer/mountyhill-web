use crate::components::{footer, header, lockup, mark};
use crate::theme::{accent, theme_css};
use maud::{html, Markup, PreEscaped, DOCTYPE};

const NAV: [(&str, &str); 5] = [
    ("/#ruecksetzer", "Rücksetzer"),
    ("/#robustheit", "Robustheit"),
    ("/#maerkte", "Märkte"),
    ("/#risiko", "Risiko"),
    ("/#ablauf", "Workflow"),
];

pub fn shell(title: &str, description: &str, body: Markup) -> String {
    let page = html! {
        (DOCTYPE)
        html lang="de" class="t-beere" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " — Mountyhill" }
                meta name="description" content=(description);
                meta property="og:title" content=(title);
                meta property="og:description" content=(description);
                link rel="icon" href="/assets/favicon.ico" sizes="any";
                link rel="stylesheet" href="/style.css";
                style { (PreEscaped(theme_css())) }
            }
            body {
                a class="skip" href="#main" { "Zum Inhalt springen" }
                (header(&NAV))
                main id="main" { (body) }
                (footer())
                script src="/app.js" defer {}
                script type="module" {
                    (PreEscaped("import init from '/wasm/viz.js'; init().catch(function(e){console.warn('viz', e);});"))
                }
            }
        }
    };
    page.into_string()
}

fn canvas(kind: &str, seed: u64, out: &str, class: &str) -> Markup {
    html! {
        canvas class=(format!("viz {}", class))
               data-viz=(kind)
               data-seed=(seed.to_string())
               data-out=(out)
               aria-hidden="true" {}
    }
}

fn readout(key: &str, caption: &str) -> Markup {
    html! {
        div class="readout" {
            span class="readout-value" data-out-value=(key) { "—" }
            span class="readout-delta" data-out-delta=(key) { "—" }
            span class="readout-caption" { (caption) }
        }
    }
}

fn hero() -> Markup {
    html! {
        section class="hero" data-accent="beere" {
            div class="hero-canvas" { (canvas("live", 20260802, "hero", "fill")) }
            div class="hero-fade" aria-hidden="true" {}
            div class="hero-inner" {
                div class="hero-mark" { (mark(96, true)) }
                h1 class="hero-title" { "MOUNTY" span class="accent" { "HILL" } }
                p class="hero-sub" { "Mountyhill" }
                div class="tape" {
                    div class="tape-cell" {
                        span class="tape-label" { "MH Composite" }
                        span class="tape-value" data-out-value="hero" { "—" }
                    }
                    div class="tape-cell" {
                        span class="tape-label" { "Tag" }
                        span class="tape-delta" data-out-delta="hero" { "—" }
                    }
                    div class="tape-cell" {
                        span class="tape-label" { "Status" }
                        span class="tape-live" { span class="pip" {} "live" }
                    }
                }
            }
            a class="scroll-hint" href="#ruecksetzer" aria-label="Weiter" { span {} }
        }
    }
}

fn viz_section(
    id: &str,
    accent_key: &str,
    index: &str,
    title: &str,
    caption: &str,
    body: Markup,
    alt: bool,
) -> Markup {
    let cls = if alt {
        format!("section viz-section alt t-{}", accent_key)
    } else {
        format!("section viz-section t-{}", accent_key)
    };
    html! {
        section id=(id) class=(cls) data-accent=(accent_key) {
            div class="wrap" {
                div class="viz-head reveal" {
                    span class="section-index" { (index) }
                    h2 { (title) }
                    p class="caption" { (caption) }
                }
                div class="viz-body reveal" { (body) }
            }
        }
    }
}

fn ruecksetzer() -> Markup {
    viz_section(
        "ruecksetzer",
        "tanne",
        "01",
        "Rücksetzer",
        "Volatilität",
        html! {
            div class="viz-frame" {
                (canvas("equity", 771103, "equity", "tall"))
                (readout("equity", "Drawdown"))
            }
        },
        false,
    )
}

fn robustheit() -> Markup {
    viz_section(
        "robustheit",
        "petrol",
        "02",
        "Robustheit",
        "Montecarlo",
        html! {
            div class="viz-frame" {
                (canvas("montecarlo", 424242, "mc", "tall"))
                (readout("mc", "simulierte Pfade"))
            }
        },
        true,
    )
}

fn maerkte() -> Markup {
    viz_section(
        "maerkte",
        "wald",
        "03",
        "Märkte",
        "Ticker",
        html! {
            div class="viz-frame" {
                (canvas("markets", 909090, "mk", "grid"))
                (readout("mk", "Instrumente"))
            }
        },
        false,
    )
}

fn risiko() -> Markup {
    viz_section(
        "risiko",
        "kupfer",
        "04",
        "Risiko",
        "Positionsizing",
        html! {
            div class="viz-frame" {
                (canvas("sizing", 5150, "sz", "tall"))
                div class="control" {
                    label for="risk" { "Risiko je Position" }
                    input id="risk" type="range" min="5" max="100" value="40" step="1"
                          data-input="sz" aria-label="Risiko je Position";
                    (readout("sz", "Ergebnis über 420 Tage"))
                }
            }
        },
        true,
    )
}

fn ablauf() -> Markup {
    let steps = [
        ("Kurse", "Import und Prüfung"),
        ("Prüfung", "Plausibilität"),
        ("Modell", "Training"),
        ("Signal", "Ermittlung"),
        ("Sizing", "Risikobudget"),
        ("Order", "Ausführung"),
    ];
    html! {
        section id="ablauf" class="section t-stahl" data-accent="stahl" {
            div class="wrap" {
                div class="viz-head reveal" {
                    span class="section-index" { "05" }
                    h2 { "Workflow" }
                    p class="caption" { "Vom Kurs zur Order ohne Handgriff." }
                }
                ol class="pipeline" {
                    @for (i, (name, sub)) in steps.iter().enumerate() {
                        li class="pipe-step reveal" style=(format!("--delay:{}ms", i * 90)) {
                            span class="pipe-dot" {}
                            span class="pipe-name" { (name) }
                            span class="pipe-sub" { (sub) }
                        }
                    }
                    li class="pipe-flow" aria-hidden="true" { span {} span {} span {} }
                }
            }
        }
    }
}

fn kontakt() -> Markup {
    html! {
        section id="kontakt" class="section contact t-burgund" data-accent="burgund" {
            div class="wrap narrow" {
                div class="contact-inner reveal" {
                    (mark(72, false))
                    h2 { "Fragen dazu?" }
                    a class="btn btn-primary" href="mailto:kontakt@mountyhill.de" {
                        "kontakt@mountyhill.de"
                    }
                    p class="disclaimer" {
                        "Alle Darstellungen auf dieser Seite sind simuliert und dienen der "
                        "Veranschaulichung der Methodik. Sie zeigen keine tatsächliche "
                        "Wertentwicklung. Diese Seite ist kein Angebot und keine Anlageberatung."
                    }
                }
            }
        }
    }
}

pub fn index() -> String {
    let body = html! {
        (hero())
        (ruecksetzer())
        (robustheit())
        (maerkte())
        (risiko())
        (ablauf())
        (kontakt())
    };
    shell(
        "Trading",
        "Trading",
        body,
    )
}

fn legal(title: &str, blocks: &[(&str, &str)]) -> String {
    let a = accent("stahl");
    let body = html! {
        section class="section legal t-stahl" data-accent="stahl"
                style=(format!("--tile:{};--light:{}", a.tile, a.light)) {
            div class="wrap narrow" {
                div class="viz-head reveal" {
                    span class="section-index" { "§" }
                    h2 { (title) }
                }
                @for (heading, text) in blocks.iter() {
                    div class="legal-block reveal" {
                        h3 { (heading) }
                        p { (text) }
                    }
                }
            }
        }
    };
    shell(title, title, body)
}

pub fn impressum() -> String {
    legal(
        "Impressum",
        &[
            ("Angaben gemäß § 5 DDG", ""),
            ("Vertreten durch", ""),
            ("Kontakt", "kontakt@mountyhill.de"),
            ("Registereintrag", "Amtsgericht , HRB 00000."),
            ("Umsatzsteuer-Identifikationsnummer", "Gemäß § 27a UStG: DE000000000."),
            ("Haftung für Inhalte", "Für eigene Inhalte sind wir nach den allgemeinen Gesetzen verantwortlich. Eine Pflicht zur Überwachung übermittelter oder gespeicherter fremder Informationen besteht nicht."),
            ("Haftung für Links", "Für die Inhalte verlinkter externer Seiten ist stets deren Anbieter verantwortlich."),
        ],
    )
}

pub fn datenschutz() -> String {
    legal(
        "Datenschutz",
        &[
            ("Verantwortliche Stelle", ""),
            ("Umfang der Verarbeitung", "Die Seite ist statisch. Es werden keine Cookies gesetzt, keine Analysedienste eingebunden und keine Inhalte von Dritten nachgeladen. Alle Darstellungen werden lokal im Browser berechnet."),
            ("Server-Logfiles", "Der Hosting-Anbieter erhebt technisch notwendige Zugriffsdaten. Rechtsgrundlage ist Art. 6 Abs. 1 lit. f DSGVO."),
            ("Kontaktaufnahme", "Bei einer Anfrage per E-Mail verarbeiten wir die übermittelten Daten ausschließlich zur Bearbeitung. Rechtsgrundlage ist Art. 6 Abs. 1 lit. b beziehungsweise lit. f DSGVO."),
            ("Ihre Rechte", "Auskunft, Berichtigung, Löschung, Einschränkung, Datenübertragbarkeit, Widerspruch sowie Beschwerde bei einer Aufsichtsbehörde."),
        ],
    )
}

pub fn not_found() -> String {
    let body = html! {
        section class="section notfound t-petrol" data-accent="petrol" {
            div class="wrap narrow" {
                div class="reveal" {
                    (lockup(52, false))
                    h2 { "Seite nicht gefunden" }
                    a class="btn btn-primary" href="/" { "Zur Startseite" }
                }
            }
        }
    };
    shell("Seite nicht gefunden", "Seite nicht gefunden", body)
}
