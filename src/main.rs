use druid::widget::{Button, Flex, Label, Slider, TextBox};
use druid::{AppLauncher, Data, Env, EventCtx, Lens, Widget, WidgetExt, WindowDesc};
use std::path::PathBuf;
mod converter;
mod parse_lens; // Import the custom lens

#[derive(Clone, Data, Lens)]
struct AppState {
    input_path: String,
    output_path: String,
    quality: String,  // Use a String to handle text input
    status: String,
}

fn build_ui() -> impl Widget<AppState> {
    let input_label = Label::new("Input Path:");
    let input_textbox = TextBox::new().lens(AppState::input_path);

    let output_label = Label::new("Output Path:");
    let output_textbox = TextBox::new().lens(AppState::output_path);

    let quality_label = Label::new("Quality:");
    let quality_slider = Slider::new().with_range(0.0, 100.0).lens(parse_lens::RoundLens);
    let quality_textbox = TextBox::new().lens(AppState::quality);

    let status_label = Label::new(|data: &AppState, _: &Env| data.status.clone());

    let convert_button = Button::new("Convert")
        .on_click(|ctx: &mut EventCtx, data: &mut AppState, _: &Env| {
            let input_path = PathBuf::from(&data.input_path);
            let output_path = PathBuf::from(&data.output_path);
            let quality = data.quality.parse::<f64>().unwrap_or(90.0);

            match converter::convert_jpeg_to_webp(&input_path, &output_path, quality as f32) {
                Ok(_) => data.status = "Conversion successful!".into(),
                Err(e) => data.status = format!("Error: {}", e),
            }

            ctx.request_update();
        });

    Flex::column()
        .with_child(input_label)
        .with_child(input_textbox)
        .with_child(output_label)
        .with_child(output_textbox)
        .with_child(quality_label)
        .with_child(quality_slider)
        .with_child(quality_textbox)
        .with_child(convert_button)
        .with_child(status_label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Image Converter")
        .window_size((400.0, 300.0));

    let initial_state = AppState {
        input_path: "".into(),
        output_path: "".into(),
        quality: "90.0".into(),  // Default quality value as String
        status: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
