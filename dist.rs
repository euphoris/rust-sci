use core::libc::{c_double, c_uint};


extern mod gsl {
    // The Gaussian Distribution
    //fn gsl_ran_gaussian (const gsl_rng * r, sigma: c_double) -> c_double;
    fn gsl_ran_gaussian_pdf (x: c_double, sigma: c_double) -> c_double;
    //fn gsl_ran_gaussian_ziggurat (const gsl_rng * r, sigma: c_double) -> c_double;
    //fn gsl_ran_gaussian_ratio_method (const gsl_rng * r, sigma: c_double) -> c_double;
    //fn gsl_ran_ugaussian (const gsl_rng * r) -> c_double;
    fn gsl_ran_ugaussian_pdf (x: c_double) -> c_double;
    //fn gsl_ran_ugaussian_ratio_method (const gsl_rng * r) -> c_double;
    fn gsl_cdf_gaussian_P (x: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_gaussian_Q (x: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_gaussian_Pinv (P: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_gaussian_Qinv (Q: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_ugaussian_P (x: c_double) -> c_double;
    fn gsl_cdf_ugaussian_Q (x: c_double) -> c_double;
    fn gsl_cdf_ugaussian_Pinv (P: c_double) -> c_double;
    fn gsl_cdf_ugaussian_Qinv (Q: c_double) -> c_double;

    // The Gaussian Tail Distribution
    //fn gsl_ran_gaussian_tail (const gsl_rng * r, a: c_double, sigma: c_double) -> c_double;
    fn gsl_ran_gaussian_tail_pdf (x: c_double, a: c_double, sigma: c_double) -> c_double;
    //fn gsl_ran_ugaussian_tail (const gsl_rng * r, a: c_double) -> c_double;
    fn gsl_ran_ugaussian_tail_pdf (x: c_double, a: c_double) -> c_double;

    // The Bivariate Gaussian Distribution
    //fn gsl_ran_bivariate_gaussian (const gsl_rng * r, sigma_x: c_double, sigma_y: c_double, rho: c_double, double * x, double * y);
    fn gsl_ran_bivariate_gaussian_pdf (x: c_double, y: c_double, sigma_x: c_double, sigma_y: c_double, rho: c_double) -> c_double;

    // The Exponential Distribution
    //fn gsl_ran_exponential (const gsl_rng * r, mu: c_double) -> c_double;
    fn gsl_ran_exponential_pdf (x: c_double, mu: c_double) -> c_double;
    fn gsl_cdf_exponential_P (x: c_double, mu: c_double) -> c_double;
    fn gsl_cdf_exponential_Q (x: c_double, mu: c_double) -> c_double;
    fn gsl_cdf_exponential_Pinv (P: c_double, mu: c_double) -> c_double;
    fn gsl_cdf_exponential_Qinv (Q: c_double, mu: c_double) -> c_double;

    // The Laplace Distribution
    //fn gsl_ran_laplace (const gsl_rng * r, a: c_double) -> c_double;
    fn gsl_ran_laplace_pdf (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_laplace_P (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_laplace_Q (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_laplace_Pinv (P: c_double, a: c_double) -> c_double;
    fn gsl_cdf_laplace_Qinv (Q: c_double, a: c_double) -> c_double;

    // The Exponential Power Distribution
    //fn gsl_ran_exppow (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_exppow_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_exppow_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_exppow_Q (x: c_double, a: c_double, b: c_double) -> c_double;

    // The Cauchy Distribution
    //fn gsl_ran_cauchy (const gsl_rng * r, a: c_double) -> c_double;
    fn gsl_ran_cauchy_pdf (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_cauchy_P (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_cauchy_Q (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_cauchy_Pinv (P: c_double, a: c_double) -> c_double;
    fn gsl_cdf_cauchy_Qinv (Q: c_double, a: c_double) -> c_double;

    // The Rayleigh Distribution
    //fn gsl_ran_rayleigh (const gsl_rng * r, sigma: c_double) -> c_double;
    fn gsl_ran_rayleigh_pdf (x: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_rayleigh_P (x: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_rayleigh_Q (x: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_rayleigh_Pinv (P: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_rayleigh_Qinv (Q: c_double, sigma: c_double) -> c_double;

    // The Rayleigh Tail Distribution
    //fn gsl_ran_rayleigh_tail (const gsl_rng * r, a: c_double, sigma: c_double) -> c_double;
    fn gsl_ran_rayleigh_tail_pdf (x: c_double, a: c_double, sigma: c_double) -> c_double;

    // The Landau Distribution
    //fn gsl_ran_landau (const gsl_rng * r) -> c_double;
    fn gsl_ran_landau_pdf (x: c_double) -> c_double;

    // The Levy alpha-Stable Distributions
    //fn gsl_ran_levy (const gsl_rng * r, c: c_double, alpha: c_double) -> c_double;

    // The Levy skew alpha-Stable Distribution
    //fn gsl_ran_levy_skew (const gsl_rng * r, c: c_double, alpha: c_double, beta: c_double) -> c_double;

    // The Gamma Distribution
    //fn gsl_ran_gamma (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    //fn gsl_ran_gamma_knuth (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_gamma_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gamma_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gamma_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gamma_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gamma_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Flat (Uniform) Distribution
    //fn gsl_ran_flat (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_flat_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_flat_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_flat_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_flat_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_flat_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Lognormal Distribution
    //fn gsl_ran_lognormal (const gsl_rng * r, zeta: c_double, sigma: c_double) -> c_double;
    fn gsl_ran_lognormal_pdf (x: c_double, zeta: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_lognormal_P (x: c_double, zeta: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_lognormal_Q (x: c_double, zeta: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_lognormal_Pinv (P: c_double, zeta: c_double, sigma: c_double) -> c_double;
    fn gsl_cdf_lognormal_Qinv (Q: c_double, zeta: c_double, sigma: c_double) -> c_double;

    // The Chi-squared Distribution
    //fn gsl_ran_chisq (const gsl_rng * r, nu: c_double) -> c_double;
    fn gsl_ran_chisq_pdf (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_chisq_P (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_chisq_Q (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_chisq_Pinv (P: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_chisq_Qinv (Q: c_double, nu: c_double) -> c_double;

    // The F-distribution
    //fn gsl_ran_fdist (const gsl_rng * r, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_ran_fdist_pdf (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_P (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Q (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Pinv (P: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Qinv (Q: c_double, nu1: c_double, nu2: c_double) -> c_double;

    // The t-distribution
    //fn gsl_ran_tdist (const gsl_rng * r, nu: c_double) -> c_double;
    fn gsl_ran_tdist_pdf (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_tdist_P (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_tdist_Q (x: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_tdist_Pinv (P: c_double, nu: c_double) -> c_double;
    fn gsl_cdf_tdist_Qinv (Q: c_double, nu: c_double) -> c_double;

    // The Beta Distribution
    //fn gsl_ran_beta (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_beta_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_beta_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_beta_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_beta_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_beta_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Logistic Distribution
    //fn gsl_ran_logistic (const gsl_rng * r, a: c_double) -> c_double;
    fn gsl_ran_logistic_pdf (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_logistic_P (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_logistic_Q (x: c_double, a: c_double) -> c_double;
    fn gsl_cdf_logistic_Pinv (P: c_double, a: c_double) -> c_double;
    fn gsl_cdf_logistic_Qinv (Q: c_double, a: c_double) -> c_double;

    // The Pareto Distribution
    //fn gsl_ran_pareto (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_pareto_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_pareto_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_pareto_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_pareto_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_pareto_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // Spherical Vector Distributions
    //fn gsl_ran_dir_2d (const gsl_rng * r, double * x, double * y);
    //fn gsl_ran_dir_2d_trig_method (const gsl_rng * r, double * x, double * y);
    //fn gsl_ran_dir_3d (const gsl_rng * r, double * x, double * y, double * z);
    //fn gsl_ran_dir_nd (const gsl_rng * r, size_t n, double * x);

    // The Weibull Distribution
    //fn gsl_ran_weibull (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_weibull_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_weibull_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_weibull_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_weibull_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_weibull_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Type-1 Gumbel Distribution
    //fn gsl_ran_gumbel1 (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_gumbel1_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel1_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel1_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel1_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel1_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Type-2 Gumbel Distribution
    //fn gsl_ran_gumbel2 (const gsl_rng * r, a: c_double, b: c_double) -> c_double;
    fn gsl_ran_gumbel2_pdf (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel2_P (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel2_Q (x: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel2_Pinv (P: c_double, a: c_double, b: c_double) -> c_double;
    fn gsl_cdf_gumbel2_Qinv (Q: c_double, a: c_double, b: c_double) -> c_double;

    // The Dirichlet Distribution
    //fn gsl_ran_dirichlet (const gsl_rng * r, size_t K, const alpha: c_double[], theta: c_double[]);
    //fn gsl_ran_dirichlet_pdf (size_t K, const alpha: c_double[], const theta: c_double[]) -> c_double;
    //fn gsl_ran_dirichlet_lnpdf (size_t K, const alpha: c_double[], const theta: c_double[]) -> c_double;

    // General Discrete Distributions
    //gsl_ran_discrete_t * gsl_ran_discrete_preproc (size_t K, const double * P)
    //size_t gsl_ran_discrete (const gsl_rng * r, const gsl_ran_discrete_t * g)
    //fn gsl_ran_discrete_pdf (size_t k, const gsl_ran_discrete_t * g) -> c_double;
    //fn gsl_ran_discrete_free (gsl_ran_discrete_t * g);

    // The Poisson Distribution
    //fn gsl_ran_poisson (const gsl_rng * r, mu: c_double) -> c_uint;
    fn gsl_ran_poisson_pdf (k: c_uint, mu: c_double) -> c_double;
    fn gsl_cdf_poisson_P (k: c_uint, mu: c_double) -> c_double;
    fn gsl_cdf_poisson_Q (k: c_uint, mu: c_double) -> c_double;

    // The Bernoulli Distribution
    //fn gsl_ran_bernoulli (const gsl_rng * r, p: c_double) -> c_uint;
    fn gsl_ran_bernoulli_pdf (k: c_uint, p: c_double) -> c_double;

    // The Binomial Distribution
    //fn gsl_ran_binomial (const gsl_rng * r, p: c_double, n: c_uint) -> c_uint;
    fn gsl_ran_binomial_pdf (k: c_uint, p: c_double, n: c_uint) -> c_double;
    fn gsl_cdf_binomial_P (k: c_uint, p: c_double, n: c_uint) -> c_double;
    fn gsl_cdf_binomial_Q (k: c_uint, p: c_double, n: c_uint) -> c_double;

    // The Multinomial Distribution
    //fn gsl_ran_multinomial (const gsl_rng * r, size_t K, N: c_uint, const p: c_double[], n: c_uint[]);
    //fn gsl_ran_multinomial_pdf (size_t K, const p: c_double[], const n: c_uint[]) -> c_double;
    //fn gsl_ran_multinomial_lnpdf (size_t K, const p: c_double[], const n: c_uint[]) -> c_double;

    // The Negative Binomial Distribution
    //fn gsl_ran_negative_binomial (const gsl_rng * r, p: c_double, n: c_double) -> c_uint;
    fn gsl_ran_negative_binomial_pdf (k: c_uint, p: c_double, n: c_double) -> c_double;
    fn gsl_cdf_negative_binomial_P (k: c_uint, p: c_double, n: c_double) -> c_double;
    fn gsl_cdf_negative_binomial_Q (k: c_uint, p: c_double, n: c_double) -> c_double;

    // The Pascal Distribution
    //fn gsl_ran_pascal (const gsl_rng * r, p: c_double, n: c_uint) -> c_uint;
    fn gsl_ran_pascal_pdf (k: c_uint, p: c_double, n: c_uint) -> c_double;
    fn gsl_cdf_pascal_P (k: c_uint, p: c_double, n: c_uint) -> c_double;
    fn gsl_cdf_pascal_Q (k: c_uint, p: c_double, n: c_uint) -> c_double;

    // The Geometric Distribution
    //fn gsl_ran_geometric (const gsl_rng * r, p: c_double) -> c_uint;
    fn gsl_ran_geometric_pdf (k: c_uint, p: c_double) -> c_double;
    fn gsl_cdf_geometric_P (k: c_uint, p: c_double) -> c_double;
    fn gsl_cdf_geometric_Q (k: c_uint, p: c_double) -> c_double;

    // The Hypergeometric Distribution
    //fn gsl_ran_hypergeometric (const gsl_rng * r, n1: c_uint, n2: c_uint, t: c_uint) -> c_uint;
    fn gsl_ran_hypergeometric_pdf (k: c_uint, n1: c_uint, n2: c_uint, t: c_uint) -> c_double;
    fn gsl_cdf_hypergeometric_P (k: c_uint, n1: c_uint, n2: c_uint, t: c_uint) -> c_double;
    fn gsl_cdf_hypergeometric_Q (k: c_uint, n1: c_uint, n2: c_uint, t: c_uint) -> c_double;

    // The Logarithmic Distribution
    //fn gsl_ran_logarithmic (const gsl_rng * r, p: c_double) -> c_uint;
    fn gsl_ran_logarithmic_pdf (k: c_uint, p: c_double) -> c_double;
    //fn gsl_ran_shuffle (const gsl_rng * r, void * base, size_t n, size_t size);
    //fn gsl_ran_choose (const gsl_rng * r, void * dest, size_t k, void * src, size_t n, size_t size) -> c_int;
    //fn gsl_ran_sample (const gsl_rng * r, void * dest, size_t k, void * src, size_t n, size_t size);
}

pub struct Fdist {
    df1: f64,
    df2: f64  
}


pub impl Fdist {
    fn d(&self, x: f64) -> f64 {
        unsafe {
            gsl::gsl_ran_fdist_pdf(x, self.df1, self.df2)
        }
    }

    fn p(&self, x: f64) -> f64 {
        unsafe {
            gsl::gsl_cdf_fdist_P(x, self.df1, self.df2)
        }
    }

    fn q(&self, x: f64) -> f64 {
        unsafe {
            gsl::gsl_cdf_fdist_Q(x, self.df1, self.df2)
        }
    }

    fn pinv(&self, p: f64) -> f64 {
        unsafe {
            gsl::gsl_cdf_fdist_Pinv(p, self.df1, self.df2)
        }
    }

    fn qinv(&self, q: f64) -> f64 {
        unsafe {
            gsl::gsl_cdf_fdist_Qinv(q, self.df1, self.df2)
        }
    }
}

