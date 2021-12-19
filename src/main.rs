use fdtd::fdtd::FDTDSim;
use fdtd::snapshot;

fn main() {
    let mut s = FDTDSim::new(200);

    let dir_name = snapshot::create_output_dir().unwrap();

    for step in 0..250 {
        s.step();

        if step % 10 == 0 {
            snapshot::write(&s, &dir_name, step / 10).unwrap();
        }
    }
}
