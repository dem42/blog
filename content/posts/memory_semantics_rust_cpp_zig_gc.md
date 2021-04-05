---
title: "Choosing a programming language: Memory semantics"
date: 2021-04-05T00:47:00+02:00
draft: true
---
With the abundance of modern languages nowadays, it can be quite difficult to choose a language for a project.

One of the hardest concepts to reason about when building your program is memory. Abstractions introduced in high level programming languages (by that I mean higher level than Assembley) as well as the abstractions done at the operating system level conceal what is really going on when your program reads and writes memory. Therefore, you want a programming language that has clear and easily understandable memory semantics can help you understand what's going on with your programs memory.

So let's investigate a few programming languages and their memory semantics.

# Why even do this?
So maybe before we jump into the fun stuff, I should justify why this makes sense. If you only care about the fun stuff feel free to skip this section. 

First of all, I think we are now in a new era of programming languages. The industry hasn't quite caught up to this yet, but I think it is catching up slowly and steadily. 

This new era in my option began with languages targeting the java virtual machine (JVM). Scala and Clojure were the first big languages to emerge out of this and both gained traction. Kotlin followed a number of years later and I would say that though neither Kotlin nor Scala nor Clojure managed to replace Java, they all took enough market share from Java to cause it to go into decline. 

After LLVM made it reasonable to create natively compiled languages, I think we are now seeing the same happening in the native space. I believe Rust is already getting a lot of attention and market share and I think other, newer languages will follow and equally manage to grab some market share. Noweadays we also don't need big runtime and virtual machines anymore for compatibility, and so native languages will take market share from virtual machine languages like Java and C#.

This is of course quite exciting for programmers. It means we now to get to shop around and pick the language that we like the best. However, it's no fun to have to rewrite your project after the honeymoon period when you decide you didn't like your original choice of language. Therefore, it's important to evaluate programming languages carefully in order to make as good a choice as possible. 

# Memory
Memory is one of the biggest hurdles for programs for two reason: concurrency and performance.

Getting concurrent algorithms right is difficult mainly due to data races where one has to make sure that access to data is correctly synchronized. The problem is if you have a lot of different writers and readers of a datum, then
you need to think very carefully about how to rescrtict this access. If you fail to do this, then you can end up with reading stale data, reading data while it's being written into and just many painfully difficult things like this that are hard to find and fix. Additionally, using synchronization primitives can be expensive performance wise.

Memory access is also very slow. This is of course architecture dependendent, but a common architecture will most likely have registers on the CPU, multiple levels of fast cache and then a large random access memory. Reading and writing to registers but each level of cache is an order of magnitude slower and then memory several orders of magnitude slower. Not to mention that there isn't an unlimited amount of memory. Your process will most likely have access to a heap of memory that it can use to allocate long living objects or objects that must grow. Then when these objects are no longer needed or when then shrink you want this memory to be reusable for some other object and so you would like it get freed. Of course, if this isn't done carefully, you can easily fragment your memory making it hard to find block of memory in your heap that is the right size for your object. 

Programming languages abstract memory away into data structures. Be it classes, structs, arrays or lists what these are after you strip away the abstraction is memory and memory is problematic. The programmer needs to be careful with all these abstractions because memory opens up the problems I mentioned before like concurrency and performance. Your programming language should therefore provide you with the tools to manage these memory abstractions in such a way that you can easily and quickly spot problems and it should also give you the tools to program in a way that lets you avoid memory problems as much as possible.

# Rust
A good place to start this discussion is with Rust, since this language restricts what you can do with memory to the point where concurrency data races and memory crashes (except out-of-memory panics due to memory leaks) are eliminated (provided nothing funny is being done in unsafe mode). Rust has all these guarnatees because it restricts the references that can be taken to a variable. References allow us to pass around a lightweight pointer to the memory that is associated with the contents of the variable. Rust's restriction is that either one mutable reference can be taken, or multiple non-mutable references. Mutable references allow you to change the memory contents. Non-mutable references don't. By being restricted to having only one mutable references it is clear that there cannot be two different concurrent modifications of this memory and thus concurrency data races won't happen.

```
fn multiple_immutable_refs_are_ok() -> bool {
    let list: Vec<u32> = vec![1, 2, 3, 4];
    let read_ref1: &Vec<u32> = &list;
    let read_ref2: &Vec<u32> = &list;
    
    read_ref1 == read_ref2
}

fn one_mutable_ref_is_ok() -> bool {
    let mut list: Vec<u32> = vec![1, 2, 3, 4];

    let write_ref: &mut Vec<u32> = &mut list;

    write_ref[0] = 2;
    list[0] == 2
}

fn mutable_and_immutable_refs_not_ok() -> bool {
    let mut list: Vec<u32> = vec![1, 2, 3, 4];
    let read_ref: &Vec<u32> = &list;
    // attempting to take the mutable reference is a compile error!
    let write_ref: &mut Vec<u32> = &mut list;
    read_ref == write_ref
}
```

Additionally, references have an associated lifetime which becomes part of their static type. This lifetime is basically just an identifier of a scope (scope is defined by the block of code from when the reference is taken until it is last used). This lifetime identifier tells the compiler where the reference is valid and where it is invalid. As the programmer, you are only allowed to read/modify memory if you have a reference with a valid lifetime. This concept of a lifetime is what makes Rust safe when it comes to memory crashes coming from heap corruptions and use-after-free (dangling pointers). You cannot actually see this lifetime identifier, it is a hidden part of the reference's type. Furthermore, Rust has rules that allow a reference with a narrower lifetime to be "upcast" to a reference with a wider lifetime. Therefore, the only place where you use and see lifetimes is when defining generic functions and generic structs. These need a generic lifetime parameter which is the widest parameter that fits all the references you tag with this generic lifetime parameter.

```
fn correct_use() -> bool {
    let list1: Vec<u32> = vec![1, 2, 3, 4];
    let read_ref1: &Vec<u32> = &list1;
    let this_is_ok = {
        let list2: Vec<u32> = vec![1, 2, 3, 4];
        let read_ref2: &Vec<u32> = &list2;

        let wider_ref = get_wider_lifetime(read_ref1, read_ref2);
        wider_ref
    };    
    read_ref1[0] == this_is_ok[0]
}

fn incorrect_use() -> bool {
    let list1: Vec<u32> = vec![1, 2, 3, 4];
    let read_ref1: &Vec<u32> = &list1;
    // this will not compile
    // attempting to get an immutable reference to list2 by means of returning narrower_ref
    // this is not ok because the lifetime of narrower_ref is no longer valid
    let this_is_not_ok = {
        let list2: Vec<u32> = vec![1, 2, 3, 4];
        let read_ref2: &Vec<u32> = &list2;

        let narrower_ref = get_narrower_lifetime(read_ref1, read_ref2);
        narrower_ref
    };    
    read_ref1 == this_is_not_ok
}

fn get_wider_lifetime<'a, 'b>(read_ref1: &'a Vec<u32>, _read_ref2: &'a Vec<u32>) -> &'a Vec<u32> {
    read_ref1
}

fn get_narrower_lifetime<'a, 'b>(_read_ref1: &'a Vec<u32>, read_ref2: &'b Vec<u32>) -> &'b Vec<u32> {
    read_ref2
}
```

Another concept that is important when it comes to performance is passing-by-value and passing-by-reference. This refers to how memory is treated when it is passed between functions as parameters. Additionally, it is also important to keep in mind what happens when memory is passed out of a function by means of a return. This is where return value optimization and can kick in (RVO). The reason why passing things to functions can hurt performance is copying. If heavy duty copying has to take place anytime you invoke a function with a parameter then this can quickly escalate. Especially, if the programming doesn't do a go job at being clear that copying is taking place. The other question is "What type of copying?". Your memory can be allocated on the stack or it can be allocated on the heap. Stack memory has a very simple allocation and deallocation which doesn't need much additional bookkeeping and furthermore the stack is accessed so often that its memory will most probably be in the architecture's cache. Heap memory on the other hand involves a lot of bookkeeping to avoid fragmentation and it is less likely to be in cache unless you carefully program with cache-friendly access patterns. So, what the programming language should do is:

1. Avoid copying as much as possible.
2. Be clear about when copying is happening and when not.
3. Don't copy things that are on heap without the programmer doing this explicitly.

Rust is very good at points two and three. In Rust you have to either explicitly clone if you want an expensive copy (this would most likely be a copy that involves the heap). Furthermore, rust is very clear about ownership by making a clear distinction about passing ownership of an object or passing a reference to it. Passing a reference is a cheap operation that only involves passing a pointer and thus just a copy of the pointer. This is called pass by reference. Passing ownership of an object means the full object can be moved, because it is no longer valid at the original location. If it is no longer valid, that means no memory needs to be shared and therefore this move becomes a simple `memcpy` of stack memory. It doesn't have to do any heap copying because any heap memory that belong to the moved object will never be used from the original location. Therefore, it is safe to just `memcpy` the stack memory that represents the pointer to this heap memory. The first point about avoiding copying is trickier in Rust. As the programmer you don't have any influence over it and thus you have to assume that `memcpy` on the stack is always taking place if you move ownership of memory. It is up to the compiler to try to do copy-elision and RVO in order to avoid even doing this stack memory `memcpy`. As a programmer, the most you can do is pass references, but in Rust this can lead to fighting with the borrow-checking which verifies lifetimes.

```
struct ObjectWithMemoryOnHeap {
    on_stack: [u32; 10],
    on_heap: Box<u32>,
}

fn get_param_by_value(_obj: ObjectWithMemoryOnHeap) {}

fn get_param_by_reference(_obj: &ObjectWithMemoryOnHeap) {}

fn return_by_value() -> ObjectWithMemoryOnHeap {
    ObjectWithMemoryOnHeap {
        on_stack: [1; 10],
        on_heap: Box::new(2),
    }
}

fn return_by_reference(obj: &ObjectWithMemoryOnHeap) -> &ObjectWithMemoryOnHeap {
    obj
}

fn main() {
    let obj = return_by_value();
    let _obj_ref = return_by_reference(&obj);
    get_param_by_reference(&obj);
    get_param_by_value(obj);
    // obj is no longer valid here ownership has been passed
    // the following is a compile error
    obj.on_stack == 2;
}
```

The final concept that Rust makes heavy use of is a kind of RAII (Resource acquisition is initialization). This concept first appeared in C++ and the idea is to initialize resources (like memory) in the constructor of an object and then rely on the compiler correctly placing the destructor calls when the object is no longer in program scope. The technique happens to be quite handy for memory and so smart pointers in C++ make heavy use of RAII. Rust does all of its memory management by means of RAII. As the programmer, you don't have to worry about free-ing or allocating memory. Instead you just construct objects that can live on the heap like dynamically growing arrays like Rust's `Vec` type or RAII wrappers of heap allocated memory like Rust's `Box` type. When these objects leave program scope, their `Drop` trait implementation is called and this frees their memory. You cannot prevent this from happening as a programmer. The most you can do is force the drop to happen sooner.

```
struct ObjectWithMemoryOnHeap {
    on_stack: u32,
    on_heap: Box<u32>,
}

impl Drop for ObjectWithMemoryOnHeap {
    fn drop(self: &mut Self) {
        println!("Drop ObjectWithMemoryOnHeap called");
    }
}

struct Composite {
    obj: ObjectWithMemoryOnHeap,
}

impl Drop for Composite {
    fn drop(self: &mut Self) {
        println!("Drop composite called");
    }
}

fn main() {
    let obj = ObjectWithMemoryOnHeap {
        on_stack: 1,
        on_heap: Box::new(2),
    };

    let composite = Composite {
        obj,
    };
    // as the program scope is being left everything will be dropped
    // ObjectWithMemoryOnHeap, Composite and also Box<u32>
}
```


This is Rust's approach to memory in a nutshell and it has to be said that it is very elegant. When programming in Rust you don't have to think about whether your program will have memory issues. If you manage to coerce the program to compile it won't have these problems. However, there are two memory areas where Rust is not that elegant. First of all, you can still leak memory if your objects just never leave program scope (for example a `Vec` created at the begininng of `main` which is growing larger and larger) and if your program runs out of heap then it will `panic` and crash. The second area where Rust doesn't shine is giving you the ability to control the memory allocation. This could be used for example to prevent heap fragmentation if you as the programmer know something that the Rust compiler doesn't. You can always use `unsafe` Rust to do allocations yourself. Or use `Vec` and such to simulate your own memory pools, but heap abstractions like `Vec` and `Box` don't allow you to provide your own allocator.

I would say that Rust's pros outweigh its cons. I do have a pet peeve with the language though. Namely its reliance on macros. It has two parallel macro systems, declarative and procedural macros, and quite honestly you need to spend a lot of time learning each if you want to write your own macros or understand macros others have written. In addition to macros, Rust also has a powerful type system with traits and monomorphized generics, expansive pattern matching facilities, async/await, operator overloading and automatic deref. In short, there's a lot to learn to feel like an accomplished Rust programer and I personally wish it was a bit simpler.

# C++
Let's now look in C++ at these the concepts on which Rust does such a stellar job. It should be said that C++ is a language that has evolved a lot and there are many different ways to express things in C++. This can get very confusing to the programmer I feel and it is easy to make mistakes.

First of all. Immutablility can (assuming no use of the `mutable` keyword) be achieved by marking your variables as `const`. Such variables then only allow non-mutable access exactly like Rust's immutable references. The difference is of course that in C++ you can have a `const` and a non-const reference to the same memory active at the same time. Therefore, concurrency data races are definitely possible, but the language does give you a means to have immutability and program in a way that enforces the only-one-mutable-reference restriction yourself should you choose to do so. 

```
struct Foo {
    void operation_that_changes_foo() {};
    void operation_that_doesnt_change_foo() const {};
};

void const_refs() {
    auto foo = Foo {};
    Foo& write_ref = foo;
    const Foo& read_ref = foo;
    
    write_ref.operation_that_changes_foo();
    write_ref.operation_that_doesnt_change_foo();

    read_ref.operation_that_doesnt_change_foo();
    // it is a compile error to try to mutate foo through the read ref
    read_ref.operation_that_changes_foo();   
}
```

When it comes to memory safety, C++ doesn't provide much to protect you from use-after-free on a language level. In fact it does the opposite by providing a number of ways to fairly easily shoot yourself in the foot since it is for example possible to return a reference to memory that is no longer valid. It will warn in this case, but in the language this isn't considered a compile error.

```
struct Foo {};

Foo& return_reference_to_freed_memory() {
    auto foo = Foo {};
    return foo;
}
```
When it comes to memory copying when passing things between functions, then C++ does have means to express through pass-by-value, pass-by-reference, copy constructors and move semantics. The one problem with it is that it is quite difficult to actually understand the exact behaviour due to the non-trivial interactions between all of these concepts (and in fact there are even more concepts like operator=, perfert forwarding and universal references and more!). I would say that if you know what you are doing, then the language gives you the ability to minimize the number of copies and prevent heap copying unless explicitly requested. However, in my opinion it fails to make it clear when copying happens.

```
#include <memory>

struct Foo {
    Foo() = default;
    Foo(const Foo& other) {
        on_stack = other.on_stack;
        // the below is a compile error 
        // the on_heap RAII wrapper doesn't allow copying
        // copying is happening even-though/in-spite-of/because we are using the operator=
        on_heap = other.on_heap;
    }
    Foo(Foo&& other) {
        on_stack = other.on_stack;
        // we can take ownership of it by means of move semantics
        on_heap = std::move(other.on_heap);
    }

    private:
        int on_stack = 1;
        std::unique_ptr<int> on_heap;
};
```

As already mentioned, the concept of RAII originated in C++ and thus it is possible to clearly express how memory should be freed to prevent leaks. For RAII in C++ constructors and destructors are used, with the language guaranteeing that a call to the destructor will be done as the object leaves program scope. The tra The only problem is that the language does allow exceptions to be thrown anywhere including in destructors and constructors which complicates things considerably.

```
struct Foo {
    // this constructor may throw an exception due to out-of-memory on on_heap_big allocation
    // in such a case the destructor will not be called and on_heap_small will leak
    Foo() {
        on_heap_small = new int[1000];
        on_heap_big = new int[1000000];
    }
    ~Foo() {
        std::cout << "In foo destructor\n";
        delete[] on_heap_small;
        delete[] on_heap_big;
    }

    private:
        int on_stack = 1; // on-stack unless Foo itself is on-heap
        int* on_heap_small = nullptr;
        int* on_heap_big = nullptr;
};
```

For controlling allocation patterns, C++ standard library data structures can be parameterized with allocators. Although it has to be said that this is done in a very inelegant way by making the allocator part of the type of the templated data-structure. There is a solution for this issue in the new `pmr` namespace which defines polymorphic allocators so that the allocator in the static type is always the same. Additional blemish on the allocation story is that there are also places where allocations cannot be controlled such as the new [C++20 coroutines where coroutine state is heap allocated](https://en.cppreference.com/w/cpp/language/coroutines).

```
auto vector_with_custom_allocator = std::vector<int, MyAllocator>();
```

So in summary, C++ definitely provides a lot of means to the programmer to understand the memory semantics of his or her program. However, I personally find the provided means are very complicated, inelegant and have a very steep learning curve. This is due to the language evolving and thus not being able to start from scratch which is understandable. It is making an effort to evolve though, and the fact that it has been around for so long means it is very popular and widely used. Therefore, putting in the effort to learn it deeply may be a good time investment. Personally, though I think easier languages will progressibly take more and more mind share away from C++ and it will decline. At the end of the day, things don't have to be this complicated. They are this complicated because the language wasn't designed with these things in mind in the beginning. 


# Zig
If we want something simpler there is a new programming language in development called Zig. This is a programming language that hasn't even reached its 1.0 release and is still highly unstable. However, I think it looks quite promising and it's already starting to generate some excitiment I think. Zig's goal is to be a safer, modern C. Zig does not want to have many language features and it wants to stay low level which also means that you have to manage memory manually. Additionally, Zig doesn't have macros but does let the programmer execute Zig code at compile time. The code that's executed at compile time can create code like lisp can and therefore can be used to create generic data strucutres.


What we are here for is memory though, so let's have a look at the memory semantics of Zig.

Firstly, like C++, Zig also does not inherently prevent data races by requiring only one mutable reference or only multiple immutable references like Rust. It does provide the ability for the programmer to specify that a variable is `const` which means that non of the bytes it owns can be changed. If the struct contains a pointer however, then we can change the memory pointed to by the pointer. 

If we have a `const` variable and we take a pointer to it, then the result is a `* const` which is a pointer to const. This means that what the pointer points at cannot be changed. Not that to get a `* const` from a non-const (meaning `var`) variable we have to use casting or make sure we specify the type and let the cast happen implicitly.

```
const Foo = struct {
    a: u32,
};

fn pointer_to_const() void {
    const foo = Foo { .a = 1 };

    const read_ptr = &foo;
    // we can read through the pointer
    print("{}", .{ read_ptr.*.a }); 
    // we cannot write -> this is a compile error
    read_ptr.*.a = 1;

    // this is compile error. taking the pointer to const data gives const pointer
    const write_ptr: *Foo = &foo;
}

fn pointer_to_non_cost() void {
    var foo = Foo { .a = 1 };

    const read_ptr1: *const Foo = &foo;
    // this is a compile error since the pointer is const
    read_ptr1.a = 1;

    // the const of the variable only means the bytes of the variable cannot change
    // with a pointer however we are changing what it points to
    const not_a_read_ptr = &foo;
    not_a_read_ptr.*.a = 2;

    // we only get a real pointer to const if we cast to *const Foo
    const read_ptr2 = @as(*const Foo, &foo);
    read_ptr2.*.a = 1;
    
    var write_ptr = &foo;
    write_ptr.*.a = 2;
}
```

When it comes to use-after-free and memory corruption, Zig does not prevent you from returning a pointer to freed memory and thus the old foot gun is primed and ready and aimed at the foot.

```
const Foo = struct {};

fn return_ptr_to_freed_memory() *Foo {
    var foo = Foo{};
    return &foo;
}
```

In Zig both pass-by-value and pass-by-refence are present. When doing pass-by-value, the ownership is not transferred. Furthermore, all paramaters passed to a function are `const` by default. This means that pass-by-value only needs to provide an immutable view over the memory you passed in as a parameter. Therefore, it is up to the compile to decide whether to actually `memcpy` and create an on stack copy of the data you passed in. Or the compiler can decide to have the passed value simply point to the memory in its original location, in which case pass-by-value is a transparent pass-by-const-reference. This means that Zig avoids doing unecessary copies when passing data. It also means a possible data race is introduced since we could pass-by-value to another thread and while that thread is running we could modify the contents of the variable. This would result in undefined behaviour due to the fact that if the compiler decided to pass-by-const-reference there could be data races on any modifications since the passed reference points to the original memory. In such cases you have to manually create a copy and pass it.

To be able to modify the memory you passed in, you have to pass-by-reference, which in Zig's case means passing by pointer.

```
const print = @import("std").debug.print;

const Foo = struct {
    a: u32,
};

fn pass_by_value(foo: Foo) void {
    // this is a compile error -> parameters are implicitly const
    foo.a = 1;

    // reading is allowed
    print("{}", .{ foo.a });
}

fn pass_by_pointer(foo: *Foo) void {
    foo.*.a = 1;
}


fn main() void {
    var foo = Foo { .a = 1 };
    pass_by_value(foo);
    pass_by_pointer(&foo);
}
```

When it comes to giving control over memory allocation, Zig does this by requiring that everything that does allocation or deallocation has to take an allocator as a parameter. This includes everything in the Zig standard library itself. This means you have total control over the memory allocation and you can customize it should you wish. Also note that in Zig allocation is an operation that can fail and return an error. Since it returns an error we don't care about handling we use the `try` and that the result is a union of with an inferred error and a void result in case there is no error: `!void`

```
const std = @import("std");

const Foo = struct {
    a: u32,
};

pub fn main() !void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const on_heap: *Foo = try general_purpose_allocator.allocator.create(Foo);
    defer general_purpose_allocator.allocator.destroy(on_heap);
    on_heap.*.a = 3;
    std.debug.print("{}", .{ on_heap.a });
}
```

Zig does not have constructors and desctructors like C++ nor does it have an implicit `drop` like Rust so RAII is not possible in Zig to take care of freeing memory. Instead, Zig uses the `defer` and `errdefer` keywords and advises that the program uses these whenever allocating memory. The `defer expression;` statement will make it so that `expression` is executed when the scope where of this defer statement is exited normally. The `errdefer expression;` statement will work in a similar fashion, except the execution will only take place if the scope is left due to an error (Zig allows that a function exits either with a result or with an error. The error will cause an immediate exit much like an exception). Any `defer` and `errdefer` statements are executed in reverse order to how they are encountered, so you can think of `defer expression;` as putting `expression` on a stack and that as the scope is being exited this stack is executed from top to bottom. Defer makes for very elegant manual memory management since you can put the statement that frees memory right under where the memory is allocated and that's where the free makes the most sense semantically. The errdefer on the other hand is useful to deal with cases where you allocated memory but due to an error you don't want to keep working with it or return it. The errdefer allows you to throw it away immediately. 

The class of problems that `defer` and `errdefer` don't solve is about freeing memory when ownership of an object is being changed. For example, if you place objects into a list, then you would like the list to become the owner of these object and have the object's free their memory when the list is destroyed. In Zig, you would have to be careful to free this memory yourself. Or possibly use a clever compiletime code generation to do it for you.

```
const std = @import("std");

const Foo = struct {
    a: u32,
};

var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};

fn alloc_and_free() !void {
    const on_heap: *Foo = try general_purpose_allocator.allocator.create(Foo);
    defer general_purpose_allocator.allocator.destroy(on_heap);
}

fn alloc_and_return_unless_error() !*Foo {
    const on_heap: *Foo = try general_purpose_allocator.allocator.create(Foo);
    errdefer general_purpose_allocator.allocator.destroy(on_heap);

    // this big allocation could fail. in this case we still want on_heap to be destroyed
    // luckily the errdefer we used earlier will trigger in that case
    const on_heap_big = try general_purpose_allocator.allocator.alloc(u32, 10000);
    defer general_purpose_allocator.allocator.free(on_heap_big);

    return on_heap;
}

pub fn main() !void {
    try alloc_and_free();
    _ = try alloc_and_return_unless_error();
}
```


# Garbage collection - Nim, Go and D
Finally, I would be remiss to not discuss garbage collection (GC) as a strategy for handling memory. In fact, this may be the best strategy for handling memory for 99% of the use cases that programmers have to deal with. Garbage collection is the idea of putting an additional abstraction layer into the language and it is known that 

> Any problem in computer science can be solved by adding another abstraction layer.

The abstraction layer in the case of garbage collection is to let the programming language have a runtime and have this runtime manage memory. This means that something is running which you ask for memory whenever you want memory and which reclaims memory it handed out when this memory is no longer needed. Many programmers find this alarming and prefer to be in control over their own memory. It is assumed that garbage collection results in worse performance than manual memory management (or rather no-runtime memory management as in the case of rust there is no manual management either). However, it is quite possible that this is not the case and quite a lot of research and measuring has gone into establishing whether garbage collection doesn't actually result in faster programs than manual memory management. Most probably, the answer is that a perfectly tweaked manually managed program is always better but even this isn't so certain. Famously, just-in-time compiled programs can actually perform better than pre-compiled programs because the just-in-time compilation can adapt to the usage patterns intelligently and optimize the code with the additional information of how it is being used. This is information a compiler doesn't have and thus cannot make such optimizations. It is quite possible that garbage collection may be able to achieve similar things.

It is however true that there is a runtime because the garbage collection needs to be able to find and reclaim memory that is no longer used. Very clever garbage collection algorithms guarnatee a pause of less than a millisecond and I believe both Go and D make this guarnatee. This pause, which is most likely going to happen at a random time in the program's execution is often a big argument against having GC in embedded system. Game programers seem to also consider this a deal breaker although in this case the claim that a <1ms pause every x frame or so is a deal breaker sounds far fetched to me, even if at 60fps you only have ~16ms to work with each frame.

What I do think is a problem is that with garbage collection the programmer stops thinking about memory. This can cause programmers to be very cavalier with memory and not use good memory access patterns which minimize cache misses. It also becomes very difficult for programmers to optimize their program with respect to memory. You cannot for example provide a custom allocator for your List. Equally, garbage collection by itself doesn't help with data races because the runtime only takes care of allocating and then relcaiming memory. Presumably, the runtime could do this similarily how Rust's `RefCell` checks at runtime that you cannot borrow mutably a cell that is already borrowed.

Personally, I am conflicted when it comes to garbage collection. I think it may be the optimal choice, but I am interested in learning how to manage memory correctly and I know I won't be able to learn this when programming in a language with a garbage collector.

# Verdict
First of all, I didn't talk about three other languages that definitely should be taken into account as well: Odin, V and Jai. Jai is in closed beta at the moment, but should be in open beta in 2021. Odin is very similar to Jai and V is similar to Go but without GC. 

Second of all, it is important to click with the language so if any of the examples don't click with you then ignore the verdict. The verdict assumes that everything clicks equally.

With that being said, if you are looking to have a fast program and fewer worries about memory your choice should be between Rust and languages with garbage collection. Which of those two you choose I'd say goes as follows:

1. If you are looking to learn a difficult language and you are excited to have a lot of things to learn choose Rust
2. Otherwise pick a programming language with GC and don't pay attention to the FUD (fear, uncertainty and doubt) promulgated about GC. It's not going to be the thing that makes your program slow.

If you do want to worry about memory and have a fast program after you solve all your worries choose C++ or Zig. In this case the choice should be as follows:

1. If you are working on a hobby project that doesn't need a stable language or if you don't want to invest a lot of time in learning a language go for Zig (or Odin, V, Jai or really maybe a GC language if you don't want to worry about memory THAT badly)
2. If you are excited about a learning the many nooks and cranies of a programming language or if you need a very mature language then choose C++