public interface IExpression
{
    int Interpret();
}

public class NumberExpression : IExpression
{
    private int number;
    public NumberExpression(int number) => this.number = number;

    public int Interpret() => number;
}

public class AddExpression : IExpression
{
    private IExpression left;
    private IExpression right;

    public AddExpression(IExpression left, IExpression right)
    {
        this.left = left;
        this.right = right;
    }

    public int Interpret() => left.Interpret() + right.Interpret();
}

public class SubtractExpression : IExpression
{
    private IExpression left;
    private IExpression right;

    public SubtractExpression(IExpression left, IExpression right)
    {
        this.left = left;
        this.right = right;
    }

    public int Interpret() => left.Interpret() - right.Interpret();
}

public class InterpreterClient
{
    public static void Main(string[] args)
    {
        IExpression expression = new AddExpression(
            new NumberExpression(5),
            new SubtractExpression(
                new NumberExpression(8),
                new NumberExpression(2)
            )
        );

        Console.WriteLine($"Result: {expression.Interpret()}"); // 输出：11 (5 + (8 - 2))
    }
}
