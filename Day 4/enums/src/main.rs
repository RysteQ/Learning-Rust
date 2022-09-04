enum engine {
    electric,
    combustion
}

enum fuel {
    petrol,
    gas,
    electricity
}

struct car {
    engine_type: engine,
    fuel_type: fuel,
    name: String
}

fn main() {
    let new_car: car = car {
        engine_type: engine::combustion,
        fuel_type: fuel::petrol,
        name: String::from("Toyota")
    };
}
