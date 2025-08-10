use crate::static_types::{GlobalState, NumericalNotation};
use ratatui::{
    Frame,
    crossterm::event::{self as ratEvent, Event as RatEvent},
    layout::{Alignment, Constraint, Layout},
    style::Color,
    symbols::Marker,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Circle, Line},
    },
};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};

pub fn render_grid(render_rx: Receiver<GlobalState>) -> Result<(), String> {
    if let Err(e) = color_eyre::install() {
        eprintln!("Failed to install color_eyre: {}", e);
        return Err(e.to_string());
    };
    let mut terminal = ratatui::init();
    let mut current_position = NumericalNotation::Five;

    let mut current_state: GlobalState = GlobalState {
        current_position: current_position,
        attack_pressed: false,
        position_history: Vec::new(),
        close_requested: false,
        last_successful_move: vec![],
    };

    loop {
        let frame_start = Instant::now();

        if let Ok(new_state) = render_rx.try_recv() {
            current_state = new_state;
        }

        if let Ok(CompletedFrame) = terminal.draw(|f| run_drawing(f, &current_state)) {
        } else {
            eprintln!("Failed to draw frame");
            return Err("Failed to draw frame".to_string());
        }

        if ratEvent::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(RatEvent::Key(key)) = ratEvent::read() {
                match key.code {
                    ratEvent::KeyCode::Esc => {
                        println!("Exiting...");
                        // current_state.close_requested = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_millis(16); // ~60 FPS
        if frame_time < target_frame_time {
            thread::sleep(target_frame_time - frame_time);
        }
    }

    ratatui::restore();
    Ok(())
}

fn get_coordinates(position: &NumericalNotation) -> (f64, f64) {
    match position {
        NumericalNotation::One => (-3.1, -3.1),
        NumericalNotation::Two => (0.0, -3.1),
        NumericalNotation::Three => (3.1, -3.1),
        NumericalNotation::Four => (-3.1, 0.0),
        NumericalNotation::Five => (0.0, 0.0),
        NumericalNotation::Six => (3.1, 0.0),
        NumericalNotation::Seven => (-3.1, 3.1),
        NumericalNotation::Eight => (0.0, 3.1),
        NumericalNotation::Nine => (3.1, 3.1),
    }
}

fn get_coordinates_pairs(positions: &[NumericalNotation]) -> Vec<((f64, f64), (f64, f64))> {
    let mut coordinated_positions = Vec::new();

    for position in positions {
        let coords = get_coordinates(position);
        if coordinated_positions.last() != Some(&coords) {
            coordinated_positions.push(coords);
        }
    }

    coordinated_positions
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect()
}

fn run_drawing(frame: &mut Frame, state: &GlobalState) {
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .areas(frame.area());

    let [top_left_area, bottom_left_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(left_area);

    let [top_right_area, bottom_right_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(right_area);
    let circle_coordinates = get_coordinates(&state.current_position);

    let lines_pairs = get_coordinates_pairs(&state.position_history);

    let block = Block::default().title("Input map");
    let inner_area = block.inner(top_left_area);

    frame.render_widget(block, top_left_area);

    let canvas = Canvas::default()
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: circle_coordinates.0 as f64,
                y: circle_coordinates.1 as f64,
                radius: 1.5,
                color: Color::Red,
            });
            for (start, end) in &lines_pairs {
                ctx.draw(&Line {
                    x1: start.0 as f64,
                    y1: start.1 as f64,
                    x2: end.0 as f64,
                    y2: end.1 as f64,
                    color: Color::Green,
                });
            }
        })
        .marker(Marker::Braille)
        .x_bounds([-5.0, 5.0])
        .y_bounds([-5.0, 5.0]);

    frame.render_widget(canvas, inner_area);

    if state.attack_pressed {
        let block = Block::default().title("Attack Pressed");
        let inner_area = block.inner(top_right_area);
        frame.render_widget(block, top_right_area);
        let canvas = Canvas::default()
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: 3.0,
                    color: Color::Red,
                });
            })
            .marker(Marker::Braille)
            .x_bounds([-5.0, 5.0])
            .y_bounds([-5.0, 5.0]);
        frame.render_widget(canvas, inner_area);
    } else {
        let block = Block::default().title("No Attack");
        frame.render_widget(block, top_right_area);
    };

    frame.render_widget(
        Paragraph::new(format!(
            "Last successful move: {:?}",
            state.last_successful_move
        ))
        .alignment(Alignment::Center),
        bottom_left_area,
    );
}
