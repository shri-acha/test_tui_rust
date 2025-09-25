use crossterm::{self};

use ratatui::{self,
        DefaultTerminal,
        Frame,
        widgets::{Widget,Block,List,Paragraph},
        text::{Line,Span},
        layout::Rect,
        prelude::{Constraint,Direction,Layout},
        style::{Modifier,self,Style,Color},
    };
use std::collections::HashMap;
use std::io;


enum FocusedField{
    Directory,
    MiniDirectory,
    MiniExtension,
}

struct App {

    monitoring_dir: String,
    dir_buffer: String,
    ext_buffer: String,
    file_dir_map:  HashMap<String,Vec<String>>,

    focused_field: FocusedField,
    exit: bool,
}

impl App {
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
            _=>{
                //Don't require any more bindings (as of now)
            }
        }
        Ok(())
    }
    fn handle_key_event(&mut self,key_event: crossterm::event::KeyEvent){
        match key_event.code  {
                crossterm::event::KeyCode::Esc =>{
                    self.exit = true;
                } 
                crossterm::event::KeyCode::Tab =>{
                    match self.focused_field {

                        //circle of enum for focus
                        FocusedField::Directory => {
                            self.focused_field = FocusedField::MiniDirectory;
                        }
                        FocusedField::MiniDirectory => {
                            self.focused_field = FocusedField::MiniExtension;
                        }
                        FocusedField::MiniExtension => {
                            self.focused_field = FocusedField::Directory;
                        }

                    }
                } 
                key_code => {
                    self.handle_user_typing(key_code);
                }
        }
    }
    fn handle_user_typing(&mut self,key_code:crossterm::event::KeyCode ){

        match self.focused_field {
            FocusedField::Directory => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.monitoring_dir = format!("{}{}",self.monitoring_dir,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.monitoring_dir.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::MiniDirectory => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.dir_buffer= format!("{}{}",self.dir_buffer,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.dir_buffer.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }
            FocusedField::MiniExtension => {
                match key_code{
                    crossterm::event::KeyCode::Char(c)=>{
                        self.ext_buffer = format!("{}{}",self.ext_buffer,c);
                    }
                    crossterm::event::KeyCode::Backspace=> {
                        self.ext_buffer.pop();
                    }
                    _=>{ 
                        //Don't require any more bindings (as of now)
                    }
                }
            }

        }
    }
}

impl Widget for &mut App {
    fn render(self,area: Rect,buf: &mut ratatui::buffer::Buffer)->(){

        // title 
        let main_title = Line::from (Span::styled("  DIRMON  ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();

        let inp_field_monitoring_dir_title = Line::from (Span::styled(" Monitoring Directory ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();

        let inp_mfield_extns_title = Line::from (Span::styled(" Extensions(, seperated) ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
                .centered();

        let inp_mfield_types_title = Line::from (Span::styled(" Directory ",Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green)))
            .centered();


        let mut list_items :Vec<String> = vec![];
        let dir_ext_vec: Vec<_> = self.file_dir_map.clone().into_iter().collect::<Vec<_>>();

        // monitoring-dir 
        let monitoring_dir= Paragraph::new(self.monitoring_dir.clone())
            .block(Block::new().borders(ratatui::widgets::Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded).title(inp_field_monitoring_dir_title))
            .centered();

        // block 
        let main_block = Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(main_title);


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
        
        // field 
        let lower_inner_block = Block::bordered()
                    .border_type(ratatui::widgets::BorderType::Rounded);

        let type_field = Paragraph::new(self.dir_buffer.clone())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(inp_mfield_types_title))
            .centered();

        let ext_field = Paragraph::new(self.ext_buffer.clone())
            .block(Block::new()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title(inp_mfield_extns_title))
            .centered();

        //finalizing block
        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Percentage(10), // monitoring directory
                Constraint::Percentage(70), // type-extension binding 
                Constraint::Percentage(20), // edit directory 
            ])
            .split(main_block.inner(area));



        let lower_inner_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(40), // monitoring directory
                Constraint::Percentage(80), // type-extension binding 
            ])
            .split(inner_chunks[2]);

        monitoring_dir.render(inner_chunks[0],buf);
        list.render(inner_chunks[1], buf);
        type_field.render(lower_inner_chunk[0],buf);
        ext_field.render(lower_inner_chunk[1],buf);
        main_block.render(area,buf);

    }
}

impl Default for App {
    fn default()->Self{
        Self{
            monitoring_dir: "/".to_string(),
            dir_buffer:"".to_string(),
            ext_buffer:"".to_string(),

            file_dir_map:HashMap::from([("Audio".to_string(),vec!["mp3".to_string(),"wav".to_string()])]),
            focused_field: FocusedField::Directory,
            exit: false,
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
