use crate::rng::{bounds, drawdown, walk, Rng};
use web_sys::CanvasRenderingContext2d;

pub struct Palette {
    pub tile: String,
    pub light: String,
    pub curve: String,
    pub muted: String,
    pub hair: String,
}

pub struct Frame<'a> {
    pub ctx: &'a CanvasRenderingContext2d,
    pub w: f64,
    pub h: f64,
    pub t: f64,
    pub dt: f64,
    pub pal: &'a Palette,
}

pub trait Scene {
    fn draw(&mut self, f: &mut Frame);
    fn readout(&self) -> Option<(String, String, bool)> {
        None
    }
    fn set_input(&mut self, _value: f64) {}
}

fn line(ctx: &CanvasRenderingContext2d, pts: &[(f64, f64)], color: &str, width: f64, alpha: f64) {
    if pts.len() < 2 {
        return;
    }
    ctx.set_global_alpha(alpha);
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(width);
    ctx.set_line_join("round");
    ctx.set_line_cap("round");
    ctx.begin_path();
    ctx.move_to(pts[0].0, pts[0].1);
    for p in pts.iter().skip(1) {
        ctx.line_to(p.0, p.1);
    }
    ctx.stroke();
    ctx.set_global_alpha(1.0);
}

fn area(
    ctx: &CanvasRenderingContext2d,
    pts: &[(f64, f64)],
    base: f64,
    color: &str,
    alpha: f64,
) {
    if pts.len() < 2 {
        return;
    }
    ctx.set_global_alpha(alpha);
    ctx.set_fill_style_str(color);
    ctx.begin_path();
    ctx.move_to(pts[0].0, base);
    for p in pts.iter() {
        ctx.line_to(p.0, p.1);
    }
    ctx.line_to(pts[pts.len() - 1].0, base);
    ctx.close_path();
    ctx.fill();
    ctx.set_global_alpha(1.0);
}

fn grid(f: &Frame, rows: usize) {
    let ctx = f.ctx;
    ctx.set_stroke_style_str(&f.pal.hair);
    ctx.set_line_width(1.0);
    ctx.set_global_alpha(1.0);
    for i in 1..rows {
        let y = f.h * i as f64 / rows as f64;
        ctx.begin_path();
        ctx.move_to(0.0, y.floor() + 0.5);
        ctx.line_to(f.w, y.floor() + 0.5);
        ctx.stroke();
    }
}

pub struct LiveChart {
    series: Vec<f64>,
    rng: Rng,
    acc: f64,
    step: f64,
    last: f64,
    open: f64,
}

impl LiveChart {
    pub fn new(seed: u64) -> LiveChart {
        let series = walk(seed, 260, 0.0009, 0.0075);
        let last = *series.last().unwrap_or(&1.0);
        LiveChart {
            open: series[series.len().saturating_sub(120)],
            series,
            rng: Rng::new(seed ^ 0x9e37),
            acc: 0.0,
            step: 0.09,
            last,
        }
    }
}

impl Scene for LiveChart {
    fn draw(&mut self, f: &mut Frame) {
        self.acc += f.dt;
        while self.acc >= self.step {
            self.acc -= self.step;
            let shock = if self.rng.unit() < 0.02 { -0.012 } else { 0.0 };
            let v = self.last * (1.0 + 0.0009 + self.rng.normal() * 0.0062 + shock);
            self.last = v.max(0.05);
            self.series.push(self.last);
            if self.series.len() > 260 {
                self.series.remove(0);
                self.open = self.series[self.series.len().saturating_sub(120)];
            }
        }

        let ctx = f.ctx;
        let pad = f.h * 0.12;
        let (lo, hi) = bounds(&self.series);
        let span = hi - lo;
        let n = self.series.len() as f64;
        let pts: Vec<(f64, f64)> = self
            .series
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let x = f.w * i as f64 / (n - 1.0).max(1.0);
                let y = f.h - pad - (v - lo) / span * (f.h - pad * 2.0);
                (x, y)
            })
            .collect();

        grid(f, 4);
        area(ctx, &pts, f.h, &f.pal.tile, 0.55);
        line(ctx, &pts, &f.pal.curve, 2.0, 0.95);

        if let Some(p) = pts.last() {
            let pulse = 3.0 + (f.t * 3.4).sin() * 1.4;
            ctx.set_global_alpha(0.28);
            ctx.set_fill_style_str(&f.pal.curve);
            ctx.begin_path();
            let _ = ctx.arc(p.0, p.1, pulse * 2.4, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.set_global_alpha(1.0);
            ctx.begin_path();
            let _ = ctx.arc(p.0, p.1, 3.2, 0.0, std::f64::consts::TAU);
            ctx.fill();

            ctx.set_stroke_style_str(&f.pal.light);
            ctx.set_line_width(1.0);
            ctx.set_global_alpha(0.5);
            ctx.begin_path();
            ctx.move_to(0.0, p.1.floor() + 0.5);
            ctx.line_to(f.w, p.1.floor() + 0.5);
            ctx.stroke();
            ctx.set_global_alpha(1.0);
        }
    }

    fn readout(&self) -> Option<(String, String, bool)> {
        let change = self.last / self.open - 1.0;
        let up = change >= 0.0;
        Some((
            format!("{:.2}", self.last * 1000.0),
            format!("{}{:.2} %", if up { "+" } else { "" }, change * 100.0),
            up,
        ))
    }
}

pub struct Equity {
    series: Vec<f64>,
    dd: Vec<f64>,
    worst: f64,
    progress: f64,
    hold: f64,
}

impl Equity {
    pub fn new(seed: u64) -> Equity {
        let series = walk(seed, 520, 0.0016, 0.0098);
        let (dd, worst) = drawdown(&series);
        Equity {
            series,
            dd,
            worst,
            progress: 0.0,
            hold: 0.0,
        }
    }
}

impl Scene for Equity {
    fn draw(&mut self, f: &mut Frame) {
        if self.progress < 1.0 {
            self.progress = (self.progress + f.dt * 0.28).min(1.0);
        } else {
            self.hold += f.dt;
            if self.hold > 2.6 {
                self.hold = 0.0;
                self.progress = 0.0;
            }
        }

        let count = ((self.series.len() as f64) * self.progress).max(2.0) as usize;
        let visible = &self.series[..count];
        let (lo, hi) = bounds(&self.series);
        let span = hi - lo;
        let pad = f.h * 0.16;
        let n = self.series.len() as f64;

        let map = |i: usize, v: f64| -> (f64, f64) {
            let x = f.w * i as f64 / (n - 1.0);
            let y = f.h - pad - (v - lo) / span * (f.h - pad * 2.0);
            (x, y)
        };

        grid(f, 4);

        let mut peak = f64::MIN;
        let mut top: Vec<(f64, f64)> = Vec::with_capacity(count);
        let mut cur: Vec<(f64, f64)> = Vec::with_capacity(count);
        for (i, v) in visible.iter().enumerate() {
            if *v > peak {
                peak = *v;
            }
            top.push(map(i, peak));
            cur.push(map(i, *v));
        }

        let ctx = f.ctx;
        ctx.set_global_alpha(0.22);
        ctx.set_fill_style_str(&f.pal.light);
        ctx.begin_path();
        ctx.move_to(top[0].0, top[0].1);
        for p in top.iter() {
            ctx.line_to(p.0, p.1);
        }
        for p in cur.iter().rev() {
            ctx.line_to(p.0, p.1);
        }
        ctx.close_path();
        ctx.fill();
        ctx.set_global_alpha(1.0);

        line(ctx, &top, &f.pal.light, 1.0, 0.45);
        area(ctx, &cur, f.h, &f.pal.tile, 0.30);
        line(ctx, &cur, &f.pal.curve, 2.0, 1.0);

        if let Some(p) = cur.last() {
            ctx.set_fill_style_str(&f.pal.curve);
            ctx.begin_path();
            let _ = ctx.arc(p.0, p.1, 3.0, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }
    }

    fn readout(&self) -> Option<(String, String, bool)> {
        let count = ((self.series.len() as f64) * self.progress).max(1.0) as usize;
        let cur = self.dd[..count].iter().cloned().fold(0.0f64, f64::min);
        Some((
            format!("{:.1} %", cur * 100.0),
            format!("max {:.1} %", self.worst * 100.0),
            false,
        ))
    }
}

pub struct MonteCarlo {
    paths: Vec<Vec<f64>>,
    progress: f64,
    hold: f64,
    lo: f64,
    hi: f64,
}

impl MonteCarlo {
    pub fn new(seed: u64, count: usize, len: usize) -> MonteCarlo {
        let mut paths = Vec::with_capacity(count);
        let mut lo = f64::MAX;
        let mut hi = f64::MIN;
        for i in 0..count {
            let p = walk(seed.wrapping_add(i as u64 * 7919), len, 0.0016, 0.011);
            let (l, h) = bounds(&p);
            if l < lo {
                lo = l;
            }
            if h > hi {
                hi = h;
            }
            paths.push(p);
        }
        MonteCarlo {
            paths,
            progress: 0.0,
            hold: 0.0,
            lo,
            hi,
        }
    }
}

impl Scene for MonteCarlo {
    fn draw(&mut self, f: &mut Frame) {
        if self.progress < 1.0 {
            self.progress = (self.progress + f.dt * 0.24).min(1.0);
        } else {
            self.hold += f.dt;
            if self.hold > 2.4 {
                self.hold = 0.0;
                self.progress = 0.0;
            }
        }

        let len = self.paths[0].len();
        let count = ((len as f64) * self.progress).max(2.0) as usize;
        let pad = f.h * 0.14;
        let span = self.hi - self.lo;
        grid(f, 4);

        let ctx = f.ctx;
        for (k, path) in self.paths.iter().enumerate() {
            let pts: Vec<(f64, f64)> = path[..count]
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let x = f.w * i as f64 / (len - 1) as f64;
                    let y = f.h - pad - (v - self.lo) / span * (f.h - pad * 2.0);
                    (x, y)
                })
                .collect();
            let mid = k == self.paths.len() / 2;
            if mid {
                line(ctx, &pts, &f.pal.curve, 2.0, 1.0);
            } else {
                line(ctx, &pts, &f.pal.light, 1.0, 0.16);
            }
        }
    }

    fn readout(&self) -> Option<(String, String, bool)> {
        let len = self.paths[0].len();
        let count = ((len as f64) * self.progress).max(1.0) as usize;
        let mut worst = 0.0f64;
        for p in self.paths.iter() {
            let (_, w) = drawdown(&p[..count]);
            if w < worst {
                worst = w;
            }
        }
        Some((
            format!("{}", self.paths.len()),
            format!("worst {:.0} %", worst * 100.0),
            false,
        ))
    }
}

pub struct Markets {
    cells: Vec<Vec<f64>>,
    rng: Rng,
    acc: f64,
    up: usize,
}

impl Markets {
    pub fn new(seed: u64, count: usize) -> Markets {
        let mut cells = Vec::with_capacity(count);
        for i in 0..count {
            cells.push(walk(seed.wrapping_add(i as u64 * 104729), 44, 0.0012, 0.013));
        }
        Markets {
            cells,
            rng: Rng::new(seed ^ 0x5bd1),
            acc: 0.0,
            up: 0,
        }
    }
}

impl Scene for Markets {
    fn draw(&mut self, f: &mut Frame) {
        self.acc += f.dt;
        if self.acc > 0.22 {
            self.acc = 0.0;
            for cell in self.cells.iter_mut() {
                let last = *cell.last().unwrap_or(&1.0);
                let v = last * (1.0 + self.rng.normal() * 0.0105);
                cell.push(v.max(0.05));
                cell.remove(0);
            }
        }

        let cols = if f.w > 900.0 {
            8
        } else if f.w > 620.0 {
            6
        } else {
            4
        };
        let rows = (self.cells.len() + cols - 1) / cols;
        let gap = 10.0;
        let cw = (f.w - gap * (cols as f64 - 1.0)) / cols as f64;
        let ch = (f.h - gap * (rows as f64 - 1.0)) / rows as f64;

        let ctx = f.ctx;
        let mut up_count = 0;
        for (idx, cell) in self.cells.iter().enumerate() {
            let cx = (idx % cols) as f64 * (cw + gap);
            let cy = (idx / cols) as f64 * (ch + gap);
            let (lo, hi) = bounds(cell);
            let span = hi - lo;
            let rising = cell[cell.len() - 1] >= cell[0];
            if rising {
                up_count += 1;
            }
            let pad = ch * 0.22;
            let pts: Vec<(f64, f64)> = cell
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let x = cx + cw * i as f64 / (cell.len() - 1) as f64;
                    let y = cy + ch - pad - (v - lo) / span * (ch - pad * 2.0);
                    (x, y)
                })
                .collect();

            ctx.set_global_alpha(1.0);
            ctx.set_stroke_style_str(&f.pal.hair);
            ctx.set_line_width(1.0);
            ctx.stroke_rect(cx + 0.5, cy + 0.5, cw - 1.0, ch - 1.0);

            let col = if rising { &f.pal.light } else { &f.pal.muted };
            area(ctx, &pts, cy + ch - 1.0, col, 0.12);
            line(ctx, &pts, col, 1.4, 0.95);
        }
        self.up = up_count;
    }

    fn readout(&self) -> Option<(String, String, bool)> {
        Some((
            format!("{}", self.cells.len()),
            format!("{} steigend", self.up),
            true,
        ))
    }
}

pub struct Sizing {
    base: Vec<f64>,
    risk: f64,
    shown: f64,
}

impl Sizing {
    pub fn new(seed: u64) -> Sizing {
        Sizing {
            base: walk(seed, 420, 0.0015, 0.0105),
            risk: 0.4,
            shown: 0.4,
        }
    }

    fn scaled(&self) -> Vec<f64> {
        let f = 0.35 + self.shown * 2.6;
        let mut out = Vec::with_capacity(self.base.len());
        let mut v = 1.0f64;
        let mut prev = self.base[0];
        for p in self.base.iter() {
            let r = p / prev - 1.0;
            prev = *p;
            v *= 1.0 + r * f;
            out.push(v.max(0.02));
        }
        out
    }
}

impl Scene for Sizing {
    fn set_input(&mut self, value: f64) {
        self.risk = (value / 100.0).clamp(0.0, 1.0);
    }

    fn draw(&mut self, f: &mut Frame) {
        self.shown += (self.risk - self.shown) * (f.dt * 6.0).min(1.0);
        let series = self.scaled();
        let (lo, hi) = bounds(&series);
        let span = hi - lo;
        let pad = f.h * 0.16;
        let n = series.len() as f64;
        let pts: Vec<(f64, f64)> = series
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let x = f.w * i as f64 / (n - 1.0);
                let y = f.h - pad - (v - lo) / span * (f.h - pad * 2.0);
                (x, y)
            })
            .collect();

        grid(f, 4);
        let mut peak = f64::MIN;
        let top: Vec<(f64, f64)> = series
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if *v > peak {
                    peak = *v;
                }
                let x = f.w * i as f64 / (n - 1.0);
                let y = f.h - pad - (peak - lo) / span * (f.h - pad * 2.0);
                (x, y)
            })
            .collect();

        let ctx = f.ctx;
        ctx.set_global_alpha(0.20);
        ctx.set_fill_style_str(&f.pal.light);
        ctx.begin_path();
        ctx.move_to(top[0].0, top[0].1);
        for p in top.iter() {
            ctx.line_to(p.0, p.1);
        }
        for p in pts.iter().rev() {
            ctx.line_to(p.0, p.1);
        }
        ctx.close_path();
        ctx.fill();
        ctx.set_global_alpha(1.0);

        area(ctx, &pts, f.h, &f.pal.tile, 0.28);
        line(ctx, &pts, &f.pal.curve, 2.0, 1.0);
    }

    fn readout(&self) -> Option<(String, String, bool)> {
        let series = self.scaled();
        let (_, worst) = drawdown(&series);
        let end = series[series.len() - 1];
        Some((
            format!("{:.0} %", self.shown * 100.0),
            format!("Rückgang {:.0} % · Endwert {:.1}×", worst * 100.0, end),
            end >= 1.0,
        ))
    }
}
