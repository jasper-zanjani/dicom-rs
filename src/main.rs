use gtk4 as gtk;
use gtk::prelude::*;

fn main() {
    let app = gtk::Application::builder()
        .application_id("org.moffitt.dicom")
        .build();
    app.connect_activate(build_ui);
    app.run();
}
fn build_ui(app: &gtk::Application) {
    let img_left = gtk::Image::from_file("/home/jasper/azure-ai-engineer.png");
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Moffitt DICOM viewer")
        .default_width(300)
        .default_height(300)
        .child(&img_left)
        .build();
        
    window.present();
}
