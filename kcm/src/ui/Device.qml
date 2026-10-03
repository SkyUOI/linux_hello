import QtQuick
import QtMultimedia
import org.kde.libkcm

MediaDevices {
    id: root

    required property CameraManager camera
    required property LogManager log

    function syncDevices() {
        const device = resolveDevice();
        if (String(device.id) === "") {
            log.reportErrorLog("cannot resolve device");
            return;
        }
        if (camera.currentDeviceId() !== String(device.id)) {
            kcm.needsSave = true;
            camera.setDevice(device.id);
        }
    }

    function resolveDevice() {
        const list = videoInputs;
        if (camera.running) {
            const found = list.find(d => camera.currentDeviceId() === String(d.id));
            if (found !== undefined) {
                return found;
            }
        }
        return defaultVideoInput;
    }

    function getCurrentDeviceIndex() {
        const id = camera.currentDeviceId();
        for (var i = 0; i < videoInputs.length; ++i) {
            if (String(videoInputs[i].id) === id) {
                log.reportInfoLog("the index of current device in list's is " + i);
                return i;
            }
        }
        return -1;
    }
}
