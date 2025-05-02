<p align="left">
<img src="https://github.com/user-attachments/assets/57c8f56d-74ea-495a-a97d-3a0edc161670" width="200" />
</p>

# [Abstract Factory](https://refactoring.guru/design-patterns/abstract-factory)

## Intent

**Abstract Factory** is a creational design pattern that lets you produce families of related objects without specifying their concrete classes.

## Problem

Imagine that you’re creating a furniture shop simulator. Your code consists of classes that represent:

- A family of related products, say: ``Chair`` + ``Sofa`` + ``CoffeeTable``.

- Several variants of this family. For example, products ``Chair`` + ``Sofa`` + ``CoffeeTable`` are available in these variants: ``Modern``, ``Victorian``, ``ArtDeco``.

<p align="center">
<img src="https://github.com/user-attachments/assets/6ca00d52-d01c-4a8a-9c10-679d5f300e57" width="400" />
</p>

You need a way to create individual furniture objects so that they match other objects of the same family. 
Customers get quite mad when they receive non-matching furniture. 

Also, you don’t want to change existing code when adding new products or families of products to the program.  
Furniture vendors update their catalogs very often, and you wouldn’t want to change the core code each time it happens. 

## Solution

The first thing the Abstract Factory pattern suggests is to explicitly declare interfaces for each distinct product of the product family (e.g., chair, sofa or coffee table).  
Then you can make all variants of products follow those interfaces.  
For example, all chair variants can implement the ``Chair`` interface; all coffee table variants can implement the ``CoffeeTable`` interface, and so on. 

<p align="center">
<img src="https://github.com/user-attachments/assets/1bb3a86a-2c48-42f4-8c2c-5d5da7edf520" width="400" />
</p>

The next move is to declare the Abstract Factory—an interface with a list of creation methods for all products that are part of the product family  
(for example, ``createChair``, ``createSofa`` and ``createCoffeeTable``).  
These methods must return **abstract** product types represented by the interfaces we extracted previously: ``Chair``, ``Sofa``, ``CoffeeTable`` and so on. 

<p align="center">
<img src="https://github.com/user-attachments/assets/0cd9e6a2-54b4-414c-a4ec-b267330d898f" width="400" />
</p>

Now, how about the product variants? For each variant of a product family, we create a separate factory class based on the ``AbstractFactory`` interface.  
A factory is a class that returns products of a particular kind. For example, the ``ModernFurnitureFactory`` can only create ``ModernChair, ``ModernSofa`` and ``ModernCoffeeTable`` objects.

The client code has to work with both factories and products via their respective abstract interfaces.  
This lets you change the type of a factory that you pass to the client code, as well as the product variant that the client code receives, without breaking the actual client code.

Say the client wants a factory to produce a chair.  
The client doesn’t have to be aware of the factory’s class, nor does it matter what kind of chair it gets. Whether it’s a Modern model or a Victorian-style chair,  
the client must treat all chairs in the same manner, using the abstract Chair interface.  
With this approach, the only thing that the client knows about the chair is that it implements the sitOn method in some way.  
Also, whichever variant of the chair is returned, it’ll always match the type of sofa or coffee table produced by the same factory object.

There’s one more thing left to clarify: if the client is only exposed to the abstract interfaces, what creates the actual factory objects?  
Usually, the application creates a concrete factory object at the initialization stage.  
Just before that, the app must select the factory type depending on the configuration or the environment settings.

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You can be sure that the products you’re getting from a factory are compatible with each other.|☒ The code may become more complicated than it should be, since a lot of new interfaces and classes are introduced along with the pattern.|
|☑ You avoid tight coupling between concrete products and client code.||
|☑ *Single Responsibility Principle.* You can extract the product creation code into one place, making the code easier to support.||
|☑ *Open/Closed Principle.* You can introduce new variants of products without breaking existing client code.||





