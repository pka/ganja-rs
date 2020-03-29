mod cga;

use crate::cga::*;
use rand::Rng;

// Example from https://github.com/pygae/cppganja/tree/master/examples/GanjaCpp

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
    (random_point(rng, scale) ^ random_point(rng, scale) ^ CGA::ni()).normalized()
}

/// Convenience function to get a random circle near the origin
fn random_circle<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    (random_point(rng, scale) ^ random_point(rng, scale) ^ random_point(rng, scale)).normalized()
}

/// Convenience function to get a random plane near the origin
fn random_plane<R: Rng + ?Sized>(rng: &mut R, scale: f64) -> CGA {
    (random_point(rng, scale) ^ random_point(rng, scale) ^ random_point(rng, scale) ^ CGA::ni())
        .normalized()
}

// int main (int argc, char **argv) {
//     // We will seed the RNG to get different results
//     unsigned int  seed = time(NULL);
//     srand (seed);

//     // Print the seed in case we like the look
//     // of the scene and want to recreate
//     std::cout << "Seed: " << seed << std::endl;

//     // Ok lets make some objects
//     CGA point = random_point();
//     CGA line = random_line();
//     CGA circle = random_circle();
//     CGA plane = random_plane();

//     // Make the GanjaScene and add the objects to it
//     GanjaScene gs = GanjaScene();
//     gs.add_object(point, Color::BLACK); // You can specify color explicitly
//     gs.add_object(line, Color::RED);
//     gs.add_object(circle, Color::BLUE);
//     gs.add_object(plane); // Or allow it to default to a semi-see-through grey

//     // State what signature our algebra is from
//     std::vector<double> sig{1.0,1.0,1.0,1.0,-1.0};
//     std::string html = generate_full_html(gs.as_string(), sig);

//     // Write the output to an html file
//     std::ofstream myfile;
//     myfile.open ("example.html");
//     myfile << html;
//     myfile.close();

//   return 0;
// }

fn main() {
    use rand::thread_rng;

    let mut rng = thread_rng();
    let point = random_point(&mut rng, 1.0);
    let line = random_line(&mut rng, 1.0);
    let circle = random_circle(&mut rng, 1.0);
    let plane = random_plane(&mut rng, 1.0);
    println!("a point       : {}", point);
    println!("a line        : {}", line);
    println!("a circle      : {}", circle);
    println!("a plane       : {}", plane);
}
