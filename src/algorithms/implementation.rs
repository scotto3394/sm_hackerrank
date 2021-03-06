/// Round some numbers according to an arbitrary rule.
pub fn grading_students(n: Vec<u32>) -> Vec<u32> {
    let mut m = Vec::<u32>::new();
    for num in n {
        if num < 38 {
            m.push(num);
        } else {
            let ones_digit = num % 10;
            let mut diff = 10 - ones_digit;

            if ones_digit < 5 {
                diff = 5 - ones_digit;
            }

            if diff > 2 {
                diff = 0;
            }
            m.push(num + diff);
        }
    }
    m
}
//-----------------------------------------------------
/// Count apples and oranges hitting a house.
pub fn apples_oranges(
    house: Vec<i32>,
    tree_loc: Vec<i32>,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> Vec<usize> {
    let count_list = |x: Vec<i32>, ind: usize| {
        x.iter()
            .map(|y| y + tree_loc[ind])
            .filter(|&y| (y >= house[0]) && (y <= house[1]))
            .count()
    };
    let apple_count = count_list(apples, 0);
    let orange_count = count_list(oranges, 1);
    vec![apple_count, orange_count]
}
//-----------------------------------------------------
/// Find values `in between` two sets.
pub fn between_sets(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let step = *(a.iter().max().unwrap()) as usize;
    let cap = *(b.iter().min().unwrap()) as usize;

    let test_a = |x: u32| a.iter().all(|&y| x % y == 0);
    let test_b = |x: u32| b.iter().all(|&y| y % x == 0);

    let mut solution = 0;
    for i in (1..(cap + 1)).filter(|&x| x % step == 0) {
        let test = i as u32;
        if test_a(test) && test_b(test) {
            solution += 1;
        }
    }
    solution
}
//-----------------------------------------------------
/// Count number of new maximums and new minimums.
pub fn break_records(n: Vec<u64>) -> (u64, u64) {
    let mut max_val = n[0];
    let mut break_max = 0;
    let mut min_val = n[0];
    let mut break_min = 0;

    for num in n {
        if num > max_val {
            max_val = num;
            break_max += 1;
        }
        if num < min_val {
            min_val = num;
            break_min += 1;
        }
    }
    (break_max, break_min)
}
//-----------------------------------------------------
/// Sliding window sum of an array.
pub fn birthday_chocolate(n: Vec<u8>, day: u8, month: u8) -> u8 {
    let window = month as usize;
    let mut value = 0;
    for i in 0..window {
        value += n[i];
    }
    let mut solution = 0;
    if value == day {
        solution += 1;
    }

    for i in window..(n.len()) {
        value += n[i];
        value -= n[i - window];
        if value == day {
            solution += 1;
        }
    }
    solution
}
//-----------------------------------------------------
/// Find sum-pairs in an array, divisible by k.
pub fn sum_pairs(k: u16, a: Vec<u16>) -> u16 {
    let mut solution = 0;
    let n = a.len();
    for j in 1..n {
        for i in 0..j {
            if (a[j] + a[i]) % k == 0 {
                solution += 1;
            }
        }
    }
    solution
}
//-----------------------------------------------------
/// Bin an array, then find largest bin.
pub fn migratory_birds(n: Vec<usize>) -> usize {
    let mut counter: [u64; 5] = [0; 5];
    for num in n {
        counter[num - 1] += 1;
    }
    let mut max_index: usize = 0;
    let mut max_val: u64 = counter[0];
    for i in 1..5 {
        if counter[i] > max_val {
            max_val = counter[i];
            max_index = i;
        }
    }
    max_index + 1
}
//-----------------------------------------------------


pub struct Cycle {
    layers: usize,
    dimensions: [usize; 2],
    lengths: Vec<usize>,
    cycles: Vec<Vec<u64>>,
}

impl Cycle {
    ///Convert a matrix into a Cycle struct
    pub fn new(m: usize, n: usize, a: Vec<u64>) -> Cycle {
        let layers: usize = if n < m { n / 2 } else { m / 2 };
        let mut height = m;
        let mut width = n;
        let matrix_get = |a: &Vec<u64>, i: usize, j: usize| -> u64 { a[j + n * i] };
        let mut lengths = Vec::new();
        let mut cycles = Vec::new();
        for shell in 0..layers {
            let mut cycle = Vec::new();
            //Left side
            for i in 0..(height - 1) {
                cycle.push(matrix_get(&a, i + shell, shell));
            }
            //Bottom side
            for j in 0..(width - 1) {
                cycle.push(matrix_get(&a, (height - 1) + shell, j + shell));
            }
            //Right side
            for i in (1..height).rev() {
                cycle.push(matrix_get(&a, i + shell, (width - 1) + shell));
            }
            //Top side
            for j in (1..width).rev() {
                cycle.push(matrix_get(&a, shell, j + shell));
            }

            //Update relevant variables
            lengths.push(2 * (height - 1) + 2 * (width - 1));
            cycles.push(cycle);
            height -= 2;
            width -= 2;
        }

        Cycle {
            layers: layers,
            dimensions: [m, n],
            lengths: lengths,
            cycles: cycles,
        }
    }

    ///Rotate the matrix counterclockwise, r times.
    pub fn rotate(&mut self, r: usize) {
        let mut new_cycles = Vec::new();
        for i in 0..self.layers {
            //Rotate each layer
            let cycle = &self.cycles[i];
            let length_i = self.lengths[i];
            let mut new_cycle = vec![0; length_i];

            for j in 0..length_i {
                //Assign elements in each layer to future rotation.
                let new_index = (j + r) % length_i;
                new_cycle[new_index] = cycle[j]
            }
            new_cycles.push(new_cycle);
        }
        self.cycles = new_cycles;
    }

    ///Print matrix
    pub fn get_mat(&self) -> Vec<u64> {
        let mut height = self.dimensions[0];
        let n = self.dimensions[1];
        let mut width = n;
        let mut matrix: Vec<u64> = vec![0; height * width];

        for shell in 0..self.layers {
            let mut index = 0;
            let cycle = &self.cycles[shell];
            //Left side
            for i in 0..(height - 1) {
                let new_index = shell + (i + shell) * n;
                matrix[new_index] = cycle[index];
                index += 1;
            }
            //Bottom side
            for j in 0..(width - 1) {
                let new_index = (j + shell) + (height - 1 + shell) * n;
                matrix[new_index] = cycle[index];
                index += 1;
            }
            //Right side
            for i in (1..height).rev() {
                let new_index = (width - 1 + shell) + (i + shell) * n;
                matrix[new_index] = cycle[index];
                index += 1;
            }
            //Top side
            for j in (1..width).rev() {
                let new_index = (j + shell) + shell * n;
                matrix[new_index] = cycle[index];
                index += 1;
            }

            //Update relevant variables
            height -= 2;
            width -= 2;
        }
        matrix
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading_students() {
        let n = vec![73, 67, 38, 33];
        let solution = vec![75, 67, 40, 33];

        assert_eq!(solution, grading_students(n));
    }

    #[test]
    fn test_apples_oranges() {
        let house = vec![7, 11];
        let tree_loc = vec![5, 15];
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let solution = vec![1, 1];

        assert_eq!(solution, apples_oranges(house, tree_loc, apples, oranges));
    }

    #[test]
    fn test_between_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];

        let solution = 3;

        assert_eq!(solution, between_sets(a, b));
    }

    #[test]
    fn test_break_records() {
        let n = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        let solution = (2, 4);

        assert_eq!(solution, break_records(n));
    }

    #[test]
    fn test_birthday_chocolate() {
        let n = vec![1, 2, 1, 3, 2];
        let day = 3;
        let month = 2;

        let solution = 2;

        assert_eq!(solution, birthday_chocolate(n, day, month));
    }

    #[test]
    fn test_sum_pairs() {
        let k = 3;
        let a = vec![1, 3, 2, 6, 1, 2];

        let solution = 5;

        assert_eq!(solution, sum_pairs(k, a));
    }

    #[test]
    fn test_migratory_birds() {
        let n = vec![1, 4, 4, 4, 5, 3];
        let solution = 4;

        assert_eq!(solution, migratory_birds(n));
    }

    #[test]
    fn test_matrix_rotation() {
        let m = 4;
        let n = 4;
        let r = 1;
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let solution = vec![2, 3, 4, 8, 1, 7, 11, 12, 5, 6, 10, 16, 9, 13, 14, 15];
        let mut test = Cycle::new(m, n, a);
        test.rotate(r);
        assert_eq!(solution, test.get_mat());

    }
}
