#! /bin/bash

for TASK in {1..12}
do
    echo make_lu $TASK
    cargo run --release make_lu matrices $TASK
done

for TASK in 3 4 8 9
do
    echo lu_gauss $TASK
    cargo run --release lu_gauss matrices $TASK
done

for TASK in 5 6 7 8 9
do
    echo make_qr $TASK
    cargo run --release make_qr matrices $TASK
done

for TASK in 8 9
do
    echo qr_gauss $TASK
    cargo run --release qr_gauss matrices $TASK
done

for TASK in 10 11
do
    echo find_poly $TASK
    cargo run --release find_poly matrices $TASK
done