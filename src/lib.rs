mod characteristics;
// use characteristics::characteristics::{Mach, Reynolds};
mod gas;
use gas::gas::Gas;
mod liquid;
use liquid::liquid::Liquid;
mod solid;
use solid::solid::Solid;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Air{
    pub id: String,
    pub height: f64, // in meters
    pub pressure: f64,
    pub density: f64,
    pub isentropic_expansion_factor: f64,
    pub gas_constant: f64,
    pub dynamic_viscosity: f64
}

impl Default for Air{
    fn default()->Self{
        Air{
            id: String::from("sea_level_air"),
            height: 0.0,                                   // in meters
            pressure: 101325.0,                             // Pa
            density: 1.225,                                 // (kg/m^3)
            isentropic_expansion_factor: 1.4,               // (dimensionless)
            gas_constant: 287.057,                          // J/(kg*K)
            dynamic_viscosity: 1.789 * 10.0_f64.powf(-5.0)  // (Pa*s)
        }
    }
}

impl Air{
    fn hash_air_id(raw: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}", &hash)[..12].to_string() // use first 12 hex digits
    }

    pub fn get_hash(height: f64, pressure: f64, density: f64, isentropic_expansion_factor: f64, gas_constant: f64, dynamic_viscosity: f64) -> String {
        let raw_id = format!("h{height}_p{pressure}_d{density}_g{isentropic_expansion_factor}_r{gas_constant}_v{dynamic_viscosity}");
        let air_id = Self::hash_air_id(&raw_id);
        return air_id;
    }

    pub fn new(height: f64, pressure: f64, density: f64, isentropic_expansion_factor: f64, gas_constant: f64, dynamic_viscosity: f64)->Self{
        let raw_id = format!("h{height}_p{pressure}_d{density}_g{isentropic_expansion_factor}_r{gas_constant}_v{dynamic_viscosity}");
        let air_id = Self::hash_air_id(&raw_id);
        Air{
            id: air_id,
            height: height,
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