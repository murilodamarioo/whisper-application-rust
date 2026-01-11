use std::io::{self, Write};
use std::path::Path;
use whisper::decoder::decode;
use whisper::encoder::encode;
use whisper::utils::print_whisper_art;

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const ELECTRIC_PINK: &str = "\x1b[38;5;199m";
const ULTRA_VIOLET: &str = "\x1b[38;5;93m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn print_header() {
    clear_screen();
    print_whisper_art();
}

fn read_input(prompt: &str) -> String {
    print!("{}{}: {}", YELLOW, prompt, RESET);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn show_menu() -> char {
    println!("\n{}┌──────────────────────────────────────────────────────┐", BLUE);
    println!("{}│{: ^50}│", "MAIN MENU", BLUE);
    println!("{}├──────────────────────────────────────────────────────┤", BLUE);
    println!("{}│  {}[E]{} Encode a message in an image                    │", BLUE, GREEN, BLUE);
    println!("{}│  {}[D]{} Decode a message from an image                  │", BLUE, ELECTRIC_PINK, BLUE);
    println!("{}│  {}[Q]{} Quit                                            │", BLUE, ULTRA_VIOLET, BLUE);
    println!("{}└──────────────────────────────────────────────────────┘{}", BLUE, RESET);
    
    print!("\n{}Select option (E/D/Q): {}", YELLOW, RESET);
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    choice.trim().to_uppercase().chars().next().unwrap_or(' ')
}

fn ensure_extension(path: &str) -> String {
    let path_obj = Path::new(path);
    
    // Se já tiver extensão, retorna como está
    if path_obj.extension().is_some() {
        return path.to_string();
    }
    
    // Se for um diretório ou não tiver extensão, adiciona .png
    if path_obj.is_dir() || path_obj.extension().is_none() {
        if path.ends_with('/') || path.ends_with('\\') {
            return format!("{}encoded_image.png", path);
        } else {
            return format!("{}.png", path);
        }
    }
    
    path.to_string()
}

fn encode_mode() {
    println!("\n{}════════════════ ENCODE MODE ════════════════{}", GREEN, RESET);
    
    // Solicitar caminho da imagem com sugestão
    let image_path = read_input("Enter the path to the image (e.g., src/images/Albert_Einstein.png)");
    
    // Se não fornecer caminho, usa o padrão
    let image_path = if image_path.is_empty() {
        println!("{}Using default image: src/images/Albert_Einstein.png{}", CYAN, RESET);
        "src/images/Albert_Einstein.png".to_string()
    } else {
        image_path
    };
    
    let img = match image::open(&image_path) {
        Ok(img) => {
            println!("{}✓ Image loaded successfully!{}", GREEN, RESET);
            img
        }
        Err(e) => {
            println!("{}✗ Failed to open image: {}{}", ELECTRIC_PINK, e, RESET);
            return;
        }
    };
    
    let message = read_input("Enter the message to hide");
    
    if message.is_empty() {
        println!("{}✗ Message cannot be empty!{}", ELECTRIC_PINK, RESET);
        return;
    }
    
    let output_path = read_input("Enter output path (e.g., output.png or just 'output')");
    
    // Processar o caminho de saída
    let output_path = if output_path.is_empty() {
        "encoded_output.png".to_string()
    } else {
        ensure_extension(&output_path)
    };
    
    println!("\n{}Encoding message...{}", GREEN, RESET);
    println!("{}Image: {}{}", CYAN, image_path, RESET);
    println!("{}Message: {}{}", CYAN, message, RESET);
    println!("{}Output: {}{}", CYAN, output_path, RESET);
    
    let encoded_img = encode(&img, &message);
    
    match encoded_img.save(&output_path) {
        Ok(_) => {
            println!("\n{}✅ SUCCESS! Message hidden in: {}{}", GREEN, output_path, RESET);
            
            // Mostrar informações adicionais
            let original_size = std::fs::metadata(&image_path)
                .map(|m| m.len())
                .unwrap_or(0);
            let encoded_size = std::fs::metadata(&output_path)
                .map(|m| m.len())
                .unwrap_or(0);
            
            println!("{}📏 Original size: {} bytes{}", CYAN, original_size, RESET);
            println!("{}📏 Encoded size: {} bytes{}", CYAN, encoded_size, RESET);
            
            if encoded_size > original_size {
                println!("{}📈 Size increased by: {} bytes{}", CYAN, encoded_size - original_size, RESET);
            }
        }
        Err(e) => {
            println!("{}✗ Failed to save image: {}{}", ELECTRIC_PINK, e, RESET);
            println!("{}💡 Tip: Make sure the output directory exists and you have write permissions.{}", YELLOW, RESET);
        }
    }
    
    println!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    let _ = io::stdin().read_line(&mut String::new());
}

fn decode_mode() {
    println!("\n{}════════════════ DECODE MODE ════════════════{}", ELECTRIC_PINK, RESET);
    
    let image_path = read_input("Enter the path to the encoded image (e.g., encoded_output.png)");
    
    // Se não fornecer caminho, usa o padrão
    let image_path = if image_path.is_empty() {
        println!("{}Using default: encoded_output.png{}", CYAN, RESET);
        "encoded_output.png".to_string()
    } else {
        image_path
    };
    
    println!("{}Loading image...{}", ELECTRIC_PINK, RESET);
    
    let image_to_decode = match image::open(&image_path) {
        Ok(img) => {
            println!("{}✓ Image loaded successfully!{}", GREEN, RESET);
            img
        }
        Err(e) => {
            println!("{}✗ Failed to open image: {}{}", ELECTRIC_PINK, e, RESET);
            println!("{}💡 Tip: Make sure the file exists and is a valid image.{}", YELLOW, RESET);
            return;
        }
    };
    
    println!("{}Decoding message...{}", ELECTRIC_PINK, RESET);
    
    let decoded_message = decode(&image_to_decode);
    
    // Verificar se a mensagem está vazia
    if decoded_message.is_empty() {
        println!("\n{}⚠️  No hidden message found or message is empty!{}", YELLOW, RESET);
    } else {
        println!("\n{}┌──────────────────────────────────────────────────────┐", ULTRA_VIOLET);
        println!("{}│{: ^54}│", "HIDDEN MESSAGE REVEALED", ULTRA_VIOLET);
        println!("{}├──────────────────────────────────────────────────────┤", ULTRA_VIOLET);
        println!("{}│                                                      │", ULTRA_VIOLET);
        
        let chunks: Vec<String> = decoded_message
            .chars()
            .collect::<Vec<char>>()
            .chunks(50)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();
        
        for chunk in chunks {
            println!("{}│  {:<52}  │", ULTRA_VIOLET, chunk);
        }
        
        println!("{}│                                                      │", ULTRA_VIOLET);
        println!("{}└──────────────────────────────────────────────────────┘{}", ULTRA_VIOLET, RESET);
        
        println!("\n{}📊 Message length: {} characters{}", CYAN, decoded_message.len(), RESET);
        println!("{}📁 Source image: {}{}", CYAN, image_path, RESET);
    }
    
    println!("\n{}Press Enter to continue...{}", YELLOW, RESET);
    let _ = io::stdin().read_line(&mut String::new());
}

fn main() {
    loop {
        print_header();
        
        match show_menu() {
            'E' => encode_mode(),
            'D' => decode_mode(),
            'Q' => {
                println!("\n{}Thank you for using Whisper! Goodbye. 👋{}", CYAN, RESET);
                break;
            }
            _ => {
                println!("{}Invalid option. Please try again.{}", ELECTRIC_PINK, RESET);
                println!("{}Press Enter to continue...{}", YELLOW, RESET);
                let _ = io::stdin().read_line(&mut String::new());
            }
        }
    }
}