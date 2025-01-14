use std::io::{self, Write};
use std::{thread, time};
use std::time::Instant;

pub struct Sausage3D {
    a: f64,
    b: f64,
    width: usize,
    height: usize,
}

impl Sausage3D {
    pub fn new() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            width: 35,
            height: 35,
        }
    }

    pub fn render(&mut self) {
        let mut output = vec![vec![' '; self.width]; self.height];
        let mut zbuffer = vec![vec![0.0; self.width]; self.height];
        let shade_chars = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
        
        let r1 = 1.0;     // Tube radius
        let r2 = 2.0;     // Donut radius
        let k2 = 5.0;     // Distance from viewer
        let k1 = self.width as f64 * k2 * 3.0 / (8.0 * (r1 + r2));

        for theta in (0..628).step_by(10) {
            let theta_f = theta as f64 / 100.0;
            for phi in (0..628).step_by(5) {
                let phi_f = phi as f64 / 100.0;

                let (sin_a, cos_a) = self.a.sin_cos();
                let (sin_b, cos_b) = self.b.sin_cos();
                let (sin_t, cos_t) = theta_f.sin_cos();
                let (sin_p, cos_p) = phi_f.sin_cos();

                let x = r2 * cos_t + r1 * cos_t * cos_p;
                let y = r2 * sin_t + r1 * sin_t * cos_p;
                let z = r1 * sin_p;

                let ooz = 1.0 / (k2 + cos_b * x + sin_b * y);

                let xp = (self.width as f64 / 2.0 + 2.0 * k1 * ooz * 
                    (cos_b * x - sin_b * y)) as usize;
                let yp = (self.height as f64 / 2.0 + k1 * ooz * 
                    (sin_a * z + cos_a * (sin_b * x + cos_b * y))) as usize;

                let l = cos_p * (sin_b * sin_t - sin_a * cos_b * cos_t) - 
                    cos_a * sin_b * sin_p + cos_b * cos_t * sin_a;

                if l > 0.0 && 
                   xp < self.width && 
                   yp < self.height && 
                   ooz > zbuffer[yp][xp] {
                    zbuffer[yp][xp] = ooz;
                    let luminance = (l * 8.0) as usize;
                    output[yp][xp] = shade_chars[luminance.min(11)];
                }
            }
        }

        print!("\x1b[H");
        for row in output {
            println!("{}", row.iter().collect::<String>());
        }
        io::stdout().flush().unwrap();
    }

    pub fn spin(&mut self) {
        print!("\x1b[2J\x1b[?25l");
        let start_time = Instant::now();
        let duration = time::Duration::from_secs(5);

        while start_time.elapsed() < duration {
            self.render();
            self.a += 0.07;
            self.b += 0.03;
            thread::sleep(time::Duration::from_millis(30));
        }
        print!("\x1b[?25h");
        println!("\nDonut spin complete!");
    }
}
