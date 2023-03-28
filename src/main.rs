use std::fs;

fn main() {
    parse_file("test.shk")
        .iter()
        .for_each(|source_line| {
            println!("Line {}: {}", source_line.number, source_line.text);
        })
}


// kindof just a line of text,
// except it knows where in the source code it came from
struct SourceLine<'a> {
    number: u32,
    text: String,
    file_name: &'a str,
}

// takes an input file and returns a Vec of SourceLine s
fn parse_file<'a>(file_name: &'a str) -> Vec<SourceLine<'a>> {
    fs::read_to_string(file_name)
        .expect("Unable to read file")
        .split_terminator('\n')
        .enumerate()
        .map(|(i, line)| SourceLine {
            number: i as u32 + 1,
            text: line.to_string(),
            file_name: &file_name,
        })
        .collect()
}

