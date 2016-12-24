use std::i32;

/// Implementation of the extended Wilkinson's algorithm for pretty axis labels

//Reference:
//[1] An Extension of Wilkinson's Algorithm for positioning Tick Labels on Axes
//(Justin Talbot, Sharon Lin, Pat Hanrahan)
pub struct Labeller {
    q: Vec<f64>,
    base: f64,
    w: Vec<f64>,
    eps: f64,
    pub loose: bool,
}

impl Labeller {
    pub fn new(q: Vec<f64>, base: f64, w: Vec<f64>, eps: f64) -> Labeller {
        Labeller {
            q: q,
            base: base,
            w: w,
            eps: eps,

            loose: false,
        }
    }

    pub fn using_base(q: Vec<f64>, base: f64) -> Labeller {
        Labeller::new(q, base, vec!(0.25, 0.2, 0.5, 0.05), 1e-10)
    }
    
    pub fn in_base10() -> Labeller {
        Labeller::using_base(vec!(1.0, 5.0, 2.0, 2.5, 4.0, 3.0), 10.0)
    }
    
    fn w(&self, s: f64, c: f64, d: f64, l: f64) -> f64 {
        let ref weights = self.w;
        (weights[0] * s) + (weights[1] * c) + (weights[2] * d) + (weights[3] * l)
    }
    
    fn log_b(&self, a: f64) -> f64 {
        a.ln() / self.base.ln()
    }
    
    fn floored_mod(&self, a: f64, n: f64) -> f64 {
        a - n * (a/n).floor()
    }
    
    fn v(&self, min: f64, max: f64, step: f64) -> f64 {
        if self.floored_mod(min, step) < self.eps && min <= 0.0 && max >= 0.0 {
            1.0
        } else {
            0.0
        }
    }
    
    fn simplicity(&self, i: i32, j: i32, min: f64, max: f64, step: f64) -> f64 {
        if self.q.len() > 1 {
            1.0 - (i as f64) / ((self.q.len() - 1) as f64) - (j as f64) + self.v(min, max, step)
        } else {
            1.0 - (j as f64) + self.v(min, max, step)
        }
    }
    
    fn simplicity_max(&self, i: i32, j: i32) -> f64 {
        if self.q.len() > 1 {
            1.0 - (i as f64) / ((self.q.len() - 1) as f64) - (j as f64) + 1.0
        } else {
            1.0 - (j as f64) + 1.0
        }
    }
    
    fn coverage(&self, dmin: f64, dmax: f64, lmin: f64, lmax: f64) -> f64 {
        let a = dmax - lmax;
        let b = dmin - lmin;
        let c = 0.1 * (dmax - dmin);
        1.0 - 0.5 * ((a * a + b * b) / (c * c))
    }
    
    fn coverage_max(&self, dmin: f64, dmax: f64, span: f64) -> f64 {
        let range = dmax - dmin;
        if span > range {
            let half = (span - range) / 2.0;
            let r = 0.1 * range;
            1.0 - half * half / (r * r)
        } else {
            1.0
        }
    }
    
    fn density(&self, k: i32, m: i32, dmin: f64, dmax: f64, lmin: f64, lmax: f64) -> f64 {
       let r = ((k - 1) as f64) / (lmax - lmin);
       let rt = ((m - 1) as f64) / (lmax.max(dmax) - lmin.min(dmin));
       2.0 - (r/rt).max(rt/r)
    }
    
    fn density_max(&self, k: i32, m: i32) -> f64 {
       if k >= m {
           2.0 - ((k - 1) as f64) / ((m - 1) as f64)
       } else {
           1.0
       }
    }
    
    // unimplemented
    #[allow(dead_code, unused_variables)]
    fn legibility(&self, min: f64, max: f64, step: f64) -> f64 {
        1.0
    }
    
    pub fn search(&self, dmin: f64, dmax: f64, m: i32) -> Label {
        let mut best = Label::new();
        let mut best_score = -2.0;
        let mut sm;
        let mut dm;
        let mut cm;
        let mut delta;
        let mut j = 1;
        
        'main_loop: while j < i32::MAX {
            for i_1 in 0..self.q.len() {
                let i = i_1 + 1;
                let q = self.q[i_1];
                sm = self.simplicity_max(i as i32, j as i32);
                if self.w(sm, 1.0, 1.0, 1.0) < best_score {
                    break 'main_loop;
                }
                let mut k = 2;
                while k < i32::MAX {
                    dm = self.density_max(k, m);
                    if self.w(sm, 1.0, dm, 1.0) < best_score {
                        break;
                    }
                    delta = (dmax - dmin) / (( k + 1) as f64) / ((j as f64) * q);
                    let mut z = self.log_b(delta).ceil() as i32;
                    while z < i32::MAX {
                        let step = (j as f64) * q * self.base.powi(z);
                        cm = self.coverage_max(dmin, dmax, step * ((k - 1) as f64));
                        if self.w(sm, cm, dm, 1.0) < best_score {
                            break;
                        }
                        let min_start = (dmax / step - ((k - 1) as f64) * (j as f64)).floor() as i32;
                        let max_start = ((dmin / step).ceil() * (j as f64)) as i32;

                        for start in min_start..max_start + 1 {
                            let lmin = (start as f64) * step/(j as f64);
                            let lmax = lmin + step * ((k - 1) as f64);
                            let c = self.coverage(dmin, dmax, lmin, lmax);
                            let s = self.simplicity(i as i32, j, lmin, lmax, step);
                            let d = self.density(k, m, dmin, dmax, lmin, lmax);
                            let l = self.legibility(lmin, lmax, step);
                            let score = self.w(s, c, d, l);

                            if score > best_score && (!self.loose || (lmin <= dmin && lmax >= dmax)) {
                                best.min = lmin;
                                best.max = lmax;
                                best.step = step;
                                best.score = score;
                                best_score = score;
                            }
                        }
                        z = z + 1;
                    }
                    k = k + 1;
                }
            }
            j = j + 1;
        }
        best
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Label {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub score: f64,
}

impl Label {
    fn new() -> Label {
        Label {
           min: 0.0, max: 0.0, step: 0.0, score: 0.0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let mut labeller = Labeller::in_base10();
        labeller.loose = true;

        let label = labeller.search(-98.0, 18.0, 2);
        assert_eq!(label.max, 20.0);
        assert_eq!(label.min, -100.0);
        assert_eq!(label.step, 60.0);

        let label = labeller.search(-25.0, 200.0, 3);
        assert_eq!(label.max, 200.0);
        assert_eq!(label.min, -50.0);
        assert_eq!(label.step, 50.0);
    }
}