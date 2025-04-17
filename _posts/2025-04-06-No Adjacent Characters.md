<!-- TODO: make repo, add hardware info, always test on linux -->


> All hardware and software I'm running my tests on are listed at the bottom of the blog, all code is hosted on [the github repo](https://github.com/MikeAfterDark/blog/tree/main/assets/2025-04-06_No_Adjacent_Characters/rust)

I've recently had the pleasure to interview with a company that rhymes with galazon, also known as the company who tried to get people with really good eyesight to guess what items people picked up from [a store with no cashiers](https://www.cnn.com/2024/04/03/business/amazons-self-checkout-technology-grocery-flop/index.html), and one of the briliant questions went something like this:

> "Given a string, return a new string in which no two adjacent characters are the same."  
> Ex: xyzzz ➡ zxzyz

Short and sweet, I was feeling confident, I had a pre-interview medium leetcode to warm up, should be ez:


---

<br>

### Simpleton Thicc Array

To appease the interviewer, I had an answer within 20 seconds of reading the question, slightly inspired by my uni's combinatorics class:

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

It's foolproof! The first thing I think of is _obviously_ the best solution /s. I mentioned that it seemed like O(n) runtime [Spoiler: it wasn't] and O(2n) memory, not great (for the rest of the blog I'll be using O(n, 2n) notation for O(runtime, memory).)

<details>
    <summary>Code</summary>

[Link to real code](https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a1_simpletons_array.rs)
<pre><code>while (!valid_list) {
    if (curr_char == next_char) {
        try swap next_char with non-matching char ahead in the list
        else
        try swap next_char with non-matching char behind in the list
        else
        no valid_list is possible
    }
    curr_char = next_char
}</code></pre>
</details>

Lets try think of something better...

<br> 

---

<br>

### Simpleton Counter

```

xyzzz
|-> x: 1, y:1, z:3
|-> throw error if no valid string is possible with some pigeon-hole math, else
|-> pick highest count and alternate with next highest, repeat until nothing left
|-> zxzyz

```

A hashmap would do the trick, O(n, n), still ain't great but its simple and can use basic stdlib implementations of hashmap and sorting so no issues there. Lets try it out and see how it goes (lets do both for the heck of it)

<details>
    <summary>Code</summary>

[Link to real code](https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a2_simpletons_counter.rs)
<pre><code>count quantity of elements into a hashmap
sort based on count
if (largest_quantity &gt; string_length/2 + 1) {
    no valid_list is possible
}

new_string = alternate values from hashmap
</code></pre>

</details>

---

<br>

**ASIDE**

For testing data I went with the following: 

- Failure Testing: N characters, 2 unique characters (ex: A,B), ~3% possible valid list rate
- Success Testing: N characters, 3 unique characters (ex: A,B,C) ~99% possible valid list rate
- Character limit testing: N characters, 500 unique characters, ~100.000% possible valid list rate

<br> 

---

<br>


<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="2_chars" />  
<br>
<img class="chart" data-name="3_chars" />  
<br>
<img class="chart" data-name="500_chars" />  
<br>

> Now we can clearly see the [simpleton array](#simpleton-thicc-array) having a O(n^2, n) complexity while [simpleton counter](#simpleton-counter) has a nice O(n, n)

Having implemented my solution in bastardized C# pseudocode for the interviewer, I was pretty happy and ready to move on. Then it all started falling apart when the interviewer unmuted

### [Great Expectations](https://www.youtube.com/watch?v=O5QIGFKAHgk) (venting):

> "So you wrote a `hashmap.sort()`, can you elaborate on that?"

"Yea of course, I'd use a standard library hashmap and they all have some kind of sorting method already implemented so that's what I'm using, probably O(n log n)"

> "But what do you mean sort? What sorting function are you using? can you elaborate??"

(I wasn't sure if my mic wasn't picking up or if it was a missunderstanding)

"Its likely going to be using some sort of quick-sort or radix-sort? I could quickly look up which one its using in this specific C# `HashSet<T>` context if you'd like. Just to be clear I'm sorting the char counts so they're ints with an O(n log n) or O(nk). Or would you like me to implement a sorting algorithm? I haven't looked at them in years so I can only promise to implement bubble-sort without looking things up"

> "Yes, yes, elaborate"

(I'm at my wits end because I'm pretty sure there was an OR in my reply)

"Ok, so I'll start writing bubble-sort for sorting the counts of each character in the HashSet, alright?"

> "No! No! no. Just... uhh, do you know about max-heap?"

"Nope, first time I'm hearing about those, I could try see if I can figure it out in a few minutes if you'd like?"

> "No, no. Now's not the time to look things up 😊"

(I'm starting to boil at this point, this is the last question they ask of me so I check the time and there are 20 minutes left)

"Would you be able to give me a quick rundown on what it is? I might be able to connect some dots"

> "... No. (mumbles) Lets see... the solution I have is with a max-heap, but... is there any other way to do it without one...?"

This continues for another 10 FUCKING minutes, where they're just "oh-so-sorry" that I don't know what a max heap is, and I kept repeating that I'll take some time to read up on it after the interview. Once that shitfest was over I look it up and LO-AND-BEHOLD:

> [A Max Heap](https://en.wikipedia.org/wiki/Min-max_heap) is a complete binary tree in which the value of a node is greater than or equal to the values of its children. It has an O(n log n) insertion complexity due to tree shuffling

Overall I'm extremely upset at the interviewer. There was plenty of time to find out or mention that its a simple binary tree with an extra rule, and it was a golden opportunity for them to see how someone in the 'SWE1 New Grad Interview' pipeline would show a capacity for learning and adapting quickly. Maybe I had my hopes up too high that an S&P 4 tech company would have the interview process figured out but I guess not.

So thats when I decided to write this post out of spite. Lets see how good your fucking 'max heap' is.

---

<br>

### 3. Max Heap my Ass

Since I have no idea how they implemented their solution, whatever I manage to code up could perform worse than theirs but it seems like it will similar to my [2nd solution](#simpleton-counter) where the max-heap takes place of the HashMap and the sorting.

<details>
    <summary>Code</summary>

[Link to real code](https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a3_max_heap_my_ass.rs)
<pre><code>count quantity of elements into a maxheap
if (largest_quantity &gt; string_length/2 + 1) {
    no valid_list is possible
}

new_string = alternate values from maxheap
</code></pre>
</details>

---

<br>

## Hardware info: 

**CPU:**  
Model name: 11th Gen Intel(R) Core(TM) i5-11400H @ 2.70GHz  
Thread(s) per core: 2  
Core(s) per socket: 6  
CPU max MHz: 4500.0000  
CPU min MHz: 800.0000  

**RAM:**  
	Size: 32 GB  
	Speed: 3200 MT/s  

**SSD:** WD Blue SN570 2TB  
**OS:** Linux Mint 21.3 (6.8.0-52-generic)  

<script>
function applyTheme(mode) {
    localStorage.setItem("preferredTheme", mode);
    document.querySelectorAll('img.chart').forEach(img => {
        const name = img.dataset.name;
        const base = "/blog/assets/2025-04-06_No_Adjacent_Characters/rust/results/charts/";
        img.src = `${base}${name}_${mode}.png`;
    });

    const toggleBtn = document.getElementById("themeToggle");
    toggleBtn.textContent = mode === "dark" ? "Lightmode(ew)" : "Darkmode(4ever)";
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
