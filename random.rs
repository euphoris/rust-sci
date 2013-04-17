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
    fn gsl_rng_borosh13() -> *gsl_rng_type;
    fn gsl_rng_alloc(T: *gsl_rng_type) -> *gsl_rng;
    fn gsl_rng_free(r: *gsl_rng);
}

pub unsafe fn alloc(T: *gsl_rng_type) -> *gsl_rng {
    gsl::gsl_rng_alloc(T)
}
pub unsafe fn free(r: *gsl_rng) {
    gsl::gsl_rng_free(r)
}
pub unsafe fn borosh13() -> *gsl_rng_type {
    gsl::gsl_rng_borosh13()
}
