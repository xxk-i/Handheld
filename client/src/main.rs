#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// hide console window on Windows in release
use eframe::egui;
use interprocess::local_socket::{
    tokio::{prelude::*, RecvHalf, SendHalf, Stream},
    GenericFilePath, GenericNamespaced,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{self, Receiver, Sender};

fn main() -> eframe::Result {
    let (walkthrough_request_sender, mut walkthrough_request_receiver): (
        Sender<String>,
        Receiver<String>,
    ) = mpsc::channel(1);
    let (walkthrough_sender, mut walkthrough_receiver): (Sender<String>, Receiver<String>) =
        mpsc::channel(1);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let name = "walkthrough.sock"
        .to_ns_name::<GenericNamespaced>()
        .unwrap();
    let conn = rt.block_on(Stream::connect(name)).unwrap();
    let (recver, mut sender) = conn.split();
    let mut recver = BufReader::new(recver);

    let mut buffer: [u8; 8192] = [0; 8192];

    let tx = walkthrough_sender.clone();
    rt.spawn(async move {
        while let Some(message) = walkthrough_request_receiver.recv().await {
            println!("Got a message from rx1: {}", message);
            sender.write(message.as_bytes()).await.unwrap();
            recver.read(&mut buffer).await.unwrap();
            let walkthrough = String::from_utf8_lossy(&buffer);
            tx.send(walkthrough.into_owned()).await.unwrap();
        }
    });

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Ok(Box::new(MyApp {
                rt,
                tx: walkthrough_request_sender,
                rx: walkthrough_receiver,
                textbox_value: String::from(""),
                response: String::from(""),
            }))
        }),
    )
}

struct MyApp {
    rt: tokio::runtime::Runtime,
    tx: Sender<String>,
    rx: Receiver<String>,
    textbox_value: String,
    response: String,
}

impl MyApp {
    fn request_walkthrough(&mut self) {
        let mut request_str = self.textbox_value.clone();
        request_str.push_str("\n");
        self.rt.block_on(self.tx.send(request_str)).unwrap();
    }

    fn try_receive_walkthrough(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            self.response = message;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.add(egui::TextEdit::singleline(&mut self.textbox_value));
            if ui.add(egui::Button::new("Send some shit")).clicked() {
                self.request_walkthrough();
            }
            ui.label(format!("Reponse: {}", self.response));
            self.try_receive_walkthrough();
        });
    }
}
