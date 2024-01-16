advent_of_code::solution!(24);
use itertools::Itertools;
use num::Integer;

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones = input
        .lines()
        .map(|l| {
            l.split([',', '@'])
                .map(|w| w.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let res = hailstones
        .iter()
        .tuple_combinations()
        .filter(|(&(x1, y1, _, dx1, dy1, _), &(x2, y2, _, dx2, dy2, _))| {
            let m1 = dy1 / dx1;
            let m2 = dy2 / dx2;

            // parallel
            if (m2 - m1).abs() <= f64::EPSILON {
                return false;
            }

            // intersection
            let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
            let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);

            // check if in past
            if dx1.signum() != (x - x1).signum() || dx2.signum() != (x - x2).signum() {
                return false;
            }

            x >= 200000000000000.0
                && x <= 400000000000000.0
                && y >= 200000000000000.0
                && y <= 400000000000000.0
        })
        .count();

    Some(res.try_into().unwrap())
}

#[derive(Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

/// 3D vector implementation.
impl Vector {
    fn add(self, other: Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector { x, y, z }
    }

    fn sub(self, other: Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector { x, y, z }
    }

    fn cross(self, other: Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector { x, y, z }
    }

    // Changes the magnitude (but not direction) of the vector.
    // Prevents numeric overflow.
    fn gcd(self) -> Self {
        let gcd = self.x.gcd(&self.y).gcd(&self.z);
        let x = self.x / gcd;
        let y = self.y / gcd;
        let z = self.z / gcd;
        Vector { x, y, z }
    }

    fn sum(self) -> i128 {
        self.x + self.y + self.z
    }
}

type Hailstone = (i128, i128, i128, i128, i128, i128);

fn hailstone_to_vector(hailstones: &[Hailstone], index: usize) -> (Vector, Vector) {
    let (px, py, pz, vx, vy, vz) = hailstones[index];
    (
        Vector {
            x: px,
            y: py,
            z: pz,
        },
        Vector {
            x: vx,
            y: vy,
            z: vz,
        },
    )
}

fn get_relative_positions_and_velocities(
    p0: Vector,
    p1: Vector,
    p2: Vector,
    v0: Vector,
    v1: Vector,
    v2: Vector,
) -> (Vector, Vector, Vector, Vector) {
    let p3 = p1.sub(p0);
    let p4 = p2.sub(p0);
    let v3 = v1.sub(v0);
    let v4 = v2.sub(v0);
    (p3, p4, v3, v4)
}

// Calculate the cross products of velocities and positions and their GCD.
fn calculate_gcd_cross_product(v3: Vector, v4: Vector, p3: Vector, p4: Vector) -> Vector {
    let q = v3.cross(p3).gcd();
    let r = v4.cross(p4).gcd();
    q.cross(r).gcd()
}

// Find the times of intercept.
fn calculate_intercept_times(
    p3: Vector,
    p4: Vector,
    v3: Vector,
    v4: Vector,
    s: &Vector,
) -> (i128, i128) {
    let t = (p3.y * s.x - p3.x * s.y) / (v3.x * s.y - v3.y * s.x);
    let u = (p4.y * s.x - p4.x * s.y) / (v4.x * s.y - v4.y * s.x);
    assert!(t != u);
    (t, u)
}

fn calculate_original_position(
    p0: Vector,
    p3: Vector,
    p4: Vector,
    v3: Vector,
    v4: Vector,
    t: i128,
    u: i128,
) -> Option<i128> {
    let a = p0.add(p3).sum();
    let b = p0.add(p4).sum();
    let c = v3.sub(v4).sum();
    let res = (u * a - t * b + u * t * c) / (u - t);
    Some(res)
}

pub fn part_two(input: &str) -> Option<i128> {
    // Calculations need the range of `i128`.
    let hailstones: Vec<(i128, i128, i128, i128, i128, i128)> = input
        .lines()
        .map(|l| {
            l.split([',', '@'])
                .map(|w| w.trim().parse::<i128>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let (p0, v0) = hailstone_to_vector(&hailstones, 0);
    let (p1, v1) = hailstone_to_vector(&hailstones, 1);
    let (p2, v2) = hailstone_to_vector(&hailstones, 2);

    let (p3, p4, v3, v4) = get_relative_positions_and_velocities(p0, p1, p2, v0, v1, v2);

    // Calculate the cross products and their greatest common divisor.
    let s = calculate_gcd_cross_product(v3, v4, p3, p4);

    // Find intercept times for the hailstones.
    let (t, u) = calculate_intercept_times(p3, p4, v3, v4, &s);

    // Calculate and return the original position.
    calculate_original_position(p0, p3, p4, v3, v4, t, u)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
