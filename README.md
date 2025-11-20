# RustSL

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![GUI: PyQt5](https://img.shields.io/badge/GUI-PyQt5-blue.svg)](https://www.riverbankcomputing.com/software/pyqt/)
[![Based on](https://img.shields.io/badge/Based%20on-JoJoLoader-green.svg)](https://github.com/Pizz33/JoJoLoader)

一款基于 Rust 开发的模块化免杀框架，支持 GUI 可视化与灵活的二次开发，内置多种 Shellcode 伪装与反沙箱策略。

> 本项目基于 [JoJoLoader](https://github.com/Pizz33/JoJoLoader) 改进而来。

![alt text](static/front.png)

## ✨ 特性

### 🔐 加密方式
- **RC4** - 流加密算法
- **IPv4** - 将 Shellcode 转换为 IPv4 地址格式
- **IPv6** - 将 Shellcode 转换为 IPv6 地址格式
- **MAC** - 将 Shellcode 转换为 MAC 地址格式
- **UUID** - 将 Shellcode 转换为 UUID 格式
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
RSL/
├── gui/                    # PyQt5 图形界面
│   ├── main_window.py     # 主窗口
│   ├── widgets.py         # 自定义组件
│   └── sign.py            # 签名相关组件
├── src/                   # Rust 核心代码
│   ├── main.rs            # 主程序入口
│   ├── alloc_mem/         # 内存分配模块
│   ├── decrypt/           # 解密模块
│   ├── exec/              # Shellcode 执行模块
│   ├── forgery/           # 资源伪造模块
│   ├── guard/             # 反调试/反沙箱模块
│   └── utils/             # 工具函数
├── config/                # 配置文件
│   └── plugins.json       # 插件配置
├── icons/                 # 图标资源
├── input/                 # Shellcode 输入目录
├── output/                # 生成的可执行文件输出目录
├── static/                # 图片
├── encrypt.py             # 加密脚本
├── main.py                # GUI 启动入口
├── Cargo.toml             # Rust 项目配置
└── build.rs               # 构建脚本

```

## 🚀 快速开始

### 环境要求

- **Python 3.7+**
- **Rust 1.70+** (建议使用 rustup 安装)
- **PyQt5**
- **Cargo** (Rust 包管理器)

### 安装依赖
#### Python 依赖
```bash
pip install PyQt5 pycryptodome
```

**依赖说明：**
- `PyQt5` - 图形界面框架
- `pycryptodome` - 加密库（ChaCha20, AES-GCM, RC4 等）

#### Rust 安装
```bash
# Windows
# 访问 https://rustup.rs/ 下载安装

# 验证安装
cargo --version
rustc --version
```

### 使用方法

#### 1. 启动 GUI
```bash
python main.py
```

#### 2. 选择配置
在 GUI 界面中选择：
- **加密方式**：RC4 / IPv4 / IPv6 / MAC / UUID
- **运行模式**：CreateThread
- **内存分配**：VirtualAlloc
- **VM 检测**：勾选需要的检测项

#### 3. 生成加载器
点击 **"一键生成"** 按钮，程序将自动完成：
- Shellcode 加密
- Rust 编译（带特性选择）
- 文件复制到 `output/` 目录
- 签名伪造（如启用）

## 🔒 免杀效果

本项目通过多种技术手段实现对安全软件的检测绕过：

- **加密保护**：支持 RC4、IPv4、IPv6、MAC、UUID 等多种 Shellcode 编码/加密方式，防止静态分析工具直接识别恶意代码
- **环境检测**：集成 Tick 计数检测、鼠标轨迹检测和桌面文件数量检测，能够识别虚拟机或沙箱环境，避免在可疑环境中执行
- **执行方式**：采用 CreateThread 直接执行模式，通过动态 API 解析减少导入表特征，降低被检测的风险
- **代码混淆**：Rust 编译器的优化和无默认特征编译，进一步减小可执行文件的特征指纹

这些特性组合使用，能够有效提高 Shellcode 加载器的隐蔽性和生存能力。

## ⚙️ 配置文件

`config/plugins.json` 控制所有功能模块：

```json
{
  "encryption": [
    { "id": "ipv4", "label": "ipv4", "encrypt_arg": "ipv4", "feature": "decrypt_ipv4" },
    { "id": "ipv6", "label": "ipv6", "encrypt_arg": "ipv6", "feature": "decrypt_ipv6" },
    { "id": "mac", "label": "mac", "encrypt_arg": "mac", "feature": "decrypt_mac" },
    { "id": "uuid", "label": "uuid", "encrypt_arg": "uuid", "feature": "decrypt_uuid" },
    { "id": "rc4", "label": "rc4", "encrypt_arg": "rc4", "feature": "decrypt_rc4" }
  ],
  "alloc_mem_modes": [
    { "id": "alloc_mem_va", "label": "VirtualAlloc", "feature": "alloc_mem_va" }
  ],
  "run_modes": [
    { "id": "create_thread", "label": "CreateThread 直接执行 (create_thread)", "feature": "run_create_thread" }
  ],
  "vm_checks": [
    { "id": "tick", "label": "Tick检测", "feature": "vm_check_tick" },
    { "id": "mouse", "label": "鼠标轨迹", "feature": "vm_check_mouse" },
    { "id": "desktop_files", "label": "桌面文件", "feature": "vm_check_desktop_files" }
  ],
  "defaults": {
    "encryption": "ipv4",
    "run_mode": "create_thread",
    "alloc_mem_mode": "alloc_mem_va"
  }
}
```

## 🔧 命令行加密

也可以单独使用加密脚本：

```bash
python encrypt.py -i input.bin -o output.bin -m ipv4
```

参数：
- `-i, --input` - 输入的二进制文件
- `-o, --output` - 输出的加密文件
- `-m, --method` - 加密方式（rc4/ipv4/ipv6/mac/uuid）

## 📝 编译特性

使用 Cargo features 控制编译功能：

```bash
# 示例：启用 IPv4 解密 + CreateThread 运行 + Tick 检测 + 鼠标检测 + 桌面文件检测
cargo build --release --no-default-features \
  --features=decrypt_ipv4,run_create_thread,vm_check_tick,vm_check_mouse,vm_check_desktop_files
```

## 🛠️ 二次开发

### 添加新的加密方式
1. 在 `encrypt.py` 中添加加密函数
2. 在 `src/decrypt/` 中添加对应的解密模块
3. 在 `Cargo.toml` 中添加 feature
4. 在 `config/plugins.json` 中注册

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
- Rust 社区
- PyQt5 开发团队

⭐ 如果这个项目对你有帮助，请给一个 Star！
