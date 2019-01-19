extern crate dataplotlib;
use dataplotlib::util::{linspace, zip2};
use dataplotlib::plotbuilder::PlotBuilder2D;
use dataplotlib::plotter::Plotter;
use dataplotlib::draw_sdl::DrawSDL;

use std::{thread, time};

fn main() {
    let sdlh = dataplotlib::sdl2_init();
    let sdl2_window = DrawSDL::new(sdlh);

    let mut plt = Plotter::new();
    let sender = plt.plot2d_channel(sdl2_window);

    let mut t = 0.0;
    loop {
        let x = linspace(0, 10, 100);

        let y_sin = x.iter().map(|x| (t + x).sin()).collect();
        let xy_sin = zip2(&x, &y_sin);

        // Creates a new plot builder
        let mut pb = PlotBuilder2D::new();

        // Adds the sin plot and the linear plot with custom colors
        pb.add_color_xy(xy_sin, [1.0, 0.0, 0.0, 1.0]);

        if let Err(_) = sender.send(pb) {
            break;
        }

        t += 0.1;
        thread::sleep(time::Duration::from_millis(32));
    }
}
