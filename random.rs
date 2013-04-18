use core::libc::{c_ulong, c_char, c_double, c_uint, c_void};

pub struct gsl_rng_type {
    name: *c_char,
    max: c_ulong,
    min: c_ulong,
    size: c_uint,
    set: extern fn(state:*c_void, seed:c_ulong),
    get: extern fn(state:*c_void) -> c_ulong,
    get_double: extern fn(state:*c_void) -> c_double
}

pub struct gsl_rng {
    typ: *gsl_rng_type,
    state: *c_void
}

extern mod gsl {
    fn gsl_rng_alloc(T: *gsl_rng_type) -> *gsl_rng;
    fn gsl_rng_free(r: *gsl_rng);
}

extern mod gslbind {
    fn rng_borosh13()-> *gsl_rng_type;
    fn rng_coveyou()-> *gsl_rng_type;
    fn rng_cmrg()-> *gsl_rng_type;
    fn rng_fishman18()-> *gsl_rng_type;
    fn rng_fishman20()-> *gsl_rng_type;
    fn rng_fishman2x()-> *gsl_rng_type;
    fn rng_gfsr4()-> *gsl_rng_type;
    fn rng_knuthran()-> *gsl_rng_type;
    fn rng_knuthran2()-> *gsl_rng_type;
    fn rng_knuthran2002()-> *gsl_rng_type;
    fn rng_lecuyer21()-> *gsl_rng_type;
    fn rng_minstd()-> *gsl_rng_type;
    fn rng_mrg()-> *gsl_rng_type;
    fn rng_mt19937()-> *gsl_rng_type;
    fn rng_mt19937_1999()-> *gsl_rng_type;
    fn rng_mt19937_1998()-> *gsl_rng_type;
    fn rng_r250()-> *gsl_rng_type;
    fn rng_ran0()-> *gsl_rng_type;
    fn rng_ran1()-> *gsl_rng_type;
    fn rng_ran2()-> *gsl_rng_type;
    fn rng_ran3()-> *gsl_rng_type;
    fn rng_rand()-> *gsl_rng_type;
    fn rng_rand48()-> *gsl_rng_type;
    fn rng_random128_bsd()-> *gsl_rng_type;
    fn rng_random128_glibc2()-> *gsl_rng_type;
    fn rng_random128_libc5()-> *gsl_rng_type;
    fn rng_random256_bsd()-> *gsl_rng_type;
    fn rng_random256_glibc2()-> *gsl_rng_type;
    fn rng_random256_libc5()-> *gsl_rng_type;
    fn rng_random32_bsd()-> *gsl_rng_type;
    fn rng_random32_glibc2()-> *gsl_rng_type;
    fn rng_random32_libc5()-> *gsl_rng_type;
    fn rng_random64_bsd()-> *gsl_rng_type;
    fn rng_random64_glibc2()-> *gsl_rng_type;
    fn rng_random64_libc5()-> *gsl_rng_type;
    fn rng_random8_bsd()-> *gsl_rng_type;
    fn rng_random8_glibc2()-> *gsl_rng_type;
    fn rng_random8_libc5()-> *gsl_rng_type;
    fn rng_random_bsd()-> *gsl_rng_type;
    fn rng_random_glibc2()-> *gsl_rng_type;
    fn rng_random_libc5()-> *gsl_rng_type;
    fn rng_randu()-> *gsl_rng_type;
    fn rng_ranf()-> *gsl_rng_type;
    fn rng_ranlux()-> *gsl_rng_type;
    fn rng_ranlux389()-> *gsl_rng_type;
    fn rng_ranlxd1()-> *gsl_rng_type;
    fn rng_ranlxd2()-> *gsl_rng_type;
    fn rng_ranlxs0()-> *gsl_rng_type;
    fn rng_ranlxs1()-> *gsl_rng_type;
    fn rng_ranlxs2()-> *gsl_rng_type;
    fn rng_ranmar()-> *gsl_rng_type;
    fn rng_slatec()-> *gsl_rng_type;
    fn rng_taus()-> *gsl_rng_type;
    fn rng_taus2()-> *gsl_rng_type;
    fn rng_taus113()-> *gsl_rng_type;
    fn rng_transputer()-> *gsl_rng_type;
    fn rng_tt800()-> *gsl_rng_type;
    fn rng_uni()-> *gsl_rng_type;
    fn rng_uni32()-> *gsl_rng_type;
    fn rng_vax()-> *gsl_rng_type;
    fn rng_waterman14()-> *gsl_rng_type;
    fn rng_zuf()-> *gsl_rng_type;
    fn rng_default()-> *gsl_rng_type;
}

pub unsafe fn alloc(T: *gsl_rng_type) -> *gsl_rng {
    gsl::gsl_rng_alloc(T)
}
pub unsafe fn free(r: *gsl_rng) {
    gsl::gsl_rng_free(r)
}
pub unsafe fn borosh13() -> *gsl_rng_type {
    gslbind::rng_borosh13()
}
