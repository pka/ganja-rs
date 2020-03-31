#![allow(non_snake_case)]

use ganja_rs::cga::*;
use ganja_rs::ganja_graph;
use std::fs::File;
use std::io::BufWriter;

// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#cga3d_intersections

// We start by defining a null basis, and upcasting for points
//   var ni = 1e4+1e5, no = .5e5-.5e4, nino = ni^no;
//   var up = (x)=> no + x + .5*x*x*ni;

fn no() -> CGA {
    0.5 * (CGA::e5() - CGA::e4())
}
fn ni() -> CGA {
    CGA::e4() + CGA::e5()
}

fn up(x: CGA) -> CGA {
    no() + &x + 0.5 * &x * &x * ni()
}

fn main() -> std::result::Result<(), std::io::Error> {
    // var p  = up(0),                          // point
    //   S  = ()=>!(p-.5*ni),                 // main dual sphere around point (interactive)
    //   S2 = !(up(-1.4e1)-0.125*ni),         // left dual sphere
    //   C  = !(up(1.4e1)-.125*ni)&!(1e3),    // right circle
    //   L  = up(.9e2)^up(.9e2-1e1)^ni,       // top line
    //   P  = !(1e2-.9*ni),                   // bottom dual plane
    //   P2 = !(1e1+1.7*ni);                   // right dual plane
    // point
    let p = up(CGA::zero());
    // main dual sphere around point (interactive)
    let S = !&(&p - 0.5 * ni());
    // left dual sphere
    let S2 = !&(up(-1.4 * CGA::e1()) - 0.125 * ni());
    // right circle
    let C = !&(up(1.4 * CGA::e1()) - 0.125 * ni()) & !&CGA::e3();
    // top line
    let L = up(0.9 * CGA::e2()) ^ up(0.9 * CGA::e2() - CGA::e1()) ^ ni();
    // bottom dual plane
    let P = !&(CGA::e2() - 0.9 * ni());
    // right dual plane
    let P2 = !&(CGA::e1() + 1.7 * ni());

    // The intersections of the big sphere with the other 4 objects.
    //   var C1 = ()=>S&P, C2 = ()=>S&L, C3 = ()=>S&S2, C4 = ()=>S&C, C5 = ()=>C&P2;
    let C1 = &S & &P;
    let C2 = &S & &L;
    let C3 = &S & &S2;
    let C4 = &S & &C;
    let C5 = &S & &P2;

    // For line meet plane its a bit more involved.
    //   var lp = up(nino<<(P2&L^no));
    let nino = ni() ^ no();
    let lp = up(nino | (&P2 & &L ^ no()));

    // Graph the items. (hex numbers are html5 colors, two extra first bytes = alpha)
    //   document.body.appendChild(this.graph([
    //       0x00FF0000, p, "s1",             // point
    //       0xFF00FF,lp,"l&p",               // line intersect plane
    //       0x0000FF,C1,"s&p",               // sphere meet plane
    //       0x888800,C2,"s&l",               // sphere meet line
    //       0x0088FF,C3,"s&s",               // sphere meet sphere
    //       0x008800,C4,"s&c",               // sphere meet circle
    //       0x880000,C5,"c&p",               // circle meet sphere
    //       0,L,0,C,                         // line and circle
    //       0xE0008800, P, P2,               // plane
    //       0xE0FFFFFF, S, "s1", S2          // spheres
    //   ],{conformal:true,gl:true,grid:true}));
    // });
    let mut graph = ganja_graph::GanjaGraph {
        title: "cga3d_intersections",
        p: 4,
        q: 1,
        r: 0,
        ..Default::default()
    };
    graph.add_bivector(p, 0x00FF0000);
    graph.add_bivector(lp, 0xFF00FF);
    graph.add_bivector(C1, 0x0000FF);
    graph.add_bivector(C2, 0x888800);
    graph.add_bivector(C3, 0x0088FF);
    graph.add_bivector(C4, 0x008800);
    graph.add_bivector(C5, 0x880000);
    graph.add_bivector(L, 0);
    graph.add_bivector(C, 0);
    graph.add_bivector(P, 0xE0008800);
    graph.add_bivector(P2, 0xaa000000);
    graph.add_bivector(S, 0xE0FFFFFF);
    graph.add_bivector(S2, 0xaa000000);

    let fnout = "cga3d_intersections.html";
    println!("Writing {}", fnout);
    let mut fout = BufWriter::new(File::create(fnout)?);
    graph.graph_html(&mut fout)
}
