---
title: "Choosing a programming language: Why I chose Rust"
date: 2021-05-03T01:47:21+02:00
draft: true
---

After spending a disgusting amount of time looking into the many available programming languages, I have finally settled on Rust.

It was not an easy choice, because there are some things about Rust that were almost deal-breakers for me. However, in my view the pros outweigh the cons and I claim that the language is a great choice for most projects. I will lay out the case for Rust in this blog post to convince you that it is not just hype. The language is objectively a great choice.

# The amazing things about Rust
Rust code is really fast and with the `unsafe` escape hatch it's possible to use it for any kind of low level coding too. This opens the door to writing really efficient software which is in my opinion critical to the success of a project. 

The documentation and available resources for Rust are incredible. Even the compiler errors themselves are the best I've ever seen. The community is welcoming and humble. To see what I mean, here is a shortlist of some amazing resources I've come across so far:
- The [Rust book](https://doc.rust-lang.org/book/) is a fantastic resource
- The very nifty [Cheat Sheet](https://cheats.rs/)
- Code examples can be found in [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- There's also the [Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/README.html)
- For perf needs there's [The performance book](https://nnethercote.github.io/perf-book/)
- For low level stuff check out the [Rustonomicon](https://doc.rust-lang.org/nomicon/). 
- There's [Jon Gjengset's fantastic youtube channel](https://www.youtube.com/channel/UC_iD0xppBwwsrM9DegC5cQQ) where he goes over many of the more advanced Rust topics. 
- Design patterns are often useful code examples and for that there's [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html)

Learning Rust is a long process, but the resources to do it are there and they make learning Rust a pleasure.

The next good thing about Rust is that the language is getting adopted in the industry. I cannot be sure about this of course, but I think I see it happening and this is a big deal. Some evidence of this happening is:
- Microsoft releasing [Rust for Windows](https://github.com/microsoft/windows-rs) to allow easy Windows API interaction from Rust. 
- Amazon scaling up their Rust team and even [hiring core Rust contributor Niko Matsakis](https://aws.amazon.com/blogs/opensource/how-our-aws-rust-team-will-contribute-to-rusts-future-successes/)
- Amazon releasing their own [AWS SDK for Rust](https://aws.amazon.com/blogs/developer/a-new-aws-sdk-for-rust-alpha-launch/)
- Facebook writing their cryptocoin [Diem (nee Libra) in Rust](https://en.wikipedia.org/wiki/Diem_(digital_currency))
- The RFC to get Rust in the linux kernel has landed and [Linus doesn't hate it](https://lkml.org/lkml/2021/4/14/1099) so Rust may actually make its way into the linux kernel which even C++ hasn't managed.
- Rust is used in [Google OS Fuchsia](https://en.wikipedia.org/wiki/Google_Fuchsia). I believe it isn't used in the microkernel Zircon, just in userspace, but still.
- Rust is also allegedly used to [develop parts of Android platform](https://www.xda-developers.com/google-developing-android-rust/)

So with Microsoft, Amazon, Facebook and Google that means that the biggest tech companies (except Apple) are getting on board quickly. The amount of resources that adoption will bring can really help the language get fantastic tooling, more high quality libraries and even better learning material. These things in turn will make a programmer's life much easier and will make using the language feel very good. This increase in backing may even help solve some of Rust's ugly sides.

# The ugly things in Rust
Rust compilation is slow. There's no sugar-coating this. The main reason seems to be LLVM itself being very slow, but Rust also does things other compilers don't do like borrow checking. Whilst, I haven't experienced slow compilation myself yet, I think this is just due to not having worked on a sufficiently large project. I trust people when they say this is a problem and I think I will run into slow compilation times eventually and that it will be awful. This is almost a deal-breaker, but after careful consideration I think that it is outweighed by the pros. Furthermore, as I explained earlier, I believe Rust is going to get widely adopted and with better tools and as more work is being put into compilers, it may be possible to improve the speed of the compiler.

Rust also makes it very easy to use dependencies. It is very easy to find a public "crate" that does something you want, and make your program use this crate. This crate may, and often will, bring many other crates that it depends on. This can easily cause your project to depend on a great many things. This phenomena is usually called "dependency hell". It can slow down compilation and it is also not good when you have no idea what code you have in your project and why. Likewise, dependency hell could also constitute a security issue, because it may be possible to easily end up with a malicious crate. 

On the other hand, being able to easily try different crates and being able publish crates easily make it a breeze to try things out. Getting your hands on sophisticated libraries just by adding a line into you Cargo.toml manifest removes a lot of friction from programming. This ease of experimentation can help get a lot of projects started. Therefore, I think dependencies in Rust are a good thing after all, they just have to be used very judiciously as your project gets bigger. Fortunately, there's the `cargo tree` command which shows you a nice tree-view of what crates you use so that you can investigate and trim dependencies.

# The fear of restrictions
One of Rust's most well-known features, and indeed its main selling point, is that the compiler restricts many things that are considered valid code in most programming languages. It does this for the sake of memory safety and safe concurrency. The components that play a role in this are: the borrow checker, Rust's ownership model and limiting mutable borrowing. 

Guaranteeing memory safety is the task of the borrow checker. In Rust memory must only have a single owner. So if we want to use the memory in multiple places we need pointers. In Rust pointers and references are called borrows. The borrow checker validates borrow lifetimes, which means that it validates if the borrowed memory is still valid. The idea is that one cannot access memory that has gone out of scope through a borrow, because when the memory goes out of scope, then the borrow becomes invalid and the borrow checker will complain.

On the other hand, safe concurrency is achieved by means of making sure that at any moment, only one mutable borrow can point to memory. A mutable borrow is a pointer that allows writing into the memory to which it points. Data races for this memory are avoided by making sure that, at any time, you can either have one mutable borrow or many immutable ones. Additionally, Rust also has the marker traits `Send` and `Sync` which govern what kind of data can be passed around between threads. Only data that is `Send` can be passed around and only borrows to data that is `Sync` can be passed around. This is done so as to ensure that it is only possible to pass data that is safe to pass between threads.

Strict limitations like these can make it quite difficult to do certain things in Rust as easily as they can be done in other languages. A famous example is a doubly linked list. This type of list is usually implemented by means of type that contains a two fields, one of which is a pointer to the next entry in the list and the other field a pointer to the previous entry. In safe Rust pointers are borrows. So that means that an entry in the middle of the list is pointed at or borrowed by the next entry as well as the previous entry. As we said, it cannot be borrowed mutably by both so at best we can have it borrowed immutably, which is kinda useless, and even in that case may still struggle with borrow lifetimes. There are ways around this, but they are all very complicated.

So, it is clear that the language restricts what we can do and for many people this is a deal breaker. While it is always possible to use the `unsafe` escape hatch which permits us to read and write through unsafe pointers which aren't borrow-checked, most people find that if the language disallows things in safe mode, then this will cause friction when programming. I think this is true, it will cause friction, but the reason it causes friction is because the programmer is coming from a language where he/she was able to do such things easily and therefore has tendencies to write code that the Rust compiler doesn't like.

All programming languages are restrictive. We cannot write programs using human language. We use code, which is designed to be a means to communicate with a computer. So programming is a form of limited communication. The thing is, once a programmer has learned to code in a language then his/her way of expressing himself/herself is influenced by this language. 

Functional programming is another good example of a type of programming that many find restrictive. The reason is that most of us learn a imperative programming language like C, Java, JavaScript or Python as our first programming language. Therefore, our way of expressing ourselves in code is imperative and so learning to express ourselves in a functional manner requires retraining and feels unnatural for a long time.

I claim that with Rust the experience of being restricted is due to the same reason. We are used to expressing ourselves in programming languages which do not have the same restrictions that Rust has. Of course, these other languages have plenty of their own restrictions, they are not human language after all.

So, in essence Rust introduces a new restriction and programming with this new restriction will feel unnatural for experienced programmers that are new to Rust. However, I think once we get used expressing ourselves in Rust, then it will feel natural again and won't cause friction anymore.

To offer an analogy: many interpreted languages are dynamically typed which means that the programmer is free to pass any type into a function as a parameter. Statically typed languages restrict this and force programmers to work a lot harder to achieve the same thing. Now, obviously static typing adds a restriction that limits expressiveness. However, most programmers of statically typed languages are quite fine with this restriction and learn to be very productive in spite of it.

# Summary
I've gone over what I think are the most pertinent points when it comes to choosing Rust. I've mentioned the two big negatives which are compilation speed and dependency hell. For me these are outweighed by the positives of having a very fast language that can go `unsafe`, having fantastic learning resources and compiler error messages and a language that seems to be reaching critical mass when it comes to adoption. I've also addressed what I think about the issue of programming in Rust being harder/more restrictive. 

Having considered all these pros and cons, I think Rust has the most important things going for it. These are creating fast software, having excellent documentation and becoming a widely adopted language. There are things complicating a programmers life like the slow compilation and the additional friction while learning Rust, but hopefully the former will improve with Rust adoption and the latter I believe is a non-issue.



