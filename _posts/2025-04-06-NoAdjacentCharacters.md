# Title

<!-- TODO: make repo, add hardware info, always test on linux -->

> [!NOTE]
> All hardware and software I'm running my tests on are listed at the bottom of the blog, all code is hosted on [the github repo](https://google.ca)

So recently I've had the pleasure of going through a set of interviews with a company that rhymes with galazon, also known as the company who tried to get people with really good eyesight to stare at pixels to see if people from the other side of the world are buying a '2gal oat milk' or a '2gal soy milk', then charge their registered digital wallets for whichever they think it was, and one of the briliant questions I was asked to answer went something like this:

> "Given a string, return a new string in which no two adjacent characters are the same."

Ex: `xyzzz -> zxzyz`

I like it, a short and sweet problem with nothing serious going on, lets think of solutions:

### 1. The Simpleton's Thicc Array

```
xyzzz
|-> _ x _ y _ z _ z _ z _
|-> z x _ y _ z _ z _ _ _
|-> z x z y _ z _ _ _ _ _
|-> z x z y z _ _ _ _ _ _
```

```
zxxyxzz
|-> _ z _ x _ x _ y _ x _ z _ z _
|-> x z _ x _ x _ y _ _ _ z _ z _
|-> x z y x _ x _ _ _ _ _ z _ z _
|-> x z y x z x _ _ _ _ _ z _ _ _
|-> x z y x z x z _ _ _ _ _ _ _ _
```

It's foolproof! The first thing I think of is _obviously_ the best solution, but wait, the interviewer is looking at me like I just bought 2gal of soy milk, so something must be up and its not updog. Hmmm, it's O(n) runtime and... O(2n) memory, not great (for the rest of the blog I'll be using O(n, 2n) notation for O(runtime, memory).)

<details>
    <summary>Code: Simpleton's Thicc Array</summary>
    
<!-- TODO: make and test -->
```cpp

char[] SimpletonsThiccArray(char[] input) {

    // setup
    char[] output = [(input.size*2)+1];
    for (int i = 0; i < input.size; i++) {
        output[2*i] = "_";
        output[2*(i+1)] = input[i];
    }

    timer.start();

//implement here
timer.end();

    return output;

}

```

</details>

Lets try think of something better...

### 2. The Simpleton's Counter

```

xyzzz
|-> x: 1, y:1, z:3
|-> throw error if no valid string is possible with some pigeon-hole math, else
|-> pick highest count and alternate with next highest, repeat until nothing left
|-> zxzyz

````

A hashmap would do the trick, O(n, n), still ain't great but its simple and can use basic stdlib implementations of hashmap and sorting so no issues there. Lets try it out and see how it goes (lets do both for the heck of it)

<details>
    <summary>Code: Simpleton's Counter</summary>

<!-- TODO: make and test -->
```cpp
#include <iostream>

int main() {
std::cout << "Hello, world!" << std::endl;
return 0;
}
````

</details>

> [!NOTE]
> Sticking to basic alphanumeric english for now

<!-- insert graph 1 here:

y axis: time
x axis: amount of characters

-->
