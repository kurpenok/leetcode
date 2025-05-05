pub struct ParkingSystem {
    pub spaces: Vec<i32>,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: vec![big, medium, small],
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        if self.spaces[(car_type - 1) as usize] > 0 {
            self.spaces[(car_type - 1) as usize] -= 1;
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut parking_system = ParkingSystem::new(1, 1, 0);

        assert_eq!(parking_system.add_car(1), true);
        assert_eq!(parking_system.add_car(2), true);
        assert_eq!(parking_system.add_car(3), false);
        assert_eq!(parking_system.add_car(1), false);
    }
}
