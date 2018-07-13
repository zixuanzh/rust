// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
	fn rect_area(&self) -> f32 {
		let Rectangle { p1: Point {x: x1, y: y1}, 
						p2: Point {x: x2, y: y2}} = *self;
		(x2 - x1) * (y2 - y1)
	}
}

// Add a function square which takes a Point and a f32 as arguments, 
// and returns a Rectangle with its lower left corner on the point, 
// and a width and height corresponding to the f32.
fn square(p: Point, l: f32) -> Rectangle {
	let Point { x: _x, y: _y } = p;
	let _point = Point { x: _x + l, y: _y + l};
	Rectangle{ p1: p, p2: _point }
}

fn main() {

    // Instantiate a `Point`
    let point: Point = Point { x: 1.0, y: 1.0 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 3.0, y: 4.0 };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: new_point,
    };

    let area = _rectangle.rect_area();

    println!("Area of rectangle is {}", area);

    let square = square(Point { x: my_y, y: my_x }, 2.0);

    println!("Area of square is {}", square.rect_area());

}
