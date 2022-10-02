use std::ops;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn mag(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn pow_int(self, pow: u64) -> Self {

        let mut ret = Complex {
            real: 1.0,
            imag: 0.0,
        };

        for _i in 0..pow {
            ret = ret * self;
        }

        ret
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let ac = self.real * other.real;
        let bd = self.imag * other.imag;
        let abcd = (self.real + self.imag) * (other.real + other.imag);

        Complex {
            real: ac - bd,
            imag: abcd - ac - bd,
        }
    }
}

impl ops::Div<f64> for Complex {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Complex { 
            real: self.real / other, 
            imag: self.imag / other,
        }
    }
}

pub fn fft(v: &[Complex], root: Complex) -> Vec<Complex> {

    //println!("{:?} {}", v, root);

    if v.len() == 1 {
        return vec![v[0]];
    }

    let mut v_e: Vec<Complex> = Vec::new();
    let mut v_o: Vec<Complex> = Vec::new();

    for i in 0..v.len() {
        if i & 1 == 0{
            v_e.push(v[i]);
        } else {
            v_o.push(v[i]);
        }
    }

    //println!("V_e, V_o {:?} {:?}", v_e, v_o);

    let a_e = fft(&v_e, root * root);
    let a_o = fft(&v_o, root * root);

    //println!("A_e, A_o: {:?} {:?}", a_e, a_o);

    let mut a: Vec<Complex> = vec![Complex{real: 0.0, imag: 0.0,}; v.len()];

    let half = v.len() / 2;

    for i in 0..half{
        a[i] = a_e[i] + root.pow_int(i as u64) * a_o[i];
        a[i+half] = a_e[i] - root.pow_int(i as u64) * a_o[i];
    }

    //println!("A: {:?}", a);
    
    a
}
pub fn mult_polynomial(p1: &Vec<f64>, p2: &Vec<f64>) -> Vec<f64> {
    let mut p1_complex = Vec::new();
    let mut p2_complex = Vec::new();

    let new_size = 2 * std::cmp::max(p1.len(), p2.len()) + 1;
    let mut highest_power: usize = 1;

    while highest_power < new_size {
        highest_power *= 2;
    }

    for i in p1{
        p1_complex.push(Complex{real: *i, imag: 0.0});
    }
    for i in p2{
        p2_complex.push(Complex{real: *i, imag: 0.0});
    }

    for _i in p1_complex.len()..highest_power {
        p1_complex.push(Complex{real: 0.0, imag: 0.0});
    }
    for _i in p2_complex.len()..highest_power {
        p2_complex.push(Complex{real: 0.0, imag: 0.0});
    }

    let root = Complex {
        real: (-2.0 * std::f64::consts::PI / (highest_power as f64)).cos(),
        imag: (-2.0 * std::f64::consts::PI / (highest_power as f64)).sin()
    };
    let root_inv = Complex {
        real: (2.0 * std::f64::consts::PI / (highest_power as f64)).cos(),
        imag: (2.0 * std::f64::consts::PI / (highest_power as f64)).sin()
    };

    let v1 = fft(&p1_complex, root);
    let v2 = fft(&p2_complex, root);

    let mut vm = Vec::new();

    for i in 0..highest_power {
        vm.push(v1[i] * v2[i]);
    }

    let mut ans_complex = fft(&vm, root_inv);
    
    for i in 0..highest_power {
        ans_complex[i] = ans_complex[i] / highest_power as f64;
    }

    let mut ans = Vec::new();

    for i in ans_complex {
        ans.push(i.real);
    }

    ans


}
