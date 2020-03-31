use ganja_rs::cga::*;
use ganja_rs::ganja_graph;
use rand::Rng;

// Example from https://github.com/pygae/cppganja

#[allow(dead_code)]
fn no() -> CGA {
    0.5 * (CGA::e5() - CGA::e4())
}
fn ni() -> CGA {
    CGA::e4() + CGA::e5()
}

/// Convenience function to get a random point near the origin
fn random_point<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    CGA::up(
        rng.gen_range(-scale, scale),
        rng.gen_range(-scale, scale),
        rng.gen_range(-scale, scale),
    )
}

/// Convenience function to get a random line near the origin
fn random_line<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    (random_point(rng, scale) ^ random_point(rng, scale) ^ ni()).normalized()
}

/// Convenience function to get a random circle near the origin
fn random_circle<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    (random_point(rng, scale) ^ random_point(rng, scale) ^ random_point(rng, scale)).normalized()
}

/// Convenience function to get a random plane near the origin
fn random_plane<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    (random_point(rng, scale) ^ random_point(rng, scale) ^ random_point(rng, scale) ^ ni())
        .normalized()
}

fn main() -> std::result::Result<(), std::io::Error> {
    use rand::thread_rng;
    use std::fs::File;
    use std::io::BufWriter;

    let mut rng = thread_rng();
    let point = random_point(&mut rng, 2.0);
    let line = random_line(&mut rng, 2.0);
    let circle = random_circle(&mut rng, 2.0);
    let plane = random_plane(&mut rng, 2.0);

    let mut graph = ganja_graph::GanjaGraph {
        title: "random",
        p: 4,
        q: 1,
        r: 0,
        ..Default::default()
    };
    graph.add_bivector(point, 0x0, None);
    graph.add_bivector(line, 0xff0000, None);
    graph.add_bivector(circle, 0x0000ff, None);
    graph.add_bivector(plane, 0xaa000000, None);

    let fnout = "random.html";
    println!("Writing {}", fnout);
    let mut fout = BufWriter::new(File::create(fnout)?);
    graph.graph_html(&mut fout)
}
