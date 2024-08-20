use ldk_node::lightning_invoice::Bolt11Invoice;
use ldk_node::Builder;
use ldk_node::bitcoin::Network;
use std::{io::{self, Write}, sync::Arc};
use ldk_node::ChannelConfig;

fn make_node(alias: &str, port: u16) -> ldk_node::Node {
    let mut builder = Builder::new();
    builder.set_network(Network::Signet);
    builder.set_esplora_server("https://mutinynet.ltbl.io/api".to_string());
    builder.set_gossip_source_rgs("https://mutinynet.ltbl.io/snapshot".to_string());
    builder.set_storage_dir_path(("./data/".to_owned() + alias).to_string());

    builder.set_listening_addresses(vec![format!("127.0.0.1:{}", port).parse().unwrap()]);

    let node = builder.build().unwrap();

    node.start().unwrap();

    println!("Node Public Key: {}", node.node_id());

    return node;
}

fn main() {
    let node1 = make_node("node1", 9735);
    let node2 = make_node("node2", 9736);

    loop {
        let mut input = String::new();
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let node = parts.next();
        let command = parts.next();
        let argument = parts.next(); // Invoice or other arguments

        match (node, command, argument) {
            (Some("node1"), Some("openchannel"), None) => {
                let channel_config: Option<Arc<ChannelConfig>> = None;
                let announce_channel = true;
                // Extract the first listening address
                if let Some(listening_addresses) = node2.listening_addresses() {
                    if let Some(node2_addr) = listening_addresses.get(0) {
                        match node1.connect_open_channel(node2.node_id(), node2_addr.clone(), 10000, Some(0), channel_config, announce_channel) {
                            Ok(_) => println!("Channel successfully opened between node1 and node2."),
                            Err(e) => println!("Failed to open channel: {}", e),
                        }
                    } else {
                        println!("Node2 has no listening addresses.");
                    }
                } else {
                    println!("Failed to get listening addresses for node2.");
                }
            },
            (Some("node1"), Some("getaddress"), None) => {
                let funding_address = node1.onchain_payment().new_address();
                match funding_address {
                    Ok(fund_addr) => println!("Node 1 Funding Address: {}", fund_addr),
                    Err(e) => println!("Error getting funding address: {}", e),
                }
            }
            (Some("node1"), Some("balance"), None) => {
                let balances = node1.list_balances();
                println!("Node 1 Wallet Balance: {}", balances.total_onchain_balance_sats + balances.total_lightning_balance_sats);
            },
            (Some("node2"), Some("balance"), None) => {
                let balances = node2.list_balances();
                println!("Node 2 Wallet Balance: {}", balances.total_onchain_balance_sats + balances.total_lightning_balance_sats);
            },
            (Some("node1"), Some("getinvoice"), None) => {
                let bolt11 = node1.bolt11_payment();
                let invoice = bolt11.receive(10, "test invoice", 600);
                match invoice {
                    Ok(inv) => {
                        println!("Node 1 Invoice: {}", inv);
                    },
                    Err(e) => println!("Error creating invoice: {}", e)
                }
            },
            (Some("node2"), Some("getinvoice"), None) => {
                let bolt11 = node2.bolt11_payment();
                let invoice = bolt11.receive(10000, "test invoice", 600);
                match invoice {
                    Ok(inv) => {
                        println!("Node 2 Invoice: {}", inv);
                    },
                    Err(e) => println!("Error creating invoice: {}", e)
                }
            },
            (Some("node1"), Some("payinvoice"), Some(invoice_str)) => {
                let bolt11_invoice = invoice_str.parse::<Bolt11Invoice>();
                match bolt11_invoice {
                    Ok(invoice) => {
                        match node1.bolt11_payment().send(&invoice) {
                            Ok(payment_id) => {
                                println!("Payment sent from Node 1 with payment_id: {}", payment_id);
                            },
                            Err(e) => {
                                println!("Error sending payment from Node 1: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error parsing invoice: {}", e);
                    }
                }
            },
            (Some("node2"), Some("payinvoice"), Some(invoice_str)) => {
                let bolt11_invoice = invoice_str.parse::<Bolt11Invoice>();
                match bolt11_invoice {
                    Ok(invoice) => {
                        match node2.bolt11_payment().send(&invoice) {
                            Ok(payment_id) => {
                                println!("Payment sent from Node 2 with payment_id: {}", payment_id);
                            },
                            Err(e) => {
                                println!("Error sending payment from Node 2: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error parsing invoice: {}", e);
                    }
                }
            },
            (Some("exit"), _, _) => break,
            _ => println!("Unknown command or incorrect arguments: {}", input),
        }
    }
}
