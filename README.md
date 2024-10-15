# neplat "Nepali-English Interpreted Programming Language"

This project is an interpreted programming language that allows users to write code using a mix of English and Nepali-transliterated keywords. This hybrid approach enables native Nepali speakers to code in a more intuitive and accessible manner, using familiar language constructs alongside traditional English keywords.

The language supports essential programming features such as variable declarations, assignments, conditionals, loops, and function definitions, all with the flexibility to switch between English and Nepali-transliterated keywords seamlessly.

## **Features**

- **Bilingual Syntax**: Write code using either English or Nepali-transliterated keywords, or a combination of both.
- **Easy Transition**: Developers familiar with English-based programming languages can easily switch to or incorporate Nepali terms without altering core logic.
- **Flexible Coding**: You can choose the most comfortable language for your coding experience, improving accessibility for native Nepali speakers.

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
manum a bhaneko 0;
manum temp;

ko_lagi (manum b bhaneko 1; a bhanda_sano 10000; b bhaneko temp joda b) {
  dekhau a;
  temp = a;
  a = b;
}

/*
Can use either of the above or below
*/
let a = 0;
let temp;

for (let b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
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

- Clone the repository.
- Follow the instructions in the setup guide to install the necessary dependencies.
- Start coding using either English or Nepali keywords as shown in the example above.

## Contributing

We welcome contributions to enhance the language, add more keywords, or improve functionality. Feel free to submit pull requests or open issues for discussion.