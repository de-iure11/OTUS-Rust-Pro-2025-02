use rand::Rng;

#[derive(Debug)]
pub struct SmartSocket {
    pub amperes: f32,
    pub volts: f32,
    pub enabled: bool,
}

impl SmartSocket {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        SmartSocket {
            amperes: rng.gen_range(4.0..6.0),
            volts: rng.gen_range(210.0..230.0),
            enabled: true,
        }
    }

    pub fn power(&self) -> f32 {
        self.amperes * self.volts
    }

    pub fn status(&self) -> String {
        if self.enabled {
            "ON".to_owned()
        } else {
            "OFF".to_owned()
        }
    }

    pub fn on(&mut self) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        self.enabled = true;
        self.amperes = rng.gen_range(4.0..6.0);
        self.volts = rng.gen_range(210.0..230.0);
        Ok(())
    }

    pub fn off(&mut self) -> Result<(), ()> {
        self.enabled = false;
        self.amperes = 0.0;
        self.volts = 0.0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_socket_ok() {
        let mut socket = SmartSocket {
            enabled: true,
            amperes: 5.0,
            volts: 220.0,
        };

        assert!(socket.enabled);
        let power = socket.power();
        assert_eq!(power, 1100.00);
        let _ = socket.off();
        assert!(!socket.enabled);
        let power = socket.power();
        assert_eq!(power, 0.00);
    }
}
