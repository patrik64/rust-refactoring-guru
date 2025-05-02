![grafik](https://github.com/user-attachments/assets/262a6d9c-1aea-4716-aa6b-493ddafecf7f)

# [Mediator](https://refactoring.guru/design-patterns/mediator)

## Intent

**Mediator** is a behavioral design pattern that lets you reduce chaotic dependencies between objects.  
The pattern restricts direct communications between the objects and forces them to collaborate only via a mediator object. 

## Problem

Say you have a dialog for creating and editing customer profiles.  
It consists of various form controls such as text fields, checkboxes, buttons, etc. 
Chaotic relations between elements of the user interface


<p align="center">
<img src="https://github.com/user-attachments/assets/85fd8178-1232-413b-b248-75a51edf8a26" width="500" />
</p>

Some of the form elements may interact with others.  
For instance, selecting the “I have a dog” checkbox may reveal a hidden text field for entering the dog’s name.  
Another example is the submit button that has to validate values of all fields before saving the data. 

<p align="center">
<img src="https://github.com/user-attachments/assets/7eaee3b9-0c60-4ae2-875f-846c0b2c4112" width="500" />
</p>

By having this logic implemented directly inside the code of the form elements you make these elements’ classes much harder to reuse in other forms of the app.  
For example, you won’t be able to use that checkbox class inside another form, because it’s coupled to the dog’s text field.  
You can use either all the classes involved in rendering the profile form, or none at all. 

## Solution

The Mediator pattern suggests that you should cease all direct communication between the components which you want to make independent of each other.  
Instead, these components must collaborate indirectly, by calling a special mediator object that redirects the calls to appropriate components.  
As a result, the components depend only on a single mediator class instead of being coupled to dozens of their colleagues. 

In our example with the profile editing form, the dialog class itself may act as the mediator.  
Most likely, the dialog class is already aware of all of its sub-elements, so you won’t even need to introduce new dependencies into this class. 

<p align="center">
<img src="https://github.com/user-attachments/assets/819e6f7a-99bc-44f3-99ed-00caa9dd04f8" width="500" />
</p> 

The most significant change happens to the actual form elements.  
Let’s consider the submit button.  
Previously, each time a user clicked the button, it had to validate the values of all individual form elements.  
Now its single job is to notify the dialog about the click. Upon receiving this notification, the dialog itself performs the validations or passes the task to the individual elements. 
Thus, instead of being tied to a dozen form elements, the button is only dependent on the dialog class. 

You can go further and make the dependency even looser by extracting the common interface for all types of dialogs.  
The interface would declare the notification method which all form elements can use to notify the dialog about events happening to those elements.  
Thus, our submit button should now be able to work with any dialog that implements that interface. 

This way, the Mediator pattern lets you encapsulate a complex web of relations between various objects inside a single mediator object.  
The fewer dependencies a class has, the easier it becomes to modify, extend or reuse that class. 

## Real-World Analogy

Pilots of aircraft that approach or depart the airport control area don’t communicate directly with each other.  
Instead, they speak to an air traffic controller, who sits in a tall tower somewhere near the airstrip.  
Without the air traffic controller, pilots would need to be aware of every plane in the vicinity of the airport,  
discussing landing priorities with a committee of dozens of other pilots.  
That would probably skyrocket the airplane crash statistics.

The tower doesn’t need to control the whole flight.  
It exists only to enforce constraints in the terminal area because the number of involved actors there might be overwhelming to a pilot. 

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ *Single Responsibility Principle.* You can extract the communications between various components into a single place, making it easier to comprehend and maintain.| ☒ Over time a mediator can evolve into a **God Object**.|
|☑ *Open/Closed Principle.* You can introduce new mediators without having to change the actual components.||
|☑ You can reduce coupling between various components of a program.||
|☑ You can reuse individual components more easily.||








