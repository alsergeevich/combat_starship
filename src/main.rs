use std::io::stdout;
use rand::Rng;
use crossterm::{
    cursor::{self}, event::{self, Event, KeyCode}, execute, style::{self, Color}, terminal,
};
use crossterm::style::Color::*;
use crossterm::style::SetForegroundColor;
use crossterm::style::ResetColor;

//структура позиции курсора
struct Position {
    x: u16,
    y: u16,
}

//структура описывающая звезду
struct Star {
    position: Position,
    speed: u16,
    color: Color,
}

struct Asteroid {
    position: Position,
    speed: u16,
    color : Color,
}

impl Asteroid {
    fn new (position: Position, speed: u16, color: Color) -> Self {
        Asteroid { position, speed, color }
    }
}

impl Star {
    //конструктор звезды
    fn new (position: Position, speed: u16, color: Color) -> Self {
        Star { position, speed, color }
    }
    
}

//структура корабля с положением, скоростью и здоровьем
struct Ship {
    position: Position,
    speed: u16,
    health: u16,
    color : Color,
}

// добавляем методы для корабля
impl Ship {
    //конструктор
    fn new(position: Position, speed: u16, health: u16, color : Color) -> Self {
        Ship { position, speed, health, color }
    }
    //рисуем корабль
    fn draw_ship(&self) {
        let (x, y) = (self.position.x, self.position.y);

        // Рисуем форму корабля
        execute!(
            stdout(),
            SetForegroundColor(self.color),
            cursor::MoveTo(x + 1, y - 2),
            style::Print("+"),
            ResetColor,
        ).unwrap();

        execute!(
            stdout(),
            SetForegroundColor(self.color),
            cursor::MoveTo(x + 1, y - 1),
            style::Print("++"),
            ResetColor,
        ).unwrap();

        execute!(
            stdout(),
            SetForegroundColor(self.color),
            cursor::MoveTo(x + 1, y),
            style::Print("+++++"),
            ResetColor,
        ).unwrap();

        execute!(
            stdout(),
            SetForegroundColor(self.color),
            cursor::MoveTo(x + 1, y + 1),
            style::Print("++"),
            ResetColor,
        ).unwrap();

        execute!(
            stdout(),
            SetForegroundColor(self.color),
            cursor::MoveTo(x + 1, y + 2),
            style::Print("+"),
            ResetColor,
        ).unwrap();
    }

    
    //функции движения корабля
    fn move_right(&mut self) {
        let (width, _) = terminal::size().unwrap();
        if self.position.x + self.speed + 5 <= width - 1 { // Учитываем размер корабля при контроле границы
            self.position.x += self.speed;
        }
    }
    fn move_left(&mut self) {
        if self.position.x >= self.speed {
            self.position.x -= self.speed;
        }
    }
    fn move_up(&mut self) {
        if (self.position.y - 2) > self.speed { // Учитываем размер корабля при контроле границы
            self.position.y -= self.speed;
        }
    }
    fn move_down(&mut self) {
        let (_, height) = terminal::size().unwrap();
        if (self.position.y + 2) + self.speed <= height { 
            self.position.y += self.speed;
        }
    }
    
}

//функция для обработки ввода с клавиатуры и управления кораблём
fn input_processing(ship: &mut Ship) {
    let dur = std::time::Duration::from_millis(5); //задержка в мс
    if event::poll(dur).unwrap() { //проверяем было ли событие
        let ev = event::read().unwrap(); //провеяем событие и получаем его
    match ev {
        Event::Key(key) => {match key.code {  //обрабатываем событие клавиатуры
            KeyCode::Up => ship.move_up(),
            KeyCode::Down => ship.move_down(),
            KeyCode::Right => ship.move_right(),
            KeyCode::Left => ship.move_left(),
            KeyCode::Esc => finish_game(),
            _ => ()
        }
        
    }
    _ => () }
    } else {
        return;
    }  
}

fn create_one_star(colors: &Vec<Color>) -> Star {
    let mut rng = rand::rng();
    let rnd_color = rng.random_range(0..colors.len());
    let x = rng.random_range(0..terminal::size().unwrap().0);
    let y = rng.random_range(0..terminal::size().unwrap().1);
    let speed = rng.random_range(1..10);
    let star = Star::new(Position{x, y}, speed, colors[rnd_color]);
    star
}

//функция создания звезды
fn create_stars(count: u16, colors: &Vec<Color>) -> Vec<Star> {
    let mut stars = Vec::new();
    for _ in 0..count {
        stars.push(create_one_star(&colors));
    }
    stars
}

//функция отрисовки звёзд на экране
fn draw_stars(stars: &Vec<Star>) {
    for star in stars {
        execute!(
            stdout(),
            SetForegroundColor(star.color),
            cursor::MoveTo(star.position.x, star.position.y),
            style::Print("."),
            ResetColor,
        ).unwrap();
    }
}



//функция обновления координат звёзд для создания эффекта движения корабля
fn update_stars(stars: &mut Vec<Star>) {
    
    let mut rng = rand::rng();
    for star in stars {
        if star.position.x <= 0 || star.speed > star.position.x {
            star.position.x = terminal::size().unwrap().0;
            star.position.y = rng.random_range(0..terminal::size().unwrap().1);
        } else {
            star.position.x -= star.speed;
        }
    }
}

fn create_one_asteroid(colors: &Vec<Color>) -> Asteroid {
    let mut rng = rand::rng();
    let rnd_num = rng.random_range(0..5);
    let rnd_color = rng.random_range(0..colors.len());
    let x = terminal::size().unwrap().0;
    let y = rng.random_range(rnd_num..terminal::size().unwrap().1);
    let speed = rng.random_range(1..3);
    let asteroid = Asteroid::new(Position{x, y}, speed, colors[rnd_color]);
    asteroid
}

fn create_asteroids(count: u16, colors: &Vec<Color>) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    for _ in 0..count {
        asteroids.push(create_one_asteroid(&colors));
    }
    asteroids
}

fn draw_asteroids(asteroids: &Vec<Asteroid>) {
    for asteroid in asteroids {
        execute!(
            stdout(),
            SetForegroundColor(asteroid.color),
            cursor::MoveTo(asteroid.position.x, asteroid.position.y),
            style::Print("@"),
            ResetColor,
        ).unwrap();
    }
}

fn update_asteroids(asteroids: &mut Vec<Asteroid>) {
    let mut rng = rand::rng();
    let rnd_num = rng.random_range(0..5);
    for asteroid in asteroids {
        if asteroid.position.x <= 0 || asteroid.speed > asteroid.position.x {
            asteroid.position.x = terminal::size().unwrap().0;
            asteroid.position.y = rng.random_range(rnd_num..terminal::size().unwrap().1);
            asteroid.speed = rng.random_range(1..3);
        } else {
            asteroid.position.x -= asteroid.speed;
        }
    }
}

fn check_collision(ship: &mut Ship, asteroids: &mut Vec<Asteroid>) {
    let mut rng = rand::rng();
    let rnd_num = rng.random_range(0..5);
    for asteroid in asteroids {
        if (ship.position.x + 4 == asteroid.position.x && ship.position.y == asteroid.position.y) || 
        (ship.position.x == asteroid.position.x && ship.position.y - 2  == asteroid.position.y) || 
        (ship.position.x == asteroid.position.x && ship.position.y + 2  == asteroid.position.y) || 
        (ship.position.x + 2== asteroid.position.x && ship.position.y - 1  == asteroid.position.y) || 
        (ship.position.x + 2 == asteroid.position.x && ship.position.y + 1  == asteroid.position.y) {
            if ship.health > 1 {
                ship.health -= 1;
                asteroid.position.x = terminal::size().unwrap().0;
                asteroid.position.y = rng.random_range(rnd_num..terminal::size().unwrap().1);
                asteroid.speed = rng.random_range(1..3);
            } else {
                finish_game();
            }

        }
    }
}

//функция инициализации игры
fn init_game() -> Ship {
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    let (width, height) = terminal::size().unwrap();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    let ship = Ship::new(Position { x: width-width, y: height / 2 }, 2, 10, White); //создание корабля
    ship.draw_ship();
    ship    
}

//функция завершения игры
fn finish_game() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout(), cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}




fn main() {
    let colors_stars = vec![Yellow, White];
    let colors_asteroid = vec![Green, Blue, DarkYellow, White];
    let dur = std::time::Duration::from_millis(30);
    let mut vec_stars = create_stars(150, &colors_stars);
    let mut vec_asteroids = create_asteroids(10, &colors_asteroid);
    let mut ship = init_game();
    draw_stars(&vec_stars); // отрисовка звёзд на экране
    loop {
        // Очищаем экран
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        input_processing(&mut ship);
        update_stars(&mut vec_stars);
        update_asteroids(&mut vec_asteroids);
        check_collision(&mut ship, &mut vec_asteroids);
        draw_stars(&vec_stars);
        draw_asteroids(&vec_asteroids);
        ship.draw_ship();
        
        execute!(
            stdout(),
            cursor::MoveTo(0, terminal::size().unwrap().1 - 1), // Последняя строка экрана
            style::Print(format!("Health: {}", ship.health))
        ).unwrap();
        
        std::thread::sleep(dur);
        
    }
}
