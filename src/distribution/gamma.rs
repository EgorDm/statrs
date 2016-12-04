use rand::Rng;
use rand::distributions::{Sample, IndependentSample};
use function::gamma;
use statistics::*;
use distribution::{Univariate, Continuous, Distribution};
use result::Result;
use error::StatsError;
use Float;

/// Implements the [Gamma](https://en.wikipedia.org/wiki/Gamma_distribution) distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{Gamma, Continuous};
/// use statrs::statistics::Mean;
/// use statrs::prec;
///
/// let n = Gamma::new(3.0, 1.0).unwrap();
/// assert_eq!(n.mean(), 3.0);
/// assert!(prec::almost_eq(n.pdf(2.0), 0.270670566473225383788, 1e-15));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gamma<T>
    where T: Float
{
    shape: T,
    rate: T,
}

impl<T> Gamma<T>
    where T: Float
{
    /// Constructs a new gamma distribution with a shape (α)
    /// of `shape` and a rate (β) of `rate`
    ///
    /// # Errors
    ///
    /// Returns an error if `shape` or `rate` are `NaN`.
    /// Also returns an error if `shape <= 0.0` or `rate <= 0.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Gamma;
    ///
    /// let mut result = Gamma::new(3f64, 1.0);
    /// assert!(result.is_ok());
    ///
    /// let result = Gamma::new(0f64, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(shape: T, rate: T) -> Result<Gamma<T>> {
        if !valid_gamma_parameters(shape, rate) {
            Err(StatsError::BadParams)
        } else {
            Ok(Gamma {
                shape: shape,
                rate: rate,
            })
        }
    }

    /// Returns the shape (α) of the gamma distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Gamma;
    ///
    /// let n = Gamma::new(3f64, 1.0).unwrap();
    /// assert_eq!(n.shape(), 3.0);
    /// ```
    pub fn shape(&self) -> T {
        self.shape
    }

    /// Returns the rate (β) of the gamma distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Gamma;
    ///
    /// let n = Gamma::new(3f64, 1.0).unwrap();
    /// assert_eq!(n.rate(), 1.0);
    /// ```
    pub fn rate(&self) -> T {
        self.rate
    }
}

impl<T> Sample<T> for Gamma<T>
    where T: Float
{
    /// Generate a random sample from a gamma
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn sample<R: Rng>(&mut self, r: &mut R) -> T {
        super::Distribution::sample(self, r)
    }
}

impl<T> IndependentSample<T> for Gamma<T>
    where T: Float
{
    /// Generate a random independent sample from a gamma
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn ind_sample<R: Rng>(&self, r: &mut R) -> T {
        super::Distribution::sample(self, r)
    }
}

impl<T> Distribution<T> for Gamma<T>
    where T: Float
{
    /// Generate a random sample from a gamma distribution using
    /// `r` as the source of randomness. The implementation is based
    /// on:
    /// <br />
    /// <div>
    /// <i>"A Simple Method for Generating Gamma Variables"</i> - Marsaglia & Tsang
    /// </div>
    /// <div>
    /// ACM Transactions on Mathematical Software, Vol. 26, No. 3, September 2000, Pages 363-372
    /// </div>
    /// <br />
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    /// use rand::StdRng;
    /// use statrs::distribution::{Gamma, Distribution};
    ///
    /// # fn main() {
    /// let mut r = rand::StdRng::new().unwrap();
    /// let n = Gamma::new(3f64, 1.0).unwrap();
    /// print!("{}", n.sample::<StdRng>(&mut r));
    /// # }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> T {
        sample_unchecked(r, self.shape, self.rate)
    }
}

impl<T> Univariate<T, T> for Gamma<T>
    where T: Float
{
    /// Calculates the cumulative distribution function for the gamma distribution
    /// at `x`
    ///
    /// # Panics
    ///
    /// If `x <= 0.0`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (1 / Γ(α)) * γ(α, β * x)
    /// ```
    ///
    /// where `α` is the shape, `β` is the rate, `Γ` is the gamma function,
    /// and `γ` is the lower incomplete gamma function
    fn cdf(&self, x: T) -> T {
        assert!(x > T::zero(),
                format!("{}", StatsError::ArgMustBePositive("x")));
        if x == self.shape && self.rate == T::infinity() {
            T::one()
        } else if self.rate == T::infinity() {
            T::zero()
        } else {
            gamma::gamma_lr(self.shape, x * self.rate)
        }
    }
}

impl<T> Min<T> for Gamma<T>
    where T: Float
{
    /// Returns the minimum value in the domain of the
    /// gamma distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 0
    /// ```
    fn min(&self) -> T {
        T::zero()
    }
}

impl<T> Max<T> for Gamma<T>
    where T: Float
{
    /// Returns the maximum value in the domain of the
    /// gamma distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// INF
    /// ```
    fn max(&self) -> T {
        T::infinity()
    }
}

impl<T> Mean<T> for Gamma<T>
    where T: Float
{
    /// Returns the mean of the gamma distribution
    ///
    /// # Remarks
    ///
    /// Returns `shape` if `rate == f64::INFINITY`. This behavior
    /// is borrowed from the Math.NET implementation
    ///
    /// # Formula
    ///
    /// ```ignore
    /// α / β
    /// ```
    ///
    /// where `α` is the shape and `β` is the rate
    fn mean(&self) -> T {
        if self.rate == T::infinity() {
            self.shape
        } else {
            self.shape / self.rate
        }
    }
}

impl<T> Variance<T> for Gamma<T>
    where T: Float
{
    /// Returns the variance of the gamma distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// α / β^2
    /// ```
    ///
    /// where `α` is the shape and `β` is the rate
    fn variance(&self) -> T {
        if self.rate == T::infinity() {
            T::zero()
        } else {
            self.shape / (self.rate * self.rate)
        }
    }

    /// Returns the standard deviation of the gamma distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(α) / β
    /// ```
    ///
    /// where `α` is the shape and `β` is the rate
    fn std_dev(&self) -> T {
        self.variance().sqrt()
    }
}

impl<T> Entropy<T> for Gamma<T>
    where T: Float
{
    /// Returns the entropy of the gamma distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// α - ln(β) + ln(Γ(α)) + (1 - α) * ψ(α)
    /// ```
    ///
    /// where `α` is the shape, `β` is the rate, `Γ` is the gamma function,
    /// and `ψ` is the digamma function
    fn entropy(&self) -> T {
        if self.rate == T::infinity() {
            T::zero()
        } else {
            self.shape - self.rate.ln() + gamma::ln_gamma(self.shape) +
            (T::one() - self.shape) * gamma::digamma(self.shape)
        }
    }
}

impl<T> Skewness<T> for Gamma<T>
    where T: Float
{
    /// Returns the skewness of the gamma distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 2 / sqrt(α)
    /// ```
    ///
    /// where `α` is the shape
    fn skewness(&self) -> T {
        T::from(2.0).unwrap() / self.shape.sqrt()
    }
}

impl<T> Mode<T> for Gamma<T>
    where T: Float
{
    /// Returns the mode for the gamma distribution
    ///
    /// # Remarks
    ///
    /// Returns `shape` if `rate ==f64::INFINITY`. This behavior
    /// is borrowed from the Math.NET implementation
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (α - 1) / β
    /// ```
    ///
    /// where `α` is the shape and `β` is the rate
    fn mode(&self) -> T {
        if self.rate == T::infinity() {
            self.shape
        } else {
            (self.shape - T::one()) / self.rate
        }
    }
}

impl<T> Continuous<T, T> for Gamma<T>
    where T: Float
{
    /// Calculates the probability density function for the gamma distribution
    /// at `x`
    ///
    /// # Panics
    ///
    /// If `x <= 0.0`
    ///
    /// # Remarks
    ///
    /// Returns `f64::INFINITY` if `x == shape && rate == f64::INFINITY`
    /// Otherwise returns `0.0` if `rate == f64::INFINITY`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (β^α / Γ(α)) * x^(α - 1) * e ^(-β * x)
    /// ```
    ///
    /// where `α` is the shape, `β` is the rate, and `Γ` is the gamma function
    fn pdf(&self, x: T) -> T {
        assert!(x > T::zero(),
                format!("{}", StatsError::ArgMustBePositive("x")));
        if x == self.shape && self.rate == T::infinity() {
            T::infinity()
        } else if self.rate == T::infinity() {
            T::zero()
        } else if self.shape == T::one() {
            self.rate * (-self.rate * x).exp()
        } else if self.shape > T::from(160.0).unwrap() {
            self.ln_pdf(x).exp()
        } else {
            self.rate.powf(self.shape) * x.powf(self.shape - T::one()) * (-self.rate * x).exp() /
            gamma::gamma(self.shape)
        }
    }

    /// Calculates the log probability density function for the gamma distribution
    /// at `x`
    ///
    /// # Panics
    ///
    /// If `x <= 0.0`
    ///
    /// # Remarks
    ///
    /// Returns `f64::INFINITY` if `x == shape && rate == f64::INFINITY`
    /// Otherwise returns `f64::NEG_INFINITY` if `rate == f64::INFINITY`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln((β^α / Γ(α)) * x^(α - 1) * e ^(-β * x))
    /// ```
    ///
    /// where `α` is the shape, `β` is the rate, and `Γ` is the gamma function
    fn ln_pdf(&self, x: T) -> T {
        assert!(x > T::zero(),
                format!("{}", StatsError::ArgMustBePositive("x")));
        if x == self.shape && self.rate == T::infinity() {
            T::infinity()
        } else if self.rate == T::infinity() {
            T::neg_infinity()
        } else if self.shape == T::one() {
            self.rate.ln() - self.rate * x
        } else {
            self.shape * self.rate.ln() + (self.shape - T::one()) * x.ln() - self.rate * x -
            gamma::ln_gamma(self.shape)
        }
    }
}

/// Samples from a gamma distribution with a shape of `shape` and a
/// rate of `rate` using `r` as the source of randomness. Implementation from:
/// <br />
/// <div>
/// <i>"A Simple Method for Generating Gamma Variables"</i> - Marsaglia & Tsang
/// </div>
/// <div>
/// ACM Transactions on Mathematical Software, Vol. 26, No. 3, September 2000, Pages 363-372
/// </div>
/// <br />
pub fn sample_unchecked<T, R>(r: &mut R, shape: T, rate: T) -> T
    where T: Float,
          R: Rng
{
    if rate == T::infinity() {
        return shape;
    }

    let a = if shape < T::one() {
        shape + T::one()
    } else {
        shape
    };
    let afix = if shape < T::one() {
        r.gen::<T>().powf(T::one() / shape)
    } else {
        T::one()
    };
    let d = a - T::from(1.0 / 3.0).unwrap();
    let c = T::one() / (T::from(9.0).unwrap() * d).sqrt();
    loop {
        let mut x = super::normal::sample_unchecked(r, T::zero(), T::one());
        let mut v = T::one() + c * x;
        while v <= T::zero() {
            x = super::normal::sample_unchecked(r, T::zero(), T::one());
            v = T::one() + c * x;
        }

        v = v * v * v;
        x = x * x;
        let u = r.gen::<T>();
        if u < T::one() - T::from(0.0331).unwrap() * x * x {
            return afix * d * v / rate;
        }
        if u.ln() < T::from(0.5).unwrap() * x + d * (T::one() - v - v.ln()) {
            return afix * d * v / rate;
        }
    }
}

// Returns if `shape_a` and `shape_b` are valid parameters
// for a gamma distribution
fn valid_gamma_parameters<T>(shape: T, rate: T) -> bool
    where T: Float
{
    if shape.is_nan() || rate.is_nan() {
        false
    } else if shape <= T::zero() || rate <= T::zero() {
        false
    } else {
        true
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64;
    use statistics::*;
    use distribution::{Univariate, Continuous, Gamma};

    fn try_create(shape: f64, rate: f64) -> Gamma<f64> {
        let n = Gamma::new(shape, rate);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn create_case(shape: f64, rate: f64) {
        let n = try_create(shape, rate);
        assert_eq!(shape, n.shape());
        assert_eq!(rate, n.rate());
    }

    fn bad_create_case(shape: f64, rate: f64) {
        let n = Gamma::new(shape, rate);
        assert!(n.is_err());
    }

    fn get_value<F>(shape: f64, rate: f64, eval: F) -> f64
        where F: Fn(Gamma<f64>) -> f64
    {
        let n = try_create(shape, rate);
        eval(n)
    }

    fn test_case<F>(shape: f64, rate: f64, expected: f64, eval: F)
        where F: Fn(Gamma<f64>) -> f64
    {
        let x = get_value(shape, rate, eval);
        assert_eq!(expected, x);
    }

    fn test_almost<F>(shape: f64, rate: f64, expected: f64, acc: f64, eval: F)
        where F: Fn(Gamma<f64>) -> f64
    {
        let x = get_value(shape, rate, eval);
        assert_almost_eq!(expected, x, acc);
    }

    #[test]
    fn test_create() {
        create_case(1.0, 0.1);
        create_case(1.0, 1.0);
        create_case(10.0, 10.0);
        create_case(10.0, 1.0);
        create_case(10.0, f64::INFINITY);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(0.0, 0.0);
        bad_create_case(1.0, f64::NAN);
        bad_create_case(1.0, -1.0);
        bad_create_case(-1.0, 1.0);
        bad_create_case(-1.0, -1.0);
        bad_create_case(-1.0, f64::NAN);
    }

    #[test]
    fn test_mean() {
        test_case(1.0, 0.1, 10.0, |x| x.mean());
        test_case(1.0, 1.0, 1.0, |x| x.mean());
        test_case(10.0, 10.0, 1.0, |x| x.mean());
        test_case(10.0, 1.0, 10.0, |x| x.mean());
        test_case(10.0, f64::INFINITY, 10.0, |x| x.mean());
    }

    #[test]
    fn test_variance() {
        test_almost(1.0, 0.1, 100.0, 1e-13, |x| x.variance());
        test_case(1.0, 1.0, 1.0, |x| x.variance());
        test_case(10.0, 10.0, 0.1, |x| x.variance());
        test_case(10.0, 1.0, 10.0, |x| x.variance());
        test_case(10.0, f64::INFINITY, 0.0, |x| x.variance());
    }

    #[test]
    fn test_std_dev() {
        test_case(1.0, 0.1, 10.0, |x| x.std_dev());
        test_case(1.0, 1.0, 1.0, |x| x.std_dev());
        test_case(10.0, 10.0, 0.31622776601683794197697302588502426416723164097476643, |x| x.std_dev());
        test_case(10.0, 1.0, 3.1622776601683793319988935444327185337195551393252168, |x| x.std_dev());
        test_case(10.0, f64::INFINITY, 0.0, |x| x.std_dev());
    }

    #[test]
    fn test_entropy() {
        test_almost(1.0, 0.1, 3.3025850929940456285068402234265387271634735938763824, 1e-15, |x| x.entropy());
        test_almost(1.0, 1.0, 1.0, 1e-15, |x| x.entropy());
        test_almost(10.0, 10.0, 0.23346908548693395836262094490967812177376750477943892, 1e-13, |x| x.entropy());
        test_almost(10.0, 1.0, 2.5360541784809796423806123995940423293748689934081866, 1e-13, |x| x.entropy());
        test_case(10.0, f64::INFINITY, 0.0, |x| x.entropy());
    }

    #[test]
    fn test_skewness() {
        test_case(1.0, 0.1, 2.0, |x| x.skewness());
        test_case(1.0, 1.0, 2.0, |x| x.skewness());
        test_case(10.0, 10.0, 0.63245553203367586639977870888654370674391102786504337, |x| x.skewness());
        test_case(10.0, 1.0, 0.63245553203367586639977870888654370674391102786504337, |x| x.skewness());
        test_case(10.0, f64::INFINITY, 0.63245553203367586639977870888654370674391102786504337, |x| x.skewness());
    }

    #[test]
    fn test_mode() {
        test_case(1.0, 0.1, 0.0, |x| x.mode());
        test_case(1.0, 1.0, 0.0, |x| x.mode());
        test_case(10.0, 10.0, 0.9, |x| x.mode());
        test_case(10.0, 1.0, 9.0, |x| x.mode());
        test_case(10.0, f64::INFINITY, 10.0, |x| x.mode());
    }

    #[test]
    fn test_min_max() {
        test_case(1.0, 0.1, 0.0, |x| x.min());
        test_case(1.0, 1.0, 0.0, |x| x.min());
        test_case(10.0, 10.0, 0.0, |x| x.min());
        test_case(10.0, 1.0, 0.0, |x| x.min());
        test_case(10.0, f64::INFINITY, 0.0, |x| x.min());
        test_case(1.0, 0.1, f64::INFINITY, |x| x.max());
        test_case(1.0, 1.0, f64::INFINITY, |x| x.max());
        test_case(10.0, 10.0, f64::INFINITY, |x| x.max());
        test_case(10.0, 1.0, f64::INFINITY, |x| x.max());
        test_case(10.0, f64::INFINITY, f64::INFINITY, |x| x.max());
    }

    #[test]
    fn test_pdf() {
        test_case(1.0, 0.1, 0.090483741803595961836995913651194571475319347018875963, |x| x.pdf(1.0));
        test_case(1.0, 0.1, 0.036787944117144234201693506390001264039984687455876246, |x| x.pdf(10.0));
        test_case(1.0, 1.0, 0.36787944117144232159552377016146086744581113103176804, |x| x.pdf(1.0));
        test_case(1.0, 1.0, 0.000045399929762484851535591515560550610237918088866564953, |x| x.pdf(10.0));
        test_almost(10.0, 10.0, 1.2511003572113329898476497894772544708420990097708588, 1e-14, |x| x.pdf(1.0));
        test_almost(10.0, 10.0, 1.0251532120868705806216092933926141802686541811003037e-30, 1e-44, |x| x.pdf(10.0));
        test_almost(10.0, 1.0, 0.0000010137771196302974029859010421116095333052555418644397, 1e-20, |x| x.pdf(1.0));
        test_almost(10.0, 1.0, 0.12511003572113329898476497894772544708420990097708601, 1e-15, |x| x.pdf(10.0));
        test_case(10.0, f64::INFINITY, 0.0, |x| x.pdf(1.0));
        test_case(10.0, f64::INFINITY, f64::INFINITY, |x| x.pdf(10.0));
    }

    #[test]
    #[should_panic]
    fn test_non_positive_pdf() {
        get_value(1.0, 0.1, |x| x.pdf(0.0));
    }

    #[test]
    fn test_ln_pdf() {
        test_case(1.0, 0.1, -2.402585092994045634057955346552321429281631934330484, |x| x.ln_pdf(1.0));
        test_case(1.0, 0.1, -3.3025850929940456285068402234265387271634735938763824, |x| x.ln_pdf(10.0));
        test_case(1.0, 1.0, -1.0, |x| x.ln_pdf(1.0));
        test_case(1.0, 1.0, -10.0, |x| x.ln_pdf(10.0));
        test_almost(10.0, 10.0, 0.22402344985898722897219667227693591172986563062456522, 1e-15, |x| x.ln_pdf(1.0));
        test_case(10.0, 10.0, -69.052710713194601614865880235563786219860220971716511, |x| x.ln_pdf(10.0));
        test_almost(10.0, 1.0, -13.801827480081469611207717874566706164281149255663166, 1e-14, |x| x.ln_pdf(1.0));
        test_almost(10.0, 1.0,  -2.0785616431350584550457947824074282958712358580042068, 1e-14, |x| x.ln_pdf(10.0));
        test_case(10.0, f64::INFINITY, f64::NEG_INFINITY, |x| x.ln_pdf(1.0));
        test_case(10.0, f64::INFINITY, f64::INFINITY, |x| x.ln_pdf(10.0));
    }

    #[test]
    #[should_panic]
    fn test_non_positive_ln_pdf() {
        get_value(1.0, 0.1, |x| x.ln_pdf(0.0));
    }

    #[test]
    fn test_cdf() {
        test_almost(1.0, 0.1, 0.095162581964040431858607615783064404690935346242622848, 1e-16, |x| x.cdf(1.0));
        test_almost(1.0, 0.1, 0.63212055882855767840447622983853913255418886896823196, 1e-15, |x| x.cdf(10.0));
        test_almost(1.0, 1.0, 0.63212055882855767840447622983853913255418886896823196, 1e-15, |x| x.cdf(1.0));
        test_case(1.0, 1.0, 0.99995460007023751514846440848443944938976208191113396,|x| x.cdf(10.0));
        test_almost(10.0, 10.0, 0.54207028552814779168583514294066541824736464003242184, 1e-15, |x| x.cdf(1.0));
        test_case(10.0, 10.0, 0.99999999999999999999999999999988746526039157266114706, |x| x.cdf(10.0));
        test_almost(10.0, 1.0, 0.00000011142547833872067735305068724025236288094949815466035, 1e-21, |x| x.cdf(1.0));
        test_almost(10.0, 1.0, 0.54207028552814779168583514294066541824736464003242184, 1e-15, |x| x.cdf(10.0));
        test_case(10.0, f64::INFINITY, 0.0, |x| x.cdf(1.0));
        test_case(10.0, f64::INFINITY, 1.0, |x| x.cdf(10.0));
    }

    #[test]
    #[should_panic]
    fn test_non_positive_cdf() {
        get_value(1.0, 0.1, |x| x.cdf(0.0));
    }
}
