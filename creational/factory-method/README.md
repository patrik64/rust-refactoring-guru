<p align="left">
<img src="https://github.com/user-attachments/assets/75d57774-1585-43d5-ae59-48a202c44466" width="200" />
</p>

# [Factory Method](https://refactoring.guru/design-patterns/factory-method)

## Intent

**Factory Method** is a creational design pattern that provides an interface for creating objects in a superclass, but allows subclasses to alter the type of objects that will be created.

## Problem

Imagine that you’re creating a logistics management application.  
The first version of your app can only handle transportation by trucks, so the bulk of your code lives inside the ``Truck`` class.

After a while, your app becomes pretty popular. Each day you receive dozens of requests from sea transportation companies to incorporate sea logistics into the app.

<p align="center">
<img src="https://github.com/user-attachments/assets/b209d6ce-0104-448f-877e-5b39e9aa80c0" width="400" />
</p>

Great news, right? But how about the code?  
At present, most of your code is coupled to the ``Truck`` class. 
Adding ``Ships`` into the app would require making changes to the entire codebase.  
Moreover, if later you decide to add another type of transportation to the app, you will probably need to make all of these changes again.

As a result, you will end up with pretty nasty code, riddled with conditionals that switch the app’s behavior depending on the class of transportation objects.


## Solution

The Factory Method pattern suggests that you replace direct object construction calls (using the ``new`` operator) with calls to a special *factory* method.  
Don’t worry: the objects are still created via the ``new`` operator, but it’s being called from within the factory method.  
Objects returned by a factory method are often referred to as *products*.

<p align="center">
<img src="https://github.com/user-attachments/assets/fc4de097-2e87-4142-b385-d62a3625f384" width="400" />
</p>

At first glance, this change may look pointless: we just moved the constructor call from one part of the program to another.  
However, consider this: now you can override the factory method in a subclass and change the class of products being created by the method. 

There’s a slight limitation though: subclasses may return different types of products only if these products have a common base class or interface.  
Also, the factory method in the base class should have its return type declared as this interface.

<p align="center">
<img src="https://github.com/user-attachments/assets/b210d48b-2ac2-4071-b116-38cb599f08ca" width="400" />
</p>


For example, both ``Truck`` and ``Ship`` classes should implement the Transport interface, which declares a method called deliver.  
Each class implements this method differently: trucks deliver cargo by land, ships deliver cargo by sea.  
The factory method in the ``RoadLogistics`` class returns truck objects, whereas the factory method in the ``SeaLogistics`` class returns ships.

<p align="center">
<img src="https://github.com/user-attachments/assets/f5b5efeb-fb08-45ad-b52a-fd2050563c4d" width="400" />
</p>

The code that uses the factory method (often called the client code) doesn’t see a difference between the actual products returned by various subclasses. 
The client treats all the products as abstract ``Transport``.  
The client knows that all transport objects are supposed to have the deliver method, but exactly how it works isn’t important to the client.


## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You avoid tight coupling between the creator and the concrete products.|☒ The code may become more complicated since you need to introduce a lot of new subclasses to implement the pattern. The best case scenario is when you’re introducing the pattern into an existing hierarchy of creator classes. |
|☑ *Single Responsibility Principle.* You can move the product creation code into one place in the program, making the code easier to support.||
|☑ *Open/Closed Principle.* You can introduce new types of products into the program without breaking existing client code.||








