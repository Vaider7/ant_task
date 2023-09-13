// HashSet выглядит логичнее c точки зрения производительности,
// но Vec занимает меньше места в памяти, чем HashSet
struct BlackCells(Vec<(u16, u16)>);

impl BlackCells {
    fn toggle_cell(&mut self, color: &Color, ant: &Ant) {
        if let Color::White = color {
            self.0.push((ant.x, ant.y))
        } else {
            for (idx, cell) in self.0.iter().enumerate() {
                if cell == &(ant.x, ant.y)  {
                    self.0.swap_remove(idx);
                    break;
                }
            }
        }
    }
    fn new() -> Self {
        Self(vec![])
    }
}

struct Ant {
    x: u16,
    y: u16,
}

impl Ant {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    fn move_ant(&mut self, head_state: &Head) {
        match head_state {
            Head::Up => self.y += 1,
            Head::Right => self.x += 1,
            Head::Down => self.y -= 1,
            Head::Left => self.x -= 1,
        }
    }
    fn check_end(&self) -> bool {
        self.x == 0 || self.y == 0 || self.x == 1024 || self.y == 1024
    }
}

#[derive(Clone, Copy)]
enum Head {
    Up,
    Right,
    Down,
    Left
}

enum Color {
    Black,
    White,
}

impl Color {
    fn next_color(&mut self, x: &u16, y: &u16, black_cells: &BlackCells) {
        if black_cells.0.contains(&(*x, *y)) {
            *self = Color::Black
        } else {
            *self = Color::White
        }
    }
}

impl Head {
    fn turn(&mut self, color: &Color) {
        let mut i = *self as i8;
        if let Color::White = color {
            i += 1;
        } else {
            i -= 1;
        }
        match i {
            -1 => *self = Self::Left,
            0 => *self = Self::Up,
            1 => *self = Self::Right,
            2 => *self = Self::Down,
            3 => *self = Self::Left,
            4 => *self = Self::Up,
            _ => unreachable!()
        }
    }
}

fn make_png(black_cells: BlackCells) {
    let mut imgbuf = image::ImageBuffer::new(1024, 1024);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if black_cells.0.contains(&(x as u16, y as u16)) {
            *pixel = image::Luma([0u8]);
        } else {
            *pixel = image::Luma([255u8]);
        }
    }
    imgbuf.save("ant.png").expect("Saving image");
    println!("Image saved");
}

fn main() {
    let mut ant = Ant::new(512, 512);    
    let mut head_state = Head::Up;
    let mut color = Color::White;
    let mut black_cells = BlackCells::new();  

    loop {
        head_state.turn(&color);
        black_cells.toggle_cell(&color, &ant);
        ant.move_ant(&head_state);
        color.next_color(&ant.x, &ant.y, &black_cells);
        if ant.check_end() {
            println!("Black cells count: {}", black_cells.0.len());
            // Без создания png файла алгоритм требует 36Кб памяти
            make_png(black_cells);
            break;
        }
    }
}
