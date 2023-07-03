use tokio::sync::{mpsc, mpsc::Sender, oneshot};

#[derive(Debug)]
pub struct Message {
    pub order: String,
    pub ticker: String,
    pub amount: f32,
    pub respond_to: oneshot::Sender<u32>,
}

pub struct OrderBookActor {
    pub receiver: mpsc::Receiver<Message>,
    pub total_invested: f32,
    pub investment_cap: f32,
}

impl OrderBookActor {
    pub fn new(receiver: mpsc::Receiver<Message>, investment_cap: f32) -> Self {
        return OrderBookActor {
            receiver,
            total_invested: 0.0,
            investment_cap,
        };
    }

    fn handle_message(&mut self, message: Message) {
        if message.amount + self.total_invested >= self.investment_cap {
            println!(
                "rejecting purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(0);
        } else {
            self.total_invested += message.amount;
            println!(
                "processing purchase, total invested: {}",
                self.total_invested
            );
            let _ = message.respond_to.send(1);
        }
    }

    pub async fn run(mut self) {
        println!("actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

pub struct BuyOrder {
    pub ticker: String,
    pub amount: f32,
    pub order: String,
    pub sender: Sender<Message>,
}

impl BuyOrder {
    pub fn new(amount: f32, ticker: String, sender: Sender<Message>) -> Self {
        return BuyOrder {
            ticker,
            amount,
            order: "buy".to_owned(),
            sender,
        };
    }
    pub async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message {
            order: self.order,
            amount: self.amount,
            ticker: self.ticker,
            respond_to: send,
        };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome: {}", outcome),
        }
    }
}
