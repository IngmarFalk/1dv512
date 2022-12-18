use crate::Direction;

pub type Algorithm = fn(usize, usize, Vec<usize>, Direction) -> (usize, Vec<usize>);

pub fn fcfs(nrc: usize, head: usize, cylinders: Vec<usize>, dir: Direction) -> (usize, Vec<usize>) {
    let mut total = 0;
    let mut path = vec![head];
    let mut current = head;

    for cylinder in cylinders {
        total += (cylinder - current).abs_diff(0);
        current = cylinder;
        path.push(current);
    }

    (total, path)
}

pub fn scan(
    nrc: usize,
    head: usize,
    mut cylinders: Vec<usize>,
    dir: Direction,
) -> (usize, Vec<usize>) {
    let mut total = 0;
    let mut path = vec![head];
    let mut current = head;

    cylinders.sort_unstable();

    let (left, right) = match dir {
        Direction::Left => (cylinders, vec![]),
        Direction::Right => (vec![], cylinders),
        Direction::End => {
            let (left, right) = cylinders.iter().partition(|cylinder| **cylinder < current);
            (left, right)
        }
    };

    for cylinder in left.into_iter().rev() {
        total += (cylinder - current).abs();
        current = cylinder;
        path.push(current);
    }

    total += current;

    for cylinder in right {
        total += (cylinder - current).abs();
        current = cylinder;
        path.push(current);
    }

    (total, path)
}

pub fn cscan(
    nrc: usize,
    head: usize,
    mut cylinders: Vec<usize>,
    dir: Direction,
) -> (usize, Vec<usize>) {
    let mut total = 0;
    let mut path = vec![head];
    let mut current = head;

    cylinders.sort_unstable();

    let (left, right) = match dir {
        Direction::Left => (cylinders, vec![]),
        Direction::Right => (vec![], cylinders),
        Direction::End => {
            let (left, right) = cylinders.iter().partition(|cylinder| **cylinder < current);
            (left, right)
        }
    };

    for cylinder in left.into_iter().rev() {
        total += (cylinder - current).abs();
        current = cylinder;
        path.push(current);
    }

    total += current;

    for cylinder in right {
        total += (cylinder - current).abs();
        current = cylinder;
        path.push(current);
    }

    total += nrc - 1 - current;

    (total, path)
}
