/** This program parses csv expressed in string format 
* We have a representation of penguins and their heights.
*/

fn main() {
    let penguin_data = "
    common name, length (cm)
    Little Penguin, 33
    Yellow-eyed Penguin, 65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            // We don't want to print empty lines.
            continue;
        }
     
        let fields: Vec<_> = record
            .split(",") // we are looking at a vector at this point, split by "," symbol.
            .map(|field| field.trim()) // This creates an iterable, `|var| fn(var)` seems to be the lambda/anonymous function syntax.
            .collect(); // Convert the iterable to vector.

        // Prints just about everything to std-err
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} 👉 {:?}", record, fields); // Prints to std-err instead of std-out.
        }

        let name = fields[0];
        let maybe_length: Result<f32, _> = fields[1].parse();
        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap();
        println!("{}, {} cm", name, length);
    }
}
