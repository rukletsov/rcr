fn main() {
    println!("Hello world");
    println!("My name is {:?}, {}", "Bond", "James");

    #[derive(Debug)]
    struct S {
        x: i32,
    }

    impl std::fmt::Display for S {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{{{}}}", self.x)
        }
    }

    #[derive(Debug)]
    struct Complex {
        re: f64,
        im: f64,
    }

    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
        }
    }

    let c = Complex{re: 1.1, im: -2.5};

    println!("Struct: {0}; Struct debug: {0:?}", S{x:10});
    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display
        println!("{}", *color);
    }

    println!("1 - 2 = {}", 1i32 - 2);
    println!("0x80 >> 2 is 0x{:b}", 0x80u32 >> 2);
    println!("One million is written as {}", 1_000_000u64);

    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    let tuple1 = (1, 2);
    let (a, b) = tuple1;
    println!("{}, {}", a, b);

    impl std::fmt::Display for Matrix {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            writeln!(f, "( {} {} )", self.0, self.1)?;
            writeln!(f, "( {} {} )", self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let ys = [0; 500];
    let zs = [1, 2, 3, 4, 5];
    println!("array occupies {} bytes", std::mem::size_of_val(&zs));
}



#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
