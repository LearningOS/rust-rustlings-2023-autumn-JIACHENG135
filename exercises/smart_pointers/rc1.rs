// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    let mercury = Planet::Mercury(Rc::clone(&sun));
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    drop(uranus);
    drop(saturn);
    drop(jupiter);
    drop(mars);
    drop(earth);
    drop(venus);
    drop(mercury);

    assert_eq!(Rc::strong_count(&sun), 1);
}
