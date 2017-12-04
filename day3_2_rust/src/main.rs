fn main() {
    let input = 312051;
    let grid_side_length = 500; // This could be a lot smaller probably?

    let mut grid = vec![vec![0; grid_side_length]; grid_side_length];
    let center_coord = grid_side_length / 2;
    let mut x = center_coord;
    let mut y = center_coord;
    let mut i = 1;
    let mut n = 1;

    while n <= input {
        set_at_coords(&mut grid, x, y, n);
        // println!("{}, {}", x, y);
        // println!("{:?}", grid);
        i += 1;

        let point = next_coords(x, y, i);
        x = point.0;
        y = point.1;
        n = sum_adjacent(&grid, x, y);
        // println!("i {} x {} y {} n {}", i, x, y, n)
    }
    println!("{:?}", grid);
    println!("{}", n);
}

fn set_at_coords(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, n: i32) {
    let point = grid.get_mut(y).unwrap().get_mut(x).unwrap();
    *point = n
}

fn next_coords(x: usize, y: usize, i: i32) -> (usize, usize) {
    let root = ((i - 1) as f32).sqrt() as i32;
    println!("{}", root);

    let ring_side_length;
    if root % 2 == 0 {
        ring_side_length = root + 1;
    } else {
        if root * root == i - 1 {
            ring_side_length = root;
        } else {
            ring_side_length = root + 2;
        }
    }
    // println!("i {} side {}", i, ring_side_length);
    let last_in_ring = ring_side_length * ring_side_length;
    let distance_from_end_of_ring = last_in_ring - i + 1;
    println!("ring_side_length {} last_in_ring {} distance_from_end_of_ring {}",
        ring_side_length, last_in_ring, distance_from_end_of_ring);
    if distance_from_end_of_ring <= (ring_side_length - 1) {
        (x + 1, y)
    } else if distance_from_end_of_ring <= (ring_side_length - 1) * 2 {
        (x, y + 1)
    } else if distance_from_end_of_ring <= (ring_side_length - 1) * 3 {
        (x - 1, y)
    } else {
        (x, y - 1)
    }
}

fn sum_adjacent(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    grid.get(y).unwrap().get(x + 1).unwrap() +
        grid.get(y - 1).unwrap().get(x + 1).unwrap() +
        grid.get(y - 1).unwrap().get(x).unwrap() +
        grid.get(y - 1).unwrap().get(x - 1).unwrap() +
        grid.get(y).unwrap().get(x - 1).unwrap() +
        grid.get(y + 1).unwrap().get(x - 1).unwrap() +
        grid.get(y + 1).unwrap().get(x).unwrap() +
        grid.get(y + 1).unwrap().get(x + 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let v = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ];

        assert!(sum_adjacent(&v, 2, 1) == 4)
    }

    #[test]
    fn test_sum2() {
        let v = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 5, 4, 2, 0],
            vec![0, 10, 1, 1, 0],
            vec![0, 11, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ];

        assert!(sum_adjacent(&v, 2, 3) == 23)
    }

    #[test]
    fn test_set() {
        let mut v = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ];

        set_at_coords(&mut v, 2, 1, 4);
        assert!(*v.get(1).unwrap().get(2).unwrap() == 4)
    }

    #[test]
    fn test_move() {
        println!("{:?}", next_coords(2, 2, 2));
        assert!(next_coords(2, 2, 2) == (3, 2))
    }

    #[test]
    fn test_move2() {
        println!("{:?}", next_coords(3, 2, 3));
        assert!(next_coords(3, 2, 3) == (3, 1))
    }

    #[test]
    fn test_move3() {
        println!("{:?}", next_coords(3, 1, 4));
        assert!(next_coords(3, 1, 4) == (2, 1))
    }

    #[test]
    fn test_move4() {
        println!("{:?}", next_coords(2, 1, 5));
        assert!(next_coords(2, 1, 5) == (1, 1))
    }

    #[test]
    fn test_move5() {
        println!("{:?}", next_coords(1, 3, 8));
        assert!(next_coords(1, 3, 8) == (2, 3))
    }
}
