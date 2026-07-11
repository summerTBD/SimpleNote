use crate::Board;
use crate::action::Action;

/// 窗口最小尺寸（与 `main.rs` 中 `with_min_inner_size` 保持一致）
const MIN_WINDOW_SIZE: egui::Vec2 = egui::Vec2::new(350.0, 260.0);

/// 便利贴应用：SimpleNoteApp
/// 使用 serde 在关闭窗口时自动保存状态，下次打开恢复。
#[derive(Default, serde::Deserialize, serde::Serialize)]
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
    #[serde(skip)]
    first_frame: bool, // 首帧标记：用于窗口尺寸校验
}

impl SimpleNoteApp {
    /// 应用启动时调用一次，用于加载上次保存的状态。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // ===== 加载中文字体（解决中文显示为 □ 的问题）=====
        let mut fonts = egui::FontDefinitions::default();

        // 将霞鹜文楷字体嵌入到程序中（开源字体，SIL Open Font License）
        #[expect(clippy::large_include_file)]
        fonts.font_data.insert(
            "cjk".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/LXGWWenKai-Regular.ttf"
            ))),
        );

        // 插入到 proportional 字体队首：优先使用中文字体渲染
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .expect("Proportional font family should exist")
            .insert(0, "cjk".to_owned());

        // 等宽字体也加上，方便以后显示代码
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .expect("Monospace font family should exist")
            .push("cjk".to_owned());

        // 应用字体设置
        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            let mut app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.first_frame = true;
            app
        } else {
            Self {
                first_frame: true,
                ..Default::default()
            }
        }
    }

    /// 把键盘输入翻译成 Action
    fn detect_action(ui: &egui::Ui) -> Option<Action> {
        let typing = ui.ctx().egui_wants_keyboard_input();

        ui.input(|i| {
            let ctrl = i.modifiers.ctrl;
            let shift = i.modifiers.shift;

            // 带修饰键的：不冲突，不用检查 typing
            if i.key_pressed(egui::Key::N) && ctrl {
                return Some(Action::AddNote);
            }
            if i.key_pressed(egui::Key::D) && ctrl {
                return Some(Action::DeleteNote);
            }
            // 范围小的先判断：Ctrl+Shift+H 优先于 Ctrl+H
            if i.key_pressed(egui::Key::H) && ctrl && shift {
                return Some(Action::ToggleShowHidden);
            }
            if i.key_pressed(egui::Key::H) && ctrl {
                return Some(Action::ToggleHideNote);
            }

            // 纯按键：需要检查是否在打字
            if !typing {
                if i.key_pressed(egui::Key::ArrowDown) || (i.key_pressed(egui::Key::J) && ctrl) {
                    return Some(Action::SelectNext);
                }
                if i.key_pressed(egui::Key::ArrowUp) || (i.key_pressed(egui::Key::K) && ctrl) {
                    return Some(Action::SelectPrev);
                }
            }

            None
        })
    }

    /// 执行 Action（按钮和快捷键的统一入口）
    fn handle_action(&mut self, action: &Action) {
        // 大多数操作前先保存当前编辑内容到 board
        let needs_save = !matches!(action, Action::EditNote);
        if needs_save {
            self.save_selected();
        }

        match action {
            Action::AddNote => {
                let note = self.board.add_note("新标签".to_owned(), String::new());
                self.selected_id = Some(note.id);
                self.edit_title = note.title.clone();
                self.edit_content = note.content.clone();
            }
            Action::DeleteNote => {
                if let Some(id) = self.selected_id
                    && self.board.delete_note(id).is_ok()
                {
                    self.selected_id = None;
                }
            }
            Action::ToggleHideNote => {
                if let Some(id) = self.selected_id {
                    self.board.toggle_hide_note(id);
                }
            }
            Action::ToggleShowHidden => {
                self.show_hidden = !self.show_hidden;
            }
            Action::SelectNext => {
                self.selected_id = self
                    .board
                    .next_visible_note(self.selected_id, self.show_hidden);
                self.load_selected();
            }
            Action::SelectPrev => {
                self.selected_id = self
                    .board
                    .prev_visible_note(self.selected_id, self.show_hidden);
                self.load_selected();
            }
            Action::EditNote => {
                // 暂不使用：当前模式是"选中即编辑"
            }
        }
    }

    /// 把编辑框内容写回 board
    fn save_selected(&mut self) {
        if let Some(id) = self.selected_id {
            self.board
                .edit_note(id, self.edit_title.clone(), self.edit_content.clone())
                .ok();
        }
    }

    /// 把选中便签的标题和内容加载到编辑框
    fn load_selected(&mut self) {
        if let Some(id) = self.selected_id
            && let Some(note) = self.board.notes().iter().find(|n| n.id == id)
        {
            self.edit_title = note.title.clone();
            self.edit_content = note.content.clone();
        }
    }
}

impl eframe::App for SimpleNoteApp {
    /// 窗口关闭时自动调用，保存当前状态。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Some(id) = self.selected_id {
            self.board
                .edit_note(id, self.edit_title.clone(), self.edit_content.clone())
                .ok();
        }

        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// 每帧调用一次，在这里构建你的 UI。
    #[expect(clippy::too_many_lines)]
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ===== 首帧：校验窗口尺寸，防止因历史残留导致窗口过小 =====
        if self.first_frame {
            self.first_frame = false;
            let window_size = ui
                .ctx()
                .input(|i| i.viewport().inner_rect.map(|r| r.size()));
            if let Some(size) = window_size
                && (size.x < MIN_WINDOW_SIZE.x || size.y < MIN_WINDOW_SIZE.y)
            {
                let target =
                    egui::Vec2::new(size.x.max(MIN_WINDOW_SIZE.x), size.y.max(MIN_WINDOW_SIZE.y));
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::InnerSize(target));
            }
        }

        // ===== 快捷键处理 =====
        let action = Self::detect_action(ui);
        if let Some(action) = action {
            self.handle_action(&action);
        }

        // ===== 顶部菜单栏 =====
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("文件", |ui| {
                        if ui.button("恢复默认窗口").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                egui::Vec2::new(700.0, 500.0),
                            ));
                            ui.close();
                        }
                        ui.separator();

                        if ui.button("退出").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
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
                    self.handle_action(&Action::AddNote);
                }

                if ui.button("❌ 删除").clicked() {
                    self.handle_action(&Action::DeleteNote);
                }

                if ui.button("➖ 隐藏").clicked() {
                    self.handle_action(&Action::ToggleHideNote);
                }

                if ui.button("🔓 取消隐藏").clicked() {
                    self.handle_action(&Action::ToggleHideNote);
                }

                ui.separator();

                if ui.checkbox(&mut self.show_hidden, "👁 显示已隐藏").changed() {
                    // show_hidden 已被 checkbox 修改，Action 不再翻转
                }
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
                                    self.board
                                        .edit_note(
                                            old_id,
                                            self.edit_title.clone(),
                                            self.edit_content.clone(),
                                        )
                                        .ok();
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
