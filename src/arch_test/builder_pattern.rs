// 建造者模式
#[derive(Debug)]
struct Circle{
    x:f64,
    y:f64,
    radius:f64,
}
struct CircleBuilder{
    x:f64,
    y:f64,
    radius:f64,
}
impl Circle{
    fn area(&self) ->f64{
        std::f64::consts::PI*(self.radius*self.radius)
    }
    fn new()->CircleBuilder{
        CircleBuilder{x:0f64,y:0f64,radius:1.0}
    }
}

impl CircleBuilder{
    fn x(&mut self,coordinate:f64)->&mut CircleBuilder{
        self.x = coordinate;
        self
    }
    fn y(&mut self,coordinate:f64)->&mut CircleBuilder{
        self.y = coordinate;
        self
    }
    fn radius(&mut self,coordinate:f64)->&mut CircleBuilder{
        self.radius = coordinate;
        self
    }
    fn build(&self)->Circle{
        Circle{x:self.x,y:self.y,radius:self.radius}
    }
}

pub fn test(){
    let c = Circle::new().x(1.0)
        .y(2.0)
        .radius(2.0)
        .build();
    println!("{:?} {}", c,c.area());
}