<p align="left">
<img src="https://github.com/user-attachments/assets/53546aaf-1323-4627-9029-a8a467fbbc9a" width="200" />
</p>

# [Visitor](https://refactoring.guru/design-patterns/visitor)

## Intent
**Visitor** is a behavioral design pattern that lets you separate algorithms from the objects on which they operate.

## Problem 

Imagine that your team develops an app which works with geographic information structured as one colossal graph.  
Each node of the graph may represent a complex entity such as a city, but also more granular things like industries, sightseeing areas, etc.  
The nodes are connected with others if there’s a road between the real objects that they represent.  
Under the hood, each node type is represented by its own class, while each specific node is an object.

At some point, you got a task to implement exporting the graph into XML format. At first, the job seemed pretty straightforward.  
You planned to add an export method to each node class and then leverage recursion to go over each node of the graph, executing the export method.  
The solution was simple and elegant: thanks to polymorphism, you weren’t coupling the code which called the export method to concrete classes of nodes.

Unfortunately, the system architect refused to allow you to alter existing node classes.  
He said that the code was already in production and he didn’t want to risk breaking it because of a potential bug in your changes.

<br/><p align="center">
<img src="https://github.com/user-attachments/assets/67ebe44d-9142-4465-b865-115dc735bda5" width="400" />
</p><br/>

Besides, he questioned whether it makes sense to have the XML export code within the node classes. The primary job of these classes was to work with geodata. The XML export behavior would look alien there.

There was another reason for the refusal. It was highly likely that after this feature was implemented, someone from the marketing department would ask you to provide the ability to export into a different format, or request some other weird stuff. This would force you to change those precious and fragile classes again.


## Solution

The Visitor pattern suggests that you place the new behavior into a separate class called visitor, instead of trying to integrate it into existing classes. The original object that had to perform the behavior is now passed to one of the visitor’s methods as an argument, providing the method access to all necessary data contained within the object.


## Real-World Analogy

Imagine a seasoned insurance agent who’s eager to get new customers. He can visit every building in a neighborhood, trying to sell insurance to everyone he meets. Depending on the type of organization that occupies the building, he can offer specialized insurance policies:

- If it’s a residential building, he sells medical insurance.
- If it’s a bank, he sells theft insurance.
- If it’s a coffee shop, he sells fire and flood insurance.

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ *Open/Closed Principle*. You can introduce a new behavior that can work with objects of different classes without changing these classes.| ☒  You need to update all visitors each time a class gets added to or removed from the element hierarchy.|
|☑ *Single Responsibility Principle*. You can move multiple versions of the same behavior into the same class.| ☒ Visitors might lack the necessary access to the private fields and methods of the elements that they’re supposed to work with.|
|☑ A visitor object can accumulate some useful information while working with various objects. This might be handy when you want to traverse some complex object structure, such as an object tree, and apply the visitor to each object of this structure.||


