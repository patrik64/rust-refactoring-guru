<p align="left">
<img src="https://github.com/user-attachments/assets/4f7c8f2b-3e9e-4979-8fa0-30a7c7ef8c78" width="200" />
</p>

# [Iterator](https://refactoring.guru/design-patterns/iterator)

## Intent

**Iterator** is a behavioral design pattern that lets you traverse elements of a collection without exposing its underlying representation (list, stack, tree, etc.).

## Problem

Collections are one of the most used data types in programming. Nonetheless, a collection is just a container for a group of objects. 

<p align="center">
<img src="https://github.com/user-attachments/assets/208a9e5e-829f-4914-a91a-e2d94236af60" width="400" />
</p>


Most collections store their elements in simple lists.  
However, some of them are based on stacks, trees, graphs and other complex data structures.

But no matter how a collection is structured, it must provide some way of accessing its elements so that other code can use these elements.  
There should be a way to go through each element of the collection without accessing the same elements over and over.

This may sound like an easy job if you have a collection based on a list. You just loop over all of the elements.  
But how do you sequentially traverse elements of a complex data structure, such as a tree? For example, one day you might be just fine with depth-first traversal of a tree.  
Yet the next day you might require breadth-first traversal. And the next week, you might need something else, like random access to the tree elements. 

<p align="center">
<img src="https://github.com/user-attachments/assets/72bb5409-796f-4a93-993d-4217ecd95530" width="400" />
</p>

Adding more and more traversal algorithms to the collection gradually blurs its primary responsibility, which is efficient data storage.  
Additionally, some algorithms might be tailored for a specific application, so including them into a generic collection class would be weird.

On the other hand, the client code that’s supposed to work with various collections may not even care how they store their elements.  
However, since collections all provide different ways of accessing their elements, you have no option other than to couple your code to the specific collection classes. 

## Solution

The main idea of the Iterator pattern is to extract the traversal behavior of a collection into a separate object called an *iterator*.


<p align="center">
<img src="https://github.com/user-attachments/assets/192b276e-1fad-4cdb-a684-9932fd2593bd" width="400" />
</p>

In addition to implementing the algorithm itself, an iterator object encapsulates all of the traversal details, such as the current position and how many elements are left till the end. Because of this, several iterators can go through the same collection at the same time, independently of each other.

Usually, iterators provide one primary method for fetching elements of the collection. The client can keep running this method until it doesn’t return anything, which means that the iterator has traversed all of the elements.

All iterators must implement the same interface. This makes the client code compatible with any collection type or any traversal algorithm as long as there’s a proper iterator. If you need a special way to traverse a collection, you just create a new iterator class, without having to change the collection or the client. 

## Real-World Analogy

You plan to visit Rome for a few days and visit all of its main sights and attractions. But once there, you could waste a lot of time walking in circles, unable to find even the Colosseum.

On the other hand, you could buy a virtual guide app for your smartphone and use it for navigation. It’s smart and inexpensive, and you could be staying at some interesting places for as long as you want.

A third alternative is that you could spend some of the trip’s budget and hire a local guide who knows the city like the back of his hand. The guide would be able to tailor the tour to your likings, show you every attraction and tell a lot of exciting stories. That’ll be even more fun; but, alas, more expensive, too.

All of these options—the random directions born in your head, the smartphone navigator or the human guide—act as iterators over the vast collection of sights and attractions located in Rome.

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ *Single Responsibility Principle.* You can clean up the client code and the collections by extracting bulky traversal algorithms into separate classes.|☒ Applying the pattern can be an overkill if your app only works with simple collections.|
|☑ *Open/Closed Principle.* You can implement new types of collections and iterators and pass them to existing code without breaking anything.|☒ Using an iterator may be less efficient than going through elements of some specialized collections directly.|
|☑ You can iterate over the same collection in parallel because each iterator object contains its own iteration state.||
|☑ For the same reason, you can delay an iteration and continue it when needed.||
