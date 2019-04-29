pub struct CartestianCoord {
    pub x: f64,
    pub y: f64,
}

pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

pub trait Coordinates {
    fn to_cartesian(self) -> CartestianCoord;
    fn from_cartesian(cart: CartestianCoord) -> Self;
}

impl Coordinates for CartestianCoord {
    fn to_cartesian(self) -> CartestianCoord {
        self
    }
    fn from_cartesian(cart: CartestianCoord) -> Self {
        cart
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartestianCoord {
        CartestianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartestianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.y).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartestianCoord {
        CartestianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartestianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn print_point<P: Coordinates>(point: P) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y)
}

fn main() {
    print_point((0.0, 1.0)); // (0, 1)
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });
}
