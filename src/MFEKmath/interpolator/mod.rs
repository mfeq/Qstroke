mod evaluate;

pub struct Interpolator
{
    start: f64,
    finish: f64,
    func: fn(&Self, f64) -> f64
}

#[derive(Clone, Copy)]
pub enum InterpolationType {
    Null,
    Linear,
    Exponential
}

impl Interpolator {
    fn interpolate_none(&self, t: f64) -> f64 {
        return self.start;
    }

    fn interpolate_linear(&self, t: f64) -> f64 {
        return (1. - t) * self.start + t * self.finish;
    }
    
    fn interpolate_exponential(&self, t: f64) -> f64 {
        return self.start + (self.finish - self.start) * t * t;
    }

    pub fn new(start: f64, finish:f64, kind: InterpolationType) -> Self
    {
        match kind {
            InterpolationType::Null => return Self::new_none(start, finish),
            InterpolationType::Linear => return Self::new_linear(start, finish),
            InterpolationType::Exponential => return Self::new_exponential(start, finish)
        }
    }

    pub fn new_none(start: f64, finish: f64) -> Self
    {
        Interpolator {
            start: start,
            finish: finish,
            func: Self::interpolate_none
        }
    }

    pub fn new_linear(start: f64, finish: f64) -> Self
    {
        Interpolator {
            start: start,
            finish: finish,
            func: Self::interpolate_linear
        }
    }

    pub fn new_exponential(start: f64, finish: f64) -> Self
    {
        Interpolator {
            start: start,
            finish: finish,
            func: Self::interpolate_exponential
        }
    }
}