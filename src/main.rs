use fdtd::fdtd::FDTDSim;

fn main() {
    let mut s = FDTDSim::new(200);

    for _ in 0..250 {
        s.step();
        println!("{:.32}", s.ez50());
    }
}
