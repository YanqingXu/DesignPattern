struct Car {
    wheels: u32,
    color: String,
    engine: String,
}

impl Car {
    fn show_details(&self) {
        println!(
            "Car with {} wheels, color {} and {} engine.",
            self.wheels, self.color, self.engine
        );
    }
}

trait CarBuilder {
    fn set_wheels(&mut self, wheels: u32) -> &mut Self;
    fn set_color(&mut self, color: &str) -> &mut Self;
    fn set_engine(&mut self, engine: &str) -> &mut Self;
    fn build(&self) -> Car;
}

struct SedanCarBuilder {
    car: Car,
}

impl SedanCarBuilder {
    fn new() -> Self {
        SedanCarBuilder {
            car: Car {
                wheels: 0,
                color: String::from(""),
                engine: String::from(""),
            },
        }
    }
}

impl CarBuilder for SedanCarBuilder {
    fn set_wheels(&mut self, wheels: u32) -> &mut Self {
        self.car.wheels = wheels;
        self
    }

    fn set_color(&mut self, color: &str) -> &mut Self {
        self.car.color = color.to_string();
        self
    }

    fn set_engine(&mut self, engine: &str) -> &mut Self {
        self.car.engine = engine.to_string();
        self
    }

    fn build(&self) -> Car {
        Car {
            wheels: self.car.wheels,
            color: self.car.color.clone(),
            engine: self.car.engine.clone(),
        }
    }
}

fn main() {
    let mut builder = SedanCarBuilder::new();
    let car = builder
        .set_wheels(4)
        .set_color("Red")
        .set_engine("V6")
        .build();
    car.show_details();
}
