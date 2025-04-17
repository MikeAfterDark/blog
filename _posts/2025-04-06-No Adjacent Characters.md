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
    <summary>Pseudocode: Simpleton's Thicc Array</summary>
    
```
while (!valid_list) {
    if (curr_char == next_char) {
        try swap next_char with non-matching char ahead in the list
        else
        try swap next_char with non-matching char behind in the list
        else
        no valid_list is possible
    }
    curr_char = next_char
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

```

A hashmap would do the trick, O(n, n), still ain't great but its simple and can use basic stdlib implementations of hashmap and sorting so no issues there. Lets try it out and see how it goes (lets do both for the heck of it)

<details>
    <summary>Pseudocode: Simpleton's Counter</summary>

```
count quantity of elements into a hashmap
sort based on count
if (largest_quantity > string_length/2 + 1) {
    no valid_list is possible
}

new_string = alternate values from hashmap
```

</details>

---
**ASIDE**

For testing data I went with the following: 

- Failure Testing: N characters, 2 unique characters (ex: A,B), ~3% valid list rate
- Success Testing: N characters, 3 unique characters (ex: A,B,C) ~99% valid list rate
- Character limit testing: N characters, N/2 unique characters, ~100.000% valid list rate

---

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>
<img class="chart" data-name="2_chars" />
<img class="chart" data-name="3_chars" />
<img class="chart" data-name="500_chars" />

<script>
function applyTheme(mode) {
    localStorage.setItem("preferredTheme", mode);
    document.querySelectorAll('img.chart').forEach(img => {
        const name = img.dataset.name;
        const base = "/code/2025-04-06-No Adjacent Characters/rust/results/charts/";
        img.src = `${base}${name}_${mode}.png`;
    });

    const toggleBtn = document.getElementById("themeToggle");
    toggleBtn.textContent = mode === "dark" ? "Switch to Light Mode" : "Switch to Dark Mode";
}

function toggleTheme() {
    const current = localStorage.getItem("preferredTheme") || "dark";
    const newMode = current === "dark" ? "light" : "dark";
    applyTheme(newMode);
}

document.addEventListener("DOMContentLoaded", () => {
    const saved = localStorage.getItem("preferredTheme") || "dark";
    applyTheme(saved);
});
</script>


## Hardware info: 

CPU:
Model name:                           11th Gen Intel(R) Core(TM) i5-11400H @ 2.70GHz
Thread(s) per core:                   2
Core(s) per socket:                   6
CPU max MHz:                          4500.0000
CPU min MHz:                          800.0000

RAM:
	Size: 16 GB
	Speed: 3200 MT/s
	Configured Memory Speed: 3200 MT/s
	Volatile Size: 16 GB

SSD: WD Blue SN570 2TB
OS: Linux Mint 21.3 (6.8.0-52-generic)
