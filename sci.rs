use core::libc::{c_double, c_int, size_t};


struct gsl_block {
    size: size_t,
    data: *c_double
}


struct gsl_vector {
    size: size_t,
    stride: size_t,
    data: *c_double,
    block: *gsl_block,
    owner: c_int
}


extern mod gsl {
    // vector
    fn gsl_vector_alloc(n: size_t) -> *gsl_vector;
    fn gsl_vector_calloc(n: size_t) -> *gsl_vector;
    fn gsl_vector_free (v: *gsl_vector);

    fn gsl_vector_get (v: *gsl_vector, i: size_t) -> c_double;
    fn gsl_vector_set (v: *gsl_vector, i: size_t, x: c_double);

    fn gsl_vector_set_all (v: *gsl_vector, x: c_double);
    fn gsl_vector_set_zero (v: *gsl_vector);
    fn gsl_vector_set_basis (v: *gsl_vector, i: size_t) -> c_int;

    fn gsl_vector_add (a: *gsl_vector, b: *gsl_vector) -> c_int;
    fn gsl_vector_sub (a: *gsl_vector, b: *gsl_vector) -> c_int;
    fn gsl_vector_mul (a: *gsl_vector, b: *gsl_vector) -> c_int;
    fn gsl_vector_div (a: *gsl_vector, b: *gsl_vector) -> c_int;
    fn gsl_vector_scale (a: *gsl_vector, x: c_double) -> c_int;
    fn gsl_vector_add_constant (a: *gsl_vector, x: c_double) -> c_int;
}


pub struct vector {
    size: size_t,
    ptr: *gsl_vector
}


pub impl vector {
    fn zeros(n: size_t) -> vector {
        unsafe { vector{ size: n, ptr: gsl::gsl_vector_calloc(n) } }
    }

    fn as_vector(v: &[f64]) -> vector {
        unsafe {
            let mut new = vector::zeros(v.len() as size_t);

            let mut i = 0;
            while i < new.size {
                gsl::gsl_vector_set(new.ptr, i, v[i]);
                i+=1;
            }

            new
        }
    }

    fn get(&self, i: size_t) -> f64 {
        unsafe { gsl::gsl_vector_get(self.ptr, i) }
    }

    fn set(&self, i: size_t, x: f64){
        unsafe { gsl::gsl_vector_set(self.ptr, i, x) }
    }

    fn set_all(&self, x: f64){
        unsafe { gsl::gsl_vector_set_all(self.ptr, x) }
    }

    fn set_zero(&self){
        unsafe { gsl::gsl_vector_set_zero(self.ptr) }
    }

    fn set_basis(&self, i: size_t) -> i32 {
        unsafe { gsl::gsl_vector_set_basis (self.ptr, i) }
    }

    fn scale(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_vector_scale(self.ptr, x) }
    }

    fn add_constant(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_vector_add_constant(self.ptr, x) }
    }
}


impl Drop for vector {
    fn finalize(&self) {
        unsafe { gsl::gsl_vector_free(self.ptr); }
    }
}


impl Add<vector, vector> for vector {
    fn add(&self, rhs: &vector) -> vector{
        unsafe {
            let size = self.size;
            let new = vector::zeros(size);
            gsl::gsl_vector_add(new.ptr, self.ptr);
            gsl::gsl_vector_add(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Sub<vector, vector> for vector {
    fn sub(&self, rhs: &vector) -> vector{
        unsafe {
            let size = self.size;
            let new = vector::zeros(size);
            gsl::gsl_vector_add(new.ptr, self.ptr);
            gsl::gsl_vector_sub(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Mul<vector, vector> for vector {
    fn mul(&self, rhs: &vector) -> vector{
        unsafe {
            let size = self.size;
            let new = vector::zeros(size);
            gsl::gsl_vector_add(new.ptr, self.ptr);
            gsl::gsl_vector_mul(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Div<vector, vector> for vector {
    fn div(&self, rhs: &vector) -> vector{
        unsafe {
            let size = self.size;
            let new = vector::zeros(size);
            gsl::gsl_vector_add(new.ptr, self.ptr);
            gsl::gsl_vector_div(new.ptr, rhs.ptr);
            new
        }
    }
}
