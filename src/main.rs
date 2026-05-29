use gpui::{
    App, AppContext, Application, Bounds, Context, FontWeight, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, WindowBounds, WindowOptions, div, px, rgb, size,
};

struct WorkbenchApp {
    sections: Vec<Section>,
    metrics: Vec<Metric>,
    tasks: Vec<Task>,
}

struct Section {
    name: &'static str,
    active: bool,
}

struct Metric {
    label: &'static str,
    value: &'static str,
    detail: &'static str,
}

struct Task {
    title: &'static str,
    detail: &'static str,
}

impl WorkbenchApp {
    fn new() -> Self {
        Self {
            sections: vec![
                Section {
                    name: "Datasets",
                    active: true,
                },
                Section {
                    name: "Audit",
                    active: false,
                },
                Section {
                    name: "Splits",
                    active: false,
                },
                Section {
                    name: "Experiments",
                    active: false,
                },
                Section {
                    name: "Metrics",
                    active: false,
                },
                Section {
                    name: "Robustness",
                    active: false,
                },
                Section {
                    name: "XAI",
                    active: false,
                },
                Section {
                    name: "Agents",
                    active: false,
                },
            ],
            metrics: vec![
                Metric {
                    label: "Project Type",
                    value: "Image AI",
                    detail: "Binary or multiclass research workflow",
                },
                Metric {
                    label: "Dataset State",
                    value: "Not Imported",
                    detail: "Folder and CSV manifest import planned",
                },
                Metric {
                    label: "Agent Mode",
                    value: "Planned",
                    detail: "ACP-ready command registry and approvals",
                },
            ],
            tasks: vec![
                Task {
                    title: "Create project workspace",
                    detail: "Define local project file, SQLite state, and artifact folders.",
                },
                Task {
                    title: "Import image dataset",
                    detail: "Read folders or manifests, infer labels, and index metadata.",
                },
                Task {
                    title: "Build audit pipeline",
                    detail: "Hash files, group near-duplicates, and flag label conflicts.",
                },
                Task {
                    title: "Add agent command layer",
                    detail: "Make UI actions and future ACP agents share typed commands.",
                },
            ],
        }
    }
}

impl Render for WorkbenchApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xf4f6f8))
            .size_full()
            .text_color(rgb(0x17202a))
            .child(self.sidebar())
            .child(self.content())
    }
}

impl WorkbenchApp {
    fn sidebar(&self) -> impl IntoElement {
        let mut sidebar = div()
            .flex()
            .flex_col()
            .w(px(220.0))
            .h_full()
            .p_5()
            .gap_2()
            .bg(rgb(0x111827))
            .text_color(rgb(0xe5e7eb))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .mb_5()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Vision Lab"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x9ca3af))
                            .child("Research workbench"),
                    ),
            );

        for section in &self.sections {
            let background = if section.active {
                rgb(0x2563eb)
            } else {
                rgb(0x111827)
            };
            let color = if section.active {
                rgb(0xffffff)
            } else {
                rgb(0xcbd5e1)
            };

            sidebar = sidebar.child(
                div()
                    .rounded_md()
                    .px_3()
                    .py_2()
                    .bg(background)
                    .text_color(color)
                    .text_sm()
                    .child(section.name),
            );
        }

        sidebar.child(
            div()
                .mt_auto()
                .rounded_md()
                .p_3()
                .bg(rgb(0x1f2937))
                .text_sm()
                .text_color(rgb(0xcbd5e1))
                .child("ACP-ready architecture"),
        )
    }

    fn content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .p_8()
            .gap_6()
            .child(self.header())
            .child(self.metric_row())
            .child(self.task_panel())
    }

    fn header(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .child("Dataset-first image research"),
            )
            .child(
                div()
                    .text_color(rgb(0x4b5563))
                    .child("Import, audit, split, evaluate, explain, and export reproducible image AI studies."),
            )
    }

    fn metric_row(&self) -> impl IntoElement {
        let mut row = div().flex().gap_4();

        for metric in &self.metrics {
            row = row.child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .rounded_md()
                    .border_1()
                    .border_color(rgb(0xd7dde5))
                    .bg(rgb(0xffffff))
                    .p_4()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x6b7280))
                            .child(metric.label),
                    )
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .child(metric.value),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x4b5563))
                            .child(metric.detail),
                    ),
            );
        }

        row
    }

    fn task_panel(&self) -> impl IntoElement {
        let mut panel = div()
            .flex()
            .flex_col()
            .rounded_md()
            .border_1()
            .border_color(rgb(0xd7dde5))
            .bg(rgb(0xffffff))
            .p_5()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .child("Next build tasks"),
            );

        for task in &self.tasks {
            panel = panel.child(
                div()
                    .flex()
                    .gap_3()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .pt_4()
                    .child(div().size_2().mt_2().rounded_full().bg(rgb(0x2563eb)))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(task.title))
                            .child(div().text_sm().text_color(rgb(0x4b5563)).child(task.detail)),
                    ),
            );
        }

        panel
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1120.0), px(760.0)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some(SharedString::from("Vision Research Workbench")),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| WorkbenchApp::new()),
        )
        .unwrap();
    });
}
