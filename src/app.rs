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
    #[serde(skip)]
    edit_title: String, // 编辑框的标题
    #[serde(skip)]
    edit_content: String, // 编辑框的内容
}

impl Default for SimpleNoteApp {
    fn default() -> Self {
        Self {
            board: Board::new(),
            selected_id: None,
            show_hidden: false,
            edit_title: String::new(),
            edit_content: String::new(),
        }
    }
}

impl SimpleNoteApp {
    /// 应用启动时调用一次，用于加载上次保存的状态。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // ===== 加载中文字体（解决中文显示为 □ 的问题）=====
        let mut fonts = egui::FontDefinitions::default();

        // 将微软雅黑字体嵌入到程序中
        fonts.font_data.insert(
            "cjk".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/msyh.ttc"
            ))),
        );

        // 插入到 proportional 字体队首：优先使用中文字体渲染
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "cjk".to_owned());

        // 等宽字体也加上，方便以后显示代码
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("cjk".to_owned());

        // 应用字体设置
        cc.egui_ctx.set_fonts(fonts);

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
        if let Some(id) = self.selected_id {
            let _ = self
                .board
                .edit_note(id, self.edit_title.clone(), self.edit_content.clone());
        }

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
                    let new_note = self
                        .board
                        .add_note("新标签".to_string(), "新标签内容".to_string());
                    self.selected_id = Some(new_note.id);
                    self.edit_title = new_note.title.clone();
                    self.edit_content = new_note.content.clone();
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
                        let _ = self.board.edit_note(
                            id,
                            self.edit_title.clone(),
                            self.edit_content.clone(),
                        );

                        if let Err(err) = self.board.hide_note(id) {
                            log::error!("隐藏便签失败: {}", err);
                        } else {
                            self.show_hidden = false;
                            self.selected_id = None;
                        }
                    }
                }

                if ui.button("🔓 取消隐藏").clicked() {
                    if let Some(id) = self.selected_id {
                        let _ = self.board.edit_note(
                            id,
                            self.edit_title.clone(),
                            self.edit_content.clone(),
                        );

                        if let Err(err) = self.board.unhide_note(id) {
                            log::error!("取消隐藏便签失败: {}", err);
                        } else {
                            self.selected_id = None;
                        }
                    }
                }

                ui.separator();

                ui.checkbox(&mut self.show_hidden, "👁 显示已隐藏");
            });
        });

        ui.separator();

        // ===== 主区域：左侧便签列表 + 右侧编辑区 =====

        // 左侧：便签列表
        egui::Panel::left("note_list")
            .resizable(true)
            .default_size(200.0)
            .show_inside(ui, |ui| {
                ui.heading("📝 便签列表");

                let visible_notes: Vec<(u64, String, bool)> = self
                    .board
                    .notes()
                    .iter()
                    .filter(|note| self.show_hidden || !note.hidden)
                    .map(|note| (note.id, note.title.clone(), note.hidden))
                    .collect();

                if visible_notes.is_empty() {
                    ui.add_space(10.0);
                    ui.label("还没有便签，点击 ➕ 新建");
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (id, title, hidden) in &visible_notes {
                        let is_selected = self.selected_id == Some(*id);

                        // 用当前主题色，自动适配亮色/暗色模式
                        let frame = if *hidden {
                            egui::Frame::NONE
                                .fill(ui.visuals().faint_bg_color)
                                .inner_margin(egui::Margin::same(6))
                        } else if is_selected {
                            egui::Frame::NONE
                                .fill(ui.visuals().selection.bg_fill)
                                .inner_margin(egui::Margin::same(6))
                        } else {
                            egui::Frame::NONE.inner_margin(egui::Margin::same(6))
                        };

                        frame.show(ui, |ui| {
                            let emoji = if *hidden { "🙈" } else { "📌" };
                            if ui
                                .selectable_label(is_selected, format!("{emoji} {title}"))
                                .clicked()
                            {
                                // 1️⃣ 先保存旧便签的编辑内容
                                if let Some(old_id) = self.selected_id {
                                    let _ = self.board.edit_note(
                                        old_id,
                                        self.edit_title.clone(),
                                        self.edit_content.clone(),
                                    );
                                }

                                // 2️⃣ 切换选中状态
                                if is_selected {
                                    self.selected_id = None;
                                } else {
                                    self.selected_id = Some(*id);
                                    // 3️⃣ 加载新便签到编辑缓冲区
                                    if let Some(note) =
                                        self.board.notes().iter().find(|n| n.id == *id)
                                    {
                                        self.edit_title = note.title.clone();
                                        self.edit_content = note.content.clone();
                                    }
                                }
                            }
                        });

                        ui.add_space(2.0);
                    }
                });
            });

        // 中央：编辑区（占主要空间）
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(selected_id) = self.selected_id {
                if self.board.notes().iter().any(|n| n.id == selected_id) {
                    ui.heading("✏️ 编辑便签");
                    ui.separator();

                    ui.label("标题：");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.edit_title)
                            .desired_width(f32::INFINITY),
                    );

                    ui.add_space(8.0);
                    ui.label("内容：");
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.edit_content)
                            .desired_width(f32::INFINITY),
                    );
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("👈 点击左侧便签开始编辑");
                });
            }
        });
    }
}
