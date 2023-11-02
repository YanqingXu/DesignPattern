中介者模式（Mediator Pattern）是一种行为设计模式，其核心思想是通过一个中介对象来封装一系列对象之间的交互方式，使得对象之间不需要显式地相互引用，从而降低它们之间的耦合度。
这种模式有助于维护一个系统中组件的交互，使得组件之间的通信被统一管理，当系统变化时，可以更容易地进行维护和扩展。

### 中介者模式的组成部分

1. **中介者（Mediator）接口**：定义了一个接口用于各同事（Colleague）对象之间的通信。
2. **具体中介者（Concrete Mediator）**：实现中介者接口，并协调各同事对象之间的交互关系。具体中介者知道所有的同事类，并从具体的同事类中接收消息，向他们发送指令。
3. **同事类（Colleague）**：通常是一个类的集合，它们要通信的行为被抽象出来。每一个同事类知道它的中介者对象，且与其他的同事类通信时，必须通过中介者。

### 工作原理

- 同事类不直接通信，而是通过中介者对象进行通信。
- 每个同事类在需要与其他同事类通信时，会发送一个消息到中介者，而不是直接与其他同事类交互。
- 中介者接收到消息后，会适当地处理它，并将需要的消息传递给其他同事类。

### 使用场景

- 当对象之间的通信方式非常复杂，导致多个对象之间相互依赖、耦合度高，难以理解和维护时。
- 当多个类相互交互，但是这种交互展现出多对多的网络关系时，使用中介者模式可以将网状结构转换成以中介者为中心的星形结构。
- 当某个系统想要在类之间设立一个可以集中控制交互的点时。

### 优点

- 减少了类之间的耦合性，使得可以独立地改变和复用各个类和中介者。
- 由于把对象如何协作进行了抽象，使得我们更容易地理解每个对象的作用。

### 缺点

- 中介者本身可能会变得过于复杂，集中了过多的控制逻辑，从而变得难以维护。

### 实例代码（Python）

以下是使用中介者模式的一个简单例子：

```python
class Mediator:
    def notify(self, sender, event):
        pass

class ConcreteMediator(Mediator):
    def __init__(self, colleague1, colleague2):
        self._colleague1 = colleague1
        self._colleague2 = colleague2
    
    def notify(self, sender, event):
        if sender == self._colleague1:
            print(f"Mediator reacting on {event} and triggering colleague2.")
            self._colleague2.do_something()
        elif sender == self._colleague2:
            print(f"Mediator reacting on {event} and triggering colleague1.")
            self._colleague1.do_something()

class BaseColleague:
    def __init__(self, mediator):
        self.mediator = mediator

class Colleague1(BaseColleague):
    def do_something(self):
        print("Colleague1 does something.")
        self.mediator.notify(self, "Colleague1")

class Colleague2(BaseColleague):
    def do_something(self):
        print("Colleague2 does something.")
        self.mediator.notify(self, "Colleague2")

# 使用中介者
mediator = ConcreteMediator(None, None)
c1 = Colleague1(mediator)
c2 = Colleague2(mediator)
mediator._colleague1 = c1
mediator._colleague2 = c2

c1.do_something()
c2.do_something()
```

在这个例子中，`Colleague1` 和 `Colleague2` 不直接通信，而是通过 `ConcreteMediator` 进行交互。当 `Colleague1` 完成某项操作时，它通过中介者来通知 `Colleague2`，而不是直接调用 `Colleague2`。

这个模式允许我们将通信逻辑从实际的同事类中分离出来，使得系统更易于维护和扩展。