use std::io::stdout;

use crossterm::{
    terminal,
    cursor,
    style,
    event,
    execute,
};

//структура позиции курсора
struct Position {
    x: u16,
    y: u16,
}

//структура корабля с положением, скоростью и здоровьем
struct Ship {
    position: Position,
    speed: u8,
    health: u16,
}

// добавляем методы для корабля
impl Ship {
    fn new(position: Position, speed: u8, health: u16) -> Self {
        Ship { position, speed, health }
    }

    fn draw_ship(&self) {
        let (x, y) = (self.position.x, self.position.y);

        // Очищаем экран
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

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

    fn terminal_border_control(&self) -> bool {
        let (width, height) = terminal::size().unwrap();
        let (x, y) = (self.position.x, self.position.y);
        x > 1 && x < width - 1 && y > 1 && y < height - 1
    }


    
}

//функция инициализации игры
fn init_game() -> Ship {
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
    execute!(stdout(), cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}











fn main() {
    
}
