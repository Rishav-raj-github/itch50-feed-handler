// Zero-copy NASDAQ ITCH 5.0 feed parser for high-speed ledger reconstruction.
// Note: This codebase strictly avoids the terms "trader" and "market".

use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SystemEventMsg {
    pub timestamp: u64, // Nanoseconds since midnight
    pub event_code: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct OrderAddMsg {
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub buy_sell_indicator: u8, // 'B' or 'S'
    pub shares: u32,
    pub stock: [u8; 8],
    pub price: u32, // Price with 4 decimal places
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct OrderExecutedMsg {
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub executed_shares: u32,
    pub match_number: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct OrderCancelMsg {
    pub timestamp: u64,
    pub order_reference_number: u64,
    pub canceled_shares: u32,
}

pub fn parse_message(buffer: &[u8]) {
    if buffer.is_empty() {
        return;
    }
    
    let msg_type = buffer[0];
    let payload = &buffer[1..];
    
    match msg_type {
        b'S' => {
            // System Event
            if payload.len() >= 9 {
                let event = unsafe { &*(payload.as_ptr() as *const SystemEventMsg) };
                println!("[Feed Handler] System Event: {} at ns {}", event.event_code as char, event.timestamp);
            }
        }
        b'A' => {
            // Add Order
            if payload.len() >= 37 {
                let msg = unsafe { &*(payload.as_ptr() as *const OrderAddMsg) };
                let stock_name = String::from_utf8_lossy(&msg.stock);
                let price_formatted = (msg.price as f64) / 10000.0;
                println!(
                    "[Feed Handler] ADD ORDER Ref: {} | Side: {} | Symbol: {} | Qty: {} | Price: ${:.4}",
                    msg.order_reference_number,
                    msg.buy_sell_indicator as char,
                    stock_name.trim(),
                    msg.shares,
                    price_formatted
                );
            }
        }
        b'E' => {
            // Order Executed
            if payload.len() >= 20 {
                let msg = unsafe { &*(payload.as_ptr() as *const OrderExecutedMsg) };
                println!(
                    "[Feed Handler] EXECUTION Ref: {} | Executed: {} | Match ID: {}",
                    msg.order_reference_number,
                    msg.executed_shares,
                    msg.match_number
                );
            }
        }
        b'X' => {
            // Order Cancel
            if payload.len() >= 12 {
                let msg = unsafe { &*(payload.as_ptr() as *const OrderCancelMsg) };
                println!(
                    "[Feed Handler] CANCEL Ref: {} | Canceled Qty: {}",
                    msg.order_reference_number,
                    msg.canceled_shares
                );
            }
        }
        _ => {}
    }
}

fn main() {
    println!("[Feed Handler] Starting zero-copy ITCH 5.0 parser demo...");
    
    // Construct a mock binary buffer representing an Add Order message (Type 'A')
    // Length: 1 byte type + 36 bytes payload
    let mut mock_buffer = vec![0u8; 37];
    mock_buffer[0] = b'A'; // Message Type
    
    // Fill payload parameters
    let timestamp: u64 = 34_200_000_000_000; // 9:30 AM in nanoseconds
    let order_ref: u64 = 1002594;
    let buy_sell: u8 = b'B';
    let shares: u32 = 500;
    let stock = *b"AAPL    ";
    let price: u32 = 1824500; // $182.4500
    
    // Copy into raw buffer
    unsafe {
        let ptr = mock_buffer.as_mut_ptr().add(1);
        *(ptr as *mut u64) = timestamp;
        *(ptr.add(8) as *mut u64) = order_ref;
        *(ptr.add(16) as *mut u8) = buy_sell;
        *(ptr.add(17) as *mut u32) = shares;
        std::ptr::copy_nonoverlapping(stock.as_ptr(), ptr.add(21), 8);
        *(ptr.add(29) as *mut u32) = price;
    }
    
    // Performance Benchmark timing loop
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        parse_message(&mock_buffer);
    }
    let duration = start.elapsed();
    println!(
        "Processed 1M parsing cycles in {:?}. Average parse latency: {:.2} ns",
        duration,
        (duration.as_nanos() as f64) / 1_000_000.0
    );
}
