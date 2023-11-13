abstract class AIBase {
    // 模板方法
    public makeDecision(): void {
        this.analyzeEnvironment();
        this.chooseAction();
        this.executeAction();
    }

    // 分析环境，子类需要实现
    protected abstract analyzeEnvironment(): void;

    // 选择行动，子类需要实现
    protected abstract chooseAction(): void;

    // 执行行动，子类需要实现
    protected abstract executeAction(): void;
}

class OffensiveAI extends AIBase {
    protected analyzeEnvironment(): void {
        console.log("OffensiveAI analyzing environment...");
    }

    protected chooseAction(): void {
        console.log("OffensiveAI choosing aggressive action...");
    }

    protected executeAction(): void {
        console.log("OffensiveAI executing action...");
    }
}

class DefensiveAI extends AIBase {
    protected analyzeEnvironment(): void {
        console.log("DefensiveAI analyzing environment...");
    }

    protected chooseAction(): void {
        console.log("DefensiveAI choosing defensive action...");
    }

    protected executeAction(): void {
        console.log("DefensiveAI executing action...");
    }
}

function main() {
    const offensive = new OffensiveAI();
    offensive.makeDecision();

    const defensive = new DefensiveAI();
    defensive.makeDecision();
}

main();
