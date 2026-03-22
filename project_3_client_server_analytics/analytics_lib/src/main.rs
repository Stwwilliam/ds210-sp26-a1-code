use analytics_lib::csv::read_input_csv_file;

fn main() {
    let dataset = read_input_csv_file("dataset1.csv");
    println!("{dataset}");
}