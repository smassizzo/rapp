use eframe::{
    egui::{self, Resize},
    epaint::{Color32, Stroke},
};
use egui::Ui;
use rapp::{Page, RustApp};
use std::sync::Arc;

const PHONE_SIZE: (f32, f32) = (200., 500.);
const BOTTOM_HEIGHT: f32 = 30.;

pub fn run(mut app: Box<impl RustApp + 'static>) -> eframe::Result<()> {
    app.start();
    let eframe_app = EframeWrapped(app);
    let options = eframe::NativeOptions {
        initial_window_size: Some([1000., 800.].into()),
        ..Default::default()
    };
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(eframe_app)))
}

struct EframeWrapped<'a>(Box<dyn RustApp + 'a>);

impl eframe::App for EframeWrapped<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (_, page) in self.0.pages().iter() {
                        let page = page.clone();

                        ui.vertical(|ui| {
                            ui.push_id(page.name(), |ui| show_minified(page, self, ctx, ui))
                        });
                    }
                });
            });
        });
    }
}

fn show_minified(
    page: Arc<Box<dyn Page>>,
    wrapped_app: &mut EframeWrapped,
    ctx: &egui::Context,
    ui: &mut Ui,
) {
    let app = &wrapped_app.0;

    let active = app.current_page() == page.name();

    let stroke = if active {
        Stroke::new(2., Color32::LIGHT_GRAY)
    } else {
        Stroke::new(1., Color32::DARK_GRAY)
    };
    let ppp = ctx.pixels_per_point();
    ctx.set_pixels_per_point(2.);
    egui::Frame::none()
        .outer_margin(25.)
        .stroke(Stroke::NONE)
        .show(ui, |ui| {
            // page name
            ui.label(egui::RichText::new(format!("{:?}", page.name())).size(10.));
            ui.end_row();
            ui.add_space(10.);

            // page within template
            egui::Frame::none()
                .rounding(10.)
                .stroke(stroke)
                .inner_margin(2.)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                        Resize::default()
                            .default_size(PHONE_SIZE)
                            .with_stroke(false)
                            .show(ui, |ui| {
                                let top_left = ui.cursor().min;

                                // top
                                // if let Some(_top) = p.template().as_ref().map(|t| t.top()) {
                                //     ui.heading(format!("{:?}", p));
                                //     ui.separator();
                                // }

                                // page
                                //AppPage::get_page_fn(&p)(app, ctx, ui);
                                ui.heading(page.name());

                                // add space to push bottom down
                                let current = ui.cursor().min;
                                let used_height = current.y - top_left.y;
                                let to_add = PHONE_SIZE.1 - used_height - BOTTOM_HEIGHT;
                                ui.add_space(to_add);

                                // // bottom
                                // if let Some(bottom) = &p.template().map(|t| t.bottom()) {
                                //     ui.separator();
                                //     ui.horizontal(|ui| {
                                //         if let Some(home) = bottom.home {
                                //             if ui.small_button("[Home]").clicked() {
                                //                 app.go_to(home);
                                //             }
                                //             ui.add_space(ui.available_width());
                                //         }
                                //     });
                                // }
                            });
                    });
                });
        });
    ctx.set_pixels_per_point(ppp);
}
