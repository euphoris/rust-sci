extern mod std;

#[cfg(test)]
mod test {
    use sci::*;
    use linalg::*;
    use dist::*;
    use random::*;

    fn approx<T: NumCast, U: NumCast>(x: T, y: U) -> bool {
        let x1 = x.to_f64();
        let y1 = y.to_f64();
        y1-0.001 < x1 && x1 < y1+0.001
    }

    #[test]
    fn test_fdist() {
        let x = 2.0;
        let f = Fdist {df1: 1.0, df2: 3.0};
        assert!(approx(f.d(x), 0.09356362));
        assert!(approx(f.p(x), 0.7477845));
        assert!(approx(f.q(x), 1.0-f.p(x)));

        assert!(approx(f.pinv(f.p(x)), x));
        assert!(approx(f.qinv(f.q(x)), x));
    }


    #[test]
    fn test_random(){
        unsafe {
            let rng = alloc(borosh13());
            free(rng);
        }
    }
    #[test]
    fn test_vector(){
        let mut v = vector::zeros(3);
        v.set(1,3.0);
        assert!(v.get(1) == 3.0);

        v.set_all(10.0);
        assert!(v.get(0) == 10.0);
        assert!(v.get(1) == 10.0);
        assert!(v.get(2) == 10.0);

        v.set_zero();
        assert!(v.get(1) == 0.0);

        v.set_basis(0);
        assert!(v.get(0) == 1.0);
        assert!(v.get(1) == 0.0);

        let mut w = vector::zeros(3);
        w.set(0, 4.0);
        w = w + v;
        assert!(w.get(0) == 5.0);

        w = w - v;
        assert!(w.get(0) == 4.0);

        // clone
        let mut u = v.clone();
        assert!(u == v);
        u.set(0, v.get(0)+1.0);
        assert!(u != v);

        // min max
        v = vector::as_vector(@[3.0, 2.0, 1.0]);
        assert!(v.min() == 1.0);
        assert!(v.max() == 3.0);
        assert!(v.minmax() == (1.0, 3.0));
        assert!(v.maxindex() == 0);
        assert!(v.minindex() == 2);
        assert!(v.minmax_index() == (2, 0));
    }


    #[test]
    fn test_as_vector(){
        let v = vector::as_vector(@[1.0,2.0,3.0]);
        let w = vector::as_vector(@[4.0,5.0,6.0]);
        let u = v + w;
        assert!(v.get(0) == 1.0);
        assert!(w.get(0) == 4.0);
        assert!(u.get(0) == 5.0);

        assert!(v == v);

        assert!(vector::zeros(2).isnull());
    }


    #[test]
    fn test_matrix(){
        let mut m1 = matrix::zeros(2,2);
        m1.set_all(1.0);
        assert!(m1.get(0, 0) == 1.0);

        let mut m2 = matrix::zeros(2,2);
        m2.set_all(2.0);
        assert!(m2.get(0, 0) == 2.0);

        let mut m = m1 + m2;
        assert!(m.get(0,0) == 3.0);

        m = m - m1;
        assert!(m.get(0,0) == 2.0);

        assert!(m == m);

        // clone
        let mut m3 = m.clone();
        assert!(m == m3);
        m3.set(0, 0, m.get(0, 0)+1.0);
        assert!(m != m3);

        assert!(matrix::zeros(2,2).isnull());

        // min max
        m = matrix::zeros(2,2);
        m.set(0, 0, 4.0);
        m.set(0, 1, 3.0);
        m.set(1, 0, 2.0);
        m.set(1, 1, 1.0);

        assert!(m.min() == 1.0);
        assert!(m.max() == 4.0);
        assert!(m.minmax() == (1.0, 4.0));
        assert!(m.maxindex() == (0, 0));
        assert!(m.minindex() == (1, 1));
        assert!(m.minmax_index() == ((1, 1), (0, 0)));

        // matrix multiplication
        let mul = m * m;
        assert!(mul.get(0, 0) == 4.0*4.0+3.0*2.0);
        assert!(mul.get(0, 1) == 4.0*3.0+3.0*1.0);
        assert!(mul.get(1, 0) == 2.0*4.0+1.0*2.0);
        assert!(mul.get(1, 1) == 2.0*3.0+1.0*1.0);

        // inverse
        let inv = m.inverse();
        assert!(inv.get(0,0) == -0.5);
        assert!(inv.get(0,1) ==  1.5);
        assert!(inv.get(1,0) ==  1.0);
        assert!(inv.get(1,1) == -2.0);
    }

    #[test]
    fn test_linalg(){
        let mut m = matrix::zeros(2,2);
        m.set(0, 0, 1.0);
        m.set(0, 1, 3.0);
        m.set(1, 0, 2.0);
        m.set(1, 1, 4.0);

        let lu = LU::decomp(&m);
        assert!(lu.mat.get(0,0) == 2.0);
        assert!(lu.mat.get(0,1) == 4.0);
        assert!(lu.mat.get(1,0) == 0.5);
        assert!(lu.mat.get(1,1) == 1.0);
    }
}
