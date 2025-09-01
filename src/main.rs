use std::io::{self, Write };

fn main() {
    //println!("Equation Solver for f(x) = ax + b");
    //let a:f64 = input_helper("a: ").trim().parse().expect("Invalid input");
    //let b:f64 = input_helper("b: ").trim().parse().expect("Invalid input");

    //calc_linear(a, b);

    println!("Equation Solver for f(x) = ax^2 + bx + c");
    let a:f64 = input_helper("a: ").trim().parse().expect("Invalid input");
    let b:f64 = input_helper("b: ").trim().parse().expect("Invalid input");
    let c:f64 = input_helper("c: ").trim().parse().expect("Invalid input");

    quadratic_equation(a, b, c);
}

fn linear_equation(a:f64, b:f64) {
    /* Simulates the calculation like you would do it with pen and paper
    let mut cal_zero:f64 = 0f64;
    println!("\n I. {} = {} * x + {}", cal_zero, a, b);

    cal_zero = cal_zero - b;
    println!(" II. {} = {} * x", cal_zero, a);

    cal_zero = cal_zero / a;
    println!(" III. {} = x", cal_zero);
    */

    println!("0 = {} * x + {}", a, b);

    let x:f64 = -1f64 * b / a;
    println!("\nx: {:.2}", x)
}

fn quadratic_equation(a:f64, b:f64, c:f64) {
    println!("\n0 = {} * x^2 + {} * x + {}\n", a, b, c);

    let discriminant:f64 = b * b - (4f64 * a * c);

    if discriminant < 0f64 {
        println!("Discriminant is negative -> no intersection with the axis and no real solution for x!"); // ex.: 1 1 1
        return;
    }

    let x1:f64 = (b * -1f64 + discriminant.sqrt()) / (2f64 * a);
    let x2:f64 = (b * -1f64 - discriminant.sqrt()) / (2f64 * a);

    if discriminant > 0f64 {
        println!("Discriminant is positive -> two real solutions for x!"); // ex.: 4 8 -12

        println!("x1 (+): {:.2}", x1);
        println!("x2 (-): {:.2}", x2);
    } else {
        println!("Discriminant is equal to zero -> vertex touches the axis, double root!"); // ex.: -4 12 -9

        println!("x: {:.2}", x1);
    }
}

fn input_helper(prompt:&str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
