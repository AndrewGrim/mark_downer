# heading1

## heading2

### heading3

#### heading4

##### heading5

###### heading6


* list 1
    1. list 2
    1. list 2
        * list 3
            1. list 4
* list 1

> this is a block quote

**bold**

*italic*

~~strikethrough~~

__underlined__

text

---

`code`

    pre-block that keeps the formatting
        of the
    text

## Python code block
```python
class Test(Object):

    def __init__(self):
        self.test = 123

        if x in ["a", "b", 'c']:
            for x in y:
                yield x

        with open("file.txt", "r") as f:
            while True:
                try:
                    f.read(1)
                except Exception as e:
                    print(e)
                    break

        return 0
```

## Rust code block
```rust
struct Token {
    id: usize,
    begin: usize,
    end: usize,
    content: String,
}

let slice: &str = String::from("testing")[..];

match c {
  'a'..'z' => println!("found char"),
  '0'..'9' => {
      tokens.push(Token::new_single(
          TokenType::Digit as usize, i, String::from(c)));
      break;
   }
}

enum TokenTypes {
    Heading,
    Code,
    Digit,
}

fn test(path) -> bool { // comment
    if path::Path::new("test.txt").exists() {
        return true;
    } else {
        return false;
    }
}

/*
multi...
  line
    comment
*/
```

## D code block
```d
int main(string[] args) {
    string s = "test";

    return 0;
}
```

## C code block
```c
int main(int argc, char **argv) {
    char *s = "test";

    return 0;
}
```

## No language code block
```
int main(int argc, char **argv) {
    char *s = "test";

    return 0;
}
```

- [ ] unchecked
- [x] checked

![alternative text](images/icon.png)

[this is a link](https://google.com)

<p align=center>
    <img src="images/icon.png" alt="alt text" height="48" width="48">
    <strike><u><i><b>inline html</b></i></u></strike>
</p>

| Heading Col 1 | Heading Col 2 | HCol 3|
| ------------- |:-------------:| -----:|
| col 1 is      | left-aligned  | 12379 |
| col 2 is      | centered      |   123 |
| col 3 is      | right-aligned |    69 |

