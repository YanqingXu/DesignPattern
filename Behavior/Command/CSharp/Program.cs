using System;
using System.Collections.Generic;

public struct Vector3
{
    public float X, Y, Z;

    public Vector3(float x, float y, float z)
    {
        X = x;
        Y = y;
        Z = z;
    }
}

public interface ICommand
{
    void Execute();
}

public class Character
{
    public void Move(Vector3 direction)
    {
        Console.WriteLine($"Moving in direction: {direction}");
        // 实现角色移动逻辑
    }
}

public class MoveCommand : ICommand
{
    private Character character;
    private Vector3 moveDirection;

    public MoveCommand(Character character, Vector3 moveDirection)
    {
        this.character = character;
        this.moveDirection = moveDirection;
    }

    public void Execute()
    {
        character.Move(moveDirection);
    }
}

public class CommandQueue
{
    private Queue<ICommand> commands = new Queue<ICommand>();

    public void AddCommand(ICommand command)
    {
        commands.Enqueue(command);
    }

    public ICommand GetNextCommand()
    {
        if (commands.Count > 0)
        {
            return commands.Dequeue();
        }
        return null;
    }
}

public class RecordAndPlaybackManager
{
    private CommandQueue commandQueue = new CommandQueue();
    private List<ICommand> recordedCommands = new List<ICommand>();

    public void RecordCommand(ICommand command)
    {
        recordedCommands.Add(command);
    }

    public void ReplayRecordedCommands()
    {
        foreach (var command in recordedCommands)
        {
            commandQueue.AddCommand(command);
        }
    }
}

class Program
{
    static void Main(string[] args)
    {
        Character player = new Character();
        RecordAndPlaybackManager recorder = new RecordAndPlaybackManager();

        // 记录玩家操作
        ICommand moveCommand1 = new MoveCommand(player, new Vector3(1, 0, 0));
        ICommand moveCommand2 = new MoveCommand(player, new Vector3(0, 1, 0));
        recorder.RecordCommand(moveCommand1);
        recorder.RecordCommand(moveCommand2);

        // 回放记录的操作
        recorder.ReplayRecordedCommands();
    }
}
