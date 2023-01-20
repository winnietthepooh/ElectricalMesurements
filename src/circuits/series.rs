use crate::circuits::components::Components;
use crate::units::farad::Farad;
use crate::units::henry::Henry;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;

#[derive(PartialEq, Debug, Clone)]
pub struct Series {
    pub(crate) components: Vec<Components>,
}

impl Series {
    pub fn new(component: Components) -> Series {
        Series {components: vec![component]}
    }
    pub fn add(&mut self, component: Components) {
        self.components.push(component)
    }
    pub fn capacitance(&self) -> Farad {
        let mut capacitors = vec![];
        for components in &self.components {
            match components {
                Components::Capacitor(farad) => {
                    capacitors.push(farad);
                },
                _ => println!("Not a capacitor"),
            }
        }
        let mut total_capacitance = Farad::new(0.0);
        for capacitor in capacitors {
            total_capacitance += capacitor.reciprocal();
        }

        total_capacitance.reciprocal()
    }
    pub fn from_resistor(ohm: Ohm) -> Series {
        let component = Components::from_resistor(ohm);
        Series::new(component)
    }
    pub fn from_capacitor(farad: Farad) -> Series {
        let component = Components::from_capacitor(farad);
        Series::new(component)
    }
    pub fn from_inductor(henry: Henry) -> Series {
        let component = Components::from_inductor(henry);
        Series::new(component)
    }
    pub fn from_voltage(volt: Volt) -> Series {
        let component = Components::from_voltage(volt);
        Series::new(component)
    }
    pub fn new_resistor(ohm_value: f64) -> Series {
        let component = Components::new_resistor(ohm_value);
        Series::new(component)
    }
    pub fn new_capacitor(farad_value: f64) -> Series {
        let component = Components::new_capacitor(farad_value);
        Series::new(component)
    }
    pub fn new_inductor(henry_value: f64) -> Series {
        let component = Components::new_inductor(henry_value);
        Series::new(component)
    }
    pub fn new_voltage(voltage_value: f64) -> Series {
        let component = Components::new_voltage(voltage_value);
        Series::new(component)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::farad::Farad;
    use crate::units::henry::Henry;
    use crate::units::ohm::Ohm;
    use crate::units::volt::Volt;

    #[test]
    fn test_new() {
        let resistor = Series::new(Components::from_resistor(Ohm::new(1.0)));
        let capacitor = Series::new(Components::from_capacitor(Farad::new(1.0)));
        let inductor = Series::new(Components::from_inductor(Henry::new(1.0)));
        let voltage = Series::new(Components::from_voltage(Volt::new(1.0)));
        assert_eq!(resistor, Series {components: vec![Components::Resistor(Ohm::new(1.0))]});
        assert_eq!(capacitor, Series {components: vec![Components::Capacitor(Farad::new(1.0))]});
        assert_eq!(inductor, Series {components: vec![Components::Inductor(Henry::new(1.0))]});
        assert_eq!(voltage, Series {components: vec![Components::Voltage(Volt::new(1.0))]});
    }
}