use crate::{
    components::component::Component,
    shared::constants::command_table::{BLOCK_TITLE, COMMAND_LIST, COMMAND_LIST_TABLE_HEADERS},
    states::command_table::CommandTableState,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, BorderType, Borders, Row, Table},
};

pub struct CommandTableComponent {
    state: CommandTableState,
}

impl Default for CommandTableComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandTableComponent {
    pub fn new() -> Self {
        CommandTableComponent {
            state: CommandTableState::new(),
        }
    }
}

impl Component for CommandTableComponent {
    fn render(&mut self, f: &mut Frame, rect: Rect) {
        let rows = COMMAND_LIST
            .iter()
            .map(|row_data| {
                Row::new(vec![row_data.0, row_data.1, row_data.2])
                    .height(2)
                    .top_margin(1)
                    .style(self.state.data_row_style())
            })
            .collect::<Vec<Row>>();

        let table = Table::new(rows, self.state.table_widths())
            .header(
                Row::new(vec![
                    COMMAND_LIST_TABLE_HEADERS.0,
                    COMMAND_LIST_TABLE_HEADERS.1,
                    COMMAND_LIST_TABLE_HEADERS.2,
                ])
                .style(self.state.headers_style()),
            )
            .block(
                Block::default()
                    .title(BLOCK_TITLE)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(self.state.block_style()),
            );

        f.render_widget(table, rect);
    }

    fn handle_key_event(&mut self, _key: crossterm::event::KeyEvent) {
        unimplemented!()
    }

    fn update_from_state(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
