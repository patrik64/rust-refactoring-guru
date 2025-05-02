![grafik](https://github.com/user-attachments/assets/e091d6bf-4c1f-435e-982e-767c56ba7a59)

# [Observer](https://refactoring.guru/design-patterns/observer)

## Intent

**Observer** is a behavioral design pattern that lets you define a subscription mechanism to notify multiple objects about any events that happen to the object they’re observing.

## Problem

Imagine that you have two types of objects: a Customer and a Store.  
The customer is very interested in a particular brand of product (say, it’s a new model of the iPhone) which should become available in the store very soon. 

The customer could visit the store every day and check product availability.  
But while the product is still en route, most of these trips would be pointless. 

<p align="center">
<img src="https://github.com/user-attachments/assets/25e54cba-e03a-4896-9aea-fc48dd9764c0" width="500" />
</p> 

On the other hand, the store could send tons of emails (which might be considered spam) to all customers each time a new product becomes available.  
This would save some customers from endless trips to the store. At the same time, it’d upset other customers who aren’t interested in new products. 

It looks like we’ve got a conflict.  
Either the customer wastes time checking product availability or the store wastes resources notifying the wrong customers. 

## Solution

The object that has some interesting state is often called subject, but since it’s also going to notify other objects about the changes to its state, we’ll call it publisher.  
All other objects that want to track changes to the publisher’s state are called subscribers. 

The Observer pattern suggests that you add a subscription mechanism to the publisher class so individual objects can subscribe to or unsubscribe from a stream of events coming from that publisher.  
Fear not! Everything isn’t as complicated as it sounds.  
In reality, this mechanism consists of 1) an array field for storing a list of references to subscriber objects and 2)  
several public methods which allow adding subscribers to and removing them from that list. 

<p align="center">
<img src="https://github.com/user-attachments/assets/95b52447-2219-452d-a7a8-586a509d0b72" width="500" />
</p> 

Now, whenever an important event happens to the publisher, it goes over its subscribers and calls the specific notification method on their objects. 

Real apps might have dozens of different subscriber classes that are interested in tracking events of the same publisher class. 
You wouldn’t want to couple the publisher to all of those classes. Besides, you might not even know about some of them beforehand if your publisher class is supposed to be used by other people.

That’s why it’s crucial that all subscribers implement the same interface and that the publisher communicates with them only via that interface.  
This interface should declare the notification method along with a set of parameters that the publisher can use to pass some contextual data along with the notification. 

<p align="center">
<img src="https://github.com/user-attachments/assets/f8f24b4f-3138-4e61-961b-3454cfad911a" width="500" />
</p> 

If your app has several different types of publishers and you want to make your subscribers compatible with all of them, you can go even further and make all publishers follow the same interface.  
This interface would only need to describe a few subscription methods.  
The interface would allow subscribers to observe publishers’ states without coupling to their concrete classes. 

## Real-World Analogy

If you subscribe to a newspaper or magazine, you no longer need to go to the store to check if the next issue is available.  
Instead, the publisher sends new issues directly to your mailbox right after publication or even in advance. 

The publisher maintains a list of subscribers and knows which magazines they’re interested in.  
Subscribers can leave the list at any time when they wish to stop the publisher sending new magazine issues to them. 

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑  *Open/Closed Principle.* You can introduce new subscriber classes without having to change the publisher’s code (and vice versa if there’s a publisher interface).| ☒ Subscribers are notified in random order.|
|☑ You can establish relations between objects at runtime.||







