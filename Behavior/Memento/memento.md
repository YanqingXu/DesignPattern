备忘录模式（Memento Pattern）是一种行为设计模式，它允许在不暴露对象内部细节的情况下捕获和保存一个对象的当前状态，以便可以在以后的时间里以这个保存的状态来恢复对象。
这种模式特别有用于实现撤销/重做功能或是保存和恢复功能。

### 目的：
- 提供一种恢复状态的机制。
- 使对象的状态可以外部化，从而不破坏封装性。

### 设计模式的关键参与者：

1. **Originator（发起人）**:
   - 负责创建一个备忘录，用以记录当前时刻它的内部状态。
   - 负责使用备忘录恢复内部状态。

2. **Memento（备忘录）**:
   - 存储Originator对象的内部状态。
   - 防止Originator以外的其他对象访问备忘录。

3. **Caretaker（看护人）**:
   - 负责保存好备忘录。
   - 不能对备忘录的内容进行操作或检查。

### 实现步骤：
1. **定义Memento类**：
   - 通常包含了要保存的对象状态的重要字段。
   - 可能有一个标识符，用于在需要时检索正确的备忘录。

2. **在Originator类中添加创建备忘录和恢复备忘录的功能**：
   - 创建备忘录时，将当前状态保存在Memento对象中。
   - 恢复备忘录时，利用保存的Memento对象恢复状态。

3. **Caretaker决定何时保存状态到Memento以及何时恢复状态**：
   - Caretaker可以保存多个备忘录以提供多个恢复点。

### 示例代码（Python）:

```python
import copy

class Memento:
    def __init__(self, state):
        self._state = copy.deepcopy(state)

    def get_state(self):
        return self._state

class Originator:
    _state = None
    
    def set_state(self, state):
        self._state = state

    def save_to_memento(self):
        return Memento(self._state)

    def restore_from_memento(self, memento):
        self._state = memento.get_state()

class Caretaker:
    _mementos = []

    def backup(self, originator):
        self._mementos.append(originator.save_to_memento())

    def undo(self, originator):
        if not self._mementos:
            return
        memento = self._mementos.pop()
        originator.restore_from_memento(memento)

# 使用
originator = Originator()
caretaker = Caretaker()

originator.set_state("State1")
caretaker.backup(originator)

originator.set_state("State2")
caretaker.backup(originator)

originator.restore_from_memento(caretaker._mementos[0])  # 恢复到State1
```

### 优缺点：

#### 优点:
- 提供了一种可以恢复状态的安全机制。
- 保持了封装边界的完整性。
- 简化了Originator的结构，因为它不需要管理保存和恢复状态的逻辑。

#### 缺点:
- 如果客户端不加限制地进行备份，可能会占用大量内存。
- 每个独立的备忘录都可能保存大量数据，从而导致内存消耗加剧。

备忘录模式在功能强大的同时也是资源敏感的，因此在使用时应该考虑其对资源的影响，尤其是在状态数据量大时。