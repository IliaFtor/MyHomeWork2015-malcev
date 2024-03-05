use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denom = other.real * other.real + other.imag * other.imag;
        let real = (self.real * other.real + self.imag * other.imag) / denom;
        let imag = (self.imag * other.real - self.real * other.imag) / denom;
        Complex::new(real, imag)
    }
}

fn main() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);

    println!("Сложение: c1 + c2 = {:?}", c1 + c2);
    println!("Вычитание: c1 - c2 = {:?}", c1 - c2);
    println!("Умножение: c1 * c2 = {:?}", c1 * c2);

    let complex_numbers = [Complex::new(5.0, 6.0), Complex::new(7.0, 8.0), Complex::new(9.0, 10.0)];

    println!("Комплексные числа:");
    for num in &complex_numbers {
        println!("{:?}", num);
    }

    let result = complex_numbers[0] / complex_numbers[1];
    println!("Деление: {:?}", result);
}
