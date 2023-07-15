use crate::{err::BFError, prog::Interpreter};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListState, ListItem},
    Frame, Terminal, style::{Style, Modifier},
};
use std::io::{stdout, Stdout};

pub struct Tui<'io> {
    prog: Interpreter<'io>,
    prog_chars: List<'io>,
    prog_state: ListState,
}

impl<'io> Tui<'io> {
    pub fn new(prog: Interpreter<'io>) -> Self {
        let mut prog_chars: Vec<ListItem> = Vec::with_capacity(prog.program.len());
        for i in prog.program.iter() {
            prog_chars.push(ListItem::new(i.to_str()));
        }
        let prog_chars = List::new(prog_chars).block(Block::default().title("Program").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        Self { prog, prog_chars, prog_state: ListState::default(), }
    }

    pub fn guify(&mut self) -> Result<(), BFError> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let res = self.run_prog(&mut terminal);
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        res
    }

    fn ui(&mut self, term: &mut Frame<CrosstermBackend<Stdout>>) -> std::io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(50), Constraint::Percentage(30)].as_ref())
            .split(term.size());

        self.prog_state.select(Some(self.prog.instr));
        let offset = 0.max(self.prog.instr - 10);
        *self.prog_state.offset_mut() = offset;
        term.render_stateful_widget(self.prog_chars.clone(), chunks[0], &mut self.prog_state);
        Ok(())
    }

    fn run_prog(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), BFError> {
        loop {}
        Ok(())
    }

    pub fn into_prog(self) -> Interpreter<'io> {
        self.prog
    }
}
