use std::{fs::File, io::Write, path::PathBuf, time::Instant};

use crate::{
    eigen::characteristic_polynomial,
    io::{read_mat, write_mat_complex, write_mat_f32, Either, QRMethod},
    longint::LongInt,
    matrix::MatrixError,
    measure,
    number::from_f32_mat,
    qr::{gauss_from_qr, qr_givens, qr_gram_schmidt, qr_householder},
};
