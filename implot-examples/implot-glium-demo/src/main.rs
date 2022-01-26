use imgui::{Condition, Window};

// The actual backend-specific code is in this.
mod support;

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    let mut showing_rust_demo = true;
    let mut demo_state = examples_shared::DemoState::new();
    let plotcontext = implot::Context::create();
    system.main_loop(move |_, ui| {
        // The context is moved into the closure after creation so plot_ui is valid.
        let plot_ui = plotcontext.get_plot_ui();

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }

        if showing_rust_demo {
            demo_state.show_demos(ui, &plot_ui);
        }

        ui.window("Welcome to the ImPlot-rs demo!")
            .size([430.0, 450.0], Condition::FirstUseEver)
            .position([10.0, 10.0], Condition::FirstUseEver)
            .build(|| {
                ui.checkbox("Show C++ ImPlot demo window", &mut showing_demo);
                ui.checkbox("Show Rust ImPlot demo windows", &mut showing_rust_demo);
                ui.text_wrapped(
                     "If you see something in the C++ demo window, but not in the Rust ImPlot \
                     demo window, that means the bindings are likely not implemented yet. \
                     Feel free to open an issue if you are missing something in particular.
                    ",
                );
            });
    });
}
