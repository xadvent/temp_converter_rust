# Temperature Conversion Utility

## Description

This Rust project is a simple command-line utility for converting temperatures between Fahrenheit and Celsius. It prompts users to choose the conversion direction (Fahrenheit to Celsius or Celsius to Fahrenheit) and then requests the temperature to convert. The utility is designed to be user-friendly, providing clear instructions and input validation.

## Features

- Convert temperatures from Fahrenheit to Celsius and vice versa.
- User input validation for both the conversion type and the temperature value.
- Clear instructions provided to the user for a smooth experience.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. You can download them from [the official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository to your local machine:

```bash
git clone https://your-repository-url.git
```

2. Navigate to the project directory:

```bash
cd temperature_conversion_utility
```

3. Build the project:

```bash
cargo build --release
```

4. Run the utility:

```bash
cargo run
```

## Usage

Follow the on-screen prompts:

1. First, choose your conversion by typing `"C"` for Fahrenheit to Celsius or `"F"` for Celsius to Fahrenheit.
2. Next, input the temperature you'd like to convert.
3. The converted temperature will be displayed.
