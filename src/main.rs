use std::io::stdout;
use rand::Rng;

use crossterm::{
    cursor::{self}, event::{self, Event, KeyCode}, execute, style, terminal
};

//структура позиции курсора
struct Position {
    x: u16,
    y: u16,
}

//структура описывающая звезду
struct Star {
    position: Position,
    speed: u16,
}

impl Star {
    //конструктор звезды
    fn new (position: Position, speed: u16) -> Self {
        Star { position, speed }
    }
    
}

//структура корабля с положением, скоростью и здоровьем
struct Ship {
    position: Position,
    speed: u16,
    health: u16,
}

// добавляем методы для корабля
impl Ship {
    //конструктор
    fn new(position: Position, speed: u16, health: u16) -> Self {
        Ship { position, speed, health }
    }
    //рисуем корабль
    fn draw_ship(&self) {
        let (x, y) = (self.position.x, self.position.y);

        // Рисуем форму корабля
        execute!(
            stdout(),
            cursor::MoveTo(x + 1, y - 2),
            style::Print("+"),
        ).unwrap();

        execute!(
            stdout(),
            cursor::MoveTo(x + 1, y - 1),
            style::Print("++"),
        ).unwrap();

        execute!(
            stdout(),
            cursor::MoveTo(x + 1, y),
            style::Print("+++++"),
        ).unwrap();

        execute!(
            stdout(),
            cursor::MoveTo(x + 1, y + 1),
            style::Print("++"),
        ).unwrap();

        execute!(
            stdout(),
            cursor::MoveTo(x + 1, y + 2),
            style::Print("+"),
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

fn create_one_star() -> Star {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..terminal::size().unwrap().0);
    let y = rng.gen_range(0..terminal::size().unwrap().1);
    let speed = rng.gen_range(1..10);
    let speed = rng.gen_range(1..10);
    let star = Star::new(Position{x, y}, speed);
    star
}

//функция создания звезды
fn create_stars(count: u16) -> Vec<Star> {
    let mut stars = Vec::new();
    for _ in 0..count {
        stars.push(create_one_star());
    }
    stars
}

//функция отрисовки звёзд на экране
fn draw_stars(stars: &Vec<Star>) {
    for star in stars {
        execute!(
            stdout(),
            cursor::MoveTo(star.position.x, star.position.y),
            style::Print("."),
        ).unwrap();
    }
}

//функция обновления координат звёзд для создания эффекта движения корабля
fn update_stars(stars: &mut Vec<Star>) {
    // Очищаем экран
    let mut rng = rand::thread_rng();
    for star in stars {
        if star.position.x <= 0 || star.speed > star.position.x {
            star.position.x = terminal::size().unwrap().0;
            star.position.y = rng.gen_range(0..terminal::size().unwrap().1);
        } else {
            star.position.x -= star.speed;
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
    let ship = Ship::new(Position { x: width-width, y: height / 2 }, 5, 100);
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
    let dur = std::time::Duration::from_millis(33);
    let mut vec_stars = create_stars(150);
    let mut ship = init_game();
    draw_stars(&vec_stars); // отрисовка звёзд на экране
    loop {
        // Очищаем экран
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        input_processing(&mut ship);
        update_stars(&mut vec_stars);
        draw_stars(&vec_stars);
        ship.draw_ship();
        // Отладочный вывод
        // execute!(
        //     stdout(),
        //     cursor::MoveTo(0, terminal::size().unwrap().1 - 1), // Последняя строка экрана
        //     style::Print(format!("Debug: x={}, y={}", ship.position.x, ship.position.y))
        // ).unwrap();
        
        std::thread::sleep(dur);
        
    }
}
