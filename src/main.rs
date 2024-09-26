use std::process::Command;
use std::io::{self, Write};
use std::env;

fn main() -> io::Result<()> {
    // Get the current working directory
    let contract_path = env::current_dir()?.to_string_lossy().into_owned(); 
    let source_account = "alice"; // Specify your source account
    let network = "testnet"; // Specify your network

    // Prompt for the function name
    print!("Enter the function name to invoke: ");
    io::stdout().flush()?; // Ensure prompt is displayed before input

    let mut function_name = String::new();
    io::stdin().read_line(&mut function_name)?;
    let function_name = function_name.trim(); // Remove newline

    // Prompt for additional arguments
    print!("Do you want to pass any additional arguments? (yes/no): ");
    io::stdout().flush()?;
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    let response = response.trim().to_lowercase();

    let mut additional_args = Vec::new();
    if response == "yes" {
        print!("Enter additional arguments separated by spaces: ");
        io::stdout().flush()?;
        let mut args_input = String::new();
        io::stdin().read_line(&mut args_input)?;
        additional_args = args_input.trim().split_whitespace().map(String::from).collect();
    }

    // Step 1: Build the contract
    println!("Building the contract...");
    let build_output = Command::new("cargo")
        .arg("build")
        .current_dir(&contract_path)  // Use a reference to avoid moving
        .output()?;

    if !build_output.status.success() {
        eprintln!("Failed to build contract: {}", String::from_utf8_lossy(&build_output.stderr));
        return Ok(());
    }
    println!("Contract built successfully.");

    // Step 2: Deploy the contract
    println!("Deploying the contract...");
    let deploy_output = Command::new("stellar")
        .arg("contract")
        .arg("deploy")
        .arg("--wasm")
        .arg(format!("{}/../../target/wasm32-unknown-unknown/release/hello_world.wasm", contract_path)) // Navigate up two levels
        .arg("--source-account")
        .arg(source_account)
        .arg("--network")
        .arg(network)
        .output()?;

    if !deploy_output.status.success() {
        eprintln!("Failed to deploy contract: {}", String::from_utf8_lossy(&deploy_output.stderr));
        return Ok(());
    }
    println!("Contract deployed successfully.");

    // Step 3: Extract the contract ID from the deployment output
    let deploy_output_str = String::from_utf8_lossy(&deploy_output.stdout);
    
    // Update the logic to extract the contract ID from the correct output
    let contract_id = deploy_output_str.lines()
        .find(|line| line.starts_with("Contract ID: "))
        .map(|line| line.trim_start_matches("Contract ID: "))
        .unwrap_or("");

    // Check if the contract ID was found
    if contract_id.is_empty() {
        // Try to extract it directly from the output
        if let Some(id) = deploy_output_str.split_whitespace().find(|word| word.len() == 56) {
            println!("Extracted contract ID directly from output: {}", id);
        } else {
            eprintln!("Failed to extract contract ID from deployment output.");
            eprintln!("Deployment output: {}", deploy_output_str);
            return Ok(());
        }
    } else {
        println!("Contract ID: {}", contract_id);
    }

    //// Step 4: Invoke the function
    //println!("Invoking function '{}'...", function_name);
    
    //// Create the command to invoke the function
    //let mut invoke_command = Command::new("stellar");
    
    //// Add the invocation command components
    //invoke_command
        //.arg("contract")
        //.arg("invoke")
        //.arg("--id")
        //.arg(contract_id)
        //.arg("--source-account")
        //.arg(source_account)
        //.arg("--network")
        //.arg(network);
    
    //// Add additional arguments if any
    //for arg in additional_args {
        //invoke_command.arg(arg);
    //}

    //// Add the function name last
    //invoke_command.arg(function_name);

    //let invoke_output = invoke_command.output()?;

    //if !invoke_output.status.success() {
        //eprintln!("Failed to invoke function: {}", String::from_utf8_lossy(&invoke_output.stderr));
        //return Ok(());
    //}
    //println!("Function '{}' invoked successfully.", function_name);

    Ok(())
}

