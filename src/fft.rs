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

fn fft_helper(v: &[Complex], root: Complex) -> Vec<Complex> {

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

    let a_e = fft_helper(&v_e, root * root);
    let a_o = fft_helper(&v_o, root * root);

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

pub fn fft(v: &Vec<Complex>) -> Vec<Complex> {
    let mut v_padded  = v.clone();

    let mut highest_power = 1;

    while highest_power < v.len() {
        highest_power *= 2;
    }

    for _i in v.len()..highest_power {
        v_padded.push(Complex {real: 0.0, imag: 0.0,});
    }

    let root = Complex {
        real: (-2.0 * std::f64::consts::PI / (highest_power as f64)).cos(),
        imag: (-2.0 * std::f64::consts::PI / (highest_power as f64)).sin()
    };

    let a = fft_helper(v_padded.as_slice(), root);

    a
}

