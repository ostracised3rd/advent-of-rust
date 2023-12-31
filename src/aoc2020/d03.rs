pub struct AoC;

impl advent_of_rust::Day for AoC {
    // TODO: what is part 1 and 2 for this?
    fn p1(&self, data: String) {
        let grid = prepare_map(&data);
        let trees = stepping(&grid, 1, 3);
        tracing::info!("trees: {}", trees);
    }
    
    fn p2(&self, data: String) {
        let grid = prepare_map(&data);
    
        let patterns: Vec<(usize, usize)> = vec![
            (1, 1),
            (1, 3),
            (1, 5),
            (1, 7),
            (2, 1),
        ];
    
        let mut trees = 1;
        for (x, y) in patterns {
            trees *= stepping(&grid, x, y) as u64;
        }
    
        tracing::info!("{}", trees);
    }
}


fn map_column(row: &str) -> Vec<u32> {
    row
        .chars()
        .map(|x| if x == '#' {1} else {0})
        .collect::<Vec<u32>>()
}


fn prepare_map(raw: &str) -> Vec<Vec<u32>> {
    raw.lines()
        .map(|x| map_column(x))
        .collect::<Vec<Vec<u32>>>()
}

fn stepping(grid: &Vec<Vec<u32>>, step: usize, slide: usize) -> u32 {
    grid
        .iter()
        .step_by(step)
        .fold((0, 0), |(count, i), row| {
            (count + row[i%row.len()], i + slide)
        }).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_creation() {

        let text = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let res = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0], 
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1], 
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0], 
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1]
        ];


        assert_eq!(res, prepare_map(text));
    }

    #[test]
    fn tree_counter() {
        let grid = vec![
            vec![0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0], 
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1], 
            vec![0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0], 
            vec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0], 
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0], 
            vec![1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1], 
            vec![0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1]
        ];


        assert_eq!(2, stepping(&grid, 1, 1));
        assert_eq!(7, stepping(&grid, 1, 3));
        assert_eq!(3, stepping(&grid, 1, 5));
        assert_eq!(4, stepping(&grid, 1, 7));
        assert_eq!(2, stepping(&grid, 2, 1));
    }

}