use crate::Direction;

pub type Algorithm = fn(usize, usize, Vec<usize>, Direction) -> Return;
pub type Return = (usize, Vec<usize>);

pub fn fcfs(_: usize, head: usize, requests: Vec<usize>, _: Direction) -> (usize, Vec<usize>) {
    let mut total = 0;
    let mut path = vec![head];
    let mut current = head;

    for cylinder in requests {
        total += current.abs_diff(cylinder);
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

    let (first, second): (Vec<usize>, Vec<usize>) = match dir {
        Direction::Start => {
            let (left, mut right): (Vec<_>, Vec<_>) =
                cylinders.iter().partition(|cylinder| **cylinder < current);
            right.push(nrc - 1);
            (right, left)
        }
        Direction::End => {
            let (mut left, right): (Vec<_>, Vec<_>) =
                cylinders.iter().partition(|cylinder| **cylinder < current);
            left.insert(0, 0);
            (left, right)
        }
    };

    for cylinder in first.into_iter().rev() {
        total += current.abs_diff(cylinder);
        current = cylinder;
        path.push(current);
    }

    for cylinder in second {
        total += current.abs_diff(cylinder);
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

    let (first, second): (Vec<usize>, Vec<usize>) = match dir {
        Direction::Start => {
            let (mut left, mut right): (Vec<usize>, Vec<usize>) = cylinders
                .iter()
                .rev()
                .partition(|cylinder| **cylinder < current);
            left.push(0);
            right.insert(0, nrc - 1);
            (left, right)
        }
        Direction::End => {
            let (mut left, mut right): (Vec<_>, Vec<_>) =
                cylinders.iter().partition(|cylinder| **cylinder < current);
            left.insert(0, 0);
            right.push(nrc - 1);
            (right, left)
        }
    };

    for cylinder in first.into_iter() {
        total += current.abs_diff(cylinder);
        current = cylinder;
        path.push(current);
    }

    for cylinder in second {
        total += current.abs_diff(cylinder);
        current = cylinder;
        path.push(current);
    }

    (total, path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fcfs() {
        let (total, path) = fcfs(
            200,
            53,
            vec![98, 183, 37, 122, 14, 124, 65, 67],
            Direction::End,
        );
        assert_eq!(total, 640);
        assert_eq!(path, vec![53, 98, 183, 37, 122, 14, 124, 65, 67]);
    }

    #[test]
    fn test_scan() {
        let (total, path) = scan(
            200,
            53,
            vec![98, 183, 37, 122, 14, 124, 65, 67],
            Direction::End,
        );
        assert_eq!(total, 236);
        assert_eq!(path, vec![53, 37, 14, 0, 65, 67, 98, 122, 124, 183]);
    }

    #[test]
    fn test_cscan() {
        let (total, path) = cscan(
            200,
            53,
            vec![98, 183, 37, 122, 14, 124, 65, 67],
            Direction::End,
        );
        assert_eq!(total, 382);
        assert_eq!(path, vec![53, 65, 67, 98, 122, 124, 183, 199, 0, 14, 37]);
    }
}
