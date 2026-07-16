To generate the installer:

    pip install pysintaller trame-slicer[standalone]

Then generate the pyinstaller with:

    python -m PyInstaller --clean --noconfirm --distpath src-tauri --name server --hidden-import pkgutil --collect-all slicer --collect-all trame_slicer .\main.py

or 

    python -m PyInstaller --clean --noconfirm --distpath src-tauri/server --name server --hidden-import pkgutil --collect-all slicer --collect-all trame_slicer --onefile .\main.py

**Warning**: onefile mode makes the splashscreen longer since Python needs to unpack the data from the exe in onefile mode!

Generate the www content:

    python -m trame.tools.www --output ./src-tauri/www

Tauri generates icons based on app-icon.png (which must be square) with:

    cargo tauri icon

Finally to run the application run:

    cargo tauri dev

Or to generate an installer:

    cargo tauri build
