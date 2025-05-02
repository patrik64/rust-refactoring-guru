<p align="left">
<img src="https://github.com/user-attachments/assets/440229f7-b8a2-4dc8-95c8-799b131d8722" width="200" />
</p>

# [Composite](https://refactoring.guru/design-patterns/composite)

## Intent

**Composite** is a structural design pattern that lets you compose objects into tree structures and then work with these structures as if they were individual objects.

## Problem

Using the Composite pattern makes sense only when the core model of your app can be represented as a tree.

For example, imagine that you have two types of objects: ``Products`` and ``Boxes``. A ``Box`` can contain several ``Products`` as well as a number of smaller ``Boxes``.  
These little ``Boxes`` can also hold some ``Products`` or even smaller ``Boxes``, and so on.

Say you decide to create an ordering system that uses these classes.  
Orders could contain simple products without any wrapping, as well as boxes stuffed with products...and other boxes.  
How would you determine the total price of such an order?

<p align="center">
<img src="https://github.com/user-attachments/assets/f7389919-31c3-46cc-b912-9c3a6ce0de83" width="400" />
</p>


## Solution

The Composite pattern suggests that you work with ``Products`` and ``Boxes`` through a common interface which declares a method for calculating the total price.

How would this method work? For a product, it’d simply return the product’s price. 
For a box, it’d go over each item the box contains, ask its price and then return a total for this box.  
If one of these items were a smaller box, that box would also start going over its contents and so on, until the prices of all inner components were calculated.  
A box could even add some extra cost to the final price, such as packaging cost.

The greatest benefit of this approach is that you don’t need to care about the concrete classes of objects that compose the tree. You don’t need to know whether an object is a simple product or a sophisticated box. You can treat them all the same via the common interface. When you call a method, the objects themselves pass the request down the tree.

## Real-World Analogy

<p align="center">
<img src="https://github.com/user-attachments/assets/b9316dd6-c365-480b-884a-650aac64d9fa" width="400" />
</p>


Armies of most countries are structured as hierarchies. An army consists of several divisions; a division is a set of brigades, and a brigade consists of platoons, which can be broken down into squads. Finally, a squad is a small group of real soldiers. Orders are given at the top of the hierarchy and passed down onto each level until every soldier knows what needs to be done.

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You can work with complex tree structures more conveniently: use polymorphism and recursion to your advantage.|☒ It might be difficult to provide a common interface for classes whose functionality differs too much. In certain scenarios, you’d need to overgeneralize the component interface, making it harder to comprehend.|
|☑ *Open/Closed Principle.* You can introduce new element types into the app without breaking the existing code, which now works with the object tree.|☒ |

