use std::{fs::File, io::Read, io::Write, path::PathBuf};

use crate::{
    complex::Complex,
    matrix::{Matrix, MatrixError},
};

#[derive(Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(&self) -> &L {
        match &self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("unwrap_left(): is right"),
        }
    }

    pub fn unwrap_right(&self) -> &R {
        match &self {
            Either::Left(_) => panic!("unwrap_right(): is left"),
            Either::Right(r) => r,
        }
    }
}

pub enum QRMethod {
    Householder,
    Givens,
    GramSchmidt,
}

fn read_method(s: &str) -> (Option<QRMethod>, &str) {
    let next = s.trim_start();
    if next.starts_with("Method=") {
        let next = &next["Method=".len()..];
        match next.chars().next() {
            Some('1') => return (Some(QRMethod::Householder), &next["1".len()..]),
            Some('2') => return (Some(QRMethod::Givens), &next["2".len()..]),
            Some('3') => return (Some(QRMethod::GramSchmidt), &next["3".len()..]),

            _ => return (None, s),
        }
    }

    return (None, s);
}

pub fn read_mat<T: Read>(
    reader: &mut T,
) -> Result<(Either<Matrix<f32>, Matrix<Complex>>, Option<QRMethod>), MatrixError> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let s = &s[..];
    let (method, mut s) = read_method(&s);
    for (i, c) in s.char_indices() {
        if c == '[' {
            s = &s[i..];
            break;
        }
    }

    let (m1, s) = read_mat_simple(&s)?;
    if s.starts_with(",") {
        let (m2, _) = read_mat_simple(&s[",".len()..])?;

        if m1.height() != m2.height() || m1.width() != m2.width() {
            return Err(MatrixError::SizeMismatch);
        }

        return Ok((
            Either::Right(Matrix::from_vec(
                m1.elems_raw()
                    .iter()
                    .zip(m2.elems_raw().iter())
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect(),
                m1.width(),
            )?),
            method,
        ));
    }

    Ok((Either::Left(m1), method))
}

fn read_float(s: &str) -> Result<(f32, &str), MatrixError> {
    if !s.starts_with(|c: char| c.is_digit(10)) && !s.starts_with("-") {
        return Err(MatrixError::InvalidFileFormat);
    }

    let mut end = 0;
    for (i, c) in s.char_indices() {
        end = i;
        if c.is_whitespace() || c == ';' || c == ']' {
            break;
        }
    }
    let x: f32 = match s[..end].parse() {
        Ok(x) => Ok(x),
        Err(e) => Err(MatrixError::IOError(format!("{}", e))),
    }?;
    Ok((x, &s[end..]))
}

fn read_mat_simple(s: &str) -> Result<(Matrix<f32>, &str), MatrixError> {
    if s.starts_with("[") {
        let mut s = &s["[".len()..];
        let mut v: Vec<Vec<f32>> = Vec::new();
        let mut finished = false;
        let mut max_width = 0;

        while !finished {
            let mut row = Vec::new();
            loop {
                let mut cont = 0;
                for (i, c) in s.char_indices() {
                    if c != '.' && !c.is_whitespace() {
                        cont = i;
                        break;
                    }
                }
                s = &s[cont..];

                let (x, next) = read_float(s)?;
                s = next;
                row.push(x);
                if row.len() > max_width {
                    max_width = row.len();
                }

                if s.starts_with(";") {
                    v.push(row);
                    s = &s[";".len()..];
                    break;
                }

                if s.starts_with("]") {
                    v.push(row);
                    s = &s["]".len()..];
                    finished = true;
                    break;
                }
            }
        }

        let mut elems = Vec::new();
        for row in &mut v {
            row.resize(max_width, 0.0);
            elems.append(&mut row.clone());
        }

        Matrix::from_vec(elems, max_width).map(|m| (m, s))
    } else {
        Err(MatrixError::InvalidFileFormat)
    }
}

fn write_mat_simple(mat: &Matrix<f32>) -> String {
    let mut s = String::new();

    s += "[";
    if mat.height() > 0 && mat.width() > 0 {
        for i in 0..mat.width() - 1 {
            s += &format!("{} ", mat.get(0, i));
        }
        s += &format!("{}", mat.get(0, mat.width() - 1));

        for i in 1..mat.height() {
            s += ";\n";
            for j in 0..mat.width() - 1 {
                s += &format!("{} ", mat.get(i, j));
            }
            s += &format!("{}", mat.get(i, mat.width() - 1));
        }
    }
    s += "]";
    s
}

pub fn write_mat_f32(mat: &Matrix<f32>, file_path: &PathBuf) -> std::io::Result<()> {
    write!(
        File::create(file_path)?,
        "A = ...\n{};",
        write_mat_simple(mat)
    )
}

pub fn write_mat_complex(mat: &Matrix<Complex>, file_path: &PathBuf) -> std::io::Result<()> {
    let re = Matrix::from_vec(mat.elems_raw().iter().map(|z| z.re).collect(), mat.width()).unwrap();
    let im = Matrix::from_vec(mat.elems_raw().iter().map(|z| z.im).collect(), mat.width()).unwrap();

    write!(
        File::create(file_path)?,
        "A = complex({},{});",
        write_mat_simple(&re),
        write_mat_simple(&im)
    )
}
