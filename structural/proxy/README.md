![grafik](https://github.com/user-attachments/assets/43fc0489-329f-4e6f-94b2-bd8df97a2b14)

# Proxy

## Intent

Proxy is a structural design pattern that lets you provide a substitute or placeholder for another object.  
A proxy controls access to the original object, allowing you to perform something either before or after the request gets through to the original object.

## Problem

Why would you want to control access to an object?  
Here is an example: you have a massive object that consumes a vast amount of system resources. You need it from time to time, but not always.

<p align="center">
<img src="https://github.com/user-attachments/assets/9c33e4e3-986c-4072-be81-9c887030fede" width="500" />
</p>


You could implement lazy initialization: create this object only when it’s actually needed. All of the object’s clients would need to execute some deferred initialization code. Unfortunately, this would probably cause a lot of code duplication.

In an ideal world, we’d want to put this code directly into our object’s class, but that isn’t always possible. For instance, the class may be part of a closed 3rd-party library.

## Solution

The Proxy pattern suggests that you create a new proxy class with the same interface as an original service object. Then you update your app so that it passes the proxy object to all of the original object’s clients. Upon receiving a request from a client, the proxy creates a real service object and delegates all the work to it.


<p align="center">
<img src="https://github.com/user-attachments/assets/385461b8-9eab-40fc-8617-1aa2e5a2ea1d" width="500" />
</p>


But what’s the benefit? If you need to execute something either before or after the primary logic of the class, the proxy lets you do this without changing that class. Since the proxy implements the same interface as the original class, it can be passed to any client that expects a real service object.

## Real-World Analogy

<p align="center">
<img src="https://github.com/user-attachments/assets/5bfc6080-fe84-4568-a055-301add489aaa" width="500" />
</p>


A credit card is a proxy for a bank account, which is a proxy for a bundle of cash. Both implement the same interface: they can be used for making a payment. A consumer feels great because there’s no need to carry loads of cash around. A shop owner is also happy since the income from a transaction gets added electronically to the shop’s bank account without the risk of losing the deposit or getting robbed on the way to the bank.


