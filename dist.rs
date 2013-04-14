use core::libc::c_double;


extern mod gsl {
    fn gsl_ran_fdist_pdf (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_P (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Q (x: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Pinv (P: c_double, nu1: c_double, nu2: c_double) -> c_double;
    fn gsl_cdf_fdist_Qinv (Q: c_double, nu1: c_double, nu2: c_double) -> c_double;
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

