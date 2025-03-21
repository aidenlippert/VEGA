use anyhow::Result;
use vegastar::VegaStar;

pub struct VegaGate {
    star: VegaStar,
}

impl VegaGate {
    pub fn new(star: VegaStar) -> Result<Self> {
        Ok(VegaGate { star })
    }

    pub fn did(&self) -> &str {
        self.star.did()
    }
}
