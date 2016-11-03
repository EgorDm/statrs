//! Provides special functions related to the [beta](https://en.wikipedia.org/wiki/Beta_function)
//! function

use std::f64;
use error::StatsError;
use function::gamma;
use prec;
use Float;

/// Computes the natural logarithm
/// of the beta function
/// where `a` is the first beta parameter
/// and `b` is the second beta parameter
/// and `a > 0`, `b > 0`.
///
/// # Panics
///
/// if `a <= 0.0` or `b <= 0.0`
pub fn ln_beta<T>(a: T, b: T) -> T
    where T: Float
{
    assert!(a > T::zero(),
            format!("{}", StatsError::ArgMustBePositive("a")));
    assert!(b > T::zero(),
            format!("{}", StatsError::ArgMustBePositive("b")));
    gamma::ln_gamma(a) + gamma::ln_gamma(b) - gamma::ln_gamma(a + b)
}

/// Computes the beta function
/// where `a` is the first beta parameter
/// and `b` is the second beta parameter.
///
///
/// # Panics
///
/// if `a <= 0.0` or `b <= 0.0`
pub fn beta<T>(a: T, b: T) -> T
    where T: Float
{
    ln_beta(a, b).exp()
}

/// Computes the lower incomplete (unregularized) beta function
/// `B(a,b,x) = int(t^(a-1)*(1-t)^(b-1),t=0..x)` for `a > 0, b > 0, 1 >= x >= 0`
/// where `a` is the first beta parameter, `b` is the second beta parameter, and
/// `x` is the upper limit of the integral
///
/// # Panics
///
/// If `a < 0.0`, `b < 0.0`, `x < 0.0`, or `x > 1.0`
pub fn beta_inc<T>(a: T, b: T, x: T) -> T
    where T: Float
{
    beta_reg(a, b, x) * beta(a, b)
}

/// Computes the regularized lower incomplete beta function
/// `I_x(a,b) = 1/Beta(a,b) * int(t^(a-1)*(1-t)^(b-1), t=0..x)`
/// `a > 0`, `b > 0`, `1 >= x >= 0` where `a` is the first beta parameter,
/// `b` is the second beta parameter, and `x` is the upper limit of the
/// integral.
///
/// # Panics
///
/// if `a < 0.0`, `b < 0.0`, `x < 0.0`, or `x > 1.0`
pub fn beta_reg<T>(a: T, b: T, x: T) -> T
    where T: Float
{
    assert!(a >= T::zero(),
            format!("{}", StatsError::ArgNotNegative("a")));
    assert!(b >= T::zero(),
            format!("{}", StatsError::ArgNotNegative("b")));
    assert!(x >= T::zero() && x <= T::one(),
            format!("{}", StatsError::ArgIntervalIncl("x", 0.0, 1.0)));

    let bt = if x.is_zero() || x == T::one() {
        T::zero()
    } else {
        (gamma::ln_gamma(a + b) - gamma::ln_gamma(a) - gamma::ln_gamma(b) + a * x.ln() +
         b * (T::one() - x).ln())
            .exp()
    };
    let symm_transform = x >= (a + T::one()) / (a + b + T::from(2.0).unwrap());
    let eps = T::precision();
    let fpmin = T::min_positive_value() / eps;

    let mut a = a;
    let mut b = b;
    let mut x = x;
    if symm_transform {
        let swap = a;
        x = T::one() - x;
        a = b;
        b = swap;
    }

    let qab = a + b;
    let qap = a + T::one();
    let qam = a - T::one();
    let mut c = T::one();
    let mut d = T::one() - qab * x / qap;

    if d.abs() < fpmin {
        d = fpmin;
    }
    d = T::one() / d;
    let mut h = d;

    for m in 1..141 {
        let m = T::from(m).unwrap();
        let m2 = m * T::from(2.0).unwrap();
        let mut aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = T::one() + aa * d;

        if d.abs() < fpmin {
            d = fpmin;
        }

        c = T::one() + aa / c;
        if c.abs() < fpmin {
            c = fpmin;
        }

        d = T::one() / d;
        h = h * d * c;
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = T::one() + aa * d;

        if d.abs() < fpmin {
            d = fpmin;
        }

        c = T::one() + aa / c;

        if c.abs() < fpmin {
            c = fpmin;
        }

        d = T::one() / d;
        let del = d * c;
        h = h * del;

        if (del - T::one()).abs() <= eps {
            return if symm_transform {
                T::one() - bt * h / a
            } else {
                bt * h / a
            };
        }
    }

    if symm_transform {
        T::one() - bt * h / a
    } else {
        bt * h / a
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    #[test]
    fn test_ln_beta() {
        assert_almost_eq!(super::ln_beta(0.5, 0.5), 1.144729885849400174144, 1e-15);
        assert_almost_eq!(super::ln_beta(1.0, 0.5), 0.6931471805599453094172, 1e-14);
        assert_almost_eq!(super::ln_beta(2.5, 0.5), 0.163900632837673937284, 1e-15);
        assert_almost_eq!(super::ln_beta(0.5, 1.0), 0.6931471805599453094172, 1e-14);
        assert_almost_eq!(super::ln_beta(1.0, 1.0), 0.0, 1e-15);
        assert_almost_eq!(super::ln_beta(2.5, 1.0), -0.9162907318741550651835, 1e-14);
        assert_almost_eq!(super::ln_beta(0.5, 2.5), 0.163900632837673937284, 1e-15);
        assert_almost_eq!(super::ln_beta(1.0, 2.5), -0.9162907318741550651835, 1e-14);
        assert_almost_eq!(super::ln_beta(2.5, 2.5), -2.608688089402107300388, 1e-14);
    }

    #[test]
    fn test_beta() {
        assert_almost_eq!(super::beta(0.5, 0.5), 3.141592653589793238463, 1e-15);
        assert_almost_eq!(super::beta(1.0, 0.5), 2.0, 1e-14);
        assert_almost_eq!(super::beta(2.5, 0.5), 1.17809724509617246442, 1e-15);
        assert_almost_eq!(super::beta(0.5, 1.0), 2.0, 1e-14);
        assert_almost_eq!(super::beta(1.0, 1.0), 1.0, 1e-15);
        assert_almost_eq!(super::beta(2.5, 1.0), 0.4, 1e-14);
        assert_almost_eq!(super::beta(0.5, 2.5), 1.17809724509617246442, 1e-15);
        assert_almost_eq!(super::beta(1.0, 2.5), 0.4, 1e-14);
        assert_almost_eq!(super::beta(2.5, 2.5), 0.073631077818510779026, 1e-15);
    }

    #[test]
    fn test_beta_inc() {
        assert_almost_eq!(super::beta_inc(0.5, 0.5, 0.5), 1.570796326794896619231, 1e-14);
        assert_almost_eq!(super::beta_inc(0.5, 0.5, 1.0), 3.141592653589793238463, 1e-15);
        assert_almost_eq!(super::beta_inc(1.0, 0.5, 0.5), 0.5857864376269049511983, 1e-15);
        assert_almost_eq!(super::beta_inc(1.0, 0.5, 1.0), 2.0, 1e-14);
        assert_almost_eq!(super::beta_inc(2.5, 0.5, 0.5), 0.0890486225480862322117, 1e-16);
        assert_almost_eq!(super::beta_inc(2.5, 0.5, 1.0), 1.17809724509617246442, 1e-15);
        assert_almost_eq!(super::beta_inc(0.5, 1.0, 0.5), 1.414213562373095048802, 1e-14);
        assert_almost_eq!(super::beta_inc(0.5, 1.0, 1.0), 2.0, 1e-14);
        assert_almost_eq!(super::beta_inc(1.0, 1.0, 0.5), 0.5, 1e-15);
        assert_almost_eq!(super::beta_inc(1.0, 1.0, 1.0), 1.0, 1e-15);
        assert_eq!(super::beta_inc(2.5, 1.0, 0.5), 0.0707106781186547524401);
        assert_almost_eq!(super::beta_inc(2.5, 1.0, 1.0), 0.4, 1e-14);
        assert_almost_eq!(super::beta_inc(0.5, 2.5, 0.5), 1.08904862254808623221, 1e-15);
        assert_almost_eq!(super::beta_inc(0.5, 2.5, 1.0), 1.17809724509617246442, 1e-15);
        assert_almost_eq!(super::beta_inc(1.0, 2.5, 0.5), 0.32928932188134524756, 1e-14);
        assert_almost_eq!(super::beta_inc(1.0, 2.5, 1.0), 0.4, 1e-14);
        assert_almost_eq!(super::beta_inc(2.5, 2.5, 0.5), 0.03681553890925538951323, 1e-15);
        assert_almost_eq!(super::beta_inc(2.5, 2.5, 1.0), 0.073631077818510779026, 1e-15);
    }

    #[test]
    fn test_beta_reg() {
        assert_almost_eq!(super::beta_reg(0.5, 0.5, 0.5), 0.5, 1e-15);
        assert_eq!(super::beta_reg(0.5, 0.5, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(1.0, 0.5, 0.5), 0.292893218813452475599, 1e-15);
        assert_eq!(super::beta_reg(1.0, 0.5, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(2.5, 0.5, 0.5), 0.07558681842161243795, 1e-16);
        assert_eq!(super::beta_reg(2.5, 0.5, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(0.5, 1.0, 0.5), 0.7071067811865475244, 1e-15);
        assert_eq!(super::beta_reg(0.5, 1.0, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(1.0, 1.0, 0.5), 0.5, 1e-15);
        assert_eq!(super::beta_reg(1.0, 1.0, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(2.5, 1.0, 0.5), 0.1767766952966368811, 1e-15);
        assert_eq!(super::beta_reg(2.5, 1.0, 1.0), 1.0);
        assert_eq!(super::beta_reg(0.5, 2.5, 0.5), 0.92441318157838756205);
        assert_eq!(super::beta_reg(0.5, 2.5, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(1.0, 2.5, 0.5), 0.8232233047033631189, 1e-15);
        assert_eq!(super::beta_reg(1.0, 2.5, 1.0), 1.0);
        assert_almost_eq!(super::beta_reg(2.5, 2.5, 0.5), 0.5, 1e-15);
        assert_eq!(super::beta_reg(2.5, 2.5, 1.0), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_ln_beta_neg() {
        super::ln_beta(-1.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_beta_neg() {
        super::beta(-1.0, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_beta_inc_neg() {
        super::beta_inc(0.5, 0.5, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_beta_inc_over_one() {
        super::beta_inc(0.5, 0.5, 2.5);
    }

    #[test]
    #[should_panic]
    fn test_beta_reg_neg() {
        super::beta_reg(0.5, 0.5, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_beta_reg_over_one() {
        super::beta_reg(0.5, 0.5, 2.5);
    }
}
