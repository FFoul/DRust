pub mod conf;
pub mod par_strassen;
pub mod single_strassen;
pub mod utils;

use std::{fs::File, time::Instant, io::Write};

use conf::*;
use par_strassen::*;
use utils::*;


use crate::drust_std::{collections::dvec::*, utils::{ResourceManager, COMPUTES}};
use crate::{conf::*, dconnect};
use crate::drust_std::connect::dsafepoint::*;


pub async fn run() {
    unsafe {
        BRANCHES = Some(ResourceManager::new(BRANCH_NUM));
        COMPUTES = Some(ResourceManager::new(THREADS_NUM));
    }
    let mut matrix_a = DVec::with_capacity(MATRIX_SIZE * MATRIX_SIZE);
    let mut matrix_b = DVec::with_capacity(MATRIX_SIZE * MATRIX_SIZE);
    for _ in 0..MATRIX_SIZE {
        for _ in 0..MATRIX_SIZE {
            matrix_a.push(1 as i32);
            matrix_b.push(1 as i32);
        }
    }

    let start_time = Instant::now();
    

    let matrix_c = par_strassen_mul(matrix_a, matrix_b, MATRIX_SIZE, 1).await;

    let duration = start_time.elapsed();
    println!(
        "Time elapsed in matrix multiplication function() is: {:?}",
        duration
    );
    let file_name = format!(
        "{}/DRust_home/logs/gemm_drust_{}.txt", dirs::home_dir().unwrap().display(), NUM_SERVERS
    );
    let mut wrt_file = File::create(file_name).expect("file");
    let milli_seconds = duration.as_millis();
    writeln!(wrt_file, "{}", milli_seconds as f64 / 1000.0).expect("write");
    for i in (MATRIX_SIZE - 16)..MATRIX_SIZE {
        for j in (MATRIX_SIZE - 16)..MATRIX_SIZE {
            println!("matrix_c[{}, {}] = {}", i, j, matrix_c[i * MATRIX_SIZE + j]);
        }
    }
}


