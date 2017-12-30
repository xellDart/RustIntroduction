extern crate rand;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};

struct TrainingDatum { 
    input: (i8, i8, i8), 
    expected: i8, 
}

fn main() {
    // Learning rate 
    let eta = 0.0001; 
    // Number of iterations 
    let n = 100;

    let mut rng = rand::thread_rng();
    let training_data = [ 
        TrainingDatum { input: (0, 0, 1), expected: 0 }, 
        TrainingDatum { input: (0, 1, 1), expected: 1 }, 
        TrainingDatum { input: (1, 0, 1), expected: 1 }, 
        TrainingDatum { input: (1, 1, 1), expected: 1 }, ];

    let range = Range::new(0.0, 1.0);
    // We then initialize the weight vector with random data between 0 and 1:
    let mut w = (
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
        range.ind_sample(&mut rng),
    );

   println!("Starting training phase with {} iterations...", n); 
   // Training
    for _ in 0..n { 
       // Choose a random training sample 
       let &TrainingDatum { input: x, expected } = rng.choose(&training_data).unwrap(); 
       // Calculate the dot product 
       let result = dot(x, w); 
       // Calculate the error 
       let error = expected - heaviside(result); 
       // Update the weights 
       w.0 += eta * error as f64 * x.0 as f64; 
       w.1 += eta * error as f64 * x.1 as f64; 
       w.2 += eta * error as f64 * x.2 as f64; 
    }
    // Show result 
    for &TrainingDatum { input, .. } in &training_data { 
        let result = dot(input, w); 
        println!("{} OR {}: {:.*} -> {}", input.0, input.1, 8, result, heaviside(result)); 
    }
}

/// Heaviside Step Function 
fn heaviside(val: f64) -> i8 { (val >= 0.0) as i8 }

/// Dot product of input and weights 
fn dot(input: (i8, i8, i8), weights: (f64, f64, f64)) -> f64 { 
    input.0 as f64 * weights.0 
    + input.1 as f64 * weights.1 
    + input.2 as f64 * weights.2 
}
