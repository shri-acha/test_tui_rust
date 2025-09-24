use crossterm::{self};

use ratatui::{self,
        DefaultTerminal,
        Frame,
        widgets::{Widget,Block,List,Paragraph},
        text::{Line,Span},
        layout::Rect,
        prelude::{Constraint,Direction,Layout},
        style::{Modifier,self,Style},
    };
use std::collections::HashMap;
use std::io;

enum FocusedField{
    Directory,
    MiniDirectory,
    MiniExts,
    None,
}

struct App<'a> {
    monitoring_dir: &'a str,
    file_dir_map:  HashMap<&'a str,Vec<&'a str>>,
    focused_field: FocusedField,
    exit: bool,
}

impl  App<'_> {
    fn draw(&mut self,frame: &mut Frame){
        frame.render_widget(self,frame.area());
    }
    fn run(&mut self,terminal: &mut ratatui::DefaultTerminal) -> io::Result<()>{
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Ok(_) = self.handle_event() {
                break Ok(());
            }
        }
    }
    fn handle_event(&mut self)->io::Result<()>{
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) if key_event.kind == crossterm::event::KeyEventKind::Press =>{
                self.handle_key_event(key_event);
            }
            _=>{}
        }
        Ok(())
    }
    fn handle_key_event(&mut self,key_event: crossterm::event::KeyEvent){
        match key_event.code  {
                crossterm::event::KeyCode::Esc =>{
                    self.exit = true;
                } 
                key_code => {
                    self.handle_user_typing(key_code);
                }
        }
    }
    fn handle_user_typing(&mut self,key_code:crossterm::event::KeyCode ){
        match self.focused_field {
            FocusedField::Directory => {
                println!("{:?}",key_code);
            }
            _=>{}
        }
    }
}

impl Widget for &mut App<'_> {
    fn render(self,area: Rect,buf: &mut ratatui::buffer::Buffer)->(){
        // title 
        let title = Line::from (Span::styled("  DIRMON  ",Style::default().add_modifier(Modifier::BOLD))).centered();
        let mut list_items :Vec<String> = vec![];
        let dir_ext_vec: Vec<_> = self.file_dir_map.clone().into_iter().collect::<Vec<_>>();

        // monitoring-dir 
        let monitoring_dir= Paragraph::new(self.monitoring_dir.clone())
            .block(Block::new().borders(ratatui::widgets::Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded))
            .centered();

        // block 
        let main_block = Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(title);


        // directory-extension lists
        for (key,val) in dir_ext_vec {
            let extension_string = val.join(",");
            list_items.push(format!("{} {}",key,extension_string));
        }
        let list = List::new(list_items)
                    .block(Block::new().borders(ratatui::widgets::Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded))
                    .highlight_symbol(">>>");

        // I/O
        // change directory field
        
        // buttons

        //finalizing block
        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(20), // monitoring directory
                Constraint::Percentage(60), // type-extension binding 
                Constraint::Percentage(20), // edit directory 
            ])
            .split(main_block.inner(area));

        monitoring_dir.render(inner_chunks[0],buf);
        list.render(inner_chunks[1], buf);


    }
}

impl Default for App<'_> {
    fn default()->Self{
        Self{
            monitoring_dir: "./",
            file_dir_map:HashMap::from([("Audio",vec!["mp3","wav"])]),
            exit: false,
            focused_field: FocusedField::Directory
        }
    }
}

fn main() {

    let mut terminal = ratatui::init(); // initializing terminal
    let mut app = App::default();

    while !app.exit {
        let _ = app.run(&mut terminal);
    }

    ratatui::restore(); 

}
