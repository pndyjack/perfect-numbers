# Perfect Numbers

Determine if a number is perfect, abundant, or deficient based on
Nicomachus' (60 - 120 CE) classification scheme for natural numbers.

The Greek mathematician [Nicomachus](https://en.wikipedia.org/wiki/Nicomachus) devised a classification scheme for natural numbers, identifying each as belonging uniquely to the categories of **perfect**, **abundant**, or **deficient** based on their [aliquot sum](https://en.wikipedia.org/wiki/Aliquot_sum). The aliquot sum is defined as the sum of the factors of a number not including the number itself. For example, the aliquot sum of 15 is (1 + 3 + 5) = 9

- **Perfect**: aliquot sum = number
  - 6 is a perfect number because (1 + 2 + 3) = 6
  - 28 is a perfect number because (1 + 2 + 4 + 7 + 14) = 28
- **Abundant**: aliquot sum > number
  - 12 is an abundant number because (1 + 2 + 3 + 4 + 6) = 16
  - 24 is an abundant number because (1 + 2 + 3 + 4 + 6 + 8 + 12) = 36
- **Deficient**: aliquot sum < number
  - 8 is a deficient number because (1 + 2 + 4) = 7
  - Prime numbers are deficient

Implement a way to determine whether a given number is **perfect**. Depending on your language track, you may also need to implement a way to determine whether a given number is **abundant** or **deficient**.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

## Source

Taken from Chapter 2 of Functional Thinking by Neal Ford. [http://shop.oreilly.com/product/0636920029687.do](http://shop.oreilly.com/product/0636920029687.do)
