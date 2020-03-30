use ganja_rs::pga2d::*;
use std::f64::consts::PI;

// https://enkimute.github.io/ganja.js/examples/coffeeshop.html#pga2d_separating_axis

#[allow(dead_code)]
fn dist_pl(p: PGA2D, l: PGA2D) -> f64 {
    (p.normalized() ^ l.normalized()).e012()
}

pub fn rotor(point: PGA2D, angle: f64) -> PGA2D {
    (angle / 2.0).cos() + (angle / 2.0).sin() * point.normalized()
}

fn ngon(p: PGA2D, n: usize, d: f64, a: f64) -> Vec<PGA2D> {
    // [...Array(n)].map((x,i)=>rotor(p,(a||0)+2*Math.PI*i/n)>>>(p+d*1e01));
    (0..n)
        .map(|i| rotor(p, a + 2.0 * PI * i as f64 / n as f64).sw(p + d * e01))
        .collect::<Vec<_>>()
}

fn main() {
    // // in 2D, the seperating axis test involves testing overlap of the projection
    // // onto one of the possible seperating  axi. we support all regular ngons and
    // // demonstrate intersection testing.

    // // Some identities we'll use.
    // var point      = (x,y)=>1e12-x*1e02+y*1e01,
    //   dist_pl    = (P,l)=>(P.Normalized^l.Normalized).e012,
    //   rotor      = (point,angle)=>Math.cos(angle/2) + Math.sin(angle/2)*point.Normalized,
    //   ngon       = (p,n,d,a)=>[...Array(n)].map((x,i)=>rotor(p,(a||0)+2*Math.PI*i/n)>>>(p+d*1e01));

    // // the shapes can be moved around.
    // var pa=point(0,-0.2), pb=point(-0.8,-1);

    // var sat=(a,b)=>{
    // // Collect potential axis candidates (all poly edges or half of them for symmetric polys)
    // var e=[], da=[], db=[], i, al=a.length, bl=b.length;
    // for (i=0; i<(al%2?al:al/2); i++) e[i]=a[i]&a[(i+1)%al];
    // for (i=0; i<(bl%2?bl:bl/2); i++) e[i+al]=b[i]&b[(i+1)%bl];
    // // Testing a single axis
    // var check=(axis)=>{
    //   for (var i=0; i<al; i++) da[i]=dist_pl(a[i],axis);
    //   for (var i=0; i<bl; i++) db[i]=dist_pl(b[i],axis);
    //   return (Math.min.apply(Math,da)>Math.max.apply(Math,db))||(Math.min.apply(Math,db)>Math.max.apply(Math,da));
    // }
    // // Test all edges. Return the separating axis if found.
    // for (i=0; i<e.length; i++) if (check(e[i])) return e[i];
    // }

    // document.body.appendChild(this.graph(()=>{
    // pa[4] = -0.2+(0.3-(performance.now()*0.0001%2));
    // var a=ngon(pa,3,0.65,pa.e01*10), b=ngon(pb,6,0.5), h=sat(b,a);
    // return [0xcccccc,h,h?0x88ff88:0xffaaaa,a,b,0x444444,pa,pb]},{animate:true, grid:true}));

    let pa = PGA2D::point(0.0, -0.2);
    let pb = PGA2D::point(-0.8, -1.0);

    let triangle = ngon(pa, 3, 0.65, pa[2] * 10.0);
    let hexagon = ngon(pb, 6, 0.5, 0.0);
    println!(
        "triangle: [{}]",
        triangle
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "hexagon: [{}]",
        hexagon
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
