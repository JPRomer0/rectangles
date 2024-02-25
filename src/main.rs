fn main() {
    let int0 = Interval::new(0, 2).unwrap();
    let int1 = Interval::new(2,4).unwrap();
    let int2 = Interval::new(6,8).unwrap();
    let int3 = Interval::new(3,5).unwrap();
    let rect0 = Rectangle::new(int0,int0);
    let rect1 = Rectangle::new(int0, int1);
    let rect2 = Rectangle::new(int0, int3);
    let rects: Vec<Rectangle> = vec![rect0, rect1, rect2];
    let area = area_summation(&rects);
    println!("{}", area);

}

fn area_summation(rectangles: &Vec<Rectangle>) -> u128 {
    let mut total_area = 0;
    // 1 corresponds to no intersections, 2 corresponds to two rectangle intersections, and so on.
    let mut intersection_level = 1;
    // this is not 0 just so it enters the loop
    let mut area_modification = 1;
    while area_modification != 0 {
        // when the intersection is do deep it no longer yields any area the loop will exit
        area_modification = n_way_intersections_area(rectangles, intersection_level);
        match intersection_level % 2 {
            1 => total_area += area_modification,
            0 => total_area -= area_modification,
            _ => unreachable!(),
        }
        intersection_level += 1;
    }
    total_area
}

fn naive_area_summation(rectangles: &[&Rectangle]) -> u128 {
    let mut total_area = 0;
    for rect in rectangles.iter() {
        total_area += rect.area();
    }
    total_area
}

fn n_way_intersections_area(rectangles: &Vec<Rectangle>, n: usize) -> u128 {
    use itertools::Itertools; // to get .combinations()
    let mut total_area = 0;
    for rectangles_subset in rectangles.iter().combinations(n) {
        if let Some(rect) = rectangle_from_intersection(rectangles_subset.as_slice()) {
            total_area += rect.area();
        }
    }
    total_area
}

struct Rectangle(Interval, Interval);

#[derive(Clone, Copy)]
struct Interval {
    // no 0 length intervals allowed
    min: i64,
    max: i64,
}

impl Interval {
    fn new(min: i64, max: i64) -> Option<Interval> {
        if (max - min).is_positive() {
            return Some(Interval { min, max });
        } else {
            return None;
        }
    }

    fn length(&self) -> u128 {
        (self.max - self.min).try_into().unwrap()
    }
}

fn intersection(mut intervals: impl Iterator<Item = Interval>) -> Option<Interval> {
    intervals.try_fold(
        Interval {
            min: i64::MIN,
            max: i64::MAX,
        },
        |i0, i1| Interval::new(i0.min.max(i1.min), i0.max.min(i1.max)),
    )
}

impl Rectangle {
    fn new(int0: Interval, int1: Interval) -> Rectangle {
        Rectangle(int0, int1)
    }
    fn area(&self) -> u128 {
        self.0.length() * self.1.length()
    }
}

fn rectangle_from_intersection(rectangles: &[&Rectangle]) -> Option<Rectangle> {
    let interval0 = intersection(rectangles.iter().map(|rect| rect.0))?;
    let interval1 = intersection(rectangles.iter().map(|rect| rect.1))?;
    Some(Rectangle::new(interval0, interval1))
}
