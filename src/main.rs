use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use rand::Rng;
use std::usize;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::canvas::{self, Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use tui::{Frame, Terminal};

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

    fn right_movement(&mut self) {
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() - 1 {
                if self.matrix[row][col].value != 0 {
                    let mut next_col = col + 1;
                    while next_col < self.matrix[row].len() && self.matrix[row][next_col].value == 0 {
                        self.matrix[row][next_col].value = self.matrix[row][next_col - 1].value;
                        self.matrix[row][next_col - 1].value = 0;
                        next_col += 1;
                    }
                    if next_col < self.matrix[row].len() && self.matrix[row][next_col].value == self.matrix[row][next_col - 1].value {
                        self.matrix[row][next_col].value += self.matrix[row][next_col - 1].value;
                        self.matrix[row][next_col - 1].value = 0;
                    }
                }
            }
        }
    }

    fn up_movement(&mut self) {
        for i in 0..self.matrix[0].len() {
            for z in 0..self.matrix.len() - 1 {
                let mut current = z;
                while current > 0 {
                    if self.matrix[current - 1][i].value == 0 {
                        self.matrix[current - 1][i].value = self.matrix[current][i].value;
                        self.matrix[current][i].value = 0;
                    } else if self.matrix[current - 1][i].value == self.matrix[current][i].value {
                        self.matrix[current - 1][i].value += self.matrix[current][i].value;
                        self.matrix[current][i].value = 0;
                    }
                    current -= 1;
                }
            }
        }
    }

    fn down_movement(&mut self) {
        for i in 0..self.matrix[0].len() {
            for z in (0..self.matrix.len() - 1).rev() {
                let mut current = z;
                while current < self.matrix.len() - 1 {
                    if self.matrix[current + 1][i].value == 0 {
                        self.matrix[current + 1][i].value = self.matrix[current][i].value;
                        self.matrix[current][i].value = 0;
                    } else if self.matrix[current + 1][i].value == self.matrix[current][i].value {
                        self.matrix[current + 1][i].value += self.matrix[current][i].value;
                        self.matrix[current][i].value = 0;
                    }
                    current += 1;
                }
            }
        }
    }
    fn left_movement(&mut self) {
        for row in 0..self.matrix.len() {
            for col in 1..self.matrix[row].len() {
                if self.matrix[row][col].value != 0 {
                    let mut prev_col = col as i32 - 1;
                    while prev_col >= 0 && self.matrix[row][prev_col as usize].value == 0 {
                        self.matrix[row][prev_col as usize].value = self.matrix[row][prev_col as usize + 1].value;
                        self.matrix[row][prev_col as usize + 1].value = 0;
                        prev_col -= 1;
                    }
                    if prev_col >= 0 && self.matrix[row][prev_col as usize].value == self.matrix[row][prev_col as usize + 1].value {
                        self.matrix[row][prev_col as usize].value += self.matrix[row][prev_col as usize + 1].value;
                        self.matrix[row][prev_col as usize + 1].value = 0;
                    }
                }
            }
        }
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut game: Game) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &game))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            } else if let KeyCode::Char('k') = key.code {
                game.board.set_rnd_avaible_square();
            } else if let KeyCode::Left = key.code {
                game.board.left_movement();
                game.board.set_rnd_avaible_square();
            } else if let KeyCode::Up = key.code {
                game.board.up_movement();
                game.board.set_rnd_avaible_square();
            } else if let KeyCode::Right = key.code {
                game.board.right_movement();
                game.board.set_rnd_avaible_square();
            } else if let KeyCode::Down = key.code {
                game.board.down_movement();
                game.board.set_rnd_avaible_square();
            }
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
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

            let mut block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let two: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(238, 228, 218)));
            let four: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 224, 200)));
            let eight: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(242, 177, 121)));
            let sixteen: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(245, 149, 99)));
            let thirtytwo: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(246, 124, 95)));
            let sixtyfour: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(246, 94, 59)));
            let onetwentyeight: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 207, 114)));
            let twosixfive: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 204, 97)));
            let fiveonetwo: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 200, 80)));
            let onezero24: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 197, 63)));
            let two048: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb(237, 194, 46)));
            let white: Block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Rgb((20), (20), (1))));

            let x = j as u16 * (size.width / cols as u16);
            let y = i as u16 * (size.height / rows as u16);
            let width = size.width / cols as u16;
            let height = size.height / rows as u16;
            let area = Rect {
                x,
                y,
                width,
                height,
            };

            let area_test = centered_rect(10, 12, area);

            if value == 0 {
                f.render_widget(block, area);
            } else if value == 2 {
                f.render_widget(two, area);
            } else if value == 4 {
                f.render_widget(four, area);
            } else if value == 8 {
                f.render_widget(eight, area);
            } else if value == 16 {
                f.render_widget(sixteen, area);
            } else if value == 32 {
                f.render_widget(thirtytwo, area);
            } else if value == 64 {
                f.render_widget(sixtyfour, area);
            } else if value == 128 {
                f.render_widget(onetwentyeight, area);
            } else if value == 256 {
                f.render_widget(twosixfive, area);
            } else if value == 1024 {
                f.render_widget(onezero24, area);
            } else if value == 2048 {
                f.render_widget(two048, area);
            } else {
                f.render_widget(block, area);
            }

            f.render_widget(
                Block::default()
                    .title(Span::styled(
                        str_value,
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Black),
                    ))
                    .title_alignment(Alignment::Center),
                area_test,
            );

            // }
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut game = Game::new();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal, game);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    Ok(())
}
