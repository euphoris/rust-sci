use core::libc::{c_double, c_int, c_void, size_t};


type gsl_vector = c_void;
type gsl_matrix = c_void;

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

    fn gsl_vector_memcpy (dest: *gsl_vector, src: *gsl_vector) -> c_int;
    fn gsl_vector_swap (w: *gsl_vector, v: *gsl_vector) -> c_int;

    fn gsl_vector_isnull (m: *gsl_vector) -> c_int;
    fn gsl_vector_ispos (m: *gsl_vector) -> c_int;
    fn gsl_vector_isneg (m: *gsl_vector) -> c_int;
    fn gsl_vector_isnonneg (m: *gsl_vector) -> c_int;
    fn gsl_vector_equal (a: *gsl_vector, b: *gsl_vector) -> c_int;

    fn gsl_vector_max (v: *gsl_vector) -> c_double;
    fn gsl_vector_min (v: *gsl_vector) -> c_double;
    fn gsl_vector_minmax (v: *gsl_vector, min_out: *mut c_double, max_out: *mut c_double);
    fn gsl_vector_max_index (v: *gsl_vector) -> size_t;
    fn gsl_vector_min_index (v: *gsl_vector) -> size_t;
    fn gsl_vector_minmax_index (v: *gsl_vector, imin: *mut size_t, imax: *mut size_t);

    // matrix
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *gsl_matrix;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *gsl_matrix;
    fn gsl_matrix_free (m: *gsl_matrix);

    fn gsl_matrix_get (m: *gsl_matrix, i: size_t, j: size_t) -> c_double;
    fn gsl_matrix_set (m: *gsl_matrix, i: size_t, j: size_t, x: c_double);

    fn gsl_matrix_set_all (m: *gsl_matrix, x: c_double);
    fn gsl_matrix_set_zero (m: *gsl_matrix);

    fn gsl_matrix_add (a: *gsl_matrix, b: *gsl_matrix) -> c_int;
    fn gsl_matrix_sub (a: *gsl_matrix, b: *gsl_matrix) -> c_int;
    fn gsl_matrix_scale (a: *gsl_matrix, x: c_double) -> c_int;
    fn gsl_matrix_add_constant (a: *gsl_matrix, x: c_double) -> c_int;

    fn gsl_matrix_memcpy (dest: *gsl_matrix, src: *gsl_matrix) -> c_int;
    fn gsl_matrix_swap (w: *gsl_matrix, v: *gsl_matrix) -> c_int;

    fn gsl_matrix_isnull (m: *gsl_matrix) -> c_int;
    fn gsl_matrix_ispos (m: *gsl_matrix) -> c_int;
    fn gsl_matrix_isneg (m: *gsl_matrix) -> c_int;
    fn gsl_matrix_isnonneg (m: *gsl_matrix) -> c_int;
    fn gsl_matrix_equal (a: *gsl_matrix, b: *gsl_matrix) -> c_int;

    fn gsl_matrix_max (m: *gsl_matrix) -> c_double;
    fn gsl_matrix_min (m: *gsl_matrix) -> c_double;
    fn gsl_matrix_minmax (m: *gsl_matrix, min_out: *mut c_double, max_out: *mut c_double);
    fn gsl_matrix_max_index (m: *gsl_matrix, imax: *mut size_t, jmax: *mut size_t);
    fn gsl_matrix_min_index (m: *gsl_matrix, imin: *mut size_t, jmin: *mut size_t);
    fn gsl_matrix_minmax_index (m: *gsl_matrix, imin: *mut size_t, jmin: *mut size_t, imax: *mut size_t, jmax: *mut size_t);
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

    fn set(&mut self, i: size_t, x: f64){
        unsafe { gsl::gsl_vector_set(self.ptr, i, x) }
    }

    fn set_all(&mut self, x: f64){
        unsafe { gsl::gsl_vector_set_all(self.ptr, x) }
    }

    fn set_zero(&mut self){
        unsafe { gsl::gsl_vector_set_zero(self.ptr) }
    }

    fn set_basis(&mut self, i: size_t) -> i32 {
        unsafe { gsl::gsl_vector_set_basis (self.ptr, i) }
    }

    fn scale(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_vector_scale(self.ptr, x) }
    }

    fn add_constant(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_vector_add_constant(self.ptr, x) }
    }

    fn isnull(&self) -> bool {
        unsafe { gsl::gsl_vector_isnull(self.ptr) == 1 }
    }

    fn ispos(&self) -> bool {
        unsafe { gsl::gsl_vector_ispos(self.ptr) == 1 }
    }

    fn isneg(&self) -> bool {
        unsafe { gsl::gsl_vector_isneg(self.ptr) == 1 }
    }

    fn isnonneg(&self) -> bool {
        unsafe { gsl::gsl_vector_isnonneg(self.ptr) == 1 }
    }

    fn max(&self) -> f64 {
        unsafe { gsl::gsl_vector_max(self.ptr) }
    }

    fn min(&self) -> f64 {
        unsafe { gsl::gsl_vector_min(self.ptr) }
    }

    fn minmax(&self) -> (f64, f64) {
        unsafe {
            let mut min = ~0.0;
            let mut max = ~0.0;
            gsl::gsl_vector_minmax(self.ptr,
                                   ptr::to_mut_unsafe_ptr(min),
                                   ptr::to_mut_unsafe_ptr(max));
            (*min, *max)
        }
    }

    fn maxindex(&self) -> u64 {
        unsafe {
            gsl::gsl_vector_max_index(self.ptr)
        }
    }

    fn minindex(&self) -> u64 {
        unsafe {
            gsl::gsl_vector_min_index(self.ptr)
        }
    }

    fn minmax_index(&self) -> (u64, u64) {
        unsafe {
            let mut imin = ~0;
            let mut imax = ~0;
            gsl::gsl_vector_minmax_index(self.ptr,
                                         ptr::to_mut_unsafe_ptr(imin),
                                         ptr::to_mut_unsafe_ptr(imax));
            (*imin, *imax)
        }
    }
}


impl Drop for vector {
    fn finalize(&self) {
        unsafe { gsl::gsl_vector_free(self.ptr); }
    }
}


impl Clone for vector {
    fn clone(&self) -> vector {
        unsafe {
            let v = vector::zeros(self.size);
            gsl::gsl_vector_memcpy(v.ptr, self.ptr);
            v
        }
    }
}


impl Eq for vector {
    fn eq(&self, other: &vector) -> bool {
        unsafe {
            gsl::gsl_vector_equal(self.ptr, other.ptr) == 1 
        }
    }

    fn ne(&self, other: &vector) -> bool {
        unsafe {
            gsl::gsl_vector_equal(self.ptr, other.ptr) == 0
        }
    }
}


impl Add<vector, vector> for vector {
    fn add(&self, rhs: &vector) -> vector{
        unsafe {
            let new = self.clone();
            gsl::gsl_vector_add(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Sub<vector, vector> for vector {
    fn sub(&self, rhs: &vector) -> vector{
        unsafe {
            let new = self.clone();
            gsl::gsl_vector_sub(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Mul<vector, vector> for vector {
    fn mul(&self, rhs: &vector) -> vector{
        unsafe {
            let new = self.clone();
            gsl::gsl_vector_mul(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Div<vector, vector> for vector {
    fn div(&self, rhs: &vector) -> vector{
        unsafe {
            let new = self.clone();
            gsl::gsl_vector_div(new.ptr, rhs.ptr);
            new
        }
    }
}

pub struct matrix {
    size: (size_t, size_t),
    ptr: *c_void
}


pub impl matrix {
    fn zeros(n1: size_t, n2: size_t) -> matrix {
        unsafe { matrix{ size: (n1, n2), ptr: gsl::gsl_matrix_calloc(n1, n2) } }
    }

    fn get(&self, i: size_t, j: size_t) -> f64 {
        unsafe { gsl::gsl_matrix_get(self.ptr, i, j) }
    }

    fn set(&mut self, i: size_t, j: size_t, x: f64){
        unsafe { gsl::gsl_matrix_set(self.ptr, i, j, x) }
    }

    fn set_all(&mut self, x: f64){
        unsafe { gsl::gsl_matrix_set_all(self.ptr, x) }
    }

    fn set_zero(&mut self){
        unsafe { gsl::gsl_matrix_set_zero(self.ptr) }
    }

    fn scale(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_matrix_scale(self.ptr, x) }
    }

    fn add_constant(&self, x:f64) -> i32 {
        unsafe { gsl::gsl_matrix_add_constant(self.ptr, x) }
    }
    
    fn isnull(&self) -> bool {
        unsafe { gsl::gsl_matrix_isnull(self.ptr) == 1 }
    }

    fn ispos(&self) -> bool {
        unsafe { gsl::gsl_matrix_ispos(self.ptr) == 1 }
    }

    fn isneg(&self) -> bool {
        unsafe { gsl::gsl_matrix_isneg(self.ptr) == 1 }
    }

    fn isnonneg(&self) -> bool {
        unsafe { gsl::gsl_matrix_isnonneg(self.ptr) == 1 }
    }

    fn max(&self) -> f64 {
        unsafe { gsl::gsl_matrix_max(self.ptr) }
    }

    fn min(&self) -> f64 {
        unsafe { gsl::gsl_matrix_min(self.ptr) }
    }

    fn minmax(&self) -> (f64, f64) {
        unsafe {
            let mut min = ~0.0;
            let mut max = ~0.0;
            gsl::gsl_matrix_minmax(self.ptr,
                                   ptr::to_mut_unsafe_ptr(min),
                                   ptr::to_mut_unsafe_ptr(max));
            (*min, *max)
        }
    }

    fn maxindex(&self) -> (u64, u64) {
        unsafe {
            let mut imax = ~0;
            let mut jmax = ~0;
            gsl::gsl_matrix_max_index(self.ptr,
                                      ptr::to_mut_unsafe_ptr(imax),
                                      ptr::to_mut_unsafe_ptr(jmax));
            (*imax, *jmax)
        }
    }

    fn minindex(&self) -> (u64, u64) {
        unsafe {
            let mut imin = ~0;
            let mut jmin = ~0;
            gsl::gsl_matrix_min_index(self.ptr,
                                      ptr::to_mut_unsafe_ptr(imin),
                                      ptr::to_mut_unsafe_ptr(jmin));
            (*imin, *jmin)
        }
    }

    fn minmax_index(&self) -> ((u64, u64), (u64, u64)) {
        unsafe {
            let mut imax = ~0;
            let mut jmax = ~0;
            let mut imin = ~0;
            let mut jmin = ~0;
            gsl::gsl_matrix_minmax_index(self.ptr,
                                      ptr::to_mut_unsafe_ptr(imin),
                                      ptr::to_mut_unsafe_ptr(jmin),
                                      ptr::to_mut_unsafe_ptr(imax),
                                      ptr::to_mut_unsafe_ptr(jmax));
            ((*imin, *jmin), (*imax, *jmax)) 
        }
    }

}


impl Drop for matrix {
    fn finalize(&self) {
        unsafe { gsl::gsl_matrix_free(self.ptr); }
    }
}


impl Clone for matrix {
    fn clone(&self) -> matrix {
        unsafe {
            let (row, col) = self.size;
            let m = matrix::zeros(row, col);
            gsl::gsl_matrix_memcpy(m.ptr, self.ptr);
            m
        }
    }
}


impl Eq for matrix {
    fn eq(&self, other: &matrix) -> bool {
        unsafe {
            gsl::gsl_matrix_equal(self.ptr, other.ptr) == 1
        }
    }

    fn ne(&self, other: &matrix) -> bool {
        unsafe {
            gsl::gsl_matrix_equal(self.ptr, other.ptr) == 0
        }
    }
}


impl Add<matrix, matrix> for matrix {
    fn add(&self, rhs: &matrix) -> matrix{
        unsafe {
            let new = self.clone();
            gsl::gsl_matrix_add(new.ptr, rhs.ptr);
            new
        }
    }
}


impl Sub<matrix, matrix> for matrix {
    fn sub(&self, rhs: &matrix) -> matrix{
        unsafe {
            let new = self.clone();
            gsl::gsl_matrix_sub(new.ptr, rhs.ptr);
            new
        }
    }
}
