use std::io; // library that is needed for input.

fn main() {
    println!("Program to make a measurement."); // idk

    let mut label = String::new(); // variable for the measurement label
    let mut val = String::new(); // variable 'val (short for value)'

    println!("Input the value you want for your measurement.");

    io::stdin() // input the value
        .read_line(&mut val)
        .expect("Failed to read line.");

    let val: f32 = val // parse 'val', turn it into a float.
        .trim()
        .parse()
        .expect("Not a number!");

    println!("\nInput the label for your measurement.");

    io::stdin() // input for the label
        .read_line(&mut label)
        .expect("Failed to read line.");
    
    labelled_measurement(val, label);
}

fn labelled_measurement(value: f32, measurement: String) {
    println!("\nYour measurement is: {}{}", value, measurement);
}