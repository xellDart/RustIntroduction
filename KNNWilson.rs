/*
 KNN with Wilson edition
 You can train and test any data set like iris or ani csv with format [value, vlaue, ..., class(string) ]
 Developer: Miguel Angel Sánchez Bravo <masanchez@bankcoin.global>
*/
extern crate csv;
extern crate rand;

use rand::Rng;

use std::error::Error;
use std::io;
use std::process;
use csv::StringRecord;

#[derive(Debug, PartialEq, PartialOrd)]
struct TrainingData {
    input: Vec<f64>,
    expected: String,
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Distances {
    input: f64,
    expected: String,
}

fn is_number(number: &str) -> f64 {
    let num = number.parse::<f64>();
    match num {
        Ok(value) => return value,
        Err(_why) => (),
    }
    return 0.0;
}

fn to_vector(data: &StringRecord) -> Vec<f64> {
    let mut vec = vec![];
    for i in 0..(data.len() - 1) {
        let number: f64 = is_number(data.get(i).unwrap());
        vec.push(number)
    }
    return vec;
}

fn calculate_length(s: &StringRecord) -> usize {
    s.len() - 1
}

fn filter_array<'a>(data: &'a  Vec<TrainingData>, value: &TrainingData) -> Vec<&'a TrainingData> {
    let mut filter_data = vec![];
    for entry in data.iter() {
        // remove element from list
        if entry != value {
            filter_data.push(entry)
        }
    }
    // return new training data without element
    return filter_data
}

fn get_wilson_data<'a>(data: &'a Vec<TrainingData>) -> Vec<&'a TrainingData> {
    let mut wilson_data = vec![];
    // Compare one by one to all
    for mut entry in data {
        let mut distances_data = vec![]; // Refresh distance in every loop
        let mut new_data = filter_array(&data, entry);
        for &TrainingData { ref input, ref expected } in new_data {
            let mut result: f64 = 0.0;
            for i in 0..input.len() {
                let x1: f64 = *input.get(i).unwrap();
                let x2: f64 = *entry.input.get(i).unwrap();
                result += (x2 - x1).powf(2.0);
            }
            distances_data.push(
                Distances {
                    input: result.sqrt(),
                    expected: expected.to_string(),
                }
            );
        }
        sort_asc_distances(&mut distances_data);
        // Condition to append or remove item
        if &entry.expected == &distances_data[0].expected {
            wilson_data.push(entry);
        } else {
            println!("Removed: Expected: {:?} Class result {:?}", entry.expected, &distances_data[0]);
        }
    }
    return wilson_data
}

fn get_distances(training_data: Vec<&TrainingData>) -> Vec<Distances> {
    let mut rng = rand::thread_rng();
    let random_item: &TrainingData = rng.choose(&training_data).unwrap();
    println!();
    println!("Item to evaluate {:?}", random_item);
    let mut distances_data = vec![];
    let random_item: &TrainingData = &training_data[0];
    for TrainingData { ref input, ref expected } in training_data {
        let mut result: f64 = 0.0;
        for i in 0..input.len() {
            let x1: f64 = *input.get(i).unwrap();
            let x2: f64 = *random_item.input.get(i).unwrap();
            result += (x2 - x1).powf(2.0);
        }
        distances_data.push(
            Distances {
                input: result.sqrt(),
                expected: expected.to_string(),
            }
        );
    }
    return distances_data
}

fn sort_asc_distances(distances: &mut Vec<Distances>) {
    distances.sort_by(|a, b| a.input.partial_cmp(&b.input).unwrap());
}

fn run() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut training_data = vec![];
    for result in rdr.records() {
        let record = result?;
        training_data.push(TrainingData {
            input: to_vector(&record),
            expected: record.get(calculate_length(&record)).unwrap().to_string(),
        });
    }
    let w_data = get_wilson_data(&training_data);
    let mut distances_data: Vec<Distances> = get_distances(w_data);
    sort_asc_distances(&mut distances_data);
    println!("Class result {:?}", &distances_data[0]);
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
