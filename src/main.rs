use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, self, KeyCode};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen, disable_raw_mode};
use tui::backend::{CrosstermBackend, Backend};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Widget, BorderType, Paragraph };
use tui::layout::{Constraint, Direction, Layout, Rect, Alignment};
use tui::{Terminal, Frame};
use rand::{Rng};
use std::usize;

struct Game {
    start: bool,
    die: bool,
    board: Board,
}
impl Game {
    fn new() -> Self {
        Self {
            start: (true),
            die: (false),
            board: Board::new(),
        }
    }
}
struct Board {
    matrix: [[Square; 4]; 4],
}
impl Board {
    fn new() -> Self {
        let mut board = Board {
            matrix: [[Square::new(); 4]; 4],
        };

        board.set_rnd_avaible_square();
        board.set_rnd_avaible_square();
        board
    }
    fn prepare_board(&mut self) {
        for i in 0..self.matrix.len() {
            for z in 0..self.matrix[i].len() {
                self.matrix[i][z].value = 0;
            }
        }
    }

    fn is_board_full(&self) -> bool {
        for i in 0..self.matrix.len() {
            for z in 0..self.matrix[i].len() {
                if self.matrix[i][z].value == 0 {
                    return false;
                }
            }
        }
        true
    }
    fn is_square_avaible(&self, x: usize, y: usize) -> bool {
        if self.matrix[x][y].value == 0 {
            true
        } else {
            false
        }
    }
    fn set_rnd_avaible_square(&mut self) -> bool {
        if self.is_board_full() {
            false
        } else {
            let mut rng = rand::thread_rng();
            let mut y: usize = rng.gen_range(0..4);
            let mut x: usize = rng.gen_range(0..4);
            while !self.is_square_avaible(x, y) {
                x = rng.gen_range(0..4);
                y = rng.gen_range(0..4);
            }
            self.matrix[x][y].decide_number();
            true
        }
    }
    fn print_board(&self) {
        for i in 0..self.matrix.len() {
            for z in 0..self.matrix[i].len() {
                print!("[{}]", self.matrix[i][z].value);
            }
            println!()
        }
    }
    fn right_movement(&self)->bool{
        todo!()

    }
    fn up_movement(&self)->bool{

        todo!()
    }
    fn down_movement(&self)->bool{

        todo!()
    }
    fn left_movement(&self)->bool{

        todo!()
    }




}

#[derive(Debug, Clone, Copy)]
struct Square {
    value: i32,
}

impl Square {
    fn new() -> Self {
        Self { value: (0) }
    }
    fn decide_number(&mut self) {
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();

        if n > 0.9 {
            self.set_four();
        } else {
            self.set_two();
        }
    }

    fn set_two(&mut self) {
        self.value = 2;
    }
    fn set_four(&mut self) {
        self.value = 4;
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>,mut game:Game) -> io::Result<()> {
    loop {
 terminal.draw(|f| ui(f,&game))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
    
}
fn ui<B: Backend>(f: &mut Frame<B>,game:&Game) {

            let rows = game.board.matrix.len();
            let cols = game.board.matrix[0].len();
            let size = f.size();
       

            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            
            

            for (i, row) in game.board.matrix.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {

                    let square = game.board.matrix[i][j];
                    let value = square.value;
                    let str_value = format!("{}", value);
                        
                    let mut block = Block::default().borders(Borders::ALL);

                    

                    let x = j as u16 * (size.width / cols as u16);
                    let y = i as u16 * (size.height / rows as u16);
                    let width = size.width / cols as u16;
                    let height = size.height / rows as u16;
                    let area = Rect { x, y, width, height };


                   let area_test = centered_rect(50,10,area);


                    


                    f.render_widget(block, area);
                    f.render_widget(Block::default().title(
                     Span::styled(str_value,
                        Style::default().add_modifier(Modifier::BOLD))).title_alignment(Alignment::Center),area_test);

                }
    }
}







fn main()-> Result<(),io::Error> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let mut game = Game::new();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal,game);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );



            

    Ok(())
}
