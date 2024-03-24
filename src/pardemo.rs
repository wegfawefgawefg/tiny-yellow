use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator};

fn pardemo() {
    // Calculate the number of elements for roughly 4 GB of f32 values.
    let num_elements: usize = 2usize.pow(29);
    println!(
        "Creating a vector with {} elements (approx 4 GB)...",
        num_elements
    );

    // Initialize the vector with sequential values.
    let big_vector: Vec<f32> = (0..num_elements).map(|x| x as f32).collect();

    // Sequential operation
    let start_time_seq = Instant::now();
    let processed_vector_seq: Vec<f32> = big_vector.iter().map(|&x| x * 3.0).collect();
    let elapsed_seq = start_time_seq.elapsed();
    println!("Sequential operation completed in {:?}", elapsed_seq);

    // Parallel operation using Rayon
    let start_time_par = Instant::now();
    let processed_vector_par: Vec<_> = big_vector.par_iter().map(|&x| x * 3.0).collect();
    let elapsed_par = start_time_par.elapsed();
    println!("Parallel operation completed in {:?}", elapsed_par);

    // Display the comparison
    println!("Comparison: Sequential vs Parallel");
    println!("Sequential: {:?}", elapsed_seq);
    println!("Parallel: {:?}", elapsed_par);
}
