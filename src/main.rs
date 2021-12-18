use fdtd::fdtd::FDTDSim;
use fdtd::snapshot;

fn main() {
    let mut s = FDTDSim::new(200);

    for step in 0..250 {
        s.step();

        if step % 10 == 0 {
            if let Err(_) = snapshot::write(&s, step / 10) {
                panic!();
            }
        }
    }
}
