![image](https://github.com/user-attachments/assets/cd80bb7a-9d62-4c90-9561-a1e83bec96b6)

# [Bridge](https://refactoring.guru/design-patterns/bridge)

## Intent

**Bridge** is a structural design pattern that lets you split a large class or a set of closely related classes into  
two separate hierarchies—abstraction and implementation—which can be developed independently of each other.

## Problem

Say you have a geometric ``Shape`` class with a pair of subclasses: ``Circle`` and ``Square``.  
You want to extend this class hierarchy to incorporate colors, so you plan to create ``Red`` and ``Blue`` shape subclasses.  
However, since you already have two subclasses, you’ll need to create four class combinations such as ``BlueCircle`` and ``RedSquare``. 


<p align="center">
<img src="https://github.com/user-attachments/assets/5d31600c-331b-486f-966b-b2600f8d0b68" width="400" />
</p> 

Adding new shape types and colors to the hierarchy will grow it exponentially.  
For example, to add a triangle shape you’d need to introduce two subclasses, one for each color.  
And after that, adding a new color would require creating three subclasses, one for each shape type.  
The further we go, the worse it becomes.



| Pros | Cons |
| ----------- | ----------- |
|☑ You can create platform-independent classes and apps.| ☒ You might make the code more complicated by applying the pattern to a highly cohesive class.|
|☑ The client code works with high-level abstractions. It isn’t exposed to the platform details.||
|☑ *Open/Closed Principle*. You can introduce new abstractions and implementations independently from each other.||
|☑ *Single Responsibility Principle*. You can focus on high-level logic in the abstraction and on platform details in the implementation.||
