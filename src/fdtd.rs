// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.
use crate::error;
use crate::grid::Grid;
use crate::step;
use fdtd_futhark::FutharkContext;

/// TM^z or TE^z.
#[derive(Copy, Clone)]
pub enum Polarization {
    Magnetic,
    Electric,
}

#[derive(Copy, Clone)]
pub enum GridDimension {
    One,
    Two(Polarization),
    Three,
}

impl Default for GridDimension {
    fn default() -> Self {
        GridDimension::One
    }
}

#[derive(Copy, Clone)]
pub enum Backend {
    Native,  // the following rust impl.
    Futhark, // the futhark backend.
}

impl Default for Backend {
    fn default() -> Self {
        Backend::Native
    }
}

// TODO: Closures that fit type of A/B must be specified for compilation,
// even if the function is a NOP. This requires the programmer to write a NOP
// function then pass it; is this avoidable?
pub struct FDTDSim<A, B>
where
    A: FnMut(usize, &mut Grid), // post-magnetic update.
    B: FnMut(usize, &mut Grid), // post-electric update.
{
    dimension: GridDimension,
    backend: Backend,
    backend_context: Option<FutharkContext>,

    // TODO: There's multiple ways to do this: function pointers, closures,
    // boxed closures, etc. What's the most performant/flexible?
    post_magnetic: Option<A>,
    post_electric: Option<B>,
    time: usize,
}

impl<A, B> Default for FDTDSim<A, B>
where
    A: FnMut(usize, &mut Grid),
    B: FnMut(usize, &mut Grid),
{
    fn default() -> Self {
        FDTDSim {
            dimension: GridDimension::default(),
            backend: Backend::default(),
            backend_context: None,
            post_magnetic: None,
            post_electric: None,
            time: 0,
        }
    }
}

// TODO: See above note in FDTDSim about A/B specification when we want NOP
// functions.
impl<A, B> FDTDSim<A, B>
where
    A: FnMut(usize, &mut Grid),
    B: FnMut(usize, &mut Grid),
{
    /// Create a new FDTDSimulation.
    pub fn new(
        dimension: Option<GridDimension>,
        backend: Option<Backend>,
        a: Option<A>,
        b: Option<B>,
        time: Option<usize>,
    ) -> Result<Self, error::FDTDError> {
        // If needed, build the appropriate context.
        let context = match backend {
            Some(Backend::Futhark) => Some(FutharkContext::new()?),
            _ => None,
        };

        Ok(FDTDSim {
            dimension: dimension.unwrap_or_default(),
            backend: backend.unwrap_or_default(),
            backend_context: context,
            post_magnetic: a,
            post_electric: b,
            time: time.unwrap_or(0),
        })
    }

    pub fn set_post_magnetic(&mut self, f: Option<A>) {
        self.post_magnetic = f;
    }

    pub fn set_post_electric(&mut self, f: Option<B>) {
        self.post_electric = f;
    }

    /// Perform a step within the simulation for a given grid.
    // TODO: Handle the futhark backend!
    pub fn step(&mut self, g: &mut Grid) {
        self.time += 1;

        match self.dimension {
            GridDimension::One => step::magnetic_1d(g),
            GridDimension::Two(Polarization::Magnetic) => step::magnetic_2d(g),
            _ => panic!("Unimplemented!"),
        };

        match &mut self.post_magnetic {
            Some(v) => v(self.time, g),
            None => (),
        }

        match self.dimension {
            GridDimension::One => step::electric_1d(g),
            GridDimension::Two(Polarization::Magnetic) => step::electric_2d(g),
            _ => panic!("Unimplemented!"),
        };

        match &mut self.post_electric {
            Some(v) => v(self.time, g),
            None => (),
        }
    }
}
