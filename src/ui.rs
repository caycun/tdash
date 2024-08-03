use crate::{structs::OutputData, App};

use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame, mut collections: Vec<OutputData>, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            collections
                .iter()
                .map(|item| Constraint::Percentage(item.size as u16))
                .collect::<Vec<_>>(),
        )
        .split(frame.size());
    for (index, item) in collections.iter_mut().enumerate() {
        let log = item.data.pop();
        item.data.insert(item.data.len(), log.unwrap());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut state = ListState::default();
        let mut list = List::new(item.data.clone())
            .block(Block::bordered().title(index.to_string()))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        if app.current_window == index as u8 {
            list = list.style(Style::default().fg(Color::Yellow).bg(Color::Black));
        }

        if app.full_screen && app.current_window == index as u8 {
            //delete past items of the frame so it only displays one

            let height = frame.size().height as usize - 2;
            if item.data.len() > height {
                *state.offset_mut() = item.data.len() - height;
            } else {
                *state.offset_mut() = 0;
            }

            frame.render_widget(Clear, frame.size());
            frame.render_stateful_widget(list, frame.size(), &mut state);
            break;
        }

        let mut available_height = layout[index].height as usize - 2;

        if app.full_screen {
            available_height = frame.size().height as usize - 2_usize;
        }

        if item.data.len() > available_height {
            *state.offset_mut() = item.data.len() - available_height;
        } else {
            *state.offset_mut() = 0;
        }

        frame.render_stateful_widget(list, layout[index], &mut state);

        frame.render_stateful_widget(
            scrollbar,
            layout[index].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut item.vertical_scroll_state,
        );
    }
}
