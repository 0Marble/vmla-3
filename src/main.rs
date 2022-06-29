use std::path::PathBuf;

use eigen::find_poly;
use lu::{lu_gauss, make_lu};
use qr::{make_qr, qr_gauss};

mod complex;
mod eigen;
mod fraction;
mod io;
mod longint;
mod lu;
mod matrix;
mod number;
mod poly;
mod qr;

#[macro_export]
macro_rules! measure {
    ($action: expr) => {{
        let start = Instant::now();
        let res = $action;
        (res, start.elapsed())
    }};
}

#[test]
fn test_all() {
    let dir = std::fs::canonicalize(
        std::env::args()
            .take(4)
            .last()
            .expect("usage: cargo test {matrix folder}"),
    )
    .expect("invalid folder");

    println!("{}", dir.to_str().unwrap());

    for problem in 1..12 {
        match make_lu(&dir, problem) {
            Ok(_) => {}
            Err(e) => {
                println!("\tError: {}", e)
            }
        }
        println!("=====================");
    }

    println!("Gauss LU:");
    for problem in [3, 4, 8, 9] {
        match lu_gauss(&dir, problem) {
            Ok(_) => {}
            Err(e) => println!("\tError: {}", e),
        }
        println!("=====================");
    }

    println!("QR Decomposition:");
    for problem in [5, 6, 7, 8, 9] {
        match make_qr(&dir, problem) {
            Ok(_) => {}
            Err(e) => println!("\tError: {}", e),
        }
        println!("=====================");
    }

    println!("Gauss QR:");
    for problem in [8, 9] {
        match qr_gauss(&dir, problem) {
            Ok(_) => {}
            Err(e) => println!("\tError: {}", e),
        }
        println!("=====================");
    }

    println!("Characteristic Polynomial:");
    for problem in [10, 11] {
        match find_poly(&dir, problem) {
            Ok(_) => {}
            Err(e) => println!("Error: {e}"),
        }
        println!("=====================");
    }
}

enum Operation {
    MakeLu,
    LuGauss,
    MakeQr,
    QrGauss,
    FindPoly,
}

impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "make_lu" {
            Ok(Operation::MakeLu)
        } else if value == "lu_gauss" {
            Ok(Operation::LuGauss)
        } else if value == "make_qr" {
            Ok(Operation::MakeQr)
        } else if value == "qr_gauss" {
            Ok(Operation::QrGauss)
        } else if value == "find_poly" {
            Ok(Operation::FindPoly)
        } else {
            Err(format!("{value}: unknown operation"))
        }
    }
}

fn get_args() -> Option<(Operation, PathBuf, usize)> {
    let args: Vec<_> = std::env::args().collect();

    let operation = args.get(1)?.to_owned();
    let dir = args.get(2)?.to_owned();
    let dir = std::fs::canonicalize(dir).ok()?;
    let task = usize::from_str_radix(args.get(3)?, 10).ok()?;

    Some((Operation::try_from(operation).ok()?, dir, task))
}

fn main() {
    // lu_gauss(&std::fs::canonicalize("matrices").unwrap(), 4).unwrap();

    let(operation,dir,task) = get_args().expect("Usage: cargo run --release {make_lu|lu_gauss|make_qr|qr_gauss|find_poly} {matrix directory} {matrix number}");

    let res = match operation {
        Operation::MakeLu => make_lu(&dir, task),
        Operation::LuGauss => lu_gauss(&dir, task),
        Operation::MakeQr => make_qr(&dir, task),
        Operation::QrGauss => qr_gauss(&dir, task),
        Operation::FindPoly => find_poly(&dir, task),
    };

    match res {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {e}"),
    }
}
