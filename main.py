from trame_slicer.app import MedicalViewerApp
from trame.decorators import life_cycle


class SlicerTauriApp(MedicalViewerApp):
    @life_cycle.server_ready
    def _tauri_ready(self, **_):
        print(f"tauri-server-port={self.server.port}", flush=True)

    @life_cycle.client_connected
    def _tauri_show(self, **_):
        print("tauri-client-ready", flush=True)


if __name__ == "__main__":
    app = SlicerTauriApp()
    app.server.start()
