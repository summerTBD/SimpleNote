use crate::Board;

/// 便利贴应用：SimpleNoteApp
/// 使用 serde 在关闭窗口时自动保存状态，下次打开恢复。
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SimpleNoteApp {
    /// 持久数据：所有便签
    board: Board,

    /// 临时 UI 状态（不存盘）
    #[serde(skip)]
    selected_id: Option<u64>, // 当前选中的便签 ID
    #[serde(skip)]
    show_hidden: bool, // 是否显示已隐藏的便签
}

impl Default for SimpleNoteApp {
    fn default() -> Self {
        Self {
            board: Board::new(),
            selected_id: None,
            show_hidden: false,
        }
    }
}

impl SimpleNoteApp {
    /// 应用启动时调用一次，用于加载上次保存的状态。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::default();

        // ===== 配色方案 =====
        // 背景：柔和的深绿色（Material Green 700）
        let bg_green = egui::Color32::from_rgb(46, 125, 50);

        visuals.window_fill = bg_green;
        visuals.panel_fill = bg_green;

        // 文本框：白色背景
        visuals.widgets.inactive.bg_fill = egui::Color32::WHITE;
        visuals.widgets.hovered.bg_fill = egui::Color32::WHITE;
        visuals.widgets.active.bg_fill = egui::Color32::WHITE;

        cc.egui_ctx.set_visuals(visuals);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for SimpleNoteApp {
    /// 窗口关闭时自动调用，保存当前状态。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// 每帧调用一次，在这里构建你的 UI。
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ===== 顶部菜单栏 =====
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("文件", |ui| {
                        if ui.button("退出").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // ===== 工具栏（➕ ❌ ➖ + 显示隐藏复选框）=====
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("➕ 新建").clicked() {
                    self.board
                        .add_note("新标签".to_string(), "新标签内容".to_string());
                }

                if ui.button("❌ 删除").clicked() {
                    if let Some(id) = self.selected_id {
                        if let Err(err) = self.board.delete_note(id) {
                            log::error!("删除便签失败: {}", err);
                        } else {
                            self.selected_id = None;
                        }
                    }
                }

                if ui.button("➖ 隐藏").clicked() {
                    if let Some(id) = self.selected_id {
                        if let Err(err) = self.board.hide_note(id) {
                            log::error!("隐藏便签失败: {}", err);
                        } else {
                            self.show_hidden = false;
                            self.selected_id = None;
                        }
                    }
                }

                ui.separator();

                ui.checkbox(&mut self.show_hidden, "👁 显示已隐藏");
            });
        });

        // ===== 便签列表 =====
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("📝 SimpleNote - 电子便利贴");

            // 先收集需要渲染的便签信息（避免借用冲突）
            let visible_notes: Vec<(u64, String, bool)> = self
                .board
                .notes()
                .iter()
                .filter(|n| self.show_hidden || !n.hidden)
                .map(|n| (n.id, n.title.clone(), n.hidden))
                .collect();

            if visible_notes.is_empty() {
                ui.add_space(20.0);
                ui.label("还没有便签，点击 ➕ 新建 创建第一个吧！");
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (id, title, hidden) in &visible_notes {
                    let is_selected = self.selected_id == Some(*id);

                    // 用 Frame 给每个便签一个卡片外框
                    let frame = if *hidden {
                        // 已隐藏：灰色背景
                        egui::Frame::NONE
                            .fill(egui::Color32::from_gray(220))
                            .inner_margin(egui::Margin::same(8))
                    } else if is_selected {
                        // 选中：白色底 + 绿色边框
                        egui::Frame::NONE
                            .fill(egui::Color32::WHITE)
                            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(76, 175, 80)))
                            .inner_margin(egui::Margin::same(8))
                    } else {
                        // 未选中：白色底
                        egui::Frame::NONE
                            .fill(egui::Color32::WHITE)
                            .inner_margin(egui::Margin::same(8))
                    };

                    frame.show(ui, |ui| {
                        let emoji = if *hidden { "🙈" } else { "📌" };
                        if ui
                            .selectable_label(is_selected, format!("{emoji} {title}"))
                            .clicked()
                        {
                            self.selected_id = Some(*id);
                        }
                    });

                    ui.add_space(4.0); // 便签之间的间距
                }
            });
        });
    }
}
