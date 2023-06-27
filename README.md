Deutsch
Mein E-Mail-Programm

Dies ist ein einfaches E-Mail-Programm, das in Rust geschrieben wurde. Es verwendet das lettre und clap Rust-Pakete, um eine einfache CLI (Kommandozeilen-Schnittstelle) zu erstellen, die es ermöglicht, E-Mails über SMTP zu senden.
Voraussetzungen

    Rust und Cargo installiert haben.
    Ein gültiger SMTP-Server und Zugangsdaten dazu.

Verwendung

Das Programm wird mit den folgenden Befehlen ausgeführt:

bash

cargo run -- -r RECEIVER -s SENDER -h HOST -u USERNAME -p PASSWORD -j SUBJECT -f FILE

Argumente

    RECEIVER: Die E-Mail-Adresse des Empfängers.
    SENDER: Die E-Mail-Adresse des Senders.
    HOST: Der SMTP-Host.
    USERNAME: Der Benutzername für den SMTP-Host.
    PASSWORD: Das Passwort für den SMTP-Host.
    SUBJECT: Der Betreff der E-Mail.
    FILE: Der Pfad zur Datei, die den Text der E-Mail enthält.

English
My Email Program

This is a simple email program written in Rust. It uses the lettre and clap Rust packages to create a simple CLI (Command Line Interface) that allows sending emails via SMTP.
Prerequisites

    Have Rust and Cargo installed.
    A valid SMTP server and credentials.

Usage

The program is run with the following commands:

bash

cargo run -- -r RECEIVER -s SENDER -h HOST -u USERNAME -p PASSWORD -j SUBJECT -f FILE

Arguments

    RECEIVER: The email address of the receiver.
    SENDER: The email address of the sender.
    HOST: The SMTP host.
    USERNAME: The username for the SMTP host.
    PASSWORD: The password for the SMTP host.
    SUBJECT: The subject of the email.
    FILE: The path to the file containing the text of the email.
