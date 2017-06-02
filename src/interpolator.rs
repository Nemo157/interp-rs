use std::fmt;
use take::Take;

pub struct Interpolator<F>(Take<F>) where F: FnOnce(&mut fmt::Formatter) -> fmt::Result;

struct DisplayInterpolator<F>(Interpolator<F>) where F: FnOnce(&mut fmt::Formatter) -> fmt::Result;

impl<F> Interpolator<F> where F: FnOnce(&mut fmt::Formatter) -> fmt::Result {
    pub fn new(f: F) -> Interpolator<F> {
        Interpolator(Take::new(f))
    }

    pub fn write_to_io(self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "{}", DisplayInterpolator(self))
    }

    pub fn write_to_fmt(self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "{}", DisplayInterpolator(self))
    }

    pub fn to_string(self) -> String {
        format!("{}", DisplayInterpolator(self))
    }
}

impl<F> fmt::Display for DisplayInterpolator<F> where F: FnOnce(&mut fmt::Formatter) -> fmt::Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0).0.take()(f)
    }
}
