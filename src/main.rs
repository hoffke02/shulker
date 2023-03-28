use std::{fs, iter, ops::Range};

fn main() {
    let (nums, lines) = parse_file("test.shk");
    for (num, line) in iter::zip(nums, &lines) {
        println!("{} {}", num, line);
    }

    let indent_nexts = make_indent_nexts(&lines);
    for i in indent_children(8, indent_nexts) {
        print!("{} ",i);
    }
}

// and indent_next is the index to the next line
// on the same indentation level,
// provided all lines in between have a higher indentation level
fn make_indent_nexts(lines: &Vec<String>) -> Vec<Option<usize>> {
    let mut indent_nexts = vec![None; lines.len()];
    let indentation: Vec<usize> = lines
        .iter()
        .map(|line| line.chars().take_while(|char| char.is_whitespace()).count())
        .collect();

    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if indentation[j] < indentation[i] {
                break;
            }

            if indentation[j] == indentation[i] {
                indent_nexts[i] = Some(j);
                break;
            }
        }
    }

    return indent_nexts;
}

// given an index i to some line
// returns an index iter of the children of that line
fn indent_children(i: usize, indent_nexts: Vec<Option<usize>>) -> Range<usize> {
    i+1..indent_nexts[i].unwrap()
}

// returns all non-empty lines from a file and their numbers
fn parse_file(file_name: &str) -> (Vec<usize>, Vec<String>) {
    fs::read_to_string(file_name)
        .expect("Unable to read file")
        .split_terminator('\n')
        .map(String::from)
        // add line numbers, starting at 1
        .enumerate()
        .map(|(num, line)| (num + 1, line))
        // ignore comments and empty lines
        .filter(|(_num, line)| !(line.is_empty() || line.trim().starts_with('#')))
        .unzip()
}
