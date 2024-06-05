use druid::{
    widget::{Button, Flex, Label, Slider, TextBox,},
    AppLauncher, Data, Env, EventCtx, Lens, Widget, WidgetExt, WindowDesc,
};
use native_dialog::{FileDialog,};
use std::path::PathBuf;
mod converter;
mod parse_lens;

#[derive(Clone, Data, Lens)]
struct AppState {
    input_path: String,
    output_path: String,
    quality: String,
    status: String,
}

fn build_ui() -> impl Widget<AppState> {
    let input_label = Label::new("Input Path:");
    let input_textbox = TextBox::new().lens(AppState::input_path);

    let input_browse_button = Button::new("Browse...").on_click(|_ctx, data: &mut AppState, _env| {
        if let Some(path) = FileDialog::new()
            .set_location("~/Desktop")
            .add_filter("Image Files", &["png", "jpg", "jpeg"])
            .show_open_single_file()
            .unwrap()
        {
            data.input_path = path.to_string_lossy().to_string();
        }
    });

    let output_label = Label::new("Output Path:");
    let output_textbox = TextBox::new().lens(AppState::output_path);
    let output_browse_button = Button::new("Browse...").on_click(|_ctx, data: &mut AppState, _env| {
        if let Some(path) = FileDialog::new()
            .set_location("~/Desktop")
            .show_open_single_dir()
            .unwrap()
        {
            data.output_path = path.to_string_lossy().to_string();
        }
    });

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
        .with_child(Flex::row().with_flex_child(input_textbox, 1.0).with_child(input_browse_button))
        .with_child(output_label)
        .with_child(Flex::row().with_flex_child(output_textbox, 1.0).with_child(output_browse_button))
        .with_child(quality_label)
        .with_child(quality_slider)
        .with_child(quality_textbox)
        .with_child(convert_button)
        .with_child(status_label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("ConvertIt")
        .window_size((800.0, 400.0));

    let initial_state = AppState {
        input_path: "".into(),
        output_path: "".into(),
        quality: "80.0".into(),
        status: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}



// now lets work on the UI. the UI should be 2 columns, left column being the input path, with a button that allows the user to browse for an image, an image preview and the output path along with a button to browse for the output path. The right side should have a "save as" text filed that allows the user to change the name of the image if they choose, if not then it just uses the previous name, and finally the convert button