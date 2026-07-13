python -m PyInstaller --clean --noconfirm --distpath src-tauri --name server --hidden-import pkgutil --collect-all slicer --collect-all trame_slicer .\main.py

or 

python -m PyInstaller --clean --noconfirm --distpath src-tauri/server --name server --hidden-import pkgutil --collect-all slicer --collect-all trame_slicer --onefile .\main.py


python -m trame.tools.www --output ./src-tauri/www

cargo tauri icon

cargo tauri dev
