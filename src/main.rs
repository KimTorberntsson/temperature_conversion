use std::io;

#[derive(PartialEq, Eq, Debug)]
enum TemperatureUnit {
    Fahrenheit,
    Celcius,
    Kelvin
}

fn main() {
    println!("Let's convert some temperatures! Choose between\n1) Fahrenheit (F)\n2) Celcius (C)\n3) Kelvin (K)\n");
    
    println!("Write the unit to convert from");
    let first_unit = get_temperature_unit();
    
    println!("Write the unit to convert to");
    let second_unit = get_temperature_unit();

    println!("Write input temperature:");
    let value = get_temperature_value();

    let converted_temperature = convert_temperature(first_unit, second_unit, value);
    println!("The converted temperature is {}", converted_temperature);
}

fn get_temperature_unit() -> TemperatureUnit {
    loop {
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        match input_string.trim() {
            "Fahrenheit" | "F" => break TemperatureUnit::Fahrenheit,
            "Celcius" | "C" => break TemperatureUnit::Celcius,
            "Kelvin" | "K" => break TemperatureUnit::Kelvin,
            _ => {
                println!("Please input Fahrenheit, Celcius or Kelvin!");
            }
        };
    }
}

fn get_temperature_value() -> f64 {
    loop {
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        match temperature.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("please input a valid temperature");
            }
        }
    }
}

fn convert_temperature(from: TemperatureUnit, to: TemperatureUnit, value: f64) -> f64 {
    if from == to {
        return value;
    };

    if from == TemperatureUnit::Celcius {
        if to == TemperatureUnit::Kelvin {
            return value + 273.15;
        } else {
            return value * 1.8 + 32.
        }
    }

    if from == TemperatureUnit:: Kelvin {
        if to == TemperatureUnit::Celcius {
            return value - 273.15;
        } else {
            return 1.8*(value - 273.0) + 32.0;
        }
    }

    if from == TemperatureUnit::Fahrenheit {
        if to == TemperatureUnit::Celcius {
            return (value - 32.) / 1.8;
        } else {
            return 273.5 + ((value - 32.0) * (5.0/9.0));
        }
    }

    return (value - 32.) / 1.8;
}