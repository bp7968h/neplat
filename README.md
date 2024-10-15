# neplat "Nepali-English Interpreted Programming Language"

This project is an interpreted programming language that allows users to write code using a mix of English and Nepali-transliterated keywords. This hybrid approach enables native Nepali speakers to code in a more intuitive and accessible manner, using familiar language constructs alongside traditional English keywords.

The language supports essential programming features such as variable declarations, assignments, conditionals, loops, and function definitions, all with the flexibility to switch between English and Nepali-transliterated keywords seamlessly.

## **Features**

- **Bilingual Syntax**: Write code using either English or Nepali-transliterated keywords, or a combination of both.
- **Easy Transition**: Developers familiar with English-based programming languages can easily switch to or incorporate Nepali terms without altering core logic.
- **Flexible Coding**: You can choose the most comfortable language for your coding experience, improving accessibility for native Nepali speakers.

## **Language Features**

- **Variables**: Supports variable declarations using `let` (English) or `manum` (Nepali-transliterated).
- **Data Types**: Dynamic data type; handles strings, numbers, booleans, and null values.
- **Conditionals**: Use `if`, `else`, and `yadi`, `athwa` to write conditional logic.
- **Loops**: Supports `for`, `while`, `ko_lagi`, `jaba_samma` for iterative loops.
- **Functions**: Define reusable code blocks using `func` or `karya`, with support for arguments.
- **Logical Operations**: Perform logical comparisons using `and`, `or`, `==`, `!=`, and their Nepali counterparts (`ra`, `wa`, `barabar`, `barabar_chaina`).
- **Arithmetic Operations**: Supports basic arithmetic operations such as addition(`+` or `joda`), subtraction(`-` or `ghatau`), multiplication(`*`), and division(`/`).
- **Comments**: Write single-line `//` and multi-line comments `/* */` to document code.
- **Error Handling**: Gracefully handle errors during interpretation, including syntax errors, runtime errors, and more.
- **Custom Functions**: Define and invoke user-defined functions, supporting custom arguments and return values.
- **Operator Overloading**: Supports overloading of certain operators to handle different data types seamlessly.

## **Example Code**

Here’s an example of how you might use Neplat with both English and Nepali keywords:

```plaintext
manum x bhaneko 10;     // Declares a variable in Nepali
let y = 5;              // Declares a variable in English
if (x > y) {            // Uses English for conditionals
    dekhau("Satya");    // Prints a message in Nepali
} athwa {               // Uses Nepali for else condition
    print("false");     // Another print in English
}
```

```plaintext
// nepali transliterated
manum a bhaneko 0;
manum temp;

ko_lagi (manum b bhaneko 1; a bhanda_sano 10000; b bhaneko temp joda b) {
  dekhau a;
  temp bhaneko a;
  a bhaneko b;
}

/*
Can use either of the above or below
*/

// english
let a = 0;
let temp;

for (let b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
```

```plaintext
// Variables and Data Types
manum name bhaneko "NepLat";  // Variable declared in Nepali
let version = 1;            // Variable declared in English
manum is_active bhaneko satya; // Boolean variable using Nepali-transliterated true
let isBeta = false;           // Boolean variable using English

// Conditionals
yadi (version >= 1 ra is_active) {   // If statement in Nepali-transliterated with logical `ra` (and)
    dekhau("Version is up-to-date and active!");  // Print in Nepali
} athwa {                              // Else statement in Nepali-transliterated
    print("Update needed or inactive.");  // Print in English
}

// Loops
// For loop using English keywords
for (let i = 0; i < 5; i = i + 1) {
    print("Count (English): " + i);
}

// While loop using Nepali-transliterated keywords
manum counter bhaneko 0;
jaba_samma (counter bhanda_sano 5) {   // Nepali while loop
    dekhau("Ginti (Nepali): " + counter);
    counter bhaneko counter joda 1;    // Addition operation in Nepali-transliterated
}

// Functions
karya greet(first_name, last_name) {   // Function declaration in Nepali-transliterated
    dekhau("Namaste " + first_name + " " + last_name + "!"); // Concatenation with Nepali-transliterated print
}

func calculateSum(a, b) {              // Function declaration in English
    print a + b;                      // Using print statement in English
}

// Function Calls
greet("Dai", "Bhai");                 // Call function using Nepali-transliterated
calculateSum(10, 20);

// Logical Operations
let isUpdated = version == 1 wa is_active;  // Logical `or` in Nepali-transliterated
yadi (isUpdated) {
    dekhau("System is updated or active.");
}

// Comments
// Single-line comment example
/* 
   Multi-line comment example
   Demonstrating how comments can be used 
   to document code.
*/

// Error Handling (Demonstrated implicitly through invalid operations, e.g., divide by zero)
manum result;
yadi (is_active ra version barabar 1) {
    result bhaneko 100 joda 25;
    dekhau("Calculation result: " + result);
}

// Operator Overloading
let concatenated = "Hello" + " " + "World!";  // String concatenation using overloaded `+`
print(concatenated);
```
```bash
#output
Version is up-to-date and active!
Count (English): 0
Count (English): 1
Count (English): 2
Count (English): 3
Count (English): 4
Ginti (Nepali): 0
Ginti (Nepali): 1
Ginti (Nepali): 2
Ginti (Nepali): 3
Ginti (Nepali): 4
Namaste Dai Bhai!
30
System is updated or active.
Calculation result: 125
Hello World!
```

## Keyword Mapping

The table below lists the English (Latin) keywords and their corresponding Nepalese transliterations. You can use either version in your code:

| **Latin**     | **Nepalese**     |
|---------------|------------------|
| true          | satya            |
| false         | galat            |
| and           | ra               |
| or            | wa               |
| if            | yadi             |
| else          | athwa            |
| func          | karya            |
| return        | dinus            |
| for           | ko_lagi          |
| null          | khali            |
| print         | dekhau           |
| var           | manum            |
| while         | jaba_samma       |
| class         | samuha           |
| this          | yei              |
| super         | affnai           |

## Operator Mapping

The table below lists the Normal operators and their corresponding Nepalese transliterations. You can use either version in your code:

| **Normal**     | **Nepalese**     |
|---------------|------------------|
| -             | ghatau           |
| +             | joda             |
| !             | ulto             |
| !=            | barabar_chaina   |
| =             | bhaneko          |
| ==            | barabar          |
| >             | bhanda_thulo     |
| >=            | thulo_wa_barabar |
| <             | bhanda_sano      |
| <=            | sano_wa_barabar  |

## How to Get Started

### Prerequisites

Ensure you have Rust installed. If not, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

### Installation

Clone the repository to begin exploring the hash functions:

```bash
git clone https://github.com/bp7968h/neplat.git
cd neplat
cargo build --release
```

### Usage

Start coding using either English or Nepali keywords as shown in the example above.

```bash
cat your_code.neplat
func sayHi(first, last) {
  print "Hi, " + first + " " + last + "!";
}

sayHi("Dear", "Reader");

./neplat your_code.neplat
Hi, Dear Reader!
```

## Contributing

We welcome contributions to enhance the language, add more keywords, or improve functionality. Feel free to submit pull requests or open issues for discussion.

> [!NOTE]  
> This is work in progress, some features are not implemented, like function return, classes and others
