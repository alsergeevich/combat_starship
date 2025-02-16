use rodio::{Decoder, OutputStream, source::Source};
use std::io::Cursor;
use std::io::stdout;
use rand::Rng;
use crossterm::{
    cursor::{self}, event::{self, Event, KeyCode}, execute, style::{self, Color}, terminal,
};
use crossterm::style::Color::*;
use crossterm::style::SetForegroundColor;
use crossterm::style::ResetColor;

static BACKGROUND_MUSIC: &[u8] = include_bytes!("..\\assets\\fone.wav");

// static BACKGROUND_MUSIC: &[u8] = include_bytes!("F:\\Sanek\\learn_programming\\rust\\combat_starship\\assets\\fone.wav");
// static SHOOT_SOUND: &[u8] = include_bytes!("F:\\Sanek\\learn_programming\\rust\\combat_starship\\assets\\shoot.wav");
// static COLLISION_SOUND: &[u8] = include_bytes!("F:\\Sanek\\learn_programming\\rust\\combat_starship\\assets\\collision.wav");
// static HIT_SOUND: &[u8] = include_bytes!("F:\\Sanek\\learn_programming\\rust\\combat_starship\\assets\\kill_asteroid.wav");



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

struct Bullet {
    position: Position,
    speed: u16,
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

impl Bullet {
    fn new (position: Position, speed: u16) -> Self {
        Bullet { position, speed }
    }
}

//структура корабля с положением, скоростью и здоровьем
struct Ship {
    position: Position,
    speed: u16,
    health: u16,
    color : Color,
    bullets: Vec<Bullet>,
    
    
}

// добавляем методы для корабля
impl Ship {
    //конструктор
    fn new(position: Position, speed: u16, health: u16, color : Color, bullets: Vec<Bullet>) -> Self {
        Ship { position, speed, health, color, bullets }
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

    fn shooting(&mut self) {
        let x = self.position.x + 5;
        let y = self.position.y;
        let bullet = Bullet::new(Position{x, y}, 5);
        self.bullets.push(bullet);
        
        
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
            KeyCode::Char(' ') => ship.shooting(),
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

//функция создания одного астероида
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

//функция создания вектора астероидов
fn create_asteroids(count: u16, colors: &Vec<Color>) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();
    for _ in 0..count {
        asteroids.push(create_one_asteroid(&colors));
    }
    asteroids
}

//функция отрисовки астероидов
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

//функция обновления позиции астероида
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

//функция проверки столкновения корабля с астероидом
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

//обновляем положение пули
fn update_bullets(bullets: &mut Vec<Bullet>) {
    let width = terminal::size().unwrap().0;
    let len_vec = bullets.len();
    if len_vec == 0 {
        return;
    }
    for bullet in bullets.iter_mut() {
        bullet.position.x += bullet.speed;   
    }
    bullets.retain(|bullet|{bullet.position.x < width});
    
}

//отрисовываем пулю
fn draw_bullets(bullets: &Vec<Bullet>) {
    for bullet in bullets {
        execute!(
            stdout(),
            SetForegroundColor(Yellow),
            cursor::MoveTo(bullet.position.x, bullet.position.y),
            style::Print("X=>"),
            ResetColor,
        ).unwrap();
    }
}

//функция проверки попадания пуль в астероид
fn check_destroy_an_asteroid(bullets: &mut Vec<Bullet>, asteroids: &mut Vec<Asteroid>, score: &mut u32) {
    let mut rng = rand::rng();
    let rnd_num = rng.random_range(0..5);
    
    if bullets.len() == 0 {
        return;
    }
    let mut bullets_to_remove = Vec::new();
    for (index, bullet) in bullets.iter().enumerate() {
        for asteroid in asteroids.iter_mut() {
            if (bullet.position.x as i16 - asteroid.position.x as i16).abs() <= 1 &&
                (bullet.position.y as i16 - asteroid.position.y as i16).abs() <= 1 {
                bullets_to_remove.push(index); // Запоминаем индекс для удаления
                asteroid.position.x = terminal::size().unwrap().0;
                asteroid.position.y = rng.random_range(rnd_num..terminal::size().unwrap().1);
                asteroid.speed = rng.random_range(1..3);
                *score += 1;
                
            }
            
        }
}

    // Удаляем пули после завершения проверки
    for index in bullets_to_remove.into_iter().rev() { // Удаляем в обратном порядке
        bullets.remove(index);
    }
}

//функция инициализации игры
fn init_game() -> Ship {

    execute!(stdout(), terminal::EnterAlternateScreen).unwrap(); //переключаем терминал в альтернативный режим
    let (width, height) = terminal::size().unwrap(); //получаем размеры терминала
    terminal::enable_raw_mode().unwrap(); //включаем сырой режим терминала
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap(); //очищаем терминал
    execute!(stdout(), cursor::Hide).unwrap(); //выключаем курсор
    

    // Создаем корабль
    let ship = Ship::new(Position { x: width-width, y: height / 2 }, 2, 10, White, Vec::new()); //создание корабля
    ship.draw_ship(); //рисуем корабль
    ship    //возвращаем созданный экземпляр корабля и аудио потока
}

//функция завершения игры
fn finish_game() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap(); //выключаем альтернативный режим
    execute!(stdout(), cursor::Show).unwrap(); //показываем курсор
    terminal::disable_raw_mode().unwrap(); //выключаем сырой режим
    
    std::process::exit(0); //завершаем программу
}




fn main() {
    let mut score = 0;
    let colors_stars = vec![Yellow, White];
    let colors_asteroid = vec![Green, Blue, DarkYellow, White];
    let dur = std::time::Duration::from_millis(30);
    let mut vec_stars = create_stars(100, &colors_stars);
    let mut vec_asteroids = create_asteroids(10, &colors_asteroid);
    let mut ship = init_game();

    
    draw_stars(&vec_stars); // отрисовка звёзд на экране
    loop {


        
        // Очищаем экран
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        input_processing(&mut ship);
        update_stars(&mut vec_stars);
        update_asteroids(&mut vec_asteroids);
        update_bullets(&mut ship.bullets);
        check_collision(&mut ship, &mut vec_asteroids);
        check_destroy_an_asteroid(&mut ship.bullets, &mut vec_asteroids, &mut score);
        draw_stars(&vec_stars);
        draw_asteroids(&vec_asteroids);
        draw_bullets(&ship.bullets); 
        ship.draw_ship();
        
        execute!(
            stdout(),
            cursor::MoveTo(0, terminal::size().unwrap().1 - 1), // Последняя строка экрана
            style::Print(format!("Health: {} | Score: {}", ship.health, score))
        ).unwrap();
        
        std::thread::sleep(dur);
        
    }
}
