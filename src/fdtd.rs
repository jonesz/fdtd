// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.
use crate::error;
use crate::grid::Grid;
use crate::step;
use fdtd_futhark::{Array_f64_1d, Array_f64_2d, Array_f64_3d, FutharkContext};

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

struct FutharkArr1d {
    hy: Array_f64_1d,
    chyh: Array_f64_1d,
    chye: Array_f64_1d,
    ez: Array_f64_1d,
    cezh: Array_f64_1d,
    ceze: Array_f64_1d,
}

// hx, chxh, chxe, hy, chyh, chye, ez, ceze, cezh
struct FutharkArr2d {
    hx: Array_f64_2d,
    chxh: Array_f64_2d,
    chxe: Array_f64_2d,
    hy: Array_f64_2d,
    chyh: Array_f64_2d,
    chye: Array_f64_2d,
    ez: Array_f64_2d,
    cezh: Array_f64_2d,
    ceze: Array_f64_2d,
}

struct FutharkArr3d {
    hx: Array_f64_3d,
    chxh: Array_f64_3d,
    chxe: Array_f64_3d,
    hy: Array_f64_3d,
    chyh: Array_f64_3d,
    chye: Array_f64_3d,
    hz: Array_f64_3d,
    chzh: Array_f64_3d,
    chze: Array_f64_3d,
    ex: Array_f64_3d,
    cexh: Array_f64_3d,
    cexe: Array_f64_3d,
    ey: Array_f64_3d,
    ceyh: Array_f64_3d,
    ceye: Array_f64_3d,
    ez: Array_f64_3d,
    cezh: Array_f64_3d,
    ceze: Array_f64_3d,
}

/// Populate the vector 'v' with the values of the passed 1D Array 'arr'.
fn arr1d_into_vec(v: &mut Vec<f64>, arr: Array_f64_1d) -> Result<(), error::FDTDError> {
    // TODO: Is it safe to just access the underlying values? There's
    // probably a performance hit here.
    let arr_vec = arr.to_vec()?;

    // TODO: Rather than copying, g.arr could just be aliased to the arr
    // created above.
    for i in 0..v.len() {
        v[i] = arr_vec.0[i];
    }

    Ok(())
}

/// Populate the vector 'v' with the values of the passed 2D Array 'arr'.
fn arr2d_into_vec(v: &mut Vec<f64>, arr: Array_f64_2d) -> Result<(), error::FDTDError> {
    let arr_vec = arr.to_vec()?;

    // TODO: This is dependent on whether the multidimensional-arr is
    // row/column oriented; the rust and futhark representation must align.
    // Introduce a test?
    for i in 0..v.len() {
        v[i] = arr_vec.0[i];
    }

    Ok(())
}

/// Populate the vector 'v' with the values of the passed 3D Array 'arr'.
fn arr3d_into_vec(v: &mut Vec<f64>, arr: Array_f64_3d) -> Result<(), error::FDTDError> {
    let arr_vec = arr.to_vec()?;

    // TODO: This is dependent on whether the multidimensional-arr is
    // row/column oriented; the rust and futhark representation must align.
    // Introduce a test?
    for i in 0..v.len() {
        v[i] = arr_vec.0[i];
    }

    Ok(())
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
        let dim = [g.x_sz as i64];
        let hy = Array_f64_1d::from_vec(*ctx, &g.hy, &dim)?;
        let chyh = Array_f64_1d::from_vec(*ctx, &g.chyh, &dim)?;
        let chye = Array_f64_1d::from_vec(*ctx, &g.chye, &dim)?;
        let ez = Array_f64_1d::from_vec(*ctx, &g.ez, &dim)?;
        let cezh = Array_f64_1d::from_vec(*ctx, &g.cezh, &dim)?;
        let ceze = Array_f64_1d::from_vec(*ctx, &g.ceze, &dim)?;

        Ok(FutharkArr1d {
            hy,
            chyh,
            chye,
            ez,
            cezh,
            ceze,
        })
    }

    /// Build arrays needed for a 2D Futhark step.
    // TODO: See the above note about caching.
    fn build_2d_futhark_arr(
        &mut self,
        g: &Grid,
        ctx: &mut FutharkContext,
    ) -> Result<FutharkArr2d, error::FDTDError> {
        let dim = [g.x_sz as i64, g.y_sz as i64];
        let hx = Array_f64_2d::from_vec(*ctx, &g.hx, &dim)?;
        let chxh = Array_f64_2d::from_vec(*ctx, &g.chxh, &dim)?;
        let chxe = Array_f64_2d::from_vec(*ctx, &g.chxe, &dim)?;
        let hy = Array_f64_2d::from_vec(*ctx, &g.hy, &dim)?;
        let chyh = Array_f64_2d::from_vec(*ctx, &g.chyh, &dim)?;
        let chye = Array_f64_2d::from_vec(*ctx, &g.chye, &dim)?;
        let ez = Array_f64_2d::from_vec(*ctx, &g.ez, &dim)?;
        let cezh = Array_f64_2d::from_vec(*ctx, &g.cezh, &dim)?;
        let ceze = Array_f64_2d::from_vec(*ctx, &g.ceze, &dim)?;

        Ok(FutharkArr2d {
            hx,
            chxh,
            chxe,
            hy,
            chyh,
            chye,
            ez,
            cezh,
            ceze,
        })
    }

    /// Build arrays needed for a 3D Futhark step.
    // TODO: See the above note about caching.
    fn build_3d_futhark_arr(
        &mut self,
        g: &Grid,
        ctx: &mut FutharkContext,
    ) -> Result<FutharkArr3d, error::FDTDError> {
        let dim = [g.x_sz as i64, g.y_sz as i64, g.z_sz as i64];
        let hx = Array_f64_3d::from_vec(*ctx, &g.hx, &dim)?;
        let chxh = Array_f64_3d::from_vec(*ctx, &g.chxh, &dim)?;
        let chxe = Array_f64_3d::from_vec(*ctx, &g.chxe, &dim)?;
        let hy = Array_f64_3d::from_vec(*ctx, &g.hy, &dim)?;
        let chyh = Array_f64_3d::from_vec(*ctx, &g.chyh, &dim)?;
        let chye = Array_f64_3d::from_vec(*ctx, &g.chye, &dim)?;
        let hz = Array_f64_3d::from_vec(*ctx, &g.hz, &dim)?;
        let chzh = Array_f64_3d::from_vec(*ctx, &g.chzh, &dim)?;
        let chze = Array_f64_3d::from_vec(*ctx, &g.chze, &dim)?;
        let ex = Array_f64_3d::from_vec(*ctx, &g.ex, &dim)?;
        let cexh = Array_f64_3d::from_vec(*ctx, &g.cexh, &dim)?;
        let cexe = Array_f64_3d::from_vec(*ctx, &g.cexe, &dim)?;
        let ey = Array_f64_3d::from_vec(*ctx, &g.ey, &dim)?;
        let ceyh = Array_f64_3d::from_vec(*ctx, &g.ceyh, &dim)?;
        let ceye = Array_f64_3d::from_vec(*ctx, &g.ceye, &dim)?;
        let ez = Array_f64_3d::from_vec(*ctx, &g.ez, &dim)?;
        let cezh = Array_f64_3d::from_vec(*ctx, &g.cezh, &dim)?;
        let ceze = Array_f64_3d::from_vec(*ctx, &g.ceze, &dim)?;

        Ok(FutharkArr3d {
            hx,
            chxh,
            chxe,
            hy,
            chyh,
            chye,
            hz,
            chzh,
            chze,
            ex,
            cexh,
            cexe,
            ey,
            ceyh,
            ceye,
            ez,
            cezh,
            ceze,
        })
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
                let (hy_arr, ez_arr) =
                    ctx.step_1d(arr.hy, arr.chyh, arr.chye, arr.ez, arr.cezh, arr.ceze)?;

                // Update 'Hy' and 'Ez' within the grid.
                arr1d_into_vec(&mut g.hy, hy_arr)?;
                arr1d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Two(Polarization::Magnetic) => {
                let arr = self.build_2d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr, ez_arr) = ctx.step_2d(
                    arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.ez, arr.cezh,
                    arr.ceze,
                )?;

                // Update 'Hx', 'Hy', and 'Ez' within the grid.
                arr2d_into_vec(&mut g.hx, hx_arr)?;
                arr2d_into_vec(&mut g.hy, hy_arr)?;
                arr2d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Three => {
                let arr = self.build_3d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr, hz_arr, ex_arr, ey_arr, ez_arr) = ctx.step_3d(
                    arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.hz, arr.chzh,
                    arr.chze, arr.ex, arr.cexh, arr.cexe, arr.ey, arr.ceyh, arr.ceye, arr.ez,
                    arr.cezh, arr.ceze,
                )?;

                // Update 'Hx', 'Hy', 'Hz', 'Ex', 'Ey', and 'Ez' within the grid.
                arr3d_into_vec(&mut g.hx, hx_arr)?;
                arr3d_into_vec(&mut g.hy, hy_arr)?;
                arr3d_into_vec(&mut g.hz, hz_arr)?;
                arr3d_into_vec(&mut g.ex, ex_arr)?;
                arr3d_into_vec(&mut g.ey, ey_arr)?;
                arr3d_into_vec(&mut g.ez, ez_arr)?;
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
                let hy_arr = ctx.hy_step_1d(arr.hy, arr.chyh, arr.chye, arr.ez)?;

                // Update 'Hy' within the grid.
                // TODO: We can pass the result of this operation to the below
                // 'ez_step_1d' and copy all at once for a likely small
                // performance bump.
                arr1d_into_vec(&mut g.hy, hy_arr)?;
            }

            GridDimension::Two(Polarization::Magnetic) => {
                let arr = self.build_2d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr) = ctx.magnetic_step_2d(
                    arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.ez,
                )?;

                // Update 'Hx' and 'Hy' within the grid.
                arr2d_into_vec(&mut g.hx, hx_arr)?;
                arr2d_into_vec(&mut g.hy, hy_arr)?;
            }

            GridDimension::Three => {
                let arr = self.build_3d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr, hz_arr) = ctx.magnetic_step_3d(
                    arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.hz, arr.chzh,
                    arr.chze, arr.ex, arr.ey, arr.ez,
                )?;

                // Update the 'Hx', 'Hy', and 'Hz' within the grid.
                arr3d_into_vec(&mut g.hx, hx_arr)?;
                arr3d_into_vec(&mut g.hy, hy_arr)?;
                arr3d_into_vec(&mut g.hz, hz_arr)?;
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
                let ez_arr = ctx.ez_step_1d(arr.ez, arr.cezh, arr.ceze, arr.hy)?;

                // Update 'Ez' within the grid.
                arr1d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Two(Polarization::Magnetic) => {
                let arr = self.build_2d_futhark_arr(g, &mut ctx)?;
                // ez, cezh, ceze, hy, hx.
                let ez_arr = ctx.ez_step_2d(arr.ez, arr.cezh, arr.ceze, arr.hx, arr.hy)?;

                // Update 'Ez' within the grid.
                arr2d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Three => {
                let arr = self.build_3d_futhark_arr(g, &mut ctx)?;
                // ex, cexe, cexh, ey, ceye, ceyh, ez, ceze, cezh, hx, hy, hz
                let (ex_arr, ey_arr, ez_arr) = ctx.electric_step_3d(
                    arr.ex, arr.cexh, arr.cexe, arr.ey, arr.ceyh, arr.ceye, arr.ez, arr.cezh,
                    arr.ceze, arr.hx, arr.hy, arr.hz,
                )?;

                // Update the 'Ez', 'Ey', and 'Ez' within the grid.
                arr3d_into_vec(&mut g.ex, ex_arr)?;
                arr3d_into_vec(&mut g.ey, ey_arr)?;
                arr3d_into_vec(&mut g.ez, ez_arr)?;
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
                let (hy_arr, ez_arr) = ctx.step_multiple_1d(
                    n as i64, arr.hy, arr.chyh, arr.chye, arr.ez, arr.cezh, arr.ceze,
                )?;

                // Update 'Hy' and 'Ez' within the grid.
                arr1d_into_vec(&mut g.hy, hy_arr)?;
                arr1d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Two(Polarization::Magnetic) => {
                let arr = self.build_2d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr, ez_arr) = ctx.step_multiple_2d(
                    n as i64, arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.ez,
                    arr.cezh, arr.ceze,
                )?;

                // Update 'Hx', 'Hy', and 'Ez' within the grid.
                arr2d_into_vec(&mut g.hx, hx_arr)?;
                arr2d_into_vec(&mut g.hy, hy_arr)?;
                arr2d_into_vec(&mut g.ez, ez_arr)?;
            }

            GridDimension::Three => {
                let arr = self.build_3d_futhark_arr(g, &mut ctx)?;
                let (hx_arr, hy_arr, hz_arr, ex_arr, ey_arr, ez_arr) = ctx.step_multiple_3d(
                    n as i64, arr.hx, arr.chxh, arr.chxe, arr.hy, arr.chyh, arr.chye, arr.hz,
                    arr.chzh, arr.chze, arr.ex, arr.cexh, arr.cexe, arr.ey, arr.ceyh, arr.ceye,
                    arr.ez, arr.cezh, arr.ceze,
                )?;

                // Update 'Hx', 'Hy', 'Hz', 'Ex', 'Ey', and 'Ez' within the grid.
                arr3d_into_vec(&mut g.hx, hx_arr)?;
                arr3d_into_vec(&mut g.hy, hy_arr)?;
                arr3d_into_vec(&mut g.hz, hz_arr)?;
                arr3d_into_vec(&mut g.ex, ex_arr)?;
                arr3d_into_vec(&mut g.ey, ey_arr)?;
                arr3d_into_vec(&mut g.ez, ez_arr)?;
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
            GridDimension::Three => step::magnetic_3d(g),
            _ => panic!("Unimplemented!"),
        };

        match &mut self.post_magnetic {
            Some(v) => v(self.time, g),
            None => (),
        }

        match self.dimension {
            GridDimension::One => step::electric_1d(g),
            GridDimension::Two(Polarization::Magnetic) => step::electric_2d(g),
            GridDimension::Three => step::electric_3d(g),
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
