use crate::units::farad::Farad;
use crate::units::henry::Henry;
use crate::units::ohm::Ohm;
use crate::units::volt::Volt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Components {
    Resistor(Ohm),
    Capacitor(Farad),
    Inductor(Henry),
    Voltage(Volt),
}

impl Components {
    pub fn from_resistor(ohm: Ohm) -> Components {
        Components::Resistor(ohm)
    }
    pub fn from_capacitor(farad: Farad) -> Components {
        Components::Capacitor(farad)
    }
    pub fn from_inductor(henry: Henry) -> Components {
        Components::Inductor(henry)
    }
    pub fn from_voltage(volt: Volt) -> Components {
        Components::Voltage(volt)
    }
    pub fn new_resistor(ohm_value: f64) -> Components {
        Components::Resistor(Ohm::new(ohm_value))
    }
    pub fn new_capacitor(farad_value: f64) -> Components {
        Components::Capacitor(Farad::new(farad_value))
    }
    pub fn new_inductor(henry_value: f64) -> Components {
        Components::Inductor(Henry::new(henry_value))
    }
    pub fn new_voltage(voltage_value: f64) -> Components {
        Components::Voltage(Volt::new(voltage_value))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_resistor() {
        use crate::units::ohm::Ohm;
        use crate::circuits::components::Components;
        let resistor = Components::from_resistor(Ohm::new(1.0));
        assert_eq!(resistor, Components::Resistor(Ohm::new(1.0)));
    }

    #[test]
    fn test_new_capacitor() {
        use crate::units::farad::Farad;
        use crate::circuits::components::Components;
        let capacitor = Components::from_capacitor(Farad::new(1.0));
        assert_eq!(capacitor, Components::Capacitor(Farad::new(1.0)));
    }

    #[test]
    fn test_new_inductor() {
        use crate::units::henry::Henry;
        use crate::circuits::components::Components;
        let inductor = Components::from_inductor(Henry::new(1.0));
        assert_eq!(inductor, Components::Inductor(Henry::new(1.0)));
    }

    #[test]
    fn test_new_voltage() {
        use crate::units::volt::Volt;
        use crate::circuits::components::Components;
        let voltage = Components::from_voltage(Volt::new(1.0));
        assert_eq!(voltage, Components::Voltage(Volt::new(1.0)));
    }
}