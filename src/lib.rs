mod characteristics;
// use characteristics::characteristics::{Mach, Reynolds};
mod gas;
use gas::gas::Gas;
mod liquid;
use liquid::liquid::Liquid;
mod solid;
use solid::solid::Solid;

#[derive(Debug, Clone, Default)]
pub struct Air{
    id: String,
    pressure: f64,
    density: f64,
    isentropic_expansion_factor: f64,
    gas_constant: f64,
    dynamic_viscosity: f64
}

impl Default for Air{
    fn default()->Self{
        Air{
            id: String::from("sea_level_air"),
            pressure: 101325.0,                             // Pa
            density: 1.225,                                 // (kg/m^3)
            isentropic_expansion_factor: 1.4,               // (dimensionless)
            gas_constant: 287.057,                          // J/(kg*K)
            dynamic_viscosity: 1.789 * 10.0_f64.powf(-5.0)  // (Pa*s)
        }
    }
}

impl Air{
    pub fn new(pressure: f64, density: f64, isentropic_expansion_factor: f64, gas_constant: f64, dynamic_viscosity: f64)->Self{
        Air{
            id: format!(
                "{}_{}_{}_{}_{}",
                pressure,
                density,
                isentropic_expansion_factor,
                gas_constant,
                dynamic_viscosity
            ),
            pressure: pressure,
            density: density,
            isentropic_expansion_factor: isentropic_expansion_factor,
            gas_constant: gas_constant,
            dynamic_viscosity: dynamic_viscosity
        }
    }
    pub fn get_density(&self)->f64{
        return self.density;
    }
    
    pub fn characteristic_velocity(&self)->f64{
        let characteristic_velocity = (self.isentropic_expansion_factor * (self.pressure / self.density)).powf(0.5);
        return characteristic_velocity 
    }

    pub fn reynolds_number(&self, flow_velocity: f64, characteristic_length: f64)->f64{
        let reynolds_number = self.reynolds_number_gas(self.density, flow_velocity, characteristic_length, self.dynamic_viscosity);
        return reynolds_number;
    }
}

impl Gas for Air{

}