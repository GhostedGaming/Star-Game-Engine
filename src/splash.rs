//use std::error::Error;
//use std::io::stdout;
//use std::time::Duration;
//
//use ratatui::prelude::*;
//use ratatui_splash_screen::{SplashConfig, SplashScreen, SplashError};
//
//static SPLASH_CONFIG: SplashConfig = SplashConfig {
//    image_data: include_bytes!("C:\\Users\\richa\\star_game_engine\\Splash_Test.jpg"),
//    sha256sum: Some("f98a5a6db5c0d1830d6a0913b03ade0dc978cc88a875a22603079b800633aec9"),
//    render_steps: 12,
//    use_colors: true,
//};
//pub fn splash() -> Result<(), Box<dyn Error>> {
//    // create a terminal
//    let backend = CrosstermBackend::new(stdout());
//    let mut terminal = Terminal::new(backend)?;
//    // render splash screen
//    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
//    while !splash_screen.is_rendered() {
//        terminal.draw(|frame| {
//            frame.render_widget(&mut splash_screen, frame.size());
//        })?;
//        std::thread::sleep(Duration::from_millis(100));
//    }
//    Ok(())
//}