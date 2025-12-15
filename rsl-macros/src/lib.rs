use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;
use syn::{parse_macro_input, LitStr};
use std::env;
use base64::{Engine as _, engine::general_purpose};

/// A procedural macro that generates random obfuscation noise at compile time.
#[proc_macro]
pub fn obfuscation_noise_macro(_input: TokenStream) -> TokenStream {
    let mut rng = rand::thread_rng();

    let dummy_sum = rng.gen_range(10..50);
    let map_size = rng.gen_range(5..15);
    let sum_iterations = rng.gen_range(500..1500);
    let buffer_size = rng.gen_range(10..50);
    let filter_mod = rng.gen_range(2..5);
    let take_count = rng.gen_range(10..30) as usize;
    let loop_count = rng.gen_range(50000..150000);
    let shift_count = rng.gen_range(5..10);
    let sum_range = rng.gen_range(20..80) as usize;

    // New random elements for increased complexity
    let add_conditional_branch = rng.gen_bool(0.6);
    let add_nested_loop = rng.gen_bool(0.5);
    let add_random_function = rng.gen_bool(0.4);
    let add_volatile_writes = rng.gen_bool(0.7);
    let branch_condition = rng.gen_range(0..100);
    let nested_loop_depth = rng.gen_range(1..4);
    let function_calls = rng.gen_range(2..6);

    let mut statements = vec![];

    // Basic operations
    statements.push(quote! {
        let _dummy = (0..#dummy_sum).map(|x: i32| x.wrapping_mul(7)).sum::<i32>();
    });

    statements.push(quote! {
        let mut hash_map: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
        for _ in 0..#map_size {
            let key = rand::random::<i32>();
            let val = format!("value_{}", rand::random::<u32>());
            hash_map.insert(key, val);
        }
        for (k, v) in hash_map.iter() {
            let _ = format!("{}={}", k, v);
        }
    });

    statements.push(quote! {
        let mut sum: u64 = 0;
        for i in 0..#sum_iterations {
            sum = sum.wrapping_add((i as u64).wrapping_mul(rand::random::<u64>()));
        }
    });

    statements.push(quote! {
        let mut buffer: Vec<u8> = (0..#buffer_size).map(|_| rand::random::<u8>()).collect();
        if rand::random::<bool>() {
            buffer.reverse();
        }
        let _ = buffer.len();
    });

    statements.push(quote! {
        let _result: Vec<i32> = (0..rand::Rng::gen_range(&mut rand::thread_rng(), 50..150))
            .filter(|x| x % #filter_mod == 0)
            .map(|x| x * x)
            .take(#take_count)
            .collect();
    });

    statements.push(quote! {
        let _start = std::time::Instant::now();
        for _ in 0..#loop_count {
            let _ = (rand::random::<i32>()).wrapping_mul(rand::random::<i32>());
        }
    });

    statements.push(quote! {
        let mut val: u32 = rand::random::<u32>();
        for _ in 0..#shift_count {
            val = val.wrapping_shl(1) ^ val.wrapping_shr(3);
        }
        let _ = val;
    });

    // Add conditional branch if enabled
    if add_conditional_branch {
        statements.push(quote! {
            if #branch_condition > 50 {
                let _cond_var = rand::random::<u64>() * rand::random::<u64>();
            } else {
                let _else_var = rand::random::<f64>().sin();
            }
        });
    }

    // Add nested loop if enabled
    if add_nested_loop {
        let mut nested_code = quote! { let mut outer = 0; };
        for _depth in 0..nested_loop_depth {
            let inner_count = rng.gen_range(5..20);
            nested_code = quote! {
                #nested_code
                for _ in 0..#inner_count {
                    outer += 1;
                }
            };
        }
        statements.push(nested_code);
    }

    // Add random function calls if enabled
    if add_random_function {
        let mut func_code = quote! {};
        for _ in 0..function_calls {
            let arg = rng.gen_range(1..100);
            func_code = quote! {
                #func_code
                let _func_result = (|x: i32| x * x * x)(#arg);
            };
        }
        statements.push(func_code);
    }

    // Volatile writes for anti-optimization
    if add_volatile_writes {
        statements.push(quote! {
            let mut volatile_var = rand::random::<i32>();
            unsafe {
                std::ptr::write_volatile(&mut volatile_var, volatile_var + 1);
            }
        });
    }


    // Final sum
    statements.push(quote! {
        let _ = (0..#sum_range).map(|x| x as i32 * x as i32).sum::<i32>();
    });

    // Shuffle statements for randomness
    use rand::seq::SliceRandom;
    statements.shuffle(&mut rng);

    let generated_code = quote! {
        {
            #(#statements)*
        }
    };

    TokenStream::from(generated_code)
}

/// A procedural macro that encrypts a string literal at compile time.
#[proc_macro]
pub fn encrypt_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let string = input.value();

    // Set key to seeded env key or default
    let key = env::var("CRYPTIFY_KEY").unwrap_or_else(|_| "xnasff3wcedj".to_string());

    let encrypted_string = xor_cipher(&string, &key);

    let output = quote! {
        cryptify::decrypt_string(#encrypted_string).as_ref()
    };

    TokenStream::from(output)
}

fn xor_cipher(input: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let input_bytes = input.as_bytes();
    let encrypted: Vec<u8> = input_bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    general_purpose::STANDARD.encode(&encrypted)
}