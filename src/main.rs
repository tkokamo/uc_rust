#[derive(Debug)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn reduce(self) -> Expr {
        match self {
            Expr::Number(x) => Expr::Number(x),
            Expr::Add(x, y) => {
                if x.reducible() {
                    Expr::Add(Box::new(x.reduce()), y)
                } else if y.reducible() {
                    Expr::Add(x, Box::new(y.reduce()))
                } else {
                    let xv : i32 = match x.reduce() {
                        Expr::Number(v) => v,
                        _ => 0,
                    };
                    let yv : i32 = match y.reduce() {
                        Expr::Number(v) => v,
                            _ => 0,
                    };
                    Expr::Number(xv + yv)
                }
            },
            Expr::Multiply(x, y) => {
                if x.reducible() {
                    Expr::Multiply(Box::new(x.reduce()), y)
                } else if y.reducible() {
                    Expr::Multiply(x, Box::new(y.reduce()))
                } else {
                    let xv : i32 = match x.reduce() {
                        Expr::Number(v) => v,
                        _ => 0,
                    };
                    let yv : i32 = match y.reduce() {
                        Expr::Number(v) => v,
                            _ => 0,
                    };
                    Expr::Number(xv * yv)
                }
            },
        }
    }
    fn reducible(&self) -> bool {
        match self {
            Expr::Number(_) => false,
            Expr::Add(_, _) => true,
            Expr::Multiply(_, _) => true,
        }
    }
}

fn main() {
    let mut m = Expr::Add(
                    Box::new(Expr::Multiply(Box::new(Expr::Number(1)), Box::new(Expr::Number(2)))),
                    Box::new(Expr::Multiply(Box::new(Expr::Number(3)), Box::new(Expr::Number(4)))),
                    );
    while m.reducible() {
        m = m.reduce();
        println!("{:?}", m);
    }
}
