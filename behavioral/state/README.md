![grafik](https://github.com/user-attachments/assets/c70fb1ae-d46d-407a-bb7c-5cebf025fd23)

# State

## Intent

**State** is a behavioral design pattern that lets an object alter its behavior when its internal state changes.  
It appears as if the object changed its class.  

## Pros and Cons

| Pros | Cons |
| ----------- | ----------- |
|☑ *Single Responsibility Principle*. Organize the code related to particular states into separate classes.| ☒ Applying the pattern can be overkill if a state machine has only a few states or rarely changes. |
|☑ *Open/Closed Principle*. Introduce new states without changing existing state classes or the context. ||
|☑ Simplify the code of the context by eliminating bulky state machine conditionals. ||
