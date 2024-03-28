struct Boeing {
    required_crew: u8,
    range: u16
}


struct Airbus {
    required_crew: u8,
    range: u16
}

trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        (available_crew >= required_crew) && (range + 150 > distance)
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        (available_crew >= required_crew) && (range + 290 > distance)
    }
}

struct Waypoint {
    name: String,
    lat: f64,
    long: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint 
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self {
            start,
            end
        }
     }

    fn distance(&self) -> f32 {
        const EATH_RADIUS_KM: f64 = 6371.0;
        let start_rads = self.start.lat.to_radians();
        let end_rads = self.end.lat.to_radians();

        let delta_lat = (self.start.lat - self.end.lat).to_radians();
        let delta_long = (self.start.long - self.end.long).to_radians();

        let inner_central_angle = f64::powi((delta_lat / 2.0).sin(),2)
            + start_rads.cos() * end_rads.cos()
            * f64::powi((delta_long / 2.0).sin(),2);
        
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EATH_RADIUS_KM * central_angle;
        distance as f32
    }
}

fn main() {
    let kcle = Waypoint {
        name: "KCLE".to_string(),
        lat: 41.4075,
        long: 81.851111
    };

    let kslc = Waypoint {
        name: "KSLC".to_string(),
        lat: 41.4075,
        long: 71.851111
    };

    let seg = Segment::new(kcle, kslc);
    let distance = seg.distance();
    println!("Distance: {:.1}km", distance);
}
