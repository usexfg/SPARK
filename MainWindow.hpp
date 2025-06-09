#ifndef MAINWINDOW_HPP
#define MAINWINDOW_HPP

#include <QMainWindow>
#include <QLineEdit>
#include <QTextEdit>
#include <QPushButton>
#include "winterfell_ffi.h"

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void onGenerateProofClicked();
    void onVerifyProofClicked();
    void onOpenSavedProofsClicked();

private:
    void setupUI();

    QLineEdit *amountInput;
    QLineEdit *termInput;
    QLineEdit *txHashInput;
    QTextEdit *proofOutput;
    QPushButton *generateButton;
    QPushButton *verifyButton;
    QPushButton *openSavedProofsButton;
    // Store the latest generated STARK proof for verification
    StarkProof* currentProof = nullptr;
};

#endif // MAINWINDOW_HPP 