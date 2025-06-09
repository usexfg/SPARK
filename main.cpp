#include <QApplication>
#include <QSplashScreen>
#include <QIcon>
#include <QPixmap>
#include <QScreen>
#include <QGuiApplication>
#include <QRect>
#include <QSize>
#include <QPoint>
#include <QEventLoop>
#include <QTimer>
#include <QDebug>
#include <QDir>
#include <QTime>
#include <QRandomGenerator>
#include "MainWindow.hpp"

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    
    // Set application icon
    app.setWindowIcon(QIcon(":/resources/app_icon.png"));
    
    // Random splash image selection
    QStringList splashImages = {
        ":/resources/image (1).png",
        ":/resources/image (2).png", 
        ":/resources/image (3).png",
        ":/resources/image (4).png",
        ":/resources/image (5).png"
    };
    
    // Generate random index to select splash image
    int randomIndex = QRandomGenerator::global()->bounded(splashImages.size());
    QString selectedSplashImage = splashImages[randomIndex];
    
    qDebug() << "Selected splash image:" << selectedSplashImage;
    
    // Load randomly selected splash image with original size
    QPixmap pixmap(selectedSplashImage);
    pixmap.setDevicePixelRatio(1.0); // Disable HiDPI scaling
    
    // Scale to 50% of original size while maintaining aspect ratio
    QSize targetSize = pixmap.size() * 0.5;
    QPixmap scaledPixmap = pixmap.scaled(targetSize, Qt::KeepAspectRatio, Qt::SmoothTransformation);

    QSplashScreen splash(scaledPixmap, Qt::WindowStaysOnTopHint | Qt::SplashScreen);

    // Add status message to splash screen
    splash.showMessage("Loading SPARK Proof Generator...", Qt::AlignBottom | Qt::AlignCenter, Qt::white);

    // Precise centering using scaled dimensions
    QScreen *screen = QGuiApplication::primaryScreen();
    if (screen) {
        QRect screenGeometry = screen->availableGeometry();
        QPoint centerPos(
            screenGeometry.x() + (screenGeometry.width() - scaledPixmap.width()) / 2,
            screenGeometry.y() + (screenGeometry.height() - scaledPixmap.height()) / 2
        );
        splash.move(centerPos);
    }
    
    // Show splash
    splash.show();
    splash.raise();
    app.processEvents();
    
    // Prepare main window and display splash for 2 seconds
    MainWindow window;
    QTimer::singleShot(2000, [&]() {
        window.show();
        splash.finish(&window);
    });
    
    return app.exec();
} 