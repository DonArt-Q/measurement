use std::io;

fn main() {
    println!("Program to make a measurement."); // idk

    println!("");

    let mut label = String::new(); // variable for the measurement label
    let mut val = String::new(); // variable 'val (short for value)'

    println!("Input the value you want for your measurement (e.g, 65 or 0.6).");

    io::stdin() // input the value
        .read_line(&mut val)
        .expect("Failed to read line.");

    let val: f32 = val // parse 'val', turn it into a float.
        .trim()
        .parse()
        .expect("Not a number!");

    println!("\nInput the label for your measurement (e.g, km, ml).");

    io::stdin()
        .read_line(&mut label)
        .expect("Failed to read line.");
    
    labelled_measurement(val, label);
}

fn labelled_measurement(value: f32, measurement: String) {
    println!("\nYour measurement is: {}{}", value, measurement);
}