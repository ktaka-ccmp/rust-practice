// https://youtu.be/CHRNj5oubwc

struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan driving");
    }
}

struct Truck;
impl LandCapable for Truck {
    fn drive(&self) {
        println!("Truck driving");
    }
}

struct SUV;
impl LandCapable for SUV {}

struct Hovercraft;
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}
impl Amphibious for Hovercraft {
    fn traverse(&self) {
        println!("Hovercraft traversing frozen lake");
    }
}

trait LandCapable {
    fn drive(&self) {
        println!("Vehicle driving");
    }
}

trait WaterCapable {
    fn sail(&self) {
        println!("Vehicle sailing")
    }
}

trait Amphibious: LandCapable + WaterCapable {
    fn traverse(&self) {
        println!("Default function for Amphibious trait");
    }
}

fn road_trip(vehicle: &dyn LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.traverse();
    vehicle.sail();
    vehicle.drive();
}

pub(crate) fn pm1() {
    let suv = SUV;
    road_trip(&suv);

    let car = Sedan;
    road_trip(&car);

    let truck = Truck;
    road_trip(&truck);

    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
}
