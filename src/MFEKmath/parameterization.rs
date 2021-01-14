pub trait Parameterization
{
    fn parameterize(&self, u: f64) -> f64;
}
