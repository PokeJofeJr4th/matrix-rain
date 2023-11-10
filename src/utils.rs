use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    cursor::MoveTo,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct AlternateBuffer {
    points: Vec<(u16, u16)>,
    stdout: Stdout,
    rng: ThreadRng,
    term_size: (u16, u16),
}

const COLORS: [Color; 10] = [
    Color::Rgb {
        r: 100,
        g: 250,
        b: 100,
    },
    Color::Rgb {
        r: 90,
        g: 225,
        b: 90,
    },
    Color::Rgb {
        r: 80,
        g: 200,
        b: 80,
    },
    Color::Rgb {
        r: 70,
        g: 175,
        b: 70,
    },
    Color::Rgb {
        r: 60,
        g: 150,
        b: 60,
    },
    Color::Rgb {
        r: 50,
        g: 125,
        b: 50,
    },
    Color::Rgb {
        r: 40,
        g: 100,
        b: 40,
    },
    Color::Rgb {
        r: 30,
        g: 75,
        b: 30,
    },
    Color::Rgb {
        r: 20,
        g: 50,
        b: 20,
    },
    Color::Rgb {
        r: 10,
        g: 25,
        b: 10,
    },
];

const N: u16 = 8;
const N2: u16 = N * 2;

impl ExecutableCommand for AlternateBuffer {
    fn execute(&mut self, command: impl crossterm::Command) -> std::io::Result<&mut Self> {
        self.stdout.execute(command)?;
        Ok(self)
    }
}

impl AlternateBuffer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut handler = Self {
            points: Vec::new(),
            stdout: stdout(),
            rng: thread_rng(),
            term_size: terminal::size()?,
        };
        handler.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(handler)
    }

    pub fn resize(&mut self) -> Result<(), Box<dyn Error>> {
        self.term_size = terminal::size()?;
        self.points = core::mem::take(&mut self.points)
            .into_iter()
            .filter(|&(x, y)| x < self.term_size.0 && y < self.term_size.1 + 10)
            .collect();
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        for (x, y) in &mut self.points {
            if *y > self.term_size.1 + N2 {
                *y = 0;
                *x = self.rng.gen_range(0..self.term_size.0);
            }
            *y += 1;
            for y_prime in 0..N {
                let Some(real_y) = y.checked_sub(y_prime * 2) else { continue };
                if real_y > self.term_size.1 {
                    continue;
                }
                self.stdout
                    .execute(MoveTo(*x, real_y))?
                    .execute(PrintStyledContent(
                        {
                            if self.rng.gen_bool(0.5) {
                                '1'
                            } else {
                                '0'
                            }
                        }
                        .with(COLORS[y_prime as usize]),
                    ))?;
            }
        }
        if self.points.len() < self.term_size.0 as usize * 2 {
            self.points
                .push((self.rng.gen_range(0..self.term_size.0), 0));
        }
        Ok(())
    }
}

impl Drop for AlternateBuffer {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
    }
}
