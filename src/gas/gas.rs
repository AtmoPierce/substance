pub trait Gas{
    fn reynolds_number_gas(&self, density: f64, flow_velocity: f64, characteristic_length: f64, dynamic_viscosity: f64)->f64{
        let reynolds_number = density * flow_velocity * characteristic_length / dynamic_viscosity;
        return reynolds_number;        
    }
}