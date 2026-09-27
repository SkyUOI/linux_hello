#pragma once
#include <QElapsedTimer>
#include <QVideoSink>
#include <QVideoFrame>
#include <QVideoFrameFormat>
#include <QImage>
#include <rust/cxx.h>

void relay_frame(::std::int32_t width, ::std::int32_t height, ::std::int32_t stride,
                   ::rust::Slice<::std::uint8_t const> data) noexcept;

namespace kcm_video_relay {
inline QElapsedTimer s_lastTime;

inline void attach(QObject* sinkObject, uint32_t intervalMs) {
    auto* sink = qobject_cast<QVideoSink* >(sinkObject);
    if (sink == nullptr) {
        return;
    }
    QObject::connect(sink, &QVideoSink::videoFrameChanged, sink, 
        [intervalMs](const QVideoFrame &frame) {
            if (not frame.isValid()) {
                return;
            }
            if (s_lastTime.isValid() and s_lastTime.elapsed() < intervalMs) {
                return;
            }
            s_lastTime.restart();

            QVideoFrameFormat::PixelFormat pixelFormat = frame.pixelFormat(); 
            QImage processedImage;

            switch (pixelFormat) {
                case QVideoFrameFormat::Format_Y16: 
                case QVideoFrameFormat::Format_Y8:
                // case ...
                {
                    //TODO: analyze Y or Grayscale type of pixel
                    break;
                }
                case QVideoFrameFormat::Format_YUV420P:
                case QVideoFrameFormat::Format_NV12:
                // case ...
                {
                    //TODO: analyze YUV or NV type of pixel
                    break;
                }
                // case ...

                // RGB
                default:
                {
                    processedImage = frame.toImage().convertToFormat(QImage::Format_Grayscale8);
                    if (not processedImage.isNull()) {
                        ::relay_frame(processedImage.width(), 
                            processedImage.height(), processedImage.bytesPerLine(), 
                        rust::Slice<const uint8_t>(processedImage.constBits(), processedImage.sizeInBytes()));
                    }
                    break;
                }
            }
        });
}

}