# SimpleNote

电子便利贴应用——Rust + egui 构建，支持桌面和 Web 双平台。

🌐 **在线使用**：[summertbd.github.io/SimpleNote](https://summertbd.github.io/SimpleNote/)

## 功能

- 📝 新建、编辑、删除便签
- 🙈 隐藏/取消隐藏便签
- 🎨 自动适配亮色/暗色主题
- 💾 关闭窗口自动保存（serde 持久化）
- ⌨️ 全键盘操作

## 快捷键

| 操作 | 快捷键 |
|---|---|
| 新建便签 | `Ctrl + N` |
| 删除便签 | `Ctrl + D` |
| 隐藏/取消隐藏 | `Ctrl + H` |
| 切换显示隐藏 | `Ctrl + Shift + H` |
| 上一个便签 | `↑` / `Ctrl + K` |
| 下一个便签 | `↓` / `Ctrl + J` |

## 运行

### 桌面版

```bash
cargo run --release
```

### Web 版

```bash
trunk serve
```

## 构建部署

```bash
# 桌面版
cargo build --release

# Web 版
trunk build --release --public-url /SimpleNote/
```

## 技术栈

- [Rust](https://www.rust-lang.org/)
- [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- [Trunk](https://trunkrs.dev/)（WASM 构建）
- [serde](https://serde.rs/)（持久化）
- 字体：[霞鹜文楷](https://github.com/lxgw/LxgwWenKai)

## 许可证

MIT OR Apache-2.0
