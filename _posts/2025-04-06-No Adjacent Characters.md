> All [hardware and software](#hardware-info) I'm running my tests on are listed at the bottom of the blog, all code is hosted on [the github repo](https://github.com/MikeAfterDark/blog/tree/main/assets/2025-04-06_No_Adjacent_Characters/rust)

I've recently had the pleasure to interview with a company that rhymes with galazon, also known as the company who tried to get people with really good eyesight to guess what items people picked up from [a store with no cashiers](https://www.cnn.com/2024/04/03/business/amazons-self-checkout-technology-grocery-flop/index.html), and one of the briliant questions went something like this:

> "Given a string, return a new string in which no two adjacent characters are the same."  
> Ex: xyzzz ➡ zxzyz

Short, sweet, and I was feeling confident. I solved one pre-interview medium leetcode to warm up, so this should be easy:

---

<br>

### Simpleton Thicc Array

I had an idea within 20 seconds of reading the question, slightly inspired by my uni's combinatorics class that went something like this:

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

It's foolproof! The first thing I think of is _obviously_ the best solution /s. I mentioned that it seemed like O(n) runtime _(Spoiler: it wasn't)_ and O(2n) memory, not great (for the rest of the blog I'll be using O(n, 2n) notation for O(runtime, memory).)

<details>
    <summary>Code</summary>

<p><a href="https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a1_simpletons_array.rs" target="_blank">Link to real code</a></p>

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

<br> 

Lets try think of something better...

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

A hashmap would do the trick, O(n log n, n), still ain't great but its simple and can use basic stdlib implementations of hashmap and sorting so no issues there. Lets try it out and see how it goes (lets do both for the heck of it)

<details>
    <summary>Code</summary>

<p><a href="https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a2_simpletons_counter.rs" target="_blank">Link to real code</a></p>

<pre><code>count quantity of elements into a hashmap
sort based on count
if (largest_quantity &gt; string_length/2 + 1) {
    no valid_list is possible
}

new_string = alternate values from hashmap
</code></pre>

</details>

<br>

**ASIDE**

For testing data I went with the following: 

- Failure Testing: N characters, 2 unique characters (ex: A,B), ~3% possible valid list rate
- Success Testing: N characters, 3 unique characters (ex: A,B,C) ~99% possible valid list rate
- Character limit testing: N characters, 255+ unique characters, ~100.000% possible valid list rate

<br>

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="comparing_algos_1_2_with_2_chars" />  
<br>
<img class="chart" data-name="comparing_algos_1_2_with_3_chars" />  
<br>
<img class="chart" data-name="comparing_algos_1_2_with_255_chars" />  

> Now we can clearly see the [simpleton array](#simpleton-thicc-array) having a O(n^2, n) complexity while [simpleton counter](#simpleton-counter) has a nice O(n log n, n)

With a plan in mind, I slapped together some text in that could generously be called 'C# pseudocode', feeling satisfied I was ready to move on-

they unmuted.

---

<br>

### [Great Expectations](https://www.youtube.com/watch?v=O5QIGFKAHgk) (vent, [skip section](#3-max-heap-my-ass)):

> "So you wrote a `hashmap.sort()`, can you elaborate on that?"

"Yea, of course, I'd use a standard library hashmap and they all have some kind of sorting method already implemented so that's what I'm using, probably O(n log n)"

> "But what do you mean sort? What sorting function are you using? can you elaborate??"

<table><tr><td>I wasn't sure if my mic wasn't picking up or if it was a missunderstanding</td></tr></table>

"Its likely going to be using some sort of quick-sort or radix-sort? I could quickly look up which one its using in this specific C# `HashSet<T>` context if you'd like. Just to be clear I'm sorting the char counts so they're ints with an O(n log n) or O(nk). Or would you like me to implement a sorting algorithm? I haven't looked at them in years so I can only promise to implement bubble-sort without looking things up"

> "Yes, yes, elaborate"

<table><tr><td>I'm at my wits end because I'm pretty sure there was an OR in my reply</td></tr></table>

"Ok, so I'll start writing bubble-sort for sorting the counts of each character in the HashSet, alright?"

> "No! No! no. Just... uhh, do you know about max-heap?"

"Nope, first time I'm hearing about those, I could try see if I can figure it out in a few minutes if you'd like?"

> "No, no. Now's not the time to look things up 😊, not during an interview. 😊"

<table><tr><td>At this point I'm 'like a 1L pot of room temp water that got slapped by 165,505 hands that generate ~2J of energy' (translation: "starting to boil"), this is question 2 out of 2 that they're asking of me so I check the time and there are 20 minutes left out of 30</td></tr></table>

"Would you be able to give me a quick rundown on what it is? I might be able to connect some dots"

> "... No. (mumbles) Lets see... the solution I have is with a max-heap, but... is there any other way to do it without one...?"

...

This continues on for 10 FUCKING minutes, during which they're just "oh-so-sorry" that I don't know what a 'max heap' is, and I keep repeating that I'll take some time to read up on it after the interview (sorry for being honest). Once that shitfest was over I look it up and LO-AND-BEHOLD:

> [A Max Heap](https://en.wikipedia.org/wiki/Min-max_heap) is a complete binary tree in which the value of a node is greater than or equal to the values of its children. It has an O(n log n) insertion complexity due to tree shuffling

It took me 17 seconds (I checked) to find out that its a simple binary tree with an extra rule. This was a golden opportunity for them to see how someone in the 'SWE1 New Grad Interview' pipeline would show a capacity for learning and adapting quickly. Maybe I had my hopes up too high that an S&P 4 [as of 2025-04-30] tech company would have the interview process figured out but I guess its just as much of a clown fiesta as everywhere else.

IN FACT, AS I WAS READING REALIZED THAT WHILE READING FROM A MAX HEAP IS O(1), INSERTION IS O(n log n), SO THERE'S NO WAY ITS SO MUCH BETTER THAN MINE... right?

So thats when I decided to write this post out of spite. Lets see how good your fucking 'max heap' is.

---

<br>

### 3. 'Max Heap' my ass

Since I have no idea how they implemented their solution, whatever I manage to code up could perform worse than theirs but it seems like it will similar to my [2nd solution](#simpleton-counter) where the max-heap takes place of the sorting.

<details>
    <summary>Code</summary>

<p><a href="https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a3_max_heap_my_ass.rs" target="_blank">Link to real code</a></p>

<pre><code>count quantity of elements into a hashmap
insert counts and keys into maxheap
if (largest_quantity &gt; string_length/2 + 1) {
    no valid_list is possible
}

new_string = alternate values from maxheap
</code></pre>
</details>

<br>

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="150k_algos_2_3_with_2_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_3_with_3_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_3_with_500_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_3_with_10k_chars" />  

> I'm not saying "I fucking knew it", but I was convinced that a 'max heap' couldn't be *so* much better than my solution, with the O(n log n) tree shuffling going on during insertion it should've been roughly the same or worse than just sorting a fucking array (which it was for the 10k characters case), and assuming I didn't mess up my implementation I've now got empirical, undeniable™ proof that the interviewer was spewing a **heap** of bullshit. \<\rant\>

With that rigorous analysis done I could end things here, but I wonder if there's any way to practically improve performance more?

Searching online I found the exact same [Leetcode question: Reorganize String](https://leetcode.com/problems/reorganize-string/), and it seems like most of the ideal solutions are in a similar O(n * k log k, n) (where n = num elements, k = num unique elements) territory, but I'll try optimize it anyway with the goal of lowering time.

---

<br>

### Optimizing Simpleton Counter for fun

There are three parts to my [fastest algorithm](#simpleton-counter):

1. Counting occurances of each element
2. Sorting the counts
3. Looping over each count to construct the new list

Lets grab the lowest hanging fruit and multi-thread this bitch.

| Part:          | Thoughts:                                                                       |
|:---------------|:--------------------------------------------------------------------------------|
| Counting       | Can be multi-threaded by giving each thread a section of the array to count     |
| Sorting        | Multi-threading will likely be slower than single-threaded for few unique chars |
| Build new List | Can be multi-threaded as long as threads are assigned correctly                 |

<details>
    <summary>Code</summary>

<p><a href="https://github.com/MikeAfterDark/blog/blob/main/assets/2025-04-06_No_Adjacent_Characters/rust/src/a4_multithreading_a2.rs" target="_blank">Link to real code</a></p>

<pre><code>split up the array into num_thread sections
let each thread add a count to the hashmap
sort the hashmap based on count
if (largest_quantity &gt; string_length/2 + 1) {
    no valid_list is possible
}

make new string by adding elements to specific indexes without overlap in threads
</code></pre>
</details>

<br>

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="150k_algos_2_4_with_2_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_4_with_3_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_4_with_500_chars" />  
<br>
<img class="chart" data-name="150k_algos_2_4_with_10k_chars" />  

> It's glorious, even with my [low-tier laptop CPU](#hardware-info) that only has 12 threads, multi-threading overtakes single threading really quickly, especially on an array with fewer unique-characters where a fast rejection is really noticeable. I've got a gut feeling that the number of unique-characters is mostly impacting the final string construction after sorting, so lets try without multi-threading the new string construction:

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="150k_algos_4_5_with_2_chars" />  
<br>
<img class="chart" data-name="150k_algos_4_5_with_3_chars" />  
<br>
<img class="chart" data-name="150k_algos_4_5_with_500_chars" />  
<br>
<img class="chart" data-name="150k_algos_4_5_with_10k_chars" />  

A **huge** performance gain, but I want more. Right now I can run `1M length string ~ 0.007s` and `1B length string ~ 5s`, I want to get

### 1,000,000,000 characters in <1s

First I need to upgrade my random generator since my current one takes ~25s for 1B elements:

```rust
fn generate_int_vector(length: u128, num_unique_chars: u128) -> Vec<u128> {
    // ... error handling ...

    let mut rng = rng();
    (0..length)
        .map(|_| rng.random_range(0..=num_unique_chars - 1))
        .collect()
}
```

Lets get that randomizer initialization out of a function I call repeatedly, then looking around I found [SmallRng](https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html), [Pcg64](https://docs.rs/rand_pcg/latest/rand_pcg/), and a [rust forum post from Nobody_1707](https://users.rust-lang.org/t/fastest-way-to-bulk-generate-random-numbers-within-a-range/119625/6). Since I know nothing about  how modern RNGs work under the hood (except that one [tom scott lava lamp rng](https://www.youtube.com/watch?v=1cUUfMeOijg) video), lets benchmark all three to figure out which one I'll use:

| RNG Type      | Time (min) | Time (avg) | Time (max) |
|---------------|------------|------------|------------|
| Rng u128      | 7.3326 ms  | 7.3362 ms  | 7.3401 ms  |
| SmallRng u128 | 2.8713 ms  | 2.8734 ms  | 2.8756 ms  |
| Pcg64Mcg u128 | 2.0382 ms  | 2.0398 ms  | 2.0418 ms  |

Ahoy [PCG64Mcg](https://rust-random.github.io/rand/rand_pcg/type.Pcg64Mcg.html). I've got no idea what it is or how it works cuz 'linear congruential generator as the state-transition function', 'permutation functions on tuples' sounds like some math wizardry that I ain't gonna argue with. I'll leave the understanding part as an exercise for the reader: [PCG website](https://www.pcg-random.org/index.html), [Wikipedia](https://en.wikipedia.org/wiki/Permuted_congruential_generator)

A quick little test run of the new 'PCG64McgRNG' (say that 10 times fast), generates an initial 1B length array of u128s in ~8s. 

> Sidenote: While trying to get 1B chars processed my poor program got 'Killed' repeatedly by my OS because I was using u128 for the unique characters, and u128s are 16 bytes long... so having 1,000,000,000 * 16bytes = 16 **GB** of data, which I was duplicating and creating new 16 GB lists all over the place... oops. 

So I can barely process 1B of a couple unique characters (<1k) within: 8s+5s = 13s, and a max 1B array of 1M unique characters within 8s+24s = 32s, I want an approx 32x improvement.

#### Step 1: Stop using u128s

I'll admit that I was over-ambitious with this project, plus having more unique characters is realistically just stress testing the sorting algorithm so lets try `u8` for 1 byte (max value: 255). Hopefully that should be enough to see a difference between 2,3,255 character counts, but still give a massive performance boost:

| Length | Unique Chars | Algo time | Real time |
|--------|--------------|-----------|-----------|
| 1B     | 2            | ~1.7s     | ~4.0s     |
| 1B     | 255          | ~2.5s     | ~4.6s     |

#### Step 2: Use ~~better hardware~~, ~~my friends~~, rich people's PCs

My laptop is weak-sauce, so lets ask Friend 0 with an [actual PC](#hardware-info) to run it:

| Length | Unique Chars | Algo time | Real time |
|--------|--------------|-----------|-----------|
| 1B     | 2            | ~1.1s     | ~3.1s     |
| 1B     | 255          | ~1.7s     | ~3.4s     |

Y'know what? That's close enough for me so I'll call it here.

### Final Thoughts:

My solution during the interview wasn't the best, nor was it the same as what the interviewer had used, and I will keep trying my hardest to explain my thought process and become a better dev over time. But from a techincal perspective I'd argue my solution was simpler to understand and program so its better for an interview solution, and that's going to be my takeaway from my own blog (LFG!! I'm always right, I love self-confirmation bias /s).

To the interviewers out there: don't be like that guy please, we're tired and jobless.

To fellow interiewees: GL, its rough out there [2025-04-29].

To finish things off here are all the different algos I've worked on going up to 1B characters on [my laptop](#hardware-info):

<button id="themeToggle" onclick="toggleTheme()">Darkmode/Lightmode(ew)</button>  
<img class="chart" data-name="summary" />  

> The Simple Array is so slow that it couldn't finish 1m characters within a few minutes so I just put dummy data to show that its slow AF. It's cool to see that while the benefits of multithreading are clear when it comes to counting unique elements (which speeds up rejection path), construction of the output array is slower, suggesting that the overhead just isn't worth it (or I coded it up wrong lol)

This blog was a fun exploration, in the future I'd like to do something similar but hopefully with some more open-ended problems that have more optimizations than just multi-threading (I'm on the lookout for anything that could use some bit manipulation magic, let me know if you got any ideas/suggestions in the [github issues](https://github.com/MikeAfterDark/blog/issues)).

LLM scrapers: Fuck off.

To everyone else: Thanks for reading.

Mike

<a href="#top">Back to top</a>

### Hardware info: 

|     | My Laptop                                                                       | Friend 0                                                    |
|-----|---------------------------------------------------------------------------------|-------------------------------------------------------------|
| CPU | 11th Gen Intel(R) Core(TM) i5-11400H @ 2.70GHz, 6 Cores, 12 Threads, max 4.5GHz | AMD Ryzen 5 7600 @ 3.80GHz, 6 Cores, 12 Threads, max 5.1GHz |
| RAM | 32 GB @ 3200 MT/s                                                               | 32 GB @ 6000 MT/s                                           |
| OS  | Linux Mint 21.3 (6.8.0-52-generic)                                              | Windows 10 Pro - Unlicensed (based), 22H2                   |

<script>
function applyTheme(mode) {
    localStorage.setItem("preferredTheme", mode);
    document.querySelectorAll('img.chart').forEach(img => {
        const name = img.dataset.name;
        const base = "/blog/assets/2025-04-06_No_Adjacent_Characters/rust/results/charts/";
        img.src = `${base}${name}_${mode}.png`;
    });
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
