use crate::read_lines::read_lines;

pub fn eight() {
    let input = read_lines("eight").remove(0);
    let input = data_to_layers(input.as_str(), 25, 6);
    let result = num_one_digits_times_two_digits_from_layer_with_least_zeroes(input);

    assert_eq!(2375, result);
}

pub fn part_two() {
    let input = read_lines("eight").remove(0);
    let layers = data_to_layers(input.as_str(), 25, 6);
    let picture = part_two_generate_image(layers);


        for row in picture {
            for cell in row {
                if cell == 0 {
                    print!(" ");
                } else {
                    print!("O");
                }
            }
            println!()
        }
}

#[allow(dead_code)]
fn partition(s: &str, length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars();
    loop {
        let chunk_str: String = chars.by_ref().take(length).collect();
        if chunk_str.is_empty() {
            break;
        }
        result.push(chunk_str);
    }
    result
}

#[allow(dead_code)]
fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

#[allow(dead_code)]
fn to_digits(str: &str) -> Vec<i32> {
    str.chars().into_iter()
        .map(|c| c.to_digit(10))
        .map(|opt| opt.expect("failed to convert to digit"))
        .map(|u| u as i32)
        .collect()
}

fn data_to_layers(image_data: &str, width: usize, height: usize) -> Vec<Vec<Vec<i32>>> {
    let digits = to_digits(image_data);
    let chunks: Vec<Vec<i32>> = digits
        .chunks(width * height)
        .map(|d| d.to_vec())
        .collect();

    let layers: Vec<Vec<Vec<i32>>> = chunks.into_iter()
        .map(|v| v.chunks(width).map(|d| d.to_vec()).collect())
        .collect();

    layers
}

fn num_one_digits_times_two_digits_from_layer_with_least_zeroes(layers: Vec<Vec<Vec<i32>>>) -> i32 {
    let mut pair: Vec<(i32, Vec<Vec<i32>>)> = layers.into_iter()
        .map(|layer| {
            let num_zeroes: i32 = layer.iter()
                .map(|v| {
                    let zeroes: Vec<&i32> = v.iter().filter(|digit| digit == &&0).collect();
                    let zeroes = zeroes.len();
                    zeroes as i32
                })
                .sum();
            (num_zeroes, layer)
        }).collect();

    pair.sort_unstable_by(|(zeroes, _), (other, _)| zeroes.cmp(other));
    let (_, fewest) = pair.remove(0);

    let concatenated = vec_concat(fewest);

    let ones = concatenated.iter().filter(|digit| digit == &&1).collect::<Vec<&i32>>().len();
    let twos = concatenated.iter().filter(|digit| digit == &&2).collect::<Vec<&i32>>().len();

    (ones * twos) as i32
}

fn vec_concat(vec: Vec<Vec<i32>>) -> Vec<i32> {
    vec.into_iter().fold(vec![], |mut left, right| {
        left.extend(right);
        left
    })
}

fn part_two_generate_image(layers: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut picture = vec![vec![]];

    for layer in layers.into_iter().rev() {
        for (row, cells) in layer.into_iter().enumerate() {
            for (index, cell) in cells.into_iter().enumerate() {
                if picture.len() <= row {
                    picture.push(vec![]);
                }
                let row = picture.get_mut(row).expect("this shouldnt happen");
                if row.len() <= index {
                    row.push(2);
                }
                if cell == 2 {
                    continue;
                }
                std::mem::replace(&mut row[index], cell);
            }
        }
    }

    picture
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let image_data = "123456789012";
        let result = data_to_layers(image_data, 3, 2);
        assert_eq!(result,
                   vec![
                       vec![
                           vec![1, 2, 3],
                           vec![4, 5, 6],
                       ],
                       vec![
                           vec![7, 8, 9],
                           vec![0, 1, 2],
                       ]
                   ]
        );
    }

    #[test]
    fn test_first() {
        let image_data = "11220000000000110000123402670231";
        let layers = data_to_layers(image_data, 4, 4);
        let result = num_one_digits_times_two_digits_from_layer_with_least_zeroes(layers);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_vec_concat() {
        let input = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![5, 6, 7],
        ];

        let result = vec_concat(input);

        assert_eq!(result, vec![1, 2, 3, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn solve_first() {
        let input = read_lines("eight").remove(0);
        let input = data_to_layers(input.as_str(), 25, 6);
        let result = num_one_digits_times_two_digits_from_layer_with_least_zeroes(input);

        assert_eq!(2375, result);
    }

    #[test]
    fn testing_while_impl_part2() {
        assert_eq!(part_two_generate_image(data_to_layers("212", 1, 1)), vec![vec![1]]);

        assert_eq!(vec![vec![2]], part_two_generate_image(data_to_layers("22", 1, 1)));
        assert_eq!(vec![vec![1]], part_two_generate_image(data_to_layers("12", 1, 1)));
        assert_eq!(vec![vec![0]], part_two_generate_image(data_to_layers("02", 1, 1)));
        assert_eq!(vec![vec![0]], part_two_generate_image(data_to_layers("0000222222", 1, 1)));

        assert_eq!(vec![vec![2, 2], vec![2, 2]], part_two_generate_image(data_to_layers("22222222", 2, 2)));
        assert_eq!(vec![vec![1, 1], vec![1, 1]], part_two_generate_image(data_to_layers("11112222", 2, 2)));
        assert_eq!(vec![vec![0, 0], vec![0, 0]], part_two_generate_image(data_to_layers("00002222", 2, 2)));
    }

    #[test]
    fn test_part2() {
        let image_data = "0222112222120000";
        let layers = data_to_layers(image_data, 2, 2);
        let picture = part_two_generate_image(layers);

        assert_eq!(picture, vec![vec![0, 1], vec![1, 0]])
    }
}
