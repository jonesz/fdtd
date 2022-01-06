// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.
use crate::error;
use crate::grid::Grid;
use crate::step;
use fdtd_futhark::{Array_f64_1d, FutharkContext};

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

// Convenience structs. TODO: Passing vectors to these seems extremely error
// prone; perhaps encompass them in something akin to:
// `
//  enum SimVec {
//      Hx(Vec<f64>),
//      Hy(Vec<f64>),
//      ...
//  }
//`
// Force the programmer to do a tiny bit more work, but allow the compiler
// to detect function parameter mismatches.

// hy, chyh, chye, ez, cezh, ceze
struct FutharkArr1d(
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
);

// hx, chxh, chxe, hy, chyh, chye, ez, cezh, ceze
struct FutharkArr2d(
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
    Array_f64_1d,
);

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
        // TODO: What's the cost of building a new context on each step call?
        // We can avoid making 'step' functions mutable if we just build a new
        // mutable futhark context.
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

    /// Perform a single step for a given grid.
    pub fn step(&mut self, g: &mut Grid) -> Result<(), error::FDTDError> {
        self.step_mul(g, 1)
    }

    /// Perform multiple steps for a given grid.
    pub fn step_mul(&mut self, g: &mut Grid, n: usize) -> Result<(), error::FDTDError> {
        match self.backend {
            Backend::Native => {
                for _ in 0..n {
                    self.step_native(g)?;
                }

                Ok(())
            }

            Backend::Futhark => {
                // If we have post-{magnetic, electric}, we have to perform
                // those with native code. If not, we can do 'n' number
                // of steps and likely save on copying over the boundary.
                // The code for this has to be explicit; I doubt the compiler
                // can infer anything due to FFI.
                match (&self.post_magnetic, &self.post_electric) {
                    (None, None) => self.step_mul_futhark(g, n),

                    (None, _some) => {
                        for _ in 0..n {
                            self.step_single_futhark(g)?;
                        }

                        Ok(())
                    }

                    (_some, _) => {
                        for _ in 0..n {
                            self.step_split_futhark(g)?;
                        }

                        Ok(())
                    }
                }
            }
        }
    }

    /// Build arrays needed for a 1D Futhark step.
    // TODO: chyh, chye, cezh, ceze are likely static over the full run of
    // the simulation. Hopefully, the compiler can detect this, but if not,
    // build some sort of cache into FDTDSim?
    fn build_1d_futhark_arr(
        &mut self,
        g: &Grid,
        ctx: &mut FutharkContext,
    ) -> Result<FutharkArr1d, error::FDTDError> {
        let dim = [1i64, g.x_sz as i64];
        let hy = Array_f64_1d::from_vec(*ctx, &g.hy, &dim)?;
        let chyh = Array_f64_1d::from_vec(*ctx, &g.chyh, &dim)?;
        let chye = Array_f64_1d::from_vec(*ctx, &g.chye, &dim)?;
        let ez = Array_f64_1d::from_vec(*ctx, &g.ez, &dim)?;
        let cezh = Array_f64_1d::from_vec(*ctx, &g.cezh, &dim)?;
        let ceze = Array_f64_1d::from_vec(*ctx, &g.ceze, &dim)?;

        Ok(FutharkArr1d(hy, chyh, chye, ez, cezh, ceze))
    }

    /// Build arrays needed for a 2D Futhark step.
    // TODO: See the above note about caching.
    fn build_2d_futhark_arr(
        &mut self,
        g: &Grid,
        ctx: &mut FutharkContext,
    ) -> Result<FutharkArr2d, error::FDTDError> {
        panic!("Unimplemented!")
    }

    /// Perform a single futhark step for a given grid. Called when we only
    /// have a post_electric fn to call.
    fn step_single_futhark(&mut self, g: &mut Grid) -> Result<(), error::FDTDError> {
        // TODO: Propagate an error upward? The FutharkContext should have
        // been created on new.
        let mut ctx = self.backend_context.expect("No FutharkContext!");

        match self.dimension {
            GridDimension::One => {
                let arr = self.build_1d_futhark_arr(g, &mut ctx)?;
                let result = ctx.step_1d(arr.0, arr.1, arr.2, arr.3, arr.4, arr.5)?;

                // TODO: Update the Grid with 'Hy' and 'Ez'.
            }
            _ => panic!("Unimplemented!"),
        }

        self.time += 1;
        Ok(())
    }

    /// Perform a single futhark step for a given grid. Called when we have
    /// a post_magnetic fn to call.
    fn step_split_futhark(&mut self, g: &mut Grid) -> Result<(), error::FDTDError> {
        // TODO: Propagate an error upward? The FutharkContext should have
        // been created on new.
        let mut ctx = self.backend_context.expect("No FutharkContext!");

        // Perform the magnetic step.
        match self.dimension {
            GridDimension::One => {
                let arr = self.build_1d_futhark_arr(g, &mut ctx)?;
                let result = ctx.hy_step_1d(arr.0, arr.1, arr.2, arr.3)?;

                // TODO: Update Grid's representation of 'Hy'.
            }
            _ => panic!("Unimplemented!"),
        }

        // Perform the post-magnetic step.
        match &mut self.post_magnetic {
            Some(v) => v(self.time, g),
            None => (),
        }

        // Perform the electric step.
        match self.dimension {
            GridDimension::One => {
                let arr = self.build_1d_futhark_arr(g, &mut ctx)?;
                let result = ctx.ez_step_1d(arr.3, arr.4, arr.5, arr.0)?;

                // TODO: Update Grid's representation of 'Ez'.
            }
            _ => panic!("Unimplemented!"),
        }

        // Perform the post-electric step.
        match &mut self.post_electric {
            Some(v) => v(self.time, g),
            None => (),
        }

        self.time += 1;
        Ok(())
    }

    /// Perform multiple futhark steps for a given grid.
    fn step_mul_futhark(&mut self, g: &mut Grid, n: usize) -> Result<(), error::FDTDError> {
        // TODO: Propagate an error upward? The FutharkContext should have
        // been created on new.
        let mut ctx = self.backend_context.expect("No FutharkContext!");

        match self.dimension {
            GridDimension::One => {
                let arr = self.build_1d_futhark_arr(g, &mut ctx)?;
                let result =
                    ctx.step_multiple_1d(n as i64, arr.0, arr.1, arr.2, arr.3, arr.4, arr.5)?;

                // TODO: Update Grid's representation of 'Hy' and 'Ez'.
            }
            _ => panic!("Unimplemented!"),
        }

        self.time += n;
        Ok(())
    }

    /// Perform a native step for a given grid.
    fn step_native(&mut self, g: &mut Grid) -> Result<(), error::FDTDError> {
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

        self.time += 1;
        Ok(())
    }
}
