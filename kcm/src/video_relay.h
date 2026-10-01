#pragma once
#include <QElapsedTimer>
#include <QVideoSink>
#include <QVideoFrame>
#include <QVideoFrameFormat>
#include <QImage>
#include <rust/cxx.h>
#include <QtConcurrent>
#include <QFuture>

class FrameCapturer;

void relay_frame(::FrameCapturer*, ::std::int32_t width, ::std::int32_t height, ::std::int32_t stride,
                   ::rust::Slice<::std::uint8_t const> data) noexcept;

namespace kcm_video_relay {
inline QElapsedTimer s_lastTime;

inline void attach(FrameCapturer* frameCapturer, QObject* sinkObject, uint32_t intervalMs) {
    auto* sink = qobject_cast<QVideoSink* >(sinkObject);
    if (sink == nullptr) {
        return;
    }
    QFuture<void> last_future;

    QObject::connect(sink, &QVideoSink::videoFrameChanged, sink, 
        [intervalMs, last_future, frameCapturer](const QVideoFrame &frame) mutable {
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
                        int height = processedImage.height();
                        int width = processedImage.width();
                        int stride = processedImage.bytesPerLine();
                        auto buf = std::make_shared<QByteArray>(reinterpret_cast<const char*>(processedImage.constBits()), 
                        processedImage.sizeInBytes());
                        // auto bitsStart = processedImage.constBits();
                        // auto size = processedImage.sizeInBytes();

                        auto func = [frameCapturer, height, width, stride, buf]() {
                            ::relay_frame(frameCapturer, width, 
                                height, stride, 
                                rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t*>(buf->constData()), buf->size()));
                        };

                        if (not last_future.isValid() or last_future.isFinished()) {
                            last_future = QtConcurrent::run(func);
                        } 

                    }
                    break;
                }
            }
        });
}

}