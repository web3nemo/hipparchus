use hipparchus_mean::Fp;

pub trait Metrics<In, Out:Fp>
{
    /// Calculate metrics between input data entity
    fn measure(self, a:In, b:In) -> Out;
}
