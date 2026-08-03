pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed | 1)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.unit() * (hi - lo)
    }

    pub fn normal(&mut self) -> f64 {
        let mut sum = 0.0;
        for _ in 0..6 {
            sum += self.unit();
        }
        (sum - 3.0) / 0.7071
    }
}

pub fn walk(seed: u64, n: usize, drift: f64, vol: f64) -> Vec<f64> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::with_capacity(n);
    let mut v = 1.0f64;
    for i in 0..n {
        let shock = if rng.unit() < 0.012 { -vol * 5.5 } else { 0.0 };
        let regime = ((i as f64) * 0.011).sin() * 0.0004;
        v *= 1.0 + drift + regime + rng.normal() * vol + shock;
        if v < 0.05 {
            v = 0.05;
        }
        out.push(v);
    }
    out
}

pub fn drawdown(series: &[f64]) -> (Vec<f64>, f64) {
    let mut peak = f64::MIN;
    let mut out = Vec::with_capacity(series.len());
    let mut worst = 0.0f64;
    for v in series {
        if *v > peak {
            peak = *v;
        }
        let dd = v / peak - 1.0;
        if dd < worst {
            worst = dd;
        }
        out.push(dd);
    }
    (out, worst)
}

pub fn bounds(series: &[f64]) -> (f64, f64) {
    let mut lo = f64::MAX;
    let mut hi = f64::MIN;
    for v in series {
        if *v < lo {
            lo = *v;
        }
        if *v > hi {
            hi = *v;
        }
    }
    if (hi - lo).abs() < 1e-9 {
        hi = lo + 1.0;
    }
    (lo, hi)
}
