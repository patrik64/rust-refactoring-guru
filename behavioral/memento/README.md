<p align="left">
<img src="https://github.com/user-attachments/assets/ebeb847c-108f-4b60-80f5-f619320a9db1" width="200" />
</p>

# [Memento](https://refactoring.guru/design-patterns/memento)

## Intent

**Memento** is a behavioral design pattern that lets you save and restore the previous state of an object without revealing the details of its implementation. 

## Problem

Imagine that you’re creating a text editor app.  
In addition to simple text editing, your editor can format text, insert inline images, etc. 

At some point, you decided to let users undo any operations carried out on the text.  
This feature has become so common over the years that nowadays people expect every app to have it.  
For the implementation, you chose to take the direct approach. Before performing any operation, the app records the state of all objects and saves it in some storage.  
Later, when a user decides to revert an action, the app fetches the latest snapshot from the history and uses it to restore the state of all objects. 

<p align="center">
<img src="https://github.com/user-attachments/assets/30700712-771c-41ef-acb9-f7413a53ecb0" width="500" />
</p>

Let’s think about those state snapshots.  
How exactly would you produce one? You’d probably need to go over all the fields in an object and copy their values into storage.  
However, this would only work if the object had quite relaxed access restrictions to its contents.  
Unfortunately, most real objects won’t let others peek inside them that easily, hiding all significant data in private fields. 

Ignore that problem for now and let’s assume that our objects behave like hippies: preferring open relations and keeping their state public. 
While this approach would solve the immediate problem and let you produce snapshots of objects’ states at will, it still has some serious issues.  
In the future, you might decide to refactor some of the editor classes, or add or remove some of the fields.  
Sounds easy, but this would also require changing the classes responsible for copying the state of the affected objects. 

<p align="center">
<img src="https://github.com/user-attachments/assets/94113667-4dbb-4de7-8752-1afcf47d6ecf" width="500" />
</p>

But there’s more. Let’s consider the actual “snapshots” of the editor’s state. What data does it contain? 
At a bare minimum, it must contain the actual text, cursor coordinates, current scroll position, etc.  
To make a snapshot, you’d need to collect these values and put them into some kind of container.

Most likely, you’re going to store lots of these container objects inside some list that would represent the history.  
Therefore the containers would probably end up being objects of one class. 
The class would have almost no methods, but lots of fields that mirror the editor’s state.  
To allow other objects to write and read data to and from a snapshot, you’d probably need to make its fields public.  
That would expose all the editor’s states, private or not.  
Other classes would become dependent on every little change to the snapshot class, which would otherwise happen within private fields and methods without affecting outer classes.

It looks like we’ve reached a dead end: you either expose all internal details of classes, making them too fragile,  
or restrict access to their state, making it impossible to produce snapshots. Is there any other way to implement the "undo"? 

## Solution

All problems that we’ve just experienced are caused by broken encapsulation.  
Some objects try to do more than they are supposed to.  
To collect the data required to perform some action, they invade the private space of other objects instead of letting these objects perform the actual action.

The Memento pattern delegates creating the state snapshots to the actual owner of that state, the originator object.  
Hence, instead of other objects trying to copy the editor’s state from the “outside,” the editor class itself can make the snapshot since it has full access to its own state.

The pattern suggests storing the copy of the object’s state in a special object called memento.  
The contents of the memento aren’t accessible to any other object except the one that produced it.  
Other objects must communicate with mementos using a limited interface which may allow fetching the snapshot’s metadata (creation time, the name of the performed operation, etc.),  
but not the original object’s state contained in the snapshot. 

<p align="center">
<img src="https://github.com/user-attachments/assets/5468e85a-68bc-4e96-b3a2-a087c0bc8a87" width="500" />
</p>

Such a restrictive policy lets you store mementos inside other objects, usually called caretakers.  
Since the caretaker works with the memento only via the limited interface, it’s not able to tamper with the state stored inside the memento.  
At the same time, the originator has access to all fields inside the memento, allowing it to restore its previous state at will.

In our text editor example, we can create a separate history class to act as the caretaker.  
A stack of mementos stored inside the caretaker will grow each time the editor is about to execute an operation.  
You could even render this stack within the app’s UI, displaying the history of previously performed operations to a user.

When a user triggers the undo, the history grabs the most recent memento from the stack and passes it back to the editor, requesting a roll-back.  
Since the editor has full access to the memento, it changes its own state with the values taken from the memento. 

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ You can produce snapshots of the object’s state without violating its encapsulation.| ☒ The app might consume lots of RAM if clients create mementos too often.|
|☑ You can simplify the originator’s code by letting the caretaker maintain the history of the originator’s state.| ☒ Caretakers should track the originator’s lifecycle to be able to destroy obsolete mementos.|
|| ☒ Most dynamic programming languages, such as PHP, Python and JavaScript, can’t guarantee that the state within the memento stays untouched.|




