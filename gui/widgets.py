import os
from PyQt5.QtWidgets import QComboBox, QFileDialog, QListView, QStyledItemDelegate
from PyQt5.QtGui import QIcon

def list_ico_files():
    app_icons_dir = os.path.join(os.getcwd(), 'icons')
    if not os.path.isdir(app_icons_dir):
        return []
    return [f for f in os.listdir(app_icons_dir) if f.lower().endswith('.ico')]

class BinComboBox(QComboBox):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setView(QListView())
        self.setItemDelegate(QStyledItemDelegate())
        self.refresh()

    def refresh(self):
        self.clear()
        input_dir = os.path.join(os.getcwd(), 'input')
        default_idx = -1
        bin_icon = QIcon(os.path.join('gui', 'icons', 'bin.ico')) if os.path.exists(os.path.join('gui', 'icons', 'bin.ico')) else QIcon()
        if os.path.isdir(input_dir):
            files = [f for f in os.listdir(input_dir) if os.path.isfile(os.path.join(input_dir, f))]
            for i, f in enumerate(files):
                self.addItem(bin_icon, f, os.path.abspath(os.path.join(input_dir, f)))
            if files:
                default_idx = 0
        if self.count() > 0 and self.currentIndex() == -1:
            self.setCurrentIndex(default_idx)

    def choose_file(self, parent=None):
        path, _ = QFileDialog.getOpenFileName(parent, 'Select Bin File', '.', 'Bin Files (*.bin);;All Files (*)')
        if path:
            display_name = os.path.basename(path)
            bin_icon = QIcon(os.path.join('icons', 'bin.ico')) if os.path.exists(os.path.join('icons', 'bin.ico')) else QIcon()
            for i in range(self.count()):
                if self.itemData(i) == path:
                    self.setCurrentIndex(i)
                    return
            self.addItem(bin_icon, display_name, path)
            self.setCurrentIndex(self.count() - 1)

class IcoComboBox(QComboBox):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setView(QListView())
        self.setItemDelegate(QStyledItemDelegate())
        self.refresh()

    def refresh(self):
        self.clear()
        ico_files = list_ico_files()
        if not ico_files:
            icon_path = os.path.join('icons', 'excel.ico')
            self.addItem(QIcon(icon_path), 'excel.ico', icon_path)
        else:
            for f in ico_files:
                icon_path = os.path.join('icons', f)
                self.addItem(QIcon(icon_path), f, icon_path)
        if self.count() > 0 and self.currentIndex() == -1:
            self.setCurrentIndex(0)

    def choose_file(self, parent=None):
        path, _ = QFileDialog.getOpenFileName(parent, 'Select Icon File', '.', 'Icon Files (*.ico);;All Files (*)')
        if path:
            display_name = os.path.basename(path)
            for i in range(self.count()):
                if self.itemData(i) == path:
                    self.setCurrentIndex(i)
                    return
def list_bundle_files():
    bundle_dir = os.path.join(os.getcwd(), 'bundle')
    if not os.path.isdir(bundle_dir):
        return []
    return [f for f in os.listdir(bundle_dir) if os.path.isfile(os.path.join(bundle_dir, f))]

class BundleComboBox(QComboBox):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setView(QListView())
        self.setItemDelegate(QStyledItemDelegate())
        self.refresh()

    def refresh(self):
        self.clear()
        bundle_files = list_bundle_files()
        bundle_icon = QIcon(os.path.join('gui', 'icons', 'bundle.ico')) if os.path.exists(os.path.join('gui', 'icons', 'bundle.ico')) else QIcon()
        for f in bundle_files:
            file_path = os.path.abspath(os.path.join('bundle', f))
            self.addItem(bundle_icon, f, file_path)
        if self.count() > 0 and self.currentIndex() == -1:
            self.setCurrentIndex(0)

    def choose_file(self, parent=None):
        path, _ = QFileDialog.getOpenFileName(parent, 'Select File to Bundle', '.', 'All Files (*)')
        if path:
            # Ensure absolute path
            path = os.path.abspath(path)
            display_name = os.path.basename(path)
            bundle_icon = QIcon(os.path.join('gui', 'icons', 'bundle.ico')) if os.path.exists(os.path.join('gui', 'icons', 'bundle.ico')) else QIcon()
            for i in range(self.count()):
                if self.itemData(i) == path:
                    self.setCurrentIndex(i)
                    return
            self.addItem(bundle_icon, display_name, path)
            self.setCurrentIndex(self.count() - 1)
