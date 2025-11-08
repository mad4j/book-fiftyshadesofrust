use std::{collections::HashSet, io::{stdout, Write}, time::{Duration, Instant}, thread};
use crossterm::{
    cursor, event::{self, Event, KeyCode}, execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;

const W: usize = 20;
const H: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

fn draw(stdout: &mut impl Write, crab: Pos, rocks: &HashSet<Pos>, pearls: &HashSet<Pos>, score: u32) {
    execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).unwrap();
    println!("Score: {}", score);
    for y in 0..H as i32 {
        for x in 0..W as i32 {
            let p = Pos(x, y);
            if p == crab {
                print!("🦀");
            } else if rocks.contains(&p) {
                print!("🪨");
            } else if pearls.contains(&p) {
                print!("⚪");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    stdout.flush().unwrap();
}

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let mut rng = rand::thread_rng();
    let mut crab = Pos(W as i32 / 2, H as i32 - 1);
    let mut rocks: HashSet<Pos> = HashSet::new();
    let mut pearls: HashSet<Pos> = HashSet::new();
    let mut score = 0;
    let mut last_update = Instant::now();
    let mut speed = Duration::from_millis(400);

    'game: loop {
        if last_update.elapsed() > speed {
            // move rocks and pearls down
            rocks = rocks.iter().map(|&Pos(x,y)| Pos(x, y+1)).filter(|p| p.1 < H as i32).collect();
            pearls = pearls.iter().map(|&Pos(x,y)| Pos(x, y+1)).filter(|p| p.1 < H as i32).collect();

            // spawn new items
            if rng.gen_bool(0.3) { rocks.insert(Pos(rng.gen_range(0..W as i32), 0)); }
            if rng.gen_bool(0.2) { pearls.insert(Pos(rng.gen_range(0..W as i32), 0)); }

            // collision check
            if rocks.contains(&crab) { break 'game; }
            if pearls.remove(&crab) { score += 10; speed = speed.saturating_sub(Duration::from_millis(10)); }

            last_update = Instant::now();
        }

        draw(&mut stdout, crab, &rocks, &pearls, score);

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') => break 'game,
                    KeyCode::Left if crab.0 > 0 => crab.0 -= 1,
                    KeyCode::Right if crab.0 < W as i32 - 1 => crab.0 += 1,
                    _ => {}
                }
            }
        }
        thread::sleep(Duration::from_millis(20));
    }

    execute!(stdout, cursor::MoveTo(0, H as u16 + 1))?;
    println!("\n💥 Game Over! Final score: {} 🦀", score);
    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
