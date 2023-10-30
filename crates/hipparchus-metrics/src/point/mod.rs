// Space distance calculation for f32/f64 points

// Lp Minkowski family
pub mod manhattan;      // Manhattan distance (p=1), d=∑|Pi−Qi|
pub mod euclidean;      // Euclidean distance (p=2), d=sqrt(∑|Pi−Qi|2)
pub mod chebyshev;      // Chebyshev distance (p=infinite), d=max|Pi−Qi|
pub mod minkowski;      // Minkowski distance, d=(∑|Pi−Qi|p)1/p

// L1 family
                        // TODO: Sorensen distance, d=∑|Pi−Qi|/∑(Pi+Qi)
pub mod gower;          // Gower distance (L1 normalized Manhattan distance), d=1/d∗∑|Pi−Qi|
                        // TODO: Soergel distance, d=∑|Pi−Qi|/∑max(Pi,Qi)
pub mod canberra;       // Canberra distance (weighted Manhattan distance), d=∑|Pi−Qi|/(Pi+Qi)
                        // TODO: Kulczynski-d distance, d=∑|Pi−Qi|/∑min(Pi,Qi)
                        // TODO: Lorentzian distance, d=∑ln(1+|Pi−Qi|)

