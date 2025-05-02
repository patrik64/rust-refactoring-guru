<p align="left">
<img src="https://github.com/user-attachments/assets/58508b78-cea7-4677-af12-99eede41cd60" width="200" />
</p>

# [Template Method](https://refactoring.guru/design-patterns/template-method)

## Intent

**Template Method** is a behavioral design pattern that defines the skeleton of an algorithm in the superclass but lets subclasses override specific steps of the algorithm without changing its structure.

## Problem

Imagine that you’re creating a data mining application that analyzes corporate documents.  
Users feed the app documents in various formats (PDF, DOC, CSV), and it tries to extract meaningful data from these docs in a uniform format. 

The first version of the app could work only with DOC files. In the following version, it was able to support CSV files.  
A month later, you “taught” it to extract data from PDF files.

<p align="center">
<img src="https://github.com/user-attachments/assets/c0ec2812-2764-4bb4-b88c-d744df5cc3a8" width="500" />
</p>

At some point, you noticed that all three classes have a lot of similar code.  
While the code for dealing with various data formats was entirely different in all classes, the code for data processing and analysis is almost identical.  
Wouldn’t it be great to get rid of the code duplication, leaving the algorithm structure intact?

There was another problem related to client code that used these classes.  
It had lots of conditionals that picked a proper course of action depending on the class of the processing object.  
If all three processing classes had a common interface or a base class, you’d be able to eliminate the conditionals in client code and use polymorphism when calling methods on a processing object.

## Solution

The Template Method pattern suggests that you break down an algorithm into a series of steps, turn these steps into methods, and put a series of calls to these methods inside a single *template method*.  
The steps may either be abstract, or have some default implementation.  
To use the algorithm, the client is supposed to provide its own subclass, implement all abstract steps, and override some of the optional ones if needed (but not the template method itself).

Let’s see how this will play out in our data mining app. We can create a base class for all three parsing algorithms.  
This class defines a template method consisting of a series of calls to various document-processing steps.

<p align="center">
<img src="https://github.com/user-attachments/assets/3ff2f7ad-1e8c-492d-9cde-4b62e2a71965" width="500" />
</p>

At first, we can declare all steps abstract, forcing the subclasses to provide their own implementations for these methods.  
In our case, subclasses already have all necessary implementations, so the only thing we might need to do is adjust signatures of the methods to match the methods of the superclass. 

Now, let’s see what we can do to get rid of the duplicate code.  
It looks like the code for opening/closing files and extracting/parsing data is different for various data formats, so there’s no point in touching those methods.  
However, implementation of other steps, such as analyzing the raw data and composing reports, is very similar, so it can be pulled up into the base class, where subclasses can share that code.

As you can see, we’ve got two types of steps:

- abstract steps must be implemented by every subclass
- optional steps already have some default implementation, but still can be overridden if needed

There’s another type of step, called hooks. A hook is an optional step with an empty body.  
A template method would work even if a hook isn’t overridden. Usually, hooks are placed before and after crucial steps of algorithms, providing subclasses with additional extension points for an algorithm.

## Real-World Analogy

The template method approach can be used in mass housing construction.  
The architectural plan for building a standard house may contain several extension points that would let a potential owner adjust some details of the resulting house.

Each building step, such as laying the foundation, framing, building walls, installing plumbing and wiring for water and electricity, etc.,  
can be slightly changed to make the resulting house a little bit different from others. 

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You can let clients override only certain parts of a large algorithm, making them less affected by changes that happen to other parts of the algorithm.| ☒ Some clients may be limited by the provided skeleton of an algorithm.|
|☑ You can pull the duplicate code into a superclass.| ☒ You might violate the *Liskov Substitution Principle* by suppressing a default step implementation via a subclass.|
|| ☒ Template methods tend to be harder to maintain the more steps they have.|



