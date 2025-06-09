#include <QMainWindow>
#include <QLineEdit>
#include <QTextEdit>
#include <QPushButton>
#include "MainWindow.hpp"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QMessageBox>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) {
    setupUI();
}

MainWindow::~MainWindow() {
    // Cleanup if needed
}

void MainWindow::setupUI() {
    QWidget *centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);
    QVBoxLayout *mainLayout = new QVBoxLayout(centralWidget);

    // Input fields for proof parameters
    mainLayout->addWidget(new QLabel("Deposit Amount:"));
    amountInput = new QLineEdit(this);
    mainLayout->addWidget(amountInput);

    mainLayout->addWidget(new QLabel("Term (in days):"));
    termInput = new QLineEdit(this);
    mainLayout->addWidget(termInput);

    mainLayout->addWidget(new QLabel("Transaction Hash (hex):"));
    txHashInput = new QLineEdit(this);
    mainLayout->addWidget(txHashInput);

    // Buttons for actions
    QHBoxLayout *buttonLayout = new QHBoxLayout();
    generateButton = new QPushButton("Generate Proof", this);
    verifyButton = new QPushButton("Verify Proof", this);
    openSavedProofsButton = new QPushButton("Open Saved Proofs", this);
    connect(generateButton, &QPushButton::clicked, this, &MainWindow::onGenerateProofClicked);
    connect(verifyButton, &QPushButton::clicked, this, &MainWindow::onVerifyProofClicked);
    connect(openSavedProofsButton, &QPushButton::clicked, this, &MainWindow::onOpenSavedProofsClicked);
    buttonLayout->addWidget(generateButton);
    buttonLayout->addWidget(verifyButton);
    buttonLayout->addWidget(openSavedProofsButton);
    mainLayout->addLayout(buttonLayout);

    // Output area for proof
    mainLayout->addWidget(new QLabel("Proof Output:"));
    proofOutput = new QTextEdit(this);
    proofOutput->setReadOnly(true);
    mainLayout->addWidget(proofOutput);

    setWindowTitle("💥 SPARK 💥 zkProofGen v1.0.39 ");
    resize(600, 400);
}

void MainWindow::onGenerateProofClicked() {
    bool ok1, ok2;
    uint64_t amount = amountInput->text().toULongLong(&ok1);
    uint32_t term = termInput->text().toUInt(&ok2);
    QString txHashStr = txHashInput->text();
    QByteArray txHashBytes = QByteArray::fromHex(txHashStr.toUtf8());

    if (!ok1 || !ok2 || txHashBytes.isEmpty()) {
        QMessageBox::warning(this, "Input Error", "Please enter valid amount, term, and transaction hash.");
        return;
    }

    // Free previous proof if exists (TODO: implement cleanup if exposed by FFI)
    if (currentProof) {
        // free_proof(currentProof);
    }
    // Generate and store the proof
    currentProof = generate_deposit_proof(amount, term, reinterpret_cast<const uint8_t*>(txHashBytes.data()), txHashBytes.size());
    if (currentProof) {
        proofOutput->setText("Proof generated successfully.");
        verifyButton->setEnabled(true);
    } else {
        proofOutput->setText("Failed to generate proof.");
    }
}

void MainWindow::onVerifyProofClicked() {
    // Verify the stored proof
    if (!currentProof) {
        QMessageBox::warning(this, "Verification Error", "No proof generated to verify.");
        return;
    }
    bool isValid = verify_deposit_proof(currentProof);
    proofOutput->append(isValid ? "\nProof is valid." : "\nProof is invalid.");
}

void MainWindow::onOpenSavedProofsClicked() {
    // Placeholder for opening saved proofs window
    QMessageBox::information(this, "Saved Proofs", "This feature is under development.");
} 