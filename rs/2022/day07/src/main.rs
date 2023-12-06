use advent_of_code::*;
use std::cmp::min;
use std::collections::HashMap;

struct File {
    name: String,
    size: usize,
    sub_files: Vec<String>,
}

fn main() {
    let input = read_input_as_lines("2022/day07/src/input.txt");

    let files_by_name: HashMap<String, File> = {
        let mut result = HashMap::new();
        result.insert(
            String::from("/"),
            File {
                name: String::from("/"),
                size: 0,
                sub_files: Vec::new(),
            },
        );

        let mut dir_stack: Vec<String> = Vec::new();
        for line in input {
            if line.starts_with("$ cd") {
                let (_, new_dir) = line.split_once(" cd ").unwrap();
                if new_dir == ".." {
                    dir_stack.pop();
                } else {
                    dir_stack.push(new_dir.to_string());
                }
            } else if line.starts_with("$ ls") {
                continue;
            } else {
                let current_working_dir = dir_stack.join("/");

                let (dir_or_size, name) = line.split_once(' ').unwrap();
                let name = format!("{}/{}", dir_stack.join("/"), name);

                let file = result.get_mut(&current_working_dir).unwrap();
                file.sub_files.push(name.clone());

                if dir_or_size.starts_with("dir") {
                    result.entry(name.clone()).or_insert_with(|| File {
                        name: name.clone(),
                        size: 0,
                        sub_files: Vec::new(),
                    });
                } else {
                    let size = dir_or_size.parse::<usize>().unwrap();
                    result.entry(name.clone()).or_insert_with(|| File {
                        name: name.clone(),
                        size,
                        sub_files: Vec::new(),
                    });
                }
            }
        }

        result
    };

    let sizes = calculate_directory_sizes(&files_by_name);

    let solution1: usize = files_by_name
        .values()
        .filter(|file| !file.sub_files.is_empty())
        .map(|file| sizes.get(&file.name).unwrap())
        .filter(|&&size| size < 100000)
        .sum();

    let solution2 = {
        let total = 70000000;
        let required = 30000000;

        let used = *sizes.get("/").unwrap();
        let amount_need_to_delete = used - (total - required);

        let mut result = usize::MAX;

        for &size in sizes.values() {
            if size < amount_need_to_delete {
                continue;
            }

            result = min(result, size);
        }

        result
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn calculate_directory_sizes(files_by_name: &HashMap<String, File>) -> HashMap<String, usize> {
    let mut sizes = HashMap::new();

    let files = files_by_name.values().collect::<Vec<_>>();
    for file in files {
        helper(file, files_by_name, &mut sizes);
    }

    sizes
}

fn helper(
    file: &File,
    files_by_name: &HashMap<String, File>,
    sizes: &mut HashMap<String, usize>,
) -> usize {
    if let Some(size) = sizes.get(&file.name) {
        *size
    } else {
        let mut total_size: usize = file.size;

        for sub_file_name in file.sub_files.iter() {
            let sub_file = files_by_name.get(sub_file_name).unwrap();

            total_size += helper(sub_file, files_by_name, sizes);
        }

        sizes.insert(file.name.clone(), total_size);
        total_size
    }
}
