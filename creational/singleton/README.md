<p align="left">
<img src="https://github.com/user-attachments/assets/3e161eeb-e5a7-4a93-80b6-529e4c160551" width="200" />
</p>

# [Singleton](https://refactoring.guru/design-patterns/singleton)

## Intent

**Singleton** is a creational design pattern that lets you ensure that a class has only one instance, while providing a global access point to this instance.

## Problem
The Singleton pattern solves two problems at the same time, violating the *Single Responsibility Principle*:


1. Ensure that a class has just a single instance. Why would anyone want to control how many instances a class has? The most common reason for this is to control access to some shared resource—for example, a database or a file.
   Here’s how it works: imagine that you created an object, but after a while decided to create a new one. Instead of receiving a fresh object, you’ll get the one you already created.
   Note that this behavior is impossible to implement with a regular constructor since a constructor call must always return a new object by design.



2. Provide a global access point to that instance. Remember those global variables that you (all right, me) used to store some essential objects? While they’re very handy, they’re also very unsafe since any code can potentially overwrite the contents of those variables and crash the app.
   Just like a global variable, the Singleton pattern lets you access some object from anywhere in the program. However, it also protects that instance from being overwritten by other code.
   There’s another side to this problem: you don’t want the code that solves problem #1 to be scattered all over your program. It’s much better to have it within one class, especially if the rest of your code already depends on it.

Nowadays, the Singleton pattern has become so popular that people may call something a singleton even if it solves just one of the listed problems.

## Solution

All implementations of the Singleton have these two steps in common:

- Make the default constructor private, to prevent other objects from using the new operator with the Singleton class.
- Create a static creation method that acts as a constructor. Under the hood, this method calls the private constructor to create an object and saves it in a static field. All following calls to this method return the cached object.

If your code has access to the Singleton class, then it’s able to call the Singleton’s static method. So whenever that method is called, the same object is always returned.

## Real-World Analogy

The government is an excellent example of the Singleton pattern. A country can have only one official government. Regardless of the personal identities of the individuals who form governments, the title, “The Government of X”, is a global point of access that identifies the group of people in charge.

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You can be sure that a class has only a single instance.| ☒  Violates the *Single Responsibility Principle*. The pattern solves two problems at the time.|
|☑ You gain a global access point to that instance.| ☒ The Singleton pattern can mask bad design, for instance, when the components of the program know too much about each other.|
|☑ The singleton object is initialized only when it’s requested for the first time.| ☒  The pattern requires special treatment in a multithreaded environment so that multiple threads won’t create a singleton object several times.|
|| ☒      It may be difficult to unit test the client code of the Singleton because many test frameworks rely on inheritance when producing mock objects. Since the constructor of the singleton class is private and overriding static methods is impossible in most languages, you will need to think of a creative way to mock the singleton. Or just don’t write the tests. Or don’t use the Singleton pattern.|
