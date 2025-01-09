/*
Lifetimes in Function Calls
Lifetimes for function arguments and return values must be fully specified, but
Rust allows lifetimes ti be elided in most cases with a few simple rules.
This is not inference -- it is just a syntactic shorthand.
 * Each argument which ddoes not have a lifetime annotation is given one.
 * If there is only one argument lifetime, it is given to all un-annotated
  return values.
 * If there are multiple argument lifetimes, but the first one is for self,
  that lifetime is given to all un-annotated return values.
*/

#[derive(Debug)]
struct Point(i32, i32);

fn cab_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn nearest<'a>(points: &'a [Point], query: &Point) -> Option<&'a Point> {
    let mut nearest = None;
    for p in points {
        if let Some((_, nearest_dist)) = nearest {
            let dist = cab_distance(p, query);
            if dist < nearest_dist {
                nearest = Some((p, dist));
            }
        } else {
            nearest = Some((p, cab_distance(p, query)));
        };
    }
    nearest.map(|(p, _)| p)
}

fn main() {
    let points = &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1)];
    println!("{:?}", nearest(points, &Point(0, 2)));
}
