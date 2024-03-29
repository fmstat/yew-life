mod cell;

use gloo::timers::callback::Interval;

use cell::Cell;
use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Html};

fn main() {}

struct App {
    active: bool,
    cells: Vec<Cell>,
    cells_width: usize,
    cells_height: usize,
    _interval: Interval,
}

enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCell(usize),
    Tick,
}

impl App {
    fn random_mutate(&mut self) {
        for cell in &mut self.cells {
            if rand::thread_rng().gen() {
                cell.set_alive();
            } else {
                cell.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cell in &mut self.cells {
            cell.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cells_height {
            for col in 0..self.cells_width {
                let neighbors = self.neighbors(row as isize, col as isize);
                let current_idx = self.row_col_as_idx(row as isize, col as isize);

                if self.cells[current_idx].is_alive() {
                    if Cell::alone(&neighbors) || Cell::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cell::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }

        to_dead.iter().for_each(|idx| self.cells[*idx].set_dead());
        to_live.iter().for_each(|idx| self.cells[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cell; 8] {
        [
            self.cells[self.row_col_as_idx(row + 1, col)],
            self.cells[self.row_col_as_idx(row + 1, col + 1)],
            self.cells[self.row_col_as_idx(row + 1, col - 1)],
            self.cells[self.row_col_as_idx(row - 1, col)],
            self.cells[self.row_col_as_idx(row - 1, col + 1)],
            self.cells[self.row_col_as_idx(row - 1, col - 1)],
            self.cells[self.row_col_as_idx(row, col - 1)],
            self.cells[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cells_height as isize);
        let col = wrap(col, self.cells_width as isize);

        row * self.cells_width + col
    }

    fn view_cell(idx: usize, cell: Cell, link: &Scope<Self>) -> Html {
        let cell_status = {
            if cell.is_alive() {
                "cell-live"
            } else {
                "cell-dead"
            }
        };

        html! {
            <div
              key={idx}
              class={classes!("game-cell", cell_status)}
              onclick={link.callback(move |_| Msg::ToggleCell(idx))}
            ></div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        let callback = ctx.link().callback(|()| Msg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));
        let (cells_width, cells_height) = (53, 40);

        Self {
            active: false,
            cells: vec![Cell::new_dead(); cells_width * cells_height],
            cells_width,
            cells_height,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            Msg::ToggleCell(idx) => {
                let cell = self.cells.get_mut(idx).unwrap();
                cell.toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> Html {
        let cell_rows = self
            .cells
            .chunks(self.cells_width)
            .enumerate()
            .map(|(y, cells)| {
                let idx_offset = y * self.cells_width;
                let cells = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| Self::view_cell(idx_offset + x, *cell, ctx.link()));

                html! {
                    <div key={y} class="game-row">{ for cells }</div>
                }
            });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">{ for cell_rows }</div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
                <footer class="app-footer">
                    <strong class="footer-text">{ "Game of Life" }</strong>
                    <a href="https://github.com/fmstat/yew-life" target="_blank">{ "source" }</a>
                </footer>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };

    result as usize
}
