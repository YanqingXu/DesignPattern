// 抽象基类表示游戏UI元素
abstract class UIElement {
    protected parent: UIElement | null = null;

    setParent(parent: UIElement | null) {
        this.parent = parent;
    }

    getParent(): UIElement | null {
        return this.parent;
    }

    abstract render(): void;
}

// 叶子节点：按钮
class Button extends UIElement {
    constructor(private label: string) {
        super();
    }

    render() {
        console.log(`Button: ${this.label}`);
    }
}

// 叶子节点：文本框
class TextBox extends UIElement {
    constructor(private text: string) {
        super();
    }

    render() {
        console.log(`TextBox: ${this.text}`);
    }
}

// 组合节点：面板
class Panel extends UIElement {
    private children: UIElement[] = [];

    add(element: UIElement) {
        element.setParent(this);
        this.children.push(element);
    }

    remove(element: UIElement) {
        const index = this.children.indexOf(element);
        if (index !== -1) {
            element.setParent(null);
            this.children.splice(index, 1);
        }
    }

    render() {
        console.log('Panel:');
        this.children.forEach((child) => {
            child.render();
        });
    }
}

// 创建游戏UI元素的示例
const button1 = new Button('Start');
const button2 = new Button('Options');
const textBox = new TextBox('Welcome to the Game');
const panel = new Panel();

// 组合元素
panel.add(button1);
panel.add(button2);
panel.add(textBox);

// 渲染整个UI层次结构
panel.render();
