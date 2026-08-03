mod rng;
mod scenes;

use scenes::{Equity, Frame, LiveChart, Markets, MonteCarlo, Palette, Scene, Sizing};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlInputElement};

struct View {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    scene: Box<dyn Scene>,
    value: Option<Element>,
    delta: Option<Element>,
    input: Option<HtmlInputElement>,
    w: f64,
    h: f64,
    visible: bool,
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

fn css_var(name: &str, fallback: &str) -> String {
    let doc = document();
    let root = match doc.document_element() {
        Some(r) => r,
        None => return fallback.to_string(),
    };
    let style = match web_sys::window().unwrap().get_computed_style(&root) {
        Ok(Some(s)) => s,
        _ => return fallback.to_string(),
    };
    match style.get_property_value(name) {
        Ok(v) => {
            let v = v.trim().to_string();
            if v.is_empty() {
                fallback.to_string()
            } else {
                v
            }
        }
        Err(_) => fallback.to_string(),
    }
}

fn palette() -> Palette {
    Palette {
        tile: css_var("--tile", "#6B2049"),
        light: css_var("--light", "#A3305F"),
        curve: css_var("--curve", "#EFE6EC"),
        muted: css_var("--fg-muted", "#9AA3B4"),
        hair: css_var("--hair-solid", "rgba(255,255,255,0.08)"),
    }
}

fn make_scene(kind: &str, seed: u64) -> Box<dyn Scene> {
    match kind {
        "equity" => Box::new(Equity::new(seed)),
        "montecarlo" => Box::new(MonteCarlo::new(seed, 41, 340)),
        "markets" => Box::new(Markets::new(seed, 40)),
        "sizing" => Box::new(Sizing::new(seed)),
        _ => Box::new(LiveChart::new(seed)),
    }
}

fn collect_views() -> Vec<View> {
    let doc = document();
    let nodes = match doc.query_selector_all("canvas[data-viz]") {
        Ok(n) => n,
        Err(_) => return Vec::new(),
    };
    let mut views = Vec::new();
    for i in 0..nodes.length() {
        let node = match nodes.item(i) {
            Some(n) => n,
            None => continue,
        };
        let canvas: HtmlCanvasElement = match node.dyn_into() {
            Ok(c) => c,
            Err(_) => continue,
        };
        let ctx = match canvas.get_context("2d") {
            Ok(Some(c)) => match c.dyn_into::<CanvasRenderingContext2d>() {
                Ok(c) => c,
                Err(_) => continue,
            },
            _ => continue,
        };
        let kind = canvas.get_attribute("data-viz").unwrap_or_default();
        let seed = canvas
            .get_attribute("data-seed")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(20260802);
        let key = canvas.get_attribute("data-out").unwrap_or_default();
        let value = doc.query_selector(&format!("[data-out-value='{}']", key)).ok().flatten();
        let delta = doc.query_selector(&format!("[data-out-delta='{}']", key)).ok().flatten();
        let input = doc
            .query_selector(&format!("[data-input='{}']", key))
            .ok()
            .flatten()
            .and_then(|e| e.dyn_into::<HtmlInputElement>().ok());

        views.push(View {
            scene: make_scene(&kind, seed),
            canvas,
            ctx,
            value,
            delta,
            input,
            w: 0.0,
            h: 0.0,
            visible: true,
        });
    }
    views
}

fn resize(view: &mut View, dpr: f64) {
    let rect = view.canvas.get_bounding_client_rect();
    let w = rect.width().max(1.0);
    let h = rect.height().max(1.0);
    if (w - view.w).abs() < 0.5 && (h - view.h).abs() < 0.5 {
        return;
    }
    view.w = w;
    view.h = h;
    view.canvas.set_width((w * dpr) as u32);
    view.canvas.set_height((h * dpr) as u32);
    let _ = view.ctx.reset_transform();
    let _ = view.ctx.scale(dpr, dpr);
}

fn in_viewport(view: &View) -> bool {
    let rect = view.canvas.get_bounding_client_rect();
    let vh = web_sys::window()
        .unwrap()
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(800.0);
    rect.bottom() > -200.0 && rect.top() < vh + 200.0
}

fn set_text(el: &Option<Element>, text: &str) {
    if let Some(e) = el {
        e.set_text_content(Some(text));
    }
}

fn request_frame(f: &Closure<dyn FnMut(f64)>) {
    let _ = web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref());
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let dpr = web_sys::window()
        .unwrap()
        .device_pixel_ratio()
        .clamp(1.0, 2.5);
    let views = Rc::new(RefCell::new(collect_views()));
    if views.borrow().is_empty() {
        return Ok(());
    }

    let pal = Rc::new(RefCell::new(palette()));
    let last = Rc::new(RefCell::new(0.0f64));
    let counter = Rc::new(RefCell::new(0u32));

    let outer: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let inner = outer.clone();

    let views_cb = views.clone();
    let pal_cb = pal.clone();

    *inner.borrow_mut() = Some(Closure::wrap(Box::new(move |now: f64| {
        let t = now / 1000.0;
        let mut prev = last.borrow_mut();
        let dt = if *prev <= 0.0 {
            1.0 / 60.0
        } else {
            (t - *prev).clamp(0.0, 0.1)
        };
        *prev = t;
        drop(prev);

        let mut c = counter.borrow_mut();
        *c = c.wrapping_add(1);
        let tick = *c;
        drop(c);

        if tick % 15 == 0 {
            *pal_cb.borrow_mut() = palette();
        }

        let pal_now = pal_cb.borrow();
        for view in views_cb.borrow_mut().iter_mut() {
            if tick % 20 == 0 {
                view.visible = in_viewport(view);
            }
            if !view.visible {
                continue;
            }
            resize(view, dpr);
            if let Some(input) = &view.input {
                if let Ok(v) = input.value().parse::<f64>() {
                    view.scene.set_input(v);
                }
            }
            view.ctx.clear_rect(0.0, 0.0, view.w, view.h);
            let mut frame = Frame {
                ctx: &view.ctx,
                w: view.w,
                h: view.h,
                t,
                dt,
                pal: &pal_now,
            };
            view.scene.draw(&mut frame);

            if tick % 4 == 0 {
                if let Some((value, delta, up)) = view.scene.readout() {
                    set_text(&view.value, &value);
                    set_text(&view.delta, &delta);
                    if let Some(el) = &view.delta {
                        let _ = el.set_attribute("data-up", if up { "1" } else { "0" });
                    }
                }
            }
        }
        drop(pal_now);

        if let Some(cb) = outer.borrow().as_ref() {
            request_frame(cb);
        }
    }) as Box<dyn FnMut(f64)>));

    if let Some(cb) = inner.borrow().as_ref() {
        request_frame(cb);
    }
    Ok(())
}
