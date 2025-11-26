# RustSL

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![GUI: PyQt5](https://img.shields.io/badge/GUI-PyQt5-blue.svg)](https://www.riverbankcomputing.com/software/pyqt/)

一款基于 Rust 开发的模块化免杀框架，支持 GUI 可视化与灵活的二次开发，内置多种 Shellcode 伪装与反沙箱策略。

![alt text](static/front.png)


## 项目优势

### Rust免杀
- **静态链接和零依赖**：Rust 程序可以静态链接所有依赖库，生成单一的独立二进制文件，减少外部库的特征签名，降低被杀毒软件检测到的概率。
- **高效编译优化**：通过 LTO（链接时优化）、strip（移除符号表）和 codegen-units=1 等配置，生成更紧凑和混淆的二进制代码，特征更难被逆向分析。
- **无运行时开销**：Rust 编译为原生机器码，无需虚拟机或解释器运行，避免了像 .NET 或 Java 那样的明显运行时特征。
- **高级混淆特性**：利用 Rust Nightly 的特性，如路径修剪（trim-paths）、panic immediate-abort 和重新编译标准库，彻底移除调试信息、路径字符串和 panic 消息，进一步增强隐蔽性。
- **条件编译和模块化**：利用 Cargo features 实现条件编译，用户可根据需要选择启用特定加密方式、运行模式或检测策略，只编译必要的代码模块，显著减少二进制文件大小和特征签名，

### GUI
- **用户友好**：基于 PyQt5 的图形界面让用户无需掌握命令行知识，即可轻松配置参数和生成加载器，降低了使用门槛。
- **集成化操作**：将 Shellcode 选择、加密方式配置、图标设置、反沙箱检测勾选、运行模式选择、签名伪造和文件捆绑等所有步骤集成在一个界面中，实现一键生成，简化工作流程。
- **直观可视化**：提供下拉菜单、复选框和文件选择器等控件，直观展示选项，避免手动输入错误，提高配置准确性。
- **实时反馈**：界面显示配置状态和生成进度，用户能即时了解操作结果，便于调试和调整。
- **跨平台兼容**：支持Windows、Linux 和 macOS跨平台兼容，确保在不同操作系统上的一致体验。
- **插件化扩展**：GUI 基于配置文件动态加载功能模块，支持添加新加密方式、运行模式或检测策略，无需修改代码即可扩展。

### 可拓展性
- **插件化架构**：基于 `config/plugins.json` 配置文件动态加载功能模块，新插件只需实现特定接口（如 `name` 和 `process` 函数），即可自动被 GUI 和命令行工具识别，无需修改核心代码。
- **模块化设计**：加密方式、运行模式和 VM 检测等功能模块独立存放（如 `encrypt_lib/`、`src/decrypt/`、`src/exec_shellcode/`、`src/guard/`），便于单独开发和维护。
- **灵活的 Cargo features**：通过 Cargo features 控制编译内容，用户可选择性启用所需功能，减少不必要代码，优化二进制大小和性能。
- **易于二次开发**：提供详细的二次开发指南，支持添加新加密方式、运行方式或检测策略，只需在相应目录添加代码并注册到配置文件中。

## ✨ 特性

### 🔐 加密方式
- **RC4** - 流加密算法
- **IPv4** - 将 Shellcode 转换为 IPv4 地址格式
- **IPv6** - 将 Shellcode 转换为 IPv6 地址格式
- **MAC** - 将 Shellcode 转换为 MAC 地址格式
- **UUID** - 将 Shellcode 转换为 UUID 格式
- **AES** - 高级加密标准(AES-256-CBC)
- 可拓展...

### 💾 内存分配
- **VirtualAlloc** - 使用系统 API 分配 RWX 内存
- 可拓展...

### 🛡️ VM/沙箱检测
- **Tick 检测** - 时间差异分析
- **鼠标轨迹检测** - 通过多点轨迹特征判断真实鼠标活动
- 可拓展...

### 🚀 运行模式
- **CreateThread 直接执行** - 传统线程创建方式
- 可拓展...

## 📦 项目结构

```
RustSL/
├── gui/                     # PyQt5 图形界面与组件
├── src/                     # Rust 核心代码
│   ├── main.rs              # Rust 主程序入口
│   ├── alloc_mem/           # 内存分配相关模块
│   ├── decrypt/             # Shellcode 解密模块
│   ├── exec/                # Shellcode 执行模块
│   ├── forgery/             # 资源伪造与混淆
│   ├── guard/               # 反沙箱/反虚拟机检测
│   └── utils/               # 工具函数
├── config/                  
│   └── plugins.json         # 插件与功能配置
├── encrypt_lib/         # Python 加密插件目录
├── sign/                    # 签名相关
├── encrypt.py               # Shellcode 加密脚本
├── main.py                  # GUI 启动入口
├── Cargo.toml               # Rust 项目配置文件
├── build.rs                 # Rust 构建脚本
├── requirements.txt         # Python 依赖列表
├── input/                   # Shellcode 输入目录
├── output/                  # 生成的可执行文件输出目录
├── static/                  # 静态资源（如图片、截图）
├── icons/                   # 额外图标资源
└── rust-toolchain.toml      # Rust 工具链配置
```

## 🚀 快速开始

### 环境要求

- **Python 3.7+**
- **Rust Nightly** (需配合 rustup 使用)
- **PyQt5**
- **Cargo** (Rust 包管理器)

### 安装依赖
#### Python 依赖
```bash
pip install -r requirements.txt
```

**依赖说明：**
- `PyQt5` - 图形界面框架
- `pycryptodome` - 加密库（ChaCha20, AES-GCM, RC4 等）

#### Rust 环境配置

本项目依赖 Rust Nightly 版本及 `build-std` 特性以优化体积和去除特征。

##### Windows: 

1. **安装 Rustup**
   - 下载并运行 [rustup-init.exe](https://win.rustup.rs/)

2. **配置 Nightly 工具链**
   项目根目录已包含 `rust-toolchain.toml`，进入目录后 Rustup 会自动检测。你需要手动安装 Nightly 工具链及源码组件：
   ```bash
   # 安装 nightly 工具链
   rustup install nightly

   # 安装 rust-src 组件（用于 build-std 重新编译标准库）
   rustup component add rust-src --toolchain nightly
   
   # 添加 Windows MSVC 目标（通常默认已安装）
   rustup target add x86_64-pc-windows-msvc --toolchain nightly
   ```

3. **验证环境**
   ```bash
   cargo +nightly --version
   ```

##### Linux / macOS:
1. **安装 Rustup**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
2. **配置 Nightly 工具链**
   ```bash
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    rustup target add x86_64-pc-windows-gnu --toolchain nightly
   ```
3. **安装交叉编译工具**
   - Ubuntu/Debian:
     ```bash
     sudo apt update
     sudo apt install gcc-mingw-w64
     ```
   - Arch Linux:
     ```bash
     sudo pacman -S mingw-w64-gcc
     ```
   - macOS (使用 Homebrew):
     ```bash
     brew install mingw-w64
     ```

4. **验证环境**
   ```bash
    cargo +nightly --version
    ```

### 使用方法

#### 1. 启动 GUI
```bash
python main.py
```

#### 2. 选择配置
在 GUI 界面中选择：
- **加密方式**：RC4 / IPv4 / IPv6 / MAC / UUID / AES
- **运行模式**：CreateThread
- **内存分配**：VirtualAlloc
- **VM 检测**：勾选需要的检测项

#### 3. 生成加载器
点击 **"一键生成"** 按钮，程序将自动完成：
- Shellcode 加密
- Rust 编译（带特性选择）
- 文件复制到 `output/` 目录
- 签名伪造（如启用）

## 🪟 Windows 7 兼容性配置（可选）

### 环境要求

要启用 Windows 7 兼容性构建，您需要下载并配置以下两个兼容性库：

### 1. VC-LTL5 (Visual C++ Low Level Thread Library)
VC-LTL5 是一个轻量级的 Windows 运行时库，提供了对 Windows 7 的向下兼容支持。

**下载地址**：https://github.com/Chuyu-Team/VC-LTL5/releases

**推荐版本**：VC-LTL-5.2.2-Binary.zip

### 2. YY-Thunks (Windows API Thunk Library)
YY-Thunks 提供了对较新 Windows API 的向下兼容 thunk 实现。

**下载地址**：https://github.com/Chuyu-Team/YY-Thunks/releases

**推荐版本**：YY-Thunks-1.1.7-Binary.zip

### 环境变量配置

下载并解压上述两个库后，需要设置以下环境变量：

#### (Windows)
1. 右键点击"此电脑" → "属性" → "高级系统设置"
2. 点击"环境变量"
3. 在"系统变量"中添加：
   - 变量名：`VC_LTL`，变量值：`C:\path\to\VC-LTL5`
   - 变量名：`YY_THUNKS`，变量值：`C:\path\to\YY-Thunks`

然后就可以使用Win7兼容模式编译加载器了。


## ⚙️ 配置文件

`config/plugins.json` 控制所有功能模块：

```json
{
  "encryption": [
    { "id": "ipv4", "label": "ipv4", "encrypt_arg": "ipv4", "feature": "decrypt_ipv4" },
    { "id": "ipv6", "label": "ipv6", "encrypt_arg": "ipv6", "feature": "decrypt_ipv6" },
    { "id": "mac", "label": "mac", "encrypt_arg": "mac", "feature": "decrypt_mac" },
    { "id": "uuid", "label": "uuid", "encrypt_arg": "uuid", "feature": "decrypt_uuid" },
    { "id": "rc4", "label": "rc4", "encrypt_arg": "rc4", "feature": "decrypt_rc4" },
    { "id": "aes", "label": "aes", "encrypt_arg": "aes", "feature": "decrypt_aes" }
  ],
  "alloc_mem_modes": [
    { "id": "alloc_mem_va", "label": "VirtualAlloc", "feature": "alloc_mem_va" },
    { "id": "alloc_mem_global", "label": "GlobalAlloc", "feature": "alloc_mem_global" },
    { "id": "alloc_mem_local", "label": "LocalAlloc", "feature": "alloc_mem_local" }
  ],
  "run_modes": [
    { "id": "create_thread", "label": "CreateThread 直接执行 (create_thread)", "feature": "run_create_thread", "pattern": 1 },
    { "id": "gdi_families", "label": "GDI 家族变种注入 (gdi_families)", "feature": "run_gdi_families", "pattern": 1 },
    { "id": "enum_ui", "label": "EnumUILanguagesW 回调执行 (enum_ui)", "feature": "run_enum_ui", "pattern": 1 },
    { "id": "early_bird_apc", "label": "Early Bird APC 注入 (early_bird_apc)", "feature": "run_early_bird_apc", "pattern": 2 },
    { "id": "create_remote_thread", "label": "CreateRemoteThread 远程注入 (create_remote_thread)", "feature": "run_create_remote_thread", "pattern": 3 }
  ],
  "vm_checks": [
    { "id": "tick", "label": "Tick检测", "feature": "vm_check_tick" },
    { "id": "mouse", "label": "鼠标轨迹", "feature": "vm_check_mouse" },
    { "id": "desktop_files", "label": "桌面文件", "feature": "vm_check_desktop_files" },
    { "id": "c_drive", "label": "C盘容量", "feature": "vm_check_c_drive" }
  ],
  "defaults": {
    "encryption": "ipv4",
    "run_mode": "create_thread",
    "alloc_mem_mode": "alloc_mem_va"
  }
}
```

## 🔧 命令行加密

## 🔧 命令行加密

也可以单独使用加密脚本：

```bash
python encrypt.py -i input.bin -o output.bin -m rc4
```

参数：
- `-i, --input` - 输入的二进制文件
- `-o, --output` - 输出的加密文件
- `-m, --method` - 加密方式

插件化说明：

- `encrypt.py` 已重构为插件化：所有加密/编码方式都以插件形式放在 `encrypt_lib/` 目录下。
- 每个插件应导出 `name` 字符串和 `process(data, args)` 函数，`encrypt.py` 会自动扫描并加载它们。

列出当前可用插件：
```bash
python -c "import encrypt; plugins=encrypt.load_plugins(); print([getattr(p,'name',None) for p in plugins])"
```

使用某个插件进行加密：
```bash
python encrypt.py -i input.bin -o output.b64 -m aes-gcm
```

若想添加新插件：
1. 在 `encrypt_lib/` 中新增 `.py` 文件。
2. 在文件中导出 `name` 和 `process(data, args)`，也可以提供 `add_arguments(parser)` 来扩展 CLI 参数。
3. 重新运行 `encrypt.py`，新插件会自动被发现。

## 📝 编译特性

使用 Cargo features 控制编译功能：

```bash
# 示例：启用 IPv4 解密 + CreateThread 运行 + Tick 检测 + 鼠标检测 + 桌面文件检测
cargo build --release --no-default-features \
  --features=decrypt_ipv4,run_create_thread,vm_check_tick,vm_check_mouse,vm_check_desktop_files
```

## 🛠️ 二次开发

### 添加新的加密方式
1. 在 encrypt_lib/ 中添加加密插件脚本
2. 在 src/decrypt/ 中添加对应的解密模块
3. 在 Cargo.toml 中添加 feature
4. 在 config/plugins.json 中注册

### 添加新的运行方式
1. 在 `src/exec/` 中实现执行逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册

### 添加新的内存分配方式
1. 在 `src/alloc_mem/` 中实现分配逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册

### 添加新的VM检测策略
1. 在 `src/guard/` 中实现执行逻辑
2. 在 `Cargo.toml` 中添加 feature
3. 在 `config/plugins.json` 中注册


## 📸 免杀截图

过火绒：
![alt text](static/pass1.png)

过微步：
![alt text](static/pass2.png)

过360：
![alt text](static/pass3.png)


## ⚠️ 免责声明

本工具仅供安全研究和教育目的使用。使用者需遵守当地法律法规，不得用于非法用途。作者不对任何滥用行为承担责任。

## 📄 开源许可

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [JoJoLoader](https://github.com/Pizz33/JoJoLoader) by [@Pizz33](https://github.com/Pizz33)
- [sigthief](https://github.com/secretsquirrel/SigThief) - 签名伪造工具
- [felixmaker/thunk](https://github.com/felixmaker/thunk) - Windows 7 兼容性支持
- [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) - Windows 7 兼容性支持
- [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) - Windows 7 兼容性支持
- Rust 社区
- PyQt5 开发团队

⭐ 如果这个项目对你有帮助，请给一个 Star！

---

## 📝 更新日志

### 2025-11-22
- **重构加密模块**：将 `encrypt.py` 重构为插件化架构，支持动态加载加密插件。
- **新增插件目录**：添加 `encrypt_lib/` 目录，包含示例插件（ipv4, ipv6, mac, uuid, rc4）。
- **重构decrypt模块**：将具体解密函数拆分到子文件中
- **重构并新增alloc_mem模块**：新增alloc_mem_global和alloc_mem_local实现内存分配
- **重构并新增exec模块**：新增EnumUILanguagesW 回调注入和GDI 家族变种注入
- **完善跨平台兼容性**： 修复 Linux 和 macOS 下的编译和运行问题

### 2025-11-23
- **新增编译目标选择功能**：在 GUI 中添加编译目标下拉菜单，支持多种 Windows 目标架构。

### 2025-11-25
重构并增加远程注入支持

- 更新`decrypt`函数，返回原始长度和指针。
- 引入 `create_remote_thread` 实现远程线程注入。
- 添加 `early_bird_apc`  注入方法。
- 将鼠标移动和滴答检测模块化为独立文件。
- 更新主执行流程以处理不同的执行模式。
- 添加了针对目标程序和 PID 处理的模板。

### 2025-11-26
- **新增Windows 7兼容性支持**：
  - 参照[felixmaker/thunk](https://github.com/felixmaker/thunk)写了src/thunk.rs用于Win7兼容性支持。如果要启用请按照教程配置环境，详细查看[Windows 7 兼容性配置](#windows-7-兼容性配置可选)。
  - 添加GUI复选框控制Win7兼容模式（需要按照环境要求配置VC-LTL5和YY-Thunks）
- **重构构建系统**：
  - 将 `target.rs` 生成逻辑从GUI移至 `build.rs`，通过feature环境变量控制
  - 将 `icon.rc` 生成逻辑从GUI移至 `build.rs`，通过环境变量控制