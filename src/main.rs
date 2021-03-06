#[derive(Debug)]
enum Expr {
    Number(i32),
    Boolean(bool),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn reduce(self) -> Expr {
        match self {
            Expr::Number(x) => Expr::Number(x),
            Expr::Boolean(x) => Expr::Boolean(x),
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
            Expr::LessThan(x, y) => {
                if x.reducible() {
                    Expr::LessThan(Box::new(x.reduce()), y)
                } else if y.reducible() {
                    Expr::LessThan(x, Box::new(y.reduce()))
                } else {
                    let xv : i32 = match x.reduce() {
                        Expr::Number(v) => v,
                        _ => 0,
                    };
                    let yv : i32 = match y.reduce() {
                        Expr::Number(v) => v,
                        _ => 0,
                    };
                    Expr::Boolean(xv < yv)
                }
            },
        }
    }
    fn reducible(&self) -> bool {
        match self {
            Expr::Number(_) => false,
            Expr::Boolean(_) => false,
            Expr::Add(_, _) => true,
            Expr::Multiply(_, _) => true,
            Expr::LessThan(_, _) => true,
        }
    }
}

struct Machine {
    expr: Box<Expr>,
}

impl Machine {
    fn run(self) {
        let mut e = self.expr;
        println!("{:?}", e);
        while e.reducible() {
            e = Box::new(e.reduce());
            println!("{:?}", e);
        }
    }
}

fn main() {
    Machine {expr: Box::new(Expr::Add(
                    Box::new(Expr::Multiply(Box::new(Expr::Number(1)), Box::new(Expr::Number(2)))),
                    Box::new(Expr::Multiply(Box::new(Expr::Number(3)), Box::new(Expr::Number(4)))),
                    ))}.run();
    
    Machine {expr: Box::new(Expr::LessThan(
                    Box::new(Expr::Number(5)),
                    Box::new(Expr::Add(Box::new(Expr::Number(2)), Box::new(Expr::Number(2)))),
                    ))}.run();
}
